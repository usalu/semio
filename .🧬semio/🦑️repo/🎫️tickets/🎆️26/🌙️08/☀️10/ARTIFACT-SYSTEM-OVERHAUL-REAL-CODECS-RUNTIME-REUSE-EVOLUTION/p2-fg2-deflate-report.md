# P2-FG2 — 🗜️deflate/rfc1950 — real grammar/protocol + binary-frame upgrade

## Summary

`stdio.deflate`'s RFC1950 zlib container is binary-native per the P2-W0 census (§1b row 8) and per
this wave's own brief. Its real byte-level codec (`encode_deflate_snapshot`/
`decode_deflate_snapshot`, ⚙️engine/component.rs:566-645) was already fully real and untouched by
this wave (F1/F6c already gave it typed CMF/FLG/dict-id/payload fields and a derived `OpBinary`).
What this wave found: **all six `.grammar.semio`/`.protocol.semio` facet files were still pre-
Phase-2 stub placeholders** (ABNF-flavored pseudo-syntax for the two snapshot files, and a stale
`serde_json` wire-shape description for the mutations/diff files — neither matches anything the
real Rust codecs actually emit), the artifact only registered 1 of the mandatory 5 `LanguageSpec`
roles, `DeflateDiff::DiffCodec` was still on F6's `print_diff().into_bytes()` text-as-binary
shortcut, and no conformance-law test module existed. All four are now fixed.

## What was real already (confirmed by direct reading, not assumed)

- **`OpBinary` for `DeflateMutation`**: already real — `encode_op`/`decode_op` forward straight to
  `dsl::variants_binary::encode_op`/`decode_op` (🧬️mutations/🦀️component.rs:122-129), matching the
  P2-W0 census's own "OpBinary via real binary" list, which names `deflate` explicitly (not the F6
  shortcut list). **No Rust change needed here** — confirmed, not assumed, by reading the impl body
  before touching anything. `opbinary_binary_upgraded: false` in the report JSON reflects this
  correctly (a no-op is not a mistake, per the recipe's own §4 checklist wording).
- **`DeflateSnapshot`**: real `store::ArtifactDsl`/`store::ArtifactPack` hand-rolled codecs (hex-text
  DSL, binary pack), PLUS a genuine `#[derive(dsl::DslRecord)]` giving it a real `__dsl_spec`
  (`fn() -> RecordSpec`) — the derive is additive (embeds `DeflateSnapshot` as
  `DeflateMutation::SetSnapshot`'s payload), not a replacement for the hand-rolled envelope format.

## What was NOT real, and what changed

### 1. Grammar files (rewritten, 3 total)
- `📸️snapshot/📝️text/📖️component.grammar.semio` — honest hex-dump grammar (`envelope-mark
  hex-body`), replacing an unparseable pseudo-ABNF stub (`cmf = 2HEXDIG`, `; comment` syntax, wrong
  header dialect entirely). Matches png/json's own precedent for a binary-native artifact's DSL TEXT
  form exactly (the DSL text IS a hex dump of the real zlib bytes, not a re-derivation of CMF/FLG
  structure — that's the protocol file's job).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — real one-line `OpText` grammar (`keyword
  key=value ...`), traced from a REAL `cargo test -p semio-s-plugin-stdio --lib artifacts::deflate
  -- --ignored --nocapture` run of a temporary `[DEBUG]`-prefixed test against representative
  `DeflateMutation` values (added, run once, deleted before closing — see §"Fixtures" below for the
  exact command). Replaces a stub describing a `serde_json` wire shape the real `print_op`/`parse_op`
  (dsl::DslVariants-derived) never emits. Confirmed live: an `Option<u32>` field's token is OMITTED
  entirely when `None` (not printed as `dict-id=null`/similar) — both in `SetPresetDictionary`'s own
  field and in the nested `DeflateSnapshot`'s `dict_id`.
- `🔺️diff/📝️text/📖️component.grammar.semio` — real grammar for the EXISTING hand-rolled
  `print_deflate_diff`/`parse_deflate_diff` (unchanged text form: space-separated `key=value`
  tokens, single-letter level tag `f`/`a`/`d`/`m`, `[0]`/`[1,<v>]` tri-state bracket for `dict_id`,
  lowercase hex for `payload`) — traced from direct reading of the real `format!(...)` call sites,
  same as xml's own diff grammar precedent (`document = tok1? tok2? tok3? tok4? tok5?`, each
  independently optional in fixed declaration order).

### 2. Protocol files (rewritten, 3 total)
- `📸️snapshot/💾️binary/📡️component.protocol.semio` — real RFC1950 byte layout. `cmf`/`flg`
  (u8 each, genuinely byte-walked) + `body` (opaque `bytes`, covers the optional DICTID prefix and
  the compressed DEFLATE bitstream) + `adler32` (u32be, genuinely byte-walked trailing field) — all
  four fields kept in ONE `header` block (not split via `chain`/`segment`) so `walk_fields`'s own
  per-block field-reservation math (`fields[index+1..].map(prim_fixed_width).sum()`) correctly
  reserves the trailing 4 bytes for `adler32` around the greedy `body` field. Documented, don't
  fabricate: the `(CMF*256+FLG) % 31 == 0` check and the FDICT-bit-gated DICTID presence are both
  genuinely NOT expressible via this dialect's `Cond` (see `mechanism_gaps` below) — the real Rust
  decoder still enforces both.
- `🧬️mutations/💾️binary/📡️component.protocol.semio` — `format u8 | ordinal varint | chain bytes`,
  copied in shape from stdio.txt's own already-real mutations protocol (same underlying
  `os_pack::encode_record_body` framework-generic wire, since `DeflateMutation` derives
  `dsl::DslOps` the same way `TxtMutation` does).
- `🔺️diff/💾️binary/📡️component.protocol.semio` — `format u8 | flags u8 | chain payload bytes`,
  copied in shape from stdio.xml's own already-real diff protocol (same `Cond`-can't-bitmask root
  cause, see below) — describes the NEW real binary frame (see §3).

### 3. `DeflateDiff::DiffCodec` binary-frame upgrade (`diffcodec_binary_upgraded: true`)
`encode_diff`/`decode_diff` were `Ok(self.print_diff().into_bytes())` / `parse_diff` round-trip
(F6's text-as-binary shortcut — confirmed via direct reading before touching, matching the P2-W0
census's finding that 100% of stdio's `DiffCodec` impls were still on this shortcut). Upgraded to a
REAL binary frame, following xml's own already-real precedent exactly:
`format u8 | flags u8 | [compression_method u8][window_bits u8][compression_level_hint u8]
[dict_id: presence u8 + optional u32][payload: rest-of-buffer bytes]`, `flags` bits 0-4 marking each
of the 5 top-level `Option` fields' presence, each present field's payload following in fixed
declaration order, `payload` last so it can be bare "rest of buffer." `DeflateDiff` stays
hand-rolled (not `#[derive(dsl::DslDiff)]`) — `dict_id: Option<Option<u32>>` is a genuine tri-state
field that blocks the derive (`dsl_derive::classify_field` peels exactly one `Option` layer, no
`impl<T: DslField> DslField for Option<T>` exists — same blocker `XmlDiff`/`GifDiff` hit, confirmed
via the file's own pre-existing doc comment, not re-derived).

### 4. 5-role `LanguageSpec` registration (`registration_roles: 5`)
`register_pilot_languages` only registered `stdio.deflate` (Document) before this wave. Added
`stdio.deflate.op` (Ops), `stdio.deflate.diff` (Diff, `protocol: None` per the 5-role scheme's own
shape), `stdio.deflate.pack` (Pack), `stdio.deflate.spr` (Spr) — all `dsl::passthrough_hooks`,
matching json/txt's own exemplar pattern exactly.

### 5. `register_schema_spec` (new: `register_schema_specs()`, wired into `register()`)
Calls `dsl::registry::register_schema_spec("stdio.deflate", DeflateSnapshot::__dsl_spec)` — real,
non-fabricated (`DeflateSnapshot` derives `dsl::DslRecord`, `__dsl_spec` genuinely exists).
Deliberately does NOT register `"stdio.deflate#diff"` — `DeflateDiff` has no derivable `RecordSpec`
(hand-rolled, tri-state-blocked derive, see §3) — filed as `mechanism_gaps` rather than fabricated.
`#[cfg(not(target_arch = "wasm32"))]`-gated matching `os_dsl::registry`'s own gate, with a new
`schema_spec_registration_resolves` test (matching txt's own precedent) confirming it genuinely
resolves through `dsl::registry::full_resolver()`.

### 6. Conformance-law test module (new, engine's own test region)
Added `mod conformance_laws` under `⚙️engine/🦀️component.rs`'s existing `#[cfg(test)] mod tests`:
`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — copied in shape from
json's own module, adapted to deflate's real facets. Added `demo_deflate_snapshot()` (engine),
`demo_mutation_cases()` (mutations, `#[cfg(test)]`), `demo_diff_cases()` (diff, `#[cfg(test)]`) as
the single sources of truth these laws (and the pre-existing hand-written unit tests) share.

## Fixtures regenerated (`fixtures_regenerated: {dsl: true, pack: true}`)

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` previously had NO preamble line at all (bare hex,
missing the mandatory `semio stdio.deflate.dsl v1` line) — a stale pre-Phase-2 artifact.
Regenerated genuinely: added a temporary `#[test] #[ignore] fn debug_dump_fixtures()` to
⚙️engine/🦀️component.rs that called the REAL `store::ArtifactDsl::print_dsl`/
`store::ArtifactPack::encode_pack`/`crate::…::engine::encode_deflate_snapshot` on a new
`demo_deflate_snapshot()` value (real preset-dictionary id set, exercising the FDICT/DICTID path;
real repetitive payload), ran it once via
`cargo test -p semio-s-plugin-stdio --lib "artifacts::deflate::standards::v_rfc1950::engine::tests::debug_dump_fixtures" -- --ignored --nocapture`,
copied the printed hex/bytes verbatim into the three fixture files (`🗣️example.dsl.semio` text,
`🗜️example.zz` + `🎒️example.pack.semio` binary via `xxd -r -p`), then deleted the temporary test
(replaced by the real `conformance_laws` module, which independently exercises the same demo value
through `fixture_honesty_law`). `example.zz`'s bytes changed from the old third-party-encoded
fixture (dynamic Huffman) to this artifact's OWN encoder's canonical fixed-Huffman output — same
"documented normal form" treatment `codec_retention_law_real_fixture_normal_form`'s own doc comment
already establishes for this codec (it always emits fixed-Huffman, never reproduces a foreign
dynamic-Huffman encoding byte-for-byte; what must round-trip exactly is the typed fields + payload,
which it does).

## Mechanism gaps hit

- **`protocol-cond-no-bitmask`** (new — not yet in the consolidated table by this exact name, but
  the identical underlying limitation was already identified in prose by this program's prior FG1
  wave for xml's own `flags` byte). `Cond` (`if <field> eq/ne/lt/le/gt/ge <literal>`) can only
  compare ONE already-decoded field's WHOLE numeric value against a static literal — no
  bitwise-AND/modulo operator exists, and it cannot reference two fields jointly either. Hit TWICE
  in this artifact:
  1. RFC1950's `(CMF*256+FLG) % 31 == 0` FCHECK constraint — spans two fields, plus needs modulo.
     Per this wave's own brief: documented in the protocol file's comment, not fabricated as an
     assertion.
  2. RFC1950's FDICT-bit-gated (bit 5 of FLG) optional DICTID presence — a single-bit test within
     an already-decoded byte, genuinely not expressible via any single eq/ne/lt/le/gt/ge threshold
     (FDICT's contribution isn't isolable from FLEVEL's higher bits by range comparison). Modeled
     honestly: folded into the SAME opaque `body` span the compressed DEFLATE bitstream already,
     independently, needs — the real Rust codec (`encode_deflate_snapshot`/`decode_deflate_snapshot`)
     still fully, genuinely types `dict_id` on its own. `DeflateDiff`'s own binary frame (§3) hits
     the SAME root cause for its 5-bit `flags` byte, modeled the same way (`format`/`flags` real,
     the 5 payload fields one opaque `chain payload bytes` tail) — matching xml's own precedent
     file exactly.
  Non-blocking in both cases — the real Rust encode/decode side stays genuinely, fully structured
  and independently round-trip tested; only the protocol DESCRIPTION's depth is limited.
- **`txt-opbinary-record-body-wire-is-framework-generic`** (already consolidated in
  `📖️grammar-recipe.md` §5, cited not re-filed) — `DeflateMutation`'s `format`/`ordinal` header is
  real; the remaining `os_pack::encode_record_body` wire is the framework-generic per-`DslOps`-type
  encoding, not expressible by `Array`/`Ref`/`repeat` as they stand (no runtime-field-id TAG
  dispatch, no symbol-table back-reference resolution). Non-blocking.
- **`register-schema-spec-needs-recordspec`** (already consolidated) — `DeflateDiff` has no
  derivable `RecordSpec` (hand-rolled, tri-state-blocked). Skipped the `"stdio.deflate#diff"`
  registration rather than fabricate one; only the snapshot id is registered. Non-blocking.

A genuine, brief framework-mechanics finding along the way (not a gap in the dialect's
*expressiveness*, just a note for future FG-wave authors): `Block::Chain`'s own walk arm
(`walk_protocol`'s `Block::Chain(prim) => walk_prim(prim, bytes, pos, state, 0)`) hardcodes
`reserved_tail=0` for itself regardless of what `trailing_reserved` computes for LATER blocks — so
two separate `chain <name> <prim>` declarations stacked back-to-back would NOT correctly reserve
room for the second one (the first would greedily consume everything). The correct construct for
"one greedy opaque field followed by a real fixed trailing field" is a single field LIST inside one
`header`/`segment` block (`walk_fields`'s own `fields[index+1..].map(prim_fixed_width).sum()`
reservation, which works correctly) — used for the snapshot protocol's `body`+`adler32` pair.
Documented inline in the snapshot protocol file's own comment for the next FG-wave agent who hits
the same shape (a compressed/opaque middle span with a real fixed trailer after it).

## Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::deflate"` → **26 passed, 0 failed, 0 ignored**
— every pre-existing law (F1/F6c's `mutation_diff_law_every_variant`, `inverse_law_mutation_and_diff_level`,
`absorb_law_scalar_lww_and_associativity`, `field_sweep_between_covers_every_field`,
`between_roundtrip_law_synthetic_and_real_fixture`, `codec_retention_law_*`,
`op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`, `codec_round_trip`,
`snapshot_codec_*`, `zlib_round_trip`, `raw_deflate_*`, `adler32_empty_is_one`,
`demo_source_nonempty`) PLUS all 6 new conformance-law tests
(`committed_facet_files_parse`/`grammar_conformance_law`/`ops_grammar_conformance_law`/
`diff_grammar_conformance_law`/`protocol_walk_law`/`fixture_honesty_law`) and
`schema_spec_registration_resolves`, all green on first real run — no iteration needed after the
`Block::Chain` reservation issue above was caught and worked around before running tests.

`cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1744 passed, 12 failed, 4 ignored**.
All 12 failures classified by file path, none touching `artifacts::deflate::*`:
`artifacts::bmp::*` (4), `artifacts::gif::standards::v87a::*` (3), `artifacts::jpg::*` (2),
`artifacts::las::*` (2), `artifacts::tiff::*` (1) — every one of these is a sibling artifact
actively mid-edit by a concurrent agent in this SAME FG2 wave (`git status` confirms all 5 have
uncommitted modifications in progress at the time of this run — `bmp`/`gif`/`jpg`/`las`/`tiff` are
explicitly named in this ticket's own repo-rules digest as sibling-wave artifacts to wait-and-retry
on, not chase). This artifact's own wave-start baseline (`cargo build -p semio-s-plugin-stdio --lib
--tests` without deflate's changes) hit a real, transient compile blocker from the same source
(`las`'s own mid-rename fixture file, `🎒️example.pack.semio` missing at the time) — retried after a
short wait per the ticket's own guidance, cleared on its own once the concurrent session's edit
landed, and `cargo check -p semio-s-plugin-stdio --lib` (non-test) was independently confirmed clean
throughout, proving this artifact's own Rust changes were never the blocker.

`bun run ./📜️script.ts policy` → zero lines mentioning `deflate` anywhere in the full breach output
(grepped explicitly) — this artifact's facet files register no `POLICY_GRAMMAR_PARSEABILITY`/
`POLICY_PROTOCOL_PARSEABILITY`/`POLICY_FIXTURE_HONESTY`/`POLICY_LANGUAGE_REGISTRATION`/
`POLICY_STDIO_JSON_TRANSFER_BAN` breaches. (The full run does report many pre-existing,
unrelated `os-state-authority`/`budget` breaches across the rest of the repo — none reference this
artifact's own files.)

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/⚙️engine/🦀️component.rs` —
  added `demo_deflate_snapshot`, `register_schema_specs` (+ wired into `register()`), full 5-role
  `register_pilot_languages`, `mod conformance_laws` (6 tests), updated doc comments. Temporary
  `debug_dump_fixtures` test added then removed.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `DiffCodec::encode_diff`/`decode_diff` upgraded to a real binary frame; added `demo_diff_cases`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `demo_mutation_cases`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
  — rewritten (real hex-dump grammar).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio`
  — rewritten (real RFC1950 layout).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  — rewritten (real op-text grammar).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
  — rewritten (real format/ordinal/opaque-tail layout).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
  — rewritten (real diff grammar).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
  — rewritten (real format/flags/opaque-tail layout).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  regenerated (real `print_dsl` output, incl. mandatory preamble line, previously missing).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📚️examples/🎬️demo/🖼️assets/🗜️example.zz` —
  regenerated (real `encode_deflate_snapshot` output for the new demo value).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` —
  new (real `encode_pack` output for the new demo value; previously did not exist as a genuine
  fixture — this facet had none).

No files outside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/**` and this report were touched.
`📦️glue.rs`, `📜️script.ts`, the SDK traits, schema/dsl/protocol/registry modules, the framework
`🧪️fixture-sweep` graduation list, and `🏪️store` were all left untouched, per the ownership boundary.

## Deviations from the literal brief

- The brief's phrasing ("optional 4-byte DICTID ... present iff FLG's FDICT bit set — real
  conditional presence, use M2's construct") reads as though `Cond` can express a single-bit test.
  Direct reading of `eval_cond`/`CondOp` (`🗣️dsl/📖️grammar/🦀️component.rs:2233-2243`) confirms it
  cannot (only whole-value eq/ne/lt/le/gt/ge against one already-decoded field). Modeled honestly
  instead, per the SAME treatment this program's own prior FG1 wave already gave an identical
  bitmask limitation in xml's diff `flags` byte — filed as `protocol-cond-no-bitmask` in
  `mechanism_gaps` above rather than silently deviating.
- `opbinary_binary_upgraded: false` — confirmed via direct reading that `OpBinary` was already real
  before this wave (P2-W0's own census independently names `deflate` in the "already real" list);
  no Rust change was needed or made for it.
