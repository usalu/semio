# Document I/O Frontier Audit — 2026-10-03

Read-only source audit; no tests or runtime executed. Only this report was written. Line references describe the observed current checkout and can shift during concurrent work.

## Finding

The native controlled glTF writer can cancel inside geometry/metadata/physical serialization, but the reachable whole-document route does not pass that control. Both document works mark themselves consumed before performing their complete synchronous chain and return Complete. A bounded wire admission/one-item export extent does not bound the admitted document, prepared geometry, or downstream serialization work.

## Exact Owners and Remaining Phases

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1203–1280`: Generation3dDocumentIoWork owns the instance handle but no retained phase cursor. Export calls retained_meshes and emit inline; import calls emit then owe_attached_previews_carrying inline. All phases run in one step. Imported extent accounts existing widgets plus three, while rich glTF import can construct several polygon graph groups; this deserves an explicit extent law against actual resulting mutations.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs:1061–1110`: Generation3dViewDocumentIoWork resolves the viewed document, projects prepared meshes, serializes, and retires the viewed source in one step. A resumable continuation must retain that exact viewed source until final publication or retirement, including selected-example ownership.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3439–3468`: export_meshes_from_evaluation traverses source channels, synchronously parses the entire session.eval_json(), deduplicates geometry, decodes inline packs or retrieves session packs, and collects owned meshes. Settled inference prevents kernel re-evaluation but does not bound this parse/project/copy stage.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🦀️.rs:39–78`: emit chooses whole-document text or export_prepared_geometry; retained_meshes collects before emit. No caller cancellation/progress reaches either function.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:729–815`: text serialization; format conversion; final UTF-8 validation or base64 envelope; import data URL splitting/base64 decoding/raw copy; format decode all remain inline. Non-glTF export also merges/projects Semio meshes before serializer. Capacity limits cap totals, not individual scheduler turns.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs:8–18`: JSON grammar parse -> JsonSnapshot -> pack Value -> DslValue -> FromValue builds multiple full representations synchronously.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs:20–65,174–223`: physical JSON/GLB decode, declaration admission, resolve_ready Semio conversion, editable polygon conversion, surface enrichment (including another payload JSON parse/stringification/validation), and scene-node application. Metadata control is locally constructed with an always-admitting callback; it is not the operation's cancellation.
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-document/🦀️.rs:101–136`: whole-payload import, generation removals, host-snapshot diff, config projection, imported.retire_cold and final mutation Vec construction are synchronous. Texture-target import is also a synchronous alternate path. Transfer cancellation ends before these phases.

## Reuse, Ownership, and Control

Existing owners must remain the command work plus existing instance/session/inference owners; no evaluator, cache, alternate state store, or new module is required.

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs:44`: serialize_prepared_meshes_bytes_controlled already preserves one NativeEncodeControl through prepared geometry and physical bytes. Its uncontrolled wrapper installs an always-true callback, which is exactly the reachable bypass to remove.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs:461`: serialize_gltf_document_owned_controlled consumes the snapshot; use this ownership transfer rather than copying it at the physical boundary.
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1507–1556`: from_json_str_controlled, to_json_string_controlled, ControlledReader and ControlledWriter are reusable first-party parse/bind/print work. They remain synchronous callback-driven operations; a callback returning false cancels rather than yielding/resuming. Do not claim they alone create retained scheduler continuation.
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs` and `🛫️encode/🦀️.rs`: NativeDecodeControl/NativeEncodeControl report completed,total,owned_bytes; begin_stage/checkpoint/advance/charge/allocate_vec/copy_text support explicit budgets and cancellation. Checkpoints occur at 256-unit crossings and text copies in 65,536-byte pieces. Stage resets mean a UI total needs existing operation-phase accounting rather than treating each callback ratio as global completion.
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:207`: existing bounded retirement step is reusable. Controlled JSON reader uses DecodedValue guards/FromValue::retire_decoded for partially built values. glTF snapshot pack implementations provide explicit retirement of extras/extensions. Retain the command payload, viewed source, intermediate snapshot, projected meshes and pending output until the same work either publishes or drains retirement; plain scope drop must not become the terminal unbounded phase.

The inspected generic JSON pack module exposes no public retained parse cursor. Reuse the existing retained command-wire parser through its owning job infrastructure where applicable; its outer envelope decoding does not automatically cover nested payload/session JSON. Extend the existing JSON owner if inner parsing needs a retained cursor; do not substitute renderer-specific GLB parsers or a second grammar.

## Same-Owner Implementation Order

1. Add fixture-driven route laws first for editor import/export and viewer export that distinguish outer parsing/projection/envelope work from the already controlled native leaf.
2. Replace consumed-only DocumentIoWork with retained phases in those exact owner files; keep borrowed session access short and retain the exact source identity/generation for subsequent steps. Yield between bounded phases through the existing ArtifactCommandWork machinery.
3. Thread existing operation progress/cancellation into JSON bind, mesh conversion and the already controlled physical glTF writer. Add retained inner-parser/serialization cursors only inside their existing owners when callback cancellation is insufficient for scheduler yielding.
4. Bound import diff/config preparation and export envelope emission before Complete. Publish one final effect/mutation batch only after all phases succeed. Cancellation/failure drains owned intermediates through existing retirement, then releases sources once; never reevaluate or mutate the retained inference geometry to satisfy export.
5. Exercise the same continuation through existing WASM/native inference runtime and host dispatch; successful native helper output alone cannot prove browser responsiveness or late cancellation.

## Missing TDD Laws

- Large admitted nested JSON/session eval JSON: first step yields before full parse/project, bounded byte/item receipts on every hop, identical resumed output under several budgets; serde_json oracle for final JSON values.
- Settled rich prepared meshes: route cancellation during session projection, Semio conversion, physical output, and binary base64 envelope emits no download, releases every source once, leaves retained inference reusable. Use the committed rich fixture and NodeIO parity after uncanceled completion.
- Import JSON/glTF: cancellation during physical parse, binding, surface enrichment, scene expansion, diff construction and retirement emits no durable/config rows and no preview rearm; final uncanceled mutations match actual graph groups and inverse restores prior graph.
- Viewer selected example: source remains alive across yields; switching source or closing owner cannot publish stale output; terminal retirement is bounded and exact once.
- Final physical/envelope crossing: cancellation after the writer completes but before DownloadMediaExport publication still suppresses output. Transfer-complete cancellation must reach decode/application.
- Runtime law: dispatch real document command through existing WASM route, observe progress phase/console evidence and cancellation while scheduler handles another interaction; no new evaluator, session, cache or extra kernel work.

Current route unit laws inspected assert wire ceiling/BoundedFirstStep/PerOperation metadata; export command tests exercise helpers. The supplied writer cancellation and mesh conversion/parity results cover their native leaves, not these outer command phases. This audit did not rerun them and makes no passing/runtime claim.
