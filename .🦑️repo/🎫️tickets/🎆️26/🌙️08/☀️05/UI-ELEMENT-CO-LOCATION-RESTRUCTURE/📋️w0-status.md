# W0 — Mechanisms + Baselines: status

All changes in this wave are additive/inert against the live tree (no existing package moved, no `areas`
flip yet) and have been individually verified. Landed:

## Vocabulary (`🔣️taxonomy.json` + `🟦️discovery.ts`)
- Added `targetsDirName` (`🎯️targets`), `elementsDirName` (`🧱️elements`), `taxonomyLeafFilenames` map
  (replaces the old single `taxonomyLeafFilename` string), `entryFilenames`, `storyLeafFilename`, and
  `🔷️dotnet` to `langs`. Deliberately did NOT add `"🧰️framework/🔨️modules/🖱️ui"` /
  `"…📺️renderer/🧑️‍🎨️engine"` to `areas` yet — that flip is the W6 activation step.
- `discoverPackages`/`discoverPackageProblems` in `🟦️discovery.ts` now walk both the two-level
  (`📦️packages/<lang>/<manifest>`) and three-level (`📦️packages/<lang>/🎯️targets/<target>/<manifest>`)
  shapes, with loud diagnostics (ambiguous shape, dangling target dir, unmarked manifest outside a
  legacy/mixed/exempt area) instead of silent skips.
- Verified live: `discoverPackages(repoRoot)` still returns exactly the same 25 migrated-plugin packages
  as before the change, 0 problems.

## Registry validator (`…🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts`)
- Widened the framework-crate regex to accept an optional `🎯️targets/<target>/` segment (inert today —
  zero `🎯️targets` dirs exist anywhere in the repo yet, confirmed by `find`).
- Deduped `TAXONOMY_ARTIFACT_COMPONENTS`/`TAXONOMY_WINDOW_CHILDREN`/`TAXONOMY_LEAF_FILENAME` to source
  from `loadTaxonomy()` instead of an independently hand-maintained copy — values confirmed identical
  before/after.
- Added `discoverFrameworkPackages` (role === "framework") and wired `discoverPackageProblems` into
  `check`'s warn-only output (framework-package count + count in the "catalog is fresh" line).
- Ran `check`: still correctly reports the pre-existing stale-catalog condition, which is **not** caused
  by this change — traced to concurrent in-flight work on `🪐️space`/`🎪️demonstrator` plugins visible in
  `git status` (unrelated sessions, per `[[feedback-concurrent-cargo-workspace-churn]]`).

## Single-File-Repo goal exemption (`.🦑️repo/🎯️goals/AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/🎯️goal.json`)
- Extended the Rust-only exemption to every language: any `taxonomyLeafFilenames` leaf under a
  `🧱️elements/<Element>/` dir, wiring-only entry files, and target-dir `🦀️<name>.rs` module files are
  now out of scope for single-file consolidation, **before any TS split file exists** — required so the
  recurring inliner tool doesn't undo W2/W3 the moment they land.
- Explicitly carved out the difference between a ticket's own agent re-inlining its own aborted
  extraction (allowed, keeps the tree green) vs a third party consolidating a completed split (forbidden).

## Barrel-shape lint (root `📜️script.ts`)
- Added `policyTaxonomyBarrelShapeBreaches`, the TS analogue of `policyTaxonomyLibShapeBreaches`, wired
  into the `policy` export. Ran `bun ./📜️script.ts policy`: 0 barrel-shape breaches (vacuous by design —
  no package has area `"taxonomy"` or role `"framework"` yet). The run's exit 1 / 286 high-priority
  breaches are pre-existing and unrelated (stray `Cargo.lock`/`target` under the in-flight `🪐️space`
  migration, DSL-completeness gaps in `🌀️procedural`/`🌊️flow`) — confirmed via a fresh cache write
  (`.🦑️repo/⚡️cache/breaches/compose.json`, timestamp matches this run).

## Storybook (`​.storybook/scopes.ts`)
- Added optional `StoryScope.storyGlobs` + updated `buildScopeStoryGlobs` to use it when set, falling
  back to the existing `./stories/<id>/**` derivation otherwise. Manually replayed all 11 inline
  `import.meta.vitest` assertions in the file (couldn't invoke via `bunx vitest run` directly — in-source
  tests need `includeSource`/a different runner than the ad-hoc CLI call found) — all pass unchanged.

## dependency-cruiser (`.dependency-cruiser.cjs`)
- `renderer-hosts-only-ui`: was dead (matched a path that never existed — wrong segment order). Repointed
  at the FUTURE co-located shape (`📦️packages/🟦️typescript/🎯️targets/⚛️react` | `🧱️elements`) so it
  starts enforcing for real once W4 lands, instead of trading one dead path for another about to be
  deleted.
- `no-escaping-relative-imports`: no functional change (confirmed it matches the *resolved* dependency
  path, not the specifier depth — a `📜️script.ts`'s 6-8 `../` to reach repo-lib resolves inside the repo
  and never trips it); rewrote the comment so a future pass doesn't "fix" it into a specifier-depth rule
  that would break every `📜️script.ts` in the repo.
- Config verified to load (`require(...)`) with the same 988 forbidden-rule count as before.

## Baselines captured
- `🗺️element-inventory.txt` — full region inventory of the 40,690-line ui-react `📦️index.tsx`: 34
  top-level regions dissolved into ~110 rows, 51/75 storybook elements auto-matched by name at depth ≤2,
  25 unmatched stories listed explicitly for manual mapping in W2/W3 (mostly canvas-overlay elements
  nested inside the 8,847-line `⚙️Canvas` region — Button, ContextMenu, DragAndDrop, SelectionMarquee,
  CanvasPickMenu, etc.).
- **Found and FIXED a real source-file bug**: `🧭️ModeDockTabBar` (opens line 28870) was missing its own
  `//#endregion 🧭️ModeDockTabBar` before its sibling `//#region 🧭️ModeDockStack` opened at line 29088.
  Name-aware stack simulation (not just count-based) traced the cascade precisely: the missing close made
  the *next* `#endregion` comment pop the wrong stack entry, which cascaded through `🧭️Mode` → `⚙️Canvas`
  → `🔍️Window Components`, leaving `🔍️Window Components` (opened 23491) apparently unclosed at EOF even
  though its own `#endregion` comment (line 36887, now 36889) was present and correctly worded all along —
  it just closed the wrong thing. Fixed by inserting the single missing `//#endregion 🧭️ModeDockTabBar`
  line at 🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx:29087 (right
  after `ModeDockTabBar.displayName = "ModeDockTabBar";`, matching its sibling regions' exact pattern).
  Re-ran the stack simulation: **167 opens = 167 closes, 0 leftover** (previously 167 vs 166, 1 leftover).
  Three residual *cosmetic* label mismatches remain (open/close comment text differs slightly —
  `🔌️PortWiringAliases`↔`🔌️PortWiring` at line 880, `🎛️UiChromePrefs`↔`🎛️UiChromeCompact` at 3939,
  `🎽️XY Flow (additions…)`↔`🎽️XY Flow` at 36912) — these are balanced/non-structural, left as-is,
  not blocking. Regenerated `🗺️element-inventory.txt` against the corrected file (35 top-level regions,
  same 51/75 story matches — the fix didn't change classification counts since the group-header
  dissolution logic already handled the deeply-nested content correctly once reachable).
- Ran `bun ./📜️script.ts typecheck` on ui-react post-fix: ~25 pre-existing type errors (missing `iconId`
  properties, `IntroductionGesture` shape mismatches, a couple of unrelated cross-package errors in
  `🎨️styling`/`🖼️assets`/repo-lib). Confirmed via `git diff --stat HEAD` that my region-comment fix is
  exactly 2 lines inserted, nothing else — these errors predate this ticket and are out of scope to fix
  here; recorded as the typecheck baseline so later waves compare error SETS, not absolute zero.
- `🧪️export-snapshot-before.txt` — 1,186 unique export symbols, 1,094 export statements (byte-identity
  target for every future W3 extraction step).
- Confirmed pre-existing build health of all three UI Rust crates before any move: `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-ui-wgpu -p semio-framework-ui-tui -p semio-framework-ui-styling`
  → clean, no errors. `@semio-tech/ui-wgpu-rs:check` → green ("ui axes are fresh") — **not red** as an
  earlier exploration pass had assumed; already fixed by other work, one fewer W0 blocker.

## W2 status: react half COMPLETE + verified; Rust half written, reviewed, cargo-check pending external unblock

Extracted the `Select` pilot element across all three targets, plus the `ElementId` schema-core pair,
and wrote `📋️TEMPLATE-UI.md` from what actually happened (real recipe, not a guess).

**React (fully verified)**: `Select` (all 10 exports) and `ElementId` (7 exports, incl. `isElementId`)
extracted to `🧱️elements/Select/🟦️component.tsx` and `🧱️elements/🫀️core/ElementId/🟦️component.tsx`.
Found and fixed a real bug in the barrel-rewiring pattern: a bare `export { X } from "leaf"` does not
create a local binding, so other code still living in the same barrel that references `X` unqualified
breaks (`Cannot find name`) — the correct pattern is `import { X } from "leaf"; export { X };`.
`bun ./📜️script.ts typecheck` after both extractions: **96 errors, identical set to the W1 baseline,
zero new, zero in the new files** — confirmed via full file+code diff, not just count.

**wgpu + tui (written, hand-verified, cargo check pending)**: extracted `render_select`/
`render_select_menu` (wgpu) and `select_on_key`/`paint_select` (tui) into
`🧱️elements/Select/🧊️component.rs` / `⌨️component.rs`. Mapped every helper dependency to its real
top-level `pub mod` (wgpu has ~22 crate-root sibling mods, not one flat namespace) and wired both as
crate-root-sibling `#[path]` modules — **not** nested inside `widgets`/`widget`, because rustc resolves
a `#[path]` on a module nested inside an *inline* parent as if that parent had its own on-disk
directory, which fails outright for a block that has no such directory (confirmed by reproducing the
"No such file or directory" error twice before landing on the working crate-root-sibling pattern).
**Caught a real bug via careful re-reading** (compiler unavailable at the time, see below): inserting
`mod select;` immediately before `pub mod widgets {` accidentally orphaned that mod's pre-existing
`#[cfg(feature = "engine")]` attribute onto my new line instead — fixed by giving each declaration its
own `#[cfg(feature = "engine")]`. Brace-balance-checked both `component.rs` files and both `lib.rs`
files as an extra manual sanity pass (23/23, 18/18, 3928/3928, 1189/1189).

**Why cargo check hasn't run to completion**: the shared workspace `Cargo.toml` has been under
continuous, heavy, unrelated concurrent churn — first the `📜️imperative` plugin migration (root
member paths pointed at a deleted crate dir for 10+ minutes), then, once that resolved, a **second**
transient break from what looks like the `📕️norm` plugin migration (107 crates, per repo docs) briefly
double-registering `semio-s-plugin-norm` under both its old and new taxonomy paths. Polled via a
background Monitor (20× 15s) plus several manual retries; the workspace has not had a clean window
long enough to run `cargo check` on the moved crates. This is unrelated to any file this ticket
touches — confirmed each time by reading the specific error (always a different plugin's manifest
path, never `ui-wgpu`/`ui-tui`/`ui-styling`) and cross-checking `git status --porcelain` for that
plugin's own in-flight changes.
**Action**: will retry `cargo check -p semio-framework-ui-wgpu --features engine -p
semio-framework-ui-tui` opportunistically as other waves proceed; not blocking further TS-side W3
work, which doesn't depend on the Cargo workspace at all.

**Update (~1h later)**: polled `cargo metadata` repeatedly over ~50 minutes (two Monitor passes, 40×15s
then 15×120s, plus manual retries) — the workspace never reached a stable window. The blocking error
kept *changing identity* (norm's duplicate-package-name conflict → norm's own deleted-then-not-yet-
recreated crate path → a malformed dependency-version TOML syntax error elsewhere under
`✏️s/🔌️plugins` → most recently a `🖍️draw`-plugin dependency failure surfacing via the shared
`dsl 🧪️fixture-sweep` crate), each one clearly unrelated to any file this ticket touches, consistent
with **multiple large concurrent migrations actively churning root `Cargo.toml` simultaneously**
(`git status` shows 248 files changed under `📕️norm` alone — a 107-crate migration per repo docs — on
top of the earlier `📜️imperative` one). This is not a narrow window that will clear soon; it's
sustained, repo-wide, multi-session activity. **Decision**: stop waiting synchronously on a fully green
`cargo metadata` for the whole ~600-member workspace — that is explicitly expected per the crate-
consolidation master ticket's own verification philosophy ("transient reds from other in-flight
migrations are expected — check yours only"). The wgpu/tui Select extraction's correctness rests on
the thorough manual review already performed (documented above: module-path mapping via
`grep -n "^pub mod "`, brace-balance sanity checks, and two real bugs actually caught this way — the
crate-root-vs-nested `#[path]` resolution failure and the `#[cfg(feature = "engine")]` orphaning bug).
**This is reported as "written and hand-verified, cargo-check-pending" — not as "verified" — until an
actual green `cargo check` runs.** Next session/agent picking this up: try
`DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-ui-wgpu --features
engine -p semio-framework-ui-tui` first, before assuming more work is needed here.

**Update (workspace stabilized, W2 Rust half now fully green)**: `cargo metadata --no-deps` succeeded
(exit 0) on a later retry — the concurrent multi-session churn documented above eventually settled.
`cargo check -p semio-framework-ui-wgpu --features engine -p semio-framework-ui-tui` then surfaced one
**real, pre-existing bug**, unrelated to the hand-review above: 2 of wgpu's 15 `include_bytes!` font
paths (`🔤️10-400.ttf`/`🔤️11-400.ttf`, the `🖱️ui`-local noto-emoji glyphs — NOT the 13 sibling lines
pointing at the separate top-level `🧰️framework/🔨️modules/🖼️assets` module, which were untouched and
correct) still had the pre-W5 `⚡️implementations/🟦️typescript/` segment baked into their relative
path. The W5 assets de-sandwich flattened `🖱️ui/🖼️assets` to lose that wrapper
(`🖱️ui/🖼️assets/🔤️fonts/…` instead of `🖱️ui/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/…`), but
that Rust-side reference was never updated — a real, if narrow, gap in W5's own dependent-sweep (W5's
own status section documents sweeping ui-react/styling/vite-config consumers of the OLD assets path,
but this wgpu font path is neither — it's a `#[path]`-adjacent literal string inside a `📦️lib.rs` that
no grep for "the moved package's name" would find, since it references a plain data file, not an
import). Fixed by removing the stale segment (confirmed via `ls` against the real on-disk location, not
guessed). Re-ran: **`cargo check -p semio-framework-ui-wgpu --features engine -p semio-framework-ui-tui`
→ clean, exit 0** (5 pre-existing unused-import/qualification warnings, zero errors). Also confirmed
`cargo check -p semio-framework-ui-wgpu --target wasm32-wasip2` → clean. **The W2 Select pilot's Rust
half (wgpu `render_select`/`render_select_menu`, tui `select_on_key`/`paint_select`, plus the
`🫀️core/ElementId` schema pair) is now fully verified, not just hand-reviewed** — the crate-root-sibling
`#[path]` wiring and the `#[cfg(feature = "engine")]` fix from the original pass both hold up under a
real compile.

## W3 status: react track COMPLETE (50/50 elements), wgpu/tui tracks COMPLETE

Ran a 7-batch Workflow (sequential agents, shared-file serialization) covering every remaining
`element:`-classified region from the inventory. **All 7 batches succeeded, 0 errors, 0 skipped.**
Independently re-verified afterward (not just trusting the agents' self-reports):
- Barrel line count: 40,376 → 25,035 (≈15,341 lines extracted).
- Region balance: 0 real mismatches (same 3 pre-existing cosmetic label mismatches as always), 0
  leftover — confirmed with a fresh stack simulation, not reused output.
- `bun ./📜️script.ts typecheck`: **96 errors**, exact match to the post-W1 baseline, run fresh myself.
- Cross-checked the error list: errors now appearing under `🧱️elements/{Canvas,Diagram,IconSelector,
  Tree,UIDialog}/🟦️component.tsx` are the *same* pre-existing bug categories (xyflow `OnNodeDrag`
  mismatch, `iconId` union-type mismatch, `ContextMenuItem[]` mismatch, `CSS.escape` shadowing,
  i18next `unknown`-to-`ReactNode`) that lived in `📦️index.tsx` before extraction — moved verbatim
  with their code, not introduced.
- 51 element directories now exist under `🧱️elements/` (50 elements + `🫀️core`); 50 files still carry
  the `🚧️W3-interim` barrel-dependency marker (expected — the shared "core" regions like Adapters/
  Utilities/i18n haven't been extracted yet, a deliberately deferred follow-up).

**Notable judgment calls the batches made correctly** (each caught via typecheck, not guessed):
duplicate `Dialog` name resolved by checking real exports (`UIDialog` vs `Dialog`, two genuinely
different components); ~15 forward-dependency symbols that were extracted but still referenced
unqualified by *other, not-yet-extracted* barrel code got `export` added in place (mechanical, no
logic change) rather than silently breaking; newly-extracted sibling elements (e.g. `Ring`→`Orb`,
`Band`/`Strip`→`Scrollable`, `ShellFindDialog`→`ShellSearchDialog`) wired as direct element-to-element
imports instead of routing back through the barrel, per the recipe. Batch 7 flagged that `Canvas`
(~2000 lines, contains nested `Mode`/`App`/`Ui` sub-concerns) is a reasonable candidate for further
sub-decomposition in a later pass — not attempted now, per its instructions.

**wgpu + tui tracks: COMPLETE.** Once the Cargo workspace stabilized (see the W2 Rust update above), ran
a 6-agent Workflow (2 tracks — wgpu, tui — each internally sequential since both edit one shared crate
`📦️lib.rs`; the two tracks ran in parallel against each other since they're different crates/files) to
extract every remaining widget. First did an Explore-agent investigation to map each Rust widget
function to its real matching react element concept (not just name similarity) before touching any
code — this caught two non-obvious calls worth recording: `render_key_value` is genuinely a different
concept from the existing `Field` element (Field = one label + one control wrapper; KeyValue = N
label→value rows) so it got its own new dir, not a Field reuse; `render_ring` matches the existing
`Ring` element (track + drag-knob), not the smaller `Orb` sub-part.

wgpu (`semio-framework-ui-wgpu`): `render_button` → **NEW** `Button` dir (react's own `Button` is still
barrel-inline, unextracted — no coordination needed, wgpu leaf only), `render_input` → `Input`,
`render_toggle` → `Toggle`, `render_key_value` → **NEW** `KeyValue`, `render_slider` → `Slider`,
`render_number_stepper` → `Stepper` (not `Steps`, a different progress-indicator concept),
`render_icon_select` → `IconSelector`, `render_ring` → `Ring`, and the 9-function `render_tree` cluster
(`measure_tree_sections_width[_state]`, `measure_tree_item_width`, `measure_tree_sections[_state]`,
`measure_tree_item_height`, `render_tree`, `render_tree_section_header`, `render_tree_item`) → `Tree`.

tui (`semio-framework-ui-tui`): `list_on_key`+`paint_list` → **NEW** `List`, `tabs_on_key`+`paint_tabs`
→ `Tabs`, `input_on_key`+`paint_input` → `Input`, `log_on_key`+`paint_log` → **NEW** `Log`,
`table_on_key`+`paint_table`+`paint_table_cell` → `Table`, `paint_label` → **NEW**
`🫀️core/Label` (a foundational primitive used everywhere, not a standalone element — mirrors ui-react's
own not-yet-extracted `Label`, which the agent found still barrel-inline too), `paint_divider` →
**NEW** `Divider`, `paint_chip` → **NEW** `Chip`, `paint_navbar`+`paint_items` → `Navbar`,
`paint_footer` → `Footer`, `paint_window`+`paint_corner_tab` → `Window` (the last 5 functions live in
`pub mod chrome`, not `widget` — confirmed via grep before wiring, wired as chrome-mod crate-root
siblings instead of widget-mod ones).

**Real judgment calls the batches made, each backed by a compile check, not guessed**: `input` as a
module name collided with wgpu's pre-existing `pub mod input` (HitKind/HitTarget/InputState) — renamed
the sibling to `input_element` (documented in-file); same collision for `tree` vs. wgpu's existing scene
mod `pub mod tree` — renamed to `tree_element`. Several widgets-mod-private helpers/consts needed a
`pub(crate)` bump to stay reachable from their new sibling module while remaining shared with
still-inline dispatch code (`measure_text_width`, `register_input_meta`, five `tree_*` gutter/chevron
helpers + 5 `TREE_*` consts, `WindowTab`/`WindowChipLayout`/`window_chip_layout` for the chrome-mod
Window pair) — each verified as genuinely still-shared (not movable) before bumping visibility rather
than moving them wholesale. tui's `LogState.lines` field access was rewritten to use the existing public
accessor method instead of the private field it used to share module scope with.

**Independent re-verification (not just trusting the 6 agents' self-reports)**: ran
`cargo check -p semio-framework-ui-wgpu --features engine -p semio-framework-ui-tui` myself against the
final stacked state of all 6 batches — clean, 0 errors. Found and fixed one real regression the batches
left behind: 8 unused-import warnings in wgpu's `widgets` mod (a `use crate::chrome::{...}` block and
`DragAxis` that were only needed by the now-moved `Button`/`Toggle` code, plus 5 of the 9 tree-cluster
names that `widgets`' own dispatch never calls directly) — trimmed by hand, re-checked: back down to
the exact 5 pre-existing warnings, 0 new. Also independently confirmed
`cargo check -p semio-framework-ui-wgpu --target wasm32-wasip2` (the actual shape-plugin consumption
target) clean, and ran `cargo test -p semio-framework-ui-tui --lib` myself: 76 passed, 1 failed
(`window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape`, a stray VS16
emoji-variation-selector baked into an unrelated `chrome`-mod test string literal at line ~2417 —
confirmed via `git diff --stat` that this session's entire diff touches 0 lines in that test's region,
genuinely pre-existing, not this ticket's bug).

wgpu's generic `measure_control`/`render_control`/`measure_widget`/`render_widget` dispatchers correctly
stay in `widgets.rs` (not extracted, per the original plan — they're shared dispatch, not per-widget
code), same for tui's `ChromeState::paint`/`window_control_at` dispatch layer in `chrome`.

## W3 follow-up: module-top-level circular-import bug found + fixed (reactHostPort/cn/sceneHostPort/uiDataLabel)

Post-W3 regression, found via `bun ./📜️script.ts test` (not typecheck — this class of bug is invisible to
`tsc`, only surfaces at module-load time): `TypeError: undefined is not an object (evaluating
'reactHostPort.forwardRef')` in `Avatar/🟦️component.tsx`, reproduced independently on ui-react's own test
suite (a genuine W3 regression, not renderer-specific).

**Root cause**: an ES-module circular-import initialization-order bug. The barrel imports each extracted
element (to re-export it); several elements import symbols back from the barrel via the `🚧️W3-interim`
marker. When a `🚧️W3-interim`-imported symbol is a barrel-defined `const`/`export let` and the *consuming*
element reads it at MODULE TOP LEVEL (not inside a function/component body), the read can land in that
binding's temporal-dead-zone: whichever module the loader reaches first in the cycle sees the other's
top-level `const`/`let` still uninitialized. Elements that only read the symbol inside function bodies are
unaffected (evaluation happens at render time, long after both modules finish loading) — this is why only
a handful of the 50 extracted elements tripped it.

**Found 4 instances, one at a time via the test suite** (fix one, rerun, next error surfaces — no way to
find them all statically, since it depends on which symbols are read at column-0 vs. inside a closure):
1. `reactHostPort.forwardRef`/`.createContext`/`.memo` at module top level in **Tree, Canvas, ActionGroup,
   ToggleGroup, Avatar, Scrollable** (6 elements; `Panel` was a 7th candidate from the initial grep but its
   usage turned out to be inside a function body — false positive, left importing via the barrel).
2. `cn(...)` at module top level (inside a top-level `cva(cn(...))` call) in **ActionGroup, Toggle** — a
   *different* barrel `const` (`twMergeUi`, `cn`'s dependency) hitting the same TDZ pattern.
3. `sceneHostPort.drei.Line` at module top level in **Scene**.
4. `uiDataLabel(...)` at module top level (inside a top-level demo-fixture object literal) in
   **VirtualFileSystem**.

**Fix, same shape every time**: extract the affected symbol (+ its minimal dependency closure) out of the
barrel into its own `🧱️elements/🫀️core/<Name>/🟦️component.tsx` file with no import back into any element,
then have the barrel `import`-then-`export` it from there (never a bare `export { X } from "leaf"` — that
doesn't create a local binding, breaks other same-file barrel code referencing `X` unqualified, per the W2
lesson already in `📋️TEMPLATE-UI.md`), and have every affected element import the symbol **directly** from
the core file instead of via the `🚧️W3-interim` barrel path. New core files landed:
- `🧱️elements/🫀️core/Ports/🟦️component.tsx` — `ReactHostPort`/`reactHostPort`/`setReactHostPort` +
  `SceneHostPort`/`sceneHostPort`/`setSceneHostPort`. `flowHostPort`/`threeHostPort`/`iconRenderPort`
  stay in the barrel (no top-level consumer). Since `export let` bindings can't be reassigned by an
  importer, the barrel's `configureHostPorts` now calls the exported `setReactHostPort`/`setSceneHostPort`
  setters instead of a direct `=` assignment for these two ports specifically.
- `🧱️elements/🫀️core/ClassNames/🟦️component.tsx` — `cn` (+ its private `twMergeUi`). Barrel's now-unused
  `ClassValue`/`clsx`-type-import and `extendTailwindMerge` adapter imports removed (the standalone
  `export { clsx } from "clsx";` re-export is untouched, doesn't need them).
- `🧱️elements/🫀️core/UiLabel/🟦️component.tsx` — `UiLabel` (branded type) + `uiDataLabel`.

**Verification**: `bun ./📜️script.ts test` went from a hard module-load crash (0 tests could even start) to
**512 tests running, 504 passing**. The remaining 8 failures are pre-existing and unrelated to this fix —
confirmed each is a genuine test-logic/mock issue (`camera.updateMatrixWorld is not a function` on a bare
test-double camera, a missing `[data-icon="beam"]` CSS-rule assertion, jsdom `event.target?.closest`/
`Node.contains` gaps in unrelated pointer-event tests) with no "before initialization"/`undefined` host-
port or barrel-symbol signature — not another instance of this bug class. `bun ./📜️script.ts typecheck`:
**96 errors, exact match to the running baseline**, none referencing the new core files or the touched
symbols — confirmed via grep, not just count. `grep -rn "^const [A-Za-z0-9_]* = (reactHostPort|
sceneHostPort)\."` and the module-top-level `cn(`/`uiDataLabel(` scans now return only the fixed files'
new direct-core imports, zero remaining barrel-routed top-level reads of these four symbols.

**Lesson for `📋️TEMPLATE-UI.md`** (not yet written back into that file — flagging here so W7/future waves
don't rediscover this): before extracting any element, grep it for barrel-`const`-typed symbols used
**outside** a function/component body (module top level, including inside top-level object/array literals
like demo fixtures or `cva(...)` calls) — those need a direct core import from day one, not a
`🚧️W3-interim` barrel import, because the barrel↔leaf cycle makes the barrel copy's initialization order
unreliable. A symbol read only inside hooks/render bodies is always safe via the barrel.

## W4 renderer-engine-wgpu package move: COMPLETE (structurally AND content-clean — see update below)

Moved `semio-framework-os-renderer-wgpu` (25,433-line `📦️lib.rs`) from
`…📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust/` to
`…📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`, mirroring the W1 template exactly
(delegated to a background agent with the W1 section as its explicit reference; independently
re-verified afterward, not just trusting the report). This crate had NO downstream path-dependents
(confirmed: only root `Cargo.toml`'s member list referenced it), so no cross-repo dependent rewrite was
needed — simpler than the W1 moves. 17 dependency paths recomputed and `existsSync`-verified (not
hand-counted); root `Cargo.toml` got exactly one line changed (member path) — confirmed via isolated
`git diff` on just that line, the file's much larger overall diff is unrelated concurrent-session churn
(a different plugin's entries being removed by another session, not touched by this move). Added
`[package.metadata.semio] role = "framework"` marker matching the sibling crates. Also fixed extra
files this crate has that `ui-wgpu` didn't: `project.json` (cwd/schema-depth/`namedInputs.default`),
`script.ts`, `build.rs`, `package.json` (whose `$schema` path turned out to be **pre-existing broken**,
pointing at a nonexistent location even before this move — fixed as a drive-by), a `🟦️typescript/`
subdir's own import, plus two real in-`lib.rs` `#[path]`/`include_bytes!` depth bugs caught directly by
cargo's own error output. Repo-wide swept and fixed real dependents: root `package.json` workspaces
glob, `.storybook/scopes.ts`, `.vscode/launch.json` + `🧩️launch.seed.jsonc` (confirmed via
`bun 🖥️launch.ts generate --check` → "is fresh", i.e. no-op regen as expected), `os/dev`'s
`📜️script.ts`/`⚙️vite.config.ts`.

**Verification, independently re-run myself** (not just the agent's self-report): old dir gone, new dir
has all 14 expected files; root `Cargo.toml` member line correct; `cargo metadata --no-deps` for the
whole workspace → exit 0. `cargo check -p semio-framework-os-renderer-wgpu` → **3 pre-existing content
errors** (`E0425` cannot find type `HostEffect`, `E0433` cannot find module `dsl_core`, `E0308` argument
mismatch on a `request_media_frames`-shaped call), confirmed identical (same codes, same call sites) both
before and after the move by the agent, and I independently reran the check post-move myself and got the
exact same 3 errors, nothing more, nothing new — genuinely a directory-structure-independent content bug
deep in application logic (plausibly fallout from another session's in-flight `framework-core`/`dsl` API
change, per this repo's known concurrent-churn pattern; `git status` shows no currently-uncommitted
changes in those dirs, so if it's fallout it's from an already-landed change elsewhere). **Not fixed —
genuinely out of this taxonomy-migration ticket's scope**, flagging for a separate ticket. Also confirmed
`cargo check --features native-bin` (same 3 errors, lib fails before the bin target is even attempted)
and `cargo check --target wasm32-unknown-unknown` (this crate's real wasm target, confirmed via its
`web-sys`/`wasm-bindgen` cfg block — NOT `wasm32-wasip2` like `ui-wgpu`; 7 errors: the same 3 plus 4 more
from `dsl`/`store` being used unconditionally in `lib.rs` despite being wasm32-gated out in `Cargo.toml`
— same root cause, same out-of-scope call).

**Update: fixed all 3, plus a 4th latent bug the fix exposed.** Re-examined instead of deferring, since
leaving a crate in the taxonomy tree that doesn't compile blocks ever verifying its own widget
extraction. All 3 were genuine, narrow, mechanical bugs, not deep design problems:
1. `E0425 HostEffect` — the outer `program_bridge` mod's own `use` block (line 5909) was simply missing
   `HostEffect`; the inner `wasm_program_exchange` sub-mod 20 lines below it already correctly imported
   `semio_framework_core::kernel::HostEffect` — added the same import to the outer mod.
2. `E0433 dsl_core` — a genuinely missing Cargo dependency, not a missing `use`: `dsl_core::decode_fault_bytes`
   needs the separate `semio-framework-os-kernel-dsl-core` crate (`🗣️dsl/🫀️core/⚡️implementations/🦀️rust`),
   which this crate's `Cargo.toml` never listed (only its sibling `dsl` = `semio-framework-os-kernel-dsl`
   was present). Confirmed the exact dependency-declaration pattern from 7 other crates that already
   depend on it (`dsl_core = { path = "...", package = "semio-framework-os-kernel-dsl-core" }`),
   computed+verified the relative path from this crate's NEW location with `path.relative()`, added it
   to `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` alongside `dsl`/`store`/`protocol`.
3. `E0308 request_media_frames` — 4 destructured `String` fields (`accept`/`frame_action`/`done_action`/
   `fallback_action`) passed by value where the callee wants `&str` — took references at the call site
   (Deref coercion handles `&String` → `&str` automatically), zero behavior change.
4. **Latent 4th bug, only surfaced once #1-3 stopped masking it**: `--features native-bin` failed with
   `E0432 unresolved import semio_framework_renderer_wgpu` — `📦️bin.rs` used the WRONG crate name
   (missing the `os_` segment the actual package name `semio-framework-os-renderer-wgpu` requires,
   confirmed against `[package] name` and `run_native`'s real definition site in `lib.rs`). Fixed the
   one-line import. This bug could never have been *seen* before, since `cargo check`'s default run
   fails on the lib target first and never even attempts the bin target while errors #1-3 stood.

Re-verified after all 4 fixes: `cargo check -p semio-framework-os-renderer-wgpu` → **0 errors** (138
pre-existing warnings, unrelated dead-code/style lints, not this ticket's concern), confirmed via both
grep-for-error-lines and exit code. **`--target wasm32-unknown-unknown` verification is currently
blocked** by an unrelated, external, in-flight concurrent session: `cargo`'s workspace resolver reports
"multiple workspace roots found" because another session has an uncommitted `[workspace]` table added to
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` (confirmed via `git status` — modified +
untracked `Cargo.lock`, someone else's in-progress edit, not touched by this ticket). This is a
workspace-wide resolver error, not specific to this crate — it currently blocks `cargo metadata` for the
ENTIRE repo, not just this check. Not something to fix myself (not this ticket's file, another session's
active work) — backgrounded a Monitor watch waiting for it to clear and will complete the
wasm32-unknown-unknown verification once it does, rather than block on it or claim false certainty.

**wasm32-unknown-unknown target: still 5 errors, NOT fixed, judged genuinely out of scope.** Once the
external "multiple workspace roots" resolver conflict cleared (another session's uncommitted
`[workspace]` table in `🌀️procedural`'s Cargo.toml — confirmed via `git status`, not this ticket's file,
left untouched), re-ran the wasm32 check: `dsl::DslValue`/`store::pack_rt::*` are called
UNCONDITIONALLY at 5 call sites (in `dock`, `program_bridge`, and `shell` modules) despite both crates
being declared under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` in `Cargo.toml` —
meaning either those call sites need `#[cfg(not(target_arch = "wasm32"))]` guards, or `dsl`/`store`
genuinely need to be available on wasm32 too (the crate's own doc header calls itself a "WASM renderer",
so excluding them entirely could itself be the actual bug). Unlike the 4 fixes above (missing import,
missing dependency, type coercion, wrong crate name — all mechanical, single correct answer), this
needs an architectural call about which side is wrong, which risks changing real runtime behavior on the
crate's primary target rather than a structural fix. Also: this crate's `Trunk.toml`/`🌐️index.html` were
already flagged during the move as having broken/unreachable relative paths pre-dating this ticket,
suggesting its wasm-specific build pipeline may already be stale/unmaintained in practice. Left
unfixed — flagging for a separate ticket with someone who knows the intended wasm/native split here.
Plain `cargo check`/`--features native-bin` (what regular dev workflows actually use) are both clean.

**W4 renderer-engine-wgpu module extraction: all 7 modules written, hand-verified; compiler verification
pending sustained external Cargo-workspace churn.** With the crate's default-target build clean, ran a
7-agent sequential Workflow (one shared `lib.rs`, strictly serialized) splitting its 7 top-level `pub
mod` blocks — `dock` (1949 lines, NEW `Dock` element, no react counterpart exists yet), `engine_canvas`
(2019 lines, NEW `EngineCanvas`), `interpreter` (1910 lines, existing `Interpreter` dir, react
counterpart already extracted), `program_bridge` (NEW `ProgramBridge`), `scenes` (NEW `Scenes`, kept as
one file per this ticket's precedent of not force-splitting large cohesive blocks further), `shell`
(existing `Shell` dir), `icon_atlas` (existing `IconRenderHost` dir) — into `🧱️elements/<Name>/🧊️component.rs`,
wired back via `#[path = "..."] pub mod <name>;` at the original crate-root location (module NAME never
changes, so every `crate::<name>::…` reference elsewhere in the file needs zero other edits). `mod
generated_plugin_hosts;` (already `#[path]`-backed to a registry-generated file) and the crate's own
`camera_dispatch_deadline_tests` mod were explicitly excluded, left in place. File went from ~25,434 to
a wiring-only crate root (each 7-line replacement).

**Every one of the 7 agents hit `cargo check` blocked by a DIFFERENT unrelated concurrent migration**
(trinity/jack, flow/bim, each confirmed via `git status` to be another session's genuine in-flight
work — missing `Cargo.toml` for a plugin mid-move, not this ticket's file) — sustained, not transient,
matching this exact pattern already documented for W2's Select pilot earlier in this file. Each agent
fell back to the same rigorous hand-verification standard already proven reliable in this ticket:
line-count arithmetic checked before writing, `diff` of the unchanged head/tail of `lib.rs` before vs.
after, and a **byte-for-byte diff of the reconstructed original body vs. the actual extracted file**
(not just "looks right by eye") for every module. Brace-balance re-checked at depth 0 in both the
trimmed `lib.rs` and each new leaf file. I independently re-ran `cargo metadata` myself afterward and it
was STILL blocked (this time by an `imperative`-plugin in-flight move) — confirming the churn is
sustained across the whole verification window, not one-off bad luck. Backgrounded a Monitor watch for
the workspace to clear; will run the actual `cargo check -p semio-framework-os-renderer-wgpu` compiler
verification the moment it does, per this ticket's standing rule to report "written and hand-verified,
cargo-check-pending" rather than claim false certainty until an actual green compile confirms it.

## W4 follow-up: renderer-engine-react barrel had incomplete re-exports (module-load crash, separate from the ui-react circular-import class)

While verifying the ui-react fix above, ran `bun ./📜️script.ts test` on `framework-renderer-react`
(`…📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react`) too — same hard module-load crash
(`TypeError: shellLabel is not a function`, `'shellLabel' is undefined`) from `Shell/🟦️component.tsx`.
Fixed `Shell`'s own `shellLabel(...)`-at-module-top-level read the same way as ui-react (imports it
DIRECTLY from `../ShellHelpers/🟦️component.tsx`, its real implementation, instead of via the
`🚧️W4-interim` barrel path) — but re-running the FULL suite (`vitest run --reporter=verbose` directly;
the `bun ./📜️script.ts test` wrapper's own budget wrapper unexpectedly killed itself at both the 15s
`fundamental` and 300s `long` levels for unrelated reasons, direct invocation ran in 9s) surfaced a
**different, pre-existing bug class**: the barrel's `//#region ShellHelpers` re-export block
(`📦️index.tsx:704-778`) was simply missing 5 of `ShellHelpers`'s 72 real exports —
**not a timing/circular-import issue, a plain incomplete rewiring** left over from whichever earlier W4
batch created that region. Diffed `ShellHelpers`'s actual `export`ed names against the barrel's
import/export list programmatically (not by eyeballing 70+ names) to find all 5 at once: `shellLabel`,
`shellTabIcon`, `shellTerminologyLabel`, `driverDisplayLabel`, `SelectionUtilityOptions` (type). Added
all 5 to both the barrel's `import {...}` and paired `export {...}` statements (same import-then-export
rule as everywhere else in this ticket).

Iterating test → fix → retest surfaced 3 more instances of the identical "real symbol exists but was
never wired through" bug, each a `ReferenceError`/`undefined` at its actual USE site (not a TDZ), each
fixed by tracing the real definition and adding it wherever the chain was broken:
- `DEFAULT_PANEL_WIDTH_PX` (`ShellHelpers`) was a bare `const`, not `export const` — never reached the
  barrel even though `Shell`/`ShellHost` both already imported it via `🚧️W4-interim`. Added `export`.
- `registeredPuzzle3dBrushMeshes` (`ShellHelpers`) — same bare-`const` bug, PLUS `World3dHost` (its only
  consumer) never imported it at all (not even attempted) — added `export`, added it to `World3dHost`'s
  `🚧️W4-interim` import list (safe there — used only inside a `useRef` initializer, not module top
  level), and to the barrel's re-export list.
- `formatKeybindingShortcut`/`buildKeysByActionId` — both genuinely `export`ed by the SEPARATE
  `@semio-tech/ui-react` package (not this barrel) and correctly imported that way inside the barrel
  itself, but `🧪️index.test.ts` imported them from the LOCAL barrel (`./📦️index.tsx`) instead of
  `@semio-tech/ui-react` directly — a mis-categorized import in the test file (everything else from
  `ui-react` in that file, e.g. `Footer`/`uiDataLabel`/`SelectionMarquee`, was already on the correct
  import line). Moved both to the `@semio-tech/ui-react` import line.

**Verification**: went from a hard crash (0 tests could load) → 290/302 → 294/302 → 298/302 passing as
each layer of missing wiring was fixed, confirmed via direct `vitest run` (not the flaky wrapper) each
time. `bun ./📜️script.ts lint` (region/host-contract check, no separate `typecheck` command exists for
this package — no `tsconfig.json` under this dir) — passes clean. **The remaining 4 failures are a
different, unrelated class**, confirmed by inspection, not assumed: a 5000ms test timeout on an async
plugin-module-loading test with a misleading/unrelated stack trace (vitest attributing it to a later,
synchronous reducer test — a known vitest artifact, not evidence of what actually hung), a CSS
`ring-primary` class assertion, `Invalid Chai property: toHaveTextContent` (missing `@testing-library/
jest-dom` matcher registration in this package's vitest setup — an environment/setup gap, not this
ticket's concern), and a `logos`-vs-`logo` regex/asset-path mismatch in a `mit-bestand` demo brand test.
None reference an import, an `undefined`/`not defined` symbol, or initialization order — a materially
different failure signature from every bug fixed above.

**Lesson, folded into the same `📋️TEMPLATE-UI.md` entry as the ui-react circular-import fix**: after
ANY batch element-extraction pass creates a barrel's `🚧️W(3|4)-interim` re-export region for an
already-extracted source file, mechanically diff that source file's real `export`ed names against the
barrel's import/export list for the region (`comm -23` on two sorted name lists, not eyeballing) —
partial re-export lists silently produce `undefined` at every downstream `🚧️W(3|4)-interim` consumer,
and typecheck alone will not catch it if the barrel's own `import`/`export` line type-checks fine (the
TS types can be structurally sound while the runtime binding is simply absent from the list).

## W5 status: COMPLETE (styling ×4 langs, assets)

Moved the remaining three styling language packages into `🎨️styling/📦️packages/<lang>/` (rust was
already done in W1): `🟦️typescript` (`@semio-tech/ui-styling`), `🐍️python`
(`@semio-tech/ui-styling-py`), `🔷️dotnet` (no existing nx wiring, moved as-is). Fixed every dependent —
literal-path ones (package.json, os/dev + renderer-engine vite/vitest configs, storybook, mit-bestand)
and the same class of self-referencing-relative-path bugs found repeatedly in W1/W2 (styling-TS's own
`index.ts`/`index.test.ts`, ui-react's `🎨️tailwind.config.ts` and 7× inline `🎨️ui.css` reads,
styling-rust's `script.ts`) — each verified via `existsSync`, not assumed.

**Confirmed and deleted 2 dead stub directories** flagged as a TODO in W0
(`🎨️styling/🟦️typescript/🎨️styling/`, `🎨️styling/🐍️python/🎨️styling/🎨️styling/`): zero repo-wide
references to either, and their content was stale (the CSS stub still had the old broken Anta-font URL
shape, the generated-token stubs had extra/missing keys vs the real generated output) — genuinely
orphaned, not live duplicates.

**Assets: corrected course from the original plan.** `🖱️ui/🖼️assets` turns out to have no
`package.json`/`project.json` at all — just static files (fonts/icons/cursor/introduction/list) plus two
small TS helpers. Per the SHAPE V2 broadcast's own `rootDataDirNames` classification (`🖼️assets` is
explicitly listed as *data*, not a package), the correct move is flattening it to the owner root
(`🖱️ui/🖼️assets/*` directly, no `📦️packages/🟦️typescript/` wrapper) rather than packaging it — adjusted
from the original plan's assumption. **Important scope-boundary finding**: the widely-imported
`@semio-tech/assets` npm alias (used by `Tree`, `Canvas`, `VirtualFileSystem`, ui-react's own barrel,
renderer-react, vscode client, compose, mit-bestand, storybook — a dozen+ consumers) resolves to a
**different, top-level `🧰️framework/🔨️modules/🖼️assets` module**, NOT `🖱️ui/🖼️assets`. That module is
outside this ticket's declared scope (🖱️ui + renderer-engine + styling + assets meant *ui's own* small
assets folder) and was correctly left untouched — verified before assuming "assets" meant the big
shared package.

Also fixed 3 stale doc-comment path references in framework-core's `📦️index.ts` (comment-only, pointing
at wgpu's pre-W1 location) — noticed during the final sweep, safe/anticipated per the plan's schema-core
notes.

**Final verification**: `bun ./📜️script.ts typecheck` from ui-react → **96 errors**, exact match,
confirming zero regressions across the entire W5 change set (styling ×4 + assets + the 3 comment fixes).

## Coexistence note: SHAPE V2 Tree Purity Broadcast (26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST)

A separate, genuine user directive landed in the shared `🔣️taxonomy.json` mid-session (new
`entryLocation`/`rootDataDirNames`/`rootDataFileNames`/`rootDocFileNames` fields), tightening the
**plugin/framework crate** shape: entry files move from a plugin's owner root into
`📦️packages/<lang>/`, sibling variant files fold into their own `component.<ext>`, data/docs relocate
to owner root. Confirmed via that ticket's own description and the crate-consolidation master.md's
"🚨🌳️ SHAPE V2" notice that this is **explicitly scoped to exclude this ticket** — both state the
`🎯️targets`/`🧱️elements` axis is untouched/additive-only. No action needed: this ticket's packages
never had entry files at an "owner root" to begin with (they were designed to live inside
`📦️packages/<lang>/🎯️targets/<target>/` from W0 onward), so there is nothing to retrofit. Verified
`🔣️taxonomy.json`'s W0-added fields are all still intact after the broadcast's additive edit.

## Deliberately NOT done in W0
- No `taxonomy.json` `areas` entries for ui/renderer-engine yet (W6).
- No directory move, no root `package.json`/`Cargo.toml` edit (W1) — those touch files the concurrently
  active plugin-migration sessions (`🪐️space`, `🎪️demonstrator` per `git status`) are also editing, and
  the plan requires registrar coordination ("no in-flight plugin agent mid-Cargo.toml-write") before
  taking that step. Left for a dedicated W1 pass with that coordination confirmed.
- No content split (W2/W3) — blocked on resolving the unclosed-region finding above first.

## W1 status: COMPLETE (react, wgpu, tui, styling-rust)

Moved all four packages into the new shape and verified end-to-end:
- `🖱️ui/⚛️react/⚡️implementations/🟦️typescript` → `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` (`@semio-tech/ui-react`)
- `🖱️ui/🧊️wgpu/⚡️implementations/🦀️rust` → `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` (`semio-framework-ui-wgpu`)
- `🖱️ui/⌨️tui/⚡️implementations/🦀️rust` → `🖱️ui/📦️packages/🦀️rust/🎯️targets/⌨️tui` (`semio-framework-ui-tui`)
- `🖱️ui/🎨️styling/⚡️implementations/🦀️rust` → `🖱️ui/🎨️styling/📦️packages/🦀️rust` (`semio-framework-ui-styling`, two-level — styling has no targets axis)

Package/crate names preserved throughout. Each got a `[package.metadata.semio] role = "framework"` / `"semio": {"role":"framework"}` marker, updated `📋️project.json` (cwd, `$schema` depth, `namedInputs.default` override for the future `🧱️elements` dir), and `📜️script.ts` repo-lib import-depth fixes.

**Real bugs found and fixed along the way** (all pre-existing, surfaced only because the move forced every relative path to be recomputed):
1. wgpu's and tui's own `ui_styling` self-dependency was computed from their OLD location by my first-pass rewrite script, not their new one — both crates moved to a different depth than styling, so this needed a second, corrected pass (verified via `path.relative()`, not manual arithmetic, after the first attempt caught the error via `cargo check`'s manifest-load failure).
2. Both crates' `[lib] path` used a historical "up-and-back-down via the old absolute segments" self-reference trick (`../../../🧊️wgpu/⚡️implementations/🦀️rust/📦️lib.rs`) that broke once the segments changed — simplified to `path = "📦️lib.rs"` (lib.rs sits directly beside Cargo.toml, no traversal needed). Same fix applied to styling's `[lib] path` and its `#[path]`-attributed `generated.rs` include, and to wgpu's `#[path]`-attributed `icon_name.rs` include.
3. wgpu's font `include_bytes!` block (15 embedded `.ttf` files) had a **pre-existing internal inconsistency**: 13 lines used a "6-ups-then-explicit-`🧰️framework/🔨️modules/`" style, the last 2 (`10-400.ttf`, `11-400.ttf`) used a shorter "3-ups-then-bare" style — both needed the depth shift, done as one bulk find/replace per style.

**Cross-repo dependent rewrite**: 25 Cargo.toml files (23 external dependents + wgpu's and tui's own self-references) had their `path = "..."` dependency strings recomputed via a small purpose-built script (`path.relative()` from each dependent's own dir to the crate's new absolute location, verified against the crate's OLD absolute location before rewriting) — not hand-edited. Root `Cargo.toml`: 3 member lines + 2 `[workspace.dependencies]` aliases updated (styling, wgpu — tui has no workspace-alias entry, matches its much smaller dependent set).

**Verification, all green**:
- `cargo check -p semio-framework-ui-wgpu -p semio-framework-ui-tui -p semio-framework-ui-styling` — clean.
- `cargo check -p semio-framework-ui-wgpu --features engine` — clean (pulls in the full retained-mode engine dep graph: parley, wgpu, taffy, kernel_3d_scene, …).
- `cargo check -p semio-framework-ui-wgpu --target wasm32-wasip2` — clean (the shape plugins actually consume).
- `cargo metadata --no-deps` for the **whole workspace** — exit 0, confirming all ~600 members (not just the ones I touched) still resolve.
- `cargo clippy -D warnings` on the three crates surfaced 16 pre-existing lint issues in tui's own logic (checked_div, too-many-arguments, map_unwrap_or) — unrelated to the move (zero logic touched), left alone, out of this ticket's scope.
- `discoverPackages()` now finds all 4 new framework packages (`ui-react`/`ui-wgpu`/`ui-tui`/`ui-styling`) with correct `lang`/`target`, `area: "legacy"` (correct — the `areas` flip to `"taxonomy"` is W6), 0 discovery problems.
- Plugin registry `check`: reaches validation cleanly; the one reported error (`🔱️trinity/♻️rewrite` missing constitutional crate slots) is a pre-existing, unrelated plugin gap.
- `bun 🖥️launch.ts generate --check`: "`.vscode/launch.json` is fresh" — confirmed no-op as W0 predicted (no playground ports on these packages).
- ui-react `typecheck`: 96 pre-existing errors, all traced by absolute-path resolution to files this ticket never touches (framework-core's missing ts-rs bindings, repo-lib, an unrelated `🧰️framework/🔨️modules/🖼️assets` module, ui-styling readonly-property errors) — confirmed zero regressions.
- `bun install`: blocked repo-wide by **unrelated** stale workspace entries from the concurrently in-flight `🎞️animate`/`🧩️puzzle` plugin migrations (dangling `workspace:*` deps pointing at directories those migrations haven't finished cleaning up). I attempted a cleanup, it surfaced a *worse* error once bun got further into resolution, so I reverted it — not this ticket's bug to fix. Worked around it for my own package by hand-repairing just the `node_modules/@semio-tech/ui-react` symlink.

## Files touched
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/🟦️discovery.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🟦️typescript/📇️registry/📜️script.ts`
- `.🦑️repo/🎯️goals/AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/🎯️goal.json`
- `📜️script.ts` (root)
- `.storybook/scopes.ts`
- `.dependency-cruiser.cjs`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE/🗺️element-inventory.txt` (new)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE/🧪️export-snapshot-before.txt` (new)

## W7 status: 51/75 stories moved; found + fixed 3 more classes of pre-existing stale-path bugs the full storybook build surfaces

Ran an Explore-agent investigation first to determine, for each of the 75 `.storybook/stories/ui/*.stories.tsx` files, whether its component has an unambiguous single home under the new `🧱️elements/` tree. 51 had exactly one: moved to `🧱️elements/<Name>/🧪️story.tsx` via a 7-agent Workflow (6 parallel move batches — fully independent files, no shared-file contention, unlike the Rust tracks — plus 1 verification agent). Set `storyGlobs` on the "ui" scope in `.storybook/scopes.ts` to include both the legacy `./stories/ui/**` glob (for what stays behind) and the new `../🧰️framework/🔨️modules/🖱️ui/🧱️elements/**/🧪️story.tsx` glob — this field was purpose-built for exactly this scenario back in W0, its own doc comment anticipated it.

**24 stories deliberately NOT moved**, three genuinely different reasons, not one bucket:
- **10 lose a same-name-dir slot conflict**: `App`/`Mode`/`UI` all render symbols that live in `Canvas/🟦️component.tsx` (alongside `Canvas`'s own story), `FileTree`/`BasicChatPanel`/`SortableTreeItems` all live in `Tree/🟦️component.tsx`, `Geometry`/`UnifiedGumball` in `Scene/🟦️component.tsx`, `ActionDropdown` in `ActionGroup/🟦️component.tsx`, `NavbarExampleSelect` in `Navbar/🟦️component.tsx`. Storybook's CSF format allows exactly one `export default meta` (one `title`) per file, and the taxonomy's `storyLeafFilename` is a single fixed name per element dir — so a dir whose own name-matching story already claims that slot cannot also hold a second title group without either breaking CSF or extending the taxonomy (neither was in the approved plan). Kept the primary (name-matching) story moved, left every conflicting extra at the old location.
- **10 barrel-inline, no element home yet**: `ButtonGroup`, `CanvasPickMenu`, `ContextMenu`, `Engagement`, `IconShotFrame`, `Label`, `PanelTabBar`, `Providers`, `SelectionMarquee`, `UIIntroduction` — their tested component still lives inline in the ui-react barrel (never extracted to its own `🧱️elements/` dir by W3, since W3 only extracted components that already had a Storybook-matched name — these either never had one or got skipped). Can't co-locate a story next to code that doesn't have its own file yet. `DragAndDrop` is a genuine split (partly barrel-inline `DragHandle`, partly `Avatar`'s `DraggableAvatar`) — left whole rather than partially move.
- **2 out of scope entirely**: `OntologyTree`, `ValidationTree` import exclusively from `@semio-tech/coda-desktop/renderer` (the separate `compose` product's renderer, aliased in `scopes.ts`), not `@semio-tech/ui-react` at all — never belonged in this migration's element tree.

**Verification surfaced 3 more classes of genuinely pre-existing, unrelated stale-path bugs** — found only because this was, as far as I can tell, the first time `bun ./📜️script.ts build storybook` (`STORYBOOK_SCOPE=ui`) had actually been run to a real production-build completion since earlier waves moved things; `dev` mode and this ticket's own targeted `test`/`typecheck` runs never exercised these specific files. Each was confirmed pre-existing via `git diff`/`git show HEAD:<path>` before fixing, none caused by the story moves themselves (which only touch `.stories.tsx`↔`🧪️story.tsx` files, never CSS/worker/vite-config):
1. **`.storybook/globals.css`** (and its Tailwind `@source` sibling line) still `@import`ed the ui-react barrel's pre-W1 sandwich location (`⚛️react/⚡️implementations/🟦️typescript`) — a hard, build-fatal `@import`, unlike `@source` which is just a non-fatal glob hint. Also still referenced the renderer-engine react barrel's pre-W4 sandwich location. Fixed both, plus added `@source` entries for both new `🧱️elements/` trees so Tailwind's class-scanner covers the now-moved element/story files (the W2 template's own note: "Tailwind source detection must cover `🧱️elements/**/*.tsx`").
2. **8 more `globals.css` files repo-wide** (`os/dev`, 5 under `compose/`, 2 under `♻️mit-bestand/`) had the exact same stale pre-W1 `@import`, confirmed via a repo-wide grep — a genuine gap in W1's own original dependent-sweep (understandable: these live in totally different top-level products, easy to miss without a global grep). Of these, 3 had an ADDITIONALLY wrong `../` up-count that predates even the pre-W1 path (computed the correct depth with `path.relative()` for each rather than assume the old up-count was ever right) — a second, unrelated pre-existing bug layered on top of the first.
3. **`ShellHost/🟦️component.tsx`**'s lazy web-worker instantiation** (`new Worker(new URL("../../product/os/core/js/🟦️backbone-🟦️worker.ts", import.meta.url))`) pointed at a location that doesn't exist and had a doubled `🟦️` in the filename itself — the real file is `🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/🟦️backbone-worker.ts` (one `🟦️`, not two). Computed and verified the correct 5-up relative path from `ShellHost`'s current location.
4. **`@semio-tech/flow-module-bim` missing from `FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE`** in `🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts` — its 8 sibling flow-extension wasm modules (core/math/text/logic/dictionary/list/brep/draw) are all registered there so Vite doesn't try to statically prebundle/resolve them (they're loaded dynamically at runtime); `bim` is a real, fully-registered flow extension (has its own `📜️script.ts` wasm build router, confirmed) that was simply never added to this one list — a genuine oversight predating this ticket, unrelated to any directory move. Added it alongside its siblings.

Rebuilding (`STORYBOOK_SCOPE=ui bun ./📜️script.ts build storybook`) after each fix progressed further each time (152 → 103 → 3413 → 3727 modules transformed) — confirming each of the 4 fixes above was a real, sequential, correctly-diagnosed blocker, not noise.

**Final blocker hit, judged genuinely out of scope, not chased further**: at 3727 modules, Rollup fails
to resolve `"kerberos"` (a native Node addon, transitive dep of MongoDB driver auth) imported from
`node_modules/playwright-core/lib/utilsBundle.js`. Traced the only files under `.storybook/` that import
`@playwright`/`playwright` at all: the 11 `*.spec.ts` Playwright test specs (`ui-new-stories.spec.ts`,
`ui-uncovered-components-stories.spec.ts`, `playwright.config.ts`, and 8 others) — none of which are
`.stories.*` files, none of which this ticket's story moves touched or could plausibly affect (moving a
`.stories.tsx` file's physical location changes nothing about how Playwright's own tooling gets bundled).
This is a pre-existing gap in the production build's dependency-externalization list (native Node addons
transitively reachable from Playwright tooling were never added to `rollupOptions.external`, likely
because nobody had run a full `build storybook` for the "ui" scope to completion before this
verification pass) — completely orthogonal to UI element taxonomy, and fixing it properly would mean
auditing Vite/Rollup external config for an unrelated tool integration, out of this ticket's scope.
**Not fixed.** The story-move work itself is confirmed correct by every check that IS in scope: all 51
moves independently verified byte-identical, `storyGlobs` config verified loading correctly, and the
build's steadily increasing module-transform count (152→3727) as each of my 4 in-scope fixes landed
proves the story-location change itself was never the blocker at any point — every failure traced to a
file this ticket didn't move.
