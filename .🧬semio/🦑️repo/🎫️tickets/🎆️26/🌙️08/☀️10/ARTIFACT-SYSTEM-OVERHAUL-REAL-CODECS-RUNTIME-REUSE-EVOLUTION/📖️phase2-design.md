# Phase 2 Design — Real-Format Grammars & Protocols

Placed by **P2-W0 (recon)**, verbatim copy of the "PHASE 2 PROGRAM — Real-Format Grammars &
Protocols (FINAL PLAN)" section from the full plan journal. This is the primary reference for
every subsequent Phase-2 wave (M1-M3, P1-P3, PC, FG1-FG4, PW, G2).

Full journal (Phase 1 + Phase 2, with execution log and both phases' survey/mechanism findings):
`~/.claude/plans/the-current-schemas-are-scalable-journal.md`

See also `p2-w0-recon-report.md` in this same ticket folder for the W0 recon deliverable this
design doc feeds into (per-format dialect-requirements census, JSON-transfer census, baselines,
concurrent-edit check).

---

# PHASE 2 PROGRAM — Real-Format Grammars & Protocols (FINAL PLAN)

## Context

Phase 1 gave all 31 stdio standards complete semantic snapshots, handcrafted diffs, mutations returning diffs, and real op-codecs. But the repo-native `.grammar.semio` (text) and `.protocol.semio` (binary) files remain inert fossils: 96/96 grammar and 93/96 protocol files under stdio are unparseable by the repo's own `parse_grammar`/`parse_protocol` (wrong header dialect, ABNF bodies with `;` comments outside the lexer alphabet), the mutations/diff grammars describe pre-F6 serde-JSON shapes the Rust codecs no longer emit, all 28 `🗣️example.dsl.semio` fixtures are fakes, and zero stdio standards are wired into the working conformance harness (`m5_handcrafted_grammar_conformance` — proven on lowpoly/dag/cad/en1992/note/fem2d).

**User decisions (asked & answered, binding)**:
1. **Real formats**: each artifact's NATIVE side is modeled by the matching dialect — json/csv/xml/md/svg/step/... grammars model the REAL text syntax; zip/png/gif/pdf/dwg/... protocols model the REAL binary layout (opaque segments only where honestly irreducible, e.g. compressed payloads). The dialects + `Recognizer` + `walk_protocol` are generalized until they can express all formats.
2. **Handcraft all ~192 files** (32 standards × 3 facets × 2) — no generator; the handcrafted-normative doctrine stands ("the .semio file is normative; handcrafted Rust is the reference implementation" — grammar-file-contract.md).
3. **Everything at compile time** — no runtime grammar/protocol interpreters. Rust engine codecs remain the executable serde; files are normative models enforced by per-artifact conformance laws (Recognizer against real text, walk_protocol against real bytes).
4. **No JSON / no serde on any transfer path** — binary whenever possible, DSL text fallback. F6's `encode_diff/encode_op = print().into_bytes()` text-as-binary shortcuts are replaced with real binary frames; the io wire-compose layer's documented JSON simplification flips to pack binary; any remaining serde_json transfer usage in stdio + its framework seams is eliminated (serde trait BOUNDS on Mutation/MutationDiff stay — usage in transfer dies, bounds-removal is out of scope).

## Target architecture (per standard)

**Native-side classification** (which dialect models the real format):
- **Text-native** (grammar models the real syntax; protocol honestly models the pack container = SEMIO envelope + UTF-8 payload): txt/utf-8, json/rfc8259, csv/rfc4180, md/commonmark, xml/1.0, svg/1.1, obj/3.0, stl/ascii, step/ap214, ifc/4 (+2x3), dxf/r12.
- **Binary-native** (protocol models the real byte layout; grammar honestly models the DSL text form — hex-dump grammar per png's accurate precedent): binary/raw, zip/2.0, png/1.2, gif/87a+89a, jpg/jfif-1.01, bmp/v3, tiff/6.0, deflate/rfc1950, las/1.0, dwg/ac1018+ac1024.
- **Hybrid/both-real**: gltf/2.0 (grammar models the glTF JSON text; protocol models the GLB container), pdf/1.4+1.7 (grammar models COS text syntax; protocol models xref/stream binary framing), ply/1.0 (grammar models ASCII header+data; protocol models binary-endian variants), docx/xlsx/pptx/bcf (protocol models the ZIP/OPC container layout — delegating to zip's protocol productions where spec-identical; grammars model the contained XML parts via xml's grammar family or honest part-level modeling).
- **Diff/mutations facets** (no native format exists): grammar models the REAL op-line/diff-line text forms F6 landed (`keyword key=value`, collection triples); protocol models the REAL binary op frame — which this program UPGRADES from text-as-binary shortcuts to true binary frames (`format u8 | ordinal varint | body` layout via `pack_rt::encode_record_body` where the type is spec-expressible, or handcrafted per-artifact binary layouts otherwise — "binary whenever possible").

**Per-standard deliverable**: 6 handcrafted real-dialect files (📖️grammar + 📡️protocol × snapshot/diff/mutations facets) + real fixtures (`🗣️example.dsl.semio` = genuine print_dsl output with preamble; `.pack.semio` = genuine encode_pack bytes; real-format fixtures already exist as `example.json/.zip/.png/...` assets) + per-artifact conformance laws IN THE ARTIFACT'S OWN TEST REGION (`dsl::parse_grammar`/`Recognizer::compile`/`recognize` + `dsl::parse_protocol`/`walk_protocol` — all pub-reachable from stdio, verified) + 5-role `LanguageSpec` registration per note's exemplar (Document/Ops/Diff/Pack/Spr ids `stdio.<artifact>[.<standard>].{op,diff,pack,spr}`) + `register_schema_spec` entries once the FullResolver API exists + binary-frame upgrade of DiffCodec/OpBinary + JSON-transfer elimination for its own seams.

## Mechanism waves (framework files, serial, sole-owner)

- **M1 — Grammar dialect + Recognizer generalization** (`🗣️dsl/📖️grammar/🦀️component.rs`, lexer if needed): whatever the real text formats demand beyond today's dialect — candidates from the census (W0 confirms): character classes/ranges (JSON string escapes, CSV quoted fields), case-sensitive literals, repetition bounds, possibly lexer-level raw-text modes for formats whose tokens aren't semio tokens (markdown!). CONSTRAINT: extension-only — the 6 existing non-stdio pilot grammars + 7 family kits + self-hosting grammars must keep parsing and passing m5 (regression gate). Recursion already works (first-match matcher, keyword-first alternatives).
- **M2 — Protocol dialect + walk_protocol generalization** (same file): real binary layouts need — candidates: segment repetition (`repeat` on Block::Segment), `Prim::Ref` resolution against local Struct blocks (currently a hard error), count-from-field scoping, offset/back-reference handling for ZIP central directory (or honest forward-walk modeling with trailing-directory as bounded tail), CRC slot documentation fields, endianness markers, conditional structures keyed on tag bytes (PNG chunk types, GIF block introducers). Same extension-only constraint (en1992's protocol conformance keeps passing).
- **M3 — Harness + registry + policy prep**: `m5` fixture-sweep auto-discovery (drop the hardcoded 6-pilot list; discover grammar+fixture pairs — the ownership keystone: F-wave agents never touch framework files to enroll), `FullResolver` public insertion API (`register_schema_spec`; update the asserts-empty test), SEMIO-envelope framework-level protocol file (described once, per-artifact files describe post-unwrap payload), `pack_rt`-level wire helpers if the io-JSON flip needs them.
- **NOT in scope** (dropped with the real-format pivot): the 5 DslField/derive gaps (tri-state/tuples/nested-arrays/generics/enum-DslField) — they were prerequisites for derive-driven record-document serde, which the user's decisions supersede; they remain documented backlog from F6. No `from_record_spec` restoration (handcraft decision).

## Execution

```
W0   Recon: per-format dialect-requirements census (what M1/M2 must add, format by format) +
     JSON-transfer census (every serde_json transfer surface in stdio + io wire layer) +
     baselines + design doc placement in ticket folder                    (serial, 1)
M1   Grammar dialect + Recognizer                                         (serial, 1 + verify)
M2   Protocol dialect + walker                                            (serial, 1 + verify)
M3   Harness auto-discovery + registry API + envelope protocol file       (serial, 1 + verify)
P1   Grammar pilots: json + csv (well-known text formats, per user)       (parallel ×2 + verify)
P2   Protocol pilots: zip + png (well-known binary formats)               (parallel ×2 + verify)
P3   Smoke pilots: txt + binary (trivial both-sides)                      (parallel ×2 + verify)
       ↳ M-fix iterations (budget 3) after any pilot files blocking dialect gaps
PC   Pilot closer: 📖️grammar-recipe.md (copy-pasteable per-standard procedure w/ canonical
     excerpts from all 6 pilots) + policy seeding (shrink-only): PARSEABILITY (grammar+protocol),
     FIXTURE_HONESTY, LANGUAGE_REGISTRATION, CONFORMANCE_PRESENCE, NO_JSON_TRANSFER  (serial, 1)
FG1  Text-native fan-out: md, xml, obj, stl, dxf, step+ifc4 (1 agent), csv-family leftovers (×6-7)
FG2  Binary-native fan-out: gif×2, jpg, bmp, tiff, deflate, las, dwg×2 (×6-7)
FG3  Hybrid/heavy: gltf (dedicated), pdf×2, ply, svg (needs xml's FG1 grammar) (×4-5)
FG4  OPC tail: docx (pattern-setter early), xlsx, pptx, bcf, ifc/2x3 (32nd standard, included) (×4-5)
     — every FG wave: parallel fan-out + independent verify (parse files with the REAL parser,
       re-run tests, byte-check fixtures, files_touched framework-path check) + serial closer
       (full-crate gate, framework m5 gate, policy shrink, trinity check, STATUS.md ledger)
PW   Policy finalize: parseability→0 stdio breaches, allowlist drains, JSON-transfer ban→0 (serial, 1)
G2   Final gate + independent re-verify                                   (serial, 1-2)
```

**Feedback protocol** (binding, in every P/F brief): fan-out agents NEVER touch framework files; a dialect/walker gap blocking a deliverable → model the honest boundary (opaque segment/documented deviation), file `mechanism_gaps[]` in the report schema, continue; orchestrator triages after each wave → M-fix iteration (serial, same gates) if blocking or ≥2 upcoming standards affected. Hard budget 3 M-fix iterations, then stop-and-replan.

**Gates** (every M/P/F wave): `cargo test -p semio-s-plugin-stdio --lib` ≥1075/0 · `cargo test -p semio-framework-os-kernel` 0 failures (m5 + 6 non-stdio pilots green) · `cargo check --workspace` clean after M-waves · `cargo check -p semio-s-plugin-trinity` per F-closer · `bun ./📜️script.ts policy` seeds shrink-only. Concurrent-session protocol: `git status --porcelain` polling on 🗣️dsl/🎒️pack before/during M-waves, STATUS.md "M-window OPEN/CLOSED" announcements, stop-and-report on foreign churn.

**Sizing**: ≈50-55 agent-runs, ~15 waves, wall clock ≈9-13 sessions. Ticket: same ARTIFACT-SYSTEM-OVERHAUL ticket (extend; subagents never close). All prior-phase repo rules carry forward (no git-mutating commands, no worktrees, emoji paths quoted, ticket-folder .txt scratch, scratch-crate iteration, classify-don't-chase external churn).

**Risk register**: dsl-crate regression = whole-repo blast (serial M-waves, extension-only dialect changes, workspace check gates) · jolly-spindle collision on dsl files (polling + windows) · dialect inadequacy discovered late (W0 census first; pilots ordered to stress grammar (json/csv) and protocol (zip/png) sides immediately after M-waves; M-fix budget) · ZIP central-directory offset modeling may exceed a linear walker (W0 census decides: extend walker vs honest bounded-tail modeling — decision recorded before M2) · fixture fakery recurrence (FIXTURE_HONESTY policy + verify byte-checks) · JSON-flip breaking wire consumers (W0 census maps every consumer; io wire flip lands as its own M3 item with both ends updated atomically).

## Verification (phase-2 definition of done)

1. All ~192 grammar/protocol files parse under `parse_grammar`/`parse_protocol` (Rust sweep test + policy tripwire) — the 189-unparseable-at-0-breaches hole is structurally closed.
2. Every standard: grammar conformance laws green against real text fixtures (real example.json/.csv/... assets AND print_dsl outputs), protocol walk laws green against real bytes (real example.zip/.png/... assets AND encode_pack outputs, `consumed == len`), op/diff-line conformance green, fixtures genuine (preamble-bearing .dsl.semio, decodable .pack.semio).
3. m5 auto-sweep enrolls every stdio standard, zero soft-skips; the 6 original non-stdio pilots still green.
4. 5-role LanguageSpec registration + schema-spec registration greppable per standard; FullResolver resolves all stdio entries.
5. Zero serde_json on any transfer path in stdio + the io wire layer (grep gate + policy rule); DiffCodec/OpBinary emit real binary frames (no `.into_bytes()` text-as-binary in encode paths).
6. `cargo test -p semio-s-plugin-stdio --lib` 0 failures (count ≥ 1075 + all new laws); `cargo test -p semio-framework-os-kernel` 0 failures; `cargo check --workspace` clean; trinity green.
7. STATUS.md ledger complete; ticket closed by orchestrator only, explicit path + file list.

**Survey findings (explorer 2, verified)**:
- A REAL normative spec exists: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT/{grammar-file-contract.md, protocol-file-contract.md, protocol-dialect-contract-v2.md}`. A REAL parser/recognizer/walker exists: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` (2070 L): `parse_grammar`/`parse_protocol`/`print_grammar`/`Recognizer::compile`/`recognize`/`walk_protocol`/`verify_protocol_source`. Real dialect: `dialect grammar` + `grammar <id>` + `extension <ext>` + `use <family>` + `start <prod>` header; `|` alternation, postfix `*+?`, `{}` grouping, ALL-CAPS terminals (IDENT/INT/FLOAT/TEXT/BOOL), `"double quotes"`, `#` comments. Protocol dialect: `protocol <id>/version/schema/start/framing magic 0x…/header fixed n/field <name> <prim>/segment <name> <len-kind> <ty>/footer` with prims u8..f64/varint/zigzag/bytes/utf8/fixed(n); law: `walk_protocol` consumes exactly bytes.len().
- **Working exemplar**: lowpoly's snapshot grammar parses, compiles to a Recognizer, and is asserted against its fixture by `🗣️dsl/🧪️fixture-sweep/🦀️component.rs` `m5_handcrafted_grammar_conformance` (wired: lowpoly/dag/cad/en1992/note/fem2d — ZERO stdio). trinity/jack's mutations grammar is the correct shape exemplar for op grammars (matches F6's `keyword key=value` OpText shape). A protocol conformance harness `m5_handcrafted_protocol_conformance` discovers `.pack.semio` fixtures — none exist for stdio.
- **Every stdio .grammar.semio (placeholder AND "handcrafted") is unparseable by the repo's own parser**: ~26 standards use a one-line `dialect grammar stdio.x.y` header (parser rejects on 2nd token) + ABNF bodies (`/` alternation, prefix repetition, `;` comments — `;` isn't even tokenizable). json+pdf/1.7 have contract-correct headers but ABNF bodies. Only ONE stdio protocol triple (json) is in the real dialect — but it's a verbatim copy of lowpoly's template (wrong magic, describes a frame the codec never produces).
- **Mutations/diff grammars are pre-F6 fossils**: they document serde JSON wire shapes; F6's OpText emits `keyword key=value` + hex + `name{[removed];[modified];[added]}` triples. md/png/txt all checked: fully inconsistent with their own Rust. Actively misleading.
- **Snapshot text serde reality**: envelope = `semio <plugin>.<artifact>.<component> v<n>` preamble line (wrap_text) / `BINARY_MAGIC [0x89,'S','E','M',0D,0A,1A,0A] ‖ u32le(token len) ‖ token ‖ payload` (wrap_binary). txt+binary snapshot grammars are ACCURATE (gold standard); png/tiff nearly; md aspirational (describes CommonMark source; fixture is JSON!); gif = hex dump w/ placeholder grammar.
- **Fixtures**: all 28 `🗣️example.dsl.semio` files are 11-83 byte fakes (missing mandatory preamble, several wrong content e.g. md/csv contain JSON, png contains "hello" hex). Only deflate's is honest. No `.pack.semio` fixtures at all.
- **Inventory**: 📖 60H/36P, 📡 46H/50P (but "H" = merely not-placeholder, NOT dialect-conformant). ifc/2x3 (32nd standard) has 6 live unsuppressed grammar-honesty breaches (never allowlisted).
- **Pilot ladder recommendation**: binary/raw (2-field model, accurate grammars) → txt/utf-8 (only standard fully derive-driven via DslVariants/DslDiff — a grammar written against `dsl::print` is mechanically re-derivable for every other DslOps artifact) → deflate or stl (first real format grammar + binary layout, no recursion) → csv (first collection triple). Avoid as pilots: gltf (179 fields), svg/dxf (recursive/huge), md/json (recursive value types stress the recognizer subset first).

**Mechanism findings (explorer 1, verified with file:line citations)**:
- **Serde is 100% RecordSpec/Shape-driven**: text = `dsl::print/parse` (🗣️dsl/🧬️schema/🦀️component.rs: Shape 26 variants incl. Record(fn()->RecordSpec) lazy self-reference seam, Statements, Map, Table, Bytes64...); binary = `pack_rt::{encode_document, decode_document}` (container w/ magic/header/segments/manifest incl. `schema_hash(spec)`) and `encode_record_body/decode_record_body` (op-level twin, used by `dsl::variants_binary::encode_op/decode_op` = `format u8|ordinal varint|record body`). Both engines walk the SAME RecordSpec. **Grammar/protocol files drive NOTHING anywhere** — `Recognizer::recognize -> bool` and `walk_protocol -> ProtocolTrace{consumed}` are validators, not codecs; `from_record_spec` (spec→grammar generator) was DELETED (grammar/🦀️component.rs:1168, "handcrafted .grammar.semio/.protocol.semio are normative"); the reverse (grammar→spec) never existed.
- **The complete intended per-artifact wiring EXISTS as an exemplar**: `DerivedDocument` (🗣️dsl/🦀️component.rs:726-783): parse_dsl = split_text_preamble → dsl::parse(__dsl_spec) → __dsl_from_record; print_dsl = dsl::print → wrap_text; encode_pack_with = pack_rt::encode_document → wrap_binary; **record_spec() = Some(__dsl_spec())** → real 32-byte pack_schema_hash → hub schema pinning (currently DISABLED for all of stdio: every hand-rolled impl returns None → [0u8;32]). OpText via DslVariants keyword probing; OpBinary via variants_binary. **note's engine registration is the only complete 5-role LanguageSpec exemplar** (note.document/op/diff/pack/spr each with grammar+protocol text) — the target registration shape. stdio today: 8/28 artifacts register ZERO languages; json registers 1 role.
- **DslField blanket-impl inventory**: ints/bool/f32/f64/String/Vec<T>/BTreeMap<String,T>/[T;N]/DslValue/Wire ONLY. Missing: Option<T>, tuples (any arity), data-carrying enums, HashMap/Box/etc.
- **The 5 gaps' clean fixes (from code structure, cited)**: (a) Option<Option<T>>: classify_field peels exactly one Option (✨️derive:225-230); deeper issue — FieldValue has one Absent, no Some(None) repr; clean fix = new `Shape::Optional(Box<Shape>)` + `FieldValue::Null` (or Statements-at-0/1 trick), arms in the 4 exhaustive matches (print_shape/parse_shape/encode_record_fields/decode_record_fields). (b) tuples: macro fan-out `impl DslField for (A,..)` arity 2..=12; homogeneous cases (all real blockers are) reuse Shape::Tuple(_, Some(N)) exactly like impl_dsl_field_int! precedent. (c) nested tuple flatten bug: print_shape:1900-1910 renders bare comma-join, parse_shape:845-861 inner loop greedily over-consumes; clean fix = bracket-wrap nested-tuple elements (print_table_cell:1991-2001 brace-wrap precedent), no new Shape needed. (d) generics: derives never touch input.generics (zero split_for_impl hits); mechanical fix at 6 emission sites; fn-pointer coercion works post-monomorphization. (e) data-carrying enums: DslScalar hard-rejects (:778-780), DslEnum/DslOps emit DslVariants only; clean fix = dsl_variants_codegen additionally emits concrete `impl DslField` delegating via Shape::Statements(T::variants()) at cardinality 1 (~15 lines, per OptionStatements precedent) — unblocks JsonValueDiff/SvgNodeDiff/DxfValue/IfcValue/PptxShapeDiff/BcfCamera.
- Also: `#[dsl(flatten)]` and `#[dsl(list)]` are parsed but dead; `#[dsl(base64)]` requires bare Vec<u8> (not Option-wrapped). 17 field attrs total. `dsl_registry::full_resolver()` returns an EMPTY map with NO public insertion API — "hosts insert schema constructors" is currently impossible; `"<schema>#diff"` convention has zero live consumers. Fixture-sweep m5 harness: grammar conformance genuinely runs for 6 pilots; protocol conformance soft-skips 6/7 (only en1992 has .pack.semio on disk). Parseability census: 96/96 stdio grammar + 93/96 stdio protocol files cannot even be lexed (wrong header + ABNF `;`/`'`/`%` outside lexer alphabet); every OTHER plugin is 100% shape-clean. Policy gates check presence/include_str!, never parseability — that's how 189 unparseable files sit at "0 breaches".
- **Ordered prerequisites the code implies**: fix 5 gaps → stdio types can carry DslArtifact/DslDiff/DslOps → __dsl_spec() exists → record_spec()=Some → spec-driven print/parse/encode_document/encode_record_body + real schema hash → grammar/protocol files handcrafted in the REAL dialect describing the ACTUAL spec-driven forms, validated by Recognizer/walk_protocol against REAL fixtures (print_dsl output as .dsl.semio, encode_pack output as .pack.semio).

**Orchestrator-verified (Phase 3 spot-check)**: `parse_grammar/parse_protocol/print_grammar/print_protocol/verify_protocol_source/walk_protocol/Recognizer/FragmentRegistry/GrammarFile/ProtocolFile/...` are ALL `pub use`d at the dsl facade (🗣️dsl/🦀️component.rs:23-26), and stdio's glue.rs declares `extern crate semio_framework_os_kernel as dsl` — per-artifact grammar/protocol conformance tests inside the stdio crate are directly feasible today (`dsl::parse_grammar(...)`, `dsl::Recognizer::compile`, `dsl::walk_protocol(...)`), no framework reachability changes needed.

## Verification (end-to-end definition of done)

1. `cargo test -p semio-s-plugin-stdio --lib` → 0 failures, with per-standard `field_sweep` present (grep count == number of standards) and all 6 law suites present per standard.
2. `cargo check -p semio-framework` clean; `impl ArtifactBuilder` implementors repo-wide compile (incl. non-stdio).
3. `bun run ./📜️script.ts verify` → zero breaches on the new rules (facet drift, grammar honesty, DiffAlgebra coverage, vcs-machinery ban) and no regressions on existing ones; allowlists shrink-only.
4. Grep gates: zero `serde_json::Value` in stdio snapshot/diff/mutations files; zero `snapshot: Option<` full-replace slots in diff files; zero apply-and-capture diff bodies; zero `*OCTET` grammar leaves.
5. Fixture suites (dancing.gif, bachelor-thesis pdf, architectural dwg, metabolism gltf) green; codec_retention_law on real fixtures.
6. F6: OpText/OpBinary/DiffCodec round-trip laws per standard (`parse(print(op))==op`, `decode(encode(op))==op`, diff twins), `POLICY_DIFF_COMPLETENESS_ALLOWLIST` gains zero stdio entries (all covered).
7. STATUS.md updated with real verified state only; ticket_close with explicit path + full file list at program end (orchestrator only, never a subagent).
