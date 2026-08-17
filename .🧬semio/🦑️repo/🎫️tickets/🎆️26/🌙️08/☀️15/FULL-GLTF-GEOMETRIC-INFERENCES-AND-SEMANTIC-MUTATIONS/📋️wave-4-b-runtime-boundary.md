# Wave 4-B Runtime Boundary and Invalidation

## Outcome

The GLTF inference now crosses the native/component boundary through a versioned owner/schema/revision/generation request and result contract. The semantic payload is always the frozen GLTF inference binary envelope; the WIT layer never projects GLTF fields into ABI records.

The guest exports a deterministic executable-inference roster and a single inference function. The host merges descriptors by `(artifact kind, inference schema)`, rejects ownership/version conflicts, serializes calls through the existing runtime store guard, validates echoed revision/generation/schema, and refuses stale, incomplete, or empty results.

## Wire contract

- Wire version: `1`.
- Request: owner, artifact/document/inference schema identities and versions, algorithm/policy versions, revision, generation, canonical snapshot pack, policy bytes, optional sorted unique touched paths, and optional session identity.
- Result: the same identity/revision/generation coordinates, canonical inference binary, structured diagnostics, completion state, cache mode, normalized touched paths, and session identity.
- Non-empty policy bytes are rejected until the executable service accepts a typed policy rather than silently ignoring it.
- Missing touched paths select an honest cold computation. Supplied paths are metadata for invalidation evidence; the current registry service is deliberately cold and labels the result `cacheMode = cold`.

## Dependency and touched-path contract

`GltfInference` declares stable stages for resources, accessors, primitives, instances, materials, relations, and final aggregation. The combined read sets cover both authored `document/*` paths and resolved `buffers/*` bytes.

Node modifications now report semantic paths such as `document/nodes/{i}/transform`, `/hierarchy`, `/mesh`, `/skin`, and `/weights`. Mesh modifications report `/primitives` and other exact subregions. Collection insertion/removal remains conservatively rooted at the collection because GLTF indices are transported across reference families.

The invalidation selector has these verified properties:

- absent/unknown touched paths invalidate every stage;
- an empty touched set invalidates none;
- node transform changes reuse resource, accessor, and primitive stages;
- node transform changes invalidate instances, relations, and aggregate stages;
- buffer/accessor paths invalidate every dependent decoding/geometry/aggregate stage.

## Changed existing production files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`

The GLTF text and binary inference codec files were completed in the preceding Wave 3-B and are the transport implementation consumed here.

## Verification evidence

- `bun nx run @semio-tech/stdio-plugin:test-quick -- 'gltf::standards::v2_0::subsets::any::schema::inferences::'`: **15/15 passed**. This includes transport determinism/corruption laws and the document/buffer dependency plus selective node-transform invalidation laws.
- `bun nx run @semio-tech/stdio-plugin:test-quick -- inference_wire_echoes_revision`: **1/1 passed**. The result echoed revision `7`, generation `9`, preserved the touched path, and its binary bytes equaled the direct native service bytes.
- `bun nx run @semio-tech/stdio-plugin:test-quick -- touched_regions_are_stable_precise`: **1/1 passed**.
- `cargo test -p semio-framework-plugin artifact_inference_wire -- --nocapture`: **1/1 passed**.
- `cargo test -p semio-framework-plugin-host stale_or_empty_guest_results_are_never_publishable -- --nocapture`: **1/1 passed**.
- `cargo check -p semio-framework-plugin-host`: completed successfully; generated WIT host bindings compile.
- `cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest`: completed successfully; generated WIT guest exports compile for WASI Preview 2.
- `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --features semio-framework-plugin/component-guest`: blocked before the stdio crate by the existing `libz-sys` cross-compilation environment (`zlib.h` absent and the native zlib smoke target unavailable for `wasm32-wasip2`). This is not a Rust/WIT contract failure; the host-side bindgen path compiled successfully.

The shared Cargo build directory was contended by concurrent agents. Two Nx attempts exceeded the repository's 30-second quick-test budget while waiting/compiling; identical focused reruns passed after the lock cleared.
