# WP-T13: Plugin Correctness Debt (Differential Adapters, Content-Addressed Ids, Lib-Test Debt)

Slice T13, session 13 (2026-09-26 19:0x). Coordinator = main chat. Predecessor: T12 (`📓️wp-t12.md` F9/F10, §S12-4, item 5).
Ports 8060–8069 / 6560–6569 (none used yet). Private cargo target `.tmp-ticket/wp-t13/target`. Captures `wp-t13/generated/`
(expendable); durable data `.🧬semio/🌐hub/s13-t13-*`. Landing rows: `📓️landing.md` § Session 13 Landing Window. Guest rebuild
requests: `wp-w3/requests/t13.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | F10: unwired differential cases + contract rule | **DONE for every non-norm case with an entry-point gap**: rules landed; **parity measured** wfc2d 30/30, grid2d (re-run pending), bitmap 21/21, wfc3d 31/31, grid3d 28/28, web-mercator 8/8, bitmap mount-contract 4/4; contract 06:29: `adapter-entry-point-missing` **0** outside norm (N1) | `.🧬semio/🌐hub/s13-t13-logs/parity-*.txt`, `breaches-0629.json` |
| 1b | F10b: JS-reader oracles never ran (no pipeline executor) | **platform executor landed; first reader-pipeline parity ever: 92/100 pairs agree** (glTF camera/skin/animation 7/8, asset 28/28, material 35/36; **obj material 4/4** after re-deriving its rows onto the committed pairs) — the 8 reds are real gltf-owner defects (F10b-1 lossy hand-written inverses, F10b-2 wrong committed create-material pair); left: bcf ×3, docx, gltf ♾️any (need committed after-documents per row: owner fixture generation) | `parity-2..6.txt`, §F10b, `generated/f10-census-3.txt` (non-norm: 0 entry gaps, 5 oracle-adapter gaps) |
| 2 | F9: content-addressed ids | **prepared, dry run 22 files / 0 problems** (`wp-t13/f9/content-id.py`: `store::content_id` + 23 sites + hashlib law); carriers (~1 095) regenerate at landing | §F9 |
| 3 | plugin lib-test debt | wfc-2d `mount_contract` gating fixed (landed); bitmap solve laws under load analysed → prepared plan (clock parameter + logical clock in correctness laws); F4 prepared (`wp-t13/f4-viewer-refusal.py`, dry run clean) | log 11:0x |
| 4 | `BatchOnlyPendingRewrite` census | strict law exists (`bun ./📜️script.ts verify interactivity commands`, V1): **4/1165 unreachable** at 11:1x — animate `exportVideoFromDeck` (host video capability slice, §AV) + space-home `bindSpaceFile`/`importSpace`/`deleteVirtualFileSystemNode` (IO-in-handle stubs → need an IO-owning job; space owner); architect/cad/space-studio all `Migrated` | `generated/census-commands-1.txt` |
| 5 | production `serde_json` | (a) wires LANDED; (b) re-scoped by census (puzzle 238 `json!`, infinite 50, …) → prepared plan (`dsl_value!` macro + per-crate codemod), next window; renderer (16 sites) = open item | log 11:0x |
| 6 | non-norm contract HIGHs | attributed + routed; F10 rows outside norm: entry-point 0, oracle-adapter 6 (F10b remainder) | `breaches-0629.json` |
| 7 | (coordinator 10:4x) chain fix: raster half-landed leaf | **PARKED, native green 10:53** (variant/use/pub mod removed; leaf files kept for the Codex team) | `check-raster-1.txt` |

### Log

- 19:08 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-t12.md`. Load 47, 4 rustc, 111 GiB free.
- 19:1x F10 census (`wp-t13/f10-census.ts`, `generated/f10-census-1.txt`, 509 cases): **15 cases carry adapter files without their
  language's entry point** (wfc 6: wfc2d, grid2d, bitmap, bitmap mount-contract, wfc3d, grid3d; norm 8: din18599 balance,
  en1997 compliance + mutate, en1994 compliance-oracle, en1996 mutate, en1990 mutate, din16798 mutate, din4108 mutate;
  surface web-mercator-tile-oracle) — 23 files — and **11 cases declare a JavaScript reader oracle with no TypeScript
  adapter** (bcf 3, docx 1, gltf 6, obj 1; their Rust adapters carry the reclassified cross-semio oracle).
- 19:2x coordinator: REBUILD START not before ~23:00; order F10 → items 4/5 → F9 (land only if compile-atomic by ~23:00).
- 19:3x platform rule landed in `🧪️test/🟦️.ts`: `ADAPTER_ENTRY_POINTS` (the one entry point per host language),
  `oracleImplementation` (shared with `⚖️parity/📋️orchestration`'s `oracleDecision`), contract breaches
  `adapter-entry-point-missing` and `oracle-adapter-missing` (both high). Platform law
  "a declared differential row whose adapter cannot run is a contract breach" **2/2 with the neighbouring law, 361 s**.
- 19:4x `law::vector::Vector<'a>` (was `&'static`) + `Leaves::read(ctx)` (the doc string's five URIs through the plan's
  fixtures); wfc bridges `<prefix>_mutation_report_json` in the 5 wfc mutation roots (`wp-t13/wfc-report-bridges.py`).
- 19:5x native `cargo check` of the 5 wfc crates with the report bridges EXIT 0 (17 min at load 84–105; captured before
  20:14 → re-measured below per preamble rule 24).
- 20:0x wfc cases wired: the 5 Python references gained `adapter()` (oracle role, Scenario Outline base ids `mutate` /
  `inverse` [+ `identity-round-trip` for bitmap and wfc3d], every standalone-replay law asserted per row); grid3d's
  reference had NO inverse — written from the vocabulary (resize also restores the per-axis cell sizes). The 5 Rust
  files became real SUBJECT adapters (`law::vector` over `<prefix>_mutation_report_json`; round trip through
  `<prefix>_snapshot_json_round_trip`, `wp-t13/wfc-round-trip-bridges.py`). The in-crate test modules that sat in the
  grid2d/grid3d case dirs moved into their mutation roots' `🔬️unit` files (`wp-t13/wfc-case-tests-relocate.py`: grid2d
  keeps its fixture-tree roster law, its two replay laws duplicated the per-kind fixture tests; grid3d's 5 roster laws
  moved verbatim with re-rooted `include_str!` paths). Python host `Context.doc_string()/doc_json()` (twin of the Rust
  runner's). Standalone replays still green (15/14/10/15/14 vectors).
- 20:0x **oracle phase** (`bun ./📜️script.ts oracle --case …`, Python host): wfc2d **30/30**, grid2d **28/28**, bitmap
  **21/21**, wfc3d **31/31**, grid3d **28/28** (captures `generated/oracle-<case>-1.txt`).
- 20:1x **contract finding**: `contract` now reports **1859 HIGH** (T12 measured 4 at 04:31 s12): **1714 in `📕️norm`** —
  a norm "Wave C" restructure (files 11:59–17:15 today: 567 `obsolete-testing-category` `🎫️fixtures`, 283
  `mutation-without-fixture`, 235 `manifest-only-mutation`, 209 `contribution-manifest-invalid`, 8 stub cases with
  untagged features); non-norm highs 35 (framework 27, raster 7, trinity F1). The 8 norm cases my census found are part of
  that in-flight rebuild → **excluded from F10** (not my scope, owner unknown), main messaged.
- 20:1x surface case `🕸️web-mercator-tile-oracle` wired: new pub `tiled_map::map_lod_band(span) -> (index, tile_z)`, pub
  `MAX_VISIBLE_TILE_REQUESTS` (the crate integration test now reads it instead of a copied 256); case `🦀️.rs` = SUBJECT
  adapter (8 scenarios: 3 vector sets through the production projection/windowing, LOD bands via `map_lod_band`, the 4
  camera laws in role through `MapHost`); `🐍️.py` gained `adapter()` (mercantile answers; spec vectors for LOD; stated laws).
  **Oracle phase 8/8** (84 s). Standalone mercantile check still PASS 47/47.
- 20:2x coordinator: norm's 8 unwired cases → slice N1. I own the non-norm HIGHs.
- 20:3x item 5a: wires editor builds `interactionSelect` args as `DslValue` (targets text through the in-repo
  `os_pack::json` writer), `optional_json_to_dsl` call gone from the crate, `serde_json` moved to `[dev-dependencies]`
  (tests keep it as the oracle); unit test asserts via `as_str()`.
- 20:4x wfc native check killed twice by me (rule 25: no rustc child > 15 min, lock convoy behind ~20 fleet cargos);
  relaunched batched: wfc ×5 + surface + wires `--lib --tests` (nohup, `generated/check-guest-4.txt`).
- 20:5x item 4 census on the tree: architect's 8 are ALREADY `Migrated` (runAnalysis/Report/Validation/search/import/
  export…); production `BatchOnlyPendingRewrite` left: cad 5, space-home 4, space engine (studio) 24 — all in LC's P8
  bundle now landing — and animate `exportVideoFromDeck` (1): it renders MP4 through a GPU rasterizer and writes files; a
  wasip2 guest has no GPU (its raster tier returns `RasterError::Adapter` by design) → needs a HOST video capability,
  not a reclassification (recorded as blocked-by-design).
- 20:5x NN (non-norm HIGHs 76): my rule's Rust entry regex rejected `-> semio_repo_test_host::Adapter` (17 repo cli/metrics/
  tree adapters) → accepts a qualified path now; re-census (`generated/f10-census-2.txt`): non-norm no-entry 3 (bitmap
  mount-contract) + no-oracle-adapter 11. The other 41: pixel-editing peer 32 (files 20:04–20:46), V1 4, ui event-feed 2,
  db storage 1, os schema-oracle 1, norm 1 → routed via main.

### AV. Host video-export capability — requirements for the slice that builds it (animate `exportVideoFromDeck`)

- **What the guest has.** `✏️s/🔌️plugins/🎞️animate/…/🎬️presentation/…/✏️editor/🎮️commands/🎥️export-video-from-deck/🦀️.rs`
  decodes a `PresentationScene` from `sceneJson` and calls `engine::export_video_from_scene(scene, output_dir)`: for every
  scene hash `compile_scene_to_assets` builds an `AnimateConfig` (Medium preset), `render_scene(.., [Mp4, LastFrame])`
  captures mobject frames and rasterizes each through `VelloRenderer` (`semio_framework_raster::SceneRasterizer`, headless
  wgpu + Vello), writes `scenes/<hash>/{mp4,last frame,scene.srt,sections}` with `std::fs`, and the command answers the
  bundle list as `Effect::DownloadMediaExport` (`animate-video-export.ops`). The frame capture (`CapturedFrame`:
  time + mobjects) and `build_vector_scene` are target-neutral; only rasterization + encoding + files are not.
- **What is missing in a guest.** Under `wasm32-wasip2` the raster tier returns `RasterError::Adapter` by design (WASI P2
  has no graphics API; `⚙️engine/🎥️video/🦀️.rs` wasip2 variant), and a guest has no host file system for `output_dir`.
  The command is classified `BatchOnlyPendingRewrite`, so the UI dispatcher refuses it (`interactive-job.not-ui-safe`);
  migrating the classification alone would ship a command that always faults.
- **What does not exist on any host.** No video encoder capability anywhere (searched TS/Rust: no `VideoEncoder`,
  `MediaRecorder`, WebCodecs, MP4 writer on a host path; ShellHost records `MediaRecorder` as a scope cut). stdio has an
  ISOBMFF/mp4 codec (`🗄️stdio/🗿️artifacts/🎥️mp4`) — a container reader/writer, not an encoder.
- **Shape of the fix (schema-first).** A host capability `media.video-render` (WIT + TS/Rust twins): the guest emits the
  deck's frame program (per scene: fps, duration, `VectorScene` per sample or the mobject timeline + camera) as a bounded,
  retained, cancellable job with progress; the host renders frames on its GPU (browser: WebGPU/OffscreenCanvas; native:
  the same `SceneRasterizer`) and encodes (browser WebCodecs → ISOBMFF via the stdio mp4 writer; native: first-party
  encoder or the mp4 writer over raw frames), then answers `DownloadMediaExport`. The command becomes `Migrated` with a
  real retained factory; the strict census law (item 4) turns green when it does.
- **04:58 resume after the overnight cut (preamble rule 28).** Every edit of mine survived and is in HEAD (22:00
  auto-commit); `git status` shows none of my files modified. Load 10, 0 rustc, 154 GiB free.
- 05:0x `tsc` over the 6 changed platform/adapter TS files: 0 errors in my code (remaining: peers' ui contract files,
  2 pre-existing test-file errors at test-platform:1445 / mutation-fixtures:39). Platform laws **3/3** (the two
  contract-rule laws + `subjectFeaturesFor`), 216 s.
- 05:0x landing gate (`wp-t13/check-landing.sh`, detached via `w2-detach.py`, log `.🧬semio/🌐hub/s13-t13-logs/`): run 1
  failed in `semio-framework-plugin` (`EmitWire` not found, 4×) — a peer's live edit of the plugin crate (file saved 05:11:23,
  after the failure at 05:10:54); stopped mine, relaunched 05:18 (run 2).
- 05:1x `wp-w3/requests/t13.txt` written (7 crates, none changes a descriptor or the ABI).
- 05:1x F9 census on the tree: 24 `DefaultHasher` id-minting sites (list in `generated/f9-minting-sites-1.txt`; norm's
  din18599 climate belongs to N1; en1990 qK no longer exists after norm's Wave C). Carrier census running.

### F10b. The 11 JavaScript-reader cases — why they never ran, and the plan

- **Measured.** bcf ×3 (`jszip-bcf-2-1-mutate-reader`), docx ×1 (`jszip-docx-ecma-376-mutate-reader`), gltf ×6
  (`three-gltf-2-0-mutate-reader`), obj ×1 (`three-obj-3-0-document-reader`) declare a JavaScript READER oracle whose
  registered rationale is "the expected state is COMMITTED as the `after` half of a byte-reproducible fixture; the reader
  parses both sides". The registry wires that as a comparison PIPELINE of probes (`gltf-2-0-three-compare-v1`: `gltf-import`
  + `gltf-compare` over roles `expected-gltf`/`actual-gltf`; `bcf-2-1-jszip-compare-v1`; `docx-ecma-376-jszip-compare-v1`;
  `obj-3-0-document-compare-v1`), all probes `bun <owner>/🔬️probes/📜️script.ts <probe> --input … --input …`, all
  `qualified`. **Nothing in the platform executes a pipeline**: `evaluatePipeline` has no caller; no adapter emits an
  `expected-*`/`actual-*` artifact. So the TS "missing adapter" is only the visible half.
- **Feature shape.** gltf's 6 features already start every row from a committed pair (`shared://<fixture>/⬅️before.gltf`
  with `➡️after.gltf` beside it) but compare under `semantic-gltf-v1` (Rust projection, no pipeline). bcf/docx rows apply
  parameters to one real input that has NO committed after; their committed pairs (`🧫️fixtures/<kind>-applied/`) start from a
  different before (md5 6c1a… vs the feature's 8aa4…) and cover 10 of 14 bcf kinds.
- **Plan (test platform + stdio test files only; nothing guest-linked).**
  1. Platform: `executePipeline(repoRoot, pipeline, probes, roleArtifacts, signal, progress)` — resolves every stage input
     role to an artifact path (oracle result → `expected-*`, subject result → `actual-*`; an unresolved role is a failed
     stage, never skipped), runs the probe's registered `command` + `--input <path>` per input, parses/validates the
     `ProbeReport` (`probeReportProblems`), then `evaluatePipeline`. `runPhases`: a case whose profile names a pipeline
     gets its parity verdict from the pipeline (per scenario pair), not from a projection diff. Law: a pipeline stage whose
     role has no artifact fails; a report that fails validation fails; equal/unequal pairs over two committed fixtures.
  2. Contract: `pipeline-role-unproduced` — a case under a pipeline profile whose adapters register no artifact for a role
     the pipeline reads (static, from the adapter source) is a breach.
  3. Oracle adapters (TS, 11 cases): per row, answer the committed `➡️after.<ext>` as artifact `expected-<ext>` (the reader
     oracle computes nothing, by its registered rationale); Rust subject adapters add `.artifact("actual-<ext>", …)` for
     the bytes they already produce.
  4. Features: gltf ×6 switch to `@comparison-semantic-gltf-reader-v1` (the pipeline profile the oracle declares);
     bcf/docx/obj rows re-pointed at their committed pairs (one row per committed pair; kinds without a pair keep their
     cross-semio rows under the sibling oracle and are listed as uncovered by the reader).
- 05:2x **F10b platform half LANDED (test platform only, not guest-linked):** `executePipeline` (runs each stage's
  registered probe `command` with `--input <path>` per declared role, whole stdout = one validated `ProbeReport`,
  `evaluatePipeline`; cancellation between stages, per-stage progress) + `pipelineRoleArtifacts` (oracle + subject
  artifacts merged by role, a role produced twice is a problem) in `🧪️test/🟦️.ts`; `runPhases` gives a pipeline profile's
  parity verdict from the pipeline per scenario pair (verdict JSON in `diffs/<testId>.pipeline.json`), never a projection
  diff. Law "a pipeline runs its probes over the pair's artifacts by role, and fails on an unproduced role or an invalid
  report" **1/1** (161 s); tsc clean on my files.
- 05:2x F10b findings measured on the registry (never visible because no pipeline ever ran): (1) every `*-import` stage
  asserts `bothImport` but `gltf-import` reads only its first input and reports `parsed` (the others to check);
  (2) the bcf and docx reader oracles do not declare their own pipeline profiles (`semantic-bcf-jszip-v1`,
  `semantic-docx-ecma-376-jszip-v1`) — they declare the non-pipeline `semantic-bcf-v1` / `…-mutate-v1`; (3) no profile
  points at `obj-3-0-document-compare-v1`, whose roles are `expected-before-obj`/`expected-after-obj`; (4) gltf features
  name only `⬅️before.gltf`, so the committed `➡️after.gltf` is not in their plans (a handler cannot read it). The stdio
  half (probes, registry profiles, feature steps, 11 TS oracle adapters, `actual-*` artifacts from the Rust subjects) is
  a prepared patch set (stdio = guest-linked tree → after PUBLISH DONE).
- 05:2x F9 carriers: 1 095 committed files carry an id of the 24 minting prefixes (architect 539, remodel 300, note 83,
  block 82, process 44, norm 13, sequence 7, …; `generated/f9-carriers-1.txt`; the regex's generic prefixes `mesh-`,
  `document-`, `catalog-` may over-count).
- 05:2x wfc-2d: `🧪️tests/🧩️mount-contract` (a test module, no feature) was mounted `#[cfg(test)]` although it reads the
  `component-app-assembly` surfaces → gated like its siblings (lib tests without the feature failed on it, pre-existing).
- 05:3x run 2 of the gate: native with `component-app-assembly` EXIT 0 (wfc ×5 `--lib --tests`), stdio test-oracle crate
  EXIT 0; without the feature wfc-2d's lib tests failed on the pre-existing `mount_contract` gating → fixed (above);
  `check-native-3` (`--keep-going`, wfc ×5 + surface + wires `--lib --tests`) **EXIT 0 05:30** (warnings present = type-checked).
  wasm32 run 2 hit H12's in-flight `AppFactory.codec` (builder initializers, fixed by H12 05:35) → `check-wasm-3`:
  **wfc ×5 (component-app-assembly) EXIT 0 05:40, wires + surface EXIT 0 05:43.** Landing row written; roll-call sent.
- 05:4x stdio production-input fixes before REBUILD START (TS/JSON only, measured with bun): `gltf-import` admits every
  input and reports `bothImport` + per-input counts (committed camera pair → `ok true [true, true]`; a missing file →
  `bothImport false`); bcf/docx reader oracles declare `semantic-bcf-jszip-v1` / `semantic-docx-ecma-376-jszip-v1`; obj:
  new `obj-document-import` probe (three OBJLoader over every input; committed set-usemtl pair → `bothImport true`),
  `obj-3-0-document-compare-v1` roles `expected-obj`/`actual-obj` with the import stage, profile `semantic-obj-document-v1`
  now names the pipeline. Registry reload: 0 contribution problems, all 4 reader pipelines resolve registered probes.
- 05:4x F10b pilot (gltf camera): test platform `committedArtifact(ctx, leaf, role, mediaType)` (a reader oracle's
  answer = the one committed fixture its steps name); camera case → `@comparison-semantic-gltf-reader-v1`, steps name
  the committed `➡️after.gltf`, new TS oracle adapter (`mutate` → after, `inverse` → before as `expected-gltf`), Rust
  subject emits `actual-gltf`. tsc clean. Parity run (camera + wfc2d) launched detached (`parity-camera-1.txt`).

### F9. Content-addressed composed-child ids — prepared plan (next cycle)

- **Census (05:1x, on the tree):** 24 `DefaultHasher` id-minting sites (`generated/f9-minting-sites-1.txt`): child handles
  of trinity jack, process3d (steps-flow, brep), dag, sequence, writer, animate (presentation, animation), gisterrain
  mesh, imperative (flow, text), note text, shooting emblem, layout background drawing, puzzle5d kind catalogs, playbook
  (flow, document), lowpoly mesh, block3d + curation catalogs; scene ids of forms, architect benchmarks + knowledge;
  remodel mesh io; norm din18599 climate (N1's). Inputs differ per site: most hash the canonical JSON of the child
  snapshot, some hash plain keys (gisterrain `content_key`, lowpoly `object_id`+`mesh_json`, remodel chunk list,
  process3d brep `slug`+text) — the patch hashes exactly the bytes each site hashes today, only through the specified
  function. `block3d_world_fit_revision` and other in-memory revisions are NOT persisted ids and stay out of scope.
- **Carriers:** 1 095 committed files carry an id of those prefixes (`generated/f9-carriers-1.txt`).
- **Patch:** (1) kernel `store::content_id(prefix, bytes) -> String` = `prefix-` + first 16 lowercase hex of SHA-256
  (`semio_framework_hash::sha256_hex`), stated in the composition schema's `childId` description; (2) the 23 non-norm sites
  call it (per-site codemod, anchors = today's `format!` lines); (3) law in the kernel: ids equal a third-party SHA-256
  (Python `hashlib` vectors committed as fixture) and are stable across processes; law in the contract (TS): no
  `DefaultHasher` in a function whose result reaches `ArtifactChild::new`/`ArtifactRef.artifact_id`; (4) carriers
  regenerated by each owner's own fixture generator / `*_REGEN_*` test switch after the code change (per-crate runbook in
  the patch dir), then every touched crate's lib tests. Needs a guest rebuild → lands in the next landing window.

### 5b. `optional_json_to_dsl` removal — prepared plan (next cycle)

- 74 call sites in 50 files: ~17 plugin crates + infinite world + plugin crate + 16 renderer sites. Shape: each app
  wraps `ActionDescriptor { args: optional_json_to_dsl(Some(json!({…}))) }`. Patch: a first-party `dsl_value!` macro in
  the value crate (json!-compatible object/array/literal grammar over `ToValue`) + codemod `json!(` → `dsl_value!(` at
  the argument sites, `optional_json_to_dsl` deleted, `serde_json` moved to `[dev-dependencies]` where no other runtime
  use remains. **Open item (routed by main after the publish): renderer `serde_json` runtime dependency — 16 sites in the
  wgpu Shell/Scenes/Interpreter keep an explicit `DslValue::from(serde_json::Value)` until the renderer's own serde_json
  debt is paid.**
- 05:58 **first pipeline parity ever executed by the platform** (camera pilot): the stages ran and produced real reports
  (4/24) — and exposed two platform defects: (1) every host wrote a role's artifact to `<artifactDir>/<role>/<file>`, so
  all scenarios of a case overwrote one file and each pair compared the LAST scenario's files → all five hosts (TS, Rust,
  Python, Go, .NET) now write `<artifactDir>/<scenario id>/<role>/<file>` (tsc clean, `go vet` clean, `dotnet build`
  0 warnings / 0 errors); (2) the TS oracle adapter was also dispatched as a SUBJECT (the gltf owner ships a TS schema
  package) and errored per row → `subjectImplementations` drops the oracle-language adapter that registers no subject
  handler (`adapterRegistersSubject`, read from the source like the entry point). Pipeline diff files now carry the
  verdict + the role→artifact map + every stage report.
- 06:11 **wfc2d parity 30/30, 60/60 passed** (Rust subject over the production codec + `law::vector` vs the Python
  reference). Chain 2 launched: camera (re-run), grid2d, bitmap, wfc3d, grid3d, web-mercator, mount-contract (`parity-2.txt`).
- 06:19 **camera re-run: 16/16 executed, pipeline parity 7/8 through three's GLTFLoader** (`parity-2.txt`). The one red is
  a REAL finding the old `semantic-gltf-v1` projection (which ignores cameras) could not see: `inverse-delete-camera`
  restores a camera without `name` ("PerspectiveCamera"), `aspectRatio` (1.5) and `zfar` (100), and leaves
  `nodes[3].camera` unbound (three: `sceneCameraCount` 1 → 0). Cause: the case's hand-written `inverse_spec`
  (`create-camera` with only `yfov`/`znear` at position 0) is lossy — it is not the production mutation's own inverse.
  Finding **F10b-1** for the gltf owner: the subject half should replay the production `inverse()` (as the wfc cases now
  do through `law::vector`), and the delete-camera inverse must re-bind the node.
- 06:2x F10b stdio-half applied for the other 4 committed-pair glTF cases (animation, skin, material, asset:
  `wp-t13/f10b/gltf-reader-cases.py --write`, 🧪️tests files only — outside the `production` nx inputs): feature → reader
  pipeline profile + step naming the committed after, TS oracle adapter, Rust subject `actual-gltf`. Every row's committed
  pair verified present (material 36, asset 28, animation 8, skin 8, camera 8 rows). tsc clean. Parity: after the publish
  (no builds after REBUILD START). **Remaining F10b (owner-level, documented):** gltf `♾️any` (7 rows on one base `.glb`,
  no committed after per row); obj material (committed pairs start from `base.mtl`/`usemtl red`, the rows and the
  hand-written `inverse_specs` from pattern-sphere — rows, params and inverses must be re-derived from the pairs; the
  adapter also registers an `identity-round-trip` the feature does not declare); bcf ×3 and docx ×1 (pairs' befores
  differ from the rows' input: bcf 6c1a… vs 8aa4…, docx 3 distinct befores vs 3abf…; bcf pairs cover 10 of 14 kinds).
- 06:2x F4 prepared (`wp-t13/f4-viewer-refusal.py`, dry run clean): `assert_viewer_never_mutates` counts a refusal as
  "emits nothing" instead of `expect`ing success (plugin crate → next landing window; H12 is editing the same file now).
- 06:30 parity chain 2: **bitmap 42/42 parity 21/21, wfc3d 62/62 31/31, grid3d 56/56 28/28, web-mercator 16/16 8/8,
  bitmap mount-contract 8/8 4/4** (all EXIT 0). grid2d: oracle 28/28 but the Rust subject host did not compile — grid2d
  (and grid3d) declared no `oracleHostPackages` entry for the stdio law crate → added to both manifests (06:26, before
  grid3d's run, which then passed); grid2d re-run in chain 3 with the 4 newly wired glTF cases.
- 06:29 repository contract (`contract-1.txt`, `breaches-0629.json`): HIGH 2 303, of which non-norm 150 (was 76 at 20:52;
  growth = peers' in-flight test layouts: inline-test-body 56, production-fixture-dependency 23, …). F10 rows outside
  norm: `adapter-entry-point-missing` **0**, `oracle-adapter-missing` **6** (the documented F10b remainder).
- 06:34 parity chain 3 (before the second cut): **grid2d 56/56 parity 28/28** (after the host-package fix), glTF reader
  pipeline **asset 56/56 parity 28/28**, **skin 7/8**, **animation 7/8** (each: the one inverse row whose hand-written
  `inverse_spec` is lossy, same class as camera's F10b-1), **material 36 executed, parity 0/0** (the TS oracle half did not
  pair — to diagnose; `parity-3.txt`). The fleet was then cut (usage limit ~06:35).
- **10:4x chain fix (coordinator, rule 30, peer code touched):** chain b3 failed on `semio-s-artifact-raster-raster`: a
  Codex raster team's half-landed leaf `🧬️mutations/🎛️change-layer-adjustment-parameter/` (untracked, 01:55–01:57; no
  leaf `🔣️.json` → derive "MutationLeaf source authority failed"; 6 E0004 in the binary/text codecs). Measured before
  deciding: the diff layer never applies `RasterLayerPatch.adjustment_parameter` (`apply_layer_patch`,
  `validate_layer_patch` and the coalescer ignore it), so completing the codec arms would ship a phantom edit; the leaf
  also needs a fail-closed owned-map apply in the retained binary path. → **PARKED**: removed only the `RasterMutation`
  variant + its `use` (`🧬️mutations/🦀️.rs`) and the `#[path]`/`pub mod change_layer_adjustment_parameter` lines (raster
  `🦀️.rs`); backups `wp-t13/raster-*-before-park.rs.txt`; the leaf files stay for the Codex team (no other reference to
  the variant exists). Native check queued through the native mutex (`check-raster-1.txt`).
- 10:53 **raster native `cargo check -p semio-s-artifact-raster-raster --lib --tests` EXIT 0** (native mutex, build-fleet-b,
  crate re-checked: 185 warning lines, 47 s; `check-raster-1.txt`). The crate declares no `component-app-assembly` feature
  (my second invocation refused that flag — nothing to check there). wasm32 left to the chain. Main told to relaunch.
- 10:5x glTF material subject adapter drift fixed (test file): `change_material_alpha_mode::apply` /
  `change_material_double_sided::apply` became `apply(&mut GltfSnapshot, payload) -> Result<()>`; the case still called the
  old `apply(payload, before) -> Result<GltfSnapshot>` (the host never compiled — which is why chain 3 paired 0/0). The
  case now applies to a clone. Parity re-run queued through the native mutex (`parity-4.txt`).
- 10:5x **F9 patch prepared** (`wp-t13/f9/content-id.py`, dry run **22 files / 0 problems**): `store::content_id(prefix,
  bytes)` (SHA-256, 16 hex) in the kernel store beside `ArtifactChild`; the 23 non-norm minting sites rewritten to it
  (bytes = exactly what each hashed; multi-field sites join with U+001F; process3d's `steps-flow` stops going through
  `serde_json`); per-function removal of the now-dead `use std::hash`; law `content_id_is_the_specified_sha256_prefix`
  in the store's `🪪️artifact-addressing` case against `contentIds` vectors computed by Python `hashlib`
  (`wp-t13/f9/content-id-vectors.json`; added to the existing fixture — no new taxonomy member). Every touched crate has
  the `store`/`dsl` aliases the new lines use. Still to do at landing: regenerate the ~1 095 carriers per owner (their
  fixture generators / `*_REGEN_*` switches), then each crate's lib tests.
- 11:0x item 5b re-scoped by measurement: removing `optional_json_to_dsl` is not a 74-line swap — the per-app wrappers
  (`fn <app>_action(action, args: Option<serde_json::Value>)`, infinite's world3d `…_measures` closures) are FED by
  `json!` at their call sites: production `json!` uses puzzle 238, infinite 50, wfc 38, layout 37, gis 26, flow 8,
  procedural 8, lowpoly 5; files still importing `serde_json` in production: puzzle 105, gis 27, flow 14, procedural 14,
  layout 13, lowpoly 13, cad 11, wfc 11, infinite 9, note 7, process 5. Plan for the next window: a first-party
  `dsl_value!` macro (json!-grammar tt-muncher over `ToValue`, exported from `semio_framework` beside the action bus),
  wrapper signatures → `Option<DslValue>`, codemod `json!(`→`dsl_value!(` per crate with a `cargo check` after each crate,
  `serde_json` → `[dev-dependencies]` where nothing else remains, `optional_json_to_dsl` deleted; renderer (16 sites)
  stays the routed open item.
- 11:05 **glTF material parity 72/72 executed, pipeline parity 35/36** (`parity-4.txt`). The red is a COMMITTED-FIXTURE
  defect the reader pipeline exposed (finding **F10b-2**, gltf owner): `create-material-applied/after.gltf` inserts the new
  material at position 1 but leaves both mesh primitives on `material: 1` — they now point at the NEW material instead of
  `matB`, i.e. the fixture changes what the meshes render — and it names the material `createdMaterial` although the row
  passes only `{"position":1}`. The production mutation remaps references ≥ 1 to 2 (as its own cross-semio oracle and the
  §5.x insert rule require). Fix = regenerate that pair with the reference remap and no invented name.
- **F10b totals (reader pipeline, first time ever executed):** camera 7/8, skin 7/8, animation 7/8 (each the lossy
  hand-written `inverse_spec`, F10b-1 class), asset 28/28, material 35/36 (F10b-2). 88/96 pairs agree through three's
  GLTFLoader; all 8 disagreements are real owner defects, none a harness artefact.
- Item 3 (wfc bitmap solve laws red only under load) analysed: `solve_with_job` drives a `BatchJobSession` with
  `now_us: default_now_us` and `InteractiveStage::BackgroundStep`, so the runtime's deliberate wall-clock quarantine
  (`SUSTAINED_OVERRUN_QUARANTINE_STEPS = 4` × 8 ms, `⏱️trace/🦀️.rs` — documented why not CPU time: wasm has no thread CPU
  clock) terminates a correct solve when the machine deschedules the test thread 4 steps in a row (load 34–103).
  Root fix (prepared plan, guest crates → next window): the five wfc `solve_with_job`s take the clock as a parameter; the
  correctness laws drive them with a logical clock (advances 1 µs per read), production keeps `default_now_us`, and one
  separate timing law keeps the real clock. Not "fixed" now (rebuild freeze).
- 11:1x item 4 measured with the strict law that already exists (V1's `bun ./📜️script.ts verify interactivity commands`,
  acceptance check `command-reachability`): **4/1165 production commands unreachable** — animate `exportVideoFromDeck`
  (§AV) and space-home `bindSpaceFile`, `importSpace`, `deleteVirtualFileSystemNode` (IO-in-`handle` stubs; need an
  IO-owning job — space owner, after the rebuild). The law is strict (no allow-list) and stays red until those exist.
- 11:1x F10b obj material re-derived onto its committed pairs (stdio 🧪️tests files; backups in `wp-t13/f10b/`): rows now
  start from `shared://<pair>/⬅️before.obj` and name `➡️after.obj`; params = the pairs' own difference (`other.mtl`,
  `usemtl blue`); the Rust adapter is subject-only (production `apply_obj_mutation`, inverse = re-`set-*` of the
  before-document's own value, `actual-obj` artifact) — its cross-semio oracle handlers and the undeclared
  `identity-round-trip` registration are gone; new TS oracle adapter (committed fixture as `expected-obj`). tsc clean;
  parity queued (`parity-5.txt`).
- 11:12 **obj material parity 8/8 executed, pipeline 4/4** (`parity-6.txt`; first run 2/4: the inverse rows refused
  "byte pass-through" because the committed pairs ARE the encoder's canonical form — the guard now applies to forward rows
  only). F10 census 11:1x (`generated/f10-census-3.txt`): outside norm **0** entry-point gaps, **5** oracle-adapter gaps
  (bcf ×3, docx, gltf ♾️any). Those five need committed after-documents per row before a reader can judge them: bcf/docx
  rows mutate one real input whose results were never committed (their committed pairs start from other befores and cover
  10/14 bcf kinds); gltf ♾️any's 7 rows (bind/unbind node-child and scene-root, create-scene, material alpha/sides) run on
  one base `.glb`, and its committed vectors are JSON quintets, not documents. Plan for the owners: generate the pairs with
  the subsets' own generators (`🏭️generator/📜️script.ts generate --only <kind>-applied`, the glTF one already writes
  three-exported pairs) and re-point the rows exactly as done for obj/gltf (`wp-t13/f10b/gltf-reader-cases.py` pattern).

- 14:1x LANDING WINDOW 2 (coordinator 14:2x, extended to 15:45 after the 14:20–14:50 usage cut). F9 and F4 applied
  (`f9/content-id.py --write` 22 files / 0 problems; `f4-viewer-refusal.py --write`); wfc solve-law clock written as
  `wp-t13/wfc-solve-clock.py` (dry run 24 files / 0 problems, applied): job `logical_now_us()` (thread-local, +1 µs per
  read), the five wfc inference modules split `solve_with_job(s)` → `solve_with_clock(s, default_now_us)` + the new
  `solve_with_clock(s, now_us)`, and every `solve_with_job(` call under `🧪️tests` (19 test files) drives
  `solve_with_clock(…, semio_framework_job::logical_now_us)` — except the one timing law (bitmap
  `a_whole_solve_grant_settles_the_genesis_inference_in_a_bounded_state_walk`, real clock kept). New job law
  `the_logical_clock_advances_one_microsecond_per_read_per_thread` (clock-stride tests). Why the logical clock cannot
  quarantine: `ClockStride` calibrates to 4 096 calls per real read when one read costs 1 µs, so a step's measured span
  is its read count, far below the 8 ms ceiling; steps stay fuel-bounded and deterministic.
- 14:5x **F9 un-landed, stays prepared** (measured reason): the committed carriers pin today's `DefaultHasher` ids —
  e.g. block3d `🧪️recolors-door-vortex-kind/🦀️.rs:40` asserts `"catalog-a602bbe51a39cd44"`; 1 045 of the 1 095
  carriers live under `🧪️tests`/`🧫️fixtures` (architect 539, remodel 300, note 83, block 82, process 44, …). Landing the
  ids without regenerating them would turn those laws red → carrier regeneration (owner generators) is the precondition.
  Reverted hunk-precisely by `wp-t13/f9/content-id-revert.py` (only hunks touching the id minting; peer hunks in layout
  (35 lines) and store (16 lines) untouched); `content-id.py` dry run afterwards: **22 files, 0 problems** (re-runnable).
  My first combined check (14:55, with F9) was stopped by me before the revert (own pids 22401/22403/22413/22414).
- 15:01 gate relaunched, reduced to F4 + clock (`wp-t13/check-lw2.sh` → `generated/lw2-check-2.log`): native lane
  (job, plugin, wfc ×5 `--lib --tests`; wfc ×5 `component-app-assembly`; job clock law; wfc ×5 `--lib inferences`),
  then the wasm32 fast gate. Queue position 7 on the native lane at launch.

### Processes

None of mine is running (11:1x). All builds/parity runs were detached via `wp-w2/w2-detach.py` and ended on their own;
two lock-convoy cargos of mine were stopped by me (rule 25). No hub, serve or browser was started; ports 8060–8069 /
6560–6569 unused.

### Files changed (T13, session 13)

- Test platform (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`): `🟦️.ts` (ADAPTER_ENTRY_POINTS, ADAPTER_SUBJECT_REGISTRATION,
  adapterRegistersSubject, oracleImplementation, contract rules, SubjectFeatures + subjectFeaturesFor, pipelineRoleArtifacts,
  executePipeline, committedArtifact, per-scenario artifact paths), `🧬️schema/🔣️.json` (SubjectFeatures), `⚖️parity/📋️orchestration/🟦️.ts`
  (pipeline parity, subject selection), `🖥️host/🏗️materialization/🟦️.ts` (subject features), `🖥️host/🐍️.py` (doc_string/doc_json,
  artifact path), `🏃️runner/🦀️.rs`, `📦️packages/🐹️go/🐹️.go`, `🖥️host/🔷️.cs` (artifact path), `🧪️tests/🧪️test-platform/🟦️.ts` (4 laws),
  `🧪️tests/🧬️mutation-fixtures/🟦️.ts` (literal).
- stdio test oracle: `🗄️stdio/🔮️oracles/⚖️law/🧬️vector/🦀️.rs` (+ unit test).
- wfc: 5 mutation roots (report bridges, 2 round-trip bridges), 5 mutate cases (Rust + Python adapters), bitmap mount-contract
  (Rust/Py/TS, feature, `subjectFeatures`), grid2d/grid3d/bitmap unit tests (moved laws), grid2d/grid3d/2d/bitmap crate roots
  (test mounts), grid2d/grid3d oracle manifests (host package).
- surface: `🗺️tiled-map/🦀️.rs` (map_lod_band, pub budget), `🗺️tiled-map/🧪️tests/🕸️web-mercator-tile-oracle/{🦀️.rs,🐍️.py}`,
  `🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs`.
- wires: `✏️editor/🦀️.rs`, its unit test, `📦️packages/🦀️rust/Cargo.toml`.
- stdio reader oracles: gltf `♾️any/🔬️probes/📜️script.ts` (gltf-import); bcf markup + docx base `🔮️oracles/🔣️.json` (pipeline profiles);
  obj geometry `🔬️probes/📜️script.ts` + `🔮️oracles/🔣️.json` (obj-document-import, pipeline roles, profile); glTF camera/animation/
  skin/material/asset cases (`🥒️.feature`, `🦀️.rs`, new `🟦️.ts`); obj material case (`🥒️.feature`, `🦀️.rs`, new `🟦️.ts`).
- raster (chain fix, peer code): `🖨️raster/🦀️.rs` (2 lines), `…/🧬️mutations/🦀️.rs` (2 lines).
- Ticket inputs `wp-t13/`: f10-census.ts, f10-oracles.ts, f10-reader-cases.ts, f10-profiles.ts, pipelines.ts, probe-commands.ts,
  registry-problems.ts, wfc-report-bridges.py, wfc-round-trip-bridges.py, wfc-rust-adapters.py, wfc-case-tests-relocate.py,
  f10b/gltf-reader-cases.py, f9/content-id.py (+ vectors), f4-viewer-refusal.py, check-*.sh, parity-*.sh, contract.sh,
  tsconfig.t13.json; backups raster-*-before-park.rs.txt, f10b/obj-material-*-before.*.
