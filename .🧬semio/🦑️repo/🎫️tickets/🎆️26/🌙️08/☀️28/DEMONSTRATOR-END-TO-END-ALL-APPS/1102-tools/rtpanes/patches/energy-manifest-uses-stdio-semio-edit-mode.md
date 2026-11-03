# Patch: energy editor manifest builds its mode and layout from the stdio-semio editor (production fault)

Symptom (rtpanes-energie.txt:97338): `app-definition.invalid: app s.energy.model@1/*#editor layout references undeclared window kind framework.window.mesh`, panic at `build_definition` -> `create_energy_model_editor()` in both smoke tests (and any production boot of the energy editor).

Root cause: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples-and-manifest/🦀️.rs:4` is
`use semio_s_artifact_stdio_semio::editor::modes::edit;`. It shadows the glob import and makes `edit::definition()` (line 85) and `edit::layout()` (line 101) the STDIO `.semio` editor's mode and layout (`stdio/🗿️artifacts/🧿️semio/🦀️.rs:222`, `mut_family_2::editor::modes::edit`), whose layout holds a `framework.window.mesh` window the energy app never declares. The energy mode/layout live in crate `semio_s_artifact_energy_model_editor_edit` (`🎭️modes/✏️edit/🦀️.rs:54 definition`, `:74 layout`, already used on line 86 and as `crate::editor_edit` at line 65), whose layout references only `energy.model.3d`, structure, zones, simulation.

Fix (one line, same file): replace line 4
    use semio_s_artifact_stdio_semio::editor::modes::edit;
with
    use crate::editor_edit as edit;
(`editor_edit` is `extern crate semio_s_artifact_energy_model_editor_edit as editor_edit;` in `✏️editor/🦀️.rs:38`.) Then check the stdio_semio import is still needed elsewhere in the crate (Cargo dep stays: the test uses SemioMembers).
Verify: `repo cargo ... test -p semio-s-artifact-energy-model-editor-mounted-tests --lib`.
