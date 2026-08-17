# Wave 5-B Runtime Evidence

## Method

Temporary `eprintln!` calls prefixed `[DEBUG]` were added only to the existing cold-native inference, inference wire-boundary, and semantic mutation round-trip tests. Each test was run through the repository's Bun/Nx route with nextest output capture disabled. The temporary lines were then removed with `apply_patch`; `rg -n '\[DEBUG\]'` over both touched test files returned no matches.

## Cold native inference

Command:

```text
bun nx run @semio-tech/stdio-plugin:test-quick -- cold_native_inference -- --nocapture
```

Captured runtime line:

```text
[DEBUG] cold-native artifact=s.stdio.gltf inference=s.stdio.gltf.inference snapshot_bytes=78 inference_bytes=38198
```

Result: `1/1 passed`. The service decoded the canonical snapshot pack, produced the typed inference, encoded it with the frozen binary codec, and matched the direct typed result.

## Guest wire boundary

Command:

```text
bun nx run @semio-tech/stdio-plugin:test-quick -- inference_wire_echoes_revision -- --nocapture
```

Captured runtime line:

```text
[DEBUG] wire-boundary revision=7 generation=9 cache=cold inference_bytes=38198 changed_paths=["document/nodes/0/transform"]
```

Result: `1/1 passed`. The wire result echoed the requested revision and generation, preserved the semantic touched path, reported the honest cold mode, and its 38,198 inference bytes exactly equaled the native service's frozen binary output.

## Semantic mutations

Command:

```text
bun nx run @semio-tech/stdio-plugin:test-quick -- semantic_operations_report_stable -- --nocapture
```

Captured runtime lines:

```text
[DEBUG] semantic-mutation operation=TransformNode { index: 1, matrix: None, translation: Some([1.0, 2.0, 3.0]), rotation: None, scale: None } touched=["document/nodes/1/transform"]
[DEBUG] semantic-mutation operation=ReparentNode { index: 1, parent: Some(0), scene: None, position: 0 } touched=["document/nodes/0/hierarchy"]
[DEBUG] semantic-mutation operation=BindNodeMesh { index: 1, mesh: Some(0) } touched=["document/nodes/1/mesh"]
[DEBUG] semantic-mutation operation=BindPrimitiveMaterial { mesh: 0, primitive: 0, material: Some(0) } touched=["document/meshes/0/primitives"]
```

Result: `1/1 passed`. Every operation produced a non-empty stable semantic region, applied exactly, and its inverse restored the original snapshot.

## Broad GLTF gate

Command:

```text
SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/stdio-plugin:test-quick -- gltf
```

Result:

```text
Starting 91 tests across 1 binary (3367 skipped)
Summary [0.507s] 91 tests run: 91 passed, 3367 skipped
```

There were no failures to classify. The passing set includes GLTF/GLB decoding and round trips, accessor normalization/sparse/interleaved cases, grammar and protocol conformance, diff algebra and codecs, precise touched regions, all current geometric indicators, inference text/binary corruption laws, dependency invalidation, reference-safe semantic mutations and inverses, morph-target transport, native inference, and wire parity.

## Cleanup verification

No temporary debug output remains in production or test sources modified for this evidence wave. The earlier cross-compilation limitation remains environmental and separate: building the full stdio plugin for `wasm32-wasip2` reaches `libz-sys` and fails because its zlib C smoke build lacks a WASI-compatible target/header; the framework WIT guest and host bindings themselves both compile successfully as recorded in the Wave 4-B report.

