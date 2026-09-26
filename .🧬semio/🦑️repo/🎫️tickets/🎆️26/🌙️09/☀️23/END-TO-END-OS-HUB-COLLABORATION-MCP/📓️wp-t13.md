# WP-T13: Plugin Correctness Debt (Differential Adapters, Content-Addressed Ids, Lib-Test Debt)

Slice T13, session 13 (2026-09-26 19:0x). Coordinator = main chat. Predecessor: T12 (`📓️wp-t12.md` F9/F10, §S12-4, item 5).
Ports 8060–8069 / 6560–6569 (none used yet). Private cargo target `.tmp-ticket/wp-t13/target`. Captures `wp-t13/generated/`
(expendable); durable data `.🧬semio/🌐hub/s13-t13-*`. Landing rows: `📓️landing.md` § Session 13 Landing Window. Guest rebuild
requests: `wp-w3/requests/t13.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | F10: unwired differential cases (7 missing `adapter()`, 11 missing TS adapters) + contract rule | in progress | — |
| 2 | F9: content-addressed composed-child ids (`hash::content_id`) + carriers + law | pending | — |
| 3 | plugin lib-test debt (wfc bitmap solve laws under load, F4 `assert_viewer_never_mutates`, other `--lib` reds) | pending | — |
| 4 | (coordinator 19:2x) `BatchOnlyPendingRewrite` census (42: architect 8, space-studio 24, cad 5, home 4, animate 1) → every command dispatchable + census law; after LC's P8 landing | pending | — |
| 5 | (coordinator 19:2x) production `serde_json` runtime deps (wires editor, `🎯️action-bus::optional_json_to_dsl`) → in-repo DSL/JSON path | (a) wires applied, check running; (b) prepare patch (74 sites / 50 files), land guest part only if green by ~23:00 | — |
| 6 | (coordinator 20:4x) the non-norm contract HIGHs (76 at 20:52) | attributed (§NN): 17 were my rule's false positives (fixed), 14 my F10 set, 4 T12 leftovers; 41 routed to main (in-flight peers) | `generated/non-norm-highs-1.txt` |

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
