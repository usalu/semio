# Build & Fixture Audit — Read-Only (no source files edited)

Scope: adversarial fixture grep + `cargo check --workspace` + TS test suites + rigorous root-cause
of the one known-pre-existing `os-kernel` failure. All commands actually run; real output pasted
below or saved as `.txt` in this folder. Attribution uses `git status --porcelain` (uncommitted =
this ticket's live tree) and `git log --date=iso` (never trust the commit-message date — it's a
frozen template per memory `feedback-auto-commit-message-date-is-fake`).

## 1. Fixture grep — adversarial verification

Grepped `.dsl` / `.dsl.semio` / `.txt` / `.json` under `✏️s/` and `🧰️framework/` (excluding
`🎯️target/`, `node_modules/`, `dist/`) for `selection|hover|hovered-|selected-|selection-mode|
selection-method|selection-targets|selected-ast-ids|feature-selection` plus the snake/camel
variants (`selected_ids`, `selectedIds`, `hoverAction`, `selectionChange`, …). ~256 raw hits;
triaged every one.

**Unrelated domain words (not the deleted config keys) — confirmed by reading context:**
- `✏️s/…/📐️cad/…/📚️examples/🔣️machine.json` (cad `machine.json` state-machine spec): `"on":
  "selection.changed"`, `interactionId: "selection.selectAll"`, etc. — this is the CAD command's own
  authoring-time state-machine event vocabulary (select-first-curve / select-second-curve gizmo
  steps), unrelated to the framework interaction mechanism. Also `"previewKind": "selected-objects"`
  in the CAD gizmo interaction jsons (rotate/scale/move/copy/mirror) — a preview-rendering enum
  value for the CAD tool's own transform gizmo, not app selection state.
- `✏️s/…/📋️forms/…/example.dsl.semio:2`: `"material selection."` — plain English prose in a form
  field description.
- `✏️s/…/🏛️architect/…/example.dsl.semio:106`: `selected-option-id` — a decision-record field in a
  governance/decision-log DSL block (`decisions [... options-considered:LIST selected-option-id:TEXT
  ...]`), unrelated to UI selection.
- `✏️s/…/📕️norm/📓️iso16757/…/example.dsl.semio:106`: `selection=class-id=class.valve series-id=...`
  — ISO 16757 product-catalogue "class/series selection" (which valve class to instantiate), an
  engineering-domain concept, not the deleted UI field.
- `🧰️framework/…/react/package.json:42`: `@radix-ui/react-hover-card` — npm package name.
- `🧰️framework/…/🕹️interaction/🧬️schema/🔣️component.json`: this IS the new framework module's own
  schema (by design), not a leftover.

**REAL problem found — but not the one the inventory anticipated.** No `.dsl`/`.dsl.semio` example
*data* fixture anywhere still carries the deleted keys (confirmed: the five `.dsl` workflow-graph
fixtures under `🧰️framework/…/🧫️fixtures/` are graph-scheduler fixtures with zero selection/hover
content; the 129 `.dsl.semio` example fixtures have none either — the only 3 kebab-case hits are
the unrelated-domain ones above). **But the *schema definition* layer is stale for 9 of the 17
in-scope plugins.** Each plugin's `🗿️artifacts/<artifact>/🏅️standards/<v>/🪆️subsets/✳️any/🧬️schema/
{🔣️component.json,🦀️component.rs}` pair (the "artifact standard" schema — a *separate* type from
the live `🎛️apps/<app>/🎚️config` type that W4 actually migrated) still declares
`selected_ids`/`hovered_id`/`selected_generation_id` as **required** fields, and git status shows
these files were **never touched**:

| Plugin | artifact-standard schema `.rs` mirror | `.json` leaf |
|---|---|---|
| `layout` | unmodified, still has the fields | unmodified, still has `"selectedIds"` required |
| `procedural` (2d) | unmodified | unmodified |
| `draw` | unmodified | unmodified |
| `puzzle` (2d) | unmodified | unmodified |
| `process` (3d) | unmodified | unmodified |
| `block` (2d/3d/5d) | unmodified (×3) | unmodified (×3) |
| `raster` | unmodified | unmodified |
| `gis` (gismap) | **modified**, field removed | unmodified — **now genuinely stale/drifted** |
| `gis` (gisterrain) | **modified**, field removed | unmodified — **now genuinely stale/drifted** |

Evidence (representative):
```
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:27
    #[state(presence)] pub selected_ids: Vec<String>,
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️component.json:10
    "selectedIds", "activeUtilityId", … (required)
$ git status --porcelain -- <both files>   →  (empty — untouched)
```
gis is the only plugin where the `.rs` mirror WAS cleaned (both `gismap` and `gisterrain`), yet
their sibling `.json` leaves were left stale — proving the drift isn't hypothetical: it's real and
happening (the `.rs` source of truth and its `.json` schema leaf now disagree for gis right now).
For the other 7 plugins neither file was touched at all, i.e. this artifact-standard-schema layer
was missed by the W4 migration wholesale (W4's summary only describes touching `🎛️apps/<app>/
🎚️config`, never `🗿️artifacts/<artifact>/…/schema`). Not consumed by any test in this session's run
(no failure observed from it), but it's a real, actionable residue: either intentionally out of
migration scope (the artifact-standard/interchange schema is a distinct type from the live app
config and may be deliberately untouched) or a genuine miss — worth a coordinator decision, not
assumed either way here.

Full raw grep saved: `fixture-grep-all.txt` was written to the scratchpad, not the ticket folder
(read-only task); the representative excerpts above are the complete real-finding set.

## 2. `cargo check --workspace`

Full output: `procedural-cargo-check-final.txt`-style — saved as scratchpad
`cargo-check-workspace.txt` (not copied into the ticket folder per read-only instructions; command
and result reproduced here). **100 workspace crates. 5 fail to compile; 95 are clean.**

```
error: could not compile `semio-compose-rs` (lib) due to 92 previous errors
error: could not compile `semio-s-plugin-architect` (lib) due to 9 previous errors
error: could not compile `semio-s-plugin-fem` (lib) due to 2 previous errors
error: could not compile `semio-s-plugin-mathematical` (lib) due to 3 previous errors
error: could not compile `semio-s-plugin-reasoning-mindmap` (lib) due to 4 previous errors
```

### `semio-compose-rs` — PRE-EXISTING, unrelated (another dev's in-flight ticket)
92 errors, all `E0433 cannot find 'dsl' in the crate root` (derive-macro hygiene) plus
`unresolved imports semio_framework_os_kernel::os_vcs::{create_document_vcs_envelope,
materialize_document_projection, ArtifactVcsEnvelope, ArtifactVcsStore, Operation, OperationDiff}`
and `cannot find ArtifactVcsCommand in os_vcs`. Evidence of independence:
- `git status --porcelain -- compose/` → empty (zero uncommitted diff anywhere in this crate).
- `git status --porcelain -- 🧰️framework/…/🌿️vcs` → empty (the `os_vcs` module itself is untouched).
- `git log --date=iso -- 🧰️framework/…/🌿️vcs` → last real commit **2026-08-12 15:50**, two days
  before this ticket opened (2026-08-14).
- The missing symbols (`ArtifactVcsStore`, `ArtifactVcsEnvelope`, …) match the currently-open
  sibling ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` (its scratch
  files are visible at the repo root of `git status` in this session) — a concurrent VCS/kernel
  rename in flight, not our hover/selection work.

### `architect`, `fem`, `mathematical`, `reasoning-mindmap` — IN SCOPE, genuinely caused by this
### ticket, missed by the W4 crate list
All 4 have **zero uncommitted changes in their own directories**
(`git status --porcelain -- ✏️s/🔌️plugins/{🏛️architect,🏗️fem,➗️mathematical,💡️reasoning}` → empty
for all four), yet all 4 fail with exactly the W1/W3 breaking-change signatures:
```
error[E0050]: method `handle` has 5 parameters but the trait declares 6
error[E0063]: missing field `interactions` in initializer of `WindowKindDefinition`
error[E0560]: struct `UiTreeNode` has no field named `selected_ids`/`highlighted_ids`/`selection_change`
error[E0599]: no method named `selection_change` found for struct `PanelTreeBuilder`
```
— i.e. these 4 crates implement `ArtifactApp`/build `WindowKindDefinition`/call
`PanelTreeBuilder`, but were never touched to add the `interaction: &InteractionView<'_>` param,
the `interactions: vec![]` field, or drop the deleted `.selected()/.selection_change()` calls. The
SDK files that changed underneath them (`🔌️plugin/🦀️component.rs`, `🛂️manifest/🦀️component.rs`)
**are** part of this ticket's uncommitted diff (`git status --porcelain` shows both `M`). Root
cause: the W4 migration briefs were built from the "17 crates with hover/selection *duplication*"
inventory (`📋️master.md`'s per-app list), but the breaking SDK signature change applies to **every**
`ArtifactApp` implementor in the workspace, including apps that never had their own selection/hover
fields to remove (architect, fem, mathematical, reasoning's `wires` app) — they still needed the
trivial mechanical update (add param, add empty `interactions`, delete the now-nonexistent
`.selection_change()` call) and nobody did it. `reasoning-mindmap`'s 4th error
(`E0432 unresolved import WIRES_PLAY_EXAMPLE_METABOLISM_ID`) is a same-crate cascading effect of
the other 3 (the constant IS declared and used correctly inside
`🎮️commands/🧬️set-active-example/🦀️component.rs`; the crate simply never finishes type-checking
far enough to resolve it once the other errors are present).

**This is the single most actionable finding in this audit**: 4 crates, all fixable by the exact
mechanical W4 pattern already documented in `w3b-summary.md`'s "New signatures every app must
implement" section — nothing novel needed, just apply it to these 4 stragglers.

## 3. TypeScript

### `bun ./📜️script.ts test` in `🧰️framework/📦️packages/🦀️rust`
Real run, exit 0: **cargo suite (`-p semio-framework`): 105 passed, 0 failed. vitest: 2 files, 146
tests, all passed.** (`rust-pkg-test.txt` in scratchpad.)

### Root `bun vitest run` — ⚠️ wrong invocation, do not trust its numbers
Ran exactly as instructed: 20 failed / 1 passed test files, 3/9 tests. **This number is an
artifact of an incomplete command, not a real signal.** The repo ships a hand-built
`🧪️vitest.config.ts` root aggregator (see its own header comment, written for ticket
`26/08/05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL`) specifically because Vitest's config
auto-discovery regex (`/^vite(?:st)?(?:\.[\w-]+)?\.config\./`) does **not** match an emoji-prefixed
filename — so a bare `bun vitest run` silently skips the real aggregator and falls back to
Vitest's own default `**/*.{test,spec}.*` glob across the *entire* repo, which is exactly what
that config's own header warns about: it picks up Playwright `.spec.ts` files (10 of the 20
"failures" are literally `Error: Playwright Test did not expect test() to be called here` —
wrong runner entirely), a missing generated-plugin-registry import, and stray ticket-folder
scratch tests never meant to run under vitest.

**Re-ran with the real config** (`bun vitest run --config "🧪️vitest.config.ts"`) for a trustworthy
signal: **16 failed / 10 passed test files, 14 failed / 542 passed tests** (`root-vitest-proper-
config.txt`). Triage:

**Confirmed PRE-EXISTING, unrelated (the two categories the task told me to expect):**
- `.storybook/os-plugins.spec.ts`: `Cannot find module '…generated/plugins.ts'` — a WASM
  plugin-registry codegen artifact that needs a build step, not source. Category confirmed.
- `…PLAYGROUND-WINDOW-MODE-COMPLETENESS-PASS/audit-playground-completeness.test.ts`
  (the named "July ticket's audit test"): `TypeError: CommandBus is not a constructor` —
  `CommandBus` no longer exists as an export of `@semio-tech/framework-core`.
  `git status --porcelain -- .…/PLAYGROUND-WINDOW-MODE-COMPLETENESS-PASS/` → empty;
  `git log --date=iso` on the test file → last real commit **2026-07-31**. Confirmed pre-existing,
  confirmed unrelated (framework-core's public API drifted independently of this ticket).
- 6 "Failed Suites" collection errors (`compose/index.ts` no-test-suite;
  `trinity-jack-lsp-worker` esbuild syntax error on a genuinely malformed TS optional-chain typo
  `graphDomain.current?: string`; 3× `cad-js-module-*` + `flow-js` index.ts all failing on
  `Cannot find module '…/🗿️artifacts/{cad,flow}/🧬️schema/🟦️component.ts'`) — none of these target
  paths/files have ever existed in the current tree and none are touched by this ticket's diff;
  all pre-existing dangling imports.
- `flow-js compute/component.ts`'s 3 `initFlowThreadPool` test failures
  (`storage.get is not a function`): the test itself passes a bare number (`4`) as the
  `storage: StoragePort` positional argument instead of `requested`. `git status --porcelain`
  on that file → empty; `git log --date=iso` → last real commit 2026-08-07. Pre-existing arg-order
  bug, unrelated.
- `renders selectable builder cards with selection ring`: renderer reads
  `node.presence.selected`, test passes a bare top-level `selected: true` (a field the type does
  independently declare, but the renderer has never wired it — `Interpreter/🟦️component.tsx` has
  zero uncommitted diff). Pre-existing dead-field mismatch, not new.
- `commandCategories orders and dedupes…` (label "Artifact"→"Document") and the two mit-bestand
  logo-path-regex tests: not in this ticket's diff hunks (see below), cosmetic/unrelated renames.

**Genuinely caused by this ticket — real, actionable bugs found by this audit:**
- **`uses the world surface selection mode instead of a stale shared invertive mode`**:
  `TypeError: resolveWorldSelectionMergeMode is not a function`. Root cause confirmed:
  `World3dHost/🟦️component.tsx` (uncommitted, `M` in this ticket) renamed the function
  `resolveWorldSelectionMergeMode` → `resolveWorldMergeMode` (present at line 2888,
  `export function resolveWorldMergeMode(configuredMode, event, persistentMode)`), but the
  giant barrel re-export list in `…/react/📦️index.tsx` (also `M`, uncommitted) still imports/
  re-exports the **old** name, and the pre-existing test (not touched by this ticket's own diff
  hunks — confirmed via `git diff` hunk positions, which only touch ~line 238 and ~803-849 of the
  1000s-line test file) still calls the old name too. Both cross the boundary as `undefined` at
  runtime. Fix: rename the re-export (and the test's import, if the test is meant to keep
  covering this) to `resolveWorldMergeMode`.
- **`interprets virtual file system component scenes`**: `Error: Element type is invalid …
  got: object`, a React "component is actually a plain object" crash. Not root-caused as deeply
  as the merge-mode one (out of time budget), but same file/same failure class (pre-existing test,
  broken by an uncommitted rename/shape change in this ticket's `World3dHost`/`index.tsx` diff) —
  flagged as **likely the same category**, needs the same kind of stale-re-export check, not
  independently confirmed line-for-line.
- **`framework-renderer-wgpu`: `builds plugin bridge handles`**: `TypeError: handle.manifest is
  not a function`, inside `🎠️kernel/🟦️component.ts:978` (`pluginHandleForBridge`). This crate/test
  pair is **fully unmodified** (`git status --porcelain` empty for both `🎠️kernel` and the wgpu
  test package) — the test mocks `manifest` as a plain **object** while the ABI now expects a
  **function** returning a JSON string (`JSON.parse(handle.manifest())`). Pre-existing API-shape
  mismatch, unrelated to this ticket (nothing in the call chain touches interaction/selection).
  Included here only because it's the wgpu "WASM plugin-bridge…built artifacts" category the task
  named — confirmed as that category, not a new one.

Net: **2 confirmed, 1 suspected real regressions from this ticket's TS work** (the stale
`resolveWorldSelectionMergeMode` re-export, and its likely sibling in the same file), everything
else in the 16/26 failing files is independently confirmed pre-existing via `git status`/
`git log --date=iso` evidence, matching (and going further than) the task's expectation that root
vitest was "already largely red before this effort."

## 4. `os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures`

Confirmed still failing, same message: `need at least 2 usable non-stdio grammar+fixture pairs for
cross-rejection, found 0`, with all 6 pilot artifacts (`fem::◻2d`, `lowpoly::💠️lowpoly`,
`cad::📐️cad`, `norm::en1992`, `dag::🕸️dag`, `note::🗒️note`) soft-skipped as "no .dsl.semio under
📚️examples (🖼️assets-first walk)".

**Root-caused, not just attributed by exclusion.** This is a real bug in
`pilot_resolve::find_example_semio` (`🧰️framework/…/🧪️fixture-sweep/🦀️component.rs:598-606`):
its per-standard branch builds the search path as
`repo_root/<artifact_rel>/🏅️standards/<standard>/📚️examples`
but every one of these 6 pilot artifacts' fixtures actually live one level deeper, under
`…/🏅️standards/<standard>/🪆️subsets/✳️any/📚️examples/…` — verified on disk for all 6:
```
✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/…/🗣️liquid-retaining-fem-anchor.dsl.semio
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
```
`find_example_semio` never constructs the `🪆️subsets/<subset>` segment at all, so
`find_example_semio_under` is handed a directory that doesn't exist, `.is_dir()` is false, and it
returns `None` — the fixture is never even opened, let alone content-inspected. This is a
**path-construction bug that fires before any parsing happens** — it cannot be a symptom of a
deleted config key breaking a fixture's *content*, because content is never reached.
Cross-confirmed: the sibling test `m5_handcrafted_grammar_conformance::…` (same `pilot_resolve`
call, same 6 pilots) doesn't hard-fail on the same missing-fixture condition, because it
soft-skips per-facet instead of asserting a minimum count — only `m5_cross_artifact_rejection`
has the `assert!(usable.len() >= 2, …)` that turns "0 found" into a hard failure. Same underlying
resolution bug, different test-level tolerance.

Independence from this ticket, confirmed:
- `git status --porcelain -- 🧰️framework/…/🧪️fixture-sweep` → empty.
- `git log --date=iso -- …/🧪️fixture-sweep/🦀️component.rs` → last real commit **2026-08-12
  10:05**, two days before this ticket opened.
- No `.dsl` grammar or `.dsl.semio` example asset under any of the 6 pilot roots was edited by
  this ticket (all show empty `git status --porcelain`).

`cargo test -p semio-framework-os-kernel`: **862 passed, 1 failed** (this one, only).

## Summary numbers

| Check | Result |
|---|---|
| Fixture DSL/JSON grep | 0 fixture *data* files carry deleted keys; 3 kebab-case hits all unrelated domain words; **9 plugins' artifact-standard schema `.rs`/`.json` pair still declares the deleted fields, untouched by W4** (real, separate finding) |
| `cargo check --workspace` | 100 crates; 95 clean; 5 fail — 1 (`semio-compose-rs`, 92 errors) pre-existing/unrelated (concurrent kernel-dissolution ticket); **4 (`architect`, `fem`, `mathematical`, `reasoning-mindmap`) caused by this ticket's SDK break, missed by the W4 crate list — mechanically fixable** |
| `cargo test -p semio-framework` | 105 passed, 0 failed |
| framework TS vitest (via `📜️script.ts test`) | 146 passed, 0 failed |
| Root vitest, naive `bun vitest run` | misleading (wrong config picked up — misses the repo's own aggregator); do not use this number |
| Root vitest, `--config 🧪️vitest.config.ts` (real signal) | 542 passed, 14 failed across 16/26 files; all but 2-3 confirmed pre-existing via git evidence; **`resolveWorldSelectionMergeMode` stale re-export is a real, ticket-caused regression** (1 confirmed + 1 same-class suspected) |
| `cargo test -p semio-framework-plugin` | 165 passed, 0 failed |
| `cargo test -p semio-framework-os-kernel` | 862 passed, 1 failed (the known one) |
| `m5_cross_artifact_rejection` | Confirmed pre-existing, root-caused to a `🪆️subsets/<subset>` path-segment bug in `pilot_resolve::find_example_semio` — fires before any fixture content is read, so it cannot be a symptom of this ticket's config-key deletions |
