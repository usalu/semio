# M1 Store Seal

Wave 1a store write-gate seal for `OS-EXCLUSIVE-STATE-AUTHORITY`.

## What changed

- **`DocumentCommand`** gained `IngestRemote { envelope }` and `PruneDrafts` (after `AmendLast`).
- **`set_state` / `set_envelope` / `ingest_remote`** are `pub(crate)` implementation details.
- **`pub fn reset(...)`** is the sole public reload API (returns `CommandReceipt`).
- **`pub fn dispatch(...)`** returns `CommandReceipt { edit_ids, generation }` and is the sealed write gate for Apply/Undo/… plus `IngestRemote` / `PruneDrafts`.
- **`IngestRemote` / `PruneDrafts` skip outbound flush** (preserves prior `ingest_remote` behaviour).
- **`CommandReceipt`**, **`DocumentEnvelopeView`**, **`DraftStore`** alias, and **`envelope_view()`** added.
- Envelope fields stay `pub` for serde/plugins; mutation is sealed through `dispatch` / `reset`.
- **`PruneDrafts`** is a reserved no-op stub for draft-lane stores.
- Call sites under store sync, plugin `VcsDocumentApp`, and OS/host workflow store now use `reset` / `dispatch(IngestRemote)`.

## OpBinary / text

- Binary ordinals: `IngestRemote=9`, `PruneDrafts=10`.
- Text: `PruneDrafts` printable/parsable; `IngestRemote` has no text form.
- Serde for `IngestRemote` uses wire `encode_envelope` / `decode_envelope` bytes.

## Cargo check

- `cargo check -p semio-framework-os-kernel --lib` → **Finished** (warnings only; see `🧪check-kernel-m1.err`).
- Plugin crate compiles after updating `Ok(())` dispatch matches to `Ok(_)`.
- `semio-framework-os` host check hits an unrelated missing `plugin_bundle_installer_shim.rs` (pre-existing; not part of this seal).
