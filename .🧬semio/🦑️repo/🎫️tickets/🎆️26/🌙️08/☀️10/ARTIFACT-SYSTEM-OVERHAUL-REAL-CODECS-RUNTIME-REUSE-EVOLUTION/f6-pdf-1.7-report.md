# F6 — `📄️pdf` standard `1.7` — OpText/OpBinary (Mutation) + DiffCodec (Diff)

**Scope**: exactly `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/**` (subset `✳️any/🧬️schema`) +
this report. 1.4 was NOT touched (a sibling F6a agent already handled it, see
`f6-pdf-1.4-report.md`). No shared files (`glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema`
framework crates, `🏪️store`) were edited. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` was not touched.

## Step 1 — classification, verified for real (not trusted from the recon sweep)

The recon report's own row 25 guessed **HAND-ROLL (3a+3b)** for pdf 1.7 ("`PdfValue` object-graph
enum … 2 enums declared directly in the diff file"). I did not trust this — I added the derive
attributes for real and ran `cargo check -p semio-s-plugin-stdio --lib`:

- `#[derive(dsl::DslDiff)]` added to `PdfDiff` → **compile error, confirmed HAND-ROLL for the Diff
  side**: `the trait bound v1_7::...::PdfObject: DslField is not satisfied` (blocker 3a — `PdfObject`
  is a genuine data-carrying enum, reachable via `PdfValueDiff::Replace`, `PdfDictAdded::item`,
  `PdfObjectAdded::value`, `PdfArrayAdded::item`) plus `PdfPagesDiff`/`PdfObjectsDiff`/`PdfDictDiff`:
  `DslField` not satisfied (these nested diff-shaped structs would each need their own derive
  cascade even setting the enum problem aside). `PdfValueDiff::Stream::raw_filter:
  Option<Option<String>>` is an independent second blocker (3b), not reached first only because the
  compiler stops at the first unsatisfied bound in that field's chain.
- `#[derive(dsl::DslOps)]` added to `PdfMutation` → **compile error, confirmed HAND-ROLL for the
  Mutation side too**: `the trait bound v1_7::...::PdfObject: DslField is not satisfied` (every
  variant that carries a raw value — `SetSnapshot`'s whole `PdfSnapshot`, `InsertObject`/
  `SetObjectValue`/`SetDictEntry`/`SetTrailerEntry`'s `value: PdfObject` — reaches `PdfObject`
  directly) **and** `the trait bound v1_7::...::PdfPathSegment: DslField is not satisfied`
  (`SetDictEntry`/`RemoveDictEntry`'s own `path: Vec<PdfPathSegment>` arg is itself a
  data-carrying enum — a new finding beyond what the recon table's diff-file-only grep could see,
  since `PdfPathSegment` lives in the diff module but is only reached via the Mutation side).

Full captured `cargo check` output for both probes: `f6-pdf-1.7-derive-check1.txt` (this folder).
Both derive attributes were removed immediately after capturing the errors — the committed code
never carries the failed derive attempts, only doc-comment citations of the real error text.

**Verdict: HAND-ROLL on both sides**, matching the recon table's guess, but verified independently
this time (including the Mutation-side `PdfPathSegment` finding the table's diff-only sweep could
not have caught).

## Step 2b — hand-rolled implementation

Followed §5's grammar template from `f6-recon-report.md`, using `SvgDiff`'s real hand-rolled
`DiffCodec` (`✏️s/…/🎨️svg/…/🔺️diff/🦀️component.rs`, region `🔖️HandcraftedDiffCodec`) as the literal
structural template for the recursive-enum-object-graph case, since pdf's `PdfObject`/`PdfValueDiff`
recursion is structurally the same shape as svg's `XmlNode`/`SvgNodeDiff` (a real tagged enum with a
recursive `Array`/`Dict` container case), just with more variants (10 vs svg's 5) and one extra
tri-state field (`Stream::raw_filter`).

### Diff side (`🔺️diff/🦀️component.rs`, new region `🔖️HandcraftedDiffCodec`)

Primitives (`hex_encode`/`hex_decode`/`enc_str`/`dec_str`/`split_top_level`/`strip_brackets`/
`encode_option`/`decode_option`) are the exact same functions as `SvgDiff`'s and `GifDiff`'s copies
— re-derived locally per the "no shared hand-roll helpers module yet" note in §5, now the 4th
artifact to duplicate them (binary doesn't count, it's pure-derive; gif89a, svg, and now pdf are the
3 hand-roll duplicators the recon report already flagged as the trigger point for extracting a
shared module — still not done here, out of this ticket's per-artifact ownership scope). Added
`enc_box`/`dec_box` for `[f64;4]` (media/crop box) — new relative to svg/gif's primitive set since
neither of those artifacts has a fixed-size float array field.

Object-graph codecs (`enc_pdf_object`/`dec_pdf_object`, recursive over `PdfObject`): single-uppercase
tag prefix per variant — `Z`=Null (bare, no brackets, the only tag with no payload), `B`=Bool,
`I`=Int, `R`=Real, `S`=Str (hex), `N`=Name (hex), `A`=Array (recursive), `D`=Dict (recursive via
`enc_dict_entry`), `F`=Ref (`[num,gen]`), `T`=Stream (`[[dict entries],hexdata,filter?]`).

Diff-value codecs (`enc_value_diff`/`dec_value_diff`, recursive over `PdfValueDiff`): mirrors the
object tag vocabulary plus `L`=Replace (whole-node, the node-KIND-changed fallback). `Stream`'s own
3 independently-optional fields (`dict`/`data`/`raw_filter`) print as their own sparse `D:`/`A:`/`F:`
tag:value pairs inside `T[...]` — same shape gif 89a's `GifFrameDiff` sparse-struct codec uses;
`raw_filter: Option<Option<String>>` is the one genuine tri-state here, handled with exactly ONE
level of `encode_option` applied to the inner `Option<String>` once the outer `Some` (= "field
touched") is established by the `F:` tag's presence — same convention `GifFrameDiff`'s
`transparent_index`/`GifDiff`'s `gct`/`loop_count` tri-states use.

Collection-triple codecs: `enc_pages_diff` (index-keyed, `modified` carries the sparse
`PdfPageDiff` — `M`/`C`/`R`/`X` tag:value pairs, `C`=crop_box is the OTHER tri-state field in this
artifact, same one-level-`encode_option` convention), `enc_dict_diff` (name-keyed, reused verbatim
for `Dict`/`Stream.dict`/top-level `trailer` per the recipe's own "trailer is Dict-shaped" rule —
this reuse was already baked into the Rust *type* shape by the prior wave, the diff codec just
follows it), `enc_array_diff` (index-keyed, nested inside `PdfValueDiff::Array` only), `enc_objects_diff`
(`ObjRef`-keyed, `(num,gen)` pair used whole as the key per the prior wave's own design note).

Top level (`print_pdf_diff`/`parse_pdf_diff`): space-separated `name=value` tokens, one per changed
top-level `PdfDiff` field (`declaredVersion`/`info`/`pages`/`objects`/`trailer`) — absent token =
unchanged, matching `PdfDiff`'s own plain (non-tri-state) `Option<T>` shape at that level.
`encode_diff`/`decode_diff` = the text bytes verbatim, same simplification `SvgDiff`/`GifDiff`/
`WriterDiff` all use.

Real captured `print_diff` output (from `diff_codec_text_binary_roundtrip_law`, obtained by
temporarily adding an `eprintln!` to the test and running `cargo test ... -- --nocapture`, then
reverting the debug line — not fabricated), `PdfDiff::between(&b, &a)`, exercising the recursive
`PdfValueDiff::Array`/`Dict`/`Stream` variants, `Stream.raw_filter`'s tri-state (`Some → None`), the
`pages`/`objects`/`trailer` triples with real `removed`/`modified`/`added` entries, and
`PdfPageDiff`'s sparse `M`/`R`/`X` tags:
```
declared-version=312e37 info=[[1,42617365],[0],[0],[0],[0],[0]]
pages=[];[0:[M:[0,0,100,100],R:0,X:6f6e65]];[1:[[0,0,50,50],[1,[1,1,2,2]],0,74776f]]
objects=[[4,0]];[[1,0]:D[[4e6577];[436f756e74:I[3]];[]],[2,0]:T[A:010203,F:[1,466c6174654465636f6465]]];[2:[3,0]:A[I[1],R[2.5],F[[1,0]]]]
trailer=[50726576];[53697a65:I[3]];[]
```

### Mutation side (`🧬️mutations/🦀️component.rs`, region `OpCodecs`)

Reuses the diff module's `pub(crate)` primitives and object-graph codecs directly (`hex_encode`/
`enc_str`/`enc_pdf_object`/`enc_pdf_page`/`enc_pdf_info`/`enc_box`/`enc_objref`/`enc_dict_entry`/
`split_top_level`/`strip_brackets`/`encode_option`/`decode_option` and their `dec_*` counterparts)
— same intra-artifact reuse pattern `SvgMutation` uses over `SvgDiff`'s primitives, not a new shared
module. Added locally (mutation-specific, not needed by the diff side): `enc_path_segment`/
`dec_path_segment` + `enc_path`/`dec_path` for `PdfPathSegment` (`I[index]`=ArrayIndex,
`K[hex]`=DictKey — the same tag-prefix convention, new letters since `PdfPathSegment` is its own
small enum), `enc_indirect_object`/`dec_indirect_object` (`[objref,value]`, needed for
`SetSnapshot`'s full `objects: Vec<PdfIndirectObject>`), `enc_pdf_snapshot`/`dec_pdf_snapshot`
(`SetSnapshot`'s whole-payload codec, positional `[schema,declaredVersion,[pages],info,[objects],
[trailer]]`).

Grammar: `keyword arg=value ...` (space-separated), one match arm per `PdfMutation` variant — same
shape the derive's own handcrafted-wrapper convention and every other hand-rolled `OpText` in this
program (svg, gif87a/89a via `DslOps`, zip) uses. `encode_op`/`decode_op` = the text bytes verbatim.

Real captured `print_op` output (from `op_text_binary_roundtrip_law`):
- `PdfMutation::NoMutation` → `"no-mutation"`
- `PdfMutation::RemoveTrailerEntry { key: "Size" }` → `"remove-trailer-entry key=53697a65"`
- `PdfMutation::SetDictEntry { id: (1,0), path: [DictKey("Kids"), ArrayIndex(2)], key: "Rotate",
  value: Int(90) }` → `"set-dict-entry id=[1,0] path=[K[4b696473],I[2]] key=526f74617465
  value=I[90]"`

## Step 3 — tests (mandatory, both paths, added)

- `diff_codec_text_binary_roundtrip_law` (new `mod handcrafted_diff_codec_tests` in the diff file,
  same structural placement `SvgDiff`/`GifDiff` use): 4 cases (`PdfDiff::default()`,
  `between(a,b)`, `between(b,a)`, `between(a,a)`) over two snapshots that together exercise every
  `PdfObject` variant (`Null`/`Bool`/`Int`/`Real`/`Str`/`Name`/`Array`/`Dict`/`Ref`/`Stream`), the
  recursive `PdfValueDiff::Array`/`Dict`/`Stream` diff shapes, `Stream.raw_filter`'s tri-state
  (`Some(FlateDecode)` → `None`), `PdfPageDiff.crop_box`'s tri-state, and all 3 collection triples
  (`pages`/`objects`/`trailer`) with real `removed`/`modified`/`added` entries. Asserts
  `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x` for every case.
- `op_text_binary_roundtrip_law` (added inside the existing `mod tests` in the mutations file): 20
  cases covering every `PdfMutation` variant, including `SetSnapshot`'s full object-graph payload,
  `SetPageCropBox`'s `Some`/`None` `Option<[f64;4]>` arg, `InsertObject`/`SetObjectValue` with
  `Array`/`Stream`/`Null` payloads, and `SetDictEntry`/`RemoveDictEntry` with both `PdfPathSegment`
  kinds (including a real nested `[DictKey, ArrayIndex]` path). Same 3 assertions per case.

## Step 4 — verification (real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslDiff`/`dsl::DslOps` probes added | 53 errors, confirms HAND-ROLL both sides (`f6-pdf-1.7-derive-check1.txt`) |
| `cargo check -p semio-s-plugin-stdio --lib` after hand-rolled `DiffCodec` added (probes reverted) | 0 errors (`f6-pdf-1.7-handcraft-check1.txt`) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf::standards::v1_7::subsets::any::schema::diff"` | 19/19 passed (incl. new `diff_codec_text_binary_roundtrip_law`) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf::standards::v1_7"` (whole 1.7 standard, all subsets/engine/examples-under-1.7) | 105/105 passed (incl. both new law tests) |
| `cargo test -p semio-s-plugin-stdio --lib "bachelor_thesis"` (the mandated real ~6.3MB fixture) | 6/6 passed, unaffected |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate, final) | **1061 passed, 0 failed** (`f6-pdf-1.7-full-crate-test-final.txt`) |

One transient note: an interim whole-crate run hit 2 compile errors in `🧊️gltf`'s mutations file
(`no method named print_op` / `no variant parse_op`, missing an `OpText` import) — confirmed to be
an unrelated, concurrently-active sibling F6 session's in-progress edit (`git status` showed that
file as `MM` and freshly modified seconds earlier; the "Concurrent Cargo Workspace Churn" pattern).
Not my file, not touched by me. A retry of the same whole-crate command moments later compiled and
passed clean (1059/0, then 1061/0 on the final re-run after other concurrent sessions added more
tests of their own in the live shared tree), confirming it was transient churn, not a regression
from this change.

## Deviations from §5's grammar conventions (documented, as instructed)

- **`enc_box`/`dec_box` for `[f64;4]`**: not present in svg/gif's primitive set (neither artifact
  has a fixed-size float array field) — added locally, positional `[a,b,c,d]`, no deviation in
  *style* (still bracket-positional like every other struct), just a new primitive for a field
  shape this artifact happens to have (`media_box`/`crop_box`).
- **`Null` as a bare, bracket-less tag (`Z`)**: every other tagged variant in this program (svg's
  `XmlNode`/`SvgNodeDiff`, gif's disposal char) always pairs a tag with a bracketed payload. `Null`
  has no payload, so `dec_pdf_object` special-cases the literal string `"Z"` before the generic
  `tag/strip_brackets` path. Documented inline; does not break `split_top_level`'s bracket-depth
  tracking since `Z` (no brackets) never appears anywhere a comma could be adjacent without a
  bracket already balancing it (it's always either alone or immediately preceded/followed by `,`/`]`
  at whatever depth the recursive call already established).
- **`PdfObjectsDiff` keyed by whole `ObjRef` (not split to bare numeric id)**: this was already
  the prior wave's own Rust-level design decision (`PdfObjectsDiff.removed: Vec<ObjRef>`, not
  `Vec<u32>`), not something introduced in this pass — the diff codec just follows the existing
  type shape (`enc_objref` emits `[num,gen]`, used as the key in the `objects{}` triple exactly
  like `enc_pdf_object`'s `Ref` case does for the value form).
- No shared "hand-roll helpers" module was extracted, per explicit instruction in both
  `f6-recon-report.md` §5 and the assignment brief — flagged again here (3rd/4th hand-roll
  duplicating the same ~8 primitive functions: gif89a, svg, gif87a possibly, now pdf) as a good
  future closer-level extraction, not attempted in this per-artifact-scoped pass.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — new region `🔖️HandcraftedDiffCodec` (primitives, object-graph codecs, diff-value codecs,
  top-level `print_pdf_diff`/`parse_pdf_diff`, `impl protocol::DiffCodec for PdfDiff`) + new
  `mod handcrafted_diff_codec_tests` with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — replaced the `serde_json`-based `OpText`/`OpBinary` stub with hand-rolled impls (region
  `OpCodecs`), reusing the diff module's `pub(crate)` primitives; added `op_text_binary_roundtrip_law`
  to the existing `mod tests`; removed a now-redundant duplicate `PdfIndirectObject` import.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-pdf-1.7-derive-check1.txt` (the real
  derive-probe compile errors), `f6-pdf-1.7-handcraft-check1.txt` (clean check after hand-roll),
  `f6-pdf-1.7-full-crate-test-final.txt` (final whole-crate 1059/0 run).

**No shared files touched**: `glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates,
`🏪️store` were all read-only this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` untouched (0 stdio
entries before and after — correct, matches the recon report's explicit "do not allowlist as a
shortcut" instruction). Only 1.7's files were touched; 1.4 (a sibling F6a agent's scope) was not
opened for writing.
