# Layout Vector Media Projection Decision

The defining media vocabulary in framework/manifest/media declares Structured payloads as explicit JSON and names its data field json. Existing generic transport presently copies that UTF8 text without parsing it, and a default importer uses a base64 string. Those permissive implementation details do not establish that raw SVG satisfies the defining Structured JSON contract. The existing Layout vector/SVG intent remains useful, but its old raw SVG plus application/json output is false.

The clean local solution is a schema-first Layout vector projection: genuine SVG carried inside a bounded JSON envelope under the semantic2d.layout kind. Keep only genuine essential source identity metadata; distinguish current authenticated frontier from the selected checkpoint content identity. Do not fabricate or relabel one as the other. MIME application/json becomes truthful. Ordinary SVG download export remains an actual SVG codec.

Reuse the existing core raw/encoded ChunkRopes, original output credit, progress, cancellation and controlled commit/retirement. Escape incrementally before PackageCommit and emit through the existing encoded queue. Do not add unbounded full copies, an extra unfunded queue, runtime external dependencies, a broad framework payload variant, or compatibility adapters. Medium engine execution and Rust schema/consumer/law execution have separate source ownership.

Native regression must exercise actual submitted media output, independently parse the JSON envelope and the inner XML, validate its authored neutral schema, and retain original child-reader/context and cleanup witnesses. No native or live browser pass is claimed by this design decision.
