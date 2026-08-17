# FG-wave: `stdio.xlsx` (ECMA-376 SpreadsheetML) — OPC-family agent

Wave scope: real grammar/protocol dialect files for xlsx's snapshot/diff/mutations facets, real
binary frames for `DiffCodec`/`OpBinary` (upgraded from the F6 text-as-binary shortcut), 5-role
`LanguageSpec` registration, the 6 per-artifact conformance-law tests, and genuine
`.dsl.semio`/`.pack.semio` demo fixtures. Xlsx is an OPC container (protocol) + contained XML parts
(grammar) artifact, following docx's just-landed FG-wave pattern (this wave's OPC pattern-setter)
per the ticket brief's explicit classification.

## Files touched

- `⚙️engine/🦀️component.rs` — added `demo_xlsx_snapshot()`, `register_pilot_languages()` (wired
  into `register()`), and the `conformance_laws` test module (6 laws, nested inside the existing
  `#[cfg(test)] mod tests`).
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten from the F6
  `*OCTET` placeholder to a real grammar modeling the contained XML parts (`[Content_Types].xml`,
  `_rels/.rels` + `xl/_rels/workbook.xml.rels`, `xl/workbook.xml`, `xl/worksheets/sheetN.xml`,
  `xl/sharedStrings.xml`), restated from docx's own FG-wave grammar for the OPC-universal parts and
  freshly traced from this artifact's own `⚙️engine/🦀️component.rs` for the xlsx-specific parts.
- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten to the
  real ZIP/OPC container layout (`repeat`/`backward`/`jump`), restated verbatim (only
  `protocol`/`schema` ids differ) from zip 2.0's own real protocol file per the ticket's OPC
  precedent (same restatement docx's own file already performed).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_xlsx_diff`/`parse_xlsx_diff` shape traced from `🔺️diff/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten to the real
  binary-frame shape (`header fixed 2 { format u8, flags u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added a full `BinaryCodecs` region (real
  LEB128-varint length-prefixed primitives, 1-byte-tagged `XlsxCellValue` binary sub-encoder, and a
  generic `NamedTripleDiff<K,D,T>` binary codec reused across all six of this artifact's collection
  triples); upgraded `DiffCodec::encode_diff`/`decode_diff` from `print_diff().into_bytes()` to the
  real frame; promoted the former test-only `sample_a`/`sample_b` to module-scope
  `snapshot_a()`/`snapshot_b()` (renamed for docx's own convention) and added `demo_diff_cases()`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real
  `print_xlsx_mutation`/`parse_xlsx_mutation` shape traced from `🧬️mutations/🦀️component.rs`.
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten to the
  real binary-frame shape (`header fixed 2 { format u8, tag u8 } + chain payload bytes`).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `OpBinaryCodec` region (real
  opc-package/workbook/snapshot binary encoders reusing the diff file's `pub(crate)` primitives);
  upgraded `OpBinary::encode_op`/`decode_op` from the text shortcut to the real 10-tag binary frame;
  promoted the former test-only `fixture`/`sweep_a`/`sweep_b`/`sample_mutations` to module-scope
  `pub(crate)` functions (`demo_mutation_cases()` renamed for docx's own convention).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — replaced the fake `"68656c6c6f"` placeholder
  (no preamble, not real xlsx bytes) with genuine `print_dsl(demo_xlsx_snapshot())` output.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new file, genuine
  `encode_pack(demo_xlsx_snapshot())` bytes (this fixture didn't exist before).

No `📡️example.spr.semio` fixture was added (optional per the checklist; not blocking).

## A genuine mid-wave discovery: `demo_xlsx_snapshot()`'s own OPC-part-order fixed point

`fixture_honesty_law`'s direct `parsed == demo()` comparison (not a re-encoded round trip) failed
on the FIRST real run — every field's CONTENT matched, only `opc.parts`' Vec ORDER differed.
Root cause, confirmed live (not assumed): `demo_xlsx_snapshot()` built via `build_minimal_xlsx`
(which produces `[workbook.xml, sharedStrings.xml, worksheet1, worksheet2]`) then appended
`xl/styles.xml` via a plain `set_part` call, giving the in-memory order
`[workbook, sst, ws1, ws2, styles]`. But `encode_xlsx`'s own `regenerate_workbook_parts` — which
BOTH `print_dsl` (via `encode_xlsx`) and `parse_dsl` (via `decode_xlsx` after a round trip) run —
`retain`s only the unmodeled part (`styles.xml`) in place, then RE-APPENDS `workbook`/`sst`/every
worksheet after it, producing `[styles, workbook, sst, ws1, ws2]` — a DIFFERENT order than the
in-memory `demo()` value's own field. `XlsxSnapshot`'s derived `PartialEq` is order-sensitive on
`opc.parts: Vec<OpcPart>`, so the two values compared unequal despite identical content. Fixed by
self-round-tripping `demo_xlsx_snapshot()` through `encode_xlsx`/`decode_xlsx` once at construction
time, landing it on the same fixed point every future `encode`/`decode` cycle reproduces — the
engine's own `regenerate_workbook_parts` doc comment already documents WHY part order stability
matters (it exists specifically so `between()` round trips reconstruct exact Vec order), this wave
just found a fixture-construction case that hadn't exercised the same invariant with an unmodeled
part appended AFTER `build_minimal_xlsx` returned. Documented inline in `demo_xlsx_snapshot()`'s
own doc comment; not a grammar/protocol/codec bug, purely a fixture-construction one, caught and
fixed live by this wave's own `fixture_honesty_law`, not silently worked around.

## Deviation from the literal checklist: `grammar_conformance_law`'s test strategy

Same deviation docx's own report already documents and the ticket brief itself anticipates for
every OPC-family member: `ArtifactDsl::print_dsl`/`parse_dsl` hex-dump the WHOLE binary OPC
package (unchanged, out of this wave's scope), so `grammar_conformance_law` instead decodes the
REAL zip entries `encode_xlsx` genuinely produces (via `zip::engine::decode_zip`, the same real
codec `opc::decode_opc` itself delegates to) and recognizes each modeled part's own real text
against the grammar. `worksheet-part`'s own production is generic over the sheet index, so both
`xl/worksheets/sheet1.xml` and `sheet2.xml` (from `demo_xlsx_snapshot()`'s 2 sheets) are checked
against the same production — direct proof the grammar matches this artifact's own real per-part
XML bytes for every worksheet count, not a single hardcoded case.

## `mechanism_gaps` (nothing new beyond what the recipe already tracks)

| gap id | area | note |
|---|---|---|
| `protocol-prim-ref-recursion` | `walk_protocol` | Hit by both `XlsxDiff` (`XlsxCellValue::Formula.cached` self-nests; the shared `NamedTripleDiff<K,D,T>` has no `Prim`-describable generic shape) and `XlsxMutation` (`SetSnapshot`'s whole `XlsxSnapshot`, `SetCell`'s direct `XlsxCellValue` payload). Real fixed `format`/`flags`\|`format`/`tag` header individually protocol-walked; the recursive payload is one opaque trailing `chain ... bytes` per `📖️grammar-recipe.md` §2.5 — the Rust `encode_diff`/`decode_op` side stays genuinely, fully structured (real varint-framed, recursively-typed binary encoders), round-trip tested independently (`diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law`, both green). |
| `repeat-cannot-embed-jump` | zip protocol (restated) | Inherited from zip's/docx's own file verbatim — the per-entry `local_off` backward-jump cross-validation stays un-modeled, same explicit plan latitude. |
| `register-schema-spec-needs-recordspec` | `dsl::registry` | `XlsxSnapshot`/`XlsxDiff`/`XlsxMutation` are all hand-rolled (confirmed by this file's own F6 doc comments: `#[derive(dsl::Dsl*)]` fails to compile — root cause `XlsxCellValue: DslField` not satisfied, plus the generic `NamedTripleDiff<K,D,T>` collection type has no `DslField` impl either, a second independent structural blocker) — `register_schema_spec` deliberately not called, same as json/csv/zip/png/docx. |
| formula-content lexical alphabet | grammar dialect, shared lexer | `<f>` formula bodies (`SUM(B2:B2)`) need `(`/`)`/`:`/`+`/`-`/`*`/`,` alongside IDENT/INT/FLOAT — confirmed live (not assumed) that every one of these is ALREADY a real single-char lexer token whose `Literal` match compares raw token TEXT (not kind), so no dialect change was needed, only a wider grammar alternation (`formula-text`). Not previously documented in the recipe as a worked example; noted here for the next OPC-family or formula-bearing artifact. |
| xlsx-specific OPC diff-type duplication | n/a | `XlsxOpcDiff`/`NamedTripleDiff<K,D,T>` are a THIRD independent copy of docx's own OPC-diff shape (same finding F5's own report already flagged for docx/xlsx/pptx/bcf) — explicitly out of this wave's scope per the ticket brief itself; not touched. |

## Verification

```
cargo check -p semio-s-plugin-stdio --lib                    # clean compile, warnings only (same "never used" pattern every FG-wave's pub(crate) demo/fixture helpers get outside #[cfg(test)], matching docx's own report)
cargo test -p semio-s-plugin-stdio --lib "artifacts::xlsx"    # 49 passed, 0 failed (after the part-order fixture fix above; re-confirmed on a clean retry after one transient, unrelated concurrent-session compile break — see below)
cargo test -p semio-s-plugin-stdio --lib                     # 1837 passed, 1 failed, 4 ignored, whole crate — the 1 failure is `artifacts::pptx::…::fixture_honesty_law`, entirely inside pptx's own in-progress sibling FG-wave scope (confirmed by file path, not touched, not chased, per this ticket's own "classify, don't chase" rule)
bun run ./📜️script.ts policy                                 # not run directly by this agent (ownership boundary excludes 📜️script.ts) — no `serde_json::{to_vec,from_slice,to_string,from_str,Value}` calls found by manual grep inside any of this artifact's own files, matching every other pilot/FG-wave's clean result
```

Full scoped test roster (final run, after the fixture-order fix): all pre-existing xlsx tests
(column-letter round trips, builder/analyzer round trips, shared-strings fidelity, Strict/
Transitional subset analyzer/builder/composer tests) plus this wave's own additions — 6
conformance-law tests (`committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`), the upgraded `diff_codec_text_binary_roundtrip_law`/
`op_text_binary_roundtrip_law` (now exercising the real binary frame, not the text-as-binary
shortcut) — all green.

One transient, unrelated compile break hit mid-verification: a concurrent session's in-progress
edit to `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` (`indexed_apply`/
`clone_via_patchable`, a framework-level generic-bound mismatch, `E0308`/`E0277`) broke the whole
crate for one run; confirmed via `git status` that file (and its `📡️spr` siblings) were mid-edit by
another session, retried once with no changes on my side, and the crate compiled clean — matching
this ticket's own documented "Concurrent Cargo Workspace Churn" pattern exactly. Never touched (it
is framework-level, outside my ownership boundary regardless).

Raw captures (this ticket folder): `p2-fg-xlsx-scoped-test.txt` (final scoped `artifacts::xlsx` run,
49/0), `p2-fg-xlsx-full-crate-test.txt` (final whole-crate run, 1837/1, the 1 being pptx's own
unrelated in-progress failure).

Ownership boundary respected: only files under `📕️xlsx/**` (plus this report and its two raw
capture `.txt` files) were edited. Reads of docx's/zip's own grammar/protocol files and public API
(for restatement/reuse, never edits). `📦️glue.rs`, `📜️script.ts`, the SDK traits, and the
schema/dsl/protocol/registry modules were never touched.
