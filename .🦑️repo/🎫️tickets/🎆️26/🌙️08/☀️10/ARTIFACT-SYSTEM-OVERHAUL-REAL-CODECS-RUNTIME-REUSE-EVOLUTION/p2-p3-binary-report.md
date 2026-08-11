# P2-P3 — `stdio.binary` (raw) — Real Grammar, Real Protocol, Real Binary Codecs

## Summary

`stdio.binary`/`raw` is the simplest artifact in the whole Phase 2 program, and it landed exactly
as sized: `OpBinary`/`DiffCodec` were ALREADY real binary before this wave (the derive path, per
Phase 1's F6 finding that binary/raw was the ONE clean full-derive-path success), so the actual
work was writing 6 handcrafted real-dialect `.grammar.semio`/`.protocol.semio` files, real fixtures,
per-artifact conformance laws, and 5-role `LanguageSpec` registration — no codec changes needed.

**Verification**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::binary"` → **25 passed, 0
failed** (own scope, incl. all 6 new conformance-law tests). Whole crate:
`cargo test -p semio-s-plugin-stdio --lib` → **1671 passed, 0 failed, 1 ignored**. `bun
./📜️script.ts policy` shows exactly one pre-existing breach under `🗿️artifacts/💾️binary/` (a
`taxonomy/emoji-prefix` warning on the `📄set-snapshot` directory name, identical to the same
breach on jpg/png/dwg's own `set-snapshot` dirs — a Phase-1 naming convention issue, not touched by
and not attributable to this wave) and zero breaches on any of the P2-specific policy rules
(grammar honesty, fixture honesty, facet-mirror drift) for this artifact.

## 1. Confirming OpBinary/DiffCodec are already real binary (no upgrade needed)

Read `🧬️mutations/🦀️component.rs` and `🔺️diff/🦀️component.rs` directly before writing anything, per
the mandatory-reading instruction:

- `BinaryMutation` derives `#[derive(dsl::DslOps)]`; its `impl protocol::OpBinary` is a pure forward
  to `dsl::variants_binary::encode_op`/`decode_op` — the generic `format u8 (=1) | variant ordinal
  varint | record body` layout, where `record body` is `store::pack_rt::encode_record_body` (the
  container-less twin: no magic/header/manifest/footer). Confirmed real by direct read, matching
  the P2-W0 census's own finding that `binary` is one of the 13 stdio standards NOT on the F6
  `print_op().into_bytes()` text-as-binary shortcut list.
- `BinaryDiff { splices: Vec<ByteSplice> }` derives `#[derive(dsl::DslDiff)]` — a plain struct with
  zero `Option<Option<T>>` tri-state fields and zero data-carrying enums (the derive's own
  restriction), so its generated `DiffCodec::encode_diff`/`decode_diff` genuinely route through
  `store::pack_rt::encode_document`/`decode_document` — the FULL `.spk` binary document container
  (magic `0x8953504B0D0A1A0A` + 32-byte header + a sequence of framed, CRC'd, deflate-compressed
  segments + an 84-byte footer), not a shortcut. Confirmed by direct read of the derive macro
  (`✨️derive/🦀️component.rs`'s `derive_dsl_diff`) and `🎒️pack/📐️format/🦀️component.rs`'s own module
  doc ("the `SPK` binary document container").

Both facts were verified empirically, not just by reading source: a temporary `#[test]` (added,
run once via `cargo test ... -- --nocapture`, then reverted before the final commit state) printed
real `print_op()`/`print_diff()`/`encode_diff()` output for every variant/case, confirming the exact
wire shapes before any grammar file was written — this is the "own artifact's own `print_dsl`/
`parse_dsl` first" instruction from the mission, applied literally.

**Result**: `diffcodec_binary_upgraded = false`, `opbinary_binary_upgraded = false` — both were
already real; this wave's job was writing accurate descriptions of what they already do, not
upgrading them.

## 2. Grammar + protocol files rewritten (all 6, all real-dialect)

All under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/`.

### 2a. `📸️snapshot/📝️text/📖️component.grammar.semio` (REWRITTEN)

Old file: single-line `dialect grammar stdio.binary.snapshot` header (2 tokens, rejected by this
dialect's parser) + ABNF body (`'semio'`/`SP`/`NL` literals, `*(...)` repetition — all outside this
dialect's alphabet). New file: real M1-dialect grammar matching `BinarySnapshot`'s handcrafted
`store::ArtifactDsl::print_dsl`/`parse_dsl` exactly — `document = artifact-mark hex-body`,
`artifact-mark = "stdio.binary"` (matching the m5 harness's own `dsl_body_from_fixture`
preamble-strip + `envelope_id()` substitution, same convention json/csv's own P1 pilots used),
`hex-body = hex` where `hex` is the framework's built-in macro (bare, no matching production — per
the mandatory `p2-p1-fix-report.md` reading, an open-ended hex run must never be a hand-rolled
`{INT|IDENT}*`-shaped **production**, since `Symbol::Star` doesn't backtrack; the macro does, and
already supports the zero-width/empty-body case `BinarySnapshot::default()` needs).

### 2b. `📸️snapshot/💾️binary/📡️component.protocol.semio` (REWRITTEN)

Old file: `magic = %x89.53.45.4D...` ABNF-alphabet body. New file: `framing record` + `chain payload
bytes` — literally the "one `chain bytes`" shape the mission called out, since
`ArtifactPack::encode_pack_with` is `wrap_binary(&envelope, &self.bytes)`: the SEMIO envelope
(described once, framework-side, per P2-M3) wrapping `self.bytes` verbatim with zero further
framing. Per M3's own documented guidance ("model it as if the bytes you're walking already start
at the payload"), this file does not re-describe the envelope.

### 2c. `🧬️mutations/📝️text/📖️component.grammar.semio` (REWRITTEN)

Old file: ABNF describing a serde-JSON `{"mutation":"...", ...}` wire struct — never emitted by the
real codec (which was ALREADY on the real `keyword key=value` `OpText` shape since Phase 1's F6,
this artifact just never got a grammar file rewrite for it). New file: 5 alternatives (`no-mutation`
/ `set-snapshot` / `splice` / `append-bytes` / `truncate-at`), every keyword/field-key copied
verbatim from directly-observed `print_op()` output (see §1's temp-test methodology), incl. the
`#[dsl(block)]` nested-record shape (`set-snapshot snapshot { schema=stdio.binary bytes="CQk=" }`)
and the `#[dsl(base64)]` quoted-base64 payload shape (`insert=".."`/`data=".."`) — no `hex` macro
needed here (unlike json/csv), since base64 payloads are always one quoted `TEXT` token, never a
bare digit-run that could collide with an adjacent keyword.

### 2d. `🧬️mutations/💾️binary/📡️component.protocol.semio` (REWRITTEN)

Old file: ABNF `json-body = *OCTET-utf8` claiming a JSON payload. New file: `header fixed 2 { field
format u8; field ordinal varint }` + `chain body bytes` — the two REAL fixed leading fields
(`dsl::variants_binary`'s `format`/`ordinal`) modeled field-by-field; the record body stays one
opaque trailing chain because `Prim::Ref` still unconditionally errors on struct/enum-shaped fields
during `walk_protocol` (confirmed unchanged since P2-M2 — same root cause every prior pilot's
`mechanism_gaps` cites), so the nested `BinarySnapshot` block inside `SetSnapshot`'s record body
isn't further protocol-walkable at this layer even though it is genuinely, boundedly-recursive (not
an arbitrary self-recursive type like json's `JsonValue`).

### 2e. `🔺️diff/📝️text/📖️component.grammar.semio` (REWRITTEN)

Old file: ABNF describing a serde-JSON splice-list struct. New file: `document = "splices" "=" "["
splice-item* "]"`, `splice-item = "offset" "=" INT "remove-len" "=" INT "insert" "=" TEXT` — matches
`print_diff()`'s real no-separator concatenated-item shape (`splices=[ offset=1 remove-len=2
insert="CQkJ" ]`), copied verbatim from direct observation, incl. the empty-list case
(`splices=[ ]`) and the zero-length-insert case (`insert=""`).

### 2f. `🔺️diff/💾️binary/📡️component.protocol.semio` (REWRITTEN — the interesting one)

Old file: ABNF `json-body = *OCTET-utf8`. New file: the real `.spk` document container, honestly
restated (cross-artifact `use` still doesn't resolve at walk time per M3, so this inlines rather
than pretending a `use semio.envelope`-style reference would work):

- `framing magic 0x8953504B0D0A1A0A` + `header fixed 32` with all 6 real header fields
  (`version_major u16`/`version_minor u16`/`required_flags u32`/`optional_flags u32`/
  `header_crc32 u32`/`reserved fixed 8`) — this EXACT magic/header-size/footer-size triple is the
  framework's own canonical worked example for this dialect
  (`📖️grammar/🦀️component.rs`'s `protocol_parse_print_round_trip_retains_body`/
  `protocol_parses_rich_struct_enum_segment_forms` tests use the identical values), so this is a
  restatement of an already-established shape, not a speculative one.
- `repeat segments { tag u8 trailer u32 until 0 arm 3 {...} arm 4 {...} arm 1 {...} arm 0 {...} }` —
  the real segment sequence `encode_document` always writes for a diff this size (`KIND_SYMBOLS=3`,
  `KIND_DOCUMENT=4`, `KIND_MANIFEST=1`, `KIND_END=0`, confirmed by direct byte-level observation of
  real `encode_diff()` output, not guessed), using M2's repeated-tag-dispatched-block construct.
  Each arm: `flags u8, seg_len varint, raw_len varint if flags eq 3, payload Array(u8,
  Field(seg_len))` — matches `encode_segment`'s real frame layout
  (`🎒️pack/📐️format/🦀️component.rs`) field-for-field; the `if flags eq 3` conditional-presence guard
  is real and structural (matches `decode_segment_at`'s own `compressed = flags & 0x01 != 0` check),
  not just empirically true for this artifact's fixtures.
- `footer fixed 84` — opaque fixed trailer (`Block::Footer` has no field-level detail in this
  dialect at all), matching the same honest-boundary treatment every other pilot gives an undecoded
  region (PDF's xref, DWG's encrypted sections).
- Segment PAYLOAD bytes are deflate-compressed (`codec: CodecId(1)` in `PackEncodeOptions::default`)
  — an honest opaque boundary, the same documented treatment this program gives every other
  compressed byte run.

This file's correctness was directly proven, not just argued: `protocol_walk_law` walks it against
the REAL bytes of `encode_diff()` for 3 representative `BinaryDiff` cases (empty, single-splice,
multi-splice) and asserts `consumed == bytes.len()` — it passed on the first attempt after the
byte-level derivation above, no iteration needed.

## 3. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`: was a preamble-less 11-byte fake
  (`68656c6c6f` with no `semio stdio.binary.dsl v1` line). Now the genuine
  `print_dsl(demo_binary_snapshot())` output, WITH the mandatory preamble:
  `semio stdio.binary.dsl v1\n68656c6c6f`.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`: did not exist before. Now the genuine
  `encode_pack(demo_binary_snapshot())` bytes (37 bytes: real 8-byte SEMIO magic + `u32le` token
  length + the `"stdio.binary.pack v1"` token + the 5-byte `"hello"` payload) — generated via a
  temporary regeneration test (`std::fs::write` from inside `#[test]`, run once, output inspected
  byte-for-byte against the expected envelope layout by hand, then reverted).
- `demo_binary_snapshot()` (new, `⚙️engine/🦀️component.rs`) is `bytes = b"hello"`, deliberately
  matching the companion REAL-format fixture asset `📚️examples/🎬️demo/🖼️assets/🎒️example.bin`
  (which is literally the raw bytes `hello`) — the single source of truth for both `.dsl.semio`/
  `.pack.semio` fixtures, asserted equal by `fixture_honesty_law`.
- `fixture_honesty_law` (new test) asserts `parse_dsl(fixture) == demo() && print_dsl(demo()) ==
  fixture` for both fixtures, byte-for-byte — they can never silently drift back to a fake.

## 4. Conformance tests (`⚙️engine/🦀️component.rs`'s new `conformance_laws` module)

- `committed_facet_files_parse` — all 6 files parse under `dsl::parse_grammar`/`dsl::parse_protocol`.
- `grammar_conformance_law` — snapshot grammar recognizes real `print_dsl` output for both the demo
  snapshot AND the empty-bytes case (exercising the `hex` macro's zero-width match).
- `ops_grammar_conformance_law` — mutations grammar recognizes real `print_op` output for every
  `BinaryMutation` variant (`mutations::demo_mutation_cases()`, extracted as a shared
  `#[cfg(test)] pub(crate)` helper from the pre-existing `all_variants()`/round-trip-law literal —
  single source of truth per CLAUDE.md, replacing the earlier per-test-parameterized version).
- `diff_grammar_conformance_law` — diff grammar recognizes real `print_diff` output for every
  representative `BinaryDiff` (`diff::demo_diff_cases()`, extracted the same way from
  `diff_codec_text_binary_roundtrip_law`'s literal case list).
- `protocol_walk_law` — `walk_protocol` against real `encode_pack` (envelope-unwrapped), every demo
  mutation's `encode_op`, and every demo diff's `encode_diff`, asserting `consumed == bytes.len()`.
- `fixture_honesty_law` — see §3.

Plus a new `demo_snapshot_round_trip` test (parallel to `codec_round_trip`, but against
`demo_binary_snapshot()` instead of the empty default).

## 5. Registration (`⚙️engine/🦀️component.rs`'s `register_pilot_languages`)

5-role `LanguageSpec` registration added, per json's/note's exemplar pattern: `stdio.binary`
(Document), `stdio.binary.op` (Ops, NEW), `stdio.binary.diff` (Diff, NEW), `stdio.binary.pack`
(Pack, NEW), `stdio.binary.spr` (Spr, NEW). All `dsl::passthrough_hooks`. `diff`'s `protocol` slot
stays `None`, matching the exemplar's own shape (the 5-role scheme has no dedicated "diff binary"
role even though the diff protocol file is real and conformance-tested — its binary form is
exercised directly by `protocol_walk_law` instead). Previously only 1 role (`stdio.binary`) was
registered.

`register_schema_spec` (P2-M3's `FullResolver` insertion API) was deliberately NOT called — see
`mechanism_gaps` below. Unlike json/csv, this is NOT because no `RecordSpec` exists (`BinarySnapshot`
and `BinaryDiff` both have real, derivable ones, `__dsl_spec()`/`__dsl_diff_spec()`) — it's because
the API registers exactly one spec under one schema id per artifact, and this facet genuinely has
three independently-derived specs (snapshot, per-mutation-variant, diff) with no single canonical
choice the current API shape expects.

## 6. JSON-transfer elimination re-check (item 7)

Re-confirmed by direct grep of `BinaryMutation`/`BinaryDiff`/`BinarySnapshot`'s own `.rs` files:
zero `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` usage anywhere. Matches the
P2-W0 census (binary was not flagged as a literal-JSON-transfer violation).

**Found, not fixed (deviation)**: the OLDER Phase-1 polyglot grammar-leaf mirrors under this
artifact's `🧬️mutations/💾️binary/`, `🔺️diff/💾️binary/`, and `🧬️mutations/📝️text/` directories
(`🌶️component.spicy` ×2, `🥋️component.ksy` ×2, `🅰️component.g4` ×1, `🔠️component.abnf` ×2) contain
FALSE claims that `OpBinary`/`DiffCodec` transport `serde_json`-encoded bytes — pre-existing
inaccuracies predating this wave (they were already wrong when written, since binary/raw's binary
codecs have been real since Phase 1's F6, never JSON at any point). These files are not
conformance-tested by anything in this repo (unlike `.grammar.semio`/`.protocol.semio`, which are)
and a full accurate rewrite in 4 more polyglot dialects (ANTLR4/Kaitai Struct/Spicy/ABNF) for 2
facets is outside this wave's explicit 7-item deliverable list (grammar.semio + protocol.semio +
fixtures + tests + registration + JSON-elimination — not the wider Phase-1 "all formats" facet-mirror
mandate). Flagged here rather than silently left, but genuinely not fixed — a real, scoped-out gap
for a future pass.

## Mechanism gaps

1. **`protocol-prim-ref-recursion`** — engine area: `dsl::grammar::protocol` (`walk_protocol`).
   Symptom: `Prim::Ref` unconditionally errors during `walk_protocol`, so the mutations protocol
   facet's `SetSnapshot` variant (whose record body embeds a nested `BinarySnapshot` block) can't be
   described field-by-field past the `format`/`ordinal` header — it stays one opaque trailing
   `chain ... bytes`. Same root cause every other P2 pilot's `mechanism_gaps` cites (json, csv, png,
   zip). Non-blocking: the Rust `encode_op`/`decode_op` side IS genuinely, correctly encoded (round-
   trip tested via `protocol_walk_law`/`op_text_binary_roundtrip_law`), just not further
   protocol-walkable at this layer.
2. **`register-schema-spec-one-spec-per-artifact`** — engine area: `dsl::registry::
   register_schema_spec`/`FullResolver`. Symptom: unlike json/csv (which have NO derivable
   `RecordSpec` at all), `stdio.binary` genuinely has THREE real, derivable `RecordSpec`s
   (`BinarySnapshot::__dsl_spec()`, one per `BinaryMutation` variant via `DslVariants`,
   `BinaryDiff::__dsl_diff_spec()`) — but `register_schema_spec(id, spec)` only accepts one spec
   under one schema id, with no API shape for "this artifact has 3 independently-schema'd facets."
   Worked around locally: skipped the call rather than arbitrarily pick one spec to register under
   a misleading id. Non-blocking, but a genuinely different flavor of gap than every prior pilot's
   (theirs was "no spec exists"; this one is "too many specs, API expects exactly one").

## Files touched

- `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/🦀️component.rs` (`demo_mutation_cases()` extracted; `all_variants(&b)` call
  sites → `demo_mutation_cases()`; unused `b` binding dropped from `op_text_binary_roundtrip_law`)
- `🧬️schema/🔺️diff/🦀️component.rs` (`demo_diff_cases()` extracted; `diff_codec_text_binary_
  roundtrip_law`'s inline literal → `demo_diff_cases()`)
- `⚙️engine/🦀️component.rs` (`demo_binary_snapshot()`, 5-role registration, `demo_snapshot_round_trip`,
  `conformance_laws` module with 6 tests)
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (regenerated, real, with mandatory preamble)
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real)
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-p3-binary-report.md`

## What I deliberately did NOT do

- Did not touch `📦️glue.rs`, `📜️script.ts`, the SDK traits, schema/dsl/protocol/registry modules, or
  `🏪️store` — every file touched is under this artifact's own `🗿️artifacts/💾️binary/**` directory or
  this report.
- Did not rewrite the Phase-1 polyglot grammar-leaf mirrors (.g4/.ksy/.spicy/.abnf/.ebnf) — see §6's
  deviation note.
- Did not call `register_schema_spec` — see `mechanism_gaps` #2.
- Did not attempt to make `Prim::Ref` recurse into the mutations protocol facet's nested
  `BinarySnapshot` block — that's a framework (`dsl`) change, outside this artifact's ownership
  boundary; documented as `mechanism_gaps` #1 and worked around with an opaque trailing chain,
  matching every other P2 pilot's identical treatment.
