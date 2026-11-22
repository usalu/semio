//! 📤 Drives one exact first-genesis message per native request until durable receipt.
use semio_framework::kernel::{ActorInstanceOpenPurpose, NaturalSourceWireAck};
use protocol::channel::{
    FirstArchiveAction as Action, FirstArchiveKey, FirstArchiveReply as Reply, FirstArchiveRequest,
    FirstGenesisArchiveOffer, FirstGenesisArchiveScope, FirstGenesisArchiveSeal,
};
use protocol::AppDocumentIdentity;
use std::{future::Future, time::{Duration, Instant}};
use store_sync::sync::FolderEventLogStorage;

pub(super) async fn persist<F: Future<Output = Result<Option<Vec<u8>>, String>>>(
    storage: &FolderEventLogStorage, purpose: &ActorInstanceOpenPurpose, accepted: &NaturalSourceWireAck,
    expected_identity: &AppDocumentIdentity, schema: &str,
    mut exchange: impl FnMut(FirstArchiveRequest, bool) -> F, current: impl Fn() -> bool,
) -> Result<Option<AppDocumentIdentity>, String> {
    let key = FirstArchiveKey::completed(accepted).ok_or("first archive requires Accepted Complete")?;
    let loan = accepted.loan.as_ref().ok_or("first archive requires its exact source loan")?;
    let result = async {
        let request = FirstArchiveRequest { key, action: Action::Open };
        let packet = receive(request, &mut exchange, &current, false).await?;
        let Some(Reply::Opened { extent, document, generation, revision }) = Reply::decode(request, &packet) else {
            return Err("first archive did not offer the committed Store".to_owned());
        };
        let identity = Reply::opened_identity(key.instance, document, &revision);
        if identity != *expected_identity { return Err("first archive changed the selected document identity".into()); }
        let scope = FirstGenesisArchiveScope::from_completed_source(
            purpose, key.instance, key.sequence, loan, accepted, identity.clone(), generation, revision,
        ).ok_or("first archive belongs to another genesis")?;
        let offer = FirstGenesisArchiveOffer::new(scope.clone(), extent as usize).ok_or("first archive extent refused")?;
        let mut spool = storage.capture_first_genesis_archive(offer.clone(), &scope).map_err(|error| error.to_string())?;
        let mut offset = 0;
        while let Some(length) = offer.page_length(offset) {
            let request = FirstArchiveRequest { key, action: Action::Preview { offset } };
            let packet = receive(request, &mut exchange, &current, false).await?;
            let Some(Reply::Page(page)) = Reply::decode(request, &packet) else {
                return Err("first archive did not return its pending page".into());
            };
            let receipt = spool.accept_page(&offer, offset, page).map_err(|error| error.to_string())?;
            if !receipt.matches(&offer, offset, length) { return Err("first archive durable page receipt changed".into()); }
            let request = FirstArchiveRequest { key, action: Action::Acknowledge { offset, length: length as u16 } };
            let packet = receive(request, &mut exchange, &current, false).await?;
            if !matches!(Reply::decode(request, &packet), Some(Reply::Acknowledged)) {
                return Err("first archive did not accept its exact page receipt".into());
            }
            offset += length as u32;
        }
        let request = FirstArchiveRequest { key, action: Action::Seal };
        let packet = receive(request, &mut exchange, &current, false).await?;
        let Some(Reply::Sealed(sha256)) = Reply::decode(request, &packet) else {
            return Err("first archive did not seal its acknowledged bytes".into());
        };
        storage.commit_first_genesis_archive(spool, &FirstGenesisArchiveSeal { offer, sha256 }, &scope, schema, &current)
            .await.map_err(|error| error.to_string())?;
        let packet = receive(request, &mut exchange, &current, false).await?;
        if !matches!(Reply::decode(request, &packet), Some(Reply::Sealed(current)) if current == sha256) {
            return Err("first archive changed during its durable append".into());
        }
        Ok(identity)
    }.await;
    let closing = FirstArchiveRequest { key, action: if result.is_ok() { Action::Close } else { Action::Cancel } };
    let settled = receive(closing, &mut exchange, &|| true, true).await.and_then(|packet| {
        matches!(Reply::decode(closing, &packet), Some(Reply::Closed))
            .then_some(()).ok_or_else(|| "first archive close did not settle".to_owned())
    });
    match (result, settled) {
        (Ok(identity), Ok(())) if current() => Ok(Some(identity)),
        (Err(failure), Err(close)) => Err(format!("{failure} ({close})")),
        (_, Err(close)) => Err(close),
        (Err(failure), Ok(())) if current() => Err(failure),
        _ => Ok(None),
    }
}

async fn receive<F: Future<Output = Result<Option<Vec<u8>>, String>>>(
    request: FirstArchiveRequest, exchange: &mut impl FnMut(FirstArchiveRequest, bool) -> F,
    current: &impl Fn() -> bool, closing: bool,
) -> Result<Vec<u8>, String> {
    let deadline = Instant::now() + Duration::from_secs(300);
    loop {
        if !current() { return Err("first archive cancelled".into()); }
        if Instant::now() >= deadline { return Err("first archive receipt stalled".into()); }
        let Some(packet) = exchange(request, closing).await? else { continue; };
        match Reply::decode(request, &packet) {
            Some(Reply::Pending) => continue,
            Some(Reply::Refused) => return Err("first archive request refused".into()),
            None => return Err("foreign first archive reply".into()),
            Some(_) => return Ok(packet),
        }
    }
}
