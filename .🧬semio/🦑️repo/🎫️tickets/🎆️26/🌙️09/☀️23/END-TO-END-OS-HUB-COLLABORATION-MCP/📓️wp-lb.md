# WP-LB — Landing Window: Plugin Open Kinds, Capability Descriptions, Oracles

Slice LB, session 13 (2026-09-26 19:0x). Coordinator = main chat. Ports 8110–8119 / 6610–6619 (none used).
Inputs: T12 (`📓️wp-t12.md` §S12-1, §S12-4 item 5, rows 1b/1c/1d/3b/5; `wp-t12/`), D1 (`📓️wp-d1.md`; `wp-d1/`).
Captures: `wp-lb/generated/` (expendable). Binaries: `CARGO_TARGET_DIR=.tmp-ticket/wp-lb/target`.
Landing rows: `📓️landing.md` § Session 13 Landing Window. Guest rebuild requests: `wp-w3/requests/lb.txt`.

## Session 13

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | T12 `postpublish-open-kinds.py` (stdio `s.stdio.<x>` kinds, editor declarations, gis terrain kind, hub fence, census law, bootstrap, rotation) | pending | |
| 2a | fem3d oracles (`wp-t12/fem3d-oracles/patch.py`) | pending | |
| 2b | outline-row-ids (`wp-t12/outline-row-ids/patch.py`) | pending | |
| 2c | example-geometry (`wp-t12/example-geometry-paths.py`) | pending | |
| 2d | energy reference drift (`wp-t12/energy-reference-drift.py`) | pending | |
| 2e | kit pack twin (`wp-t12/kit-tower-pack.py`) | pending | |
| 2f | brep reference carrier (`wp-t12/brep-reference-carrier.py`) | pending | |
| 2g | txt oracle refusal (`wp-t12/txt-oracle-refusal.py`) | pending | |
| 3 | D1 `d1-frozen.py --apply` + capability audit + description census + `search::long` | pending | |
| 4 | cross-plugin verb-arg census → framework law + plugin fixes | pending | |

### Log

- 19:05 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-t12.md`, `📓️wp-d1.md`. Load 23, 4 rustc, 113 GiB free, wasm mutex free.
- 19:08 item 1 dry run on the current tree: **79 files / 0 problems** (`open-kinds-dry-1.txt`) → `--write` 19:08:39, 79 files
  (`open-kinds-write-1.txt`); touched crates: 36 stdio artifact crates + gisterrain + plugins stdio/gis + `semio-hub`
  (`open-kinds-crates.txt`). Remaining `"stdio.<x>"` literals outside stdio are format kinds (`*_stdio_kinds`) and
  generated descriptor/catalog JSON (W3's describe/generate rewrites them), as the script documents.
- 19:1x–19:24 waited for rustc ≤ 14 (peaks at load 80: peers' stdio closures recompiling after the edit).
- 19:2x item 2 dry runs on the current tree, all clean: fem3d 7/0, outline-row-ids 9/0, example-geometry 2/0, energy 3/0,
  kit 1/0 (50 019 → 50 018 bytes), brep 4/0 (13/13 vectors, 26 rows apply+invert), txt 1/0 (`item2-dry-1.txt`).
- 19:25 item 3 `d1-frozen.py --apply` (dry run clean first): gis2d +10 descriptions/+8 Chrome, gis3d +2/+1, vcs +5/+1,
  stdio contract fn + 62 `setActiveExample` sites (`d1-frozen-apply-1.txt`). Applied together with item 1 so the stdio/gis
  closure recompiles ONCE for the fleet; one combined native check of the union (44 crates + demonstrator,
  `--features semio-s-plugin-stdio/full-app-catalog,semio-hub/integration-fixtures --lib --tests`), nohup pid 29917,
  capture `check-set1-native-1.txt`.
- 19:26 item 2a fem3d `patch.py --write` (dry run 7/0 again first): 5 references + 2 features. Python oracle through the
  platform: `🏋️mutate-fem3d-1-load` **60/60** (308 s, `oracle-fem3d-1.txt`); the other 5 cases detached (pid 36595,
  `oracle-fem3d-2.txt`).
- 19:33 combined native check run 1 **EXIT 101** (`check-set1-native-1.txt`) — NOT from the landed hunks: with
  `semio-s-plugin-stdio/full-app-catalog` the avi/mp3/mp4/wav editors fail (E0407 `register_tool_job_factories`,
  `build_tool_job`, `bounded_first_step_tool_proofs!` … "not a member of trait `ArtifactOwnedToolJobFactory`",
  E0433 `semio_framework_job`). **Finding LB-F1:** 52 stdio editors outside the shipped component (every editor except
  the 7 text/data ones of `component-app-assembly`) carry the `ArtifactEditor` tool-job methods INSIDE
  `impl ArtifactOwnedToolJobFactory for <X>EditorExampleFactory` (in HEAD since ≤ 09-23, e.g. wav `34c9fca3239`); the
  library-only `full-app-catalog` fleet has not compiled since and nothing builds it (the shipped component and the
  hub's `full-artifact-catalog` never compile those editor modules). Census script output above (52 files).
- 19:35 check run 2 without `full-app-catalog` (shipped features: stdio `plugin-root`, hub default +
  `integration-fixtures`), nohup pid 36493, `check-set1-native-2.txt`.
- 19:5x coordinator: take LB-F1 after items 1–3, before item 4. Codemod `wp-lb/lb-f1-editor-tool-jobs.py` written: moves
  the six misplaced items into `impl ArtifactEditor for <X>` after its `const DOCUMENT_SCHEMA` (the txt layout), keeps
  `Owner/TOOL_IDS/DOCUMENT_SCHEMA/PUBLICATION_CONTRACTS` on the factory, adds `semio-framework-job = { workspace = true }`
  to the 19 crates lacking it; dry run **52 editors, 19 manifests, 20 crates, 0 problems** (not applied yet).
- 20:0x fem3d oracles: material **19/19**, analysis **6/6**, boundary EXIT 1 without a summary line (runner discarded the
  log; re-run with `oracle-keep.sh`). Coordinator rule 22 (swap 24.3/25.6 GB: one cargo/nx at a time) → stopped my oracle
  chain (pids 36595/53916/53921 + children) while the check runs; mesh + any-mesh + boundary re-run after it.
- 20:11 check run 2 (shipped features) **EXIT 101** with 2 errors (`check-set1-native-2.txt`): (a) MINE — T12's fence-law
  hunk `terrain_target.profiles[0].open_targets.push(… package: terrain_target.profiles[0]… )` (E0502, IndexMut +
  shared borrow) → fixed by binding `package` first; (b) a PEER's in-flight hub edit `🏗️bootstrap/🦀️.rs:9897`
  (`admin_observability` returns `Json<serde_json::Value>` but builds `HubObservabilityV1`, file mtime 20:04) — not
  touched. All stdio/gis/vcs/demonstrator crates compiled. **Run 2 predates 20:14 (rule 24: false-fresh fingerprints
  from P8's scratch clone) → superseded, re-run below.**
- 20:12 LB-F1 `--write`: 52 editors + 19 manifests (`lb-f1-write-1.txt`).
- 20:22 run 3 (started 20:12, pre-20:14) killed (my pids 60328/60330/60336); run 4 started 20:22 (pid 66564):
  set-1 crates except the hub with `semio-s-plugin-stdio/full-app-catalog --lib --tests`
  (`check-set1-native-4.txt`), then `semio-hub --lib --profile test` (`check-hub-libtest-2.txt`; the bin test target is
  blocked by the peer's bootstrap edit).
- 20:22–21:02 run 4 stalled 39 min at "Checking wasmtime-wasi" (0 % CPU, no rustc; 28 idle cargos fleet-wide) → killed
  mine, told main. 21:02 run 5 was stopped by the coordinator's cleanup (EXIT 143). Coordinator: default build-dir jammed
  by a Codex peer's cargos + orphans; rule 27 → landing slices build in
  `CARGO_BUILD_BUILD_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/build-landing`. `check-native.sh` exports it now.
- 21:15 run 6 (cold landing build-dir), pid 22456: set-1 + LB-F1 crates `--features semio-s-plugin-stdio/full-app-catalog
  --lib --tests` (`check-set1-native-6.txt`), then hub `--lib --profile test` (`check-hub-libtest-4.txt`).
