# WP-T6: Dashboard Extraction, Print Distribution Package, Gallery Measurement, Placement Rows, Contract Delta

Slice: T6 (session 10). Captures: `.tmp-ticket/wp-t6/generated/`. Ticket inputs: `.tmp-ticket/wp-t6/*`.
Inherits: T4 §1.3, §4, §5; T1 §3.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Dashboard crate extraction | **Done.** `cargo check --tests` clean (0 warnings in the crate), unit tests **29/29**, parity case `🌳️command-tree-projection` **2/2** (no-oracle decision, so parity 0/0); `semio` binary smoke-tested | `dashboard-check-1.txt`, `dashboard-test-1.txt`, `parity-dashboard-1.txt`, `cli-check-1.txt`, `semio-bin-1.txt` |
| 2. Print `semio-viz-charts-distribution.sty` | **Done.** All 20 documents that needed it and not `patterns` execute green: box-violin 4/4, histogram-density 5/5, evaluation-curves 4/4, heatmap-matrix 3/3, quadrant-table 4/4 parity. All 51 catalogue variants of the 8 families render (tectonic, render mode). Print owner run: **507 passed, 0 failed, 118 errored (was 137), parity 208/291**; every errored scenario is a `patterns` document (40 documents, blocked on the user's approval) | `parity-print-*-1.txt`, `parity-print-all-2.txt`, `print-errors-classified-2.txt`, `distribution-render-2.txt`, `render-montage.png` |
| 2b. Print gallery measurement module | **Module done; evidence BLOCKED on `patterns`.** Matrix 81 sections × 2 themes × 2 languages = 324 variants; the measurement compiles, reads pages with PDF.js and hashes, but every gallery section loads `semio-viz-mark` → `patterns`, so `test viz fixtures` cannot regenerate real evidence until the approval lands | `gallery-probe-1.txt` |
| 3. Placement rows | **Done: 16 → 0** (vitest 4, registration 4, depth 6, self-test 2; covering hub 4, flow 2, stdio 1) | `contract-1.txt`, §3 |
| 4. Test-platform suite + contract delta | Suite **113 pass / 3 fail** (116). All 3 failures are timeouts under peer load: discovery idempotence (30 s), contract-zero (30 s, expected to fail anyway), purity narrowing (60 s). Contract **318 → 361**: my classes −16, runtime inventory −30 (peers), plus about 103 rows from a new mutation-outcome/production-reachability rule (fem 2d/3d, wfc, cad, gis, layout, sequence; peers) | `test-platform-1.txt`, `contract-1.txt`, `contract-delta-1.txt` |

## 1. Dashboard extraction

Followed T4 §5, with one correction: the dashboard's own `🌀️daemon` and `🌳️command-tree` already superseded the cli copies, so only six modules moved and the terminal was restored.

- **Moved** from `🦑️repo/🎮️commands/` into `🔨️modules/🎛️dashboard/` (plain `mv`, unit tests travel with them): `🌊️workflow`, `🔌️plugin-registry`, `🛝️playground-development-session` → `🛝️playground-session`, `⌨️cli-usage-presentation` → `⌨️usage`, `📇️playground-catalog-query` → `📇️playground-catalog`, `📜️root-script-delegation` → `📜️root-delegation`. Each gained an emoji `//!` module doc.
- **Deleted** as superseded: `🖥️terminal-dashboard-daemon` (the dashboard `🌀️daemon` already owns the `run` command, with the Windows named pipe), `🌳️command-tree-discovery` (the dashboard `🌳️command-tree` is its superset with repo-domain leaves), the `🎮️commands/🔣️.json` collection and the directory itself.
- **Restored** `🖥️terminal/🦀️.rs` from `bb961413d4^:…/🎮️commands/🎛️terminal-dashboard/🦀️.rs` and ported it to the current tree: `CommandLeaf::Process` spawns into a PTY window as before; `CommandLeaf::Repo` opens an output window titled by `action_key` and feeds it `RepoAction::execute(root)` in process. Common window setup is one `open_output`. Both `[DEBUG]` lines of the old file were dropped.
- **cli** (`⌨️cli/🦀️.rs`, 910 → 33 lines) now owns only the verb dispatch and calls the dashboard modules directly (no re-exports). The root comes from `semio_framework_repo_workspace::find_repo_root` instead of a private copy. The new `command-tree` verb is wired, and the usage text (and its unit test) lists it. The cli's `🧪️tests/🔬️unit` duplicated the dashboard's unit tests over the deleted copies, so it was removed.
- **Taxonomy:** the 9 repo-only names were removed from `members-of-commands.memberNames`, and the stale `🎮️commands/🌳️command-tree-discovery` row was removed from `🧅️layering.json`.
- **Not in scope, still open:** the cli's `repo` binary (`📦️mcp-main.rs`) and its 8 Rust test adapters call `semio_framework_repo_cli::mcp_verb` / `repo_cli`, which have never existed. That is the unwritten Rust port of the repo CLI (T4 §1.4). `cargo check -p semio-framework-repo-cli --lib --bin semio` is clean.
- **Runtime:** `semio` (non-tty) prints the usage and exits 1; `semio command-tree` prints the discovered tree; `semio daemon status` reports "daemon not running"; `semio plugin registry check` reports the two missing generated catalogs (the registry has not been regenerated since the reboot).

## 2. `semio-viz-charts-distribution.sty`

No copy exists in any git revision, ticket folder or cache (git history, `mdfind`, `/private/tmp`, `/var/folders` searched). The package was rebuilt from what the code around it states, not guessed:

- **What its consumers need.** `undefined-cs.py` lists every csname the 7 requiring packages and the fixtures use that no package defines. There are 32: the frame state `\l_semio_viz_cb_*` plus `\semio_viz_cb_reset:`/`\semio_viz_cb_frame:`, the key set `semio / viz / family / common`, the 7 probing primitives `\semio_viz_g_{rect,circle,line,polyline,polygon,arc,text}` (with their `:Vn`/`:xn` variants), and `\semio_viz_palette:n`.
- **What the primitives emit** comes from the adapters' oracles: `geometry/rect` = x,y,w,h (heatmap, table-bars); `circle` = x,y,r; `line` = x1,y1,x2,y2 (table rule); `text` = x,y (the table anchors index text records two numbers apart); `arc` = cx,cy,inner,outer,start,end in d3-shape radians (pie, polar); polygon/polyline = the flat point list (radar).
- **The frame** is `[pad, width−pad] × [pad, height−pad]` in millimetres with y up. This is fixed by the heatmap `scaleBand` oracle and by the table's header at y = height. The defaults (60 × 40, pad 3; data `demo-distribution`, value `value`, group `grp`) come from `🔣️viz-api.json` and the guide's frame height.
- **Families**: histogram, density, ecdf, box, violin, strip, quantile, stem-leaf. Their keys and defaults are those of `🔣️viz-api.json` and the 51 catalogue kinds of `semio-viz-catalog.sty`. Every statistic delegates to `semio-viz-transform` (bin, kde, ecdf, R-7 quantile, probit) and `semio-viz-scale` (linear, band). `geometry/box` carries Tukey's five numbers per group; `geometry/letter` = depth, lower, upper, at 2^-(d+1), exactly as the adapter's oracle states.
- **Demo tables** (`demo-scores`, `demo-parts`, `demo-profile`, `demo-interval`, `demo-control`, `demo-quadrant`). Their columns are those the families read and `🧬️schema` lists. `demo-parts` columns were corrected in the schema to `part, share, group`, which is what the code reads.
  - **Values taken from the tests:** scores and labels (evaluation adapter), shares (pie adapter), profile `s1` (polar adapter).
  - **Values derived from the tests:** interval estimates 0.82/1.14/0.66/1.03/0.91/0.95. They are solved from the bar-in-cell widths of the quadrant-table feature (range 0.66..1.14).
  - **Values authored, because no test or document pins them:** part names/groups, profile `s2`/`s3`, interval bounds and weights, control, quadrant.
- **`\semio_viz_palette:n`** is the one-based categorical slot. It delegates to the theme's `\semio_viz_theme_color:n`, and 122 call sites in 7 packages use it.
- **Verified**, five cases 20/20 parity (see Status). A render-mode document of all 51 variants compiles, and every family draws distinct geometry (`render-montage.png`, `jitter-05.png`).
- **Blocked, awaiting the user's approval for tikz `patterns`**: 40 documents in 25 cases, plus `🖼️gallery-render`. That covers every remaining errored scenario of the print owner.
- **Print owner debt found, not mine:**
  - `📚️catalog-coverage::implemented-keys-documented` has 20 findings, all axis/scale/composition `legend` keys; `api-reference` was stale. I regenerated `🔣️viz-api.json` through `generateVizArtifacts`. That added the `semio-viz-axis` package and removed the stale `📊️viz-gallery/🔓️viz-api.tex` duplicate, and `api-reference` is now green.
  - The print `📋️project.json` target `generate-viz` calls `bun ./📜️script.ts generate viz`, but the print package router registers no `generate` command.

## 2b. Gallery measurement

- **Owner.** `🔨️modules/📊️visualization-gallery/🔬️probes/🟦️.ts` is the gallery's PDF measurement tool, and it already owned `pdfStableHash`. It now also carries:
  - `printGalleryMatrix(sections?)`: every `📊️viz-gallery` section × `VIZ_GALLERY_THEMES` × `VIZ_LANGUAGES`;
  - `printGallerySource`: rewrites the `\documentclass[…]{semio}` theme and language, so no default ever applies;
  - `measurePrintGalleryVariant(variant, signal?)`: compiles in a cache work dir through `compilePrintTexOnce`, reads each page with PDF.js, finds every drawn kind's page by its localized catalogue title, and records the page text and the stable hash;
  - `writePrintGalleryEvidence(sections, signal?)`.
- **Schema-first.** New `$defs` `GalleryTheme`, `GalleryKindPage`, `GalleryVariantMeasurement` and `GalleryRenderEvidence` in print `🧬️schema/🔣️.json`, with matching TS types in `🧬️schema/🟦️.ts`.
- **Generator.** `bun ./📜️script.ts test viz fixtures [section…]` (print package) now regenerates the evidence; with sections given, it keeps the other committed variants. The adapter imports from the probes module instead of the non-existent `🎮️commands/…/🧪️tests/🟦️.ts`.
- **Run.** The matrix is 324 variants. Measuring `viz-3/dark/en` stops in `semio-viz-mark.sty:12` on `patterns`. Real evidence is therefore **blocked on the approval**, and `{"variants": {}}` stays committed until then.

## 3. Placement rows

| Rows | Root cause | Fix | Verified |
|---|---|---|---|
| hub ×4 (depth) | three `BundleScript` runners and a shared fixture-expectation helper nested in `🧪️tests` | Runners → hub `📜️script.ts` (`//#region 🧱️SourceGateRunners`), as T4 did for os mcp. The helper → hub schema module `🌎️hub/🧬️schema/🛂️fixture-expectation/{🟦️.ts,🔣️.json}` (new draft-07 schema). Taxonomy kind `hub-fixture-expectation` now sits under `hub-schema` (`^fixture-expectation$`); `hub-tests-schema`, `hub-foundation-source-execution` removed. foundation-source and socket-grant-command-source fixtures, schemas (owner/context counts), laws and `📋️project.json` inputs updated | foundation-source law 11/12. The 1 failure is the mcp credential-source order in the os mcp runner (peer area, untouched). Router loads and lists every route |
| flow ×2 + self-test | ajv admission helper nested in a case; ownership self-test as a bare `🧪️tests/🟦️.ts` | helper → `🌊️flow/🕸️wasm/🔮️oracles/🛂️contract-admission/🟦️.ts` (owner oracle, like `🧑‍🎨engine/🔮️oracles`); ownership → case `🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts`. 5 importers, fixture tokens, core `📋️project.json`, core router updated | all import paths resolve; the ownership law now passes schema, owner count and paths, then fails on `host-runtime` exports (`createFlowPumpScheduler` added by a peer; fixture drift, not placement) |
| stdio self-test | `testStdioComposition` in the build module | → case `🗄️stdio/🧪️tests/🧩️composition-consumption/🟦️.ts`. The build module exports `runStdioTypeScriptCompiler` (one compiler flag set). Ownership fixture +1 owner, schema 7→8 | route reaches the moved test, then `Bun.build` cannot resolve `@semio-tech/stdio-*` workspace links (needs `bun install`). Also fixed the router's broken `../../🏗️build` import |
| os vitest+registration | inline persistence-data-class block in `💻️os/🟦️.ts` | removed; the fixture-driven case `🧪️tests/🗃️persistence-data-class` (already in `include`) is its superset | os vitest 371/372 (the 1 failure is a store-worker timing law) |
| server vitest+registration | in-source suite | → `🖥️server/🧪️tests/🔬️wire/🟦️.ts` beside its Rust twin, `registerServerWireTests(vitest, deps, source)` | **14/14** |
| ui react vitest+registration | two inline describes | → `🖱️ui/🧪️tests/🖼️icon-render-camera/🟦️.tsx` | **3/3** |
| presentation react vitest+registration | 3,860-line in-source suite; an `import.meta.vitest` autoplay gate in production | → `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx`, generated by `extract-presentation-suite.py` + `rebind-reexports.py`. 6 internals exported; re-exported names imported from `@semio-tech/presentation`. The production gate was removed (jsdom has no media; `play` is already guarded) | **136/147, the identical 11 failures as the pre-change baseline** (`presentation-fail-0.txt` vs `-3.txt`) |

**Found, not fixed (peer or owner):**
- The socket-grant-command-source law was already stale at HEAD: its taxonomy kinds and the `hubSocketGrantCommandSources` named input do not exist.
- `verifyStdioCommandOwnership`, imported by the stdio ownership case and router, is defined nowhere.

## 4. Delta (`classify.py` T4 `contract-2` → `contract-1`)

| Class | T4 | T6 | Δ |
|---|---|---|---|
| test-source-depth | 6 | 0 | −6 |
| vitest-wiring | 4 | 0 | −4 |
| test-registration | 4 | 0 | −4 |
| self-test | 2 | 0 | −2 |
| no-runtime-inventory | 172 | 142 | −30 (peers) |
| new: "mutation … declared but production …" / "declares outcomes […]" / "reachable through …" | 0 | ~103 | +103 (a new contract rule over fem, wfc, cad, gis, layout and sequence catalogs; peers) |
| everything else | unchanged | | |
| **Total** | **318** | **361** | **+43** |

## Processes (pids)

All detached and exited, none left running:

- dashboard tests 98204
- dashboard parity 99115
- semio binary smoke 5516
- print parity 34971 (killed at 12:35 by the usage cut, rerun as 93603)
- test-platform + contract chain 1037

No servers, no ports, no fleet-mutex work: no wasm32 and no `semio-hub` cargo. `wp-t6/target` and the render work dir were deleted.

## Files changed (T6)

- **Dashboard:**
  - `🦑️repo/🔨️modules/🎛️dashboard/{🌊️workflow,🔌️plugin-registry,🛝️playground-session,⌨️usage,📇️playground-catalog,📜️root-delegation}/**` (moved)
  - `🎛️dashboard/🖥️terminal/🦀️.rs` (restored and ported)
  - `🦑️repo/🎮️commands/**` (deleted)
  - `⌨️cli/🦀️.rs`
  - `⌨️cli/🧪️tests/🔬️unit/🦀️.rs` (deleted)
  - `📚️library/🔣️taxonomy.json`
  - `🧅️layering.json`
- **Print:**
  - `🖋️latex/semio-viz-charts-distribution.sty` (new)
  - `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`
  - `🔨️modules/📊️visualization-gallery/🔬️probes/🟦️.ts`
  - `🎮️commands/🧪️print-pipeline-verification/🟦️.ts`
  - `🧪️tests/🖼️gallery-render/🟦️.ts`
  - regenerated `🖼️assets/🔣️viz-api.json`; the generator removed `🧾️template/📊️viz-gallery/🔓️viz-api.tex`
- **Hub:**
  - `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
  - `🧬️schema/🛂️fixture-expectation/{🟦️.ts,🔣️.json}` (moved and new)
  - 3 runner files deleted
  - `🧫️fixtures/🧱️{foundation-source,socket-grant-command-source}/🔣️.json` and `🧬️schema/…` for both
  - `🧪️tests/🧱️{foundation-source,socket-grant-command-source}/🟦️.ts`
- **Flow:**
  - `🕸️wasm/🔮️oracles/🛂️contract-admission/🟦️.ts`
  - `🕸️wasm/🌐️browser/🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts`
  - 5 importers, ownership fixture, core `📜️script.ts` and `📋️project.json`
- **Stdio:**
  - `🧩️composition/{🏗️build,🏃️commands}/🟦️.ts`
  - `🧪️tests/🧩️composition-consumption/🟦️.ts`
  - `🧫️fixtures/🏃️command-ownership/🔣️.json`
  - `🧬️schema/🏃️command-ownership/🔣️.json`
- **os:** `💻️os/🟦️.ts`
- **Server:** `🖥️server/🟦️.ts`, `🖥️server/🧪️tests/🔬️wire/🟦️.ts`
- **ui:** `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, `🖱️ui/🧪️tests/🖼️icon-render-camera/🟦️.tsx`
- **Presentation:** `🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`, `🎤️presentation/🧪️tests/🎞️presentation-react-deck/🟦️.tsx`
- **Ticket inputs:** `wp-t6/*.py`, `wp-t6/*.ts`, `wp-t6/*.tex`, `wp-t6/terminal-dashboard-at-622.rs`
