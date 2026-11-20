# Shooting Drawing and Camera Persistence

Each shot can store typed rendering controls. JSON Schema, GraphQL and protobuf describe the same settings, and the TypeScript artifact parser validates their ranges. The Rust DSL and packed snapshot derive the nested record from the document type. SQLite exposes the controls as queryable shot columns and uses binary64 companion columns to preserve numeric identities. The authored demo and forest DSL snapshots include the optional drawing column.

Drawing edits use the `SetShotDrawing` event with a sparse patch. An explicit nullable fill color restores material-derived color. Undo reverts affected controls and releases a previously absent settings record once it has returned to defaults. Independent edits retain their own values when another edit is applied or undone.

Camera edits use the `PatchShotCamera` event. The first edit atomically creates and links a deterministic per-shot saved camera, using the currently resolved camera as its seed. The saved catalogue camera remains intact when a linked shot is first edited. Later events take only their designated projection, zoom, field-of-view, eye, target or up control from the payload. Undo restores the individual control and releases an unchanged first capture, restoring the previous camera link. Existing snapshot codecs persist both the camera and link.

Independent Python checks passed the committed forward and inverse vectors for both events, including replay of a second independent field around the first event's undo. Ajv and the TypeScript drawing parser agreed on valid and rejected patch inputs. GraphQL agreed on snapshot drawing fields, and Ajv validated the camera event and atomic diff. The TypeScript SQLite codec round-tripped both present and absent drawing records, independently verified by Bun SQLite queries and a partial-state rejection probe. Native workspace verification is coordinated by the parent task.

## Per-shot auto-fit

A shot can inherit the scene's auto-fit or set a boolean override. The `SetShotAutoFit` event changes one shot and retains the previous optional value in its inverse. JSON and the diff distinguish an explicit false from clearing the override. The DSL adds an optional Boolean shot column. SQLite stores the same field as a nullable integer with a zero-or-one constraint.

A focused `repo tools exec bun --light -- -e …` probe exercised the actual TypeScript snapshot parser and SQLite codec through binary database serialization. An independent Bun SQLite query verified true and false with and without drawing settings, and editing the row to NULL verified inherited behavior on reload and resave. Ajv checked artifact, snapshot, create-shot, event, and diff representations. The Python oracle verified the neutral fixture, exact undo across all nine assignments between the three optional states, and retention of another shot's edit.

## Integration audit

A repository-wide Rust reference check found all shot and shot-patch literals in Shooting. The external host composition and bridge refer to the mutation type without matching individual variants. All 34 variant declarations, binary tags, and oracle kinds agree in order. The three added leaves are mounted in the owning package and re-exported through the aggregation and editor packages. The native compiler lane remains coordinated by the parent.

A replayed camera undo now leaves the current document intact if its original camera slot was deleted or relinked. The Python oracle passed this case alongside all 34 forward and inverse vectors. The Rust regression simulates deleting the captured slot before attempting the original edit's inverse.

## Native source admission

The mutation package originally had 37 source inputs for both library and test configurations. Its managed build hit the 32-file boundary before compilation. The camera-control triad now compiles in the existing camera mutation package, and the auto-fit triad compiles in the mutation family package. The Rust component paths, schema references, binary tags and editor import paths are preserved.

The ticket's `source-boundary-audit.mjs`, using the managed build's source-closure scanner, reported identical counts for the macOS host and `wasm32-unknown-unknown`. The original owner now uses 31 library and 31 test files, the camera owner uses 31 and 31, and the mutation family owner uses 28 and 30. The aggregate mutation and editor closures stayed at 10 and 29, and 8 and 27, respectively.
