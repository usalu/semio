# 🔎️ W3 closing audit — EXTRACT-WFC-PLUGIN

Read-only audit against the brief, the fleet log (`📓️status.md`, `📓️plan.md`, the five slice reports,
five `📓️audit-*.md`, five `📓️playground-*.md`, `📓️engine.md`, `📓️scaffolding.md`,
`📓️procedural-cleanup.md`, `📓️gates.md`, `📓️close-ladder.md`), the code on disk, and the probe
artifacts under `$T/🗑️generated/`. No files were edited, no git-modifying command was run, no build
was run — every "GREEN"/"RED" claim below is either verified by reading source/logs/PNGs directly, or
is quoted from a fleet report with its own citation kept intact.

`$T = .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN`

## 1. Extraction

| check | result |
|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly` no longer exists | ✅️ confirmed on disk — the folder contains only `🌀️generation2d`/`🧊️generation3d` |
| Engine lives at `✏️s/🔌️plugins/🀄️wfc/⚙️engine`, modules un-gated | ✅️ confirmed: crate root has 38 `pub mod` lines, only `oracle` and `model_vectors` still carry `#[cfg(test)]` (plus the test-only `grid_job_drive_tests` module) — matches `📓️engine.md` §3's "34 modules lifted out of `#[cfg(test)]`, only oracle + model_vectors stay test-only" |
| Repo-wide grep for `procedural_assembly\|s\.assembly\|data\.assembly\|s\.procedural\.assembly\|wfc_engine\|🧩️wfc-engine` (excluding node_modules/target/dist/⚡️cache/🗑️generated/tickets/.git) | ✅️ 11 hits, **all legitimate**: 10 are the crate name `semio_s_plugin_wfc_engine` (a substring match on `wfc_engine`, not a stale reference) inside the five artifacts' inference files, plus one historical-provenance comment in `🧱️grid3d/…/🧩️outcome/🦀️.rs:4` ("unlike `s.assembly`'s own outcome test…") which `📓️gates.md` §1 already catalogued and left deliberately. One unrelated false positive in an archived research markdown (`♻️mit-bestand/🔎️recherche/_archive/…/piece_bauteilpass_interface.md:1020`, `logistics.assembly_access_zones`). Zero live referrers. |

**Verdict: PASS.** The extraction is complete and clean.

## 2. Per-artifact conformance to the brief

Evidence is file:line where useful; window-kind/tile-media/neighbour facts were also cross-checked
against the live descriptor `✏️s/🔌️plugins/🀄️wfc/🔣️.json` (mtime 17:22, `manifest.apps`, 10 apps).

### bitmap (`🖼️bitmap`) — "input bitmap, output bitmap, locally similar"
- Document: `BitmapSnapshot { input: BitmapInput{width,height,palette,pixels}, output: BitmapOutputSpec, model: OverlappingModel, pinned }` (`…/🧬️schema/📸️snapshot/🦀️.rs:117-231`).
- Editor windows: `wfc-bitmap-input` + `wfc-bitmap-output`, both `canvas-2d` (descriptor). Input is the interactive paint surface (`stroke-begin/extend/commit`), output is the read-only solved render — exactly the brief's pair.
- Solve `s.wfc.bitmap.solve`: `extract_2d` (+ symmetry) → `Grid2dTopology` → resumable `WfcJob` (`📓️bitmap.md`).
- **Live proof**: `$T/🗑️generated/playground-bitmap/interact/3-solve.png` — "Rooms 16" example: input pane shows a small room/corridor motif (dark walls, orange door markers on a beige grid); output pane shows a larger tiled pattern built from the same wall/door vocabulary, locally similar to the input, exactly as the brief describes. A second solve on "flowers-24" is proven in `8-solve-second-example.png` (stems/petals, per `📓️status.md` B1).

### 2d-grid (`🔲️grid2d`) — "regular rectangular grid, fixed L/R/T/B neighbours, vector or bitmap tiles"
- Direction enum `WfcDirection2d { Left, Right, Top, Bottom }` (`…/📸️snapshot/🦀️.rs:102-110`, wire-renamed `LEFT/RIGHT/TOP/BOTTOM`) — exactly the four fixed neighbours the brief asks for, no arbitrary relation.
- Tile media `WfcTileMedia2d { Bitmap{…}, Vector{paths}, Image{child} }` (`🦀️.rs:70-73`) — **both** vector and bitmap supported, as required.
- Editor windows: `wfc-grid2d-grid` (Board2d-backed, cell grid, pin/mask by click) + `wfc-grid2d-preview` (Canvas2d, tile media per solved cell) — "grid canvas with cells + 2d preview" verbatim.
- **Live proof**: `playground-grid2d/interact-3/8-solve.png` — "Pipes" example: left pane is a dark cell grid with one pinned (blue "elbo…") and one masked (red) cell; right preview pane shows solved cyan pipe segments connecting across the grid, a real vector-tile result.

### 2d (`◻️2d`, dev name "wfc2d") — "arbitrary neighbours, arbitrary non-rectangular shape, rectangular tiles, graph + 2d preview"
- `SlotEdge { relation: String }` and `GraphRule { relation: Option<String> }` (`…/📸️snapshot/🦀️.rs:137-160`) — an **arbitrary, named** relation vocabulary, not a fixed 4/6-direction enum; slots carry free x/y/width/height, so shape is not grid-constrained.
- Tile media `Wfc2dTileMedia { Bitmap, Vector, Image }` (`🦀️.rs:88-100`) — vector and bitmap, matching the brief's "vector graphics, bitmaps, etc."
- Editor windows: `wfc-graph` (`node-graph` surface, one node per slot, adjacency wires) + `wfc-2d-preview` (`canvas-2d`, each slot rect filled with its solved tile) — "graph with slots + 2d preview" verbatim.
- **Live proof**: `playground-wfc2d/interact/2b-preview-solved.png` — three slot rectangles, the middle ("corridor") a distinct blue fill from the two rooms — a real, visibly-differentiated solved assignment.

### 3d-grid (`🧱️grid3d`) — "regular non-uniform box grid, six fixed neighbours, mesh tiles, grid box + 3d preview"
- Direction enum `Grid3dDirection { Left, Right, Front, Back, Bottom, Top }` (`…/📸️snapshot/🦀️.rs:97-116`) — the six fixed neighbours the brief asks for.
- `cell_sizes_x/y/z: Vec<f64>`, one entry per column/row/layer (`🦀️.rs:256-266`) — **non-uniform** per-axis box sizing, not a uniform cube grid.
- Tile media `Grid3dTileMedia { Mesh{positions,indices,color}, MeshChild{child} }` (`🦀️.rs:38-59`) — meshes, as required.
- Editor windows: `wfc-grid3d-grid` (World3d, one box instance per cell) + `wfc-grid3d-preview` (World3d, one mesh per tile / one instance per solved cell) — "grid box with cells + 3d preview of the meshes" verbatim.
- **Live proof**: `playground-grid3d/boot-3/final.png` — "Building Blocks" example: left pane shows a translucent wireframe 4×4×N box grid (pink/cyan cell tint), right preview pane shows the same box populated with solved meshes in matching colours — a real, non-uniform 3d solve rendered in both windows.

### 3d (`🧊️3d`, dev name "wfc3d") — "arbitrary neighbours, arbitrary non-boxed shape, mesh tiles, same graph as 2d + 3d preview"
- `SlotEdge{relation: String}` / `GraphRule{relation: Option<String>}` (`…/📸️snapshot/🦀️.rs:86-111`) — arbitrary named relations, same shape as wfc2d's.
- Tile media `TileMedia3d { Mesh, MeshChild }` (`🦀️.rs:33-49`) — meshes.
- Editor windows: `wfc-graph` (shared with 2d, see §2.1) + `wfc-3d-preview` (World3d instances at slot x/y/z) — "the same graph-with-slots window as 2d + a 3d preview" verbatim.
- **Live proof**: `playground-wfc3d/interact-5/4-example-two-rooms-and-a-corridor.png` — graph pane shows three connected slot nodes in a row; preview pane shows two solved 3d boxes (gray room, tan room) meeting at a shared face — a real graph-driven 3d solve.

### 2.1 `wfc-graph` window shared verbatim between 2d and 3d
Both `diff` checks below return **zero lines of difference**:
```
diff ◻️2d/…/✏️editor/…/🪟️windows/🕸️graph/🦀️.rs          🧊️3d/…/✏️editor/…/🪟️windows/🕸️graph/🦀️.rs           → empty (225/225 lines)
diff ◻️2d/…/🕸️graph/🧪️tests/🔬️unit/🦀️.rs                🧊️3d/…/🕸️graph/🧪️tests/🔬️unit/🦀️.rs                → empty (59/59 lines)
```
Confirms `📓️wfc2d.md`/`📓️status.md` B5's claim: the window is a pure `SlotGraphView` projection with no
artifact-specific code; wfc3d supplies only its own `Wfc3dGraphView` newtype.

**Verdict: PASS on all five artifacts.** Every structural requirement in the brief (window pair, window
kind, tile-media union, neighbour model, grid-shape rule) is present in the document model and
confirmed live in the running app via screenshot.

## 3. Test evidence

Latest `test result:` line found per crate (search scope: `playground-*`, `close-ladder`, `gates`, and
the per-artifact folders under `$T/🗑️generated/`, newest mtime wins):

| crate | latest result | source |
|---|---|---|
| `semio-s-plugin-wfc-engine --lib` | `301 passed; 0 failed` (idle machine) / `300 passed; 1 failed` (loaded — one flaky watchdog-timing assertion, p99 always passes) | `📓️engine.md` §6, §8 |
| `semio-s-artifact-wfc-bitmap --lib` | **185 passed; 0 failed; 1 ignored** | `🗑️generated/playground-bitmap/test-final.txt:781` |
| `semio-s-artifact-wfc-grid2d --lib` | **211 passed; 0 failed** | `🗑️generated/playground-grid2d/test-6.txt:968` |
| `semio-s-artifact-wfc-2d --lib` | **190 passed; 0 failed** | `🗑️generated/playground-wfc2d/test-8.log:959` |
| `semio-s-artifact-wfc-grid3d --lib` | **207 passed; 0 failed; 2 ignored** | `🗑️generated/playground-grid3d/test-final.log:889` — supersedes the 204/2-failed reading `📓️close-ladder.md` §4 flagged as "not mine, owner B4"; B4's own fix landed after L's snapshot |
| `semio-s-artifact-wfc-3d --lib` | **238 passed; 0 failed; 2 ignored** | `🗑️generated/playground-wfc3d/test-final.txt:873` |
| `semio-s-plugin-wfc --lib` (13 surface laws + `descriptor_is_fresh`) | 🔴 **13 passed; 1 failed** (`descriptor_is_fresh`) | `🗑️generated/close-ladder/plugin-lib.log:829`, the LATEST of the two logged runs (later than `gates/plugin-test-lib-final.log`, which read 14/14 green before the B wave staled it — see §5.1) |
| `semio-s-plugin-wfc --test close_ladder` | **11 passed; 0 failed** (6 editor + 5 viewer laws) | `🗑️generated/close-ladder/ladder-final.log` |
| `semio-s-plugin-wfc --test boot_deadline` | **1 passed** | `🗑️generated/gates/plugin-test-boot-deadline-2.log` |
| `semio-s-plugin-wfc --test idle_turns` | **2 passed** | `🗑️generated/gates/plugin-test-idle-turns-2.log` |
| `wasm32-wasip2` check, plugin + engine | 🟢️ both green, 0 warnings from wfc crates | `📓️gates.md` §0, `📓️close-ladder.md` §3 |
| `@semio-tech/wfc-js:test` (TS) | **10 files / 148 tests, 0 failed** | `📓️status.md` line 159 (coordinator's fix + re-run at 15:5x) — **not re-run after B3's later `🕸️NodeGraph`/graph-scale edits**, so treat as the last-known-good figure, not a final one |
| `semio-s-plugin-procedural --lib` | 🟡 8 passed, **1 pre-existing failure** (`generation2d_viewer_never_mutates`, ordered-map retirement panic) | `📓️procedural-cleanup.md` §2 — confirmed by `git diff 7bea15c349` touching none of the implicated files; unrelated to this ticket |
| `semio-s-plugin-procedural --test idle_turns` | 🟡 **2 pre-existing failures** (generation2d retention, 29-31 KB/turn) | `📓️procedural-cleanup.md` §2, same non-wfc cause |
| `semio-framework-os-mcp --lib inference` | 🔴 **does not compile** (10 pre-existing errors, none in the file slice C edited) | `📓️procedural-cleanup.md` §2 — slice C's assembly→wfc MCP fixture swap is therefore **unverified by execution** |

### Reds, all accounted for
1. **`descriptor_is_fresh` (plugin-wfc --lib) — genuinely open.** `📓️gates.md`/`📓️close-ladder.md` both
   flag it "red for the whole B wave, owner: coordinator/G once B* settle"; no later log re-runs it, and
   B3 (wfc2d, finished ~17:43, "`🕸️graph/🦀️.rs` CHANGED") and B5 (wfc3d, "this slice ADDED window
   actions… `🔣️.json` is staler than before") both landed edits after the last logged `describe`. **A
   final `wfc-plugin:describe` → `plugin-registry:generate` → re-run of `cargo test -p semio-s-plugin-wfc
   --lib` is required before this can be called green.**
2. `semio-s-artifact-wfc-grid3d`'s two failures from B4's in-flight `worldSelect` rename are **fixed**
   (207/0/2 supersedes 204/2/2 — confirmed above by mtime).
3. Procedural's 3 pre-existing failures and the MCP compile failure are **not this ticket's fault**
   (each traced to files this ticket never touched), but the MCP one means slice C's own change is
   formally unverified — worth a follow-up once the peer breakage clears.
4. `plugin-registry:check`: wfc 78 → 54 findings, all 54 in repo-endemic categories (`📓️gates.md` §3.1
   gives the repo-wide counts for each). Not a regression, not blocking, judged acceptable by slice G.

## 4. Conventions

| check | result |
|---|---|
| No app-to-app crate coupling | ✅️ grepped all five artifact `Cargo.toml`s for `semio-s-plugin-{draw,raster,lowpoly,procedural,puzzle,remodel,note,cad,fem}` / `semio-s-artifact-{same}` — **zero hits**. Only the wfc engine + framework/stdio deps. |
| Verbs `fix`/`clear`/`remove`/`restore` only (no bare `pin`/`unpin`/`mask`/`unmask`) | ✅️ `grep -rn 'verb: "pin"\|"unpin"\|"mask"\|"unmask"'` under the plugin — zero hits. Spot-checked `pin-pixel`'s `SemanticDescriptor { verb: "fix", record: "Fixed" }` — matches the coordinator's §7 amendment. |
| Labels en + de | ✅️ descriptor `manifest.apps[…].windowKinds[…].actions[…].label` carries `{native:{en,de}, reuse:{en,de}}` pairs throughout (e.g. "Paint Pixels"/"Pixel malen", "Resize Input"/"Eingabe skalieren"); plugin-level `.label("WFC")` matches the plan's "WFC (en) / WFC (de)" (identical string both languages, same pattern as `puzzle`/`procedural`). |
| Taxonomy emoji admissible | ✅️ three inadmissible emoji were caught and fixed during the ticket: `▦️grid2d` (U+25A6, not Extended_Pictographic) → `🔲️grid2d`; `🌊️wfc` (folded to the same grapheme as sibling `🌊️flow`) → `🀄️wfc`; `⬡️hex-ring` (U+2B21) → `🔷️hex-ring`. `loadTaxonomy(repoRoot)` verified green after all three (`📓️status.md` line 51, 68; `📓️gates.md` §2.1). |
| Plugin registered | ✅️ `🗺️catalog.json` line 62 (`{"pluginId":"wfc","directoryName":"🀄️wfc"}`); root `Cargo.toml` lines 106-107/259-263/326-330/502-503 (6 crates: plugin + engine + 5 artifacts); `package.json` line 85 (TS workspace); `.claude/launch.json` lines 441-461 (5 react-attach entries); `🔣️taxonomy.json` line 11617 (`members-of-plugins: "🀄️wfc"`). |
| Descriptor lists 10 apps with examples | ✅️ `✏️s/🔌️plugins/🀄️wfc/🔣️.json` → `manifest.apps` has exactly 10 entries (5 editors + 5 viewers), window kinds match §2 exactly (`canvas-2d`/`node-graph`/`world-3d` as appropriate), `manifest.examples` has 13 top-level entries (same shape as `puzzle`'s 7-top-level/0-per-app convention). **Caveat**: this file's freshness is exactly what `descriptor_is_fresh` (§3.1) disputes — it may be stale relative to B3/B5's very last edits. |

## 5. DoD table

| brief requirement | verdict | evidence |
|---|---|---|
| Assembly dissolved, engine moved verbatim, un-gated | ✅️ PASS | §1 |
| No stale assembly/wfc-engine referrers repo-wide | ✅️ PASS | §1 |
| bitmap: input/output bitmap editor, locally-similar solve | ✅️ PASS | §2, `interact/3-solve.png` |
| 2d-grid: fixed 4-neighbour, vector+bitmap tiles, grid+preview windows | ✅️ PASS | §2, `interact-3/8-solve.png` |
| 2d: arbitrary named-relation neighbours, non-rectangular shape, graph+preview windows | ✅️ PASS | §2, `interact/2b-preview-solved.png` |
| 3d-grid: fixed 6-neighbour, non-uniform box, mesh tiles, grid-box+preview windows | ✅️ PASS | §2, `boot-3/final.png` |
| 3d: arbitrary neighbours, non-boxed shape, mesh tiles, shared graph+preview windows | ✅️ PASS | §2, `interact-5/4-example-…png` |
| `wfc-graph` shared verbatim between 2d/3d | ✅️ PASS | §2.1, 0-line diffs |
| Solve visible live in the running app, all 5 artifacts | ✅️ PASS | §2, five screenshots read and described |
| Per-crate unit tests green | ✅️ PASS (5/5 artifact crates + engine) | §3 |
| Plugin-level close ladder (editors + viewers) | ✅️ PASS 11/11 | §3, `📓️close-ladder.md` |
| Plugin-level surface/boot/idle laws | ✅️ PASS (13 surface + boot + idle) | §3 |
| Plugin descriptor freshness (`descriptor_is_fresh`) | 🟡 PARTIAL / OPEN | §3.1 — needs a final `describe` + re-verify pass |
| TS oracle suite (`wfc-js:test`) | 🟡 PARTIAL | §3 — last known 148/148 green but not re-run after B3's final edits |
| No app-to-app coupling | ✅️ PASS | §4 |
| Approved verbs only | ✅️ PASS | §4 |
| Labels en+de | ✅️ PASS | §4 |
| Taxonomy emoji admissible | ✅️ PASS | §4 |
| Plugin fully registered | ✅️ PASS | §4 |
| `plugin-registry:check` clean | 🟡 PARTIAL (54 residual, repo-endemic, non-blocking) | §3 |
| `verify taxonomy` runnable end-to-end | 🔴 FAIL (blocked upstream) | §6 |
| `bun ./📜️script.ts policy` runnable | 🔴 FAIL (blocked upstream) | §6 |
| MCP fixture swap (slice C) verified by execution | 🔴 FAIL (peer target won't compile) | §3 |

**Score: 17 PASS / 3 PARTIAL / 3 FAIL** (of 23 tracked items).

## 6. Open items, prioritized

### Blocking (must close before this ticket can be called done)
1. **`descriptor_is_fresh` is red as of the last recorded run.** Run `bun nx run
   @semio-tech/wfc-plugin:describe` → `bun nx run @semio-tech/plugin-registry:generate` →
   `cargo test -p semio-s-plugin-wfc --lib` once, now that all five B-slices have stopped editing
   window actions/commands. Nobody has done this since the B wave started (§3.1). This is the
   single concrete regression left in the plugin.
2. **`semio-framework-os-mcp --lib inference` still doesn't compile**, so slice C's assembly→wfc swap
   in the MCP quick-inference fixture (`…finds_the_real_wfc_roster`) has never actually run. Not this
   ticket's bug (10 pre-existing errors, none in the edited file), but it means one of the 22 referrer
   fixes in `📓️procedural-cleanup.md` is unverified. Re-run once the peer target compiles.

### Not blocking, tracked for other owners (do not re-open here)
3. `verify taxonomy` cannot classify a single wfc path — blocked by a stale, gitignored distribution
   manifest whose regeneration is itself broken by a moved file
   (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`) that a tracked `🔗️inputs.json`
   still points at the old location for. Owner: whoever moved the react build tooling (`📓️gates.md` §6).
4. `bun ./📜️script.ts policy` cannot run at all — the repo CLI binary (`💻️client/client`) is not built,
   and building it fails because the Go `cmd/repo` package it names does not exist on disk. Same root
   cause as this session's own failed `repo`/`semio` MCP connections. Owner: repo-client module
   (`📓️gates.md` §7). Consequence: the three "comment inside a definition" sites the brief named could
   not be confirmed as policy violations and were left alone rather than rewritten blind.
5. Procedural's 3 pre-existing test failures (`generation2d_viewer_never_mutates`, 2×`idle_turns`) — a
   peer's framework-state issue, proven unrelated by `git diff` against files this ticket never touched.

### Nice-to-have (polish, explicitly deferred by the fleet with reasoning)
6. `plugin-registry:check`'s 54 residual findings — all in categories the whole repo carries at similar
   or higher rates (`🎚️config`/`👥️presence` schema leaves, mounted-test child dirs, `🎮️commands` stub) —
   judged acceptable by slice G, `📓️gates.md` §3.1.
7. wfc3d's live `connect-slots` (wire-drawn) gesture is proven by unit test + Actions-form dispatch but
   not by a raw headless canvas drag — the wasm graph surface exposes no port-anchor screen rect for a
   probe to aim at (`📓️status.md` B5's "open question", a suggested `data-port-rects-json` follow-up).
8. `🔲️grid2d`/`🧱️grid3d`/`🧊️3d` have no `semio_repo_test_host` Rust adapter for their mutation oracle
   (bitmap and wfc2d do); the oracle manifests + Python references are committed and green regardless.
9. Four artifacts' declared 🖼️assets deviation (byte-correct `.dsl.semio` printer not yet written for
   `◻️2d`'s 4 examples / `🔲️grid2d`'s 2) — explicitly documented as a deliberate choice, not an omission.

## 7. Closing summary (for the ticket's `summary` field)

Wave function collapse has been extracted from the procedural plugin's assembly artifact into its own
`🀄️wfc` plugin with five artifacts — bitmap, 2d-grid, 2d, 3d-grid and 3d — each matching the brief's
window pair, tile-media union and neighbour model exactly, with the shared `wfc-graph` window proven
byte-identical between 2d and 3d and every solve proven visible in the live react app via screenshot
for all five. The shared 10 kLOC engine moved verbatim and is fully un-gated for production use, and
the source procedural plugin is assembly-free with every referrer (22 sites) accounted for. Per-crate
unit tests are green across the board (185–238 passing per artifact, 0 failures), the plugin-wide
close ladder went from 0-of-6 (every editor SIGABRT on close) to 11-of-11 after a real, well-diagnosed
fix (all five editors and viewers were missing their owned-store disposer declarations), and along the
way the fleet found and fixed a dozen genuine engine/framework bugs (retained-payload leaks,
`terminal_is_empty` mis-gating, per-step payload-page grants, a silently-ignored `rename_all` case, a
D4-symmetry ordering bug, three inadmissible taxonomy emoji) that would otherwise have shipped latent
in every future WFC-driven artifact. One concrete regression remains open at hand-off: the plugin's own
`descriptor_is_fresh` test is red because the last two artifact slices added window actions after the
last descriptor regeneration, and nobody has re-run `describe` since — a five-minute fix once the B
wave's edits are confirmed settled. Two repo-wide gates (`verify taxonomy`, `policy`) remain unrunnable
for reasons entirely outside this plugin (a misdirected distribution-manifest input path, a missing
repo-client binary), and one MCP test target the cleanup slice touched still can't compile due to
unrelated pre-existing peer errors, leaving that one fixture swap formally unverified by execution.
