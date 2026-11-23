# Presence, Space-History, MCP and Runtime Residue

Ticket `26/07/23/GET-PROCESS-WORKING-END-TO-END`. Builds ran through `stale-cargo.sh` (native `build-domain`, shared lock; each run waited 20 to 60 minutes behind other agents). Helper scripts: `presence-run.sh`, `residue-run.sh` (unique labels, because `stale-<label>.txt` is shared with other agents: a reused label such as `rt1` was overwritten by another agent).

## 1. Presence snapshot (`🏪️store/👥️presence/📸️snapshot`)

### Fixed (product, `🔤️json/🦀️.rs`)

| Failure | Root cause | Fix |
| --- | --- | --- |
| `original projection must be driven or returned through its resident frame` | `decode_presence_json` advanced the source admission with `admission.advance(effect)` only. After the admission reaches `projectionStart` the projection must run through a resident `OriginalProjectionFrame` (stages begin, drive, transfer producer, accept answer, finish). | A `StagedProjection<DslValue>` (existing `native-receiving-original` type) is quoted and advanced before the plain admission step (`staged_advance`), and closed (`projection.close_quote`/`close`) before the inputs are returned on the refusal path. |
| `native original receiver close lacks its original invocation` | `NativeReceivingOriginal::close_borrowed` requires `effect.invocation()`, which only exists when the close runs as an original producer action. The old `receiver_close` ran inside a plain `receive_effect`. | `ReceiverClose<P>` is an `OriginalEffectProducer` (begin close, then `RetirementCursor::close_into`/`recover_close_into`, converting `RetirementStep` to `RetainedCloneStep`). `close_poll` drives it through `native.receive_original_effect` with an `OriginalEffectAuthority`, a `NativeReceivingEffect<OriginalRetirementFailure>` frame and the same quoted demand and work accounting as `effect_step`. |
| Source publication left unfinished (`source must finish full granted ownership closure`) | `take_output` publishes in two turns (`publication_pending`). | The take loops until `!publication_pending`. |

Test side (`🧪️tests/🔬️unit/🦀️.rs`): the work and construction recipients now return the real `OriginalSourceTurnIdentity` of an `OriginalSourceTurnIssuer` dispatch, and `receive` wraps the turn in `with_source_witness` (the decoder refuses `UnsupportedOwner: native decoder has no original Source dispatch witness` otherwise). This also resolved the `UnsupportedOwner` instead of `Canceled` assertion path.

### State after the fixes

Decode now runs the whole parse, admission, receiving, take and close for the accepted rows (previously it stopped in the parse stage). The three tests still fail on accounting laws only:

- `accepted_rows...`: `decode requests are exactly the accepted capacity receipts` is off by 724 B (592511 requested against 591787 receipted).
- `refused_rows...` ("Empty Input"): 408 B net heap after the closing loop.
- `undersized_grants...` (`items` axis): 208 B net heap.

### Measurements (allocator probes without printing inside the observed scope; printing inside `observe` pollutes the numbers)

- Everything the JSON codec does is exact: after parse, admission and receiving advance the heap equals the receipts (the constant offset is the frame birth, receipted by the snapshot driver wallet). Every `effect_step` window was exact too (a temporary per-step check printed no mismatch).
- The unreceipted heap appears in exactly one close step of the receiver: `+472 B requested, 208 B released` while the receipt reports `+64 B`. That is 408 B requested and 208 B released with no receipt: a 208 B block that grows to 408 B and stays allocated, 200 B net. The refused (408 B) and undersized (208 B) leaks are the same two blocks, left allocated after the frame is closed. Dropping the control and the retirement recipient afterwards releases nothing, so these blocks are not held by them.
- This step is the start of the source close (`RetainedCloneSource::close_step`, `owner_close=Some(RetainedOwnerGate::new(ControlledRetirement::retain(owner)))` in `🧬️retained-clone/🔗️source/🦀️.rs`), which reports `copied_items:1` and no capacity. `ControlledRetirement::retain` builds a `ControlledCore` whose `cursors` is `PagedList<Box<dyn RetirementCursor>>::with_page_items(8)` (`♻️retirement/🎮️controlled/🦀️.rs`): one page of eight boxed cursors is about 208 B and a second page about 200 B more. That matches the 208 to 408 growth. This page storage is the strongest candidate for the common unreceipted allocation: it is born by the retirement stack of any `ControlledRetirement` and is neither quoted (`next_close_demand` capacity) nor receipted, nor returned by the funded close.
- `size_of::<OriginalRetirementFailure>()` is 336 B and `NativeSnapshotFailure` 6912 B. The first close turn of every refused owner births one 336 B failure spare (`OriginalFailureStorage`) and releases it with the last owner; it is receipted by the original birth step, so it is not the residue.
- The 2544 B per receive turn quoted in the earlier notes was not reproduced in the presence crate after the projection and close fixes. It is attached to invoked steps (reply handoff) and to the `collection` and `schema` tests; those crates were not rebuilt here, so the claim that it shares the 208/408 source is not confirmed.

### Proposed root fix (not applied)

`ControlledCore::new` (or the source close start) must either preborn the first cursor page inside the quoted and receipted demand of the close step that creates the `ControlledRetirement`, or keep the cursor stack in an inline `InlineStack` bounded by the quoted depth. Either way the quote of `next_close_demand` for the step that retains the owner must include the page capacity and the progress must report it as `retained_capacity_bytes` with the matching release at the end, so that `heap == receipts` holds. The change is in the shared value retirement crates, which other agents are editing, so it is left for the owner of `🌱️value/♻️retirement`.

## 2. Space-history (`🏪️store/📜️space-history`)

Not progressed. The first baseline run was blocked by another agent's `🧵️job/⏱️context/🛬️owned` compile error (`recv_dbg` not found, a temporary `[DEBUG]` helper in a file that must not be edited here). A rerun was not reached before the lock queue (20 to 60 minutes per build) exhausted the time box. The injection seams to port are unchanged: `🚦️native/🧪️tests/🧾️json-recovery`, `🪢️partial-projection` (`armed`/`refused` recipients with `Cell`), `🛂️validation/🧪️tests`. They must use the same pattern as the presence test (source identity on the recipients plus `with_source_witness`) before the refusal and cancel injections can fire.

## 3. MCP (`🌉️mcp`)

- The lib (126 tests) compiles (fixture sync from earlier holds); this run was filtered to the one failing test; the boxed `TurnFault::Guest` match compiles.
- `original_probe_pack_receiving...` fails at `close_decode(native,policy).result.unwrap()` with `allocation_refusal: Owned(OwnershipLimit "original failure storage exceeds its retirement grant")`. The failure comes from `quote_failure_storage` (`🌱️value/🛬️decode/♻️retirement/🦀️.rs`): the quoted capacity is `min(grant.maximum_capacity_bytes, capacity - spent, maximum_bytes - owned_bytes)` and must cover `OriginalFailureStorage::birth_bytes()` (336 B). The test builds `NativeDecodeControl::new(1048576, ..)`, exactly the policy capacity, so a cancelled owner that already owns most of the ceiling leaves no headroom for the failure spare. A diagnostic that printed `owned_bytes()` at the refusal was queued but cancelled while still waiting for the build lock and then reverted; the headroom hypothesis is therefore unverified. Next step: print `native.owned_bytes()` at the refusal, then give the control `policy capacity + OriginalFailureStorage::birth_bytes()` of ceiling (the test must leave headroom for the failure spare) if confirmed.

## 4. Plugin interpreter runtime and host

- `owned_instance_open_tests` (host and runtime): the neutral component `semio_plugin_host_test_component.wasm` has no producer in the tree. Added `wasm_component!`/`wasm_component_in` to `semio-framework-test-fixtures` (`🔨️modules/🧪️test/🧲️fixture-reader/🦀️.rs`): it runs `cargo build --target wasm32-wasip2 --manifest-path <component Cargo.toml> --message-format=json-render-diagnostics` once per process under the caller's own cargo environment (same target and build dirs, no new build dir), takes the `.wasm` from the artifact message and publishes it atomically at the existing `dist/components` path (ignored by `**/📦️packages/*/dist/`). Both `fixture_component()` functions now call it. Not compiled yet (the lock queue); the wasm build of the component itself has not been verified either.
- `microsecond_clock...` and `source_clock_reply...`: not reached (a run was queued and cancelled while waiting for the build lock).

## Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🔤️json/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧪️tests/🔬️owned-instance-open/🦀️.rs`
- ticket helpers `presence-run.sh`, `residue-run.sh`
