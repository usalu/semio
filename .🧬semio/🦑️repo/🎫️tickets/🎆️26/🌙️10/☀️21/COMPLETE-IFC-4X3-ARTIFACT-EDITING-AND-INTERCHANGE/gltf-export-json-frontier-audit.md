# glTF Export And GLB JSON Frontier Audit

Read-only source plan for the remaining actual TypeScript boundaries in io/gltf.ts. Source capture correction is independently checked in gltf-source-independent-audit.md. This report identifies uncheckpointed work/copy, not a new IFC/glTF semantic conformance bug or measured native receipt failure. No production edit or duplicate full suite was performed.

## IFC triangles to glTF snapshot

After controlled geometry extraction, vertex serialization checks every fixed 2048 vertices rather than captured batchSize. The entire indices loop at line 30 has no checkpoint. Per-mesh Uint8Array allocation, final chunks aggregation with bytes.set, and final Array.from(bytes) can each process a full payload synchronously. Array.from occurs after the final gltf progress checkpoint, leaving no opportunity to cancel that last scalar-array birth before publication. Source ownership is already private; the missing boundary is original-control scheduling through materialization.

Use original captured batchSize for vertex and index emission. Populate the actual schema-owned final Array<number> buffer in bounded byte batches directly from admitted mesh chunks, avoiding a second complete intermediate Uint8Array plus whole Array.from when practical. If chunks remain, read only a bounded byte slice per turn; bytes.set(chunk) is still a whole-copy primitive even when called once per chunk. Check cumulative safe byte count and the admitted byte/value ceiling before allocations; do not defer uint32/range failure until after copying. Allocation itself remains a JavaScript host primitive, not a five-axis native loan.

Neutral tests should require nonzero original phases for vertex, index and final byte/scalar-buffer emission at batchSize=1; abort at a later prefix of each and assert no artifact publication. Rebind the caller's control object in that callback and abort the original signal to prove original-control custody. Keep the existing actual Khronos geometry and negative-scale/Three comparisons unchanged. Do not reduce success assertions or cancellation granularity to gain throughput.

## GLB writer JSON

Current writer builds native(document) through recursive arrays/Object.entries, serializes the complete tree with JSON.stringify and encodes all UTF8 text in one TextEncoder.encode before checking total GLB uint32 size. The shared plain copier charges values/properties/depth but shares immutable string scalars; therefore one very large name/extras string can pass the same source-property ceiling as a short string. That ceiling does not bound complete encoded JSON size or synchronous string work. Large tree conversion, serialization and encoding have no interior progress/cancel observation.

A concrete long-term boundary is a schema-owned iterative token/fragment writer behind an async/generator interface, preserving the current native conversion: exact safe integral indices, Binary64 rendering, canonical omission/default behavior, attribute/target member representation, extras/extensions values and last-wins object semantics. Count encoded UTF8 bytes progressively and refuse before exceeding the explicit framing/output ceiling. Encode/copy only bounded string slices, keeping Unicode surrogate boundaries correct, and check original cancellation between fragments. Final 32-bit length arithmetic must use safe checked cumulative counts. A preflight alone followed by the same whole stringify/encode leaves the expensive second pass uncancellable.

Tests should include one long ordinary name and long nested extras, escaping, non-BMP Unicode, duplicate extras object members, safe-index boundaries and JSON/BIN alignment. Abort while visiting/encoding later string fragments, retain original control and assert no partial public GLB. Compare the complete success bytes/document with the genuine existing writer semantics and independent Three/Khronos loading. No third-party runtime serializer is required or proposed.

## GLB reader JSON

Raw source copy now establishes private byte ownership and limits complete source bytes with maxValues. Inside the first chunk, fatal UTF8 decode, JSON.parse and parseGltfDocument remain whole synchronous passes. The raw byte ceiling bounds their input size but does not supply progress, cancellation or the same explicit container-depth/record budget during those passes. The chunk loop itself also has no checkpoint for repeated unknown chunks; BIN padding check is at most three bytes and is not the material issue.

Prefer a schema-owned cooperative UTF8/JSON ingress interface rather than a second disconnected generic parser. Preserve exact current framing and syntax choices: first JSON, embedded BIN, aligned extents, last-wins object keys, finite numbers/integral index validation, UTF8 error policy/BOM policy, and unknown chunk behavior. Generic depth/node limits should fail explicitly before the typed projection allocates an unrestricted tree. Keep physical GLB framing/control admission distinct from typed document semantic refusal.

Until that interface exists, report this synchronous JSON segment as an explicit bounded-input host primitive; do not describe raw byte-copy cancellation as cancellation inside JSON parsing. A test with a long JSON string/tree should reach an actual interior parser/typed-projection phase before claiming the frontier closed. The existing exact raw-subarray/brand/getter/source ownership tests remain necessary and independent.

This plan does not assert native execution: authentic Rust compiler ingress still refuses before rustc. Native original-wallet gates and retained admission remain unchanged.

## Export materialization correction reviewed

Current ifc4x3ToGltf source checks cumulative safe-integer/u32/maxValues byte totals before each per-mesh allocation. Positive JavaScript array lengths and the cumulative ceiling bound component byte extents and offsets before DataView writes. Vertex and index loops now use captured original batchSize and completion checkpoints. Final Array<number> is filled directly from chunks in original byte batches; the complete merged Uint8Array and Array.from are gone. Final byte completion and import/export completion are both awaited before returning the artifact. No new concrete arithmetic/control defect was found in this narrow source review. The host allocation itself and geometry-extraction source custody remain separate previously documented boundaries.

Parent reports six export cancellation tests / 36 assertions and fresh complete official Khronos emitted validation/Three comparisons. The source review does not turn those TypeScript results into native custody evidence. Remaining writer/reader JSON work below is unchanged by export materialization.
