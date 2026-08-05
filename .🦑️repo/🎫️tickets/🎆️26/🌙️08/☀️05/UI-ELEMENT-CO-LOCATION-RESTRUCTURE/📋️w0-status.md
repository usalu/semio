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
