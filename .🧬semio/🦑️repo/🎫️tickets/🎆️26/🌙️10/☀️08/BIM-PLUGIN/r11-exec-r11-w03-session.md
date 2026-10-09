# r11-w03-session execution report (WP-03: one inference path, progress and cancellation)

Status: **WRITTEN BUT UNVERIFIED by cargo** (no BIM build or test could run). The BIM crate depends on framework crates that do not compile
at the committed state (owner commit 677, retirement refactor): last gate run `cargo check -p semio-s-artifact-bim-model --lib`
(`T/🗑️generated/r11-w03-session/check9.txt`) fails in `semio-framework-plugin` (869 errors, was 980 -> 282 -> 0 -> 869 while r11-store/store-a/store-b
work), `semio-framework-artifact-playbook-playbook` (10) and `semio-framework-artifact-infinite-dag` (16, both still on `SnapshotRetirementStep`).
None of these errors is in a BIM file. What was verified instead: the whole BIM crate parses (`rustc -Zunpretty=normal --edition 2021 [--cfg test]
<crate root>` loads every `#[path]` module and every test file: 0 errors, so no file is missing), `bun T/r3-f1-check-names.ts` reports no problem for
any directory I added (its 19 problems are peers' duplicate emoji), and every new path is far below 256 characters (longest 150).

S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, I = `S/🧬️schema/💡️inferences/🕸️model-graph`.

## Result

One inference path. Every consumer reads derived values through a `ModelInferenceSession` of the model graph; the editor's own session type and
per-field `fields!` table are gone; nothing outside the graph runs a graph.

| Consumer | Before | After |
|---|---|---|
| Graph session `I/📡️session` | test-only, unbounded `resolve_ready`, panics | stepped and cancellable (`begin`/`begin_full`/`begin_sync`, `step(fuel)`, `finish`, `SessionRun::{cancel,progress,finished}`, `RunProgress`), `try_update/try_refresh/try_sync` return `InferenceError` as values, `update/refresh/sync` keep the last complete inference and put the fault in `report().fault`, `record(diff)` + trusted-diff `sync`, `probe::<WANT>`, an `epoch` so a run another run overtook is dropped instead of overwriting the newer inference. (z-incremental rebuilt the engine side of `step/finish` on `step_field_update`/`FieldUpdate` under this API unchanged.) |
| Registry (new) `I/🗂️registry` | editor `thread_local SESSIONS` + viewer `thread_local MEMO` + snapshot clone + deep compare | one `thread_local` map of sessions per instance (shared by editor, viewer, exports, imports): `with_inference`, `try_with_inference`, `record_mutations`, `report`, `probe`/`probe_rooms`, `begin/step/finish`, `Analysis` (the stepped, cancellable job object), `close`, `terminal_is_empty`. The registry is not borrowed while a read runs, so a read may reach it again. A failed run reads as the empty inference, never as another snapshot's. |
| Editor `S/✏️editor/🔮️inference` | own session type, `compute_*` table | **deleted** (directory contents and the mount in the crate root). `with_inference/record_mutations/close/terminal_is_empty` callers (canvas pointer-down, render, gestures, close hooks, 13 files) point at the registry. |
| Area gesture `rooms_of` | cloned snapshot + `rooms_of` | `registry::probe_rooms(ctx.instance, &probe)` on the instance's probe session (`ToolContext.instance`); `rooms_of` **deleted**. |
| Viewer `S/🖌️render` MEMO | `thread_local MEMO`, `solids()/plans()` | MEMO and both functions **deleted**; viewer window `render(snapshot, inference, ...)` takes the inference, the viewer root reads it from the registry by instance and clears it in `mounted_job_close_step`/`mounted_jobs_terminal_is_empty`. |
| IFC export | `ModelInference::infer` | `model_to_part21` via `registry::try_with_inference`; `projection::report(model, &document)` split out. Zoning report likewise. |
| CSV / glTF / SVG export | fresh `infer` / uncached graph per call | CSV and SVG via the registry (`io::with_inferred` for the serializer); glTF `scene::build(snapshot, &inference)` is pure, `model_to_gltf` returns `Result`. |
| IFC import | `compute_storey_levels` | `registry::probe::<{kinds::LEVELS}>` on the model read so far. |
| Oracle helper `encode_inference_projection_json` | `infer` | `try_with_inference` + pure `projection_of`. |
| Thin projections | `compute_*`, `infer_selected` (12 fns) | `#[cfg(test)]` only (production has none; ~220 peer test call sites keep working). The two external oracle subjects that used `compute_element_solids` (`🎲️infer-bim-1-solids-three`, `🔳️infer-bim-1-wall-solids`) use the registry. `ModelInference::infer` stays as the uncached reference every session run is tested against; it is fallible now (`try_infer_selected`, no panic). |

### Jobs (ArtifactCommandWork, progress + cancel)

- `exportModel` (`S/✏️editor/🎮️commands/📤️export-model`): `ExportJob`/`ModelExportWork`. Stage `bim-export-infer` per step (256 nodes), then `bim-export-encode`, then one
  `Effect::DownloadMediaExport` (ifc, glb as base64, svg, csv). `Progress { stage, preview }` carries en+de JSON; `cx.is_cancelled()` cancels the analysis (finished nodes stay cached).
  `handle` is the one-go dispatch path.
- `analyseModel` (`S/✏️editor/🎮️commands/🔎️analyse-model`): the recompute job: settles the instance's session in steps, completes with an empty `Emit`.
- Both rows are in the command table (`[HostOnly]; View`), bridge, `build_tool_job`, mount tree, fault notices (`bim.export.*`, `bim.analyse.*`) and `BimLabels` (en first, de second):
  labels `cmd_export_model(_describe)`, `cmd_analyse_model(_describe)`, `measure_export`, `measure_analyse`, `export_ifc/glb/svg/csv`, four fault notices each.
  UI: world window chrome has an "Analyse model" toggle and an "Export" group with one download toggle per format (`bim.measure.world.analyse`, `bim.measure.world.export.*`). The framework operation display shows the stage text and its cancel.
- New dependency `semio-framework-io-base64` (framework crate, same path as drawing uses) in the BIM package `Cargo.toml` for the glb download.

## Tests written (all unverified, none run)

- `I/📡️session/🧪️tests/🔬️unit` (9): monotonic progress and stepped == fresh; cancel keeps finished nodes, publishes nothing, next run reuses them (`computed == nodes - finished`, `reused == finished`);
  cancelled update unsettles (a gated diff is not served); unfinished run is an error value and the session recovers; gated run needs no step; synced == fresh after storey/wall/rename edits;
  unexplained or untrusted recorded diffs walk the plan; recorded diff computes only its neighbourhood; an overtaken run is dropped.
- `I/🗂️registry/🧪️tests/🔬️unit` (11): agrees with fresh; recorded mutation recomputes only what it touches; gated edit; same snapshot from memory; unexplained change; summed sequence and untrusted record;
  per-instance and close; re-entrant read; job stepping with monotonic progress; cancelled job reuse; probe room (30..48 m2 in the 8x6 demo, document untouched).
- `export-model/🧪️tests/🔬️unit` (7), `analyse-model/🧪️tests/🔬️unit` (2): every format equals its serializer's output; stepped == one-shot for every format; progress never decreases; cancel keeps nodes; unknown format refused; `handle` downloads the one-shot file; analyse settles the session.
- Existing suites updated: render tests (memo test removed, `with_inference`), viewer window tests, gltf scene/projection tests (local `build` helper), chrome test (two new world measure ids), `every_command()` and bridge tests (the two commands), IFC test imports.
- Language-agnostic + third-party: scenario `@id-export-ifc-stepped` in `S/🧪️tests/🏗️export-bim-1-ifc/🥒️.feature` (oracle: the existing IfcOpenShell handler registered for the new id in `🐍️.py`;
  subject: `🦀️.rs` cancels a job half-way, runs a second one on the same session, decodes its file and reports `projection::report`). The session-vs-fresh law itself has no third-party library to reproduce; it is covered transitively by the existing shapely/IfcOpenShell inference oracles, which now run through the session (IFC `projection`, svg `projection`, element-solids subjects).

## Commands and results

| Command | Result |
|---|---|
| `gate r11-w03-session -- cargo check ... -p semio-s-artifact-bim-model --lib` (check1..9) | fails in framework dependencies only (see Status); BIM crate never reached |
| `rustc -Zunpretty=normal --edition 2021 --cfg test <crate root>` | 0 errors (all modules and test files load) |
| `bun T/r3-f1-check-names.ts` | 19 problems, none in my directories |
| `cargo test --lib`, wasm32-wasip2 check, oracles, generators | NOT RUN (blocked) |

Re-run when `cargo check -p semio-s-artifact-bim-model --lib` compiles: `cargo test ... -p semio-s-artifact-bim-model --lib -- session:: registry:: export_model analyse_model chrome:: gltf::`,
then `--target wasm32-wasip2`, then the `export-bim-1-ifc` case at level quick. The generators (`r3-f1-gen-*`) need no re-run: no mutation, feature table or oracle row of theirs changed.

## Open items

1. **Build and test everything above** once the framework compiles; expect small compile fixes (I could only parse).
2. `export_schedule_csv::CsvJob` (peer w13) duplicates the stepping of `registry::Analysis`; it should adopt `Analysis` (I only repointed its import from the removed `crate::render::inference` to the registry). Its command is still missing from `every_command()` in `✏️editor/🧪️tests/🔬️unit` (the "covers every row" test fails for it until w13 adds it; I added mine).
3. Viewer: no progress/cancel UI. The viewer is structurally read-only (five window-config verbs on the bounded route) and renders synchronously from the shared session; a viewer job needs its own resumable factory and proofs. Not done.
4. Engine requests for z-incremental: none beyond what exists (`step_field_update` is used through the session). If `InferenceSession` ever keeps per-run state outside `SessionRun`, `epoch` in the session is where overtaking runs are rejected.
5. `S/🖌️render/🔮️inference/🧪️tests/🔬️unit` is an empty directory I could not remove (`Device or resource busy`, a process holds it); git does not track it.
6. `ModelInference::infer` remains the uncached reference; the `#[cfg(test)]` `compute_*` helpers can be replaced by a test kit later (mass edit of peers' test files was not safe while they work).
7. The stage texts of the progress previews are static en+de JSON in the command modules (the framework contract takes `&'static [u8]`), not `BimLabels` rows.
