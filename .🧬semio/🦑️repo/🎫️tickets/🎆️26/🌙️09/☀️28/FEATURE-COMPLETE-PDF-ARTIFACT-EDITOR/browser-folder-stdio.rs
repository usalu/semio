//! 🚪 Serves one authorized folder journal operation over the Vite child's private stdio.
use semio_framework_async::block_on;
use semio_os_spr::{
    channel::{AppDocumentIdentity, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, FIRST_GENESIS_ARCHIVE_PAGE_BYTES, FirstGenesisArchiveOffer, FirstGenesisArchiveScope, FirstGenesisArchiveSeal},
    wire::command_ingress::{ActorInstanceOpenPurpose, NATURAL_SOURCE_COMMAND_KIND, NaturalSourceWireAck, NaturalSourceWireDeclaration, NaturalSourceWireLoan, NaturalSourceWirePhase, NaturalSourceWireResult},
};
use semio_os_store_sync_native::FolderEventLogStorage;
use std::{
    env, io::{self, Read, Write}, path::PathBuf, process::ExitCode,
    sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc}, thread,
};

const MAGIC: &[u8; 8] = b"SEMIOFJ1";
const PAGE: usize = FIRST_GENESIS_ARCHIVE_PAGE_BYTES;
const START: u8 = 1;
const BYTES: u8 = 2;
const SEAL: u8 = 3;
const CANCEL: u8 = 4;

enum Frame {
    Page { offset: u32, length: u16, bytes: [u8; PAGE] },
    Publish([u8; 32]),
}

fn invalid(detail: &'static str) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, detail) }

fn read_array<const N: usize>(input: &mut impl Read) -> io::Result<[u8; N]> {
    let mut bytes = [0; N];
    input.read_exact(&mut bytes)?;
    Ok(bytes)
}

fn read_text(input: &mut impl Read, maximum: usize) -> io::Result<String> {
    let length = u16::from_le_bytes(read_array(input)?) as usize;
    if length == 0 || length > maximum { return Err(invalid("invalid folder scope text extent")); }
    let mut bytes = vec![0; length];
    input.read_exact(&mut bytes)?;
    if bytes.contains(&0) { return Err(invalid("invalid folder scope text")); }
    String::from_utf8(bytes).map_err(|_| invalid("invalid folder scope UTF-8"))
}

fn read_scope(input: &mut impl Read, document_id: String) -> io::Result<FirstGenesisArchiveOffer> {
    if &read_array::<8>(input)? != MAGIC { return Err(invalid("unknown folder journal protocol")); }
    let archive_extent = u32::from_le_bytes(read_array(input)?) as usize;
    let extent = u32::from_le_bytes(read_array(input)?);
    let instance_id = u32::from_le_bytes(read_array(input)?);
    let operation = u64::from_le_bytes(read_array(input)?);
    let generation = u64::from_le_bytes(read_array(input)?);
    let seq = u64::from_le_bytes(read_array(input)?);
    let store_generation = u64::from_le_bytes(read_array(input)?);
    let revision: [u8; 32] = read_array(input)?;
    let declaration = NaturalSourceWireDeclaration {
        format_kind: read_text(input, 512)?,
        extension: read_text(input, 512)?,
        media_type: read_text(input, 512)?,
        binary: true,
    };
    let loan = NaturalSourceWireLoan {
        operation_id: format!("{operation:016x}"), generation: format!("{generation:016x}"),
        instance_id, declaration: declaration.clone(), extent,
    };
    let purpose = ActorInstanceOpenPurpose::NaturalSource { declaration, extent };
    let accepted = NaturalSourceWireAck {
        owner: u64::from(instance_id), seq, kind: NATURAL_SOURCE_COMMAND_KIND, extent, loan: Some(loan.clone()),
        phase: NaturalSourceWirePhase::Complete, page_index: None, result: NaturalSourceWireResult::Accepted,
    };
    let document = AppDocumentIdentity {
        app_instance_id: instance_id, parent_document_id: Some(document_id),
        base_revision: u64::from_be_bytes(revision[..8].try_into().unwrap()),
    };
    FirstGenesisArchiveScope::from_completed_source(&purpose, instance_id, seq, &loan, &accepted, document, store_generation, revision)
        .and_then(|scope| FirstGenesisArchiveOffer::new(scope, archive_extent))
        .ok_or_else(|| invalid("invalid accepted source or archive frontier"))
}

fn receive(input: &mut impl Read) -> io::Result<Option<Frame>> {
    match read_array::<1>(input)?[0] {
        BYTES => {
            let offset = u32::from_le_bytes(read_array(input)?);
            let length = u16::from_le_bytes(read_array(input)?);
            if length == 0 || usize::from(length) > PAGE { return Err(invalid("invalid folder archive page extent")); }
            let mut bytes = [0; PAGE];
            input.read_exact(&mut bytes[..usize::from(length)])?;
            Ok(Some(Frame::Page { offset, length, bytes }))
        }
        SEAL => Ok(Some(Frame::Publish(read_array(input)?))),
        CANCEL => Ok(None),
        _ => Err(invalid("invalid folder archive command")),
    }
}

fn first(storage: FolderEventLogStorage, schema: &str, document: String, output: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    let offer = read_scope(&mut io::stdin().lock(), document)?;
    let scope = &offer.scope;
    let mut capture = storage.capture_first_genesis_archive(offer.clone(), scope)?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let incoming = cancelled.clone();
    let (send, frames) = mpsc::sync_channel(0);
    thread::spawn(move || {
        let mut input = io::stdin().lock();
        while let Ok(Some(frame)) = receive(&mut input) {
            if send.send(frame).is_err() { return; }
        }
        incoming.store(true, Ordering::Release);
    });
    output.write_all(&[START])?;
    output.flush()?;
    let current = || !cancelled.load(Ordering::Acquire);
    while let Ok(frame) = frames.recv() {
        if !current() { return Err(invalid("folder archive request cancelled").into()); }
        match frame {
            Frame::Page { offset, length, bytes } => {
                let receipt = capture.accept_page(&offer, offset, &bytes[..usize::from(length)])?;
                if !receipt.matches(&offer, offset, usize::from(length)) { return Err(invalid("folder archive page was not acknowledged").into()); }
                output.write_all(&[BYTES])?;
                output.write_all(&offset.to_le_bytes())?;
                output.write_all(&length.to_le_bytes())?;
                output.flush()?;
            }
            Frame::Publish(sha256) => {
                let seal = FirstGenesisArchiveSeal { offer: offer.clone(), sha256 };
                block_on(storage.commit_first_genesis_archive(capture, &seal, scope, schema, current))?;
                output.write_all(&[SEAL])?;
                output.write_all(&offer.extent.to_le_bytes())?;
                output.write_all(&sha256)?;
                output.flush()?;
                return Ok(());
            }
        }
    }
    Err(invalid("folder archive request ended before publication").into())
}

fn put(storage: FolderEventLogStorage, schema: &str, document: &str, output: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = io::stdin().lock();
    if &read_array::<8>(&mut input)? != MAGIC { return Err(invalid("unknown folder journal protocol").into()); }
    let length = u32::from_le_bytes(read_array(&mut input)?) as usize;
    if length == 0 || length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES { return Err(invalid("ordinary archive exceeds the format boundary").into()); }
    let mut archive = vec![0; length];
    input.read_exact(&mut archive)?;
    if input.read(&mut [0])? != 0 { return Err(invalid("ordinary archive has trailing request bytes").into()); }
    block_on(storage.write_archive(document, schema, &archive))?;
    output.write_all(&[SEAL])?;
    output.flush()?;
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let mode = arguments.next().and_then(|value| value.into_string().ok()).ok_or_else(|| invalid("missing folder journal operation"))?;
    let folder = arguments.next().map(PathBuf::from).filter(|value| value.is_absolute()).ok_or_else(|| invalid("missing absolute folder path"))?;
    let document = arguments.next().and_then(|value| value.into_string().ok()).filter(|value| !value.is_empty() && value.len() <= 512).ok_or_else(|| invalid("invalid folder document identity"))?;
    let schema = arguments.next().and_then(|value| value.into_string().ok()).filter(|value| value.len() <= 1024).ok_or_else(|| invalid("invalid folder document schema"))?;
    if arguments.next().is_some() { return Err(invalid("unexpected folder journal argument").into()); }
    let storage = FolderEventLogStorage::new(folder);
    match mode.as_str() {
        "first" if !schema.is_empty() => first(storage, &schema, document, &mut io::stdout().lock()),
        "put" => put(storage, &schema, &document, &mut io::stdout().lock()),
        _ => Err(invalid("unsupported folder journal operation").into()),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => { eprintln!("folder journal: {error}"); ExitCode::FAILURE }
    }
}
