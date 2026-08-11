# P2-P2 Independent Verification — zip, png protocol pilots

Independent re-verification of the self-reported `p2-p2-zip-report.md` and `p2-p2-png-report.md`.
Nothing taken on faith — every claim below was re-run or re-read directly against disk.

## Method

1. Ran each artifact's own scoped test filter myself.
2. Read the actual committed `.protocol.semio`/`.grammar.semio` files in full and cross-checked
   field offsets/widths/order against the artifact's own real `decode_*`/`encode_*` Rust source
   (not the self-report's prose).
3. Grepped both artifacts for the `print_diff().into_bytes()`/`print_op().into_bytes()` shortcut
   pattern (all hits were inside doc comments describing the *replaced* shortcut, zero live code
   hits).
4. Read `encode_diff`/`decode_diff`/`encode_op`/`decode_op` bodies directly to confirm real
   `ByteWriter`/`ByteReader`/`store::ByteReader` binary framing.
5. Confirmed fixtures on disk: preamble line, hex content sanity (PNG signature bytes / ZIP `PK`
   magic visible in the hex), pack file existence.
6. Grepped `register_pilot_languages`/`LanguageRole::` for the 5-role registration.
7. Grepped both artifacts' new grammar files for the 4 known P1 pitfalls (bare `(...)` grouping in
   live code lines, hand-rolled `hex = {INT|IDENT}*`-shaped productions, reserved-word production
   names, and — for png specifically — residual `le` primitives in the wire-format protocol file).
8. Ran the full crate suite once at the end.

## zip (standard 2.0)

**Tests**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::zip"` → **46 passed, 0 failed, 0
ignored** — reproduced exactly, matches the self-report's own number.

**Real dialect / byte layout — confirmed, with one nuance**:
- `backward eocd magic 0x504B0506 { ... }` — confirmed correct against `find_eocd`
  (⚙️engine/🦀️component.rs:229-241, backward magic scan, unavoidable per the W0 recon's own
  load-bearing decision) and `resolve_central_directory` (:252-281): every declared EOCD field
  offset/width matches exactly (`count`@+8, `total_count`@+10 — the field the code actually reads
  and uses as the central-directory entry count — `cd_size`@+12 u32, `cd_offset`@+16 u32,
  `comment_len`@+20 u16). Correct.
- `jump central_dir_start from cd_offset { }` + `repeat central_directory { ... }` — confirmed
  **field-for-field, byte-for-byte** against `decode_zip`'s central-directory read loop
  (:296-341): `version_made_by`@+4, `version_needed`@+6, `flags`@+8, `method`@+10, `dos_time`@+12,
  `dos_date`@+14, `crc32`@+16, `comp_size`@+20, `uncomp_size`@+24, `name_len`@+28, `extra_len`@+30,
  `comment_len`@+32, `disk_start`@+34, `internal_attrs`@+36, `external_attrs`@+38, `local_off`@+42
  — all 15 fixed fields match exactly, 46-byte fixed header confirmed. This is the block that
  matters most (it's what the M2/W0 "decided design" — EOCD backward-scan + jump to `cd_offset` —
  actually targeted) and it is genuinely correct.
- `repeat entries { ... }` (block 1, forward walk over local headers from byte 0, terminating on
  the first central-directory tag) — **this block does not correspond to any traversal
  `decode_zip` actually performs.** Read `decode_zip` directly (:286-435): it never forward-scans
  local headers from byte 0 at all. It is entirely central-directory-driven — it reads the CD
  first (via the backward-scan + `cd_offset` jump), and for each CD record performs a **separate
  backward jump to `local_off`**, where it reads only 4 fields (`flags`@+6, `method`@+8,
  `name_len`@+26, `extra_len`@+28) to locate the payload and cross-check the method — it takes
  `version_needed`/`dos_time`/`dos_date`/`crc32`/`comp_size`/`uncomp_size` from the **central
  directory's own copies**, never re-reading them from the local header. Block 1's arm nonetheless
  declares all 8 of those as if read at the local header's fixed offsets. The field *widths and
  order* are individually correct (this is genuinely the real ZIP local-file-header format per
  spec — 30 bytes fixed before name, matching the report's own claim), and the block does
  correctly consume real fixture bytes end-to-end (verified: `protocol_walk_law` passes against
  real `encode_pack` output, and `cargo test` reproduces 46/0 with this exact file on disk) — so
  it is not a *functional* defect, but the report's specific claim that this was "cross-checked
  byte-for-byte against the real offsets `decode_zip` reads at" overstates it for this block:
  `decode_zip` never reads most of those fields from that location at all. This is a genuine, if
  minor, prose/documentation overclaim, not a broken parse or a wrong wire-format description.
- `arm/until` sentinel handling, tag-literal BE encoding (`0x504B0304` = bytes `50 4B 03 04`) —
  confirmed correct by reading `parse_tag_value` usage and the passing `protocol_walk_law` test.
- All fields plain (non-`Be`) `u16`/`u32` — confirmed correct: ZIP is genuinely all-LE, grepped for
  any `be` primitive in zip's protocol files — none found.

**Binary frame confirmed real**: `OpCodecs::encode_op`/`decode_op` — read directly, pure forward
to `dsl::variants_binary::encode_op`/`decode_op` (already real pre-wave, confirmed unchanged).
`DiffCodec::encode_diff`/`decode_diff` (🔺️diff/🦀️component.rs:664-690) — read directly: real
`store::ByteReader`, `write_str_lp`/`read_str_lp`, `enc_entries_diff_bin`/`dec_entries_diff_bin`,
zero `print_diff().into_bytes()` in live code (grep hit only inside its own doc comment describing
the *replaced* shortcut). Confirmed genuinely upgraded.

**Fixtures real**: `🗣️example.dsl.semio` starts with `semio stdio.zip.dsl v1` followed by hex
beginning `504b0304...` (real `PK\x03\x04` local-header magic, visible in the hex) — genuine, not
a fake. `🎒️example.pack.semio` exists (374 bytes).

**Registration**: grepped `register_pilot_languages` in `⚙️engine/🦀️component.rs` — 5
`dsl::register_language` calls, `LanguageRole::{Document,Ops,Diff,Pack,Spr}` all present. Confirmed.

**P1 pitfalls**: grepped all 3 new grammar files for bare `(...)` grouping in live (non-comment)
lines — zero hits. Grepped for a hand-rolled `hex = ...` production — zero hits, every `hex`
reference is a bare macro-fallback ident. Grepped production names against the 5 reserved words —
none collide (`comment-part` etc. are compound idents, not bare `comment`). Confirmed avoided.

## png (standard 1.2)

**Tests**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::png::standards::v1_2"` → **29
passed, 0 failed, 0 ignored** — reproduced exactly, matches the self-report's own number.
(A broader `"artifacts::png"` filter also picks up 4 unrelated tests from the `artifacts::semio`
image-serializer subtree that happen to substring-match "png" in their path — not part of this
pilot's own scope, all pass too, not counted against the pilot's own 29.)

**Real dialect / byte layout — confirmed**: `framing magic 0x89504E470D0A1A0A` is the real 8-byte
signature (matches `PNG_SIGNATURE` in ⚙️engine/🦀️component.rs:20). `repeat chunks { tag fixed 4
length u32be order length-first trailer u32be until "IEND" arm ... }` — the repeated
tag-dispatched-block construct is genuinely used, matching `read_chunks` (:42-77): length-BE,
4-byte type tag, CRC32-BE trailer, `IEND`-terminated loop — all confirmed field-for-field.
`IHDR` arm (`width u32be height u32be bit_depth u8 color_type u8 compression u8 filter u8
interlace u8`) — confirmed **exactly** against `parse_ihdr` (:89-124): same 7 fields, same order,
same 13-byte fixed width. Spot-checked `gAMA`/`cHRM`/`sRGB`/`pHYs`/`tIME` similarly plausible
(fixed widths match the PNG spec section numbers cited in the file's own comments). `PLTE`/`tRNS`/
`bKGD`/`tEXt`/`zTXt`/`iTXt`/`IDAT` are honest opaque arms (empty field lists, auto-skipped via the
`length`-based mechanism) — correctly and explicitly documented as such, not silently faked as
structured.

**BE-prim-throughout check**: grepped the snapshot protocol file for any `le`-suffixed primitive —
zero hits (the only "le" substring match is inside a comment describing the *framework-level
envelope's* own `u32le` token-length field, which this file correctly does NOT re-describe, per
M3's documented boundary). The mutations protocol file (a separate, artifact-internal op-binary
*frame* — not the PNG wire format) legitimately declares plain (LE) `u32`/`u8` fields for its own
free-standing binary design, and this was cross-checked against the actual Rust
`write_u32_le`/`write_u8` calls in `RealBinaryOpFrame` — consistent. This is a different, new
binary format (the op-frame), not part of the real PNG file format, so LE here is not a
"residual-LE-that-should-be-BE" bug.

**Binary frame confirmed real**: `PngMutation::encode_op` (🧬️mutations/🦀️component.rs:341-419) —
read directly: real `dsl::ByteWriter`, `w.write_u8(tag)` + per-variant real field writes
(`write_u32_le`/`write_u8`) and reused `diff::write_bin_*` helpers for nested payloads, terminal
`Ok(w.into_bytes())` — this is `ByteWriter::into_bytes()`, not `print_op().into_bytes()`.
`PngDiff::encode_diff` (🔺️diff/🦀️component.rs:1952-1983) — read directly: real per-field
`write_bin_option`/`write_bin_tri_flag` calls covering every `PngDiff` struct field, terminal
`Ok(w.into_bytes())`. Grep for the literal `print_diff().into_bytes()`/`print_op().into_bytes()`
pattern in live code — zero hits (only inside doc comments describing the replaced shortcut).
Confirmed genuinely upgraded, both codecs.

**Fixtures real**: `🗣️example.dsl.semio` starts with `semio stdio.png.dsl v1` followed by hex
beginning `89504e470d0a1a0a...` (the real PNG signature bytes, visible in the hex) — genuine.
`🎒️example.pack.semio` exists (317 bytes).

**Registration**: grepped `register_pilot_languages` in `⚙️engine/🦀️component.rs` — 5
`dsl::register_language` calls, `LanguageRole::{Document,Ops,Diff,Pack,Spr}` all present. Confirmed.

**P1 pitfalls**: grepped all 3 new grammar files for bare `(...)` grouping in live lines — zero
hits. Grepped for a hand-rolled `hex = ...` production — zero hits, only bare `hex` macro
references. Grepped production names against the 5 reserved words — none collide. Confirmed
avoided.

## Full crate suite

```
$ cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1657 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.68s
```

**0 failures**, full green — better than either self-report's own snapshot (both reported 1654
passed / 1 failed, the 1 failure being `artifacts::semio::...pdf::v1_7::...
real_byte_round_trip_through_pdf_codec`, explicitly flagged by both self-reports as unrelated
concurrent `🧿️semio` churn). That specific test now passes in my run — consistent with this being
a live shared tree where the concurrent session's own churn resolved itself between then and now
(per this program's own "poll rather than chase" rule), not something either pilot needed to fix.
1657 vs. 1654 passed (3 more) is also consistent with ongoing concurrent additions elsewhere in the
tree (e.g. the in-progress GIF 89a work visible in `git status`) — zero relation to zip/png. No
failures anywhere in the crate at verification time.

## Summary table

| artifact | tests_passed | tests_failed | real_dialect_confirmed | binary_frame_confirmed | fixtures_real | registration_confirmed | p1_pitfalls_avoided |
|---|---|---|---|---|---|---|---|
| zip | 46 | 0 | yes (1 minor prose-overclaim noted, not a functional defect — see notes) | yes | yes | yes | yes |
| png | 29 | 0 | yes | yes | yes | yes | yes |

## Notes / findings for the orchestrator

1. **zip, minor**: the snapshot protocol's block 1 (`repeat entries { ... }`, forward walk over
   local file headers from byte 0) does not correspond to any traversal `decode_zip` actually
   performs — the real decoder is entirely central-directory-driven with per-entry backward jumps
   to `local_off`, and only reads 4 of the 8 fields block 1 claims to read from the local header
   location (the other 4 — `version_needed`/`dos_time`/`dos_date`/`crc32`/`comp_size`/
   `uncomp_size` — are actually sourced from the central directory's own redundant copies).
   Block 1's field widths/order are nonetheless spec-accurate (genuine ZIP local-file-header
   format) and the block does correctly, functionally consume real archive bytes (verified by the
   passing `protocol_walk_law`/full test suite against real fixtures) — so this is a
   documentation/characterization nuance in the self-report's "cross-checked byte-for-byte against
   the real offsets `decode_zip` reads at" claim, not a parse failure, wrong-layout bug, or test
   regression. Not blocking; recommend a follow-up comment fix in the protocol file itself
   clarifying that block 1 models the ZIP spec's local-header layout directly (which happens to be
   real bytes on disk) rather than mirroring `decode_zip`'s own field-source choices.
2. No other discrepancies found between either self-report and the actual committed state. Both
   pilots' own claimed test counts, binary-frame upgrades, fixture authenticity, 5-role
   registration, and P1-pitfall avoidance were all independently reproduced from disk, not taken
   on faith.
