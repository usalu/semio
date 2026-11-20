# Retained STL import

`brep.io.importStl` now runs as an original retained geometry job. It consumes the existing base64 `data` text and numeric tolerance, then publishes one `geometry` result through the same native mesh transaction as OBJ. The original text address and length remain bound to the cursor, and all typed backing it creates stays private until publication.

The STL decoder trims outer Unicode whitespace incrementally and validates canonical standard base64 in windows of at most 16 quartets. It accumulates only the fixed 84-byte STL header and decoded length from that validation pass. Header-based detection preserves binary STL with a `solid` header when the declared byte length matches, and allows trailer bytes for binary streams whose header lacks that prefix. Binary decoding rereads only the normal and vertex being emitted.

ASCII mode validates UTF-8 and reads at most eight scalars per event. It counts `vertex` records with valid finite coordinates, preserves the current `facet normal`, then repeats the cursor to fill original typed triangle buffers. Coordinate tokens have a 64-byte bound. Both formats admit up to 100,000 triangles. No entire decoded byte buffer is allocated. Positions, normals and indices receive separate exact-capacity retained grants and pass to the original native mesh cursor without element copying.

The shared native cursor validates coherent manifold edge uses and welds exactly equal positions. It produces an open shell when edges have one use and a solid when every edge has two opposite uses. The final native registry insertion and matching Session claim use the common retained gate. Before publication, cancellation rolls back private arena records one at a time. After publication, close retires scratch and journal state only. The mesh interpretation retains the existing cursor's single-shell representation.

## Verification

- The language-neutral cases cover an exact-length binary STL starting with `solid`, a binary trailer, an ASCII facet with no trailing newline, and a UTF-8 ASCII name. They check native kind, vertex and edge count, triangle count, area and signed volume. The malformed cases cover base64 padding bits, truncated binary facets, incomplete ASCII triangles and noncanonical padding after otherwise valid binary geometry.
- Three.js STLLoader agreed with all four geometric cases. The combined STL and OBJ oracle run passed 18 assertions.
- The extended native witness compiled the actual decoders, mesh cursor and Engine publication adapter against cached repository units. It compared allocator observations with every receipt, exercised both formats through commit and rollback, checked source identity and kernel revision, and tested invalid cases. It used real body and map types with a minimal registry adapter. Run `🐍️obj-import-native-check.py` with the unit store's `prototype/build` directory. Scoped scratch outputs are removed automatically.
- The registered Flow test `original_stl_import_publishes_binary_and_ascii_after_funded_topology` shares its assertions with the OBJ test. The full registered Cargo suite remains pending under the parent ticket's storage gate.
- Source closure measured 3,969 active lines for Engine, 4,000 for the native registry, 2,315 for mesh I/O, 2,634 for the Flow original jobs, and 1,826 for Session core. `git diff --check` passed.
