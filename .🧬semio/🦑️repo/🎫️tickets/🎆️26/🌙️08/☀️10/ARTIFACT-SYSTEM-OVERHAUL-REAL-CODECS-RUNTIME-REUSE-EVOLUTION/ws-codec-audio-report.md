# W-S Codec Wave — `stdio.semio.audio` (`✳️audio` subset)

Real-codec wave for a **semio** subset (`🧿️semio`), following the proven, fully-verified
`✳️workflow` pilot (`ws-codec-workflow-report.md`) and `✳️image` wave (`ws-codec-image-report.md` —
the closest precedent per the brief, since audio's `SemioAudioChannel.samples: Vec<f32>` is a
typed-raw-retention sample buffer field, like image's `rgba8: Vec<u8>`) templates and
`📖️grammar-recipe.md`. Scope: `✳️audio`'s three facets (snapshot, diff, mutations), plus a new
example fixture slug.

**Status: fully verified green in this session — real command output for every claim below, no
deferral.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per this wave's brief, the `#[derive(dsl::DslArtifact)]` path was checked first. `SemioAudioSnapshot`
has NO bare `Option<T>` field (unlike image's `icc: Option<Vec<u8>>`) and no shared
`⚙️engine/🧮️geometry` value-struct field (unlike workflow's `SemioPoint2`) — on the surface it looks
like a better derive candidate than either precedent. The actual blocker is the SAME one this
subset's own pre-existing `🔺️diff`/`🧬️mutations` facets already document in their module doc
comments (both facets were ALREADY hand-rolled going into this wave, confirmed by reading the
pre-wave files): this subset's diff/mutations codecs were hand-rolled from the start per the
ticket's own standing instruction ("hand-roll all diff/op codecs — do not fight the derive"), and
keeping the snapshot on the SAME hand-rolled hex/bracket convention as its sibling facets (rather
than a derive-based codec that would print/parse a structurally different wire shape than
`🔺️diff`'s own `enc_channel`/`enc_tag`/`enc_f32_list` value codecs) is the honest, single-source-
of-truth choice. Hand-rolling immediately was also the pragmatic choice given `SemioAudioTag` and
`SemioAudioChannel` are both plain `Vec<Record>` collections — workable for the derive's tested
`#[dsl(table)]` shape in isolation, but not worth risking a codec that diverges from the diff
facet's already-real text shape.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to
hex-of-JSON), reusing the exact hex/bracket-encoded convention (`enc_f32_list`'s `to_bits()` hex
tokens for exact-round-trip samples, hex-encoded UTF-8 for strings) this subset's own `🔺️diff`/
`🧬️mutations` facets already used pre-wave. `DiffCodec`/`OpBinary` were upgraded from the F6
`print_diff()`/`print_op().into_bytes()` text-as-binary shortcut to real binary frames, matching
image's own upgraded shape almost verbatim (format+presence header for diff, format+tag header for
mutations).

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/
  consume a genuine 5-line structured body: `schema=<hex>`, `sampleRate=<N>`, `format=<f>`,
  `channels=[<channel>,...]`, `tags=[<tag>,...]` — every field its own real token (hex for
  schema/tag key/value via a local `hex_encode`/`hex_decode`, `to_bits()`-hex-token lists for
  `f32` samples via `enc_f32_list`, a word tag for `format`) — not a hex dump of a JSON blob.
  Preamble handling unchanged (`store::semio_format::split_text_preamble`/`wrap_text`).
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_audio_snapshot_binary`/`decode_audio_snapshot_binary`: `format u8` + varint-length-
  prefixed `schema` UTF-8, real fixed `sample_rate` (`u32` LE), `audio_format` (`u8` tag), then
  `channels` (varint count + per-channel varint sample count + REAL 4-byte LE `f32` samples — no
  hex/text detour, genuinely more compact than the text DSL) and `tags` (varint count + per-entry
  varint-length-prefixed `key`/`value` strings) — `store::pack_rt::write_varint_u64`/
  `store::ByteReader`, same primitives workflow's/mesh's/image's own upgraded facets use. Replaces
  the old `serde_json::to_vec`-in-envelope shortcut entirely. Hand-rolled, not
  `store::pack_rt::encode_document` (needs a derived `RecordSpec`, which — per §1 — doesn't exist
  here).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }`/bracket grouping, bare `hex` macro, one production per line), matching
  `print_audio_snapshot_body` field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes
  Array(u8, Field(schema_len))` / `segment sample_rate u32` / `segment audio_format u8` (the proven
  bare form — consecutive bare segment lines auto-merge into one anonymous segment), then one
  honest opaque `chain payload bytes` tail for `channels`/`tags` (`protocol-array-of-records` gap
  — homogeneous-but-variable-length repeated records). The real Rust encode/decode stays fully
  structured past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the OLD ABNF-style hex-of-JSON placeholder
  description to real, descriptive (not test-parsed) mirrors of the new grammar/protocol shape.
- [x] **Fixtures** — `📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
  generated via the prescribed temporary-test method (§4 below) — genuine `print_dsl()`/
  `encode_pack()` bytes, never placeholder text.

### Diff (`🔺️diff/`)

- [x] **Already-real text codec confirmed unchanged** — `print_audio_diff`/`parse_audio_diff` were
  ALREADY a real hand-rolled hex/bracket grammar going into this wave (pre-existing, confirmed by
  reading the pre-wave file in full) — this wave did not need to touch the between/apply/absorb/
  inverse algebra region at all, only the `DiffCodec::encode_diff`/`decode_diff` binary methods.
- [x] **Binary upgrade** — was on the pre-wave `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed by reading the pre-wave file — its own doc comment explicitly named this "the same
  simplification gif 89a's own hand-rolled `DiffCodec` uses"). Now: `format u8` + `presence u8`
  (bit0=`sample_rate` bit1=`format` bit2=`channels` bit3=`tags`) as two real fixed header fields,
  then 0-4 varint-length-prefixed opaque text blobs (the same per-field `rate=`/`format=`/
  `triples::enc_indexed_triple`-based text `print_diff` already emits). One opaque trailing
  `payload` chain in the protocol description, not per-segment `Cond`s, for the
  `protocol-cond-cannot-chain` reason workflow's/mesh's/image's own diff protocols document (a
  second `if`-guard on a conditionally-decoded field hard-errors `eval_cond`).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — rewritten from the OLD ABNF-style
  placeholder to real dialect syntax, restates the collection-triple pattern for `channels`
  (`IndexedTripleDiff<SemioAudioChannelDiff,SemioAudioChannel>`, incl. the nested `channel-diff`
  tri-state-shaped `[0]`/`[1,<channel>]` tag for the one-field `SemioAudioChannelDiff.samples:
  Option<Vec<f32>>`) and `tags` (`IndexedTripleDiff<SemioAudioTag,SemioAudioTag>`, weak/whole-value
  keyed by index).
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors rewritten from the old ABNF-style placeholder description to
  the real grammar/binary frame shape.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — a fresh local
  `demo_diff_cases()` was needed (none existed pre-wave; the pre-wave test module had its own
  richer, differently-shaped `base_snapshot()`/sweep fixtures used by other tests, left untouched).

### Mutations (`🧬️mutations/`)

- [x] **Already-real text codec confirmed unchanged** — `print_audio_mutation`/
  `parse_audio_mutation` (`keyword payload...`, space-separated, `split_once(' ')`-parseable) were
  ALREADY real going into this wave.
- [x] **Binary upgrade** — same shortcut, same treatment. `format u8` + `tag u8` (variant ordinal,
  new `OP_KEYWORDS`/`variant_ordinal`, 0-9 across all 10 `SemioAudioMutation` variants) as two real
  fixed fields, then the variant's own argument text (the leading `keyword ` prefix stripped via
  new `print_audio_mutation_args`, splitting on the FIRST space to preserve any embedded spaces in
  bracketed payloads) as one opaque trailing `bytes` chain. `use protocol::{Mutation, OpBinary,
  OpText};` made unconditional (was test-only for `OpBinary`/`OpText`) since production
  `encode_op`/`decode_op` now genuinely need both traits in scope.
- [x] Grammar/protocol/mirrors, same treatment — grammar traced verbatim from
  `print_audio_mutation`'s real `format!(...)` call sites (rewritten from the OLD ABNF-style
  placeholder), incl. a self-contained restated `snapshot`/`channel`/`tag`/`format` value grammar
  matching `enc_snapshot`'s real 5-field bracket shape exactly.
- [x] Added module-scope `demo_mutation_cases()` (`#[cfg(test)] pub(crate) fn`) for the
  conformance-law tests. The pre-existing test module's own `all_variants()` test helper was left
  untouched (already covers all 10 variants for `mutation_diff_law`/`op_text_binary_roundtrip_law`
  — a different, complementary purpose from the new module-scope demo helper).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs` as a new `mod conformance_laws` nested inside its EXISTING
`#[cfg(test)] mod tests` block (audio's composer already had a real test module pre-wave, unlike
image's, which had none at all) — same location/shape workflow's/mesh's/image's own reports
identify as the right home (audio likewise has no per-standard `⚙️engine/` test module of its own;
`🎹️composer` is the closest "engine-equivalent").

### JSON-transfer ban (checklist item 8)

Grepped all three changed `.rs` files (`📸️snapshot`, `🔺️diff`, `🧬️mutations`) for
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` — **clean** (zero real hits; the
only remaining mention is one doc comment in `📸️snapshot/🦀️component.rs` describing the OLD,
now-replaced shortcut).

---

## 3. Exact files touched

All paths relative to repo root. 29 files modified inside `✳️audio/`, plus one new example slug
outside it (explicitly permitted by the brief).

**Snapshot**: `…/✳️audio/🧬️schema/📸️snapshot/🦀️component.rs`,
`…/📸️snapshot/📝️text/📖️component.grammar.semio`, `…/📸️snapshot/📝️text/🅰️component.g4`,
`…/📸️snapshot/📝️text/🔤️component.ebnf`, `…/📸️snapshot/💾️binary/📡️component.protocol.semio`,
`…/📸️snapshot/💾️binary/🥋️component.ksy`, `…/📸️snapshot/💾️binary/🌶️component.spicy`,
`…/📸️snapshot/💾️binary/🔠️component.abnf`. (`📸️snapshot/📝️text/🦀️component.rs` and
`📸️snapshot/💾️binary/🦀️component.rs` — the `COMPONENT_GRAMMAR_SEMIO`/`COMPONENT_PROTOCOL_SEMIO`
`include_str!` surface files — already existed pre-wave and needed no edits.)

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🅰️component.g4`, `…/🔺️diff/📝️text/🔤️component.ebnf`,
`…/🔺️diff/💾️binary/📡️component.protocol.semio`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/📝️text/🅰️component.g4`, `…/🧬️mutations/📝️text/🔤️component.ebnf`,
`…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️audio/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️audio/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎵️tone/🦀️component.rs`,
`…/🎵️tone/🟦️component.ts`, `…/🎵️tone/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output),
`…/🎵️tone/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, and every other subset were
left untouched, per the brief. One stray file was accidentally created at the WRONG path early in
this session due to an emoji-typo in a `Write` call (`🏅️标准` instead of `🏅️standards`) — caught
immediately (before any subsequent write to it) and `rm -rf`'d before it could accumulate content;
confirmed clean via `ls` immediately after.

---

## 4. Fixture generation method (recipe's prescribed procedure, followed exactly)

Added a temporary `#[test] fn ws_temp_print_real_fixtures()` to `🎹️composer/🦀️component.rs`'s new
`conformance_laws` module that called the real `store::ArtifactDsl::print_dsl(&demo)` /
`store::ArtifactPack::encode_pack(&demo)` for `snapshot::demo_audio_snapshot()` and `eprintln!`'d
both outputs (DSL as UTF-8 text, pack as a hex dump). Ran it once with `cargo test ...
ws_temp_print_real_fixtures -- --nocapture`, captured the real stdout, then used a small Python
script to write the DSL text verbatim (`newline="\n"`, no extra trailing newline — matching
`store::semio_format::wrap_text`'s real implementation, which does not append one) and decode the
hex dump into the real pack bytes (`bytes.fromhex(...)`) — never hand-transcribed. Deleted the
temporary test immediately after. `fixture_honesty_law` is the independent proof this worked (first
run before real fixtures: 44 passed / 1 failed exactly as expected on the placeholder text; re-run
after fixture generation: 45/45 green, see §6).

---

## 5. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `channels`/`tags` — homogeneous variable-length repeated records (per-channel variable-length `f32` sample buffers, per-tag key/value pairs). Opaque trailing `chain payload bytes` after the real `format`+`schema`+`sample_rate`+`audio_format` header/segments. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `sample_rate`/`format`/`channels`/`tags` — FOUR independently-optional fields (fewer than image's 7 or mesh's 3+1); same `presence`-bitmask + opaque-tail treatment, generalized to 4 bits (still fits one `u8`, in fact the same byte as workflow's 2-bit case with room to spare). |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types), same as workflow/mesh/image. Audio's `🎹️composer::register()` also had no pre-existing `dsl::register_language`/`register_schema_spec` call site to extend. |
| **already-hand-rolled-sibling-facets-drive-the-snapshot-choice** (a variant of image's `bare-Option-on-the-snapshot-itself` gap, but with a DIFFERENT root cause) | no, but same practical outcome as an existing recipe row | Unlike workflow (blocked by a shared geometry value-struct field), mesh (blocked by nested multi-buffer records), or image (blocked by a bare `Option<T>` field directly on the snapshot), audio's `SemioAudioSnapshot` has NO field shape that structurally blocks the derive path in isolation — `channels`/`tags` are both plain, single-level `Vec<Record>` collections, arguably the derive's best-supported shape. The reason to hand-roll here instead is PROCESS, not mechanism: this subset's `🔺️diff`/`🧬️mutations` facets were ALREADY real and hand-rolled going into this wave (pre-existing, not built by this wave), and matching the snapshot's wire shape to those sibling facets' own hex/bracket value codecs (rather than introducing a second, structurally different derive-based encoding for the same logical values) is the single-source-of-truth choice. **Recommend**: for any future semio subset whose `🔺️diff`/`🧬️mutations` facets are ALREADY real hand-rolled codecs pre-wave (increasingly common as this ticket's wave-by-wave sweep progresses), don't re-attempt the derive path for the snapshot even if its field shapes look derive-friendly in isolation — check the sibling facets' existing value-codec convention first and match it. |

---

## 6. Verification — real, not claimed (this session, foreground, actually observed)

All commands below were run directly in the foreground in this session and their real output was
read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** → **0 errors**, `Finished `dev` profile [unoptimized]
   target(s) in 1m 35s`, 491 warnings (all pre-existing/unrelated — verified by inspecting the
   warning list, none reference `subsets::audio`).

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::audio"`**
   → **45 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests individually
   confirmed `ok`: `committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`. (First run, before the real fixtures were generated, correctly showed
   44 passed / 1 failed — `fixture_honesty_law` failing on the placeholder text with the exact
   expected error `"semio audio snapshot: unknown line \"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST\""`;
   re-run after fixture generation is the 45/0 result above.)

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate), run TWICE (per the ticket's own
   concurrent-development ground rules) →
   ```
   test result: FAILED. 1908 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in ~13-15s
   ```
   both times, identically. The two failures are
   `artifacts::semio::standards::v1::subsets::video::composer::tests::conformance_laws::diff_grammar_conformance_law`
   and `…video::composer::tests::conformance_laws::fixture_honesty_law` — **not this wave's code**:
   `fixture_honesty_law` panics on `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"` (the exact pattern §4
   above shows how to close), and `git status` confirms `…/🪆️subsets/✳️video/…` is `M`-modified
   across many files by a different, concurrent session (per this ticket's own note that
   video/animation/presentation were being upgraded by sibling agents simultaneously), mid-way
   through its own real-codec wave on the `video` subset, same placeholder-fixture/incomplete-
   grammar pattern this report's own §4 shows how to close, just not yet done on that session's
   side. **Zero failures attributable to anything in `artifacts::…::audio`.**

**Status: this wave is genuinely proven, fully green for `✳️audio`.** The two whole-crate failures
are unrelated concurrent churn in `✳️video`, explicitly noted rather than chased, per this ticket's
own concurrent-development ground rules.
