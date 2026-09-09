# Jack Query and Result Window Ownership

## Ownership decision

Jack's authored query belongs to the exact query-editor window config. It is persisted and restored with that window. Query execution, progress, cancellation state, output, and error belong to the exact results-window transient selected by the command. They reset when the window is recreated. The artifact remains the owner of graph content. The application has no Jack config or transient state.

A retained query operation captures two separate authorities from the trusted attached-window roster: the invoking editor window config and the explicitly supplied results window transient. Both windows must be attached to the current application/document and have the expected Jack window kinds. Payload data cannot grant a target. Completion writes `SetQuery` only to the captured editor config and `ReplaceQueryResult` only to the captured results transient. Closing either required window retires the operation.

## Implemented behavior

- Replaced the global Jack query config with `JackEditorWindowConfig` and an addressed `SetQuery` mutation.
- Replaced the global Jack query result transient with `JackResultsWindowTransient` and an addressed `ReplaceQueryResult` mutation.
- Made the Jack application use `NoConfig`, `NoConfigMutation`, `NoTransient`, and `NoTransientMutation`.
- Added an SDK hook through `ArtifactApp` and `ArtifactEditor` so an app can capture one explicit command-selected window-transient target while the normal invoking-window config capture remains intact.
- Updated `RunQuery` and `LoadExampleQuery` to require `resultsWindowId`; retained preparation validates both exact windows, snapshots the query source from the invoking editor config, and records both IDs.
- Kept graph camera, LOD, text edit, and format command config lanes at `WindowConfig` ownership.
- Rendered editor query text from the exact editor config and result state from the exact results transient.
- Added a native two-editor/two-results isolation and lifecycle test plus a language-neutral Ajv/JSON-patch oracle across all five schema formats.
- Corrected Jack artifact, snapshot, and diff non-Rust contracts to the native shared `ArtifactChild` identity. Removed stale embedded `nodes`/`edges` and replacement `artifact` diff shapes. Added a committed-fixture Ajv and production-parser contract gate.

## Validation evidence

Green:

- Direct invocation of the current `testJackGraphWindowConfigOracle` export after concurrent taxonomy changes.
  - `[DEBUG] Jack window ownership oracle: independent graph settings, editor query sources, results outputs, generations, persisted source reload, and transient reset`
- Direct invocation of the current `testJackDocumentContract` export after concurrent taxonomy changes.
  - `[DEBUG] Jack artifact, snapshot, and diff accept the committed shared-child fixture and refuse embedded graph and replacement-artifact shapes`
- `bun ./📜️script.ts jack-window-config oracle`
  - `[DEBUG] Jack window ownership oracle...`
- `bun ./📜️script.ts jack-query-ownership oracle`
  - SQLite query/mutation, selection, cancellation/checkpoint/grant, and oversize cases completed.
- `bun nx run workspace:test-jack-document-contract`
  - `[DEBUG] Jack artifact, snapshot, and diff accept the committed shared-child fixture and refuse embedded graph and replacement-artifact shapes`
- `bun ./📜️script.ts verify artifact-field-parity test`
  - schema-policy, field-discovery, AST, and Ajv checks completed for 192 owners.
- `git diff --check` over the implementation scopes produced no diagnostics.

Native status:

- The earlier taxonomy and DAG demo-asset blockers were corrected by their owners. Fresh ticket-script runs are green for `jack-window-config oracle` and `jack-query-ownership oracle`. A fresh direct run of the Jack document-contract export is also green after correcting concurrently rewritten schema paths.
- An earlier focused Cargo check was green before the final ownership-tree deletion and native-test additions. A later repository reset build reached the intermediate Jack source and reported three local errors; the invalid results-transient publication constant and both missing results-window-kind paths were corrected.
- The final requested `bun ./📜️script.ts jack-window-config native` attempt waited seven minutes for the shared ticket `cargo-trinity` build lock. It never entered compilation and emitted no compiler or test diagnostic, so only that waiting invocation was canceled with exit 130. A simultaneous Nx document-contract retry likewise waited on another project-graph construction; its current test export was run directly and passed.
- The post-correction native `jack_graph_window_config_` isolation gate and combined native `jack-query-ownership` gate therefore remain pending once the shared Cargo target is available.

## Pending generic transient retirement seam

`JackResultsWindowTransientOwner` currently uses `bounded_window_transient_store_disposer`. The generic `BoundedTransientStoreDisposer` measures the whole root with `print_dsl` and retains the replaced store until one close step provides `maximum_bytes >= retained_bytes`. A result payload larger than the fixed 4096-byte maintenance grant can therefore remain pending indefinitely. This is a shared lifecycle defect, not a reason to inflate Jack's close grants. No Jack-specific workaround or larger grant was added; the parent task is handing the generic bounded transient retirement seam to its lifecycle audit.

The required `repo://goals` read was attempted before implementation, but the repository MCP resource was unavailable to this task. The existing active ticket and its supplied goal context were used; the ticket remains open for the parent task.

## Exact file ledger

The shared framework hook was changed in:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

The following Jack runtime/editor files were changed for query ownership:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs` (removal of the global config module and addition of the `query_window_config` module mount; other diff hunks in this shared root are concurrent work)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `.../✏️editor/🫧️transient/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs`
- `.../✏️editor/🎮️commands/▶️run-query/🧵️job/🦀️.rs`
- `.../✏️editor/🎮️commands/✏️text-edit/🦀️.rs`
- `.../✏️editor/🎮️commands/✨️format-document/🦀️.rs`
- `.../✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs`
- `.../✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs`
- `.../✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs`
- `.../✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs`
- `.../✏️editor/🎮️commands/🧫️set-fixture-json/🦀️.rs`
- `.../✏️editor/🎮️commands/🧭️reorganize/🦀️.rs`
- `.../✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs`
- `.../✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🦀️.rs`

Here and below, `...` expands to `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any`.

The exact editor-window config owner files added are:

- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/{🦀️.rs,🔣️.json}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🧬️schema/🧬️mutations/🔎️set-query/🧬️schema/🔣️.json`

The exact results-window transient owner files added are:

- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/{🦀️.rs,🔣️.json}`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/📊️replace-query-result/🧬️schema/🔣️.json`

The old owner paths removed are:

- `.../✏️editor/🎚️config/` in full: owner, Rust/JSON/TypeScript/GraphQL/Proto state schemas, aggregate mutation schemas, `SetQuery`, `ReplaceConfig`, their payload schemas, fixture, and two Rust test files.
- `.../✏️editor/🫧️transient/🧬️schema/🧬️mutations/` in full: aggregate owner plus `ReplaceQueryResult` and its payload schema.
- The placeholder files at `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/📌️.empty.md` and `.../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/📌️.empty.md`.

The language-neutral query ownership oracle files changed are:

- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🔣️.json`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts`

The Jack document contracts changed are:

- `.../🧬️schema/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../🧬️schema/📸️snapshot/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../🧬️schema/🔺️diff/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
- `.../🧬️schema/🧪️tests/🪪️document-contract/{🔣️.json,🟦️.ts}`

Validation registration changed only the Jack-related entries in these shared files:

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`
- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

Concurrent fixture-taxonomy moves, example-asset moves, WASM removal, and other artifact/editor changes visible in the working tree are outside this ledger and were preserved.
