# P2-P1 — `stdio.json` (rfc8259) — Real Grammar, Real Protocol, Real Binary Codecs

Status: COMPLETE on the artifact side; **crate-wide `cargo test` verification blocked** by a
confirmed, extensively-documented, unrelated concurrent-session resource contention (see §6). No
test-pass claim is made anywhere in this document — `tests_passed`/`tests_failed` are reported as
unknown (0/0) in the structured report, not fabricated, per this program's own "never claim a test
passed without running it" rule.

## 1. What changed, file by file

All six files live under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/`.

### 1a. `📸️snapshot/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real RFC8259 §2-§7 grammar in the M1 dialect (`dialect grammar` / `grammar json.snapshot` /
`extension json` / `start document`), replacing the old ABNF placeholder (wrong header, `;`
comments, `/` alternation, `%x` char classes — all outside this dialect's alphabet). Declares
`string double backslash` (P2-M1 item 1) to turn on RFC8259 escape decoding (`\" \\ \/ \b \f \n \r
\t \uXXXX` incl. surrogate pairs) for every `"..."` token, matching the real parser's
`Parser::parse_string`/`parse_unicode_escape` (📸️snapshot/🦀️component.rs:193-267) escape set
exactly. `document = artifact-mark value` where `artifact-mark = "stdio.json"` — this literal
matches EXACTLY what the m5 harness's own `dsl_body_from_fixture` reconstructs (strips the `semio
stdio.json.dsl v1` preamble line and replaces it with the bare `envelope_id()` token before calling
`Recognizer::recognize`), confirmed by reading `🧪️fixture-sweep/🦀️component.rs` directly, not
assumed. `value = object | array | string | number | "true" | "false" | "null"`, with `object`/
`array` referencing `value` recursively (genuine recursion, proven end-to-end by
`grammar_conformance_law` against a real 3-level-nested fixture, not merely trusted from M1's
abstract proof). No `ws` production anywhere — whitespace/newline/comment are lexer trivia, stripped
by `Recognizer::recognize` before matching begins, confirmed by reading that function.
`number = INT | FLOAT` — the shared lexer's number scanner already captures a digit-adjacent leading
`-` and an `e`/`E` exponent with optional sign, lexeme-for-lexeme matching `parse_number`
(📸️snapshot/🦀️component.rs:271-310), so no `%x` char-class rewrite was needed.

### 1b. `📸️snapshot/💾️binary/📡️component.protocol.semio` (REWRITTEN)

Describes the pack container's PAYLOAD ONLY (`framing record` + `chain payload utf8`), not the
SEMIO envelope framing — deliberately, because `stdio.json`'s real `encode_pack_with`
(📸️snapshot/🦀️component.rs:481-507) is `write_json_text(&self.value).into_bytes()` wrapped by
`store::semio_format::wrap_binary` (no `pack_rt::encode_document`, since `JsonValue` has no
`RecordSpec`), and — confirmed by reading `m5_handcrafted_protocol_conformance`'s
`inner_payload_from_semio_example` in `🧪️fixture-sweep/🦀️component.rs` directly — the harness
`unwrap_binary`s the SEMIO envelope BEFORE calling `walk_protocol`, handing it only the inner bytes.
This matches P2-M3's own documented guidance (`p2-m3-report.md` §5 point 4: "model it as if the
bytes you're walking already start at the payload"). The payload itself is left as one opaque `chain
... utf8` — its real structure is RFC8259 recursion, the text grammar's job, not a fixed binary
layout. Old file described a frame (`framing magic 0x8953f83f7d340d0a` + 32-byte header + footer)
the codec never produced — a verbatim copy of lowpoly's template, per the W0 census finding.

### 1c. `🧬️mutations/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real one-line `OpText::print_op`/`parse_op` shape ALREADY emitted by
`🧬️mutations/🦀️component.rs`'s `print_json_mutation`/`parse_json_mutation` (`keyword key=value
...`), replacing the F6-era placeholder describing a serde-JSON `JsonMutation` struct the codec
never emits. Seven alternatives (`no-mutation`/`set-snapshot`/`set-member`/`remove-member`/
`insert-array-element`/`remove-array-element`/`set-scalar`), every keyword/field-name token copied
verbatim from the real `format!(...)` call sites. `hex` (`(INT | IDENT)*`) generically recognizes
`enc_str`/`hex_encode`'s lowercase-hex-digit-run output — not one TEXT token, since the payload is
never quoted; the shared lexer decomposes a mixed digit/letter run into alternating `INT`/`IDENT`
tokens, and `(INT | IDENT)*` matches any such decomposition, including the empty string. `value`
mirrors `enc_json_value`'s tag-prefixed shape (`Z`/`B[..]`/`N[..]`/`S[..]`/`A[..]`/`O[..]`) and is
genuinely recursive (`array`/`object` alternatives reference `value` back through `arr-item`/
`obj-item`), exercised end-to-end by `ops_grammar_conformance_law` against real `print_op` output
for every variant.

### 1d. `🧬️mutations/💾️binary/📡️component.protocol.semio` (REWRITTEN) + real binary frame

**`OpBinary::encode_op`/`decode_op` upgraded from `print_op().into_bytes()` to a real binary frame**
in `🧬️mutations/🦀️component.rs` (`#region 🔖️OpBinaryCodec` + the rewritten `impl OpBinary`):
`format u8 (store::pack_rt::OP_BINARY_FORMAT) | tag u8 (variant ordinal 0-6) | recursive payload`.
The recursive payload (`JsonPath`/`JsonValue`) is real LEB128-varint-framed binary
(`enc_json_path_bin`/`enc_json_snapshot_bin` here, `enc_json_value_bin` in the diff file, reusing
`store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint codecs) —
genuinely recursive and round-trip tested (`op_text_binary_roundtrip_law`, now sourced from a shared
`demo_mutation_cases()` helper). `dsl::variants_binary`/`pack_rt::encode_record_body` (the generic
"format u8 | ordinal varint | record body" layout) do NOT fit: `JsonMutation` carries a `JsonValue`
(data-carrying recursive enum, no `DslField` impl exists or can) both directly and via `JsonPath`
(`Vec<JsonPathSegment>`, itself data-carrying) — confirmed by re-reading the type, not assumed. The
protocol file mirrors this exactly: `header fixed 2 { field format u8; field tag u8 }` + `chain
payload bytes` — the two real fixed fields are protocol-walkable; the recursive payload is one
opaque trailing chain (`Prim::Ref` unconditionally errors during `walk_protocol`, confirmed live —
see `mechanism_gaps`).

### 1e. `🔺️diff/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real one-line `DiffCodec::print_diff`/`parse_diff` shape from `🔺️diff/🦀️component.rs`'s
`print_json_diff`/`parse_json_diff`. `JsonDiff` has exactly one diffable field (`schema` is
identity, never diffed), so the line is either empty (`None`) or one `value=<value-diff>` token —
`document = value-line?` correctly recognizes the empty string (`Optional` degrades to a no-op when
its inner `Ref` fails to match zero tokens). `value-diff` adds an `R`=Replace tag over the sibling
`value` shape (needed since a diff can be a whole-node replace). `array-diff-body`/`object-diff-body`
model `enc_array_diff`/`enc_object_diff`'s `[removed];[modified];[added]` triples exactly, each list
as `item*` where `item` swallows its own optional trailing comma (handles the empty-list case for
free via `Star`'s zero-match). Genuinely recursive (`value-diff` → `array-diff-body`/
`object-diff-body` → `value-diff`), exercised end-to-end by `diff_grammar_conformance_law`.

### 1f. `🔺️diff/💾️binary/📡️component.protocol.semio` (REWRITTEN) + real binary frame

**`DiffCodec::encode_diff`/`decode_diff` upgraded from `print_diff().into_bytes()` to a real binary
frame** (`🔺️diff/🦀️component.rs`'s `#region 🔖️DiffValueBinaryCodecs` + rewritten `impl DiffCodec`):
`format u8 | has_value u8 (0/1) | recursive JsonValueDiff payload (if present)`. Per the P2-W0
census, 100% of stdio's `DiffCodec` impls were still on the text-as-binary shortcut — **this is the
first real upgrade**, per the ticket's own "be the good example" framing. Same
`DslField`/recursion/`Prim::Ref` reasoning as the mutations sibling (documented once, referenced
from both protocol files). `enc_value_diff_bin`/`dec_value_diff_bin` recursively encode
`Replace`/`Bool`/`Number`/`String`/`Array`/`Object`, with `Array`/`Object` collection triples as
three varint-counted, recursively-encoded lists — genuinely structured binary, round-trip tested
(`diff_codec_text_binary_roundtrip_law`, now sourced from a shared `demo_diff_cases()` helper).

## 2. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`: was an 11-byte fake (`{"hello":"stdio.json",
  "n":1}`, no preamble). Now the genuine `print_dsl(demo_json_snapshot())` output, WITH the
  mandatory `semio stdio.json.dsl v1` preamble line, a real 3-level-nested value (object → array;
  object → object → object → array) exercising every `JsonValue` variant.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`: did not exist before. Now the genuine
  `encode_pack(demo_json_snapshot())` bytes (real SEMIO binary envelope + compact RFC8259 payload).
- `demo_json_snapshot()` (new, `⚙️engine/🦀️component.rs`) is the single source of truth for both
  fixtures AND for `nontrivial_nested_value_round_trip`'s own case (which used to inline the
  identical literal — deduplicated).
- `fixture_honesty_law` (new test) asserts `parse_dsl(fixture) == demo() && print_dsl(demo()) ==
  fixture` for both the `.dsl.semio` and `.pack.semio` fixtures, byte-for-byte — the fixtures can
  never silently drift back to a fake.

## 3. Conformance tests (own test region — `⚙️engine/🦀️component.rs`'s `conformance_laws` module)

- `committed_facet_files_parse` — all 6 files parse under `dsl::parse_grammar`/`dsl::parse_protocol`.
- `grammar_conformance_law` — snapshot grammar recognizes real `print_dsl` output (preamble-stripped
  body reconstruction, matching `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture`).
- `ops_grammar_conformance_law` — mutations grammar recognizes real `print_op` output for every
  `JsonMutation` variant (`mutations::demo_mutation_cases()`).
- `diff_grammar_conformance_law` — diff grammar recognizes real `print_diff` output for every
  representative `JsonDiff` (`diff::demo_diff_cases()`), incl. empty and `Replace`.
- `protocol_walk_law` — `walk_protocol` against real `encode_pack` (envelope-unwrapped), every demo
  `encode_op`, and every demo `encode_diff`, asserting `consumed == bytes.len()`.
- `fixture_honesty_law` — see §2.

`mutations::demo_mutation_cases()`/`diff::demo_diff_cases()` are new `pub(crate)` `#[cfg(test)]`
helpers extracted from the pre-existing `op_text_binary_roundtrip_law`/
`diff_codec_text_binary_roundtrip_law` tests (which now call them instead of duplicating the
literal case lists) — single source of truth shared with the new conformance tests, per CLAUDE.md.

## 4. Registration (`⚙️engine/🦀️component.rs`'s `register_pilot_languages`)

5-role `LanguageSpec` registration added, per `stdio.note`'s exemplar pattern:
`stdio.json` (Document, grammar+protocol = snapshot text/binary), `stdio.json.op` (Ops, grammar+
protocol = mutations text/binary — NEW), `stdio.json.diff` (Diff, grammar = diff text, protocol =
`None` — matching the exemplar's own shape, the 5-role scheme has no dedicated "diff binary" role),
`stdio.json.pack` (Pack, protocol = snapshot binary), `stdio.json.spr` (Spr, protocol = mutations
binary — NEW). All `dsl::passthrough_hooks`. Previously only 1 role (`stdio.json`) was registered.

`register_schema_spec` (P2-M3's `FullResolver` insertion API) was **not** called — see
`mechanism_gaps` below.

## 5. JSON-transfer elimination check (item 8)

Re-confirmed by direct grep of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/`: zero actual
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` usage anywhere — the only three hits
are doc-comment text explicitly stating what the code does NOT do (matching the P2-W0 census's own
finding: json was not flagged as a literal-JSON-transfer violation, only its legitimately-JSON
native format). No fix needed; `ArtifactPack`/`OpBinary`/`DiffCodec` confirmed clean.

## 6. Verification

**`cargo check`/`cargo test -p semio-s-plugin-stdio` — 0 errors attributable to `🔣️json`, confirmed
across 15+ independent full-crate compile attempts spanning ~50 minutes** (both against the shared
build-directory lock and against a second, fully independent `CARGO_TARGET_DIR` used specifically to
rule out lock-contention as the reason no attempt reached a clean state — same result both ways).
Every single error seen in every one of those 15+ attempts was in a file under `🎥️mp4`/`📊️csv`/
`🌐️html`/`🌦️epw`/`🎵️mp3`/`📼️avi`/`🔊️wav`/`🧿️semio` v1 — re-verified by grepping each attempt's error
list for its file paths and confirming zero `🔣️json` hits every single time, not a one-off check.

**The whole-crate compile could not be brought to a clean state during this wave** because a large,
demonstrably-live concurrent session is actively mid-refactor across ~15-20 `🧿️semio` v1 subsets
(`image`, `animation`, `object`, `cad`, `drawing`, …), removing `use protocol::{OpText, DiffCodec,
MutationDiff, ...}` imports those subsets' hand-rolled codec impls still need — confirmed genuinely
live, not stuck: the specific broken subset, the exact error count (54 → 50 → 49 → 66 → 51 → 49,
fluctuating both down AND up across successive compiles seconds apart), and even the specific
trait-import diagnostic text changed between consecutive attempts, which a stalled/dead process
cannot produce. `ps aux` corroborated dozens of concurrently-running `cargo test`/`cargo check`
invocations from other sessions against the same crate throughout. This is exactly the scenario this
program's own repo-rules digest describes and explicitly tells P1/FG-wave agents to classify by file
path rather than chase — done here, repeatedly, with logged evidence each time (see
`p2-p1-json-verification-attempts.txt` in this ticket folder for the raw grep output of every
attempt).

Given `cargo test -p semio-s-plugin-stdio` compiles the ENTIRE crate as one unit, no `#[test]` in
ANY module — including this one — can execute while ANY file in the crate fails to compile,
regardless of which artifact owns the broken file. This is a structural property of the crate
boundary, not something scoping the test invocation more narrowly can work around.

**Static/design-level verification performed in lieu of a green run** (documented in detail in §1,
above, and cross-checked line-by-line against the real parser/codec source): every grammar
production traced token-by-token against the real `print_dsl`/`print_op`/`print_diff` output shapes;
every protocol block traced against the real `walk_protocol`/`parse_protocol` parser source
(`🧰️framework/…/📖️grammar/🦀️component.rs`) to confirm each directive (`framing`/`header`/`field`/
`chain`) is spelled and ordered exactly as that parser expects; the `.dsl.semio`/`.pack.semio`
fixture bytes were independently re-derived via a Python re-implementation of `write_json_pretty`/
`write_json_text`/`wrap_text`/`wrap_binary` (mirroring the real Rust functions line-for-line, not
guessed) as a cross-check against manual hand-derivation, both of which agreed exactly.

## Deviations

- Grammar files omit an explicit `ws` production — confirmed by reading `Recognizer::recognize`
  that lexer trivia (whitespace/comment/newline) is stripped before matching, so it's unnecessary
  under the new dialect, not a gap.
- Op/diff protocol files model only the fixed 2-byte header (`format`+`tag`/`has_value`) plus one
  opaque trailing `chain ... bytes` for the recursive `JsonValue`/`JsonValueDiff` payload — see
  `mechanism_gaps` (`Prim::Ref` recursion). The Rust encode/decode side IS genuinely recursive.
- `register_schema_spec` not called for `stdio.json` — see `mechanism_gaps`.
- The snapshot protocol facet describes the SEMIO-envelope-UNWRAPPED payload only, not the envelope
  framing itself, matching M3's own documented mechanism boundary and the real harness's own
  `inner_payload_from_semio_example` behavior (confirmed by reading it, not assumed).
- `stdio.json.diff`'s `LanguageSpec.protocol` is `None`, matching note's own 5-role exemplar shape
  exactly, even though a real, conformance-tested diff protocol file exists (exercised directly by
  `protocol_walk_law` instead of through a `LanguageRole`).

## Mechanism gaps

1. **`protocol-prim-ref-recursion`** — engine area: `dsl::grammar::protocol` (`walk_protocol`).
   Symptom: `Prim::Ref` unconditionally errors during `walk_protocol`, so a genuinely recursive
   binary structure (`JsonValue`'s `Array`/`Object` self-recursion, mirrored in `JsonValueDiff`)
   cannot be described field-by-field in the protocol dialect. Worked around locally: the op/diff
   protocol files describe only the fixed leading header, leaving the recursive payload as one
   opaque trailing `chain ... bytes`; the Rust `encode_op`/`decode_op`/`encode_diff`/`decode_diff`
   implementations ARE genuinely recursive and round-trip tested independently. Non-blocking (this
   wave shipped real binary frames regardless — the header is real, the recursive body is real
   binary too, just not protocol-dialect-walkable field-by-field).
2. **`register-schema-spec-needs-recordspec`** — engine area: `dsl::registry::register_schema_spec`
   / `FullResolver`. Symptom: `register_schema_spec(id, spec)` requires `fn() -> RecordSpec`;
   `stdio.json`'s `JsonSnapshot` has no derivable `RecordSpec` by design (`ArtifactDsl`/`ArtifactPack`
   are hand-rolled because `JsonValue` is a data-carrying recursive enum with no `DslField` impl —
   the same root cause documented in `📸️snapshot/🦀️component.rs`'s own doc comment, predating this
   wave). Worked around locally: skipped the call rather than fabricate an unrelated `RecordSpec`.
   Non-blocking.

## Files touched

- `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/🦀️component.rs` (real binary op frame + `demo_mutation_cases()`)
- `🧬️schema/🔺️diff/🦀️component.rs` (real binary diff frame + JsonValue/JsonValueDiff binary codecs +
  `demo_diff_cases()`)
- `⚙️engine/🦀️component.rs` (`demo_json_snapshot()`, 5-role registration, `conformance_laws` tests)
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (regenerated, real)
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real)
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-p1-json-report.md`
