# Remodel keyed diff publication

The artifact editor's `build_artifact_store_one_item_preparation_factory` uses the explicit cold mutation factory. The production editor and artifact builder apply streams, assets, durable chunks, cameras, rig extrinsics, and ground control points through `ColdMutationDiff`. The aggregated `RemodelingMutation` has no retained implementation. The typed retained diff API is independently exposed through `RetainedDiff<RemodelingSnapshot>`.

The retained plan prepares ten assigned slots. Its preflight now refuses any nonempty stream, asset, durable-content, camera, rig, or ground-control-point delta with `UnsupportedOwner` before staging an assignment. Empty row wrappers can still accompany supported assigned fields.

Cold apply stages the touched keyed collections and installs them after every fallible row operation succeeds. It no longer clones the entire unpublished snapshot before applying the diff. A portable nine-case fixture combines an ID assignment, a stream offset patch, and durable-content rows to check successful publication and rollback. The fixture passes strict Ajv schema validation and an independent `fast-json-patch` implementation.

The owned row buffer and edit helpers now record real original Work before local state changes. Their existing neutral fixtures and retained parameter tests use the original-effect test driver. Full retained keyed execution still needs a coordinator for each collection, typed stream and ground-control-point member patches, durable content append/truncate, and aggregate mutation preparation wired into the editor.

The repo MCP test timed out and the repo CLI returned a read-only error in its host-capacity cache. The authorized native test is running through the shared ticket build directory.
