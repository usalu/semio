# Text, Office, Table, and Archive Editor Integration

## Retained edit acceptance fixtures

These values target fields present in each editor's `initial_snapshot()`. Each replacement changes the snapshot, survives the snapshot type's `FromValue`/`ToValue` round trip exactly, and avoids changing the artifact schema identity.

| Catalog key | RFC 6901 path | Typed JSON value |
| --- | --- | --- |
| `txt` | `/trailingNewline` | `true` |
| `md` | `/blocks` | `[{"kind":"paragraph","inlines":[{"kind":"text","text":"Edited"}]}]` |
| `csv` | `/hasHeader` | `false` |
| `tsv` | `/trailingNewline` | `true` |
| `json` | `/value` | `true` |
| `xml` | `/doc/root` | `{"kind":"element","name":"root","attrs":[],"children":[]}` |
| `html` | `/doctype` | `"DOCTYPE HTML"` |
| `xlsx` | `/workbook/sharedStrings` | `["Edited"]` |
| `docx` | `/document/body` | `[{"kind":"paragraph","runs":[{"text":"Edited","bold":false,"italic":false,"underline":false,"extraRunProperties":[]}],"style":null,"extraParagraphProperties":[]}]` |
| `pptx` | `/presentation/slides` | `[{"shapes":[]}]` |
| `pdf` for every 1.4 editor | `/pages/0/text` | `"Edited"` |
| every `s.stdio.pdf@1.7/*#editor` coordinate | `/metadata` | `"<xmp>Edited</xmp>"` |
| `zip` | `/comment` | `"Edited"` |
| `deflate` | `/windowBits` | `6` |
| `binary` | `/bytes` | `[0,255,92,32]` |
| `bcf` | `/version` | `"2.1"` |
| `epw` | `/location/city` | `"Hannover"` |

The PDF family needs exact application overrides for the seven 1.7 coordinates because PDF 1.4 and PDF 1.7 deliberately use different snapshot schemas. The affected subset coordinates are `*`, `a`, `e`, `h`, `ua`, `vt`, and `x`.

## Details paging

The shared Details renderer uses a lazy ordinal window primitive. It does not allocate a collection proportional to pixel, byte, vertex, archive-member, or document-node counts. Every collection node owns a scoped `TreeWindows` path, and nested host requests address the full ancestry. Collection controls remain fixed children and do not shift the logical row offset.

RFC 6901 paths use the direct `path`/`from` action fields while they fit one bounded UI text value. Longer paths use the canonical `pathChunks`/`fromChunks` lists, split only at UTF-8 character boundaries. Stable node ids are hashes and never contain the potentially long user key. Paths larger than the bounded UI list carrier remain editable through the complete typed snapshot Source draft.

The renderer derives creation controls from the registered snapshot JSON Schema. Empty arrays of records receive a valid item template, nullable records receive a complete non-null template, and missing object properties receive one typed creation control each. Generic primitive insertion is shown only when the schema permits untyped values. String enums render as localized selects; other enum values render as typed choices. Integer, unsigned integer, and floating-point controls carry exact JSON text with `valueEncoding: "json"`, avoiding JavaScript number coercion and preserving `1` versus `1.0`.

Long strings, long keys, and the full typed snapshot Source use prepopulated local drafts with explicit localized Apply/Discard, collaborator-change conflict handling, applying/cancel/failure states, and distinct surface ids. Validation failures do not replace the persisted document and the local draft remains available.

## Native editing

CSV, TSV, XLSX, BCF, and EPW table cells dispatch format-owned mutation actions from editable table cells. XLSX row ordinals use the same complete workbook flattening in rendering and reduction, so edits retain the exact worksheet and sparse cell address.

TXT, Markdown, and HTML keep complete natural-file text canvases whose canonical `textEdit` action accepts empty documents and publishes retained format-owned mutations. JSON and XML keep their windowed structure canvas and add a prepopulated natural JSON/XML source draft that applies intentionally through the format-owned root `set-node` route. The typed Details Source remains an advanced complete snapshot JSON view.

DOCX, PPTX, and every PDF editor pass the host `TreeWindows` ledger into `DocumentWindowKit::render_windowed`, making every page reachable without assembling the whole document. PPTX derives both hosted and unhosted page text from every text-bearing shape.

Binary renders the entire admitted byte buffer as editable lowercase hexadecimal through the paged text scene; it no longer drops the tail after a preview cap. Its strict retained `textEdit` route rejects odd or non-hexadecimal drafts without changing the document. Deflate exposes its compression header through the same canonical text route while the full payload stays editable in Details. ZIP entries and all other nested archive fields use the shared typed collection controls and retained snapshot mutations.

## Schema constraint boundary

Every accepted Details event now validates the complete candidate snapshot against the registered normative `snapshot` JSON Schema after typed lossless decoding and before a mutation is emitted. The validator resolves cross-schema `$ref` documents through the framework schema-export registry and caches the compiled validator by exact snapshot schema id. A failure returns `snapshot-edit.constraint-invalid` with the validator's field path, such as `$.windowBits`, while the original snapshot remains unchanged. Invalid registered schema contracts are reported separately as `snapshot-edit.invalid-schema-contract`.

The neutral regression applies a maximum-constrained integer through the same typed event path and verifies the boundary value, the rejected value, field diagnostic, and atomic preservation with a `serde_json` serialization oracle. The Deflate regression registers its real descriptor and verifies that `/windowBits = 15` is accepted while `255`, which fits the Rust `u8` carrier but violates the RFC 1950 schema maximum, is rejected without mutation.

## Focused validation

- `NX_DAEMON=false bun x --no-install nx run @semio-tech/stdio-artifact-contract-rs:check --output-style=static` passed after adding the first-party schema registry dependency.
- The feature-enabled CSV editor baseline completed with 13 passed and 44 filtered tests.
- The focused Details suite completed with 9 passed and 13 filtered tests after the schema-template, nested-paging, typed numeric, enum, and chunked-pointer additions.
- The natural JSON/XML source, binary, BCF, PPTX, and schema-constraint checks remain queued behind the repository's serialized Cargo build and are not recorded as passing until their current Nx runs finish.
