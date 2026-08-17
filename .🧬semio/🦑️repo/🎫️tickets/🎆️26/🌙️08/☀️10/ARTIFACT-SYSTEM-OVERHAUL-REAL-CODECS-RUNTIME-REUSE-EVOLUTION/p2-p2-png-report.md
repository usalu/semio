# P2-P2 — `stdio.png` (standard 1.2) — Real Protocol, Real Grammar, Real Binary Codecs

Status: COMPLETE. `cargo test -p semio-s-plugin-stdio --lib "artifacts::png::standards::v1_2"`
→ **29 passed, 0 failed, 0 ignored**. Full crate: `cargo test -p semio-s-plugin-stdio --lib` →
**1654 passed, 1 failed, 1 ignored** — the 1 failure
(`artifacts::semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::pdf::v1_7::any::component::tests::real_byte_round_trip_through_pdf_codec`)
is inside `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/...` — a completely different artifact
(the concurrent `🧿️semio` wave explicitly flagged in this program's own repo-rules digest),
reproduced identically across two consecutive full-crate runs (`"hellosemio"` vs `"hello\nsemio"`,
a newline-handling bug in that artifact's own PDF text serializer), zero connection to any
`📷️png` file. The 1 ignored test (`artifacts::csv::standards::v_rfc4180::engine::tests::
zzz_generate_p2p1_fixtures`) is a pre-existing csv fixture-generator from that pilot's own P2-P1
wave, not touched this session.

## 1. What changed, file by file

All six `.grammar.semio`/`.protocol.semio` files live under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/`.

### 1a. `📸️snapshot/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real M1-dialect hex-dump grammar (`dialect grammar` / `grammar stdio.png.snapshot` /
`extension png` / `start document`), replacing the old one-line `dialect grammar stdio.png.snapshot`
+ ABNF-flavored header (unparseable by the real dialect). `document = envelope-mark hex-body`,
`envelope-mark = "stdio.png"` matches EXACTLY what `dsl_body_from_fixture` reconstructs after
stripping the preamble, and `hex-body = hex` uses the framework's built-in `hex` macro (not a
hand-rolled `{INT | IDENT}*` production — the P2-P1 fix report's own documented rule). This is
PNG's own honest precedent confirmed directly against the real codec
(`📸️snapshot/🦀️component.rs`'s `ArtifactDsl` impl): `print_dsl`/`parse_dsl` hex-encode/decode the
REAL binary PNG bytes verbatim — PNG has no textual syntax of its own, so a hex-dump grammar is
the accurate model, not a shortcut.

### 1b. `📸️snapshot/💾️binary/📡️component.protocol.semio` (REWRITTEN — the main deliverable)

Real PNG 1.2 §5 byte layout: `framing magic 0x89504E470D0A1A0A` (the real 8-byte signature,
genuinely byte-checked at walk time — not a `header fixed 8` skip) followed by one
`repeat chunks { tag fixed 4  length u32be  order length-first  trailer u32be  until "IEND"
arm ... }` block — exactly M2's own worked PNG example (`p2-m2-report.md` item 1), extended to
every chunk kind `⚙️engine/🦀️component.rs`'s real `decode_png` actually types:

- **Fully structured** (every field genuinely, individually BE-walked): `IHDR` (7 fields, always
  13 bytes), `gAMA` (1 field, 4 bytes), `cHRM` (8 fields, 32 bytes), `sRGB` (1 field, 1 byte),
  `pHYs` (3 fields, 9 bytes), `tIME` (6 fields, 7 bytes).
- **Honest opaque arms** (declared and named — self-documenting, matching the real decoder's own
  typed-but-not-byte-structural handling — but with an empty field list, letting `walk_repeat`'s
  own length-declared auto-skip consume them, the SAME mechanism that skips a genuinely unknown
  chunk type): `PLTE` (variable-length 3-byte-entry run, no named length field to `Array`-count
  against — see `mechanism_gaps`), `tRNS`/`bKGD` (shape depends on `color_type`, and this
  dialect's `Cond` can't chain a second conditional onto a field only conditionally decoded — see
  `mechanism_gaps`), `tEXt`/`zTXt`/`iTXt` (NUL-delimited + optionally zlib-compressed), `IDAT`
  (compressed bitstream, the user's own "opaque segment" carve-out). Genuinely-unrecognized
  ancillary chunks (e.g. `prIV`) fall through the SAME built-in skip with no arm declared at all.

This is the SEMIO-envelope-UNWRAPPED payload (P2-M3 §3/§5 point 4) — `PngSnapshot::
encode_pack_with` is `encode_png(self)` wrapped by `store::semio_format::wrap_binary`, so this
file starts exactly where the envelope's own framework-level protocol file hands off.

### 1c. `🧬️mutations/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real one-line `keyword key=value ...` op-text form ALREADY emitted by
`🧬️mutations/🦀️component.rs`'s `print_png_mutation`/`parse_png_mutation`, replacing the pre-F6
serde-JSON fossil. 17 alternatives (`no-mutation-op` … `remove-unknown-chunk-op`), every keyword/
field-name token copied verbatim from the real `format!(...)` call sites. Shared positional-tuple
value grammars (`rgb-value`/`transparency-value`/`chromaticities-value`/`physical-dims-value`/
`timestamp-value`/`background-value`/`text-chunk-value`/`chunk-value`/`chunk-marker-value`) mirror
`🔺️diff/🦀️component.rs`'s `enc_*`/`dec_*` functions exactly (the SAME primitives the diff grammar
below reuses — both grammars document the SAME shared `hex` macro rationale).

### 1d. `🧬️mutations/💾️binary/📡️component.protocol.semio` (REWRITTEN) + real binary frame

**`OpBinary::encode_op`/`decode_op` upgraded from `print_op().into_bytes()` to a real binary
frame** in `🧬️mutations/🦀️component.rs` (new `RealBinaryOpFrame` region): `tag u8` (hand-assigned
ordinal, declaration order 0–16) + per-variant fields, via `dsl::ByteWriter`/`dsl::ByteReader`,
reusing `🔺️diff/🦀️component.rs`'s own new binary-primitive helpers (`write_bin_option`/
`write_bin_snapshot`/`write_bin_rgb`/`write_bin_transparency`/…) instead of duplicating them —
every genuinely scalar field (the ordinal, `index`, `SetHeader`'s five IHDR fields) is
individually, honestly byte-walked; every nested/tri-state/enum-shaped payload is a length-
consuming opaque `bytes` tail, matching the protocol file's own `arm N { ... bytes }` shape
exactly (deliberately placed last per arm, csv's own precedent).

### 1e. `🔺️diff/📝️text/📖️component.grammar.semio` (REWRITTEN)

Real one-line `print_diff`/`parse_diff` form from `🔺️diff/🦀️component.rs`'s `print_png_diff`/
`parse_png_diff` — a space-separated `name=value` token per changed top-level scalar/tri-state
field, `text-chunks`/`chunk-order`/`unknown-chunks` as the real removed/modified/added
COLLECTION-TRIPLE (`name{[removed];[modified];[added]}`, the csv-pilot-established copy-pasteable
shape), `plte`'s OUTER tri-state wrapping the SAME bare triple body inside `encode_option`'s
uniform `[0]`/`[1,…]` tag. Genuinely index-keyed (PNG explicitly permits duplicate `tEXt`
keywords, matching `PngTextChunk`'s own doc comment on why index, not keyword, is the diff key).

### 1f. `🔺️diff/💾️binary/📡️component.protocol.semio` (REWRITTEN) + real binary frame

**`DiffCodec::encode_diff`/`decode_diff` upgraded from `print_diff().into_bytes()` to a real
binary frame** (`🔺️diff/🦀️component.rs`'s new `RealBinaryDiffFrame`/`RealBinaryPrimitives`
regions): one presence-flag byte per field, field-for-field in `PngDiff`'s own struct order.
Plain `Option<T>` fields use a 2-way flag (`0`=unchanged, `1`=changed); TRI-STATE
`Option<Option<T>>` fields (`plte`/`trns`/`gama`/`chrm`/`srgb`/`phys`/`time`/`bkgd`) use a 3-way
flag (`0`=unchanged, `1`=cleared-to-`None`, `2`=set-to-`Some(value)`) — a genuinely new design
choice this wave had to make (not precedented by csv, whose own `CsvDiff` has no tri-state
fields): chaining two `if`-guarded conditional fields (an outer presence flag, then an inner
value-presence flag) would need the inner field's `Cond` to reference a field that was ITSELF
only conditionally decoded, and `eval_cond` unconditionally errors ("condition references
unknown field") when its guarded field was never read — confirmed by reading `eval_cond`/
`walk_fields` directly, not assumed. The single flat 3-way flag sidesteps this entirely. Fixed-
width nested values (`gama`/`chrm`/`srgb`/`phys`/`time`) are modeled as REAL, individually-walked
structured fields (every `PngChromaticities`/`PngPhysicalDims`/`PngTimestamp` sub-field gets its
own conditional field, gated on the SAME always-read outer flag); genuinely variable-length
payloads (`plte`/`trns`/`bkgd`/`text_chunks`/`pixels`/`chunk_order`/`unknown_chunks`) are length-
prefixed opaque byte blobs (`<name>_len varint` + `Array(u8, Field(<name>_len))`).

## 2. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`: was an 11-byte fake (`68656c6c6f`, hex for
  `"hello"`, no preamble). Now the genuine `print_dsl(demo_png_snapshot())` output (599 bytes),
  WITH the mandatory `semio stdio.png.dsl v1` preamble line.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`: did not exist before. Now the genuine
  `encode_pack(demo_png_snapshot())` bytes (317 bytes — real SEMIO binary envelope + the real
  encoded PNG file bytes).
- `demo_png_snapshot()` (new, `⚙️engine/🦀️component.rs`) is the single source of truth for both
  fixtures. It is DELIBERATELY safe against `encode_png`'s own canonicalization
  (`bit_depth`/`color_type`/`interlace` set to exactly `8`/`Rgba`/`false`, the values
  `encode_png` always hardcodes regardless of the snapshot's own fields — see `encode_png`'s own
  `EncodeScopeNote`); `trns` is deliberately `None` (a `tRNS` chunk decoded under
  `color_type == 6` is spec-mandated to be IGNORED — confirmed by reading `decode_png`'s own
  `_ => {}` arm — so no other value could ever round-trip); `bkgd` uses the `Rgb` variant
  specifically (the only variant whose 6-byte wire shape matches what `color_type == 6` expects,
  `2|6 => 6 bytes`). Exercises `PLTE` (3 entries), every typed ancillary chunk (gAMA/cHRM/sRGB/
  pHYs/tIME/bKGD), one text chunk, one verbatim-retained unknown ancillary chunk (`prIV`), and a
  3×3 non-solid raster, in a real relative chunk order that survives encode→decode exactly
  (verified: the physical byte order `encode_png` writes reproduces the IDENTICAL `chunk_order`
  marker sequence on decode).
- **Regeneration method** (documented, not silently improvised): PNG's real codec depends on this
  artifact's own hand-rolled deflate compressor (`crate::artifacts::deflate::engine::
  zlib_compress`), which — unlike json/csv's pure-text formats — cannot be hand-derived or
  cross-checked in Python byte-for-byte. The fixtures were generated by running the REAL Rust
  `print_dsl`/`encode_pack` functions directly: a temporary `#[ignore]`d test
  (`generate_real_fixtures`, in `⚙️engine/🦀️component.rs`'s `conformance_laws` module) wrote both
  files via `std::fs::write`, run once via `cargo test … -- --include-ignored`, confirmed correct
  by `fixture_honesty_law`, then DELETED from the source (not left behind — CLAUDE.md's "no
  migration scripts" rule, and matching this wave's own "temp files in the ticket folder, never
  committed to the artifact" boundary).

## 3. Conformance tests (own test region — `⚙️engine/🦀️component.rs`'s new `conformance_laws`
module, nested inside the pre-existing `mod tests`)

- `committed_facet_files_parse` — all 6 files parse under `dsl::parse_grammar`/`dsl::parse_protocol`.
- `grammar_conformance_law` — snapshot grammar recognizes real `print_dsl` output (preamble-
  stripped body reconstruction, matching `m5_handcrafted_grammar_conformance`'s own
  `dsl_body_from_fixture`).
- `ops_grammar_conformance_law` — mutations grammar recognizes real `print_op` output for every
  `PngMutation` variant (`mutations::demo_mutation_cases()`, moved to module scope this wave —
  see §4).
- `diff_grammar_conformance_law` — diff grammar recognizes real `print_diff` output for every
  representative `PngDiff` (`diff::demo_diff_cases()`, new this wave).
- `protocol_walk_law` — `walk_protocol` against real `encode_pack` (envelope-unwrapped), every
  demo `encode_op`, and every demo `encode_diff`, asserting `consumed == bytes.len()`.
- `fixture_honesty_law` — see §2.

All 6 pass (§ verification below), plus the 23 pre-existing engine/mutations/diff tests
(codec round-trips, `mutation_diff_law`/`inverse_law`/`absorb_law`/`absorb_law_associativity`/
`field_sweep_covers_every_mutable_field`/`op_text_binary_roundtrip_law`/
`diff_codec_text_binary_roundtrip_law`, all of Phase 1's own color-type/ancillary/Adam7 fixtures)
— zero regressions.

## 4. Real-binary-frame + reuse refactor (single source of truth, per CLAUDE.md)

- `🔺️diff/🦀️component.rs` gained two new regions: `RealBinaryPrimitives` (per-value binary
  codecs — `write_bin_str`/`write_bin_blob`/`write_bin_rgb`/`write_bin_transparency`/
  `write_bin_chromaticities`/`write_bin_physical_dims`/`write_bin_timestamp`/
  `write_bin_background`/`write_bin_text_chunk`/`write_bin_chunk`/`write_bin_chunk_marker`/
  `write_bin_option`/`write_bin_vec`/`write_bin_snapshot`, all `pub(crate)`, plus their `read_bin_*`
  counterparts) and `RealBinaryDiffFrame` (the four collection-triple binary encoders +
  `write_bin_tri_flag`/`read_bin_tri_flag`). `🧬️mutations/🦀️component.rs`'s own `OpBinary` impl
  reuses ALL of these directly (`diff::write_bin_snapshot(&mut w, snapshot)` for `SetSnapshot`,
  etc.) rather than re-deriving a second binary encoding — same intra-artifact reuse direction the
  pre-existing TEXT codecs (`enc_str`/`enc_rgb`/…) already establish.
- `demo_mutation_cases()`/`demo_base_snapshot()` (mutations) and `demo_diff_cases()`/
  `demo_snap_a()`/`demo_snap_b()`/`demo_empty_snap()` (diff) moved to module scope
  (`#[cfg(test)] pub(crate)`), single source of truth for both the artifact's own pre-existing
  tests AND the engine's new `conformance_laws` — the pre-existing `mod tests`' own
  `base_snapshot()`/`text_chunk()`/`all_variants()` are now thin aliases (`fn base_snapshot() ->
  PngSnapshot { demo_base_snapshot() }`) rather than duplicate literal fixtures, so the many
  ad-hoc call sites inside the pre-existing test suite (`absorb_law` etc.) didn't need touching.

## 5. Registration (`⚙️engine/🦀️component.rs`'s `register_pilot_languages`, new)

5-role `LanguageSpec` registration added, per `stdio.note`'s exemplar pattern: `stdio.png`
(Document, grammar+protocol = snapshot text/binary), `stdio.png.op` (Ops, grammar+protocol =
mutations text/binary — NEW), `stdio.png.diff` (Diff, grammar = diff text, protocol = `None` —
matching the exemplar's own shape, the 5-role scheme has no dedicated "diff binary" role),
`stdio.png.pack` (Pack, protocol = snapshot binary), `stdio.png.spr` (Spr, protocol = mutations
binary — NEW). All `dsl::passthrough_hooks`. `register()` now calls the new
`register_pilot_languages()` instead of inlining a single `dsl::register_language` call —
previously only 1 role (`stdio.png`) was registered.

`register_schema_spec` (P2-M3's `FullResolver` insertion API) was **not** called — see
`mechanism_gaps` below (same root cause json/csv already documented).

## 6. JSON-transfer elimination check (item 8)

`grep -rn "serde_json::to_vec\|serde_json::from_slice\|serde_json::to_string\|serde_json::from_str\|serde_json::Value" ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/` → zero hits. Confirmed clean,
matching W0's own finding that png was not flagged as a literal-JSON-transfer violation.
`ArtifactPack`/`OpBinary`/`DiffCodec` all genuinely binary end to end.

## 7. Verification

```
$ cargo test -p semio-s-plugin-stdio --lib "artifacts::png::standards::v1_2"
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 1627 filtered out; finished in 0.04s

$ cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 1654 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.7-8.0s
(the 1 failure and 1 ignored are both outside 🗿️artifacts/📷️png — see status line above; confirmed
stable/reproducible-but-unrelated across two consecutive full-crate runs)
```

`cargo check -p semio-s-plugin-stdio --lib` also verified clean before adding tests (no errors,
only the repo's own ambient warning baseline — `hidden lifetime parameters` on `&mut
dsl::ByteReader` params, matching csv's own documented cosmetic baseline exactly).

### A real bug caught and fixed during this wave (not left for the reader to find)

The mutations grammar's `op = ...` alternation and the diff grammar's `diff = ...` optional-
clause sequence were FIRST drafted wrapped across multiple physical source lines for readability.
`committed_facet_files_parse`/`ops_grammar_conformance_law`/`diff_grammar_conformance_law` caught
this immediately (`"expected Ident, found Pipe"` / `"expected Equals, found Question"`) —
confirming the P2-P1 fix report's own documented rule in a REAL, previously-undiscovered instance
(neither json nor csv's own productions happened to need a 17-alternative or 17-optional-clause
line long enough to tempt a multi-line wrap): `parse_alternatives`'s `while cursor.peek().kind ==
GKind::Pipe` check does not skip a leading `Newline` token, so a continuation line's leading `|`
is never reached — the production silently truncates at the first line break. Fixed by collapsing
both productions to one physical line each (confirmed via `cat -n` grep sweep of every production
in all three new grammar files, that no other production wraps).

## Deviations

- Grammar files omit an explicit `ws`/whitespace production — lexer trivia is stripped before
  matching, matching json/csv's own documented reasoning.
- Op/diff protocol files model only genuinely fixed/scalar fields plus opaque `bytes` tails for
  nested/tri-state/enum payloads — see `mechanism_gaps` (`protocol-prim-ref-recursion`). The Rust
  `encode_op`/`decode_op`/`encode_diff`/`decode_diff` implementations ARE fully, genuinely
  structured recursive/nested real binary.
- The snapshot protocol facet describes the SEMIO-envelope-UNWRAPPED payload only, matching M3's
  documented mechanism boundary.
- `stdio.png.diff`'s `LanguageSpec.protocol` is `None`, matching note's own 5-role exemplar shape
  exactly, even though a real, conformance-tested diff protocol file exists (exercised directly by
  `protocol_walk_law` instead of through a `LanguageRole`).
- `register_schema_spec` not called for `stdio.png`/`stdio.png#diff` — see `mechanism_gaps`.
- PNG's `tRNS`/`bKGD` chunks are typed by the real Rust decoder but modeled as one honest opaque
  arm each in the protocol file (not per-`color_type`-branch structured fields) — see
  `mechanism_gaps` (`protocol-cond-cannot-chain`), a genuinely new gap this wave surfaced that
  neither json nor csv's own simpler diff shapes (no tri-state-inside-tag-dispatch combination)
  had reason to hit.

## Mechanism gaps

1. **`protocol-cond-cannot-chain`** — engine area: `dsl::grammar::protocol` (`eval_cond`/
   `walk_fields`). Symptom: `Cond` can gate at most one UNCONDITIONALLY-decoded field; chaining a
   second `if`-guarded field onto one that was ITSELF only conditionally decoded (e.g. "read a
   variant tag only if an outer flag says the value is present, then dispatch further fields on
   that tag") makes `eval_cond` hard-error ("condition references unknown field") whenever the
   outer condition was false, since the inner field's name was never inserted into `WalkState.env`.
   Confirmed by reading `eval_cond`/`walk_fields` directly (`📖️grammar/🦀️component.rs`), not
   assumed. Blocks describing PNG's `tRNS`/`bKGD` chunk bodies (whose real shape depends on the
   earlier-decoded `color_type`) as per-branch structured fields inside the protocol dialect.
   Worked around locally: modeled both as one honest opaque arm each (the real Rust decoder still
   fully types them — this is purely a protocol-DESCRIPTION depth limit); the diff protocol file's
   own tri-state fields use a single flat 3-way flag specifically to AVOID hitting this same wall
   (documented in that file's own header comment as the reason for that design choice). Non-
   blocking.
2. **`protocol-repeat-length-not-named`** — engine area: `dsl::grammar::protocol`
   (`walk_repeat`'s `DispatchOrder::LengthFirst` read). Symptom: a `repeat` block's own `length`
   directive value is used internally for the auto-skip/overrun check but is never bound to a
   named field in `WalkState.env` (unlike an ordinary `field <name> <ty>` inside `walk_fields`,
   which always inserts its name) — so an arm's OWN fields cannot reference "the declared length
   of THIS chunk" via `Array(_, Field(name))` to walk a variable-count run whose count IS that
   length (PNG's `PLTE`, `length / 3` RGB entries). Worked around locally: `PLTE` is one honest
   opaque arm (auto-skipped via the SAME length-based mechanism a genuinely unrecognized chunk
   type gets, just explicitly named for self-documentation). Non-blocking.
3. **`protocol-prim-ref-recursion`** — engine area: `dsl::grammar::protocol` (`walk_protocol`,
   `Prim::Ref` arm). Symptom: `Prim::Ref` unconditionally errors during `walk_protocol` (confirmed
   unchanged since P2-M2/json's/csv's own identical finding), so a nested struct/enum payload
   (`PngSnapshot` inside `SetSnapshot`, `PngTransparency`/`PngBackground` inside their own
   mutation/diff fields, the four collection-triple diff types) cannot be described field-by-field
   in the protocol dialect. Worked around locally exactly like csv's own precedent: every such
   payload is one opaque trailing `bytes` (mutations, always the last field in its arm) or
   length-prefixed `Array(u8, Field(<name>_len))` blob (diff, since the diff frame has MULTIPLE
   such payloads and only the true last one could safely use bare `bytes`). The Rust encode/decode
   side IS genuinely, fully structured recursive real binary — round-trip tested independently
   (`diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law`). Non-blocking.
4. **`register-schema-spec-needs-recordspec`** — engine area: `dsl::registry::register_schema_spec`
   / `FullResolver`. Symptom: requires `fn() -> RecordSpec`; `PngSnapshot`'s `ArtifactDsl`/
   `ArtifactPack` are hand-rolled (the real payload is a hex-dumped/PNG-structured binary byte
   stream, not a `dsl`-derivable record shape — same root cause json's/csv's own reports already
   document for their own hand-rolled types). Worked around locally: skipped the call rather than
   fabricate an unrelated `RecordSpec`. Non-blocking.

## Files touched

- `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` (rewritten)
- `🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` (rewritten)
- `🧬️schema/🧬️mutations/🦀️component.rs` (real binary op frame, `RealBinaryOpFrame` region;
  `demo_mutation_cases()`/`demo_base_snapshot()`/`demo_text_chunk()` moved to module scope;
  `mod tests`'s own `base_snapshot`/`text_chunk`/`all_variants` now thin aliases)
- `🧬️schema/🔺️diff/🦀️component.rs` (real binary diff frame + shared binary primitives,
  `RealBinaryPrimitives`/`RealBinaryDiffFrame` regions; `demo_diff_cases()`/`demo_snap_a()`/
  `demo_snap_b()`/`demo_empty_snap()` moved to module scope; `handcrafted_diff_codec_tests`'s own
  round-trip test now calls `demo_diff_cases()` instead of duplicating the case list)
- `⚙️engine/🦀️component.rs` (`demo_png_snapshot()`, `register_pilot_languages()` (5 roles),
  `conformance_laws` test module, unused `PngDiff` import removed)
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (regenerated, real)
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new, real)
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-p2-png-report.md`

No file outside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/**` and this report was touched.
`📦️glue.rs`/`📜️script.ts`/the SDK traits/the schema/dsl/protocol/registry modules/`🏪️store` were
never edited (confirmed by `git status --porcelain` scoped to those paths, unchanged by this
session).
