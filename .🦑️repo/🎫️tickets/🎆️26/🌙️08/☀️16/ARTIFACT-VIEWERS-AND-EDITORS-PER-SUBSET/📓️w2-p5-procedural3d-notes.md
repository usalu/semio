# W2 Packet P5 (procedural3d) — Migration Notes

Lane: W2 packet P5, plugin `🌀️procedural`, artifact kind `🧊️procedural3d` ONLY (the `🧊️3d` app). Sibling
sessions own `◻2d`/`🎛️apps/◻2d` and `🧩️assembly` in the same plugin tree, disjoint subtrees, not
touched. Recipe followed: `📓️w2-cad-report.md`'s 16-step "Migration recipe". Contract:
`📋️contract-freeze.md` §1, §2, §2.6. SDK-gap context read from `📓️w0-f-report.md` / `📓️w2-fix-report.md`
first, confirming `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/
`ViewEmit`/`Dialect`/`StandardId`/`SubsetId`/`MeshWindowKit`/`MeshView`/`WindowKit` are ALL curated
bare in `semio_framework_plugin`'s crate-root `pub use app::{ … };` as of this packet (verified fresh
at `🧰️framework/…/🔌️plugin/🦀️component.rs:18413-18549` before writing any import) — this packet's
files use them bare throughout, no `app::` qualification, unlike the cad/lowpoly precedent files
written before those gaps closed.

## Emoji-typo trap hit (and caught) mid-session

Per CLAUDE.md's explicit warning, I hand-typed `🏅️标准` (Chinese, visually near-identical to
`🏅️standards` at a glance) twice while composing tool-call paths from memory instead of pasting a
verified string: once as a stray `Read` (harmless, errored "file does not exist") and once as a real
`Write` that created a wrong sibling tree
(`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️标准/…`). Caught immediately by `ls`-ing the
artifact root right after, `rm -rf`'d the wrong tree, and switched to the disciplined method for
every subsequent path: `find`/`ls` once, save the exact byte string to a `.path` file in the ticket
scratch folder, `Read` that file to get it back into context, never retype an emoji segment from
memory again. No corruption reached any file I report as delivered below.

## What moved (`✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/` → `.../🧊️procedural3d/…/✳️any/✏️editor/`)

The scaffold's entire `✏️editor/` tree (19 files, all `SCAFFOLD: true`/`📌️empty.md` placeholders) was
`rm -rf`'d, then the whole real app tree `cp -r`'d in as one unit (preserves internal structure per
recipe step 3), landing 99 files. The app's leftover empty `⚙️engine/` directory (zero files — its
content was already rehomed into the app root by the prior ENGINELESS ticket, per that file's own doc
comments) was not part of the copy in any meaningful sense and its empty shell was removed. A
`🫧️transient/📌️empty.md` was added at the editor root (required facet per
`surfaceRequiredChildDirs`, absent from the source app since it has no dedicated transient-state
facet — `Transient = NoTransient`).

Every `crate::apps::procedural3d::` reference across the 45 files that had one became
`crate::editor::procedural3d::` (scoped `sed`, editor tree only; verified 0 remaining with a
whole-tree grep before and after). No `include_str!`/`include_bytes!` needed a depth fix — every one
found (`📚️examples/🎬️demo-session`'s asset, `🎚️config/🧬️schema`'s 5 self+cross-facet reads) stayed
inside the moved subtree at its original relative depth, confirmed by grep across the whole tree.

Both modes, all 5 windows, moved intact:
- `🎭️modes/✏️edit/{component.rs, 🪟️windows/{👁️preview, 🕸️flow}}` (2 windows)
- `🎭️modes/🧬️generate/{component.rs, 🪟️windows/{👁️preview, 📝️form, 🗂️generations}}` (3 windows,
  ADDED whole — the scaffold only had `✏️edit`, per the brief's explicit instruction)

Plus `🎚️config`(+schema), `👥️presence`(+schema), `📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue}`,
`📚️examples/🎬️demo-session`, `🗣️terminology`, `🌉️wasm`, all 31 `🎮️commands/*` groups.

### `component.rs` (editor root, `…/✏️editor/🦀️component.rs`)

- `impl ArtifactApp for Procedural3dPlayApp` → `impl ArtifactEditor for Procedural3dPlayApp` (:118).
- `const APP_ID: &'static str = PROCEDURAL_3D_PLAY_APP_ID;` removed; added
  `const DIALECT: Dialect = crate::artifacts::procedural3d::PROCEDURAL3D_DIALECT;` (:132).
  `PROCEDURAL_3D_PLAY_APP_ID` the plain `&str` constant (used by `ActionFactory`/world-scene
  controller ids throughout the windows) is UNCHANGED and still defined at :38 — it is a UI-wiring
  string, unrelated to the removed trait const of the same literal value.
- `create_procedural3d_app()`: `App::builder(PROCEDURAL_3D_PLAY_APP_ID, LocalizedLabel::native(…))`
  → `Editor::builder(crate::artifacts::procedural3d::PROCEDURAL3D_DIALECT)` (:412); return type
  `App` → `semio_framework_plugin::AppDefinition`; trailing `App::from_builder(…)` wrapper removed,
  chain ends in `.build_definition()` (:522). Every other builder call in the ~110-line chain
  (`.document`, `.command`, `.artifact_kind`, `.mode_def` ×2, `.default_mode_id`, `.mode_layout`,
  `.window_kind_def` ×5, `.default_layout`, `.named_layout`, `.panel_tab_def` ×3, `.mutation` ×3,
  `.action_with`/`.view_action`/`.action_args`, `.utility` ×3, `.window_kind_utilities`,
  `.interaction`, `.window_kind_interactions` ×3, `.keybinding` ×2, `.config`, `.io`) verified present
  on `EditorBuilder` (all forwarded from `AppBuilder` via the `surface_builder_forward!` macro at
  `🔌️plugin/🦀️component.rs:13603-13670`, or handled specially at :13573-13596) — no method rename
  needed.
- **Dropped, not ported**: 8 `.example(PROCEDURAL_EXAMPLE_*, …)` calls and the trailing
  `.workflow("procedural3d", "Procedural 3D", "brep")` call — `EditorBuilder` has no `.example(...)`/
  `.workflow(...)` (contract §2.4/§7.4 gap, same one the cad pilot found; confirmed absent by reading
  `EditorBuilder`'s own impl block, not assumed). The subset's own 8 real `📚️examples/🎬️<slug>`
  facets (pre-existing, untouched) are the modern replacement surface.
- Test-module fallout, all in the same file:
  - `pub type Procedural3dApp = VcsArtifactApp<Procedural3dPlayApp>` →
    `VcsArtifactApp<EditorApp<Procedural3dPlayApp>>` (:934).
  - `new_app::<Procedural3dPlayApp>()` → `new_app::<EditorApp<Procedural3dPlayApp>>()` (:945).
  - `new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)` →
    `new_app_with_registry::<EditorApp<Procedural3dPlayApp>>(procedural3d_app_manifest_for_testkit)`
    (:949) — added `procedural3d_app_manifest_for_testkit() -> semio_framework_plugin::App` (:938-940)
    wrapping `create_procedural3d_app()`'s new `AppDefinition` back into the `App { definition,
    examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
    still require (framework testkit gap, contract §2.4/§7.4-adjacent, not fixable in this lease).
  - `declared_actions_bridge_to_commands` test (:1087) now calls
    `assert_declared_actions_bridge_to_commands::<EditorApp<Procedural3dPlayApp>>
    (testkit::procedural3d_app_manifest_for_testkit)`.
  - Grepped the WHOLE moved tree for `ArtifactApp`, `VcsArtifactApp<`, `App::builder`,
    `App::from_builder` before declaring done: zero hits outside the two intentional
    `EditorApp`/`VcsArtifactApp<EditorApp<…>>` occurrences above.

### Artifact root (`…/🧊️procedural3d/🦀️component.rs`, in-scope — this IS my destination tree)

Added `pub const PROCEDURAL3D_DIALECT: Dialect = Dialect { artifact_kind:
"s.procedural.procedural3d", standard: StandardId("1"), subset: SubsetId::ANY };` at the ARTIFACT
level (not under `editor`/`viewer`), so the viewer file can read it without ever importing through
the sibling `editor` module. `artifact_kind` value verified against this same file's own
`definition()` — the `s.procedural3d.schema.artifact` capability's `.descriptor(b"s.procedural.
procedural3d")?` row (:56-58) — matching the task brief's stated value exactly, not assumed.
`standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Canonical
surface ids: **`s.procedural.procedural3d@1/*#editor`** / **`s.procedural.procedural3d@1/*#viewer`**.

Fixed the one real `crate::apps::procedural3d::` reference in this file:
`.document_codec::<crate::apps::procedural3d::Procedural3dPlayApp>()` →
`.document_codec::<EditorApp<crate::editor::procedural3d::Procedural3dPlayApp>>()` (runtime
`ArtifactApp` bound needs the SDK adapter, not the authoring trait implementor directly — same fix
the cad pilot made). Added two tests: `dialect_artifact_kind_matches_the_schema_capability_descriptor`
and kept `artifact_kind_schema_matches_the_document_schema` as-is.

Noted, not touched: `crate::artifacts::procedural3d::op::Procedural3dMutation` (used by several
command files, pre-existing, unmodified by this packet) resolves through a `pub mod op { … }` alias
declared in `📦️packages/🦀️rust/📦️glue.rs` (outside this packet's lease) — grepped to confirm it
exists somewhere in the plugin tree rather than assuming, found there, left alone. My own new files
(artifact root, viewer) instead use the artifact root's own directly-verified `pub use …
Procedural3dMutation;` re-export path, since that's the one I authored and can vouch for.

## Viewer (`…/✳️any/👁️viewer/`) — genuinely independent, real, minimal

Scaffold's placeholder window `🎭️modes/👁️view/🪟️windows/🪟️main/` renamed to `👁️preview/` (its 6
placeholder facet dirs — `🎚️config`, `🎚️options`, `🎬️actions`, `👥️presence`, `🪛️utilities`,
`🫧️transient` — kept as-is, they're genuinely empty for this window); its two scaffold leaves
(`🦀️component.rs`, `🟦️component.ts`) deleted and replaced with real content. Surface-root and
mode-root scaffold leaves replaced with real content too. 19 files total (identical shape to the cad
viewer precedent: 2 real `.rs`+`.ts` pairs — surface root, window — plus 1 real mode-root `.rs` with
no `.ts` twin, plus 14 untouched `📌️empty.md` placeholders for genuinely-empty required facets).

- `Snapshot = Procedural3dSnapshot`, `Mutation = Procedural3dMutation` — both the SAME artifact-level
  types the editor uses (decode-only per contract §2.2), imported from `crate::artifacts::procedural3d`
  directly, never through `crate::editor::procedural3d`.
- `Config`/`ConfigMutation` = `NoConfig`/`NoConfigMutation`; `Presence`/`Transient` likewise
  `NoPresence`/`NoTransient` — a viewer needs no persisted per-session state to render; camera uses
  hardcoded defaults (documented as an intentional simplification, matching the cad/lowpoly
  precedent).
- `Command` = one-variant `Procedural3dViewCommand::Noop`, `#[derive(…, Default)]` with
  `#[default]` on the variant (`V::Command: Default` bound the real
  `testkit::assert_viewer_never_mutates<V>` — landed by W0-F — requires; cad's own `CadViewCommand`
  predates that and does NOT derive `Default` yet, a pre-existing gap in that packet, not one I
  introduced here). `handle` always returns `Ok(ViewEmit::default())`.
- One real window, `👁️preview` (`🎭️modes/👁️view/🪟️windows/👁️preview`), using `MeshWindowKit`
  (contract §2.6) — `WINDOW_KIND_ID`/`BODY_KEY` both `MeshWindowKit::KIND_ID` (`"framework.window.
  mesh"`), `definition()` = `MeshWindowKit::window_kind()`, render via `MeshWindowKit::render(&view)`,
  exactly the lowpoly precedent's shape.
- **Real evaluated geometry, not a fallback-box placeholder.** Unlike the cad/lowpoly precedents
  (whose fallback-box is real parity with a pre-existing, unrelated editor-side gap in composed-child
  mesh resolution), procedural3d's whole purpose IS generated geometry, and the evaluate+tessellate
  path needed no editor-side session or config to run once: `flow::FlowHost::from_fixture(fixture.
  clone())` → `.set_neuron_kind_infos_json(…)` → `.evaluate()` → `flow::tessellate_geometry(handle,
  tolerance)` per preview-widget geometry handle, all called directly (no import through
  `crate::editor::procedural3d`). Four small pure helpers (`is_brep_geometry_handle`,
  `collect_geometry_handles_from_eval`, `geometry_handles_for_widget`, `mesh_has_preview_geometry`)
  are duplicated verbatim from the editor's own `component.rs` (not imported — `policyViewerPurityBreaches`
  forbids it), same pattern the lowpoly/cad precedents use for their own duplicated helpers.
  Documented in the file's own module doc comment as an intentional simplification: no
  `FlowEvalSession` cache, no incremental tick chain — the whole fixture re-evaluates fresh on every
  render call, which a read-only viewer can afford but the editor's live-editing session cannot.
  Two tests confirm this isn't a silent no-op: `render_produces_a_scene_node_for_the_default_document`
  and `render_emits_real_tessellated_geometry_for_the_default_fixture` (asserts non-`"[]"` mesh/instance
  JSON for the default fixture).
- `create_procedural3d_viewer() -> AppDefinition` via
  `Viewer::builder(PROCEDURAL3D_DIALECT)…build_definition()`.
- Grepped the whole `👁️viewer/` tree for the literal substring `::editor::` (policyViewerPurityBreaches
  is a literal substring match, including inside comments) and for `.mutation(`/`Emit::mutations`/
  `artifact_mutations`: **zero hits**, both before and after writing every file (phrased every warning
  comment as "the sibling `editor` module" / "the other surface", never with the leading/trailing `::`).

## TS twins (5 editor windows + editor surface root; 1 viewer window + viewer surface root)

Every twin is typed from the actual Rust `render()` signature it mirrors, not the full document
schema — same "essential inputs, not the whole snapshot" scope the cad precedent set.

| Rust window | TS twin path |
|---|---|
| `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview` | same dir `🟦️component.ts` |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow` | same dir `🟦️component.ts` |
| `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview` | same dir `🟦️component.ts` |
| `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form` | same dir `🟦️component.ts` |
| `✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations` | same dir `🟦️component.ts` |
| `✏️editor` surface root | `✏️editor/🟦️component.ts` — namespaced `export * as <name>Window from …` per
  window (never a blanket `export *`: every window twin declares its own `<Name>ViewModel`) |
| `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview` | same dir `🟦️component.ts` |
| `👁️viewer` surface root | `👁️viewer/🟦️component.ts` — namespaced re-export of the one window |

## What I did NOT do, and why

1. **`.example(...)`/`.workflow(...)` dropped from the editor manifest** — no such method on
   `EditorBuilder` (see above). Not silently lost: the subset's own 8 `📚️examples/*` facets are the
   pre-existing replacement, untouched by this packet.
2. **No `crate::editor::procedural3d::op` alias verified end-to-end** — `op` is a `pub mod` alias
   declared in `📦️glue.rs` (outside my lease); several pre-existing command files reference
   `crate::artifacts::procedural3d::op::Procedural3dMutation` through it. Confirmed the alias exists
   somewhere in the plugin tree (grep hit in `glue.rs`) rather than assumed; not modified, not my file.
3. **Did not delete `🎛️apps/🧊️3d/`** — per explicit instruction, left in place for the coordinator's
   later whole-tree deletion after all three sibling W2-P5 sessions finish.
4. **Did not touch** `📦️packages/🦀️rust/📦️glue.rs`, the plugin root `✏️s/🔌️plugins/🌀️procedural/
   🦀️component.rs`, `Cargo.toml`, any tsconfig/vitest config, `🎛️apps/◻2d/**`,
   `🗿️artifacts/🌀️procedural2d/**`, `🗿️artifacts/🧩️assembly/**`, `🧰️framework/**` — all out of scope
   per the task brief, confirmed by grep that no file I touched leaked outside
   `🎛️apps/🧊️3d/**`/`🗿️artifacts/🧊️procedural3d/**`.
5. **No `cargo check` run.** My new `✏️editor`/`👁️viewer` modules aren't mounted into any `mod` tree
   yet (that's `glue.rs`, the coordinator's job) — a crate-wide check right now would only exercise
   the still-in-place OLD `🎛️apps/🧊️3d` tree and say nothing about this packet's new files. Verified
   instead by: whole-tree grep for leftover `crate::apps::procedural3d::` (0), grep for `ArtifactApp`/
   `App::builder`/`App::from_builder`/`VcsArtifactApp<` fallout (0 unexpected), grep for
   `::editor::`/mutation-shaped calls in the viewer tree (0), a Python brace/paren/bracket balance
   check on every hand-edited/hand-written `.rs` file (all balanced), and `find`-verifying every new
   path landed where intended immediately after writing it.

## SDK gaps observed (already reported by W0-F/W2-FIX, not new — recorded for completeness)

- `EditorBuilder`/`ViewerBuilder` have no `.example(...)`/`.workflow(...)` (contract §7.4, item 4)  —
  hit again here exactly as the cad pilot predicted every W2 packet would.
- `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still expect
  `fn() -> App`, not the new `AppDefinition` (contract §7.4, item 3) — local
  `procedural3d_app_manifest_for_testkit` wrapper needed, same as every other migrated packet.
- `semio_framework_plugin::app::InteractionView` remains outside the curated crate-root re-export
  list (unrelated to this ticket's own SDK additions) — kept qualified as `app::InteractionView` in
  both the editor and viewer files, matching the cad precedent's still-open note on this exact name.

## Coordinator TODO (plugin root `#[cfg(test)] mod surface_tests`, NOT added by me — outside my lease)

When wiring `📦️glue.rs`/plugin root, add real testkit assertions on the two new types:
```rust
semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<
    editor::procedural3d::Procedural3dPlayApp, viewer::procedural3d::Procedural3dViewer>();
semio_framework_plugin::testkit::assert_viewer_never_mutates::<viewer::procedural3d::Procedural3dViewer>();
```
Both real framework functions (landed by W0-F, contract §2.5) — no local stand-ins needed, unlike the
cad pilot which predates that landing. `Procedural3dViewCommand` already derives `Default` so the
`V::Command: Default` bound `assert_viewer_never_mutates` needs is satisfied.

## Files touched

Created:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**`
  (106 files — moved app content + 6 new real `🟦️component.ts` twins: 5 windows + surface root)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**`
  (19 files — real `🦀️component.rs`/`🟦️component.ts` at surface root and window, real mode-root
  `🦀️component.rs`; the rest genuinely-empty `📌️empty.md` facets)

Edited:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs` (`PROCEDURAL3D_DIALECT`,
  `.document_codec::<EditorApp<…>>()`, two new tests)

Not deleted (per instruction): `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/**` (left in place).

Scratch (ticket folder, this session): none needed beyond this report — no `.txt` logs were produced
since no `cargo`/`bun` command was run (see "What I did NOT do", item 5).
