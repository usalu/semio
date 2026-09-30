# Generation3d Prepared Geometry IO

The actual app law121 failed after120 laws passed: snapshot-only mesh export constructed a fresh FlowHost without the composition's retained geometry authority. Imported mesh graph documents therefore yielded empty geometry. The original codec and coordinate assertions are preserved by the app-law owner.

The artifact IO contract now separates reversible graph document text from caller-prepared neutral geometry. `document_io::export_document(snapshot)` and `export_document_bytes(snapshot)` encode text without evaluation; `export_geometry(&SemioMeshSnapshot,format)` and the six typed `serialize_mesh_bytes` leaves encode prepared geometry. All six user formats remain available. Six obsolete graph-only serializer wrappers, graph→geometry composer entries, and matching composer capability claims are removed. The canonical materialized mesh codecs remain their actual owner.

`mesh_bridge::merge_meshes` and both mesh/document bridge functions belong to artifact IO. `generation3d_mesh_from_document(doc,prepared)` validates the declared graph and prepared mesh and returns those exact supplied coordinates/indices; it does not prove graph-to-mesh correspondence independently. The caller owns evaluating that graph through its supplied authority and passing its materialized result. The app-law owner tests that complete path. Geometry evaluation, job retention, delivery and cancellation remain composition-owned. The editor's retained export preparation remains available to the instance media export context being corrected by the root agent.

The schema-owned `Generation3dExportInputs` declaration and neutral surface fixture state truthful graph/geometry formats. Ajv independently checks supported and hostile input classifications. Native laws verify the six real codec/envelope routes, graph-only registry claims, invalid explicit bridge inputs, and merged geometry offsets/bounds against Parry's independent volume. The root agent owns a Rust AST authority-boundary guard, its neutral fixtures and Syn test dependency; those changes are outside this file manifest.

Verification pending: uncached Nx `@semio-tech/procedural-generation3d-rs:canonical-architecture` invokes its own `📜️script.ts canonical-io`, validating Ajv and all native `io-round-trip` laws. First run passed five Ajv assertions and compiled the actual 45-law native IO binary. It ran26 laws successfully, then failed solely the old cause-text assertion `no preview geometry` against the truthful `prepared geometry has no positions`;18 subsequent laws were cancelled by fail-fast. The assertion was corrected to check the exact new cause without weakening the empty-geometry refusal. Final locked rerun pending root AST-guard mount. Output: `🗑️generated/generation3d-canonical-io.log`. App-law runtime DEBUG and final clean receipts are owned by the app-law agent. No full app275 or all-workspace green claim is made here.

Updated files (19; no created/deleted implementation files):

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️io/🗿️artifact-surface.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/📤️export-document/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🗿️artifact-surface/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🗿️artifact-surface/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔁️round-trip/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs`

Preserved complete verification receipt: uncached locked canonical IO passed all46 native laws,0skipped,0.878s execution after8m14s build; original41Ajv checks passed. `🗑️generated/generation3d-canonical-io-final.log` preserves this result.

A subsequent necessary regression exercises the remaining true text composer with an actual three-widget imported mesh graph, both native-text and JSON-bridge source dialects, complete graph equality after text import and a third-party SerdeJSON reader. The schema-owned `Generation3dRegistryTextRoundTrip` neutral contract specifies both source kinds and graph preservation. The production composer currently lacks explicit retirement of its rebuilt cold snapshot; the pre-fix47-law run is active at `🗑️generated/generation3d-canonical-io-registry-red.log`. Root separately added a17th hostile AST method-call fixture and will correct its visitor after RED. Final expected portable contribution is8own+38rootAjv assertions; final native inventory47. No additional paths or targets beyond the19-file own manifest.
