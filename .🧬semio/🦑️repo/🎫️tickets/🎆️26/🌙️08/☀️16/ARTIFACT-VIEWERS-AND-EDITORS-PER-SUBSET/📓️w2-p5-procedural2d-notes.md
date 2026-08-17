# W2 Packet P5 (procedural2d) — Notes

Lane: W2 packet P5, plugin `🌀️procedural`, artifact kind `🌀️procedural2d` ONLY (the `◻2d` app). Scope:
migrate the retired `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/` app into `✏️editor`, author a real
`👁️viewer`. Sibling sessions own `🧊️procedural3d`/`🧊️3d` and the brand-new `🧩️assembly` in the same
plugin tree — neither touched. `📦️glue.rs`/plugin root `🦀️component.rs`/Cargo.toml/tsconfig/vitest
config are NOT touched (coordinator's job after all packets land).

Recipe followed: `📓️w2-cad-report.md`'s 16-step migration recipe, with the SDK re-export gap already
closed by `📓️w0-f-report.md`/`📓️w2-fix-report.md` — every `ArtifactEditor`/`ArtifactViewer`/`Editor`/
`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit`/`Dialect`/`StandardId`/`SubsetId` import in this packet's
own new code is bare (`use semio_framework_plugin::{...}`), confirmed against the live curated
`pub use app::{ … };` block before writing a single import. `InteractionView` is **not** in that list
(confirmed by grep) — kept `use semio_framework_plugin::app::InteractionView;` qualified, matching the
cad pilot's still-current pattern. `TreeWindowKit` was not needed (procedural2d's windows are
NodeGraph/Canvas2d, not tree-shaped).

## What moved (editor: `…/✳️any/✏️editor/`, 96 files)

Whole `🎛️apps/◻2d/` tree moved intact, preserving internal structure: root `🦀️component.rs`,
`🎚️config` (+ 5-leaf schema), `👥️presence` (+ 5-leaf schema), `🎮️commands/*` (all 21 groups —
`canvas-pointer-{down,move,up}`, `canvas-wheel`, `set-show-mode`, `connect-media-ports`,
`move-media-node`, `node-graph-edit`, `node-graph-viewport`, `reorganize`, `set-locale`, `add-widget`,
`remove-widget`, `add-generation`, `enter-generate`, `remove-generation`, `rename-generation`,
`select-generation`, `update-generation-values`, `flow-eval-tick`, `set-eval-outputs`), `📌️panels/*`
(3: `📄️artifact`, `🔍️inspection`, `🛍️catalogue`), `🗣️terminology`, `🌉️wasm`, `📚️examples/🎬️demo-session`
(root + `🖼️assets/🎮️demo.cmd.semio` + `🧪️tests/{rs,ts}`), and both modes under `🎭️modes/`:

- `✏️edit/{component.rs, 🪟️windows/{🕸️flow,👁️preview}}` — the scaffold's single `🪟️main` placeholder
  deleted, replaced by the two real windows.
- `🧬️generate/{component.rs, 🪟️windows/{👁️preview,📝️form,🗂️generations}}` — a wholly NEW mode dir (the
  scaffolder only pre-built `✏️edit`); built the full required-facet shape (`🎚️config`, `🎮️commands`,
  `👥️presence`, `🫧️transient`, all `📌️empty.md`) plus the three real windows by hand.

Every internal `crate::apps::procedural2d::` reference across all 32 occurrences in the moved files
was mechanically rewritten to `crate::editor::procedural2d::` (`sed`, editor tree only, verified 0
remaining with a follow-up grep). No `include_str!`/`include_bytes!` needed a depth fix — every macro
(`🎚️config/🧬️schema` reaching into `👥️presence/🧬️schema` via `../../`, `📚️examples/🎬️demo-session`
reaching its own `🖼️assets/`) references a sibling that moved with it in the same relative shape; a
Python resolver script confirmed every `include_str!` path in the moved tree resolves on disk (0
missing).

## Editor root `component.rs` — trait/manifest edits

File: `…/✳️any/✏️editor/🦀️component.rs` (716 lines, freshly authored, not sed-transformed, since the
trait-impl rewrite touches nearly every line of that region).

- `impl ArtifactEditor for Procedural2dPlayApp` — line 118 (was `impl ArtifactApp for …`).
- `const DIALECT: Dialect = PROCEDURAL2D_DIALECT;` — line 132 (new; `const APP_ID` deleted entirely —
  `PROCEDURAL2D_PLAY_APP_ID` stays as a plain string tag, still used by `procedural2d_action`'s
  `ActionFactory` and every window's canvas-scene controller id, per recipe step 7's "fine to keep if
  it's a plain string tag, not a trait const" carve-out).
- `const DOCUMENT_SCHEMA: &'static str = PROCEDURAL_2D_SCHEMA;` — line 133, unchanged value.
- `pub fn create_procedural2d_app() -> semio_framework_plugin::AppDefinition` — line 351,
  `Editor::builder(PROCEDURAL2D_DIALECT).document([...])....build_definition()` — line 441 is the
  `.build_definition()` call closing the chain. The label argument is gone (auto-set to "Editor"/
  "Editor" by `Editor::builder`); every other builder call from the pre-migration `App::builder(...)`
  chain ported verbatim (`.command`, `.artifact_kind`, `.icon_id`, `.mode_def` ×2, `.mode_layout`,
  `.default_mode_id`, `.window_kind_def` ×5, `.default_layout`, `.named_layout`, `.panel_tab_def` ×3,
  `.action_with`/`.mutation`/`.view_action`/`.action_args`, `.interaction`, `.window_kind_interactions`
  ×3, `.keybinding` ×2, `.config`, `.io`).
- **Dropped, not ported**: the trailing `.example("default", …).workflow("procedural2d", …)` calls —
  `EditorBuilder` has no `.example(...)`/`.workflow(...)` method (contract §2.4's `App { definition,
  examples }` split; `.editor::<E>(def)` only ever takes the bare `AppDefinition`). Not silently lost:
  flagged here and in the manifest's own doc comment (line ~437). The subset's own `📚️examples`
  facet is the modern, role-agnostic replacement surface, per the cad pilot's identical finding.

## Test-module fallout fixed (same root file)

- `pub(crate) mod testkit` (line 448): `Procedural2dApp = VcsArtifactApp<EditorApp<Procedural2dPlayApp>>`;
  `app()`/`app_with_registry()` call `new_app::<EditorApp<Procedural2dPlayApp>>()` /
  `new_app_with_registry::<EditorApp<Procedural2dPlayApp>>(procedural2d_manifest_for_testkit)`.
- New local wrapper `procedural2d_manifest_for_testkit() -> App { App { definition:
  create_procedural2d_app(), examples: Vec::new() } }` (framework testkit gap — `new_app_with_registry`/
  `assert_declared_actions_bridge_to_commands` still take `fn() -> App`, unchanged for this ticket).
- `declared_actions_bridge_to_commands` test (line 594):
  `assert_declared_actions_bridge_to_commands::<EditorApp<Procedural2dPlayApp>>(…::testkit::procedural2d_manifest_for_testkit)`.
- `<Procedural2dPlayApp as ArtifactEditor>::media_ports()` (was `as ArtifactApp`) in
  `media_ports_declare_params_in_and_drawing_out`.
- `the_manifest_stitches_every_taxonomy_node` test: `create_procedural2d_app()` now returns
  `AppDefinition` directly (was `App`), so `serde_json::to_string(&create_procedural2d_app())` dropped
  the old `.definition` field access.
- Whole moved tree grepped for `ArtifactApp`, `VcsArtifactApp<`, `App::builder`, `App::from_builder`,
  `PROCEDURAL2D_PLAY_APP_ID` used as a trait const — all clean (0 hits of the bad patterns).

## Artifact-level root `component.rs` (`…/🌀️procedural2d/🦀️component.rs`, in-scope, edited)

- `pub const PROCEDURAL2D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.procedural2d",
  standard: StandardId("1"), subset: SubsetId::ANY };` — line 18, ARTIFACT level (not under
  `editor`/`viewer`), so the viewer file reads it without ever importing through the sibling editor
  module. `artifact_kind` matches this same file's `definition()`'s `s.procedural2d.schema.artifact`
  capability descriptor bytes (`s.procedural.procedural2d`); `standard`/`subset` match the
  `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk — canonical surface ids are
  `s.procedural.procedural2d@1/*#editor` / `s.procedural.procedural2d@1/*#viewer`.
- `.document_codec::<crate::apps::procedural2d::Procedural2dPlayApp>()` → `.document_codec::<
  semio_framework_plugin::EditorApp<crate::editor::procedural2d::Procedural2dPlayApp>>()` — line 132
  (the runtime `ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor).
- One stray doc-comment fixed (line 39, `crate::apps::procedural2d::create_procedural2d_app` →
  `crate::editor::procedural2d::create_procedural2d_app`) — cosmetic, not a compile dependency.

## `component.ts` twins (6 real files, no blanket `export *`)

| window | path |
|---|---|
| Flow (edit mode) | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🟦️component.ts` |
| Preview (edit mode) | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🟦️component.ts` |
| Preview (generate mode) | `…/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️component.ts` |
| Form (generate mode) | `…/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️component.ts` |
| Generations (generate mode) | `…/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️component.ts` |
| Editor surface root | `…/✏️editor/🟦️component.ts` |

(`…` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any`.)

Each window twin declares a typed `ViewModel` interface + `windowKindId`/`bodyKey` (`surfaceId` where
the Rust side has one) string-literal constants, mirroring that window's real `render()` boundary —
none are empty placeholders. The surface root uses **namespaced** re-exports (`export * as
flowWindow from "./…"`, etc.) even though no two windows currently share an export name — kept
namespaced for the same reason cad did (a future window reusing a common name like `ViewModel` stays
safe instead of becoming a silent ambiguous re-export).

## Viewer (`…/✳️any/👁️viewer/`, 19 files)

- `Procedural2dViewer: ArtifactViewer` (`…/👁️viewer/🦀️component.rs:37`), never `ArtifactEditor`/
  `ArtifactApp`. `Snapshot = Procedural2dSnapshot`, `Mutation =
  crate::artifacts::procedural2d::op::Procedural2dMutation` — both artifact-level, shared with the
  editor (decode-only per contract §2.2). `Config`/`ConfigMutation` = `NoConfig`/`NoConfigMutation`;
  `Presence`/`PresenceMutation` = `NoPresence`/`NoPresenceMutation`; `Transient`/`TransientMutation` =
  `NoTransient`/`NoTransientMutation` — a viewer needs no persisted per-session state to render a
  fixed-camera schematic. `const DIALECT: Dialect = PROCEDURAL2D_DIALECT;` (line 48), read straight
  off the artifact-level const, no editor import.
- `Command = Procedural2dViewCommand` — one variant, `Noop`, deriving `Default` (so the framework's
  real `testkit::assert_viewer_never_mutates::<V>() where V::Command: Default` can synthesize one once
  the coordinator wires it — see "For the coordinator" below). `handle` always returns
  `Ok(ViewEmit::default())`.
- One real window, `👁️preview` (`🎭️modes/👁️view/🪟️windows/👁️preview`), rendering a schematic node-box
  per widget straight off `Procedural2dSnapshot.fixture` (position from `fixture.layout`, id from the
  artifact-level `widget_id` helper) through `build_canvas_2d_scene`/`Canvas2dScene` — **not** by
  calling into the editor. This duplicates the small, pure "wire" schematic-overlay logic the editor's
  own preview window also has (contract §2.2's "duplication is the deliberate cost of independence").
  Camera is a hardcoded `(0, 0, 1)` default (no persisted viewer camera — intentional simplification,
  documented in the window's own doc comment, not a bug) — same pattern the cad pilot's viewer used.
  The evaluated drawing-handle overlay (needs a live `flow::FlowEvalSession`, an editor-dispatch-time
  concept) is deliberately NOT reproduced — a stateless viewer `render` has no session to read.
- `create_procedural2d_viewer() -> AppDefinition` via `Viewer::builder(PROCEDURAL2D_DIALECT)…
  build_definition()` (`…/👁️viewer/🦀️component.rs:73`).
- Grepped the whole viewer tree for `::editor::`, `.mutation(`, `Emit::mutations`,
  `artifact_mutations` (the exact `policyViewerPurityBreaches` pattern set) — **0 hits**, including
  inside every doc comment (phrased "the sibling editor module" everywhere, never the literal
  `::editor::` substring).

## What I did NOT do, and why

- Did not touch `📦️glue.rs`, the plugin root `🦀️component.rs`, `Cargo.toml`, or any
  tsconfig/vitest config — out of lease per the brief; the coordinator wires these after all three
  sibling W2-P5 sessions (`procedural2d`, `procedural3d`, `assembly`) land. Confirmed by grep that the
  plugin root still has exactly one real reference,
  `.document_app::<crate::apps::procedural2d::Procedural2dPlayApp>(crate::apps::procedural2d::create_procedural2d_app())`
  (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs:53`), and `glue.rs` still mounts all 39 old
  `../../🎛️apps/◻2d/…` `#[path]` entries — an exact 1:1 match with everything this packet moved,
  confirming nothing was missed and nothing over-moved.
- Did not delete `🎛️apps/◻2d/` — left in place per instructions, coordinator deletes the whole
  `🎛️apps/` tree later.
- Did not touch the subset-level `📚️examples/🎬️demo` facet (`…/✳️any/📚️examples/🎬️demo/`) — that is a
  W1-E scaffold placeholder at the SUBSET level, a different facet than the editor's own
  `📚️examples/🎬️demo-session` (moved, real). Out of this packet's named deliverables (editor/viewer
  surfaces only); still scaffold, flagged for whoever owns subset-level example authoring.
- Did not add a read-only `🧬️generate`-mode twin to the viewer — contract §1 only requires ≥1 mode
  with ≥1 real window for viewer completeness; the single `👁️preview` window meets that, kept minimal
  per the brief's explicit guidance.
- Did not refactor the five generation-command files' identical duplicated `refresh_generation_preview`/
  `handle_generation` helper block (pre-existing in the source app, copied verbatim into
  `add-generation`/`remove-generation`/`rename-generation`/`select-generation`/
  `update-generation-values`) — pre-existing duplication in the app being migrated, not introduced by
  this migration, and refactoring it is out of this packet's narrow scope.

## SDK gaps hit (already known, reconfirmed against live source)

1. `EditorBuilder`/`ViewerBuilder` (`Viewer::builder`/`Editor::builder`) take a bare `AppDefinition`,
   no `.example(...)`/`.workflow(...)` — the pre-migration app-level example registration
   (`"default"`) is dropped, not silently: see manifest doc comment. Same gap #4 the cad pilot
   reported; still open as of this packet.
2. `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take
   `fn() -> App`, not `fn() -> AppDefinition` — needed the same local `App { definition, examples }`
   wrapper every W2 packet needs (gap #3 from `📓️w0-f-report.md`, still open).
3. `TreeWindowKit`/other five window kits genuinely not needed here (NodeGraph/Canvas2d only) — no new
   finding to add.

## For the coordinator

When wiring `📦️glue.rs`/plugin root, the `#[cfg(test)] mod surface_tests` calls to add:

```rust
semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<
    crate::editor::procedural2d::Procedural2dPlayApp, crate::viewer::procedural2d::Procedural2dViewer,
>();
semio_framework_plugin::testkit::assert_viewer_never_mutates::<
    crate::viewer::procedural2d::Procedural2dViewer,
>();
```

Both are the REAL framework functions (landed by `📓️w0-f-report.md`/`📓️w2-fix-report.md`, no local
stand-ins needed) — `Procedural2dViewCommand` already derives `Default` for
`assert_viewer_never_mutates`'s bound. Mount paths to derive when rewriting `glue.rs`'s two new
regions: every leaf under `✏️editor` in this notes file's file list, and every leaf under `👁️viewer`
(surface root, `🎭️modes/👁️view/{component.rs, 🪟️windows/👁️preview/component.rs}`, the 12
`📌️empty.md` required-facet placeholders unchanged).

## Verification

- `grep -rn "crate::apps::procedural2d" …/✏️editor …/👁️viewer` → 0 hits (post-sed, post-authoring).
- `include_str!`/`include_bytes!` disk-resolution script → 0 missing across the whole moved editor
  tree.
- `policyViewerPurityBreaches` pattern grep (`::editor::`, `.mutation(`, `Emit::mutations`,
  `artifact_mutations`) across the whole viewer tree → 0 hits.
- File counts: editor 96 files, viewer 19 files (surface root ×2, mode root ×1, one real window ×2,
  12 required-facet `📌️empty.md`) — matches the cad pilot's viewer shape (19 files) exactly.
- Could not run `cargo check` — glue.rs isn't rewired to mount `crate::editor::procedural2d`/
  `crate::viewer::procedural2d` yet (expected, per the brief; coordinator's job).

No scratch/log files were needed beyond this notes file (no cargo runs, no policy script runs — those
require the coordinator's glue.rs rewiring first).
