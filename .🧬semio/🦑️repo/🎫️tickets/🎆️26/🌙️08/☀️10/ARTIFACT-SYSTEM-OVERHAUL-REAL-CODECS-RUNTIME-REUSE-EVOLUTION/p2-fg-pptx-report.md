# FG-wave: `stdio.pptx` (ECMA-376 PresentationML) — OPC-family sibling, following docx's pattern

Wave scope: real grammar/protocol dialect files for pptx's snapshot/diff/mutations facets, real
binary frames for `DiffCodec`/`OpBinary` (upgraded from the F1-era `*OCTET`/text-as-binary
shortcuts), 5-role `LanguageSpec` registration, the 6 per-artifact conformance-law tests, and
genuine `.dsl.semio`/`.pack.semio` demo fixtures. Follows docx's own just-landed OPC
container/contained-XML-parts pattern (`p2-fg-docx-report.md`, this ticket folder) — pptx restates
zip's real ZIP/OPC container layout for its binary facets and models its own real PresentationML
XML vocabulary (`[Content_Types].xml`, `_rels/.rels`, `ppt/presentation.xml`,
`ppt/slides/slideN.xml`'s `p:spTree` shape tree) for its text facets.

## Files touched

- `⚙️engine/🦀️component.rs` — added `demo_pptx_snapshot()`, `register_pilot_languages()` (wired
  into `register()`), and the `conformance_laws` test module (6 laws).
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten from the
  F1 `dialect grammar stdio.pptx.snapshot` / `*OCTET` placeholder to a real grammar modeling the
  contained XML parts: `[Content_Types].xml`/`_rels/.rels` (generic OPC layer, restated verbatim
  from `🎒️zip/📦️opc/🦀️component.rs`, identical shape docx's own file already restates),
  `ppt/presentation.xml` (`p:sldMasterIdLst`/`p:sldIdLst`), and `ppt/slides/slideN.xml`'s real
  `p:spTree` shape tree (`p:sp` TextBox/Placeholder, `p:pic` Picture, generic `x-elem` raw fallback
  for `Other`/`p:graphicFrame`/`p:grpSp`) — every element/attribute traced directly from
  `⚙️engine/🦀️component.rs`'s `shape_to_xml`/`slide_to_xml`/`presentation_to_xml`, not invented.
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten to the
  real ZIP/OPC container layout (`repeat`/`backward`/`jump`), restated verbatim (only
  `protocol`/`schema` ids and doc-comment part list differ) from zip 2.0's own real protocol file,
  same restatement docx's own file already performs.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_pptx_diff`/`parse_pptx_diff` shape traced from `🔺️diff/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten to the real
  binary-frame shape (`header fixed 2 { format u8, flags u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added a full `BinaryCodecs` region (real
  LEB128-varint length-prefixed primitives, fixed 8-byte little-endian `PptxTransform` EMU fields,
  recursive `PptxShape`/`PptxShapeDiff`/`PptxSlide`/`OpcPart`/`OpcRelationship` binary sub-encoders,
  generic indexed/named-triple binary codecs); upgraded `DiffCodec::encode_diff`/`decode_diff` from
  `print_diff().into_bytes()` to the real frame; added `demo_snapshot_a`/`demo_snapshot_b`/
  `demo_diff_cases()` at module scope (non-test); also fixed a pre-existing region-marker
  double-`//#endregion 🔖️HandcraftedDiffCodec` imbalance while in the file.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_pptx_mutation`/`parse_pptx_mutation` shape traced from `🧬️mutations/🦀️component.rs` (all
  9 variants).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten to the
  real binary-frame shape (`header fixed 2 { format u8, tag u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `OpBinaryCodec` region (real
  snapshot/package/slide binary encoders reusing the diff file's `pub(crate)` primitives); upgraded
  `OpBinary::encode_op`/`decode_op` from the text shortcut to the real 9-tag binary frame; added
  `demo_fixture()`/`demo_mutation_cases()` at module scope (non-test).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — replaced the fake `"68656c6c6f"` placeholder
  (no preamble, not real pptx bytes) with genuine `print_dsl(demo_pptx_snapshot())` output.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new file, genuine
  `encode_pack(demo_pptx_snapshot())` bytes (this fixture didn't exist before).

No `📡️example.spr.semio` fixture was added (optional per the checklist; not blocking).

## A genuine bug found and fixed: `demo_pptx_snapshot()`'s double-regenerate part-order drift

Building the fixture-honesty round trip surfaced a real bug, independent of the grammar/protocol
work: `demo_pptx_snapshot()` originally called `build_minimal_pptx(presentation)` (which internally
calls `regenerate_presentation_parts` once) and then manually appended one extra unmodeled raw part
(`ppt/media/image1.png`) via `snap.opc.set_part(...)`. `regenerate_presentation_parts` retains-away
and re-appends every `ppt/slides/*` part AND `ppt/presentation.xml` on **every** call (the exact
fix F5's own `double_regenerate_keeps_opc_parts_order_stable` regression test already covers for
the *empty-package* case) — but a **second** regenerate pass (triggered by any later `encode_pptx`
call, e.g. from `print_dsl`/`encode_pack`/`grammar_conformance_law`) on a package that already has
an *extra* manually-appended part squeezed in after the first pass reorders `opc.parts` so the
slide/presentation parts land **after** that extra part instead of before it — flipping
`demo_pptx_snapshot()`'s own `Vec<OpcPart>` order relative to what a fresh
`print_dsl(demo)`-then-`parse_dsl` round trip actually produces. Caught by `fixture_honesty_law`'s
same-process self-check (`print_dsl`/`parse_dsl` mismatch on the very first attempt, before any
fixture file was even copied over — ruled out cross-process/build nondeterminism directly).
Fixed by round-tripping `demo_pptx_snapshot()` through one real `encode_pptx`/`decode_pptx` pass
before returning, canonicalizing the part order to the stable fixed point every *later*
`encode_pptx` call will already agree with. Documented inline at the fix site.

## Known, documented limitation: the generic `x-elem` raw-retention fallback vs. the fused-slash lexer property

Docx's own report (`p2-fg-docx-report.md`) already documents the shared lexer's real
`is_ident_continue`-includes-`/` property: an **unattributed** self-closing tag (`<name/>`, no
space, no attribute) fuses its final identifier segment with the slash into ONE token. Every one of
this artifact's own **typed** shape productions (`p:nvPr`/`p:grpSpPr`/`p:cNvGrpSpPr`/`p:cNvPicPr`/
`a:bodyPr`/`a:fillRect`/`a:avLst`/an empty `a:p`) models this precisely with an explicit fused
literal token (e.g. `"grpSpPr/"`), traced directly from `⚙️engine/🦀️component.rs`'s own
`attrs: vec![]` call sites — no ambiguity, since each is a distinct, exact literal string.

The **generic** `x-elem` raw-retention fallback (restated from xml's/docx's own `x-elem`
productions, used for `PptxShape::Other`/unrecognized `p:spTree` children) cannot be fixed the same
way: since `IDENT` matches by TOKEN CLASS not exact text, a bare `LT x-name GT` (the fused-slash
case, once the trailing `/` is absorbed into the name's own token) is token-sequence-**identical**
to the START of a genuine open tag (`x-open-tag = LT x-name x-attr* GT` with zero attrs) — the
grammar has no way to disambiguate "this is a complete self-closed element" from "this is an open
tag, real content follows" without unbounded lookahead. This is a genuine, deeper limitation of the
x-elem restatement than the typed-production case, not something this wave's scope covers fixing
(x-elem's own recursive-descent shape would need a structural redesign, out of a single FG-wave's
scope). Sidestepped honestly in this wave's own demo fixture by keeping every `x-elem`-modeled
raw-retained element attributed (`demo_pptx_snapshot()`'s `Other` shape's `p:graphicFrame` payload
deliberately omits the unattributed `<a:graphic/>` child a real one would carry, documented inline)
rather than silently exercising an unrecognized input. Filed below as a new `mechanism_gaps` entry.

## Ambient concurrent-session state observed, not touched

A test named `zzz_generate_p2p1_fixtures` (inside the `conformance_laws` module, between
`protocol_walk_law` and `fixture_honesty_law`) was found already present in
`⚙️engine/🦀️component.rs` partway through this wave — not authored by this session, referencing
the same `demo_pptx_snapshot()`/`mutations`/`diff` module shapes this wave independently built.
Per this ticket's own "live shared tree, other sessions active right now" guidance: classified by
content (an inert `#[ignore]`d test, zero effect on any of this wave's own 6 conformance laws or
existing tests, confirmed by the final green test run below) and left untouched rather than
chased or removed.

## `mechanism_gaps` (consolidated with the recipe's own table, two new entries)

| gap id | area | note |
|---|---|---|
| `protocol-prim-ref-recursion` | `walk_protocol` | Hit by both `PptxDiff` (`PptxShapeDiff::Replace` nesting back through the whole `PptxShape` tree, plus every `IndexedTripleDiff`/`NamedTripleDiff` instantiation) and `PptxMutation` (`SetSnapshot`'s whole `PptxSnapshot`, `InsertSlide`/`InsertShape`'s bare payloads). Real fixed `format`/`flags`\|`format`/`tag` header individually protocol-walked; the recursive payload is one opaque `chain ... bytes` tail per `📖️grammar-recipe.md` §2.5 — the Rust `encode_diff`/`encode_op` side stays genuinely, fully structured (real varint-framed, recursively-typed binary encoders), round-trip tested independently (`diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law`, both green). |
| `repeat-cannot-embed-jump` | zip protocol (restated) | Inherited from zip's own file verbatim — the per-entry `local_off` backward-jump cross-validation stays un-modeled, same explicit plan latitude docx's own file already documents. |
| `register-schema-spec-needs-recordspec` | `dsl::registry` | `PptxSnapshot`/`PptxDiff`/`PptxMutation` are all hand-rolled (confirmed by this file's own F6 doc comments: `#[derive(dsl::Dsl*)]` fails to compile on every one of these types, same three-reason citation docx's own diff file documents) — `register_schema_spec` deliberately not called, same as json/csv/zip/png/docx. |
| `xml-empty-tag-slash-fusion` | shared lexer, `is_ident_continue` | Already documented by docx's own report — reconfirmed here against pptx's own real fused sites (`p:nvPr`/`p:grpSpPr`/`p:cNvGrpSpPr`/`p:cNvPicPr`/`a:bodyPr`/`a:fillRect`/`a:avLst`/empty `a:p`), each modeled as one fused literal token. |
| `x-elem-fused-empty-tag-ambiguity` (new, this wave) | grammar dialect, generic raw-retention fallback | The GENERIC `x-elem` restatement (unlike a TYPED production's exact-literal fused token) cannot disambiguate a fused-slash bare self-close (`LT x-name GT`) from the start of a genuine open tag using only same-shape lookahead — both are the identical token sequence once the trailing `/` is absorbed into the IDENT token's own text (`IDENT` matches by class, not exact string). Deeper than the already-documented typed-production case; would need a structural redesign of `x-elem`'s own shape (e.g. a dedicated fused-terminal or bounded lookahead), out of this wave's scope. Workaround used here: keep this wave's own `x-elem`-modeled demo content attributed (documented inline at the fixture's own construction site) rather than exercising the gap silently. |
| pptx-specific OPC diff-type duplication | n/a | `PptxOpc*Diff` types are defined in pptx's own `🔺️diff/🦀️component.rs` (own-file copy), same as docx's `DocxOpc*Diff` — explicitly out of this wave's ownership boundary to hoist to `zip::opc` (flagged as `glue_followup` in F5's own report already; this wave adds no new instance of the duplication, just upgrades the existing types' codecs). |

## Verification

```
cargo check -p semio-s-plugin-stdio --lib          # clean, 0 errors (confirmed 3x across the wave)
cargo test -p semio-s-plugin-stdio --lib "artifacts::pptx"   # 58 passed, 0 failed, 1 ignored (the
                                                               # ambient concurrent-session test above)
cargo test -p semio-s-plugin-stdio --lib                     # 1844 passed, 0 failed, 3 ignored
bun run /Users/ueli/Documents/semio/script.ts policy          # zero pptx entries anywhere in the
                                                               # breach report (grepped, not assumed)
                                                               # -- manual grep for
                                                               # serde_json::{to_vec,from_slice,
                                                               # to_string,from_str,Value} inside
                                                               # this artifact's own schema files
                                                               # came back clean too
```

All 6 conformance laws pass: `committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`.

Ownership boundary respected: only files under `🎞️pptx/**` were edited (plus reads of zip/docx's
public API and grammar/protocol files for restatement, never edits). `📦️glue.rs`, `📜️script.ts`,
the SDK traits, and the schema/dsl/protocol/registry modules were never touched. Did not call
`ticket_open`/`ticket_close`/`ticket_reopen` per this wave's own instructions.
