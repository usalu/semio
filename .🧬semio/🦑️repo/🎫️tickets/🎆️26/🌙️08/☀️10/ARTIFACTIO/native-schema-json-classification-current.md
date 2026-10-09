# Current Native Schema JSON Call Classification

The independent AST census reported 143 calls in 3016 TypeScript schema-path sources. These counts include test command routers, test-support and contract probes; those are not semantic runtime codecs. Generic directory transport, deployment metadata, repository source inspection and schema generator serialization use separate host representation boundaries and are not artifact snapshot or mutation codecs.

The concrete remaining artifact-related findings from the current review are:

- Print inference rows encode bundling and polygon coordinate paths as JSON strings. Replace these hidden textual bodies with scalar typed coordinate rows (`detail`, `order`, `x`, `y`) and update the independent D3 witness. Group keys and render-style comparison also stringify semantic values; compare their owned values directly.
- GIS Map's root schema TypeScript module is used only by one test and loads JSON schema from the filesystem through Ajv. Move the compiler to explicit test support and redirect the only consumer.
- Raster editor selection config embeds JSON coverage spans in a string and its schema parser decodes them. This lies outside artifact mutation schemas, whose coverage spans are already typed. The editor boundary still needs explicit typed spans; do not regard the guard GREEN as proof of this consumer boundary.
- Remodeling and generated PDF validators use JSON only in diagnostic messages or schema declaration constants, not for artifact representation admission. No representation mutation behavior is inferred from these calls alone.

Each remaining classification must be supported by reading its enclosing definition and consumer, rather than adding a blanket JSON-call ban. Native execution remains open while shared Value retirement compilation changes.
