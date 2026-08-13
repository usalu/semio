# Packet: stdio Office/Document Formats — Engine Dissolution Report

Scope: `📕️xlsx`, `🎞️pptx`, `📝️md`, `📰xml` (stdio's remaining un-dissolved engines). Reference
pattern: `📜️docx` (already fully dissolved before this packet started).

## Result summary

All four `⚙️engine` directories are deleted. `xlsx`/`pptx`'s prior-session compatibility shims
(`pub mod engine { pub use super::standards::…::engine::*; }` in glue.rs) are also deleted — the
real dissolution replaces them, matching docx's own shimless end state.

```
find ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📕️xlsx,🎞️pptx,📝️md,📰xml} -name "⚙️engine" -type d
→ 0 results
```

## Destination per region per artifact

### 📕️xlsx (ecma-376/✳️any) — had a shim; real dissolution done this wave
- `XlsxError` + OPC/XML constants (`SML_NS`, `WORKBOOK_PART`, …) + `column_letter`/`column_index`/`column_letters_of` pure helpers → `🚪️io/🦀️component.rs`.
- `io_registry` (moved verbatim) → `🚪️io/🦀️component.rs`.
- `build_minimal_xlsx`/`encode_xlsx` + `*_to_xml` mapping (workbook/worksheet/sst XML render) → **new** `🚪️io/📤️export/🧵️serializers/🦀️component.rs` (docx pattern; this direct-mount file didn't exist yet — same "docx gap" the ticket flagged).
- `decode_xlsx`/`sniff_xlsx_bytes` + `*_from_xml` mapping → **new** `🚪️io/📥️import/🧩️deserializers/🦀️component.rs`.
- `empty_xlsx_snapshot`/`demo_xlsx_snapshot` + the whole `#[cfg(test)] mod tests` (unit tests + `conformance_laws`) → `🧬️schema/🦀️component.rs`.
- `XlsxEngine` struct (zero construction sites, confirmed via repo-wide grep) → **deleted outright**.
- `register()`/`register_artifact_inferences()`/`register_pilot_languages()` (zero real call sites — only `declaration()`'s own doc-comment mentions) → **deleted outright**.

### 🎞️pptx (ecma-376/✳️any) — had a shim; real dissolution done this wave
Identical shape to xlsx: `PptxError` + constants + the 3 synthesized-boilerplate XML string
constants (`MINIMAL_SLIDE_MASTER_XML`/`MINIMAL_SLIDE_LAYOUT_XML`/`MINIMAL_THEME_XML`) +
`resolve_office_document_relationship`/`attr`/`attr_val`/`find_child`/`element_children`/
`node_to_text`/`node_from_text` → `🚪️io/🦀️component.rs` (shared by both codec halves).
`build_minimal_pptx`/`encode_pptx` + shape/slide/presentation XML *writers* → new
`🚪️io/📤️export/🧵️serializers/🦀️component.rs`. `decode_pptx`/`sniff_pptx_bytes` + shape/slide/
presentation XML *readers* → new `🚪️io/📥️import/🧩️deserializers/🦀️component.rs`.
`empty_pptx_snapshot`/`demo_pptx_snapshot` + full test module → `🧬️schema/🦀️component.rs`.
`PptxEngine` (zero construction sites) + orphaned `register()`/`register_artifact_inferences()`/
`register_pilot_languages()` (zero real callers) → deleted outright.

### 📝️md (commonmark/✳️any) — heaviest, ~1600 LOC, no shim
No OPC/binary container here — the "codec" IS the CommonMark text codec itself.
`parse_markdown_blocks` + the full block/inline **parser** (`BlockLineClassifiers`,
`BlockParser`, `InlineParser`) → `🚪️io/📥️import/🧩️deserializers/🦀️component.rs`.
`render_markdown_blocks` + the full block/inline **renderer** (`BlockRenderer`,
`InlineRenderer`) → `🚪️io/📤️export/🧵️serializers/🦀️component.rs`. `io_registry` → `🚪️io/🦀️component.rs`.
`empty_md_snapshot`/`demo_md_snapshot` + full test module (mutation/diff/absorb/between/
codec-retention/field-sweep laws + parser unit tests + conformance laws) → `🧬️schema/🦀️component.rs`.
`MdEngine` (zero construction sites) + orphaned `register()`/`register_artifact_schema()`/
`register_artifact_inferences()`/`register_pilot_languages()` (zero real callers — md is **not**
one of stdio's 10 protected plugin-root calls, verified by grepping the actual non-comment
`engine::register()` call sites in `🗄️stdio/🦀️component.rs`: only binary/txt/ifc/gif/bmp/semio/
wav/html appear) → deleted outright.

### 📰xml (1.0/✳️any) — smallest, no shim
No dedicated codec to move: the real XML text codec (`xml_document_from_text`/
`xml_document_to_text`) already lived in `🧬️schema/📸️snapshot`, untouched. Only
`empty_xml_snapshot`/`demo_xml_snapshot` + the full test module (mutation/diff/absorb/between/
codec-retention/field-sweep laws + conformance laws) → `🧬️schema/🦀️component.rs`. `io_registry` →
`🚪️io/🦀️component.rs`. `XmlEngine` (zero construction sites) + orphaned `register()`/
`register_artifact_schema()`/`register_artifact_inferences()`/`register_pilot_languages()` (zero
real callers) → deleted outright. Glue.rs nesting confirmed **inside** `subsets::any` (matching
csv's shape, per the ticket's own warning — different from md/json's sibling-to-`subsets` shape).

## Consumer call sites repointed (all internal to stdio + 2 external plugins)

- `xlsx`/`pptx`: each artifact's own top-level `component.rs` `io_registry` module +
  `declaration()`'s `.composers(...)`; the `strict`/`transitional` subset `schema`/`io` files;
  the `mutations`/`diff`/`snapshot` schema files' `build_minimal_*`/`encode_*`/`decode_*` calls;
  the zip/xml cross-format bridge stub files under each artifact's own
  `io/{export/serializers,import/deserializers}/artifacts/{zip,xml}/...`.
- `md`: same shape for the top-level `component.rs` (2 sites: `.composers(...)` and the root
  `io_registry` module) + the `snapshot`/schema `looks_like_markdown` sniff helper + the md→txt
  bridge stub files. **External** (outside stdio): 6 real call sites across
  `🔱️trinity/🔌️jack`, `🔱️trinity/♻️rewrite`, and `📜️imperative/📜️imperative` (each artifact's own
  md import/export bridge component, referencing `md::engine::render_markdown_blocks`/
  `parse_markdown_blocks`) — repointed to
  `md::standards::v_commonmark::subsets::any::io::{export::serializers,import::deserializers}::*`.
  No consumer used the old flattened `MdSnapshot.body` shape — every one already used
  `schema::snapshot::{MdBlock, MdInline}`, so no lossy bridge was needed.
- `xml`: top-level `component.rs` had 2 real call sites (the `.composers(...)` line AND a
  second `io_registry as v1_0` import inside its own root `io_registry` module — the second one
  was easy to miss and *was* initially missed, caught by the compiler on the second pass).

Repo-wide `semio_s_plugin_stdio::artifacts::{xlsx,pptx,xml}::` external-consumer grep found only
`XlsxSnapshot`/`PptxSnapshot`/`XmlSnapshot`/`STDIO_*_DOCUMENT_SCHEMA` re-export usage (all
unaffected — these are still re-exported from each artifact's own top-level `component.rs`).

## Bare `io_registry` shadow hazard

Every relocated `.composers(...)`/`io_registry as …` reference uses its full
`standards::…::subsets::any::io::io_registry` path. Repo-wide bare-call grep (`[^:]io_registry::entries()`
excluding fully-qualified matches) returns 0 — only doc-comment prose mentions the bare form.

## Assertion arithmetic

Every test function's body was moved **verbatim** (only `use` paths were updated to the new
locations) for xlsx, pptx, md, and xml — same count of `#[test]` fns and same count of `assert*!`
calls per function, before and after. One deliberate exception: xlsx's
`shared_strings_are_carried_verbatim_never_resolved_or_deduped` originally called the (now
module-private to `📥️import/🧩️deserializers`) `shared_strings_from_xml` directly to assert the raw
parsed strings; since that helper is no longer reachable from `🧬️schema`, the same three value
assertions are now made after a full `encode_xlsx`/`decode_xlsx` round trip instead — the assertions
survive, just reached through the public codec surface rather than the private parser, documented
inline in the test.

## Compiler output

`RUSTC_WRAPPER="" CARGO_TARGET_DIR=target/stdio_doc2 cargo check -p semio-s-plugin-stdio --all-targets`
run repeatedly through the wave (once per artifact + one final combined run). **Zero** errors
reference `📕️xlsx`, `🎞️pptx`, `📝️md`, or `📰xml` in any run. The final combined run shows 24–25
`error[...]` blocks, **all** in `🏗️ifc`, `🗜️deflate`, `🎒️zip`, `💾️binary`, `🎨️svg`, `🎞️gif`,
`📄️pdf` — every one confirmed via `git status --short` to be an **uncommitted, in-progress
concurrent-session edit** in a directory I never touched (several show `D` for a deleted
`⚙️engine` file mid-dissolution by another session, matching this repo's documented concurrent-
workspace-churn pattern). One is corroborating: docx's own already-dissolved
`grammar_conformance_law` test (`zip::engine::decode_zip`) shows the identical zip-cascade break,
proving the breakage predates and is independent of this packet's changes.

`0` dangling `#[path = "..."]` mounts in `glue.rs` (verified via the ticket's own Python check).

## Deviations from the ticket's suggested destinations

- xlsx/pptx needed the same "missing direct component mount" gap docx needed fixing — the
  `🚪️io/📤️export/🧵️serializers/🦀️component.rs` and `🚪️io/📥️import/🧩️deserializers/🦀️component.rs`
  *direct* files didn't exist (only the nested `artifacts/{zip,xml}/.../component.rs` cross-format
  bridges did); both were created and mounted in `glue.rs`, mirroring docx exactly.
- Minor new `warning: unnecessary qualification` / a couple of `unused import` warnings were
  introduced by the mechanical full-path rewrite in `mutations`/`diff` schema files (a shorter
  alias was already `use`d in a few of those files) and one genuinely-unused import in xlsx's new
  deserializers file (fixed: `SHARED_STRINGS_PART`/`WORKBOOK_PART` removed from that file's
  `use` list). The rest were left as-is — cosmetic, pre-existing-pattern (`ArtifactAnalyzer`
  unused-import and "hidden lifetime parameters" warnings already existed repo-wide, confirmed
  present in xlsx *before* any edits in this wave's very first `cargo check`).

## Files touched (created, edited, deleted)

**Created:**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`

**Edited (io/schema/root component.rs for each artifact, plus consumer call sites):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️component.rs`, `.../🚪️io/🦀️component.rs`, `.../🧬️schema/🦀️component.rs`, `.../🧬️schema/🔺️diff/🦀️component.rs`, `.../🧬️schema/🧬️mutations/🦀️component.rs`, `.../🧬️schema/📸️snapshot/🦀️component.rs`, `.../✳️strict/🧬️schema/🦀️component.rs`, `.../✳️strict/🚪️io/🦀️component.rs`, `.../✳️transitional/🧬️schema/🦀️component.rs`, `.../✳️transitional/🚪️io/🦀️component.rs`, and the zip/xml bridge stubs under `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/{🎒️zip,📰xml}/...`
- Same shape for `🎞️pptx` and `📝️md` (top-level, io, schema, diff, mutations, snapshot, strict/transitional where present, txt bridge stubs for md).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🦀️component.rs`, `.../🏅️standards/🔖️1.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, `.../🧬️schema/🦀️component.rs`.
- External: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/.../📝️md/...` (import + export), `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/.../📝️md/...` (import + export), `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/.../📝️md/...` (import + export).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — removed all 4 `⚙️engine` mounts + the xlsx/pptx/md/xml `engine` shims, added the new xlsx/pptx/md direct-serializer/deserializer mounts.

**Deleted:**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/⚙️engine/` (whole dir)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/⚙️engine/` (whole dir)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/⚙️engine/` (whole dir)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/⚙️engine/` (whole dir)

## Unverified

None — the final combined `cargo check -p semio-s-plugin-stdio --all-targets` completed (not a
lock-contention timeout) and its output was fully inspected; every remaining error was attributed
to a concurrent session via `git status`.
