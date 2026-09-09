# Rewriting Concrete Window Configuration

## Ownership And Behavior

Rewriting no longer declares an app configuration record or app configuration mutations. It uses the shared `NoConfig` contract. Each Before, After, LHS and RHS graph window registers a typed window-config owner. The four kinds share a small state/mutation implementation under the editor's common window configuration module; the SDK addresses each concrete window instance independently.

The window state contains an optional camera and one LOD value. Camera absence uses the current document fixture's framing; changing a viewport persists only that window's local override. Reorganizing a rule remains a document-layout mutation. Resetting a rule changes the document and does not overwrite other windows' local view choices. LOD commands resolve the exact instance/kind through trusted `ViewModel.window_instances`; payload window IDs are not plugin-owned selection fields.

The camera/LOD commands emit only `WindowConfig` publication and have a retained command factory with a 4,096-byte input bound, a single work item, and an 8,192-byte result envelope bound. The SDK owns event publication, inverse metadata, per-window generations, pack/reopen and close. Existing document commands retain their explicit batch classification; this change does not claim to make those expensive commands interactive.

## Validation Status

The language-neutral JSON Patch/Ajv oracle initially failed on the old `beforePaneCamera` and `lodModeByWindow` app fields. After their removal it passed in 2.1 seconds and emitted a runtime diagnostic confirming the two independent window projections and empty app config. The fixture now validates mutation payloads as well as resulting window states. An earlier test-path typo was corrected before the meaningful red result.

Native run 10 passed all four selected tests with zero failures through Nx in 3 minutes 17 seconds. The tests cover the camera/LOD trace, exact inverses, text/binary mutation round trips, concrete-window command addressing, and exact document/mutation retirement. The runtime regression dispatches both concrete windows concurrently through retained publication, checks independent generations and window measures, compares unchanged document/app envelopes, renders both windows, and loads their saved configuration into a fresh app. Its production diagnostics confirm those assertions.

The runtime regression exposed three shared SDK defects, now corrected: cached document/config roots prevented close from retiring the owned stores; unrelated operations replaced each other's cancellation leases; and the fixed window-config preparation advertised one work item although the forward and inverse candidates together require two. The projection assertion now decodes the actual packed NodeGraphScene through the shared testkit scene decoder. Both original and reopened app instances complete close within their bounds.

The SDK lane has implemented exact-instance enumeration for window measures/engagements and explicit window-config persistence messages in both Rust and TypeScript. The regression exercises the underlying production pack/load APIs.

## New Files

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🔍️set-lod-mode/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧵️job/🦀️.rs`

## Removed App Configuration Files

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-before-pane-camera/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-before-pane-camera/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️set-before-pane-camera/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔍️set-lod-mode/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔍️set-lod-mode/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔍️set-lod-mode/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/🔁️mutation-contracts.json`

The separately selected `reorganize_uses_document_mutations` native regression passed through Nx (one passed, zero failed; 12 minutes 45 seconds including the shared build lock). It confirms that rule reorganization remains a real document mutation.
