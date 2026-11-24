# 🧠️ Why `semio-s-artifact-stdio-pdf-model` needs gigabytes to compile

All numbers: one gated build per measurement (`native-cargo.sh`, 3 jobs, dev profile, debuginfo off, incremental off), command
`rustc -p semio-s-artifact-stdio-pdf-model --lib -- -Ztime-passes -Zmacro-stats`, sampler `measure-model-memory.sh` (top and ps every 20 s,
logs in `🗑️generated/model-mem-*`). rustc's own `-Ztime-passes` rss column is the authoritative peak, the 20 s samples only confirm it.
The measuring flags add a little memory; before and after use the same flags. The 8 GB seen with plain `top` is the same build without them.

## Cause

The crate is 270 KB of hand-written source. The derives expand it to about 107 MB of source text (1.7 M lines), because each derive emits a
complete hand-written state machine per type instead of calling shared code:

| expansion (macro-stats) | uses | bytes before | avg per use before |
| --- | --- | --- | --- |
| `NativeReceive` on the model types | 82 | 18.3 MB | 223 KB |
| `NativeReceive` on generated enum carriers/payloads | 635 | 50.8 MB | 80 KB |
| `RetireOwned` (carriers + types) | 635 + 82 | 15.6 + 3.3 MB | 25 + 40 KB |
| `RetainedClone` | 82 | 14.3 MB | 175 KB |
| `ToValue` / `FromValue` / `CanonicalJsonTree` | 81 / 81 / 79 | 3.1 / 2.1 / 0.5 MB | |

Two multipliers make it superlinear:

1. Every enum variant of an adjacently or internally tagged enum expands to a payload struct and a carrier struct, and each of those derives the
   full record receiver again. 82 types become 717 fully derived receivers (7.7 per type).
2. Each derived receiver is a copy of the same ~250 line state machine (scan, match, place, close, retire, seed, 40 `NativeReceiver` methods).
   Only about a tenth of it depends on the field types, yet it is re-parsed, resolved, type-checked, borrow-checked, linted and
   re-encoded per type (and per field arm, nine arm families per field).

rustc phases before (rss at phase end): expand 1.4 GB, typeck 3.7 GB, borrowck 3.9 GB, privacy 5.4 GB, lints 5.4 GB, metadata 6.3 GB, peak rss 6.4 GB, peak footprint **8.6 GB**.

## Fix (root, no behaviour change intended)

* `NativeReceive` record derive: the machine moved into the framework once. `owned/🏗️record-machine/🦀️.rs` (`NativeRecordCore`, `NativeRecordReceiver`,
  `NativeRecordArms`, `RecordSpec`, `FieldShape`, erased `RecordSlotView`/`RecordSlot<T>` over the receive slot and the phased default slot, the per-field generic
  arms `field_plan/field_apply/field_leaf/field_work/field_nested/field_retire/field_begin/field_close_plan/field_close_step/field_admit`).
  A derived record now emits only its slot struct, a static `RecordSpec` descriptor table and one-line arms per field and family
  (about 17 KB per carrier instead of 80 KB, 46 KB instead of 223 KB per model type). Another agent moved the machine into the new crate
  `semio-framework-value-native-record-machine` to keep the owner within the source boundary; the derive output paths follow it.
* `NativeReceive` enum derive: `owned/🏗️record-machine/🔀️enum/🦀️.rs` (`NativeEnumCore`, `NativeEnumReceiver`, `EnumChild<P,T,N>`, `EnumChildOps`,
  `NativeEnumChildren`). An enum emits its children enum, one constructor closure and two `match` arms per variant.
* `RetireOwned` derive: the 20-method `RetirementCursor` delegation of the generated wrapper cursor became one generic
  `retirement::ShellCursor<S: RetirementShell>` (`♻️retirement/🪞️shell/🦀️.rs`); the derive emits two `match` arms (`shell`, `shell_mut`) per type.
* `OwnedFrame` and `SharedFrame` slots share one macro-generated erased slot implementation (`🧩️slot/🦀️.rs`).
* Derive unit tests adjusted to the new expansion shape (`🛬️receive/🧪️tests/🦀️.rs`).

## After (same flags)

`top` MEM (physical footprint including compressed pages, what the 8 GB figure is) is the honest metric. rustc's `-Ztime-passes` rss column and `ps` rss
under-report whenever the machine is compressing (other agents compile in parallel), so the first "after" run looked better than it was.

| | before | after1 (record + enum + RetireOwned shell) | after2 (+ no rooted vtable, static terminal check) |
| --- | --- | --- | --- |
| derive expansion total | 107 MB | 47 MB | 47 MB |
| macro_expand_crate rss | 1.4 GB | 0.8 GB | 0.8 GB |
| typeck + borrowck (rss at borrowck end) | 3.9 GB | 3.1 GB | 3.8 GB |
| privacy + lint phases | 5.4 GB | 3.25 GB | 3.86 GB |
| LLVM passes / link_rlib | 7 s / 0.6 s | 32 s / 13 s | 9 s / 0.8 s |
| mono items (-Zprint-mono-items) / rlib | not measured | 807 231 / 1.47 GB | small (rlib no longer dominated by receivers) |
| **peak footprint (top MEM, 1 s samples for after2)** | **8.6 GB** | **9.3 GB** | **5.2 GB** (peak rss 5.3 GB) |

after1 regressed because a private non-generic `born()` called from the public generic `receiver()` became a codegen root, its drop glue reached
`Drop for NativeRecordReceiver`, whose `is_empty` coerced the field arms to `dyn NativeRecordArms`, and that vtable instantiates every arm of every record
(807 K mono items). Fix: `born` is now a method of the generic `NativeRecordArms` impl, the terminal check is static (`slots_empty`) and the owner count uses
the same erased view only on demand. The model crate itself now instantiates nothing from the receive machinery; crates that really drive receivers still do.

## What still blocks 2.5 GB (diagnosed, not yet fixed)

The target is not reached: 8.6 GB to 5.2 GB. What is left is front-end memory (3.8 GB rss at the end of borrowck) plus metadata and code generation on top of it
(`generate_crate_metadata` +0.8 GB, `compile_first_CGU_batch` +0.8 GB), because rustc keeps typeck results and MIR alive through codegen. The remaining 47 MB of expansion:

| expansion after | uses | bytes | avg |
| --- | --- | --- | --- |
| `RetainedClone` (still the per-row state machine) | 82 | 14.3 MB | 175 KB |
| `NativeReceive` carriers + model types | 635 + 82 | 11.3 + 3.7 MB | 18 + 46 KB |
| `RetireOwned` | 635 + 82 | 10.1 + 2.0 MB | 16 + 24 KB |
| `ToValue` / `FromValue` | 81 / 81 | 3.1 / 2.1 MB | |

Next steps, by yield (each needs a gated build, so they were not stacked blindly on top of an unverified base):

1. `RetainedClone`: same recipe as the record machine. Core state (phase, route, close, selected, child_reply, output) in a generic runtime struct, the 14 arm families
   (`plan_rows`, `demand_arms`, `work_arms`, `selection_arms`, `advance_routes`, `close_routes`, `close_find`, `close_values`, the four `demand/work_children/values` and `empty`)
   as per-row generic fns over `T`. Expected 14 MB to about 3 MB.
2. `NativeReceive` arms: replace the nine arm families per field by one `match` arm that calls a single generic `field_op::<T>(op)` with an `Op` enum, and erase `N` and the
   const field count from the core (`partial` and `output` behind an `OwnerOps` view) so the shared machine is instantiated once, not once per record, in the crates that drive receivers.
3. `RetireOwned`: the shared and boxed original-admission declarations (`original_allocation_retirement`, about 7 KB of identical allocator code per type) as one generic runtime function.
4. Crates that drive receivers (the snapshot, mutation and plugin crates behind `original_pack_value_operation_receiving!`) still instantiate `NativeReceiveSlot<dyn Source, T, F>`
   per field type T (the 807 K mono items of after1 were the whole machine; with the vtable root gone the per-T slot instances remain). Erasing `T` from the slot's
   poll and close machinery is the remaining codegen lever there.

Splitting the model crate was rejected: its ten modules reference each other cyclically (`object_model` and `document_metadata`, `content_operators` and `xobjects` and
`colour_and_graphics_state`), so no crate boundary exists along the module lines.

## Verification status

* derive crate (unit and integration) and owned crate (record, enum, shared receiver, work, retirement, map, set, persistent list suites): green. The one failure, `owned::tree::array::tests::sequence_storage_refusal_retains_the_exact_unpublished_value`, is a missing fixture path (`🧫️fixtures/📐️.json`), unrelated.
* PDF lib suite 38/0, io-export 19/0, page-editor 2/0 and page-editor-tests 65/0: green after the final change set.
* XLSX and XML libraries type-check with the new derive output; their test targets already fail on stale test code (`fixture`, `opc_layer`, `RetainedWorkOutcome` call shapes), unrelated to the derives.
* The measurement logs and sampler output lived in `🗑️generated/model-mem-*` and were deleted after the numbers were taken; `measure-model-memory.sh` reproduces them
  (`sh measure-model-memory.sh TAG rustc -p semio-s-artifact-stdio-pdf-model --lib -- -Ztime-passes -Zmacro-stats -Zthreads=1`; touch the model crate root first, or change a flag, else cargo reuses the build).
