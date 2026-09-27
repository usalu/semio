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

The default native provider now traverses `ToValue::value_shape_at_path`, `value_key_at_path`, and scalar `value_at_path` projections. Opening an object or array no longer calls `to_value()` for the complete snapshot. The typed Source projection remains collapsed by default and materializes only when the host explicitly opens that section. A neutral native provider test makes `to_value()` panic and proves the Details tree can still enumerate keys and edit a scalar.

RFC 6901 paths use the direct `path`/`from` action fields while they fit one bounded UI text value. Longer paths use the canonical `pathChunks`/`fromChunks` lists, split only at UTF-8 character boundaries. Stable node ids are hashes and never contain the potentially long user key. Paths larger than the bounded UI list carrier remain editable through the complete typed snapshot Source draft.

The renderer derives creation controls from the registered snapshot JSON Schema. Empty arrays of records receive a valid item template, nullable records receive a complete non-null template, and missing object properties receive one typed creation control each. Generic primitive insertion is shown only when the schema permits untyped values. String enums render as localized selects; other enum values render as typed choices. Integer, unsigned integer, and floating-point controls carry exact JSON text with `valueEncoding: "json"`, avoiding JavaScript number coercion and preserving `1` versus `1.0`.

Tagged `oneOf`/`anyOf` details select their schema branch from the current native value shape and a disjoint `const`/`enum` discriminator read through the lazy provider. Shared field names therefore use the active variant's enum, title, description, and creation constraints. Missing or ambiguous tags expose no branch-specific metadata instead of silently taking the first branch. A discriminator is display-only; the union object exposes localized variant-switch actions that replace it with a complete schema-derived target template. Scalar shared fields whose schemas are identical in both branches are retained, while branch-specific fields receive valid target defaults. The neutral binding fixture verifies the complete `/variant` replacement and retained shared label.

Details capabilities are derived from the selected schema branch before controls are assembled. Fixed declared property names do not expose Rename, required properties do not expose Remove, `/schema` is display-only, dynamic `additionalProperties` keys remain renameable/removable, array Remove disappears at `minItems`, and collection insertion disappears at `maxItems`/`maxProperties`. Declared properties absent from the native value appear as localized typed creation buttons using their schema default or generated valid template. The neutral fixture asserts the concrete control and action-argument presence in English and German.

Schema presentation follows the same registered document graph as retained validation. Details indexes sibling snapshot documents by their normative `$id`, rewrites exact local and external references into one bounded bundle, and intersects `allOf` object fields, required names, enum values, collection limits, and numeric/string bounds before choosing a union branch. The production-shaped registry law registers an envelope plus two external sibling descriptors, selects the active branch from its nested discriminator, verifies nested fixed/required controls, its enum and German title, and a missing optional field, then validates the complete alternate variant with both the first-party cross-document validator and `serde_json`. Nested `schema` identities and `const` fields are display-only.

Variant switches and declared missing-property controls are themselves lazily windowed. The provider exposes indexed count/read operations, and the renderer materializes only the host-requested slice under the correct nested `TreeWindows` scope. The language-neutral paging fixture advertises 1,000 alternate variants and 1,000 missing properties, requests three and two rows respectively, and counts exactly those five provider reads. No temporary vector proportional to either schema-derived control set is allocated.

Generated creation candidates are checked twice before the UI exposes them: a bounded schema-aware constructor rejects incompatible defaults and fills required/minimum object content, then a cached authoritative `OwnedJsonSchemaValidator` validates the exact candidate against the resolved fragment and bundled definitions. The regression includes an invalid annotation default rejected only by a regular-expression constraint, proving the button is omitted instead of advertising an edit that the reducer must reject. The same final validation runs after compatible fields are retained during a tagged-union switch.

Large dynamic objects no longer scan their existing key set merely to render controls. Declared missing properties use targeted path probes, dynamic insertion asks the provider for a bounded conflict-free candidate, every derived pointer is checked against the chunk carrier before binding, and `minProperties` suppresses impossible removal. Neutral lazy-object and exact-path-budget laws are authored; the active native contract run has not yet produced a result for them.

Long strings, long keys, and the full typed snapshot Source use prepopulated local drafts with explicit localized Apply/Discard, collaborator-change conflict handling, applying/cancel/failure states, and distinct surface ids. Validation failures do not replace the persisted document and the local draft remains available.

## Native editing

CSV, TSV, BCF, and EPW table cells dispatch format-owned mutation actions from editable table cells.

CSV, TSV, BCF, and EPW require renderer-shaped numeric `row`/`column` arguments and an explicitly present text `value`; an empty cell value remains valid. CSV, TSV, and BCF already own retained native factories. EPW now registers the shared bounded native factory for `set-cell`, advertises its proof/publication contract, and shares one guarded reducer between direct and retained execution. Unknown columns and stale rows return a fault instead of publishing a silent no-op. A registered host-dispatch law uses the actual numeric renderer arguments and verifies the persisted record field.

CSV, TSV, and BCF now bind each rendered cell to a complete `{row,column,revision}` address through `render_editable_cells`. Hosted renders use the store's already-computed fixed canonical revision, and admitted commands compare against the corresponding `AppOperationContext` revision. Normal rendering and command preflight therefore encode only 32 bytes instead of serializing the complete table; unhosted unit rendering retains an explicit snapshot-derived fallback. A collaborator change rejects the pending draft before positional rows can redirect it, while a still-current empty value remains a valid intentional cell clear. Their command codecs preserve Unicode, newlines, literal escape-like text, and reject malformed Unicode hexadecimal without slicing a UTF-8 string at byte offsets. Parser, retained dispatch, stale-revision, stale-address, scene-payload, and authoritative-revision laws are authored; the active shared native run has not yet produced a result for them.

The base, Strict, and Transitional XLSX roots now bind every editable value cell to its worksheet identity, native row and column, and a revision of the typed cell value. Their parsers require the complete renderer payload, binary replay preserves Unicode/newlines/literal escape sequences, stale addresses and collaborator revisions fault without mutation, and the shared bounded native factory owns retained execution. The value draft shows formula source as `=expression`; empty text clears the cell, formula text remains a formula, an apostrophe forces literal text, booleans retain their type, and only finite complete numbers become numeric cells. Shared-string indices and formula caches remain fully editable in Details. The language-neutral action fixture is decoded by both `serde_json` and the DSL JSON codec; root and scene laws cover exact command replay and emitted address/revision arguments. These laws are authored but await the active optimized/native builds before a passing result is claimed.

TXT, Markdown, and HTML keep complete natural-file text canvases whose canonical `textEdit` action accepts empty documents and publishes retained format-owned mutations. JSON and XML keep their windowed structure canvas and add a prepopulated natural JSON/XML source draft that applies intentionally through the format-owned root `set-node` route. The typed Details Source remains an advanced complete snapshot JSON view.

DOCX, PPTX, and every PDF editor pass the host `TreeWindows` ledger into `DocumentWindowKit::render_windowed`, making every page reachable without assembling the whole document. PPTX derives both hosted and unhosted page text from every text-bearing shape.

Binary renders the entire admitted byte buffer as editable lowercase hexadecimal through the paged text scene; it no longer drops the tail after a preview cap. Its strict retained `textEdit` route requires an explicitly present source, allows an intentional empty buffer, and rejects odd, Unicode, or non-hexadecimal drafts without changing the document. Deflate exposes its compression header through the same canonical text route while the full payload stays editable in Details. Its source parser requires each modeled field exactly once, rejects unknown fields, and enforces the snapshot schema's 0–15 compression-method and window-bit ranges before publishing either header mutation. ZIP entries and all other nested archive fields use the shared typed collection controls and retained snapshot mutations.

## Compact native mutation roster

The assigned directory set contains 34 concrete editor roots and 17 distinct mutation aggregates. The earlier 32-root estimate omitted two of the ten PDF coordinates.

| Aggregate | Editor roots | Compact snapshot leaf | Large unrelated payload law |
| --- | ---: | --- | --- |
| `CsvMutation` | 1 | Schema-first `PatchSnapshot` leaf, direct text/binary codecs, sparse diff, inverse, aggregate dispatch, and retained editor hook mounted | Neutral 2 MiB unrelated quoted-field fixture plus Rust and TypeScript oracle laws authored |
| `JsonMutation` | 2 | Schema-first `PatchSnapshot` leaf, direct text/binary codecs, sparse diff, inverse, aggregate dispatch, and retained editor hook mounted | Neutral 2 MiB unrelated object-member fixture plus Rust and TypeScript oracle laws authored |
| `PdfMutation` | 10 | Missing | Missing |
| `XlsxMutation` | 3 | Missing | Missing |
| `DocxMutation` | 3 | Missing | Missing |
| `PptxMutation` | 3 | Missing | Missing |
| `ZipMutation` | 2 | Missing | Missing |
| `XmlMutation`, `XmlValidMutation` | 2 | Missing | Missing |
| `TxtMutation`, `MdMutation`, `TsvMutation`, `HtmlMutation`, `BinaryMutation`, `DeflateMutation`, `BcfMutation`, `EpwMutation` | 8 | Missing | Missing |

Until these leaves are mounted into every aggregate, a Details edit may still publish a full replacement mutation whose forward or inverse payload exceeds the 1 MiB publication ceiling even when the changed path is tiny. The native compact leaf must carry only `SnapshotPatch`; its sparse diff must preserve every unrelated field, and its inverse must be derived against the exact pre-state. SetSnapshot remains useful for import/open replacement, not ordinary Details edits.

## Schema constraint boundary

Every accepted Details event validates against the registered normative `snapshot` JSON Schema after typed lossless decoding and before a mutation is emitted. Schema selection uses the editor's authoritative `ArtifactEditor::DOCUMENT_SCHEMA`, requires an exact registered descriptor, and verifies that the descriptor belongs to the editor artifact. It does not synthesize ids from standard/subset names or fall back by prefix. This resolves ordinary root descriptors such as `s.stdio.deflate` and versioned descriptors such as the PDF 1.7 schema without allowing an edited `/schema` field to select its own validator.

The full-snapshot reducer caches the compiled validator by exact native document schema. Compact path mutations use `OwnedJsonSchemaValidator::validate_dsl_fragment_with_context`: leaf constraints validate the candidate directly, structural operations request the smallest post-edit ancestor whose keywords observe membership, and the native context provider refuses explicitly after its bounded node budget. Standalone `oneOf`/`anyOf` records with disjoint `const`/`enum` tags select their branch by reading only the tag. The language-agnostic fixture keeps an unrelated 2 MiB payload, exercises integer range, enum, required removal, `minItems`, dependencies, and discriminated unions, and compares the complete candidate with AJV.

Fragment validation has separate value and shape projections. Insert/remove on a homogeneous array validates its final `minItems`/`maxItems` length through `Array { len }` and validates only an inserted item, so a 2,097,152-byte array does not cross the 65,536-node projection budget. The neutral vectors cover accepted large insert/remove operations plus rejected minimum/maximum boundaries. `contains`, enabled `uniqueItems`, and tuple-array positional shifts still require the bounded complete frontier; supporting those operations on arrays above that bound needs a cancellable streaming constraint projection and remains unfinished.

A failure returns `snapshot-edit.constraint-invalid` with the validator's field path, such as `$.windowBits`, while the original snapshot remains unchanged. Invalid registered schema contracts are reported separately as `snapshot-edit.invalid-schema-contract`.

The neutral regression applies a maximum-constrained integer through the same typed event path and verifies the boundary value, the rejected value, field diagnostic, and atomic preservation with a `serde_json` serialization oracle. The Deflate regression registers its real descriptor and verifies that `/windowBits = 15` is accepted while `255`, which fits the Rust `u8` carrier but violates the RFC 1950 schema maximum, is rejected without mutation.

## Focused validation

- `NX_DAEMON=false bun x --no-install nx run @semio-tech/stdio-artifact-contract-rs:check --output-style=static` passed before the lazy native projection and fragment-validation additions.
- The feature-enabled CSV editor baseline completed with 13 passed and 44 filtered tests.
- The focused Details suite completed with 9 passed and 13 filtered tests after the schema-template, nested-paging, typed numeric, enum, and chunked-pointer additions.
- The external-reference/allOf registry path, authoritative template filtering, bounded large-object controls, `minProperties`, nested schema identity, and complete variant-switch additions were authored after that 9-test run. They remain unverified while the sole replacement contract job waits on the shared native Cargo lock.
- The first replacement contract run reached the crate after 111 minutes and stopped before tests on twelve Details-test compile errors: six fixture paths, four `usize`/`u32` window fields, and one ambiguous generic JSON decode reported twice. Those were repaired together. Session `63260` then exposed one stale `dsl::ToValue` path; session `64122` exposed one owned-`UiValue` borrow and one remaining ambiguous JSON fixture decode. All reported compile errors are repaired. The sole warm replacement is session `73383` with log `🗑️generated/schema-constraint-contract-current-5.log`; it remains active and no passing result is claimed.
- The language-agnostic fragment fixture passed its AJV Nx oracle after the large-array additions: 1 test, 26 expectations, including the 2,097,152-item array boundaries. The first native contract rerun stopped in shared generated `ToValue` consumers before reaching this crate. The core owner repaired generated name hygiene, key enumeration, and `Self::from_value` ambiguity; the replacement focused contract run remains active in the shared Cargo queue. No native result is recorded as passing until that exact command finishes.
- The natural JSON/XML source, binary, BCF, PPTX, and representative table/document checks remain pending and are not recorded as passing until their Nx runs finish.
