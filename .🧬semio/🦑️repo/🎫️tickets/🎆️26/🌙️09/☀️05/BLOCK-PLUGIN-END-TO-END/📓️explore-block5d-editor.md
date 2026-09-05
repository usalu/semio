# block5d editor — exploration notes

(Findings produced by a read-only Sonnet explorer; persisted by the coordinator because the explorer would not create files.)
Paths are repo-relative under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/` unless stated.

## 1. Modes / windows
- Editor: one mode `edit` (`✏️editor/🎭️modes/✏️edit/🦀️.rs:9`), two windows:
  - `block5d-board` — `SurfaceKind::Board2d` (`✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs:16-20`), render reads `part_kind.label` / `grips.len()` (`:38-42`).
  - `block5d-world` — `SurfaceKind::World3d` (`…/🌐️world/🦀️.rs:16-20`), render reads `part_kind.label` + first representation's `mesh_url` (`:32-33`); shows "—" when empty.
- Viewer: one mode `view`, one window built on the framework `MeshWindowKit` (`👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs:16-38`), reads `representations[].mesh_url`.

## 2. Document state / default boot
- `Block5dSnapshot` / `Block5dArtifact` (`🧬️schema/🦀️.rs:13-42`).
- Default boot document is `Block5dSnapshot::default()` — EMPTY (`🧬️schema/🦀️.rs:267-269`, wired via `ArtifactEditor::initial_snapshot` at `✏️editor/🦀️.rs:439-441`). Examples `hexagonal-cut-concrete-forest-left` and `nakagin-capsule` load only on explicit `setActiveExample`.

## 3. Commands (`✏️editor/🦀️.rs:136-144`; all `Migrated` at `:597-603`; each has a real handler)
| id | handler |
|---|---|
| `patchPartKind` | `🎮️commands/🏷️patch-part-kind/🦀️.rs` |
| `addGripKind` | `🎮️commands/🔘️add-grip-kind/🦀️.rs` |
| `removeGripKind` | `🎮️commands/🗑️remove-grip-kind/🦀️.rs` |
| `addGrip` | `🎮️commands/🌱️add-grip/🦀️.rs` |
| `removeGrip` | `🎮️commands/➖️remove-grip/🦀️.rs` |
| `setActiveExample` | `🎮️commands/🎬️set-active-example/🦀️.rs` |
| `edit` | `🎮️commands/🎨️edit/🦀️.rs` |
All dispatch through `command.dispatch(doc, cfg)` (`✏️editor/🦀️.rs:480`). No `unimplemented!`/`todo!`.

## 4. Tool proofs — healthy (the precedent for block2d/block3d)
- `bounded_first_step_tool_proofs!` at `✏️editor/🦀️.rs:387-396` sets `factory_type: Block5dRetainedCommandJobFactory`.
- Factory `Block5dRetainedCommandJobFactory` `:184-223`; `register_tool_job_factories` `:397-401`; `build_tool_job` `:402-434`; `Block5dStorePreparationFactory` + `build_artifact_store_one_item_preparation_factory` `:227-357` / `:382-385`.
- `BLOCK5D_RETAINED_TOOL_IDS` (`:148`) = `patchPartKind, addGripKind, removeGripKind, addGrip, removeGrip, setActiveExample, edit`; matches `BLOCK5D_PUBLICATION_CONTRACTS` (`:153-159`) and the Migrated set exactly.
- DEFECT (fixed by W1): the TS oracle `📦️packages/🟦️typescript/📜️script.ts:19` resolves `plugin/🔣️publication-authority.json` + `.schema.json` at the plugin root, which do not exist; the files live at `🧪️publication-authority/🔣️.json` and `🧬️.schema.json`. Sibling puzzle resolves inside the directory (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:157-158`). The route set itself matches.

## 5. Examples
- `examples()` at `✳️any/🦀️.rs:20-23` registers `art_5d_hexagonal_cut_concrete_forest_left::source()` and `art_5d_nakagin_capsule::source()` (mounted at `📦️packages/🦀️rust/🦀️.rs:2697-2703`).
- `setActiveExample` `include_str!`s the DSL assets at compile time (`🧬️schema/📸️snapshot/📝️text/🦀️.rs:13-15`) — loaded at real runtime, not only tests.
- DEAD facet: `✏️editor/📚️examples/🎬️demo-session/🦀️.rs` mounted as `app_5d_demo_session` (`📦️packages/🦀️rust/🦀️.rs:2681-2682`) but never referenced (see doc comment `✏️editor/🦀️.rs:543-546`).

## 6. Other
- Subset-root doc (`✳️any/🦀️.rs:7-10`) self-flags: 12 typed zip/txt/png/json/stl/obj import/export impls under `🚪️io/📥️import` / `📤️export` are NOT registered on the `io_mechanism` channel.
- Every `🖐️5d` `#[path]` mount in the crate entry resolves on disk.
