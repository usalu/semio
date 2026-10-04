# HTML Public Snapshot I/O Light Audit

Read-only source observation 2026-10-03; read current stdio route readback first. No builds, Cargo, runtime execution, Git changes, or production edits. Authored laws are not execution receipts.

## Actual Owners and Fields

Artifact root `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🦀️.rs` declares exactly one factory: `stdio.native.html.v1`, schema `stdio.html`, dialect `s.stdio.html@5:*`, extension `html`. `native_codec()` constructs `ArtifactCodec::of::<HtmlSnapshot, HtmlMutation>` and hashes the owner's binary protocol. `contribution()`, `definition()`, `assembly()` and `declaration().document_codec_bare` provide the actual owner publication chain. Stdio root now accepts a caller-selected ContributionRegistry; HTML inclusion is not a global default.

All paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/`:

- `🧬️schema/📸️snapshot/🦀️.rs`: HtmlSnapshot owns `schema: String`, optional raw `doctype: Option<String>`, root HtmlNode. Element owns name, ordered attributes, ordered children; attribute owns name and optional value (None distinct from empty); Text and Comment own text; RawText owns Script/Style parent kind and text. NodePath is addressing, not stored metadata. No namespace, source span, quote style, prolog comments, or extra prolog records are owned snapshot fields. Do not invent them for full-cohort claims. Root permits every node variant in the typed model, though HTML native parsing/writing has narrower document semantics.
- `🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql`: 8 domain tables: html_document (id/schema/doctype/root_node_id), html_node (id/kind), html_element (node_id/name), html_text and html_comment (node_id/text), html_raw_text (node_id/parent_kind/text), html_attribute (id/element_node_id/ordinal/name/value), html_child (id/parent_element_node_id/ordinal/child_node_id). Schema is inside sqlite directory, not directly snapshot directory.
- `🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`: iterative projection/reconstruction, explicit typed-component ownership and ordering validation; document id is fixed to 1. Node/component/reference ids need not be traversal ids on reconstruction. Attributes and child rows sort by authored ordinals. Root validity checks membership, not Element kind.
- Snapshot file lines 774 onward opts ArtifactPack into `sqlite_snapshot_codec`; provider overrides preflight and subset guard but neither `decode_sqlite_snapshot_native` nor `encode_sqlite_snapshot_native`, nor recursive retirement.

## Public Boundary

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10886` defines native hooks. Defaults checkpoint then explicitly refuse unsupported controlled native decoding/encoding. Erased export calls native decode; erased import calls native encode (not preflight-plus-legacy-print). Thus HTML's existing erased Text/Binary import law currently has an unsupported hook dependency, known staged work; no run was performed here.

`🧰️framework/🔨️modules/🚪️io/🦀️.rs:2087` publishes NativeSnapshotRegistration from the actual codec capability; schema/TypeId/function identity participate in conflict validation. Exact registered typed-file APIs at 2413/2425 check owner TypeId and schema before projection/reconstruction, carry reserved `semio_snapshot` metadata, and avoid native wire lowering. Supplemental registered family routes use the erased codec. Native means public artifact payload bridge here; SnapshotEncoding currently has Text/Binary, not a Native enum variant.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:41960` exposes Plugin schema/export/import wrappers. These wrappers and actual Plugin assembly are not exercised by the six owner-local laws. An owner's direct helper or `ArtifactPack::sqlite_snapshot_codec()` call does not certify assembled family registration.

## Current Authored Laws

Selected file `🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` contains **6 #[test] laws, 0 async_test laws**. The two async import-deserializer tests under `🚪️io/🧪️tests/🔬️import-deserializers-unit/🦀️.rs` lack sqlite_snapshot selector and are not SQLite selected laws.

| Existing law suffix after sqlite_snapshot_html_ | Coverage authored |
|---|---|
| owned_dialect_guard_requires_its_declared_coordinate | exact owner coordinate, bad kind/standard/subset/schema, immediate guard cancellation |
| preserves_boolean_empty_attributes_and_raw_text | typed projection + raw SQLite byte helper roundtrip; pack/DSL comparison |
| optional_and_variant_fields_survive_without_xml_coercion | custom schema, absent doctype, each leaf/root variant, empty element attributes, default |
| rejects_invalid_graphs_shapes_and_resource_limits | 6 corruptions; exact/one-below domain row/value limits; immediate and third-checkpoint projection cancellation |
| independent_queries_edits_and_html_parser_oracle | Bun SQLite integrity/FK checks and attribute edit; HTMLRewriter ordered element/attribute observation |
| erased_native_preflight_admission_and_limits | direct erased import Text/Binary, helper preflight limit/cancel; no registered factory invocation |

No owner-local typed-file, actual factory identity, Plugin-public, registered family route, identifier renumber, permuted table/row, full native deep-cohort, progress monotonicity, mid-reconstruction cancellation, or recovery-after-cancellation law is authored in that selected file. Bun oracle is authored, not rerun. Existing JSON fixture `🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json` supplies HTML text + coordinate/invalid coordinates + query/expected key values; tests still hard-code some expected values. It is neutral source data, not a complete neutral law matrix.

## Proposed Neutral Law Matrix

| Cohort | Required source assertions | Public boundary |
|---|---|---|
| All owned fields | schema custom/default; doctype absent/empty/nonempty; every root variant; element mixed nesting; ordered duplicate/name-case attributes; None/empty/Unicode/entities; ordered empty/text/comment children; Script/Style raw text | typed registered file export/import; typed equality and relational equality |
| Native admitted documents | same semantically admitted HTML document fields through Text and Binary; deep and wide trees; long single text/comment/attribute/raw text; absent doctype | actual owner factory codec and registered family export/import; Plugin wrappers separately |
| Publication | exact factory id/schema/extension/hash/TypeId/SQL; declaration and selected contribution assembly; one exact bidirectional family hop and public schema resolution | actual factory + real assembly, not constructing ad hoc registration |
| Relational independence | permute rows/tables; renumber node ids and all component/attribute/child/document-root FKs consistently; renumber attr/child ids; preserve ordinal order | independent SQLite edit then typed/public import; retain document id=1 contract |
| Malformed graph | dangling root/component/parent/child, duplicate ownership, missing/extra/wrong component, cycle/disconnected graph, negative/duplicate/gapped ordinals, wrong cell types and schema | independent edit, public rejection with no success payload |
| Controls | cancel each observed phase initially and mid-way; bounded checkpoints for large single fields/deep/wide inputs; exact/one-below row/value/file/table limits; progress bounds and terminal completion; repeat success after failure | typed file + family + native; metadata rows/value costs included in public limits |
| Oracle | external SQLite integrity/FK + ordered queries and full-cohort edit; independent HTML parser for admitted native document semantics | neutral expected data; no helper-only coverage substitution |

Source control gaps: projection uses unchunked String copies and attribute `.into()`; reconstruction uses unchunked `.into()`/to_string and ordinary sort_by_key; traversal pushes can process large sibling sets without checkpoint inside stack extension; recursive HtmlNode default retirement can remain deep-stack sensitive. Preflight does not include self.schema and starts with fixed 1024, so require explicit huge-schema resource law. Measure/construct counters differ (component rows count in total but not all ticks; reconstruct re-ticks built rows), so require observed progress law before asserting monotonicity. Native parser/serializer and pack implementation remain recursive and uncontrolled by the absent owner native hooks. These are source observations, not runtime failures.
