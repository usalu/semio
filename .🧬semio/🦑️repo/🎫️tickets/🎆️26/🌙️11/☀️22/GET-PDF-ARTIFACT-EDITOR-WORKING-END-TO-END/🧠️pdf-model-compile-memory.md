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

## Round 2 (2026-11-25): every PDF crate under 4 GB

Same method, all builds through `native-cargo.sh` (`-Z threads=1`, 3 jobs, exclusive gate, memory guard active). Model crate: `measure-model-memory.sh`
(now `CRATE=<crate name>` selects the sampled rustc; 1–3 s `top` MEM = physical footprint, rustc `-Ztime-passes` rss per phase). Hub crate:
`measure-hub-memory.sh <tag>` (the wasm32-wasip2 prototype profile of `rebuild-pdf-wasm.sh`, plus `-Zdump-mono-stats`). Suites: `verify-memory-step.sh <tag>`
(PDF lib suite, io-export, page-editor tests with `--no-fail-fast`, snapshot-value `editor::pdf14`, XLSX/XML lib type-check, 5 s sampler of every rustc).

### Baseline (quiet machine)

The "9 GB" kills of the model crate were not reproducible with `-Z threads=1` on a quiet machine: the lib peaks at **4.38 GB** footprint (rss 3.99 GB,
borrowck end 3.36 GB, `generate_crate_metadata` 4.18 GB, 49 s). The 9–14 GB rustc processes in `memory-guard.log` (09:46–09:59) were builds outside the
gate (`-Z threads=0`, the repository default, one front-end thread and one first CGU batch per core). Expansion is unchanged from the last round (47 MB):
`RetainedClone` 14.3 MB (82 × 175 KB) is still the largest single derive.

### Step 1: `RetainedClone` hoisted into the framework

* `🧬️retained-clone/🧬️contract/🏗️record-cursor/🦀️.rs` (`retained_clone::record`): one `RetainedRecordCursor<O,R>` holds the core state (phase, output,
  source binding, spent, closing, close, route, selected quote, child reply) and implements every `RetainedCloneCursor` method once: plan, advance reply,
  close reply, demands, work, selection, release. Rows are `RetainedRecordRow<T>` (field cursor plus published value); the machine reaches them through two
  type-erased views, `RetainedRecordQuery` and `RetainedRecordDrive`, implemented once per field type `T` (not per owner), so the row code is shared by every
  record that has a field of that type. Row codes, close codes and every refusal text are those of the old expansion.
* The derive (`✨️derive/🧬️retained-clone/🦀️.rs`) emits per type only the rows struct, `RetainedRecordRows` (row count, rows per variant, live-variant selector,
  two one-arm-per-row dispatchers `query`/`drive` that also project the source, and the constructor) and the `RetainedClone` impl.
* Cause found on the way: the extra wrapper levels pushed the structural `Send` walk of deeply nested records past the recursion limit
  (`semio-s-artifact-stdio-pdf-snapshot-2`, `OptionCursor<PdfPredictor>: Send`). `RetainedRecordRow<T>` now states `Send` explicitly; a generic
  compile-time assertion in the same file proves `RetainedFieldCursor<T>: Send` and `Option<T>: Send` for every `T: RetainedClone`, so the explicit impl is
  exactly the auto impl and the walk stops at each row.

| model crate | baseline | after step 1 |
| --- | --- | --- |
| `RetainedClone` expansion | 14.3 MB (175 KB per type) | **1.39 MB** (17 KB per type) |
| `macro_expand_crate` rss | 0.77 GB | 0.60 GB |
| type check end / borrowck end rss | 2.01 / 3.36 GB | 1.84 / **2.75 GB** |
| `generate_crate_metadata` / `codegen_crate` rss | 4.18 / 4.18 GB | 3.39 / 3.82 GB |
| **peak footprint (top MEM)** | **4.38 GB** | **3.50 GB** |
| total time | 48.9 s | 30.5 s |

Verification (`verify-step1b`): PDF lib suite 27 passed, the 11 failures are all `NotFound` from the `bun` SQLite oracle missing on `PATH` in the script
(environment, fixed in the script for the next runs); XLSX and XML libraries type-check; all PDF crates and their test targets type-check
(`check -p semio-s-artifact-stdio-pdf -p …-io-export -p …-page-editor-tests -p …-pdf-snapshot-value-tests --lib --tests`).

### Step 3: `RetireOwned` original admission shared

* `♻️retirement/🧱️record/🏛️original/🦀️.rs` (`retirement::record`): one `OriginalRecordRetirement<O>` cursor and one `admit_original_record::<O>` allocator for
  both original allocations (`OriginalRecordOwner`, implemented by `SharedOriginal<T>` and `BoxOriginal<T>`; the two refusal texts that differed stay
  selected by `O::SHARED`), plus `original_record_demand::<O>()` for the quoted birth. The derived record emits its field ports once as
  `OriginalRecordFields` (one demand arm and one step arm per field, shared by the shared and the boxed original, which before each carried a full copy of
  the cursor struct, its allocator and all arms) and four one-line `RetireOwned` methods per allocation. The `Drop` refusal and the aligned-field check
  stay per type.

| model crate | after step 1 | after step 3 |
| --- | --- | --- |
| `RetireOwned` expansion (carriers + types) | 10.1 + 2.0 MB | **4.3 + 1.2 MB** |
| total derive expansion | 34 MB | 28 MB |
| type check end / borrowck end rss | 1.84 / 2.75 GB | 1.56 / **2.33 GB** |
| `generate_crate_metadata` / `codegen_crate` rss | 3.39 / 3.82 GB | 3.15 / 3.34 GB |
| **peak footprint (top MEM)** | **3.50 GB** | **3.17 GB** |
| total time | 30.5 s | 27.8 s |

Verification after steps 1 + 3 (`verify-step3`): PDF lib suite **38/0**, io-export **19/0**, snapshot-value `editor::pdf14` **34/0**, XLSX library type-checks.
page-editor-tests 67 passed, 2 failed: `mesh_move_shifts_the_decode_range` (`stdio.pdf.content-preserved`) and `paint_fidelity::canvas_paint_matches_mupdf_probes`
(`codec-ccitt-g4: image packed body does not match its declared geometry`) — image/mesh lifting refusals inside another session's uncommitted work on
`🔆️coded-image`, `📝️native-page-edit` and `🌈️paint-fidelity` (its `[DEBUG] lift_image` logging is in the same output), not clone or retirement behaviour.
The XML check failed on `semio_framework_trace` missing in `semio-plugin-instance-app`, another session's edit in flight at that moment (its dependency
landed minutes later). Sampled peaks of that run: `semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests` **3.71 GB** (was 5.2–5.5 GB in round 4),
`semio_s_artifact_stdio_pdf_model` 2.92 GB (dev, 20 s samples).

### Step 6: the 20-app hub split into runtime shards

Cause: `semio_hub_stdio_pdf` closed all 20 apps (`VcsArtifactApp<EditorApp<E>>` / `ViewerApp<V>`) in one `dyn_enum_close!`, so this one crate instantiated
the app-generic runtime (≈2 MB of framework source generic over `A: ArtifactApp`) twenty times. The editors are distinct types on purpose (dialect constant,
command enum, tool-owner witness identity), so sharing one instantiation would change behaviour; erasing them behind a dynamic app interface is against
the framework's static-dispatch rule (`sdk-dedyn`). The behaviour-preserving lever is where the instantiation happens.

* The stdio composition owners authority (`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🔣️.json`, schema `🧬️schema/🔣️.json`) gives every owner a `shards`
  list; `stdio-pdf` declares three: PDF 1.4 (6 apps), PDF 1.7 editors (7), PDF 1.7 viewers (7). The generator (`🧩️composition/🟦️.ts`) validates that the
  shards partition the owner's apps exactly, publishes each shard root (its own `dyn_enum_close!` over its apps) and shard manifest region, nests the shard
  enums as the owner enum's variants with one `From` per app (so `PluginBuilder::editor::<E>` / `viewer::<V>` and `plugin()` are unchanged), and adds the
  shard crates to the owner's dependency region. Every other owner has `"shards": []` and its output is byte-identical.
* New crates (handcrafted manifests, members of `🌎️hub/Cargo.toml`): `semio-hub-stdio-pdf-1-4`, `semio-hub-stdio-pdf-1-7-editors`,
  `semio-hub-stdio-pdf-1-7-viewers` under `🧩️extensions/📘️pdf/🧩️shards/`. Each instantiates the runtime of only its apps; the hub crate calls the shard
  enums' non-generic impls and keeps only the builder registrations.
* Composition tests (`🧱️production-preparation`, `🪪️owner-pairs`): 7/7 (the production-preparation test copies the shard manifests too).
* The generated app enum dispatches one more `match` level per call; nothing else changes at run time.

| wasm32-wasip2, prototype profile, `-Z threads=1` (`measure-hub-memory.sh hub6`, 20 s samples for the shards) | before (round 4/5) | after |
| --- | --- | --- |
| `semio_hub_stdio_pdf` | **5.94–8.76 GB** (r4f/r4g), 7.44 GB (r5b1) | **1.74 GB** (rss 1.94 GB; type check 0.52, metadata 1.54 GB) |
| `semio_hub_stdio_pdf_1_7_editors` / `_1_7_viewers` / `_1_4` | – | 3.18 / 1.97 / 1.65 GB |
| `semio_s_artifact_stdio_pdf_model` (wasm) | 4.14–5.06 GB | 2.85 GB |

This moves instantiations between crates (compile memory); it does not remove generated code. Code size is the next section.

## Guest code size

The browser compiles the component's biggest core module; its code section and function count are what Liftoff pays for (`🧮️pdf-browser-page-memory.md`).
Measured with `wasm-code-size.mjs` (biggest core module of a component: code, data, defined functions, median and largest body), builds through
`measured-wasm-build.sh` (the staged browser build) and `measure-wasm-profile.sh <tag> <opt> <lto> <codegen-units>` (the same component in its own
profile `pv<tag>`, so the staged build is neither replaced nor invalidated; 5 s samples of every rustc). `wasm-opt` runs through `measure-wasm-opt.sh`
(repository binaryen 130, stopped above 6 GB). Every row is one full rebuild of the wasm graph in lane `w`.

| build | opt-level | lto | CGUs | code | functions | median / largest body | build | peak rustc (PDF crates) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| before (09:37, round 5) | 1 | off | 16 | 271.5 MB | 573,605 | 121 B / 388 KB | – | 7.44 GB hub (r5b1) |
| r6a: steps 1, 3, 6 (staged) | 1 | off | 16 | 278.4 MB | 588,041 | 121 B / 388 KB | 40 min (33.4 cargo) | 3.43 GB component |
| pz | `"z"` | off | 16 | **184.8 MB** | 533,976 | 87 B / 140 KB | 31.7 min | 3.49 GB component |
| pzt | `"z"` | thin | 16 | 186.6 MB | 481,384 | 91 B / 201 KB | 39.6 min | **5.55 GB** component |
| pzf | `"z"` | fat | 16 | – | – | – | killed after 23 min | **8.87 GB** component (guard SIGKILL) |
| pz1 | `"z"` | off | 1 | – | – | – | killed after 33 min | **8.21 GB** component (guard SIGKILL); 1.7 editors 5.49, model 4.88, 1.4 4.55 GB |

* r6a vs before: the shards instantiate the per-snapshot functions (decode, codec, store) once per shard instead of once per crate, +14,436 functions
  (+2.5%); the rest of the new work in the tree is in the same build. r6a's stripped staged core module is 292.5 MB (code 278.4, data 12.6).
* `panic = "abort"`: no lever, it is already the `wasm32-wasip2` target default (`rustc --print target-spec-json`: `"panic-strategy": "abort"`).
* `wasm-opt -Oz` on r6a's 292.5 MB core module: stopped by the guard at **6.55 GB after 92 s**, still reading; binaryen cannot optimize a module of
  this size within the memory rule. On pz's core (`wasm-core-extract.mjs`, `--strip-debug -Oz`): stopped at **6.19 GB after 126 s**, same result.
* Thin LTO removes 10% of the functions (cross-crate inlining) but no code bytes, and costs +2 GB in the component crate; fat LTO cannot link within
  the guard. Neither is worth it at this module size.
* `codegen-units = 1` cannot link the component within the guard either, and it lifts every large crate (one CGU holds the whole crate in LLVM at once):
  the model crate to 4.88 GB and the 1.7 editor shard to 5.49 GB.
* Decision: `rebuild-pdf-wasm.sh` (the staged browser dev build) now uses `opt-level = "z"`; LTO, `codegen-units` and the other profile values stay.
  The next staged build will be the pz shape: 185 MB code, 534k functions, every PDF rustc ≤ 3.5 GB. Not yet rebuilt/staged with it.
* `opt-level = "s"` was not built: `"z"` is the stronger size setting and already the only lever that fits the guard; `"s"` would land between 1 and `"z"`.

### Where the 80 MB target stands

No profile setting reaches it: the best feasible build is 185 MB of code. What remains is generated code, not code generation:

1. **Ship what the page serves.** The browser component is the whole 20-app composition. One PDF 1.7 editor shard instantiates 7 apps; a component of
   only the apps a page opens (or one component per app) cuts the code roughly in proportion. This is now a composition choice: the shard crates of
   step 6 are exactly such units, so a PDF-editor component can close one shard instead of three.
2. **Remove per-app generic layers in the framework.** The 20× multiplier is the `A: ArtifactApp`-generic runtime (≈2 MB of source); where it only uses
   `A::Snapshot`/`A::Mutation`, splitting it into functions generic over those types alone collapses the 7 PDF 1.7 apps to one instantiation.
3. **Per-type derive machines** (steps 1 and 3 here) removed front-end work; the receive arms (step 2) are the remaining per-type expansion.

### Steps not taken in this round, and why

* Step 2 (`NativeReceive` arms into one op, owner type erased from the core) and step 4 (`ToValue`/`FromValue` tables): after steps 1 and 3 the model
  crate peaks at 3.17 GB and every PDF crate is under 4 GB, so neither is needed for the memory target. They remain the next code-size levers inside
  the model crate (`NativeReceive` 16.5 MB and `ToValue`/`FromValue` 5.2 MB of the remaining 28 MB expansion).
* Step 5 (splitting the model crate along the mutation families): only required if the model crate still exceeded the target; it does not.

### Final peaks (every PDF crate, `-Z threads=1`)

| crate | before | after |
| --- | --- | --- |
| `semio_s_artifact_stdio_pdf_model` (host lib, 1 s samples) | 4.38 GB | **3.17 GB** |
| `semio_s_artifact_stdio_pdf_model` (wasm, prototype) | 4.14–5.06 GB | 2.85 GB (opt 1) / 2.82 GB (`"z"`) |
| `semio_hub_stdio_pdf` (wasm) | 5.94–8.76 GB | **1.74 GB** (rss 1.94) / 1.39 GB (`"z"`) |
| `semio_hub_stdio_pdf_1_7_editors` / `_1_7_viewers` / `_1_4` (wasm, new) | – | 3.08–3.18 / 1.97–3.27 / 1.65 GB |
| `semio_hub_stdio_pdf_component` (wasm, cdylib) | 3.81–6.79 GB | 3.43 GB (opt 1) / 3.49 GB (`"z"`) |
| `semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests` (host) | 5.2–5.5 GB | **3.54–3.71 GB** |

### Changed files (round 2)

* `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🏗️record-cursor/🦀️.rs` (new), `…/🧬️contract/🦀️.rs` (mounts `record`)
* `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs` (`RetainedClone` record expansion, `RetireOwned` original admission)
* `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧱️record/🏛️original/🦀️.rs` (new), `…/🧱️record/🦀️.rs` (mounts it)
* `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`, `🔣️.json`, `🧬️schema/🔣️.json`, `🧪️tests/🧱️production-preparation/🟦️.ts`
* `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🦀️.rs` and `📦️packages/🦀️rust/Cargo.toml` (generated), `🧩️shards/{4️⃣1.4,7️⃣1.7/✏️editors,7️⃣1.7/👁️viewers}/`
  (`🦀️.rs` generated, `📦️packages/🦀️rust/Cargo.toml` new), `🌎️hub/Cargo.toml` (members)
* Ticket scripts: `verify-memory-step.sh`, `measure-hub-memory.sh`, `measure-wasm-profile.sh`, `measure-wasm-opt.sh`, `prepare-stdio-composition.ts`,
  `wasm-code-size.mjs`, `wasm-core-extract.mjs` (new); `measure-model-memory.sh` (`CRATE`), `rebuild-pdf-wasm.sh` (`opt-level = "z"`)
