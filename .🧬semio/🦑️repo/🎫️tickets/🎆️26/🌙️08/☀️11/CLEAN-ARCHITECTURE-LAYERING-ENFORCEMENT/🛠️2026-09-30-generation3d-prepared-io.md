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

Actual47-law RED:19passed/2failed/26cancelled, Nextest `3c3c061b-ac55-4cd7-b16c-2eb3902cae8c`. The new registry law failed with `ordered-map root must be explicitly retired before drop`; the root AST guard failed `host-method`, actualtrue/expectedfalse. Both actual failures are preserved in `🗑️generated/generation3d-canonical-io-registry-red.log`. The composer now encodes while its rebuilt snapshot is live, explicitly retires it, then propagates the owned byte result or error. The root agent added method-call traversal/refusal after its actual RED. Final47-law locked run is active at `🗑️generated/generation3d-canonical-io-registry-green.log`.

Actual product runtime confirmation (app-law owner): `🗑️generated/app-harness-final/exact-cargo-laws-D5c5BK/00/law-120.stdout` passed the original real OBJ/GLB/STL bridge law1/1. Temporary `[DEBUG]` measurements reported `authority=true`,38vertices/20triangles. The app-law owner removed both DEBUG statements and dispatched its clean suite; that final suite is independent and not claimed green here. A later inline mesh fixture defect was owned and repaired by that agent. No local app harness, source SHA or preservation fixture was edited by this IO slice.

The first final47 post-fix invocation stopped before any native work: the root portable AST oracle used strict Ajv without registering the newly required `x-semio-formats` annotation. Receipt `🗑️generated/generation3d-canonical-io-registry-green.log` records the named unknown-keyword diagnostic. The root owner is correcting annotation registration while retaining strict schema checking. This result is not a native failure or a green receipt.

Final complete clean receipt: `🗑️generated/generation3d-canonical-io-registry-final.log` passed uncached Nx in3m0s. Both portable contributions passed (38 AST-authority and8 input/registry assertions). Nextest `69f6a204-ed8c-491b-bf2a-62a597300b0e` ran47/47 native laws,0skipped,0.564s execution. This supersedes the pending verification notes above and verifies both cold text-composer retirement and the hostile method-call guard after their actual RED. The47th law is additive artifact IO coverage, separate from the preserved355 original composition corpus.
