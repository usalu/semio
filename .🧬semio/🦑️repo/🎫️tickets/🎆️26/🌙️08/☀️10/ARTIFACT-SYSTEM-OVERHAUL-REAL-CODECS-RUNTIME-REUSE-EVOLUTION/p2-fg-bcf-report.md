# FG-wave: `stdio.bcf` (BCF-XML 2.1, "bcfzip") — OPC-family agent, deliberately NOT OPC

Wave scope: real grammar/protocol dialect files for bcf's snapshot/diff/mutations facets, real
binary frames for `DiffCodec`/`OpBinary` (upgraded from the F6 text-as-binary shortcut), 5-role
`LanguageSpec` registration, the 6 per-artifact conformance-law tests, and genuine
`.dsl.semio`/`.pack.semio` demo fixtures.

## Deviation from the ticket brief: bcf is NOT an OPC package

The brief classifies the whole wave (docx/xlsx/pptx/bcf) as "OPC container (protocol) + contained
XML parts (grammar, following xml's own FG1 family)" and points at docx's just-landed files as the
pattern. Confirmed by direct read of THIS artifact's own real Rust codec (never assumed) — three
independent doc comments say the same thing outright:

- `⚙️engine/🦀️component.rs`'s own module doc: "`bcfzip` is NOT an OPC package (no
  content-types/relationships apparatus) so this artifact builds its own simple wrapper directly on
  `zip::ZipEntry` rather than reusing `zip::opc::OpcPackage`".
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s own module doc: "bcfzip container
  (BCF-XML 2.1): a plain flat zip of per-topic folders, NOT an OPC package".
- This ticket's own `p2-w0-recon-report.md` bcf row, which quotes the same lines and explicitly
  flags "Cannot reuse OPC-specific grammar built for docx/xlsx/pptx (no content-types/rels layer)".

`encode_bcf`/`decode_bcf` (`⚙️engine/🦀️component.rs:397-473`) call
`crate::artifacts::zip::engine::{encode_zip,decode_zip}` **directly** — never
`zip::opc::{encode_opc,decode_opc}`. So the grammar facet models bcf's own real, non-OPC XML parts
(`bcf.version`, `<guid>/markup.bcf`, `<guid>/<guid>.bcfv`) instead of
`[Content_Types].xml`/`_rels/.rels`/a namespaced document body. The **binary/protocol** facet still
correctly follows the ticket's OPC-family precedent: bcf's container is byte-identical real ZIP
2.0, one layer *below* OPC, so `../💾️binary/📡️component.protocol.semio` restates the SAME real
`repeat`/`backward`/`jump` layout zip's own file (and docx's own restatement of it) already
establish — only the container framing is shared with the OPC family, not the part vocabulary.

## Files touched

- `⚙️engine/🦀️component.rs` — added `demo_bcf_snapshot()`, `register_pilot_languages()` (wired into
  `register()`), and the `conformance_laws` test module (6 laws, nested inside the existing
  `#[cfg(test)] mod tests`).
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten from the F6
  `*OCTET` placeholder to a real grammar modeling bcf's own non-OPC XML parts (`bcf.version`,
  `markup.bcf`'s `Topic`/`Comment`/`Viewpoints`, `.bcfv`'s `VisualizationInfo`/`Components`/
  `PerspectiveCamera`/`OrthogonalCamera`), traced directly from `⚙️engine/🦀️component.rs`'s real
  parse/emit functions.
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten to the real
  ZIP container layout (`repeat`/`backward`/`jump`), restated verbatim (only `protocol`/`schema`
  ids differ) from zip 2.0's own real protocol file, per the OPC-family container precedent (which
  DOES apply here even though bcf itself isn't OPC — see deviation above).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_diff`/`parse_diff` shape traced from `🔺️diff/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten to the real
  binary-frame shape (`header fixed 2 { format u8, flags u8 } + chain payload bytes`; 3 flag bits
  for `version`/`topics`/`parts`).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added a full `BinaryCodecs` region (real
  LEB128-varint length-prefixed primitives, recursive `BcfTopic`/`BcfComment`/`BcfViewpoint`/
  `BcfCamera`/`BcfComponents`/`BcfRawPart` binary sub-encoders, a generic name-keyed-triple binary
  codec instantiated for `topics`/`comments`/`viewpoints`/`parts`); upgraded
  `DiffCodec::encode_diff`/`decode_diff` from `print_diff().into_bytes()` to the real frame; added
  `demo_snapshot_a`/`demo_snapshot_b`/`demo_diff_cases()`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_op`/`parse_op` shape traced from `🧬️mutations/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten to the
  real binary-frame shape (`header fixed 2 { format u8, tag u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added an `OpBinaryCodec` region (tri-state/
  list binary primitives, reusing the diff file's `pub(crate)` value encoders for the heavier
  types); upgraded `OpBinary::encode_op`/`decode_op` from the text shortcut to the real 14-tag
  binary frame; added `demo_mutation_cases()`.
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — replaced the fake `"68656c6c6f"` placeholder
  (no preamble, not real bcf bytes) with genuine `print_dsl(demo_bcf_snapshot())` output (real
  hex-dumped bcfzip bytes behind the `semio stdio.bcf.dsl v1` preamble).
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new file, genuine
  `encode_pack(demo_bcf_snapshot())` bytes (this fixture didn't exist before).

No `📡️example.spr.semio` fixture was added (optional per the checklist; not blocking).

## A genuine authoring bug caught and fixed: `comment` as a bare production name

While drafting both the mutations and diff grammar files, the value-shape production for
`BcfComment` was initially named the bare `comment` (its own real type name, lowercased) — this
collides with the grammar dialect's five RESERVED header-directive keywords
(`extension`/`use`/`start`/`comment`/`string`, `📖️grammar-recipe.md` §3 pitfall 3): `parse_grammar`
parses ANY leading ident matching one of those five words as a header directive, WHEREVER it
appears in the file, not just in the header. Caught before compiling (grepped for the pitfall
proactively, same defensive check the recipe's own pitfall list recommends) and renamed to
`bcf-comment` in both files, with the collision documented inline (matching xml's own `xml-comment`
rename precedent this ticket's recipe already cites). `insert-comment`/`remove-comment`/
`set-comment`/`comment-item`/`comment-diff` etc. were never at risk — those are distinct hyphenated
idents, not the bare word `comment`.

## A genuine mid-wave discovery: ISO-8601 timestamps / emails / content-position GUIDs need a wider `text-run`

While running `grammar_conformance_law` against the real `<guid>/markup.bcf` part, `recognize()`
returned `false` on the very first real fixture (`8f9e21f0-.../markup.bcf`, the demo topic's
`<CreationDate>2024-01-01T00:00:00+00:00</CreationDate>` content). Bisected against the real lexer
source (`🔍️lexer/🦀️component.rs`, not guessed): the shared lexer's digit rule only swallows a `-`
into the PRECEDING number when the very next char is a DIGIT — `2024-01-01T00:00:00+00:00` lexes as
`Int("2024") Int("-01") Int("-01") Ident("T00") Colon Int("00") Colon Int("00") Plus Int("00")
Colon Int("00")`, hitting bare `Colon`/`Plus` tokens `{IDENT|INT|FLOAT}+` (docx's/xml's own
`text-run` shape) never anticipated. The SAME artifact's `<CreationAuthor>`/`<Author>` email
content (`ueli@example.com`) independently needs `At`(`@`); and a content-position hyphenated GUID
(`<Viewpoint>`/`<Snapshot>` filename text) needs bare `Minus`(`-`) whenever a segment boundary
hyphen is followed by a LETTER rather than a digit. All four (`:`/`+`/`@`/`-`) are already real
single-char lexer tokens — extended this artifact's own `text-run` production to
`{IDENT|INT|FLOAT|":"|"+"|"@"|"-"}+` and documented the reasoning inline. This is a genuine,
artifact-agnostic lexer property (any XML-shaped grammar modeling real timestamp/email/hyphenated
CONTENT text hits it), not a bcf-specific hack — worth the recipe's attention alongside docx's own
self-closing-tag-fusion note if a future FG-wave standard's own text content needs the same shapes.

## Deviation from the literal checklist: `grammar_conformance_law`'s test strategy

Same shape docx's own report documents for the identical reason: `BcfSnapshot`'s
`ArtifactDsl::print_dsl`/`ArtifactPack::encode_pack` hex-dump the WHOLE binary zip container
(`⚙️engine/🦀️component.rs`'s `encode_bcf`/`decode_bcf`), matching this facet's SIBLING binary
protocol, not the text grammar. So `grammar_conformance_law` decodes the REAL zip entries
`encode_bcf` genuinely produces (via `zip::engine::decode_zip`, the same real codec `encode_bcf`
itself delegates to directly) and recognizes each of the three modeled part KINDS (`bcf.version`,
`<guid>/markup.bcf`, `<guid>/<guid>.bcfv`) own real text against the grammar — direct proof the
grammar matches this artifact's own real per-part XML bytes.

## `mechanism_gaps` (nothing new beyond what the recipe already tracks)

| gap id | area | note |
|---|---|---|
| `protocol-prim-ref-recursion` | `walk_protocol` | Hit by both `BcfDiff` (recursive `topics`→`comments`/`viewpoints` name-keyed triples, `BcfCamera` reached via `BcfViewpointDiff`) and `BcfMutation` (`SetSnapshot`'s whole `BcfSnapshot`, `SetViewpointCamera`/`SetViewpointComponents`' bare enum/struct payloads). Real fixed `format`/`flags`\|`format`/`tag` header individually protocol-walked; the recursive payload is one opaque `chain ... bytes` tail per `📖️grammar-recipe.md` §2.5 — the Rust `encode_diff`/`encode_op` side stays genuinely, fully structured (real varint-framed, recursively-typed binary encoders), round-trip tested independently (`op_text_binary_roundtrip_law`/`diff_codec_text_binary_roundtrip_law`, both pre-existing and still green). |
| `repeat-cannot-embed-jump` | zip protocol (restated) | Inherited from zip's own file verbatim — the per-entry `local_off` backward-jump cross-validation stays un-modeled, same explicit plan latitude. |
| `register-schema-spec-needs-recordspec` | `dsl::registry` | `BcfSnapshot`/`BcfDiff`/`BcfMutation` are all hand-rolled (confirmed by this file's own F6 doc comments: `#[derive(dsl::Dsl*)]` fails to compile on every one of these types via real `cargo check` errors, both `BcfCamera`'s enum-payload and the tri-state/`Option<Option<T>>` blockers) — `register_schema_spec` deliberately not called, same as json/csv/zip/png/docx. |
| bcf-is-not-OPC (this wave's own finding) | native-side classification | The ticket brief's own OPC-family classification does not literally apply to bcf's PART vocabulary (only its container framing) — documented above, not a mechanism gap so much as a brief-vs-reality correction, filed here so future FG-wave dispatch doesn't re-assume bcf carries `[Content_Types].xml`/`_rels/.rels`. |

## Verification

```
cargo check -p semio-s-plugin-stdio --lib                    # 0 errors (after riding out transient concurrent build-lock contention, per this ticket's own repo-rules note)
cargo test -p semio-s-plugin-stdio --lib "artifacts::bcf"     # 27 passed, 0 failed, 0 ignored (incl. all 6 new conformance-law tests + the pre-existing 21 F5/F6c-era tests, all still green)
cargo test -p semio-s-plugin-stdio --lib                     # 1837 passed, 1 failed, 4 ignored -- the ONE failure is artifacts::pptx::…::conformance_laws::fixture_honesty_law
                                                                (a shipped pptx .dsl.semio fixture not parsing back to demo_pptx_snapshot()), entirely outside this agent's
                                                                bcf/** ownership boundary and never touched by this wave -- classified as unrelated concurrent-session churn
                                                                per this ticket's own repo-rules note, not chased. Zero bcf-related failures.
bun run ./📜️script.ts policy                                  # 21607 repo-wide breaches (pre-existing, unrelated to this wave); bcf's own hits: 1 handcrafted-grammar/generic-spec
                                                                (false positive -- the checker naively substring-matches "payload" anywhere in the file text, including this file's OWN doc
                                                                comment discussing "whole-payload encoding"; docx's/xlsx's/dxf's/deflate's own already-real, already-verified grammar files
                                                                hit the identical false positive), 3 artifact-schema/facet-completeness + 2 mutation-migration/* (checker looks for
                                                                🧬️mutations//⚙️engine//🧬️schema/ directly under the ARTIFACT ROOT, not under bcf's real 🏅️standards/🔖️2.1/🪆️subsets/✳️any/ nesting
                                                                -- docx hits the identical 3 facet-completeness rows), plus cosmetic taxonomy/emoji-prefix + stdio-artifacts/composer +
                                                                os-state-authority/item-scope-global + artifact-schema/type-name-parity hits that docx's own already-closed row ALSO has,
                                                                one-for-one, at the same structural positions (composer.rs/OnceLock, io/deserializers path). None of
                                                                POLICY_GRAMMAR_PARSEABILITY/POLICY_PROTOCOL_PARSEABILITY/POLICY_FIXTURE_HONESTY/POLICY_LANGUAGE_REGISTRATION/
                                                                POLICY_STDIO_JSON_TRANSFER_BAN appear as check categories in this script's current output at all (same finding docx's
                                                                own report already documents) -- manual grep for serde_json::{to_vec,from_slice,to_string,from_str,Value} inside this
                                                                artifact's own files came back clean. Zero NEW breaches attributable to this wave's own grammar/protocol/codec work.
```

Ownership boundary respected: only files under `💬️bcf/**` were edited (plus reads of zip's/xml's/
docx's public API and grammar/protocol files for restatement precedent, never edits).
`📦️glue.rs`, `📜️script.ts`, the SDK traits, and the schema/dsl/protocol/registry modules were
never touched. IFC's own `🔖️4` standard was never touched (not this agent's scope regardless).
