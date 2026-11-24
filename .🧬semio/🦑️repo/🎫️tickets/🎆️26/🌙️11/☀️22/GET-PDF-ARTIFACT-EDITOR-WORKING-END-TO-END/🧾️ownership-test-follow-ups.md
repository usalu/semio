# Ownership test follow-ups

Native runs went through `sh native-cargo.sh`, one cargo at a time, always filtered. Logs live in `🗑️generated/f2-*.log`, `f3-*.log`, `f4-*.log`.

## 1. Restored tests (both behaviours still exist, both ported to the effect-shaped API)

| Test | Where | Verdict | Result |
| --- | --- | --- | --- |
| `ephemeral_original_request_denials_and_cancellation_preserve_exact_custody` | `🔌️plugin/🫧️transient/🧵️publication/🧬️native/🧪️tests/🦀️.rs` (+ `🧫️fixtures/🔣️.json` restored from `HEAD`, `mod tests` re-declared in `🧬️native/🦀️.rs`) | Behaviour exists: `Preparation::advance` refuses a denied grant per axis (items, copy, capacity, depth) without touching its phase or the heap, and `close_step`/`close_step_controlled` keep the base and the original decoder custody. | passes |
| `original_receiving_refusal_keeps_the_raw_semantic_owner` | `🔌️plugin/🏛️ownership/⚠️failure/🧪️tests/🦀️.rs` | Behaviour exists in the new shape: a producer error leaves `with_native_receiving` inside `OriginalNativeReceivingOutcome.call.operation` with its heap untouched, and `PluginLifecyclePrimary::with_receiving_receipt` pairs a refused receipt with that raw owner. Only the generic producer error type is gone, so the test now carries a `ValueError`. | passes |
| config-factory law (`original_bounded_config_factory_retains_source_until_exact_physical_retirement`) | `🔌️plugin/♻️lifecycle/🦀️.rs` | Per-axis denial restored for `factory_retire_owned` (items, capacity, depth, copy) and for `artifact_retirement_box_close_step` (items, release). | passes |

### Why the previous agent's ports refused, and the actual causes

- The decoder-bound close needs **the same decoder** that advanced the preparation: the preparation records `(ledger identity, recipient identity)` on its first `advance`, so a bare `NativeDecodeControl` for the advance and a caller-turn decoder for the close can never match. Both phases now run in `OriginalJobFixtureCaller` turns with one `admitted_decoder()` (`turn_with_decoder` for advance, `retirement_turn_with_decoder` for close).
- A close grant equal to the quote does not move a reply-shaped ticket: the effect needs one more item, two more depth levels and capacity beyond the quote for the reply handoff, so the test closes with `closing(demand)` (copy at least the 4096 body, items 2, depth plus 2, capacity and release plus 4096). Without that slack the ticket answered `Progress(default)` forever. This quote/handoff gap is a framework finding, not fixed here.
- The "foreign control" probe needs a foreign decoder that already holds a receipt (one trivial receiving turn); a decoder without one is refused earlier as `InvariantViolated`, not `OwnershipLimit`.
- `AuthoringWallet::run_invoked` widens items and depth to at least 1 and adds the handoff copy, so a denied `items: 0` or `depth: 0` grant never reaches the factory. The denial axes narrow inside the effect with `effect.nested(denied)`.

## 2. `semio-plugin-ownership --lib` (2 failures in `reserved_job_session::original_refused_source_tests`)

- `original_lifecycle_refused_source_original_heap_and_context_are_retained_until_paid_close`: `StepContext::new` has no parent source supplier, and `RefusedWorkerSource::close_step` validates the context against the Source parent. Now runs under `OriginalJobFixtureCaller` with the caller's alias token as the worker cancellation (`bare_turn`, no origin service). It also needed the fixture `turn.maximumCopyBytes` raised from 4096 to 16384: the failure owner's close step quotes `body + frame` (6088 copy bytes for a 4096 body), so a 4096 turn can never fund it. Stale fixture, not a code bug.
- `original_reserved_pending_worker_same_source_denials_admission_cancel_and_paid_close`: `born 98,128` versus `freed 91,264`, difference **6,864 bytes**. Cause: every `caller.turn` serves the origin refresh (the resident return table), and those births live until `caller.retire`. The law compared them with the owner's own births. Fixed in the test: `born == freed + (caller.born_bytes() - resident_at_admission)`, and the delta is exactly 6,864, so the whole difference is the resident return table and nothing else.

`semio-plugin-ownership --lib`: **21 passed, 0 failed** (was 17 passed, 2 failed, 19 total at the start of the follow-up; the two restored tests and the already-present laws bring it to 21).

## 3. `original_causal_envelope_clone_preserves_wire_value_and_shared_actor_identity`

- **Abort fixed (cause was the harness, not the work recipient).** `drive_reply` in `semio-framework-value-clone-source-testing` unwrapped the producer drive. The foreign-source probe makes `advance_into` refuse with a plain `ValueError` (`original clone source or projected path changed`) before completing its reply, so the unwrap panicked with the resident action live and the thread-local bank then asserted at thread exit (`original work issuer retains an issued action`, abort). It now records the refusal as the reply with `OriginalEffectAction::restore_reply`, drives once more so the same action settles, and returns the refusal as the recorded `Err`. `drive_reply` takes `E: From<ValueError>` (every caller uses `OriginalRetirementFailure` or `ValueError`). The test binary no longer aborts; it fails as a normal test.
- **Test still red, for two independent reasons found behind the abort** (neither was reachable before, the abort hid them):
  1. The clone cursor quotes a capacity-only native birth step as `copy 0, capacity 392` but the step actually copies 424 bytes (`size_of::<MutationEnvelope>()`, the field cursor header) and retains 392. With a funded grant exactly equal to the quote the step answers `Progress(default)` with no work forever (stall at step 601 of the full-prefix run, independent of the foreign probe); with copy +4096, capacity +4096, items 2, depth +2 it progresses with `copied 424, retained 392`. The under-quote is in the native birth demand of the field cursor (`🧬️retained-clone/📦️backing`, `native.birth_demand()`/`reserve_demand`), which other agents are editing.
  2. The test's `grant()` asserts `copy + capacity <= 4096` for every indivisible body; the close frames now quote `3056/3008` and `4832/4784` (the envelope is 424 bytes). Not weakened here because the right bound depends on whether those frames grew on purpose.
- Test file restored byte for byte from `HEAD`; only `drive_reply` changed.

## 4. Child registry and fresh-footprint tests

- `fixed_child_member_registry_is_born_with_its_page_table_only_and_births_one_page_per_first_admission` never compiled. Four errors blocked the whole `semio-plugin-tests-02` crate:
  - `ChildMemberEntry` unqualified: now `semio_plugin_child::ChildMemberEntry`.
  - `registry.slots` private on `ArtifactFixedRegistry` (two uses in `original_child_root_registry_custody_and_full_physical_parent_receipt`): replaced by `registry.empty_backing_byte_demand()` before and after the denied release (the backing identity is no longer a pointer, the resident page table is its own release quote).
  - `publish_mounted_owned_child_operation_unit` gained a third `RetainedCloneGrant` argument (`🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver`): passes the turn's `grant`.
  - Result: the target test **passes**. The `child_member_registry` filter runs 25 tests: 17 passed, 8 failed. The 8 failures are other never-run laws in the same file (`displaced_composition_pins_...`, `original_archive_history_...`, `original_child_member_reference_metadata_...`, `original_child_root_registry_custody_...` at line 245 with heap `(32, 312)` versus receipt `(0, 280)`, `original_child_snapshot_metadata_...`, `original_mounted_fault_capture_...`, `original_snapshot_return_pump_...`, `owned_document_ingress_paging_...`). Not touched.
- Fresh-footprint law: `a_freshly_born_pdf_app_and_its_stores_hold_only_a_small_bounded_heap_footprint` in the PDF 1.7 editor unit tests (`📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`). It births the registered `EditorApp<Pdf17Editor>` (document, config, draft and interaction stores plus every registry and queue) through `new_registered_app`, measures the thread's retained heap before and after (`semio_framework_trace::retained_heap_bytes_on_this_thread`), asserts it is non-zero and at most `maximumBornBytes` of the language-neutral fixture `🧪️tests/🧫️fixtures/🪶️fresh-app-footprint/🔣️.json` (`plugin.fresh-app-footprint.v1`), then closes the app through the paid fixture close. **Measured: 6,127,047 bytes**; the bound is 6,815,744 (6.5 MiB) so any new eager page or backing fails it. It passes. The crate gained the `semio-framework-trace` dev-dependency (path form: the PDF workspace has no workspace entry for it).
- Why it lives in the PDF crate: I first wrote it in the declarations fixture (`semio-plugin-tests-18`), whose host crate has never compiled (568 errors: missing `semio_plugin_decl_media`, `FixtureApps` visibility, unresolved `protocol_laws`, `InstanceBirth` and others), and removed it again there. `semio-plugin-tests-02` compiles but has no app-birthing law, and `semio-plugin-tests-16` (the contract-app driver) was not attempted.
- 6.1 MB is not yet "small": it is the still-eager part named in `🧮️pdf-release-drain.md` (maintenance ladder, store-initialization worker pages, the other `VecDeque` backings). The law is a ratchet: lower `maximumBornBytes` as those drain.
## Changed files

- `🔌️plugin/👷️job/🚪️session/🫴️refusal/🧪️tests/🦀️.rs` and `🧫️fixtures/🔣️.json`
- `🔌️plugin/♻️lifecycle/🦀️.rs`
- `🔌️plugin/🏛️ownership/⚠️failure/🧪️tests/🦀️.rs`
- `🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs` (declares `mod tests`), `🧪️tests/🦀️.rs` and `🧪️tests/🧫️fixtures/🔣️.json` (restored)
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs` (`drive_reply`)
- `🔌️plugin/🧪️tests/🔬️app-child-member-registry/🦀️.rs`
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs`
- PDF: `🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🧫️fixtures/🪶️fresh-app-footprint/🔣️.json`, `🧬️schema/📸️snapshot/🧪️tests/🔬️pdf-snapshot-value/📦️packages/🦀️rust/Cargo.toml`

## Counts

| Run | Result |
| --- | --- |
| `semio-plugin-ownership --lib` (root workspace) | 21 passed, 0 failed |
| `semio-framework-replication-causal --lib -- original_causal_envelope_clone_preserves` | still 1 failed, no abort |
| `semio-plugin-tests-02 --lib -- child_member_registry` | 17 passed, 8 failed (target test passes; 8 other never-run laws fail) |
| PDF `a_freshly_born_pdf_app...` | 1 passed |
