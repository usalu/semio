# FG-wave: `stdio.docx` (ECMA-376 WordprocessingML) — OPC pattern-setter

Wave scope: real grammar/protocol dialect files for docx's snapshot/diff/mutations facets, real
binary frames for `DiffCodec`/`OpBinary` (upgraded from the F6 text-as-binary shortcut), 5-role
`LanguageSpec` registration, the 6 per-artifact conformance-law tests, and genuine
`.dsl.semio`/`.pack.semio` demo fixtures. Docx is this wave's OPC container/contained-XML-parts
pattern-setter for the sibling xlsx/pptx/bcf/ifc-2x3 agents.

## Files touched

- `⚙️engine/🦀️component.rs` — added `demo_docx_snapshot()`, `register_pilot_languages()` (wired
  into `register()`), and the `conformance_laws` test module (6 laws).
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten from the
  F6 `*OCTET` placeholder to a real grammar modeling the contained XML parts
  (`[Content_Types].xml`, `_rels/.rels`, `word/document.xml`, `word/styles.xml`), restated from
  xml's own FG1 grammar shape per the ticket brief's explicit OPC classification.
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten to the
  real ZIP/OPC container layout (`repeat`/`backward`/`jump`), restated verbatim (only
  `protocol`/`schema` ids differ) from zip 2.0's own real protocol file per the ticket's OPC
  precedent.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_diff`/`parse_diff` shape traced from `🔺️diff/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten to the real
  binary-frame shape (`header fixed 2 { format u8, flags u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added a full `BinaryCodecs` region (real
  LEB128-varint length-prefixed primitives, recursive `XmlNode`/`DocxBlock`/`DocxBlockDiff`/
  `DocxStyle`/`OpcPart`/`OpcRelationship` binary sub-encoders, generic indexed/named-triple binary
  codecs); upgraded `DiffCodec::encode_diff`/`decode_diff` from `print_diff().into_bytes()` to the
  real frame; promoted `snapshot_a`/`snapshot_b`/`xml_node` to module scope and added
  `demo_diff_cases()`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_op`/`parse_op` shape traced from `🧬️mutations/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten to the
  real binary-frame shape (`header fixed 2 { format u8, tag u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `OpBinaryCodec` region (real
  path/snapshot/package/document binary encoders reusing the diff file's `pub(crate)` primitives);
  upgraded `OpBinary::encode_op`/`decode_op` from the text shortcut to the real 13-tag binary
  frame; promoted `fixture`/`table_path`/`sweep_a`/`sweep_b` to module scope and added
  `demo_mutation_cases()` (renamed from the former test-only `sample_mutations()`).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — replaced the fake `"68656c6c6f"` placeholder
  (no preamble, not real docx bytes) with genuine `print_dsl(demo_docx_snapshot())` output.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new file, genuine
  `encode_pack(demo_docx_snapshot())` bytes (this fixture didn't exist before).

No `📡️example.spr.semio` fixture was added (optional per the checklist; not blocking).

## A genuine mid-wave discovery: unattributed self-closing-tag lexer fusion

While building `grammar_conformance_law` I hit a real, reproducible failure:
`recognizer.recognize()` returned `Ok(false)` for `<w:rPr><w:b/></w:rPr>` even though the identical
shape with an attribute (`<w:pStyle w:val="H"/>`) recognized fine. Bisected with throwaway mini
grammars (never guessed): the shared lexer's `is_ident_continue` (`🔍️lexer/🦀️component.rs`)
includes `/` alongside `_`/`-`/`.`. With **nothing** between an element name and its self-closing
slash, the two fuse into **one** `IDENT` token (`"b/"`, not `"b"` then `"/"`) — an attribute's
closing quote is what normally breaks the identifier run before `pStyle`'s own `/`. Fixed by
modeling `w:b`/`w:i` as one fused-literal token (`"b/"`/`"i/"`) in the grammar, documented inline.
This is a genuine, artifact-agnostic lexer property (any unattributed self-closing tag hits it),
not a docx-specific hack — worth the sibling xlsx/pptx/bcf/ifc agents' attention if their own
XML-part grammars ever model an unattributed empty element (e.g. a bare `<a:noFill/>`-shaped tag).

## Deviation from the literal checklist: `grammar_conformance_law`'s test strategy

Every existing pilot's `grammar_conformance_law` feeds real `print_dsl()` output straight to the
snapshot grammar's recognizer. Docx's own `ArtifactDsl::parse_dsl`/`print_dsl`
(`📸️snapshot/🦀️component.rs`) hex-dumps the **whole binary OPC package** — that hasn't changed,
and changing it was out of this wave's scope (not requested, and a much larger undertaking than
the grammar/protocol/diff/mutation deliverables actually asked for). That wire text is binary hex,
not XML syntax, so it cannot be fed through a grammar that (per the ticket brief's own explicit
instruction) models the contained XML parts' real syntax instead of hex-dumping.

Resolution: `grammar_conformance_law` decodes the REAL zip entries `encode_docx` genuinely produces
(via `zip::engine::decode_zip`, the same real codec `opc::decode_opc` itself delegates to) and
recognizes each of the four modeled parts' own real text against the grammar — direct proof the
grammar matches this artifact's own real per-part XML bytes, not an invented approximation, and
arguably a *stronger* proof than a print_dsl-recognize test would have been for this hybrid
(binary-container + text-contained-parts) artifact shape. `committed_facet_files_parse`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`, and
`fixture_honesty_law` all follow the pilot template unchanged.

## `mechanism_gaps` (nothing new beyond what the recipe already tracks, one new lexer note)

| gap id | area | note |
|---|---|---|
| `protocol-prim-ref-recursion` | `walk_protocol` | Hit by both `DocxDiff` (recursive `DocxBlockDiff` via `Table`) and `DocxMutation` (`SetSnapshot`'s whole `DocxSnapshot`, `InsertBlock`/`SetBlockContent`/`InsertStyle`'s bare payloads). Real fixed `format`/`flags`|`format`/`tag` header individually protocol-walked; the recursive payload is one opaque `chain ... bytes` tail per `📖️grammar-recipe.md` §2.5 — the Rust `encode_diff`/`decode_op` side stays genuinely, fully structured (real varint-framed, recursively-typed binary encoders), round-trip tested independently (`diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law`, both green). |
| `repeat-cannot-embed-jump` | zip protocol (restated) | Inherited from zip's own file verbatim — the per-entry `local_off` backward-jump cross-validation stays un-modeled, same explicit plan latitude. |
| `register-schema-spec-needs-recordspec` | `dsl::registry` | `DocxSnapshot`/`DocxDiff`/`DocxMutation` are all hand-rolled (confirmed by this file's own F6 doc comments: `#[derive(dsl::Dsl*)]` fails to compile on every one of these types) — `register_schema_spec` deliberately not called, same as json/csv/zip/png. |
| `xml-empty-tag-slash-fusion` (new, this wave) | shared lexer, `is_ident_continue` | An unattributed self-closing tag (`<X/>`, no space, no attribute) lexes its name+slash as ONE fused `IDENT` token — `is_ident_continue` includes `/`. Not previously documented in the recipe. Workaround: model the fused form as one literal (`"b/"`), not two tokens. Affects any future XML-shaped grammar exercising a genuinely attribute-less empty element. |
| docx-specific OPC diff-type duplication | n/a | Explicitly out of scope per the ticket brief itself (Phase 1's own documented deferred cleanup — `DocxOpcDiff`/etc. duplicated per-artifact instead of hoisted to `zip::opc`); not touched, as instructed. |

## Verification

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::docx"   # 56 passed, 0 failed
cargo test -p semio-s-plugin-stdio --lib                     # 1812 passed, 0 failed, 1 ignored (pre-existing, unrelated)
bun run ./📜️script.ts policy                                 # no docx findings beyond pre-existing os-state-authority/item-scope-global hits in composer files this wave never touched; POLICY_GRAMMAR_PARSEABILITY/POLICY_PROTOCOL_PARSEABILITY/POLICY_FIXTURE_HONESTY/POLICY_LANGUAGE_REGISTRATION/POLICY_STDIO_JSON_TRANSFER_BAN do not currently appear as check categories in this script's output at all (confirmed by grep, not assumed) — manual grep for serde_json::{to_vec,from_slice,to_string,from_str,Value} inside this artifact's own files came back clean
```

Ownership boundary respected: only files under `📜️docx/**` were edited (plus reads of zip/xml's
public API and grammar/protocol files for restatement, never edits). `📦️glue.rs`, `📜️script.ts`,
the SDK traits, and the schema/dsl/protocol/registry modules were never touched.
