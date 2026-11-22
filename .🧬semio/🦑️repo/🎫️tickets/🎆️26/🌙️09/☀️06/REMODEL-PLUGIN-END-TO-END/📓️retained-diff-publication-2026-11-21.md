# Remodel keyed diff publication

The artifact editor's `build_artifact_store_one_item_preparation_factory` uses the explicit cold mutation factory. The production editor and artifact builder apply streams, assets, durable chunks, cameras, rig extrinsics, and ground control points through `ColdMutationDiff`. The aggregated `RemodelingMutation` has no retained implementation. The typed retained diff API is independently exposed through `RetainedDiff<RemodelingSnapshot>`.

The retained plan prepares ten assigned slots. Its preflight now refuses any nonempty stream, asset, durable-content, camera, rig, or ground-control-point delta with `UnsupportedOwner` before staging an assignment. Empty row wrappers can still accompany supported assigned fields.

Cold apply stages the touched keyed collections and installs them after every fallible row operation succeeds. It no longer clones the entire unpublished snapshot before applying the diff. A portable nine-case fixture combines an ID assignment, a stream offset patch, and durable-content rows to check successful publication and rollback. The fixture passes strict Ajv schema validation and an independent `fast-json-patch` implementation.

The owned row buffer and edit helpers now record real original Work before local state changes. Their existing neutral fixtures and retained parameter tests use the original-effect test driver. Full retained keyed execution still needs a coordinator for each collection, typed stream and ground-control-point member patches, durable content append/truncate, and aggregate mutation preparation wired into the editor.

The existing independent fixture oracles also passed eight retained parameter cases, fifteen row edit cases, and six row buffer cases. The repo MCP test timed out and the repo CLI returned a read-only error in its host-capacity cache. The first native attempt compiled a consumer after the shared construction helper had changed and failed against the earlier compiled dependency.

The refreshed artifact build compiled the production diff library. The standalone Rust test build then reached the diff test target and reported 18 fixture compilation errors. The unit module now imports the cold diff trait for its 17 composition calls and uses the ordered map's cold insertion API. The queued retest was canceled before it acquired the shared target because available disk space dropped below 15 GiB and the WASM component build took priority. The Rust unit tests await a fresh run.

The artifact workspace lock includes the existing `semio-framework-value-clone-source-testing` crate as a diff dev dependency. Cargo also resolved eight missing path crates already declared by workspace manifests.

The later warmed run compiled all 14 tests. Twelve passed, including the cold transaction fixture and owned row cancellation laws. The retained parameter test exposed a shared sparse-row work underquote. Source binding used three Work units while the sparse prep quoted one. The prep now uses the binding's actual quote and checks the effect's remaining Work before entering a fresh operation. Recovery verifies the issued field route and both original source bindings before continuing the child action. The test also exercises an attempted recovery before any child was issued.

The keyed refusal test assumed an unstarted assigned cursor had no closing work. Its field cursors require paid closure even before source capture. The test now checks the refusal receipt and compares the funded close sequence to that of a fresh owner.

The final warmed native run of `semio-s-artifact-remodel-remodeling-model-diff` passed all 14 unit tests in 56.10 seconds. This included each keyed refusal and the authored, inverse, and cancellation frontiers for all eight parameter facets. The nine durable-content transaction cases and the eight parameter cases also passed their independent JSON Patch checks after the corrections.
