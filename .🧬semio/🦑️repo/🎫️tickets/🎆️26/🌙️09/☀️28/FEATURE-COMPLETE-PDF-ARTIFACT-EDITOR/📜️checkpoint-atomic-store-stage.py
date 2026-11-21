"""📦 Stage Check In replay through the trusted Store's exact grouped merge."""
from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
STORE = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store")
FACTORY = STORE / "📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs"
ENGINE = STORE / "⚙️engine/🦀️.rs"
LAW = STORE / "🧪️tests/🌌️bin-04/🔬️unit/📮️document-codec-replay-and-projection-gate-laws/🦀️.rs"
FIXTURE = LAW.with_name("🔣️check-in-original-image.json")


def one(text, source, replacement):
    """🎯 Replace one exact production or test anchor."""
    assert text.count(source) == 1, source
    return text.replace(source, replacement)


def stage():
    """🔏 Record each unchanged preimage next to the trusted grouped replay postimage."""
    source = FACTORY.read_text()
    engine = ENGINE.read_text()
    law = LAW.read_text()
    close = "/// 📜️ Folds an [`semio_os_spr::encode_envelopes`] ledger stream"
    body = """/// 📦 Opens a trusted Check In ledger without losing an authored edit's boundary.
fn decode_check_in_replay(
    bytes: &[u8], document: &str,
) -> Result<Vec<semio_os_spr::atomic_hub::CheckpointReplayGroup>, VcsError> {
    use semio_os_spr::atomic_hub::{CheckpointReplayGroup, decode_checkpoint_replay};
    if let Some(commits) = decode_checkpoint_replay(bytes, document).map_err(|error| VcsError::Deserialize(error.into()))? {
        return Ok(commits);
    }
    let rows = semio_os_spr::decode_envelopes(bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    if rows.iter().any(|row| row.transaction.as_ref().is_some_and(|tx| semio_os_spr::atomic_group::is_atomic_backbone_tool(&tx.tool))) {
        return Err(VcsError::Deserialize("Check In original media requires its entire authored edit".into()));
    }
    Ok(vec![CheckpointReplayGroup::Ordinary(rows)])
}

async fn fold_check_in_replay<P, M>(
    store: &mut ArtifactStore<P, M>, commits: Vec<semio_os_spr::atomic_hub::CheckpointReplayGroup>,
) -> Result<(), VcsError>
where
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>,
{
    use semio_os_spr::atomic_hub::CheckpointReplayGroup;
    for commit in commits {
        match commit {
            CheckpointReplayGroup::Ordinary(rows) => {
                for row in rows {
                    check_in_report(store.ingest_remote(row).await?)?;
                }
            }
            CheckpointReplayGroup::Original(edit) => {
                match store.ingest_original_atomic(&edit).await? {
                    AtomicStoreReceive::Finished(report) => check_in_report(report)?,
                    AtomicStoreReceive::AwaitingDependencies => return Err(VcsError::ValidationFailed(
                        "Check In authored edit is waiting on an unknown dependency".into(),
                    )),
                }
            }
        }
    }
    Ok(())
}

fn check_in_report(report: semio_os_spr::MergeReport) -> Result<(), VcsError> {
    if report.accepted && report.conflict.is_none() { return Ok(()); }
    Err(VcsError::ValidationFailed("replay-envelopes quarantined a ledger envelope as a conflict".into()))
}

/// 📜️ Folds an authoritative ledger stream"""
    next = one(source, close, body)
    next = one(next, "use semio_os_store_engine::lifted_store::ArtifactStore;",
        "use semio_os_store_engine::{AtomicStoreReceive, lifted_store::ArtifactStore};")
    next = one(next, """    let envelopes = semio_os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let parsed = parse_document_pack""", """    let parsed = parse_document_pack""")
    next = one(next, """    let _canonical_line = canonical_check_in_line(&envelope.id);
    envelope.active_alternative_id = None;""", """    let _canonical_line = canonical_check_in_line(&envelope.id);
    let commits = match decode_check_in_replay(envelopes, &envelope.id) {
        Ok(commits) => commits,
        Err(error) => {
            envelope.retire_unadopted();
            return Err(error);
        }
    };
    envelope.active_alternative_id = None;""")
    start = next.index("    for envelope in envelopes {", next.index("pub async fn replay_envelopes_onto_pair"))
    end = next.index("    if folded.is_ok() && !store.log.dag.pending_is_empty()", start)
    next = next[:start] + """    if folded.is_ok() {
        folded = fold_check_in_replay(&mut store, commits).await;
    }
""" + next[end:]
    next = one(next, "the SAME gate every replica folds a remote envelope through ([`ArtifactStore::ingest_remote`]):",
        "the replica's ordinary merge ([`ArtifactStore::ingest_remote`]) or one authenticated authored edit:")
    engine_next = engine + "\n/// 🧾 Receives one entire verified original edit on the canonical Store frontier.\npub use semio_os_store_engine_bundle::AtomicStoreReceive;\n"
    test = (TICKET / "📜️checkpoint-atomic-store-law.rs").read_text()
    fixture = (TICKET / "🔣️checkpoint-atomic-store-law.json").read_text()
    assert not FIXTURE.exists()
    rows = ((FACTORY, source, next), (ENGINE, engine, engine_next), (LAW, law, law + "\n" + test), (FIXTURE, None, fixture))
    result = {"preimages": {}, "candidate": {}}
    for path, before, after in rows:
        fresh = path.read_text() if path.exists() else None
        assert fresh == before, path
        result["preimages"][str(path)] = sha256(before.encode()).hexdigest() if before is not None else None
        result["candidate"][str(path)] = after
    output = GENERATED / "checkpoint-atomic-store-overlay.json"
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print("[DEBUG] Staged trusted checkpoint replay and neutral Store law", len(rows))


if __name__ == "__main__":
    stage()
