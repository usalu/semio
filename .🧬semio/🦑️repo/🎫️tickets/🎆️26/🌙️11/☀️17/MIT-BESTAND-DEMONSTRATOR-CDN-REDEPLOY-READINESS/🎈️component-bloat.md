# Component bloat: root cause and fix

## Evidence (before)
- Bitcode member `cgu.00` of the snapshot rlib (`semio-s-artifact-energy-model-snapshot`, 521 MB): 37,132 functions, 34,530 of them `ControlledRetirement<T>` instances (641 distinct `T`); 4.67 M LLVM IR lines, of which `ControlledRetirement<T>::step_original` alone was 2.17 M (46 %, 5,116 instances because it is also generic over the `invoke` closure), `step_reply` 0.41 M, `step_with` 0.27 M, `next_demand` 0.22 M+0.09 M, `process_child_step` 0.24 M.
- The initialization (1.26 GB), mutations (1.0 GB), stdio-semio (1.2 GB) and snapshot rlibs are large because every crate that retires an owned type re-instantiates the full frontier state machine per owner type.

## Root cause
`🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs`: `impl<T: RetireOwned> ControlledRetirement<T>` held the entire T-independent retirement frontier machine (about 150 dense lines of stepping, quoting, reply recovery) inside the generic impl, with only `value: Option<T>` actually depending on `T`. Each owner type (641 in the energy snapshot) and each closure got its own copy.

## Fix
Same file: the frontier state and logic moved into the non-generic `ControlledCore`; `ControlledRetirement<T>` keeps only `value: ManuallyDrop<Option<T>>` plus forwarding wrappers. The four T-dependent facts (present, ready, supported, birth demand, cursor admission) go through the private object-safe `OriginalSlot` trait implemented for `Option<T>`. `step_with`/`step_original` take `&mut dyn FnMut` (`Invoke`) instead of generic closures; `reserve_cursor_slot` lost its unused generic reserve parameter. Public API and behaviour are unchanged.

## Verification
- `repo cargo --store=shared --configuration=release -- check -p semio-framework-value-retirement` passes (no new warnings from the controlled module).
- Not yet measured: after-sizes. The energy plugin crates are not members of the default cargo workspace (`-p` is refused), they are built crate-by-crate by the component build lanes, and `repo run @semio-tech/framework-value:test` failed in the repo graph service (host log reference in the run output) before reaching cargo. Expectation: per-owner-type instances shrink from about 54 large functions (about 425 IR lines each for `step_original` alone) to thin forwarding wrappers, removing about 90 percent of the 4.4 M IR lines in the snapshot crate; the dominant 1 GB rlibs and the 150 to 525 MB code sections should fall by an order of magnitude.

## Components needing rebuild
Every wasm component whose dependency graph contains `semio-framework-value-retirement` (all plugins using owned-value retirement): energy, fem, demonstrator, and every other plugin, because the controlled module is part of the shared value framework. Cached rlibs for the energy snapshot, initialization, mutations and stdio-semio crates are stale.

## Follow-up
Run the energy component build and compare `libsemio_s_artifact_energy_model_snapshot` (521 MB before) and the component size (612 MB before). If still large, next families are `admit_frame`, `admit_typed_controlled_retirement` (634 instances each) and the remaining 2,602 non-retirement symbols.

# Round 2: energy component after the retirement fix (analysis only, edits gated)

## Measured
- Component `.cache/b/r/m/components/runs/wv8mPf6/w/bfc4d2be009b4efa75d3159f/component.wasm`: 442,444,737 B (was 612 MB). Parsed by section: code 423,290,059 B in **754,576 functions** (mean 561 B), data 17.1 MB in 2 segments, tables/elements 1 MB. Data is not the problem for energy; function count is. No `wasm-tools`/`wasm-opt` is installed (`repo tools list`: wasm-opt missing), so no size-after-optimisation pass exists.
- rlibs after the fix (same hashes, rebuilt): initialization 1.07 GB (was 1.26), stdio-semio 1.11 GB (1.22), mutations 0.90 GB (1.0), snapshot 0.45 GB (0.52). The first fix removed about 13 percent of rlib bytes; most bloat is other generic families.

## Next families (initialization rlib, bitcode: 284,513 functions, 24.2 M IR lines)
Weighted by IR lines (count of instances):
| lines | n | family | where |
|---|---|---|---|
| 3.58 M | 5,525 | `NativeReceivingEffect::drive_original_producer_bound<P,H,closure>` | `🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs:258` |
| 1.12 M | 5,525 | `drive_original_effect_bound_with<P,closure>` | `…/🧳️frame/🎛️driver/🦀️.rs:32` |
| 0.50 M + 0.48 M + 1.17 M | 5.5 k each | `receive_original_effect`, its closure, `poll` | same driver macro / `🛬️decode/♻️retirement/🦀️.rs:131` |
| 2.19 M / 0.87 M / 0.74 M / 0.53 M | 613 | derived receive state `plan`, `close_plan`, `apply`, `close_poll` (one per mutation variant, 3,500 IR lines for `plan`) | `🌱️value/✨️derive/🛬️receive/🦀️.rs:115-146` and `🔀️enum/🦀️.rs:48-66` |
| 1.69 M | 1,850 | `NativeReceiverBackingClose::drive` | `native_receiver_protocol::backing` |
| 1.20 M | 9,962 | `native_close_settle` | `native_receiver_protocol` |
| 0.66 M + 0.41 M + 0.30 M + 0.25 M | about 920 | `advance`, `advance_local`, `advance_nested`, `quote` (derive) | `✨️derive/🛬️receive` |
| 0.61 M | 12,748 | `Result::map` closures around the above | |
| 0.23 M, 0.21 M, 0.13 M | 1,544 / 1,249 | `admit_frame`, `admit_retirement_cursor`, `retirement_cursor_birth_demand` | `♻️retirement/🎮️controlled`, `♻️retirement/🦀️.rs:78,90` |

About 30 percent of the crate is the NativeSlotProducer driver chain (P and closure generic although the body depends only on the failure type `E` and the host `H`); about 20 percent is derive output that is per type by construction.

## Planned cuts (in order)
1. Driver chain: introduce an object-safe `OriginalProducerDrive<H,E>` (`invoke` + `finish`) implemented by a small adapter `(authority, FnOnce)`; make `drive_original_producer_bound`, `drive_original_effect_bound_with`, `receive_original_effect` and the `poll` path non-generic over `P` and the closure (generic only over `E`, `H`). Expected: remove about 6.5 M of 24 M IR lines (27 percent).
2. `NativeReceiverBackingClose::drive` and `native_close_settle`: same pattern (about 2.9 M lines).
3. Derive-generated `plan/apply/close_plan/close_poll`: table-driven slot descriptors in the framework instead of straight-line code per variant (largest remaining block, about 4.4 M lines).
4. `admit_frame`/`admit_retirement_cursor`: share one non-generic body over `Layout` and an erased take callback.
5. Build flags (needs the build policy owner): `-Zshare-generics=yes` so identical instantiations are not rebuilt in each of the crates (only deduplicated by LTO afterwards), and `wasm-opt` (pinned 130 but not provisioned) which would run duplicate-function and inlining passes on the 754 k functions.

Reaching under 100 MB needs items 1 to 4 together (estimated 55 to 60 percent fewer IR lines) plus item 5; items 1 to 4 alone are unlikely to reach 100 MB. No Rust edits were made in this round because the serial `native-lanes.sh component` chain is still running (the `pgrep -f "^bash ./native-lanes.sh component"` guard does not match: the process is started as `bash /Users/ueli/Documents/semio/.🧬semio/…/native-lanes.sh component`, so use `pgrep -f "native-lanes.sh component"`).

## Round 3: drafts ready, gate still closed
Patch scripts (apply once `pgrep -f "native-lanes.sh component"` is empty twice 60 s apart), in `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-WM3Vc0/7c25358c-f2ca-40df-b930-7919fd8d29e5/scratchpad/p/`:
- `step1.py`: `OriginalProducerDrive<H,E>` + `BoundProducerDrive` in the receiving-frame crate, `drive_original_producer_dyn`, driver macro `drive_original_effect_dyn`/`receive_original_effect_dyn`, retirement control forwards through the dyn driver.
- `step2.py`: `native_close_settle` over `native_close_settle_dyn`; `admit_frame` over a shared `admit_frame_slot`.
- `step3.py`: `NativeFieldPlanSlot` + `plan_field_stage` in the owned tree; the derive's per-field `plan` arms (8-stage present/missing copies per field) become one call each.
Deferred: `apply`/`close_plan`/`close_poll` hoisting, `NativeReceiverBackingClose::drive`/`quote` vtable, `admit_retirement_cursor`.

## Round 4: patches applied, compile blocked by disk
- Gate opened (no `native-lanes.sh` process). `step1.py`, `step2.py`, `step3.py` applied cleanly (all asserts matched). Files changed: `🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`, `…/🧳️frame/🎛️driver/🦀️.rs`, `🌱️value/🛬️decode/♻️retirement/🦀️.rs`, `🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs`, `🌱️value/♻️retirement/🎮️controlled/🦀️.rs`, `🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs`, `🌱️value/✨️derive/🛬️receive/🦀️.rs`.
- Not yet compiled: `repo cargo --store=isolated … check` refuses to start because the Data volume `/` is 97.7 percent used (limit 97 percent, 5.3 GiB free, about 6.8 GiB needed). `repo cache cas gc` freed only 180 MiB; I removed my own scratch files and six old `components/runs` directories on the separate cache mount (that mount has 69 GiB free and does not help).

## Round 5: measured result of steps 1 to 3
- `check` passes for retirement, receiving-frame, receiver-protocol, native-owned, native-decoding and derive (isolated, release); the full energy build also compiled the step-3 derive output.
- Energy component (`.cache/b/r/m/components/runs/w6FIa88/w/bfc4d2be009b4efa75d3159f/component.wasm`): **379,181,287 B** (442 MB before steps 1 to 3, 612 MB originally). `jco transpile` still traps with `out of bounds memory access`: `[FAIL 1] component-🔋️energy`.
- Conclusion: steps 1 to 3 cut only 14 percent of bytes although they removed about 40 percent of the measured IR lines of the targeted families; the remaining per-type code (derive `apply`/`close_*`/`advance*`, backing `drive`, `native_relay`, per-type retirement cursors) dominates. Reaching 228 MB needs another 40 percent.
- Not done: crate test suites, demonstrator build (it would trap the same way, estimated about 440 MB).

## Round 6: next cuts (DRAFT, not applied; apply only after the redeploy)
Same recipe as `plan_field_stage`: one non-generic helper per per-field stage machine, reached through an object-safe slot trait, with only the typed edges kept in the macro.
1. `apply` (`✨️derive/🛬️receive/🦀️.rs`, `field_apply` arms, 8-stage present and missing copy per field): add `NativeFieldApplySlot` (impl for `NativeReceiveSlot<dyn NativeValueSource,T>` and `NativeDefaultSlot<T>`: `apply_take(&mut self,pending:&mut dyn NativePendingField,effect)`, `apply_terminal_is_empty`) and `NativePendingField` (impl for `ManuallyDrop<Option<T>>`: `is_none`). `place_field::<T>` becomes a fn pointer argument `unsafe fn(&mut NativeRecordStorage<Name>,usize,&mut dyn NativePendingField,usize,&mut Effect)` instantiated per (Name,T) as a 3-line shim. Helper `apply_field_stage(stage,present,slot,pending,partial_place,source,field,initialized,work)->Result<Option<Step>,Primary>` carries the stage 1 to 7 transitions; stage 0 of present stays in the macro. Expected: removes most of `apply` (0.74 M IR lines).
2. `close_plan`/`close_steps`/`nested_retire`/`nested_begin`: the four per-field matches only call `NativeFieldPlanSlot` methods (plus `native_close_settle_dyn`), so route them through the same trait (`plan_close_demands`, add `close_step_dyn(&mut self,body,effect)`/`begin_close_into_dyn`) and delete the per-field closures (0.87 M + 0.53 M lines). The retirement `next_owner_demands::<T>` edge is passed as a fn pointer like `field_demand`.
3. `advance`, `advance_local`, `advance_nested`, `advance_work_demand`, `quote`: same trait; `nested_steps` already only dispatches to `slot.advance(...)`, add `advance_dyn`.
4. `NativeReceiverBackingClose::drive`/`quote` (`🧳️owned/🧬️protocol/♻️backing/🦀️.rs`, 1.69 M + 0.47 M lines, one per receiver type C): introduce `trait BackingReceiver { fn terminal_is_empty; fn is_closing; fn begin_close_demand; fn begin_close_work_demand; fn close_demands; fn close_work_demand; fn native_terminal_release; fn begin_close_into; fn recover_begin_close_into; fn close_step; fn close_original_step; fn recover_*; }` as the object-safe subset of `NativeReceiver<I,T>`, plus `trait BackingStorage { get/get_mut as &dyn/&mut dyn BackingReceiver, reserved, address, take_box_into, release, box_demand, box_work_demand, release_demand }` implemented by `NativeOwnedStorage<C>`; `drive` and `quote` take `&mut dyn BackingStorage` and `&mut dyn BackingRetired` (for `Option<Box<C>>`), keeping the public generic wrappers `close_into` etc. as one-line forwards.
5. `native_relay`, `admit_retirement_cursor`, `retirement_cursor_birth_demand`: `native_relay` takes `&mut dyn FnMut` like `native_close_settle_dyn`; the cursor admit splits the allocation into a `Layout`-parameterised core as `admit_frame_slot` does.
Estimated additional reduction: 3.5 to 4 M of the remaining 15 M IR lines in the initialization crate; combined with the shipped cuts, about 40 percent of the original bytes, i.e. roughly 300 MB for energy before wasm-opt and `-Zshare-generics`.

## Round 7: component chain on the patched sources
| component | result | detail |
|---|---|---|
| demonstrator | FAIL 1 | 1271 units; failed unit `semio-framework-job-failure` (rmeta) then no component; no size |
| energy | FAIL 1 | `semio-hub-energy-component` final crate killed, rustc exit status 137 (jetsam, swap peak 2.84 GiB, rustc 6.7 GiB resident sum); the earlier 379,181,287 B component is the last good size |
| fem | FAIL 1 at describe | component.wasm 127,594,960 B (was 228 MB); describe refuses: 4 declared artifact kinds not owned ("native io: original interpreter deadline requires its caller clock") |
| procedural | FAIL 1 at describe | component.wasm 151,047,665 B (was ~160 to 230 MB); describe: "plugin assembly failed - no declared composer capability owns the runtime claims" |
Crate tests: retirement (30 tests), receiving-frame (4), receiver-protocol (4) pass; native-owned, native-decoding, derive not run.
