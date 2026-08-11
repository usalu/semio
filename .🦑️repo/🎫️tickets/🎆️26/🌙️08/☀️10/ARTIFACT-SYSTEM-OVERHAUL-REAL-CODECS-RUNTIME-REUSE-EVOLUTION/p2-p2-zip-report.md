# P2-P2 — `stdio.zip` (standard 2.0) — Real Grammar, Real Protocol, Real Binary Diff Codec

Status: COMPLETE. `cargo test -p semio-s-plugin-stdio --lib "artifacts::zip"` → **46 passed, 0
failed**. Whole-crate `cargo test -p semio-s-plugin-stdio --lib` → **1654 passed, 1 failed, 2
ignored** — the 1 failure
(`artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec`)
is under `artifacts::semio::...` (the `🧿️semio` v1 artifact family), explicitly flagged in this
wave's own brief as a large, live, unrelated concurrent session's territory — classified by file
path, not chased, per the repo's standing "classify, don't chase external churn" rule. Zero zip
files touched by that session (confirmed: every failing/changed path is under `artifacts::semio`,
never `artifacts::zip`).

## 1. Reading done first (per the brief's mandatory order)

Read in full before writing any file: the Phase 2 "PHASE 2 PROGRAM" section of the journal plan,
`p2-w0-recon-report.md` (zip's own §1b row + the dedicated "ZIP central-directory — forward-walk
vs. backward-seek" paragraph), `p2-m2-report.md` (exact `repeat`/`backward`/`jump`/`Cond`/BE-`Prim`
syntax, incl. its own worked ZIP example and its documented deviation #1 — `repeat` arms cannot yet
embed a `jump`), `p2-m3-report.md` (m5 auto-discovery + `STDIO_CONFORMANCE_GRADUATED` graduation
mechanism — not touched, zero stdio standards graduated by any wave yet, correctly left that way),
`p2-p1-fix-report.md` (**mandatory**: grouping is always `{...}`, never `(...)`; hex-byte runs use
the framework `hex` macro, never `{INT|IDENT}*`; five reserved header words can never be production
names), and both `p2-p1-json-report.md`/`p2-p1-csv-report.md` as worked per-facet-deliverable
exemplars (fixture/registration/test-structure conventions carried over exactly).

Also read, directly, before designing: the real `⚙️engine::{decode_zip,encode_zip}`
(`🏅️standards/🔖️2.0/⚙️engine/🦀️component.rs`), the real `ZipSnapshot::{parse_dsl,print_dsl}` (hex
of `encode_zip` bytes, confirming zip's DSL text form is honestly a hex dump, not an aspiration),
the real `ZipMutation`/`ZipDiff` codecs (`🧬️mutations/🦀️component.rs`, `🔺️diff/🦀️component.rs`), and
the M2 dialect's own parser source (`🗣️dsl/📖️grammar/🦀️component.rs`) for the EXACT syntax of
`repeat`/`backward`/`jump`/`Array(inner, Count)`/tag-literal encoding — not guessed from the report
prose alone.

## 2. What changed, file by file

All under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/`.

### 2a. `📸️snapshot/📝️text/📖️component.grammar.semio` (REWRITTEN)

`stdio.zip` is binary-native (per this program's own classification) — its DSL text form is
honestly just a hex dump of the real `encode_zip` container bytes (confirmed by reading
`ZipSnapshot::print_dsl`/`parse_dsl` directly, not assumed), matching png's own accurate
hex-dump-grammar precedent. `document = artifact-mark hex`, where `artifact-mark = "stdio.zip"`
matches the m5 harness's own preamble-stripped body reconstruction exactly, and `hex` is the
framework macro (not a hand-rolled `{INT|IDENT}*` production — mandatory per the fix report).
Replaces the old one-line `dialect grammar stdio.zip.snapshot` header + ABNF `payload = *OCTET`
placeholder (unparseable by the real dialect).

### 2b. `📸️snapshot/💾️binary/📡️component.protocol.semio` (REWRITTEN) — **THE main deliverable**

Real ZIP 2.0 byte layout, field-for-field against `decode_zip`/`encode_zip`, using M2's `repeat`/
`backward`/`jump` exactly as the plan's dedicated ZIP paragraph and M2's own worked example
prescribe. Three blocks, in file order:

1. `repeat entries { tag fixed 4 until 0x504B0102 arm 0x504B0304 {...30 real fields...} arm
   0x504B0102 {} }` — local file headers + per-entry payload, forward from byte 0, terminating
   cleanly on the first central-directory signature (an empty arm for that tag, matching
   `walk_repeat`'s "sentinel must also match an arm" rule).
2. `backward eocd magic 0x504B0506 { 8 real fields incl. cd_offset }` — the EOCD genuinely cannot be
   located by a forward walk (its own comment field is 0-65535 bytes; W0's own "load-bearing
   decision": "nothing about the central directory's span is knowable until the backward scan
   happens first").
3. `jump central_dir_start from cd_offset {}` + `repeat central_directory { ... same shape as (1)
   ... }` — `cd_offset`, decoded by block 2, is looked up in the walk-wide field env (M2 item 3)
   and used to reposition `pos` absolutely, then the real, repeated central-directory records are
   walked forward until the EOCD signature reappears (terminating the same way block 1 does).

Every declared field width/order was cross-checked byte-for-byte against the real offsets
`decode_zip` reads at (local header: 30 bytes fixed before name, matches exactly; central
directory record: 46 bytes fixed before name, matches exactly; EOCD: 22 bytes fixed before
comment, matches exactly) — not merely "looks plausible." Confirmed via direct code read: **all
ZIP fields are little-endian** (no BE fields exist in this format specifically, per the module's
own doc comment) — the plain (non-`Be`) `u16`/`u32` prims are correct throughout.

**Two deliberate, documented deviations from full fidelity** (both explicitly latitude-permitted
by the plan/M2 report):
- The optional per-entry `local_off` backward-jump-to-local-header cross-validation `decode_zip`
  also performs is NOT modeled — M2's own report (deviation #1) confirms `repeat`'s arm bodies
  cannot yet embed a `jump` sub-directive per iteration; the central directory's own real per-entry
  fields (`local_off` itself decoded but not dereferenced) are the honest, complete model this wave
  ships. Filed as `mechanism_gaps`.
- The optional trailing streaming data-descriptor (general-purpose bit 3) after an entry's payload
  is not modeled — `encode_zip` never emits one (always clears bit 3, sizes always written up
  front, confirmed by reading its own doc comment); a third-party archive using one is still
  decodable by the real Rust `decode_zip`, just outside this declarative walker's honest reach for
  a construct our own writer never produces.

Because this file declares `backward`/`jump` blocks, `walk_protocol`'s final `pos == bytes.len()`
check is (correctly, per M2's own documented exception) skipped — the walk ends mid-EOCD-record
(4 bytes past its magic, where the final `central_directory` repeat's sentinel match stops), not at
EOF; the EOCD's own fields were already fully captured by block 2. `protocol_walk_law` (below)
asserts this explicitly rather than a blind `consumed == len`.

**Known limitation, not modeled**: the empty-archive (0 entries) case, where the very first tag
encountered is the EOCD's own signature rather than a local-header/central-dir tag — `until` only
supports one sentinel value per `repeat` block, and the two real "this repeat is done" conditions
(central-dir tag for the normal case, EOCD tag for the zero-entry case) can't both be declared at
once without composing two sentinels, which the dialect doesn't support. The demo/test fixtures all
use ≥1 entries, so this never surfaces in this wave's own laws; documented here rather than silently
worked around.

### 2c. `🧬️mutations/📝️text/📖️component.grammar.semio` (REWRITTEN)

`ZipMutation::print_op`/`parse_op` are GENERIC — `dsl::DslVariants::to_named_record` +
`dsl::print(..., JoinMode::Inline)`, no per-artifact custom printer (unlike json/csv's own
hand-rolled printers) — confirmed by reading `🧬️mutations/🦀️component.rs`'s `OpCodecs` region
directly. Traced the exact print shape through the framework's own derive/print source
(`✨️derive/🦀️component.rs`'s `to_kebab`/`dsl_variants_codegen`, `🧬️schema/🦀️component.rs`'s
`print_record_fields`/`keyed_field_rank`/`scalar_to_text`) rather than guessing: keyword = kebab
Rust variant name (`SetEntryData` → `set-entry-data`), field key = kebab Rust field ident
(`new_name` → `new-name`), every `ZipMutation` field is `keyed_field_rank` rank 0 (scalar) except
the two `#[dsl(block)]` nested-record fields and the two bare `Vec<ZipExtraField>` list fields
(rank 1), so print order is exactly Rust declaration order for every variant.

10 of 13 variants (every field a plain scalar) modeled precisely, field-for-field, token-for-token.
The 3 genuinely-recursive-payload variants (`SetSnapshot{snapshot}`, `AddEntry{index,entry}`,
`SetEntryExtra{local_extra,central_extra}`) — whose printed form nests a whole `ZipEntry`/
`ZipSnapshot` block or a list of `ZipExtraField` records with no closing delimiter a fixed, finite
production can bound (nesting depth is data-dependent, not grammar-dependent) — are modeled
honestly via `REST` (P2-M1's raw-span terminal), the same honest-boundary treatment this program
gives a compressed payload or a self-recursive protocol value. `ops_grammar_conformance_law` proves
`REST` genuinely swallows their real output, not just that the 10 simple variants parse.

Replaces the old pre-F6 fossil describing a `serde_json::to_string` shape (`{"mutation":"setSnapshot",...}`)
the codec no longer emits at all.

### 2d. `🧬️mutations/💾️binary/📡️component.protocol.semio` (REWRITTEN, Rust unchanged — already real)

`ZipMutation::encode_op`/`decode_op` were **already** real binary before this wave — a pure forward
to `dsl::variants_binary::encode_op`/`decode_op` (confirmed live by reading `🧬️mutations/🦀️component.rs`'s
`OpCodecs` region: no `print_op().into_bytes()` shortcut anywhere), matching the P2-W0 census's own
finding that zip is one of the 13 standards NOT on the F6 text-as-binary shortcut. **No Rust change
needed here** — `opbinary_binary_upgraded: false` in the structured report reflects "already real,"
not "upgraded by this wave." The protocol file was still the old stub
(`payload = utf8-bytes-of(mutation-line)` describing a fiction the codec never produced) and is now
rewritten to the REAL shape: `field format u8` + `field ordinal varint` (both genuinely
protocol-walkable, matching `dsl::variants_binary`'s real 2-part header) + `chain payload bytes`
(the per-variant record body — 13 different shapes, one per `ZipMutation` variant — has no single
field list this dialect can describe for all of them at once; same category of gap as json's
`Prim::Ref` recursion, filed as `mechanism_gaps`).

### 2e. `🔺️diff/📝️text/📖️component.grammar.semio` (REWRITTEN)

`ZipDiff`'s `DiffCodec` is hand-rolled (confirmed by reading `🔺️diff/🦀️component.rs`'s own doc
comment: `#[derive(dsl::DslDiff)]` cannot bind the tri-state `Option<Option<i64>>` `unix_mtime`
field) — real one-line grammar matching `print_zip_diff`/`parse_zip_diff` exactly: space-separated
top-level `comment=<hex>`/`entries{...}` tokens, hex for strings/bytes (via the framework `hex`
macro, never `{INT|IDENT}*`), a uniform `[0]`/`[1,<T>]` tag for the tri-state, single-letter
`tag:value` pairs for `ZipEntryDiff`'s 14 sparse fields, and `entries{[removed];[modified];[added]}`
for the one NAME-keyed collection triple. Genuinely exercised end-to-end by
`diff_grammar_conformance_law` against real `between()` output (both directions) covering every
field including the tri-state clear/set. Replaces the old pre-F6 `serde_json` fossil.

### 2f. `🔺️diff/💾️binary/📡️component.protocol.semio` (REWRITTEN) + **real binary frame upgraded**

**`DiffCodec::encode_diff`/`decode_diff` upgraded from F6's `print_diff().into_bytes()` shortcut to
a real binary frame** (`🔺️diff/🦀️component.rs`'s new `#region 🔖️BinaryDiffCodec`) — per the P2-W0
census, 100% of stdio's `DiffCodec` impls were still on that shortcut; this is a real upgrade, not
a no-op. `ZipDiff` is FLAT (no self-recursive value type, unlike json's `JsonValue`), so unlike
json's diff facet this is genuinely structured binary all the way down: `format u8 | has_comment u8
| has_entries u8` as three real, protocol-walkable fixed fields, then (when present)
length-prefixed UTF-8 `comment` and a real, recursively-encoded `entries` triple — a `u16` bitmask
per `ZipEntryDiff` (14 sparse fields, one bit each, present-fields-only payload in bitmask order)
for `modified[]`, and a positional 14-field encode for the whole `ZipEntry` payload `added[]`
carries. All varint/length-prefixed via `store::pack_rt::write_varint_u64`/`store::ByteReader`
(same convention P2-P1's json pilot established), plus a small local `write_varint_i64` helper
(zigzag, matching `crate::os_pack`'s own formula) since `store::pack_rt` re-exports the unsigned
writer and the reader's zigzag decode, but not the signed writer.

The protocol file itself models the real 3-byte fixed header field-by-field (genuinely
protocol-walkable, not opaque) — what stays opaque is only the variable-length `entries` triple
body, for the same "this dialect's `Array` repeats one fixed `Prim`, not a repeated multi-field
record" reason documented in 2d, filed once as `mechanism_gaps` and referenced from both files.

## 3. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`: was a genuine 11-byte fake (hex of the literal
  string `"hello"`, no preamble — confirmed the exact W0 finding). Now the genuine
  `print_dsl(demo_zip_snapshot())` output, WITH the mandatory `semio stdio.zip.dsl v1` preamble
  line: a real 345-byte ZIP archive (2 entries — one `Stored`, one `Deflate` — extra fields,
  distinct timestamps incl. a real Info-ZIP `UT` mtime record on one entry and none on the other,
  distinct attrs/comments, an archive-level comment) hex-encoded, 713 bytes total.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`: did not exist before (0 bytes is not "did not
  exist," it is the literal prior state — confirmed absent from disk pre-wave). Now the genuine
  `encode_pack(demo_zip_snapshot())` bytes (real SEMIO binary envelope + the same raw ZIP bytes),
  374 bytes.
- `demo_zip_snapshot()` (new, `⚙️engine/🦀️component.rs`) is the single source of truth for both
  fixtures and for `protocol_walk_law`'s own pack-facet case. Both fixtures were generated by
  actually running the real Rust `print_dsl`/`encode_pack` functions (a temporary `[DEBUG]`-prefixed
  dump test, added, run once, and removed — never left in the tree) and copying the exact bytes
  into place — not hand-derived or guessed. **Real, hard-earned correctness note**: the first
  attempt's `demo_zip_snapshot()` used `flags: 0` and an arbitrary (non-UT-shaped) `local_extra`
  payload, which `fixture_honesty_law` caught immediately — `encode_zip` unconditionally normalizes
  the UTF-8 flag bit on every round trip, and a `local_extra` payload that isn't a real Info-ZIP
  `UT` record round-trips to a DIFFERENT `unix_mtime` than the one written. Fixed by constructing
  `demo_zip_snapshot()` in the exact POST-round-trip normal form (`flags: 0x0800` up front, a
  correctly-shaped `UT` payload) so `parse_dsl(fixture) == demo()` holds byte-for-byte — documented
  in the fixture's own doc comment so a future editor doesn't reintroduce the same trap.
- `fixture_honesty_law` (new test) asserts `parse_dsl(fixture) == demo() && print_dsl(demo()) ==
  fixture` for `.dsl.semio`, and the pack twin, byte-for-byte — the fixtures can never silently
  drift back to a fake.

## 4. Conformance tests (own test region — `⚙️engine/🦀️component.rs`'s new `conformance_laws` module)

- `committed_facet_files_parse` — all 6 files parse under `dsl::parse_grammar`/`dsl::parse_protocol`.
- `grammar_conformance_law` — snapshot (hex-dump) grammar recognizes real `print_dsl` output.
- `ops_grammar_conformance_law` — mutations grammar recognizes real `print_op` output for all 15
  representative cases in `mutations::demo_mutation_cases()` (13 variants, 2 of them exercised in
  both tri-state shapes), including the 3 `REST`-modeled variants.
- `diff_grammar_conformance_law` — diff grammar recognizes real `print_diff` output for all 3 cases
  in `diff::demo_diff_cases()` (empty, and a `between()` result in both directions).
- `protocol_walk_law` — `walk_protocol` against real `encode_pack` (envelope-unwrapped; asserts a
  sane in-range `consumed`, NOT `== len`, per the documented jump exception — see 2b), every demo
  op's `encode_op` (asserts `consumed == len`, ordinary rule holds), every demo diff's `encode_diff`
  (same).
- `fixture_honesty_law` — see §3.

`mutations::demo_mutation_cases()` (new `pub(crate) #[cfg(test)]` fn, plus `entry()`/
`base_snapshot()` promoted from test-local to module-scope alongside it) and `diff::demo_diff_cases()`
(new) are single sources of truth shared with the pre-existing `op_text_binary_roundtrip_law`/
`diff_codec_text_binary_roundtrip_law` tests (which now call them instead of duplicating the
literal case lists) — same convention P2-P1's json pilot established, per CLAUDE.md's single-
source-of-truth rule.

## 5. Registration (`⚙️engine/🦀️component.rs`'s `register_pilot_languages`)

5-role `LanguageSpec` registration, per note's exemplar pattern (previously only 1 role registered,
`stdio.zip` Document): `stdio.zip` (Document, grammar+protocol = snapshot text/binary),
`stdio.zip.op` (Ops, grammar+protocol = mutations text/binary — NEW), `stdio.zip.diff` (Diff,
grammar = diff text, protocol = `None` — matching the exemplar's own shape, the 5-role scheme has
no dedicated "diff binary" role), `stdio.zip.pack` (Pack, protocol = snapshot binary — NEW),
`stdio.zip.spr` (Spr, protocol = mutations binary — NEW). All `dsl::passthrough_hooks`.
`register_schema_spec` (P2-M3's `FullResolver` insertion API) was **not** called — see
`mechanism_gaps`.

## 6. JSON-transfer elimination check (item 8)

Re-confirmed by direct grep of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/`: zero
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` usage anywhere in this artifact's
own scope. Matches the P2-W0 census's own finding — zip was not flagged as a literal-JSON-transfer
violation. `ArtifactPack`/`OpBinary`/`DiffCodec` confirmed clean; `DiffCodec` additionally upgraded
from the softer "text-as-binary" category (§2f) to real structured binary this wave.

## 7. OPC-family pattern-setter note (per this wave's explicit brief item)

Per the brief: docx/xlsx/pptx/bcf (a later wave) will eventually want to build on this file's shape
even though cross-artifact `use` doesn't technically work yet (confirmed still non-functional both
sides — `ProtocolFile.uses` is parsed/round-tripped but `walk_protocol` never reads it; unchanged
since W0/M2/M3). This wave wrote clean, well-named, reusable-in-spirit productions:
`📸️snapshot/💾️binary/📡️component.protocol.semio`'s three top-level blocks are named `entries`
(local headers), `eocd` (backward-scan), `central_dir_start`/`central_directory` (jump + repeat) —
these names and the exact byte-field lists inside each `arm` are the ones a future OPC-tail agent
should reference by name/path (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio`)
in its own brief, even without live `use` resolution — copy the `central_directory` arm's field
list verbatim for a `[Content_Types].xml`/`.rels`-bearing OPC container's own central-directory
walk, since the byte layout is spec-identical.

## 8. Verification

```
$ cargo test -p semio-s-plugin-stdio --lib "artifacts::zip"
test result: ok. 46 passed; 0 failed; 0 ignored; 0 measured; 1611 filtered out; finished in 0.03s

$ cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 1654 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 7.89s
```

The 1 whole-crate failure —
`artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec`
— is under `artifacts::semio::...`, the `🧿️semio` v1 artifact family this wave's own brief
explicitly named as a large, live, unrelated concurrent session's active territory ("expect
transient compile breaks unrelated to your work... Classify via file path before assuming it's
yours"). Confirmed zero relationship to zip: not under `artifacts::zip::`, not touching any file
this wave edited, not a grammar/protocol/dialect-mechanism failure (it's a PDF byte-round-trip
assertion inside a different plugin subtree entirely). Classified, not chased, per the repo's
standing rule; not investigated further.

Compile hygiene: both `cargo test` invocations above compiled the ENTIRE `semio-s-plugin-stdio`
crate clean (0 compile errors) both times — confirming this wave's edits (6 rewritten `.semio`
files, 2 Rust files edited: `🔺️diff/🦀️component.rs` for the binary `DiffCodec` upgrade,
`🧬️mutations/🦀️component.rs` for the `demo_mutation_cases()` extraction, `⚙️engine/🦀️component.rs`
for `demo_zip_snapshot()` + 5-role registration + `conformance_laws`) introduced zero regressions
anywhere else in the crate.

## Deviations

- Empty-archive (0-entry ZIP) is not modeled by the snapshot protocol's `repeat` blocks — see §2b's
  own "Known limitation" note. Not exercised by this wave's own fixtures/laws (all use ≥1 entries).
- Per-entry `local_off` backward-jump cross-validation not modeled in the snapshot protocol — M2's
  own documented latitude, see §2b.
- Streaming data-descriptor (general-purpose bit 3) not modeled in the snapshot protocol — honest
  boundary, `encode_zip` never emits one, see §2b.
- `register_schema_spec` not called for `stdio.zip`/`stdio.zip.diff` — see `mechanism_gaps`.
- The snapshot protocol facet describes the SEMIO-envelope-UNWRAPPED payload only (the real ZIP
  container bytes), matching M3's own documented mechanism boundary — the framework-level envelope
  file is not re-described here.
- `stdio.zip.diff`'s `LanguageSpec.protocol` is `None`, matching note's own 5-role exemplar shape
  exactly, even though a real, conformance-tested diff protocol file exists (exercised directly by
  `protocol_walk_law` instead of through a `LanguageRole`).
- 3 of 13 `ZipMutation` variants (`SetSnapshot`/`AddEntry`/`SetEntryExtra`) are modeled via `REST`
  in the mutations grammar rather than field-by-field — see §2c; the 10 remaining variants are
  modeled precisely.

## Mechanism gaps

1. **`protocol-array-of-records`** — engine area: `dsl::grammar::protocol` (`Prim::Array`,
   `Block::Repeat`'s arm-body grammar). Symptom: `Prim::Array(inner, Count)` repeats one
   FIXED-WIDTH `Prim`, never a repeated multi-field RECORD of per-item-varying shape; `repeat`'s
   own arm bodies are close but tag-dispatched (need a discriminator), not a plain
   "repeat N times, N from an earlier count field" construct for HOMOGENEOUS untagged records.
   Blocks: (a) `ZipMutation`'s per-variant `pack_rt::encode_record_body` payload (13 different
   record shapes, one per variant — `🧬️mutations/💾️binary/📡️component.protocol.semio`), (b)
   `ZipDiff`'s `entries` collection triple (`removed`/`modified`/`added`, each a runtime-counted
   list of `String`/`{name,ZipEntryDiff}`/`{index,ZipEntry}` — `🔺️diff/💾️binary/📡️component.protocol.semio`).
   Worked around locally: both protocol files model their real fixed header precisely, leaving the
   variable per-item body as one opaque trailing `chain ... bytes`; the Rust encode/decode side IS
   genuinely real, structured, recursively-typed binary in both cases (confirmed by direct read,
   not assumed). Non-blocking — same category as json's own filed `protocol-prim-ref-recursion` gap,
   arguably the general form of it (repeated heterogeneous records vs. self-recursive values).
2. **`repeat-cannot-embed-jump`** — engine area: `dsl::grammar::protocol` (`Block::Repeat`'s
   `RepeatArm.fields: Vec<Field>`, no `Block::JumpTo` variant reachable from inside an arm).
   Symptom: ZIP's real per-entry `local_off` backward-jump-to-local-header cross-validation cannot
   be composed with a `repeat` over central-directory entries — confirmed already documented by
   M2's own report (deviation #1), re-confirmed here by attempting the composition directly.
   Non-blocking, explicitly latitude-permitted by the plan ("the central-directory entries alone
   already carry the real per-entry metadata").
3. **`register-schema-spec-needs-recordspec`** — engine area: `dsl::registry::register_schema_spec`
   / `FullResolver`. Symptom: requires `fn() -> RecordSpec`; `ZipSnapshot`'s `ArtifactDsl`/
   `ArtifactPack` are hand-rolled (hex-of-bytes, no `RecordSpec` — same root cause json/csv already
   filed) and `ZipDiff` is hand-rolled for the `Option<Option<i64>>` tri-state reason documented at
   the top of its own file. Worked around locally: skipped the call rather than fabricate an
   unrelated `RecordSpec`. Non-blocking.

## Files touched

- `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten — main deliverable)
- `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten; Rust `OpBinary` was
  already real, untouched)
- `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🔺️diff/🦀️component.rs` (real binary `DiffCodec` frame + `enc_entry_bin`/
  `enc_entry_diff_bin`/`enc_entries_diff_bin` + `demo_diff_cases()`)
- `🧬️schema/🧬️mutations/🦀️component.rs` (`entry()`/`base_snapshot()`/`demo_mutation_cases()`
  promoted to module scope; `op_text_binary_roundtrip_law` now calls `demo_mutation_cases()`)
- `⚙️engine/🦀️component.rs` (`demo_zip_snapshot()`, 5-role registration, `conformance_laws` tests)
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (regenerated, real)
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real)
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-p2-zip-report.md`
