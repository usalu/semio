# Compile-memory budget per worker class (2026-11-24)

Requirement: every compile unit fits the smallest worker (a 16 GiB Mac that runs several agents). The only per-crate gate was the 262144-byte source-closure boundary, which cannot see a crate that is small in source and large in monomorphization (`semio-s-artifact-stdio-semio`: 48 KB, 5 to 6 GB). This note records the budget that now sits beside it, the audit of the recorded history and the carving done.

## 1. Budget design

Schema first: `Policy.const.unitCeiling` of `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json` (the single source; the TypeScript and the Python oracle read it).

| field | value | meaning |
|---|---|---|
| `workerClass` | `16g` | the class the fleet is carved for |
| `classes.16g.hostMegabytes` | 16384 | the host the class stands for |
| `classes.16g.libraryMegabytes` | 1536 | peak RSS of any non-`cdylib` unit (lib, proc-macro, bin, test) |
| `classes.16g.finalMegabytes` | 4096 | peak RSS of a final `cdylib` link unit only |
| `classes.16g.finalBackend` | `serial` | the final allowance holds only with `-Z no-parallel-backend` (already appended to every granted cdylib final by `serial_backend` of the scheduler; the unit-ceiling test asserts the scheduler corpus agrees) |
| `carving` | closed list | units known to exceed their budget, each with its measured peak and the reduction it waits for; it may only shrink |

The peak 4096 MB per unit that existed before applied to everything; a library may no longer use the final allowance.

### Enforcement (three layers)
1. Law (`🧪️tests/🧢️unit-ceiling/🟦️.ts`, "Law: no unit of this machine's real ledger ..."): reads `<repo>/.cache/b/r/m/components/build-telemetry/units.jsonl`, keeps only fresh rows, and fails listing every unit above its budget that is not on the carving list (key, peak, kind, budget, EN text), and every carving entry that fits again ("remove it from the carving list"). Skipped when there is no ledger.
2. Build-time warning, observed peak: `unitCeilingAlarmV1` (component pass `recordCompletedUnit`, and `reportUnitCeilingV1` of the unit-cache wrapper) now applies the kind budget; one bilingual EN / DE line per unit key and build.
3. Build-time warning, cold prior: `coldBudgetWarningV1` is called by the broker `predict` for every library unit without fresh history whose cold estimate (`coldBaseBytes + coldBytesPerKloc * kloc`) is above the library budget and that is not carved; one bilingual line per key, never a refusal.

### Fresh rows
`freshUnitObservationsV1`: per key only rows that record `lines` and lie within `forecastSizeToleranceBasisPoints` (25 %) of the latest row (the same rule as the admission forecast). A row without `lines`, or one of a source that was split or has grown, measured another unit, so a stale 4.9 GB row of the pre-split `stdio_semio` or of the retired `members-*` group crates cannot fail the law.

### Tests (language-agnostic corpus + independent oracle)
`🧪️tests/🧢️unit-ceiling/`: `🔣️.json` (23 verdict cases incl. library/final boundaries, 9 freshness cases), `🧬️schema/🔣️.json` (closed corpus schema), `🐍️.py` (Python oracle, stdlib only, reads the policy from the schema), `🟦️.ts` (10 tests: schema closure, oracle equality for verdicts and fresh rows, kind and boundary laws, freshness laws, cold warning, final backend tie, bilingual lines, real-ledger law). `repo test-file`: 10 pass, 0 fail.

### Limits found while measuring (honest)
- A source-based cold prior does not predict these crates. On the 1397 library crates of the ledger with a known source dir, a macro-site or `async fn` density estimate has recall 1 of 21 at 100 to 400 false positives; the offenders are thin crates (5 to 15 KB of source in the hubs and test carves) over heavy generic instantiation. So the cold warning only fires on source size (about 22 kloc and up); the real gate is the first observed peak (layer 2) and the law.
- `lines` is the line count under the directory of the lib file, not of the mounted source closure. A crate whose mounts changed but whose lib dir did not (the hub composition crates) keeps its old rows "fresh" until the next build records a new row. Follow-up: record the closure bytes of `readRustSourceClosureV1` in the observation.
- Peak depends on the grant: `semio-s-artifact-stdio-txt-tests` was 2.64 GiB at a 25 s wall and 1.14 GiB at a 136 s wall (serial grant). The law uses the largest fresh row, i.e. the worst concurrency a 16 GiB worker may grant.

## 2. Audit (history of 2026-11-24, 10930 rows, 1500 unit keys)
Libraries above 1536 MB, largest fresh row (stale = no row since the source last changed, so the number is older than the current source). Closure = source-closure bytes of the wasm32 lib (budget 262144).

| crate | peak GiB | rows | closure B | status |
|---|---|---|---|---|
| semio-s-artifact-flow-flow-unit-tests | 3.92 (max 4.21) | fresh | 13789 | carving: split-tests |
| semio-hub-flow | 3.83 (max 4.09) | fresh | 11100 | split (below) |
| semio-hub-procedural | 3.64 | stale | 9328 | split (below) |
| semio-s-artifact-stdio-pdf-model | 2.90 | stale | 176150 | carving: split-families |
| semio-hub-raster | 2.86 | stale | 5189 | split (below) |
| semio-s-artifact-stdio-txt-tests | 2.64 | fresh | 77202 | carving: split-app-fleet |
| semio-s-artifact-stdio-binary-io-registry | 2.60 | fresh | 38084 | carving: split-app-fleet |
| semio-plugin-actor-input | 2.21 | fresh | 35573 | carving |
| semio-s-artifact-stdio-semio-presentation-member | 2.19 | fresh | 1490 | carving: split-store |
| semio-s-artifact-stdio-gltf-model-snapshot | 2.15 | stale | 158147 | carving |
| semio-s-artifact-raster-raster-editor-session | 2.13 | stale | 137510 | carving |
| semio-s-artifact-stdio-dwg-snapshot-objects | 2.11 (max 4.17) | fresh | 222301 | carving: split-families |
| semio-s-artifact-stdio-dwg-snapshot-document | 1.96 | fresh | 26260 | carving |
| semio-framework-artifact-flow-flow-model | 1.89 | fresh | 79198 | carving |
| semio-s-artifact-raster-raster-editor | 1.80 | stale | 261505 | carving |
| semio-s-artifact-stdio-semio-model-member | 1.80 | fresh | 1371 | carving: split-store |
| semio-s-artifact-stdio-semio-cad-member | 1.67 | fresh | 1337 | carving: split-store |
| semio-s-artifact-stdio-semio-graph-editor | 1.65 (max 1.84) | fresh | 92386 | carving |
| semio-s-artifact-stdio-semio-animation-member | 1.57 | fresh | 1439 | carving: split-store |
| semio-os-flow-host-core | 1.56 | fresh | 232500 | carving |
Finals above 4096 MB (before the serial back end): `semio-hub-raster-component` 5.92 GiB, `semio-hub-flow-component` 4.05 GiB, `semio-hub-cad-component` 6550 MB, `semio-hub-puzzle-component` 6148 MB (listed, action `serial-backend` / `thin-lto-closure`). Retired by the stdio-semio carve (history only): `stdio-semio` 6.02 GiB, `members-object-table` 2.65, `members-kit-model` 2.63, `members-animation-brep` 2.24, `members-text-video` 1.61.

### Root causes (what the audit shows)
1. App fleet closure (`dyn_enum_close! { enum XApps: PluginApp { Editor(VcsArtifactApp<EditorApp<..>>), Viewer(..) } }`): every variant instantiates the roughly 60-method `PluginApp` delegation, a quarter of them `async`, over the generic app machinery in the crate that closes the enum, about 1 to 1.9 GiB per variant. Hubs (`semio-hub-flow`, `-procedural` with 4 variants, `-raster`), and the artifact carve crates `*-tests`, `*-io-registry` that define the artifact `XApps` (`txt-tests`, `binary-io-registry`).
2. Single generic store (`space_members!` member crates, 1.6 to 2.2 GiB): one typed store per crate is the floor of the current macro.
3. Derive-heavy schema carves (`pdf-model`, `dwg-snapshot-objects`, `gltf-model-snapshot`, `raster-editor*`): size follows the number of `ToValue` / `FromValue` / `DslRecord` records in one crate; the families are separable.
4. Test aggregators (`flow-unit-tests`: 20 mounted command test modules plus a 21 KB context, instantiated as one unit).

## 3. Carving done

### App-fleet nesting for the three hubs (source edits applied, NOT yet compiled)
Applied by `split-hub-apps.py` (kept here). Each variant of the hub `dyn_enum_close!` moves into its own crate under `🌎️hub/🧩️compositions/<hub>/🗃️apps/<icon><slug>/`, a one-variant `XApps: PluginApp` enum; the hub enum now nests them (`FlowApps::FlowEditor(FlowEditorApps)`), and the hub implements `From<VcsArtifactApp<..>>` / `From<OriginalVcsApp<..>>` by hand because the plugin builder bounds on those types. The instantiation of the app machinery of a variant therefore happens in its own unit; the hub only delegates to concrete futures.
- flow: `semio-hub-flow-editor-app`, `semio-hub-flow-viewer-app`
- procedural: `semio-hub-procedural-generation2d-editor-app`, `-generation2d-viewer-app`, `-generation3d-editor-app`, `-generation3d-viewer-app`
- raster: `semio-hub-raster-editor-app`, `semio-hub-raster-viewer-app`
Touched: `🌎️hub/Cargo.toml` (members, workspace deps), `🌎️hub/Cargo.lock` (new packages, edges), `📋️build-owners.json`, the three hub manifests and roots, 10 new crates.

Verification status: the gated check (`repo cargo --store=shared --configuration=prototype --parallelism=1 -- check -p semio-hub-flow-editor-app -p semio-hub-flow-viewer-app -p semio-hub-flow --lib`) was running (689 of about 900 units, no error) and was stopped on the coordinator's order after the host ran out of memory. Not verified: compile success, the hub surface test (`FlowApps::FlowEditor(_)` still matches the outer enum) and the real peak. Resume with the same command, then procedural and raster, `memory_pressure` normal and at least 40 % free.

### Not carved (needs a design, listed in `carving`)
- `flow-unit-tests`: 20 mounted command test modules plus one 21 KB context in one unit; split by command group, each group with its own slim context.
- `txt-tests`, `binary-io-registry`: the artifact `XApps` enum shares its crate with the mounted editor and viewer sources; nest as for the hubs after the editor and viewer are separate crates.
- `pdf-model`: the families have no `crate::` imports but `super::` cycles; strongly connected groups are {object-model, document-metadata, navigation}, {content-operators, colour-and-graphics-state, xobjects}, then font-model, annotations-and-forms, page-model as leaves, plus the mounted sqlite, truetype and metrics owners; five crates behind a facade, with the `carve.owned` list and about 40 lock files updated.
- `dwg-snapshot-objects` (8 family modules, 222 KB), `gltf-model-snapshot`, `raster-editor*`: same family split.

## 4. Appliers (written, dry run by default, nothing applied or compiled)
`carve_lib.py` (shared), `apply-family-split.py <pdf|dwg-objects|gltf|raster-editor|raster-session> [--apply]`, `split-fleet-editor-viewer.py <txt|binary> [--apply]`, `split-hub-apps.py` (applied for the hubs). Each writes `📓️apply-<name>-report.md` / `📓️dry-run-fleet-<name>.md` (new crates, consumer files repointed, lock files, warnings, unresolved). `--apply` refuses while anything is unresolved. `DUMP=<dir>` writes only the new files for review.
Rule: no facade, no re-exports of moved modules; consumers are repointed to the owning crate and get its dependency; the root keeps what stays. The plan found real blockers and the specs were reduced accordingly: dwg families use only each other in a DAG (tables, associativity, metadata macros below); `raster-editor` splits only `base64_ingress` and `original_work` (`app`, `app_assembly`, `store_preparation` use `crate::config/panels/modes/commands`, which must move first); `raster-editor-session` leaves `scene_helpers` in the root (uses `crate::schema`).
`artifact_kind` cycle: the pure function moves to `<artifact>-kind` (below editor, viewer and the carve crate; it needs only the document schema constant); moved sources call it directly; binary needs no kind crate.
Unverified by compiler: generated roots, dependency pruning by identifier use, `#[macro_export]` of the dwg `metadata` macros, glob-import repoints for pdf, and the fact that derives may expand to `crate::` paths. The first gated check after the go decides.
