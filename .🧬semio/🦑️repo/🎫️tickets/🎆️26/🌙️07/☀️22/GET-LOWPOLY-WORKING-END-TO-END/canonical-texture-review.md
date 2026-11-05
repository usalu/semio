# Canonical Texture Handoff Review

Independent read-only source review plus fresh strict-schema asset verification, 2026-11-05. No Native, server, production edits, or compile/pass claims.

## Confirmed Corrections

Owning subset prefix `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any`.

New texture helper enforces storage0or262144bytes, compute exactly262144. Materialized pixels validates before allocating/cloning; compaction only folds a full-size all-white buffer. Compositing validates every layer, including invisible layers, before allocating output; exact4byte chunks no longer pad missing channels or silently crop a differing validated length. Brush/fill/sample check full compute extent. Stroke no longer infers square size.

Engine ensure_all_paint_buffers validates all existing objects/layers before any insertion/materialization; it expands only empty. ensure_paint_layer validates target layers before insertion and rejects invalid requested index without inserting. Retained snapshot admission now calls per-layer validation. These reviewed refusal paths preserve state on invalid extent.

Handcrafted ArtifactPack encode/decode validates snapshot. Text DSL parse validates after record construction; SQLite write validates before rows and reconstructed snapshot validates before return. Validation errors propagate through their typed fallible interfaces. This establishes these entry points, not every generated raw binding.

## Remaining Untrusted Boundary Holes

1. **Raw canonical JSON / generated FromValue remains permissive.** `🚪️io/📝️text/🦀️.rs` lowpoly_json_bind recursively turns base64 strings into intrinsic Bytes then calls generic FromValue::from_value, with no semantic texture validation. LowpolyPaintLayer still derives FromValue using the ordinary bytes adapter. Therefore lowpoly_json_decode::<LowpolySnapshot> and nested typed layer/mutation binding do not themselves enforce extent. The JSONcommand handler was concurrently updated by Verify to call parsed.validate_paint_pixels before LoadDocument; that protects this action path but does not seal the reusable raw JSON API/serde fixture binding. Runtime was notified.

2. **Object insertion bypasses layer-edit validation.** CreateObject diff only checks duplicate ID then constructs diff_objects_add from the supplied object. LowpolyDiff::apply clones/commits object delta and assigns next without validating next's paint layers. Existing paint_edits_after now validates original layers and Insert/Replace layer payloads before applying a candidate, but an inserted whole object can avoid that route. Validate the completed candidate before assignment and author public malformed-object atomic refusal regression. Runtime was notified; no source change from this audit.

3. **Published physical JSON schema remains broader than canonical semantic schema.** Main `🧬️schema/🔣️.json` pixels is still type:string/contentEncoding:base64 without empty/full decoded extent constraints. New `🎨️texture/🔣️.json` independently describes intrinsic octets, and TS parseLowpolyPixels invokes its semantic validator. Do not directly replace physical base64 string schema with an array reference; publish an explicit physical encoding constraint/semantic check consistent with decoded canonical byte length. Ajv contentEncoding annotation alone does not validate this condition.

ArtifactDsl print remains infallible and does not validate malformed in-memory objects; valid-document callers must enforce the invariant before publication. This is an API limitation, not a claim malformed text can pass the now-checked parser. Generated/reference controlled native codec paths were not exhaustively traced in this review; avoid claiming all Pack APIs are sealed solely because handcrafted decode_pack_with is checked.

## Fresh Fixture Independence Check

Executed registered light Node with independent strict Ajv and Buffer decode against the actual texture schema. Fresh exact audited84files contain98layer records:79empty,19full262144; zero invalid. Rechecked every79white path from canonical-white-fixtures.md: all still explicit empty strings. No field was rewritten in this review; mixed-file Runtime edits preserved white intent.

The19nonwhite values are fresh full canvases rather than keeping small invalid fixtures to evade admission. Runtime reports NumPy coordinate-grid/sparse-diff verification of changed brush/eraser cases; that is separate peer execution evidence, not this review's geometry test. My independent run verifies canonical lengths and white preservation, not recomputation of every changed pixel/diff expectation.

## Localization And Execution Limits

Paint commands use dedicated lowpoly.paint.invalid-texture FaultCode and typed errors include actual extent. I requested the exact bilingual notice owner from Runtime because the focused declaration search had not yet established its registration. A string fault code alone is not proof of localized notice coverage; keep that item pending until the named owner/tests are reviewed.

No Rust body compiled or executed here. Current public snapshot JSON metadata/budget gap is separately tracked by snapshot-action-arguments.md and snapshot-import-budget.md; canonical fixture validity does not make those actions usable.


## Same-Review Corrections Rechecked

Runtime responded and the latest source was independently re-read. The first two boundary issues above are now closed **at source level**:

- LowpolyPaintLayer's existing FromValue derive now declares validate=crate::paint_texture::validate_layer and validate_controlled=crate::paint_texture::validate_layer_controlled. Cold hook delegates exact length validation; controlled hook checkpoints then delegates. This attaches semantic validation to nested layer binding rather than only the JSON command. Raw JSON and nested mutation payloads therefore inherit the owning validation hook when decoded through that generated trait. No custom adapter was introduced.
- LowpolyDiff::apply now validates original snapshot before cloning, and completed candidate before assigning *unpublished. A malformed inserted object cannot commit through that diff boundary; previous state remains untouched on refusal.

Exact bilingual notice owner is now confirmed: `✏️editor/🕹️play-app/🦀️.rs:93–95`, ArtifactEditor::fault_notices, has three native English/German rows. lowpoly.paint.invalid-texture explains empty or exact256×256 RGBA; snapshot capacity/invalid errors have separate notices. The earlier localization pending note is resolved for declaration presence, not executed localization tests.

The main physical JSON schema still has a broader base64 field annotation than the separate canonical semantic contract; treat that as declaration alignment follow-up rather than a remaining demonstrated runtime acceptance hole after the new FromValue hooks. No compile/Native inference is made from these source rechecks.


## Physical Wire Schema Follow-Up

Runtime subsequently aligned main JSON schema pixels with empty or exact349528character canonical base64 ending in valid1byte padding. Independently executed registered light Node + strict Ajv against the actual field schema:0/262144decoded byte cases pass;262140,262148,3,8,256,4194304 refuse. All eight matched the neutral extent contract, exit0. Exact pattern [A-Za-z0-9+/]{349525}[AQgw]== constrains valid final significant bits, not just string length. This resolves the earlier physical schema alignment follow-up at source/oracle level. No Native claims.
