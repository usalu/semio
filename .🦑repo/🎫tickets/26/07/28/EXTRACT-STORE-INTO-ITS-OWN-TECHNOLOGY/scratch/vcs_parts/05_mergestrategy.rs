//#region 🔖MergeStrategy
// 🎞️ CW3 kernel cut-over: `merge_concurrent_diffs` moved (real per-`MergeStrategyKind` dispatch,
// replacing this fn's blind `absorb()` collapse) to `protocol_crdt`, re-exported via the
// `🚧TEMPORARY protocol shim` near the top of this file — zero external callers existed (grepped
// repo-wide), so the signature change is invisible outside this crate's own (now-deleted) def.

/// @emoji 🔒 Content-addressed checkpoint id: `ck-<hex16(blake3(parent_id || ordered_change_content_
/// hashes || message || authors || timestamp))>`, replacing the old fully-random counter-string
/// scheme (`create_document_vcs_id("checkpoint")`) — two peers that independently commit the
/// identical checkpoint content (same parent, same changes in the same order, same message/authors/
/// timestamp) now converge on the identical id instead of minting two different ones. `changes` must
/// already contain every entry `change_ids` references (including one freshly created by this same
/// commit, if any) — callers push a new `Change` before calling this.
fn content_addressed_checkpoint_id(parent_id: Option<&str>, change_ids: &[String], changes: &[Change], message: Option<&str>, authors: &[Author], timestamp: &str) -> String {
    let mut input = Vec::new();
    input.extend_from_slice(parent_id.unwrap_or("").as_bytes());
    input.push(0);
    for change_id in change_ids {
        let change_hash = changes
            .iter()
            .find(|change| change.id == *change_id)
            .map(|change| *blake3::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes())
            .unwrap_or([0u8; 32]);
        input.extend_from_slice(&change_hash);
    }
    input.push(0);
    input.extend_from_slice(message.unwrap_or("").as_bytes());
    input.push(0);
    for author in authors {
        input.extend_from_slice(author.id.as_bytes());
        input.push(0);
    }
    input.push(0);
    input.extend_from_slice(timestamp.as_bytes());
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("ck-{hex16}")
}
//#endregion 🔖MergeStrategy
