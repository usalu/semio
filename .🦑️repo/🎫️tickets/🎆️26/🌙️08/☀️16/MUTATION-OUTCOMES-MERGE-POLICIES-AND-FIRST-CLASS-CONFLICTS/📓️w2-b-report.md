# Lane 2-B (Rust wasmtime host) — report

## Summary

The host had zero references to `MutationMessage`/`DispatchReport`/`MergePolicy` before this lane;
it is now the missing link. All four tasks landed inside the exclusive lease
(`🔌️plugin/🖥️host/🦀️component.rs`, `🏃️run/**`).

## 1. Decode/expose new payloads (host)

New region `🔖️MutationReports` in `host/🦀️component.rs` (after `🔖️ArtifactSession`):
- `decode_dispatch_report(bytes) -> Result<protocol::DispatchReport, PluginHostError>`
- `decode_merge_report(bytes) -> Result<protocol::MergeReport, PluginHostError>`
- `decode_conflicts(bytes) -> Result<Vec<protocol::Conflict>, PluginHostError>`
- `next_host_seq()` — one shared monotonic seq source for every host-initiated command
  (`context_menu` was refactored onto it too, removing its duplicate local static).

`ArtifactSession` gained four fields, mirrored the SAME way `LoadDocument`/`LoadConfig`/`Emit` already
mirror into it (`pre_adopt_command_packs`/`post_adopt_frame_packs` — no second event/callback
mechanism invented):
- `merge_policy: protocol::MergePolicy` — mirrored from `AppCommand::SetMergePolicy` on the way past.
- `last_dispatch_messages: Vec<protocol::MutationMessage>` — mirrored from `AppFrame::Invocation.messages`
  (success) or `AppFrame::Error.report` (rejection), whichever last carried a non-empty report.
- `last_merge_report: Option<protocol::MergeReport>` — mirrored from unsolicited `AppFrame::MergeReport`.
- `open_conflicts: Vec<protocol::Conflict>` — mirrored from `AppFrame::Conflicts`.

A host caller observes these the same way it already observes `document`/`config`/`draft`: via
`WasmPluginRuntime::document_session(instance_id)`.

## 2. Merge-policy handshake + pass-throughs (host)

- `WasmPluginRuntime::hello(instance_id, app_id, actor, config, merge_policy)` — batches
  `AppCommand::Hello` + `AppCommand::SetMergePolicy` in ONE `exchange` call, so the session's policy is
  established before any other command reaches the instance. (`Hello`'s own wire shape is frozen —
  contract §C8's tag table adds no policy field to it — so this is the real `SetMergePolicy` wire
  command, ridden along with the handshake rather than a second mechanism.)
- `WasmPluginRuntime::set_merge_policy(instance_id, policy)` — standalone setter (mid-session change).
- `WasmPluginRuntime::resolve_conflict(instance_id, conflict_id, resolution) -> MergeReport`.
- `WasmPluginRuntime::read_conflicts(instance_id) -> Vec<Conflict>`.

Fixed two call sites that were about to break on `AppFrame::Error`'s new trailing `report` field
(`context_menu`'s match arm, and the in-file `FakeCluster` transaction-test fixture's construction
site) — both inside my lease.

## 3. `🏃️run/**`: policy from config/CLI + diagnostics

- `SpaceRunner<H>` gained a `merge_policy: protocol::MergePolicy` field (constructor param, default
  `Normal`); `compute_node` batches `AppCommand::SetMergePolicy` right after `Hello` for every node.
- `bin.rs`: `--policy <laissez-faire|normal|vigilant>` CLI flag (default `Normal`), threaded into
  `SpaceRunner::new`, echoed via `eprintln!("[os run] merge-policy: ...")`.
- New `dispatch_report_summary`/`dispatch_error_message` helpers decode a rejected frame's trailing
  `report` into real `code: message [target]` text, folded into every `AppFrame::Error` arm inside
  `compute_node` (6 sites) — a rejected node's `RunError` text (and, through it,
  `sink.record(RunMutation::Log{..})`'s sealed diagnostics on failure) now names the REAL
  `mutation.*` message, not just the generic `mutation.rejected` fault summary.
- `FakeHost` test fixture updated to answer `SetMergePolicy` with `Done` (else every existing
  `SpaceRunner` test would break on "sent no reply to seq N").

### Pre-existing breakage fixed in-lease (not mine to introduce, but blocking `cargo check` on files
inside my lease, so fixed rather than left broken — confirmed via `git diff` that none of these lines
were touched by prior lane work in this session)
- `run/🦀️component.rs::frame_in_reply_to` was missing match arms for `TransactionProposal`/
  `TransactionPrepared`/`TransactionCommitted`/`TransactionRolledBack`/`MergeReport`/`Conflicts`
  (E0004 non-exhaustive) — added all six.
- `run/🦀️component.rs::io_router_stats` still declared `-> (usize, usize)` after `IoRouter::stats`
  (host, same lease) already returns `Result` — `unwrap_or((0, 0))` (diagnostic-only stat line).
- `bin.rs` referenced `store::{ArtifactCommand, ParsedDocumentText, parse_document_pack, now_iso,
  BlobStore, create_document_envelope, ArtifactStore}` — none of these are re-exported at
  `semio_framework`'s crate root (confirmed: zero occurrences in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`).
  Added `extern crate semio_framework_os_kernel as protocol;` (bin targets need their own alias copies,
  per this file's own header comment) and repointed all seven onto `protocol::`/kept `store::` only for
  the symbols that DO still resolve there.

## 4. Wasmtime e2e test

`host/🦀️component.rs`, new region `🔖️MergePolicyE2e`,
`merge_policy_gates_a_real_dispatch_and_laissez_faire_still_surfaces_its_message` — uses the real
`block` plugin (already-built wasm at `🧑️‍💻️dev/🔌️plugin-modules/block/`), same
`WasmPluginRuntime::load`/`create_app`/`exchange` convention every other real-component test in this
file uses (no new plugin crate):
1. Looks up the block2d editor's real `app_id` from `runtime.manifest.apps` (dialect
   `s.block.block2d`, role `Editor`) — never hardcoded.
2. `Hello` → real `Welcome`.
3. `addHandleKind` then `addHandle` (real `ManifestActionInvocation`s) mint one real handle
   (`handle-1`) — proves the wasm document store is real, not a stub.
4. `ReadDocument` captures the document bytes.
5. **Normal** (default, never set explicitly): `removeHandle{id:"nonexistent-handle-id"}` →
   `AppFrame::Error{fault.code=="mutation.rejected", report}`; `report` decodes to a real
   `DispatchReport{policy: Normal, messages: [...mutation.target-missing...]}`; a follow-up
   `ReadDocument` proves the document is byte-for-byte unchanged.
6. **LaissezFaire**: `set_merge_policy(instance_id, LaissezFaire)`, same `removeHandle` dispatch →
   NOT rejected, comes back as `AppFrame::Invocation` whose `messages` still decode to
   `mutation.target-missing` (Law 2: an Error-level outcome's diff is empty regardless of policy — the
   dispatch still *applies* per §C9, and the message still surfaces).

## Verify (real counts, both commands run to completion)

`cargo check -p semio-framework-plugin-host`: **0 errors**, 4 pre-existing warnings (none mine — the
`Err(error)` unused-variable warning at `component.rs:109` predates this lane).
Raw: `🧪️w2-b-cargo-check-host.txt`.

`cargo test -p semio-framework-plugin-host`: **43 passed; 0 failed; 0 ignored** (42 pre-existing + the
1 new e2e test, `merge_policy_gates_a_real_dispatch_and_laissez_faire_still_surfaces_its_message`,
confirmed genuinely executed against the real `block.wasm`, not skipped). Doc-tests: 0.
Raw: `🧪️w2-b-cargo-test-host-final.txt` (also `🧪️w2-b-e2e-test.txt` for the isolated first run).

`cargo check -p semio-framework-os-run`: **0 errors** after the pre-existing-breakage fixes above.
Raw: `🧪️w2-b-cargo-check-run.txt`.

`cargo test -p semio-framework-os-run --lib`: **15 passed; 0 failed; 0 ignored**. The crate's
doc-test harness (0 actual doctests) intermittently fails to link
(`extern location for semio_framework_os_kernel does not exist: .../libsemio_framework_os_kernel.rlib`)
— a stale/incomplete build-artifact race from concurrent sibling-lane cargo activity in this shared
tree (matches the documented "Concurrent Cargo Workspace Churn" pattern), not a real failure: the same
`--lib` run is clean on retry and 0 doctests exist to actually fail. Raw: `🧪️w2-b-cargo-test-run.txt`.

Per the brief: `semio-framework-os-kernel`, `semio-framework-os-kernel-db`, `semio-compose-rs`,
`semio-s-plugin-stdio` were reported clean by other lanes at hand-off; nothing in this lane's checks
contradicts that.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs`

## Logs (ticket folder, `.txt`)

`🧪️w2-b-cargo-check-host.txt`, `🧪️w2-b-cargo-check-run.txt`, `🧪️w2-b-cargo-test-host.txt`,
`🧪️w2-b-cargo-test-host-final.txt`, `🧪️w2-b-cargo-test-run.txt`, `🧪️w2-b-e2e-test.txt`.
