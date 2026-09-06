# Explore: fem2d artifact + editor (`✏️s/🔌️plugins/🏗️fem`, `🗿️artifacts/◻️2d`)

Generated: 2026-09-06 (Sonnet, read-only explorer). No cargo/bun/nx/git-mutating commands were run;
all findings are static (grep/find/read) evidence with file:line citations. Peer report in the same
ticket folder: `📓️explore-fem3d-editor.md` (fem3d side — not duplicated here except for cross-checks).

## 1. `Fem2dPlayApp` / `Fem2dViewer` mounts + window inventory

Crate entry: `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` (1611 lines).

- `pub mod editor::fem2d` mount at **line 1357-58**: `#[path = "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"] mod component; pub use component::*;` — this is where `Fem2dPlayApp` (struct at `…/✏️editor/🦀️.rs:423`) lives.
- `pub mod viewer::fem2d` mount at **line 1543-44**: `#[path = "…/👁️viewer/🦀️.rs"] mod component;` — `Fem2dViewer` (struct at `…/👁️viewer/🦀️.rs:36`).

Editor submodule mounts (all under `editor::fem2d`, lines 1364-1442):
| submodule | line | file | purpose |
|---|---|---|---|
| `config` (+`config::schema`) | 1368/1374 | `✏️editor/🎚️config/🦀️.rs` | `Fem2dConfig`/`Fem2dConfigMutation` (camera, result display, locale) |
| `presence` (+`presence::schema`) | 1378/1382 | `✏️editor/👥️presence/🦀️.rs` | `Fem2dPresence` — deliberately empty (doc comment: "selection is transient, camera/results live in config") |
| `wasm` | 1385 | `✏️editor/🌉️wasm/🦀️.rs` | wasm bridge |
| `session` | 1388 | `✏️editor/🧵️session/🦀️.rs` | maintenance/close/reconcile steps |
| `commands` (19 modules) | 1390-1426 | `✏️editor/🎮️commands/*/🦀️.rs` | one file per action payload (see §3) |
| `modes::edit` | 1440 | `✏️editor/🎭️modes/✏️edit/🦀️.rs` | edit mode component |
| `modes::edit::windows::model` | 1442 | `…/🪟️windows/🧱️model/🦀️.rs` (1358 lines) | live structure canvas, gumball, selection |
| `modes::edit::windows::results` | 1444 | `…/🪟️windows/📊️results/🦀️.rs` (375 lines) | static/modal/buckling result panels |

Viewer submodule mounts (lines 1552-1558):
| submodule | line | file |
|---|---|---|
| `modes::view` | 1552 | `👁️viewer/🎭️modes/👁️view/🦀️.rs` |
| `modes::view::windows::model` | 1558 | `…/🪟️windows/🧱️model/🦀️.rs` (135 lines) |

**No `results` window exists on the viewer side** — the viewer has exactly one window (`model`), a
read-only duplicate render (nodes/members/supports + mesh-edge preview) that explicitly does **not**
import from the editor (viewer-purity rule, doc comment lines 1-7 of that file) and has **no results
overlay** ("that lives in the editor's separate results window, which this viewer does not (yet)
mirror" — its own doc comment). This is a real content gap (not a placeholder bug): a fem2d viewer
session can see the structure but never the analysis results.

**Placeholder/empty-text check**: no `TODO`/`unimplemented!`/`todo!`/stub markers in
`✏️editor/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🦀️.rs`, `👁️viewer/🦀️.rs`, or
`👁️viewer/🎭️modes/👁️view/🦀️.rs`. The **results window** (`…/📊️results/🦀️.rs`) does render short
data-labels as legitimate empty-state fallbacks — `placeholder()` helper at line 124, used for "No
load case defined" (140/259), "Result not found: …" (143), and analysis-error strings (136/239/263).
These are intentional UX fallbacks, not dead/broken code.

## 2. Mutation inventory (all 6 subsets)

25 mutation kinds total, every one has `🔺️diff/`, `↩️inverse/`, exactly 1 `🧪️tests/*` fixture case,
and a `🧬️.schema.json` (verified per-directory, e.g.
`…/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/{🔺️diff,↩️inverse,🧪️tests/📍️appends-node-n3,🧬️.schema.json,🦀️.rs}`).
**No mutation dir has zero fixture cases.**

| Subset | Mutations (count) |
|---|---|
| 🌐️any | 0 (aggregator only, see below) |
| 🕸️mesh | 11: create-node, delete-node, create-element, delete-element, replace-element, create-section, delete-section, replace-section, create-region, delete-region, replace-region |
| 🏋️load | 7: create-load-case, delete-load-case, add-load, remove-load, change-load-case-self-weight, create-combination, delete-combination |
| 🧱️material | 3: create-material, delete-material, replace-material |
| 🛡️boundary | 3: create-support, delete-support, replace-support |
| 📈️analysis | 1: update-analysis-settings |
| **Total** | **25** |

Aggregate enum `Fem2dMutation` (`🌐️any/🧬️schema/🧬️mutations/🦀️.rs:30-55`, `#[derive(dsl::Mutations)]`)
lists exactly 25 variants (`CreateNode` … `UpdateAnalysisSettings`), one per fixture directory above
— **variant count matches disk 1:1 in both directions**, no missing/extra variant either way.

## 3. Editor action dispatch health — the central finding

`Fem2dCommand` (`✏️editor/🦀️.rs:43-68`, `app_commands!` macro) declares **19 actions**: `addNode,
addBar, addBeam, addMaterial, addSection, addSupport, addNodalLoad, addMemberUdl, addAreaLoad,
addRegion, addLoadCase, addCombination, setSelfWeight, setAnalysisSettings, removeSelection,
setActiveExample, setCamera, setResultDisplay, setLocale`.

**Classification split** (`.action_interactive_job(...)` calls, lines 732-750):
- `Migrated` (3): `setCamera`, `setResultDisplay`, `setLocale` — line 748-750.
- `BatchOnlyPendingRewrite` (**16**): every editing action, **including `setActiveExample`** — lines 732-747.

**Retained-command factory** (`Fem2dRetainedCommandJobFactory`, lines 104-152): `factory_type:
Fem2dRetainedCommandJobFactory` is wired via `bounded_first_step_tool_proofs!` (line 447-460), and
`FEM2D_RETAINED_TOOL_IDS` (line 75) = `["setCamera", "setResultDisplay", "setLocale"]` — **only the
3 `Migrated` actions are covered**. Its own `classification()` returns `Migrated` (line 126-128).

Per project convention ("a `BatchOnlyPendingRewrite` command is hard-dead in the app, not merely
unoptimized"), **all 16 editing/example actions are dead in interactive dispatch** — only camera pan,
result-display toggling, and locale switching actually work live; every structural edit (add
node/bar/beam/material/section/support/load/region/case/combination) and the example switch are
batch-only.

**Comparison with the fixed references** (as instructed):
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: **11/11 Migrated, 0 BatchOnlyPendingRewrite**; `factory_type: Block2dRetainedCommandJobFactory` (line 426) covers **9** tool ids including `setActiveExample` and `edit` (line 174). Fully migrated reference.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`: 6 Migrated / 34 BatchOnlyPendingRewrite (itself not fully migrated), but its `PUZZLE2D_RETAINED_TOOL_IDS` (line 1072) **does include `setActiveExample`** — unlike fem2d.

So fem2d is behind **both** cited references specifically on `setActiveExample`: block2d and puzzle2d
both migrated example-switching; fem2d did not. Combined with the boot-snapshot finding in §4, this
means fem2d's only path to load its bundled example is a dead action.

**Correction to the prior S-END-TO-END catalog note**: `.🧬semio/…/☀️05/S-END-TO-END/📓️explore-per-plugin-blockers.md:49-50` labels this "16 viewer dead" for both fem rows. That is a mislabel — verified by direct grep-count on both files:
```
editor/🦀️.rs  Migrated=4  BatchOnlyPendingRewrite=16  factory_type=1  bounded_first_step_tool_proofs=1
viewer/🦀️.rs  Migrated=0  BatchOnlyPendingRewrite=0   factory_type=0  bounded_first_step_tool_proofs=0
```
(Migrated=4 counts the 3 `.action_interactive_job` `Migrated` rows + the factory's own
`classification()` returning `Migrated`.) The viewer file has **zero** commands/classifications at
all — it is a pure `ArtifactViewer` with no dispatch surface (correct by design, per its own
viewer-purity doc comment). The 16 dead classifications are unambiguously in the **editor** file. The
"catalog audit: missing mutation module" half of that row cross-references
`.🧬semio/…/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-plugin-catalog-completion-audit.md:68` — a
cross-plugin `stdio`-dependent catalog-completion issue affecting 12 plugins' mutation-module
ownership/paths (`animate`, `block`, `cad`, `fem`, `remodel`, …), not something specific to fem2d's
own source; the audit explicitly groups fem2d and fem3d together ("same").

## 4. Boot snapshot / examples

`Fem2dPlayApp::initial_snapshot()` (`✏️editor/🦀️.rs:499-501`) returns
`crate::artifacts::fem2d::schema::empty_fem2d_snapshot()` — **the app boots completely empty**, not
the bundled demo.

One example only: `📚️examples/🎬️demo/` (`🖼️assets/🗣️.dsl.semio` + `🦀️.rs` + `🟦️.ts` + its own
`🧪️tests/`). `FEM2D_EXAMPLE_DSL` (`✏️editor/🦀️.rs:36`) is a `pub const` alias for
`crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT` (an `include_str!`-backed DSL constant), read
directly by the `setActiveExample` command handler and by every test fixture.

A `🚧️ SDK GAP` doc comment at `✏️editor/🦀️.rs:646-651` states plainly: `EditorBuilder` has no
`.example(...)`/`.workflow(...)` methods, so the former example/workflow manifest registration was
**dropped, not ported**, when this file was migrated off the pre-B1 SDK. The only surviving path to
load the demo is the `setActiveExample` command (`✏️editor/🎮️commands/📚️set-active-example/🦀️.rs`,
mounted line 1416), which builds a `reset_document_effect` (`Effect::LoadDocument`, outside undo
history — `✏️editor/🦀️.rs:611-622`).

**Net effect**: the app boots empty, and its one and only way back to the shipped example
(`setActiveExample`) is classified `BatchOnlyPendingRewrite` — dead per §3. In the live interactive
app there is currently no reachable path from "empty document" to "the demo example" at all; the
example is only exercised by direct unit-test dispatch (`dispatch(&mut app,
Fem2dCommand::SetActiveExample(...))`, bypassing the interactive-job gate entirely).

## 5. `#[path]`/`include_str!` resolution audit (crate entry)

Script: Python, walks every `#[path = "..."]` and `include_str!("...")` literal in
`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs`, resolves relative to the crate-entry directory,
checks `os.path.exists`.

```
TOTAL entries checked: 343
TOTAL resolved:        300
TOTAL unresolved:       43   (21 under ◻️2d, 22 under 🧊️3d, 0 elsewhere)
```

All 43 unresolved entries are **mutation-test `#[path]` mounts** of the shape
`…/🧬️mutations/<kind>/🧪️tests/<long-descriptive-slug>/🦀️.rs`. Root cause confirmed by directly
listing one such directory:

```
crate entry literal: …/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-cascading-to-its-support/🦀️.rs
actual on-disk dir:  …/🕳️delete-node/🧪️tests/🚫️removes-node-n3-without-6eab3f/🦀️.rs
```

The on-disk directory name has been **truncated and given a 6-hex-char hash suffix**, while the
crate entry's `#[path]` literal still names the full original descriptive slug. This is not
Unicode-normalization (checked NFC/NFD both fail) — it looks like a filesystem/tool-imposed
path-length limit truncated the directory name at some point after the `#[path]` literal was
written (or was generated for it), and every one of these 21 (fem2d) + 22 (fem3d) mount lines is
now a dangling `#[path]` — a hard `mod`-resolution compile error for each, when the crate finally
does compile. This is a real, previously-unflagged defect distinct from anything in the prior
S-END-TO-END/terra catalog audits.

## 6. Framework-API drift indicators

Checked against the classes documented in
`.🧬semio/…/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w6-de-async.md` and `📓️w7a-block2d-compile.md`.

| Class | fem2d result |
|---|---|
| `async fn diff` / `async fn inverse` / `async fn handle` / `async fn print_dsl` / `async fn parse_dsl` | **0 matches**, all clean (framework traits are sync per `w6-de-async.md`'s authoritative table) |
| Total `async fn` under `🏗️fem` | 160, of which 154 are `#[semio_framework_async_macros::async_test]` test fns; the rest sampled in `◻️2d` are all `#[test]`-style round-trip tests — **no non-test async fn found in `◻️2d`** |
| `UiNode::`/`ui_stack_vertical`/`ui_text(` (retired tree helpers) | **0 matches** under `◻️2d` — clean, never adopted the old API |
| `command_from_action` signature `Value` vs `dsl::DslValue` | **N/A** — fem2d has no `📌️panels/` directory at all (unlike block2d's `📌️panels/🔍️inspection`), so this ActionFactory/inspection-panel error class doesn't apply here |
| `render` returning bare `ComponentTree` vs `Result` | **Correct**: `ArtifactEditor::render` (`✏️editor/🦀️.rs:617`) returns `semio_framework_plugin::UiAssemblyResult<ComponentTree>`, built via `.map(built_to_component_tree)` (line 622); both window bodies return `UiAssemblyResult<BuiltNode>` |
| `with_ports` on an async `AppIo` builder | **N/A / clean** — `fem2d_io()` (`✏️editor/🦀️.rs:376-384`) constructs the `AppIo` struct literal directly (`ports: vec![...]`), never calling a `.with_ports(...)` builder method, so it never hits the async-builder mismatch block2d had |
| **Missing `DESCRIPTORS`/`descriptor` on Config/Presence `Mutation` impls (block2d's error class 3, E0046)** | **PRESENT — confirmed defect.** See below. |

### 6a. Confirmed defect: `Fem2dConfigMutation`/`Fem2dPresenceMutation` are missing `protocol::Mutation::DESCRIPTORS`/`descriptor`

`protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-152`) declares:
```rust
pub trait Mutation<P>: Clone + ToValue + FromValue {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];   // no default
    fn descriptor(&self) -> &'static MutationLeafDescriptor; // no default
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Vec<Self>;
    // (everything after this has a default impl)
}
```
Neither associated item has a default. `EditorApp`/`ArtifactEditor::ConfigMutation` requires
`::protocol::Mutation<Self::Config>` (trait bound at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26637` and `:27015`).

fem2d's two hand-written impls both omit `DESCRIPTORS`/`descriptor`:
- `✏️editor/🎚️config/🦀️.rs:179-198` — `impl Mutation<Fem2dConfig> for Fem2dConfigMutation` has only `type Diff`, `diff`, `inverse`.
- `✏️editor/👥️presence/🦀️.rs:51-61` — `impl Mutation<Fem2dPresence> for Fem2dPresenceMutation` has only `type Diff`, `diff`, `inverse`.

Ruled out `dsl::Mutations`/`store::impl_whole_record_config!` supplying these automatically:
`dsl::DslOps` (used by both enums, per its own doc comment at
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1663-1763`, "P6: DslOps emits
DslVariants only — OpText/OpBinary must be handcrafted per artifact") does not emit `Mutation`
either. `store::impl_whole_record_config!(Fem2dConfig)` (macro body at
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9986-9996`) implements `ConfigRecord` and
`MutationDiff<Fem2dConfig>` for the **config type**, not `Mutation` for the **mutation enum**. There
is no other candidate source for these two items anywhere in either file.

This is exactly block2d's fixed error class 3 (`w7a-block2d-compile.md` §3, "E0046
`DESCRIPTORS`/`descriptor` missing on Config/Presence" — block2d's fix added 2 descriptors
(`Snapshot`, `SetLocale`) to its config enum and 1 (`Snapshot`) to its presence enum). fem2d has the
identical shape of bug, unfixed: 4 variants missing descriptors in `Fem2dConfigMutation` (`Snapshot`,
`SetResultDisplay`, `SetCamera`, `SetLocale`) and 1 in `Fem2dPresenceMutation` (`Noop`).

## Gaps to fix (prioritized)

1. **P0 — `Fem2dConfigMutation`/`Fem2dPresenceMutation` missing `DESCRIPTORS`/`descriptor`** (§6a): compile-blocking (E0046) once the crate is built against the current framework trait; copy block2d's fix shape (`✏️editor/🎚️config/🦀️.rs` / `👥️presence/🦀️.rs` there) — one `MutationLeafDescriptor` per variant + a `descriptor()` match arm, for both files in fem2d.
2. **P0 — 43 dangling `#[path]` mounts** (§5): every fem2d/fem3d mutation-test fixture whose on-disk directory got truncated+hashed no longer matches its crate-entry `#[path]` literal. Either rename the 43 on-disk directories back to the full descriptive slug the crate entry expects, or shorten the crate-entry literals (and doc-facing test names) to match the truncated form — but the two must agree before this crate can build.
3. **P1 — 16 dead editor actions incl. `setActiveExample`** (§3): fem2d only migrated 3/19 actions to the retained-command factory. Every structural edit and the example switch are `BatchOnlyPendingRewrite` (dead in live dispatch). Follow block2d's/puzzle2d's `Fem2dRetainedCommandJobFactory`/`FEM2D_RETAINED_TOOL_IDS` pattern to add the remaining 16, prioritizing `setActiveExample` since it is also the only way out of the empty boot state (§4).
4. **P1 — empty boot + dead example switch = no reachable example in the live app** (§4): once `setActiveExample` is migrated (item 3), this resolves itself; if not, consider restoring an `EditorBuilder.example(...)` equivalent once the SDK gap noted at `✏️editor/🦀️.rs:646-651` is closed (shared with fem3d and other B1-migrated apps — check before duplicating effort).
5. **P2 — viewer has no results window** (§1): `viewer::fem2d::modes::view::windows` only mounts `model`; a fem2d viewer session can never see analysis results, only geometry. Low priority relative to 1-3 since it's a missing feature, not a broken one, and is explicitly flagged "not yet" in the source's own doc comment.
6. **P3 — mislabeled prior audit row**: flag to whoever maintains `S-END-TO-END/📓️explore-per-plugin-blockers.md` that the fem rows' "16 viewer dead" should read "16 editor actions dead (BatchOnlyPendingRewrite)" — the viewer file has zero classified actions for both fem2d and fem3d.
