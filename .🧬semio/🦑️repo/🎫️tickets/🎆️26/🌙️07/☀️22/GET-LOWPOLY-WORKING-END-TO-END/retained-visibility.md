# Retained Command Visibility

The extracted retained command module returned a plain-private disposition enum through a parent-visible function. Production sibling `play_app` consumes that function. The enum now has `pub(super)` visibility; retained helpers, route identifiers and execution contract remain internal.

Three implementation tests (partition/publication, progress/replay/freshness/close, and turn ceiling) now mount under `retained_commands::tests` in the owning library. They can access private retained details without a public testing API. `lowpoly_editor_unit` remains an external integration target for command serialization, manifests, media import/export and registered app behavior. Mounted and command-specific integration targets are unchanged.

Both test locations use one command specimen file, `editor/🧪️tests/🧰️support/commands.rs`. The external target now imports edit mode and panel body identifiers explicitly from their owning packages. The moved retained operation fixture supplies the current mounted maintenance retained grant.

`LowpolyRetainedCommandWork` and `LowpolyCommandJobFactory` remain parent-visible. Neither escapes a public associated type: factory payload/job types are framework retained command wrappers; owner is `EditorApp<LowpolyPlayApp>`. No new behavior, schema, runtime dependency, public testing API or compatibility layer was introduced.

Source checks confirmed that all three retained test functions were preserved, their shared fixture resolves, external integration tests no longer reference private retained symbols, and no retained item gained unrestricted public visibility. The longest new source path is 140 characters. Native compilation and test execution have not been run because the shared dependency repair is still pending; source checks are not a compile-pass claim. The runtime target must execute editor library tests as well as its existing integration targets.

Changed source files:

- `editor/🧵️retained-commands/🦀️.rs`
- `editor/🧵️retained-commands/🧪️tests/🔬️unit/🦀️.rs`
- `editor/🧪️tests/🔬️unit/🦀️.rs`
- `editor/🧪️tests/🧰️support/commands.rs`

The retained-visibility input script remains in this ticket as required. No generated output files were created by this subtask.
