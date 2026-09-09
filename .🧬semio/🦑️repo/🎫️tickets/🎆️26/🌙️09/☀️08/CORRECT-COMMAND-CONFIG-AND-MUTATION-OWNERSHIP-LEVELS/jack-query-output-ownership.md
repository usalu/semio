# Jack Query Output Ownership

Query source is authored app configuration. Computed query output belongs to the live app transient store and must not be serialized into document/config snapshots or their diff projections. The runtime transition will publish one owned query completion with document mutations, query config, and a transient result together, without executing the query twice. The result is associated with the canonical operation id. Editor caret state is independent and keyed to the concrete editor window.

The result field and config mutation have been removed from every serialized projection below. Runtime retained-query and rendering integration is in progress. The native domain cursor test passed with four neutral cases; the assembled editor remains under validation.

## Changed Projection Paths

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/🧹️keeps-an-edge-c65f2a/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/🏷️keeps-a-node-ea762c/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/📍️keeps-a-node-at-the-815499/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/✏️keeps-the-name-a-node-d2f59b/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📊️set-result/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/🔁️mutation-contracts.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📊️set-result/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📊️set-result/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📊️set-result/🔣️.json`

## Query Work Cursor

The domain executor now has a retained cursor for one match candidate, filter row, or mutation item per step. The first four neutral query cases passed against the existing synchronous executor and serde JSON golden result. A fifth graph-result case now verifies that the typed result retains its local graph child owner. SQLite independently passed all five equivalent neutral queries, including mutation counts. The graph-result extension awaits its native rerun. Graph construction, mutation revalidation, final result assembly, and cleanup still use whole-graph helpers; those operations need an explicit cost/bounds review before claiming a fully bounded query job.

Added/updated: the executor `🦀️.rs`, `🪜️execution/🦀️.rs`, `🧪️tests/🪜️resumable-query/🔣️.json`, and `🧪️tests/🔬️unit/🦀️.rs`.

## Retained App Integration

Run Query and Load Example Query now share one retained query-work implementation rather than three copied synchronous helpers. Their completion publishes the typed domain `QueryResult` (or query error) to the app transient store, alongside authored query configuration and any document mutations. The output retains its graph child owner directly; serializing a graph result to a JSON string and parsing the opaque child handle back would lose that local owner. Results render from the transient request snapshot. Initial configuration no longer computes a result. The old example-loader helper is removed; the existing distinct menu actions remain declarations of the shared query operation.

The typed transient schema implementation is present. The initial native command used default crate features and therefore exercised the domain cursor only. The permanent verification command now enables `component-app-assembly` and runs the SQLite oracle first. Its first assembled-editor build is active; no assembled-editor runtime pass is claimed. The runtime test pumps both maintenance and publication, acknowledges result pages, and closes the registered app on success or failure.

## Remaining Ownership Checks

- The SDK captures one generation for the whole transient store. `TransientStore::begin_publish_one` rejects any later generation. A concurrent caret change may therefore invalidate an unrelated pending query result. A real interleaving regression is required before selecting the narrowest conflict policy.
- The current transient publication preflight serializes the result and retirement prints the whole transient DSL. These paths need the same explicit cost and local-child-owner audit as the query job.
- Initial configuration stores authored query source without evaluating it. Startup/result-empty behavior must be checked in the assembled editor.

## Additional Changed Paths

- Executor test `🧪️tests/🪜️resumable-query/🟦️.ts` contains the independent SQLite oracle; its paired SQL lives in the neutral JSON fixture.
- Root `📜️script.ts` runs the oracle and assembled-editor native query test.
- Ticket validation `📜️script.ts` enables assembled-editor features for the Rewriting command test too.

## Graph Manifest Ownership

`Graph::to_fixture` and `Graph::subgraph_fixture` previously replaced every source graph's manifest identity with the example catalog id `nakagin`. The in-memory graph now retains its own `manifest_id`, and both projections preserve that identity. `JackSnapshot::to_json`/`from_json` also carry the inline manifest so a graph without a catalog id keeps its actual schema across a JSON round trip. This is a direct edit to Jack's root `🦀️.rs`, without a migration or compatibility wrapper.

The sixth neutral query case uses an inline manifest and expects a graph result with a null catalog id. Native assertions also compare the serialized inline manifest through serde and decode it back. This extension is awaiting the active assembled-editor native build. The separately registered Bun/Nx oracle target passed all six SQLite cases and logged the exact case count.

The oracle target is registered in root `📋️project.json`, `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`, and the isolated ticket validation project. Both the oracle-only target and full native query target use the root `📜️script.ts` implementation.

## Component Validation Follow-up

The assembled Jack test build reached the editor and failed on 26 compile errors. The coordinator corrected public module imports for the retained query job, exposed the preset query helper from its actual command owner, converted bounded Results-label errors, and updated the existing context-menu/window-measure tests for the explicit ViewModel argument. The runtime test now formats typed Fault values with Debug instead of requiring Display. The transient owner is resolving the remaining QueryResult field codec and exact mutation component-owner errors; these failures are not a passing native runtime result.

The SDK inspection confirmed that simultaneous typed operations capture app-wide config and transient generations. A window-only update can therefore invalidate an unrelated app query publication. Ownership partitioning and its interleaving regression remain open; silently weakening generation validation is not acceptable.

## Independent Window Interleaving Regression

The language-neutral executor fixture now defines one query admitted alongside two concrete editor-window caret updates. The native regression admits all three before draining worker/publication/ACK/retirement; it requires three transient receipts, matching generation advancement, unchanged document content, the shared Results table, and each selection in its exact rendered editor window. SQLite independently checks the two window selection rows. Native execution is pending compilation fixes; this test was added before changing publication authority. The existing single-query test shares the bounded operation drain helper.

Edited sources: Jack editor aggregate (removed dangling retired completion command attribute), editor unit tests, executor resumable-query JSON/TypeScript fixtures. The codec test now includes LoadExampleQuery and FormatDocument.
