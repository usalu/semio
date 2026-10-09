# PDF Typed Retained Projection Design

The retained graph currently calls `io::carry_graph_edit` from semantic diff and conformance construction. That function reruns the native lifter, including PDF content, CMap, string encoding, date and metadata decoding. Moving the function alone would conceal codecs in schema. Native date parsing and formatting are first moved to the explicit text snapshot date owner, keeping `PdfDate` as pure component values.

The retained graph must own known decoded records rather than native-layout bytes when semantic projection depends on them. A physical graph admission pass must resolve native roles and decode text/date/content/CMap/image records once. The semantic graph projector consumes those first-party typed records and performs resource identity renaming only. Opaque unknown attachments may retain bytes, but known content interpretation belongs to physical admission/emission. No cached serialized mutation result or compatibility byte authority is introduced.

Date extraction validation is pending the registered native PDF suite. The existing native grammar test is relocated to its physical owner; semantic date value roundtrip tests remain in schema.

## Content Application Ownership

A retained content stream may be used by different pages or forms with different inherited resource dictionaries. Physical admission must decode native operands under each application context, so a single decoded-text cache attached only to the byte stream is insufficient. The canonical graph needs semantic content operands and explicit application resource bindings; native font-width grouping and Unicode decoding are completed in the IO binder. Pure projection may resolve aliases into resource identities, but may not reinterpret native string bytes, content tokens, CMap programs, dates or XML metadata.

Known string/date/content/CMap roles require first-party typed payload variants. Opaque binary retained values are admissible for unknown attachments and document identifiers. Known native payload conversion is not deferred to mutation apply, and previously serialized mutation outputs are not used as authority. The registered PDF production check after date extraction completed successfully; full retained projection remains outstanding.

The first pure projection component is now `schema::content_mapping`: resource identity maps and `rename_content` consume decoded operators and first-party names only. The physical lifter imports that component directly. Native parsing, font code decoding and retained graph lane carry remain outstanding; moving these pure operations does not establish full graph closure.

## Canonical Lookup Extraction And Actual Verification

`schema::graph_source::{ObjectSource, GraphSource}` now owns first-party retained reference lookup. Physical xref resolution implements the semantic source trait and no forwarding xref alias remains. The physical lifter and colour/image adapters import the semantic source directly. Exact existing lookup behaviour is preserved in this extraction.

Registered `@semio-tech/stdio-pdf-rs:check` attempt 2 completed successfully with Cargo 13.77 seconds and Nx 35.3 seconds, after fixing the component mount's sibling lookup to the canonical schema snapshot path. These pure components do not remove the known `carry_graph_edit` violation by themselves.

## Remaining Concrete Codec Inputs

The native lifter interprets `PdfObject::Str(Vec<u8>)` as PDFDocEncoding or Unicode text in metadata, destinations, filespecs, actions, outlines, form options and name-tree keys. Date positions then parse native text. `Stream.data` is interpreted as native content operators on pages, forms, tiling patterns and Type3 charprocs; native ToUnicode/CMap programs on fonts; UTF8 metadata, JavaScript and rich form content; and PostScript function text through the colour helper. CID-system registry/ordering and URI positions also decode native byte spelling. Unknown binary streams, embedded files, document IDs and opaque font programs remain acceptable retained bytes when no semantic operation interprets their representation.

The next model change must represent those known decoded values directly and make native admission/emission the only codec owner. Semantic lane carry should be rebuilt over owned graph records and explicit decoded application contexts. `PdfTextString::Codes` currently owns native byte arrays, so code-width grouping must also become physical; logical code units may be retained as unsigned words and mapped by pure first-party font values.

## Native Content Fragment and Font Ownership Constraints

The native lifter concatenates all page Contents stream bytes with a newline before parsing, so physical record admission must handle a page content sequence before handing operators to semantic projection. Parsing each native stream independently would change valid PDF programs that split one operator or operand across streams. The canonical semantic sequence must contain complete typed operators; physical output can divide those complete operators at declared stream record boundaries. A retained fragment partition is layout owned by I/O, not an instruction for the schema to tokenize bytes.

Font-sensitive native string admission must produce logical integer character codes, with native byte grouping performed in I/O against admitted codespace widths. The pure projection can then map those integer codes through owned ToUnicode/CMap/base encoding records for each application resource binding. This avoids caching a Unicode string interpreted under one page or form font environment when the same retained stream is shared by another environment. The current PdfTextString Codes byte carrier should become an explicit logical code sequence in the decoded retained content owner; the existing physical text/native codecs must bind their wire bytes into those values.

Known graph role payloads need authored typed text, PdfDate, operator sequence, ToUnicode, embedded CMap, UTF-8 metadata/JavaScript, and calculator function program owners. Unknown records and font/image/attachment payloads that semantic code does not interpret can remain explicit opaque bytes. Existing graph field edits must mutate these typed owners; no parser callback, native facade alias, or serialized mutation output cache belongs in the pure projector.

## Additional Semantic Helper Producers

PDF schema conformance-support constructs placeholder ICC and embedded-file native stream bytes from formatted strings at lines 286 and 343. These producers must be revised with the owned retained payload model too: the semantic helper should author a decoded or explicitly opaque domain value, and physical I/O should perform the native text-byte construction. The generic namespace gate currently reports the carry_graph_edit import rather than these indirect native producers, so passing that gate alone will not establish full PDF closure.
