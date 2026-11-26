# Folder document identity and reopen

The native SDK opened a WFC bitmap at its real editor with the expected pack schema and returned `artifactKind: null`. The folder schema resource projected the canonical SPR schema, plugin, and app. It omitted the editor dialect that `artifact_open` reads from that resource. Folder content provenance also used the SPR schema for its artifact kind.

Installed owner discovery retains the editor's dialect kind through schema alias selection. Folder creation and reopen carry that exact kind into the binding. The bound schema resource and untrusted content provenance now publish it. The hub binding gets its kind from the authenticated execution-target lease's parent dialect. The existing hub schema and provenance derive the same field from authenticated catalog selections.

Folder reopen checks the pair initially returned to the caller against the durable snapshot while holding the same bindings lock local mutation uses to publish. It keeps the existing binding when the owner, pack, SPR, and optional recursive archive match. Changes that occur between the original read and binding return a precondition error. A fresh open accepts the externally changed snapshot and seeds a distinct binding. A recursive archive change alone also replaces the binding.

## Verification

- `repo test-file` ran the workspace's neutral conformance fixture through Bun, Ajv and SQLite. All four tests passed, with 51 assertions. The vectors cover distinct schema and kind, stable reopen, intervening mutation, fresh external content, and archive-only changes.
- The native `folder_reopen` regression uses the Note editor's distinct kind `s.note.note` and pack schema `note.document`. It checks the public `artifact_open` result, schema and body resources, exact relay and counter identity on unchanged reopen, and replacement after a durable change.
- The initial focused Cargo attempt stopped at a concurrent DSL dependency edit, an unresolved `retirement::retire_field` import. After the dependency published the missing definition, the pinned offline `semio-os-mcp --test folder_reopen` retry linked both the gateway and regression and passed its single native test in 0.67 seconds.

## Export before opening the folder

Export now resolves the owner from the stored Pack and SPR when the process has no existing binding. The regression seeds a Note journal and asks the native workspace to resolve its export owner before any open. The official SDK verifier also accepts `--export-before-open` and compares the resulting archive with the persisted bytes. The original component previously timed out during export even after opening. Successful archive output is still unverified.

The rebuilt native folder integration test passed, including the export-before-open owner assertion. An official SDK using the original WFC component also progressed from cold export to two genuine guest turns and returned the typed budget refusal.
