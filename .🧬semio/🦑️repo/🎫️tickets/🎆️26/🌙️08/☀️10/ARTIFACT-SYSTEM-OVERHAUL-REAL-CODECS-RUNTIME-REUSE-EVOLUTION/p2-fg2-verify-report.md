# P2-FG2 Independent Verification Report

Verifier: independent agent, trusting nothing from FG2's own self-reports — every claim below was
re-derived from disk (real committed `.rs`/`.grammar.semio`/`.protocol.semio` files) and from a
fresh `cargo test` run in this session, not copied from `p2-fg2-*-report.md`.

## Summary

All 9 standards (gif/87a, gif/89a, jpg/jfif-1.01, bmp/v3, tiff/6.0, deflate/rfc1950, las/1.0,
dwg/ac1018, dwg/ac1024) pass their own scoped test filter with 0 failures. The #1 FG1-era
shortfall — `DiffCodec`/`OpBinary` still routing through `print_diff()/print_op().into_bytes()`
text-as-binary — **does NOT recur anywhere in this wave**. Every hand-rolled `encode_diff`/
`decode_diff` and `encode_op`/`decode_op` body was read directly (not assumed) and is a genuine
field-by-field (or tag-dispatched) binary frame; every "already-real" claim (`dsl::variants_binary`
forwarding for `OpBinary`, `#[derive(dsl::DslDiff)]`-generated `.spk`-container codecs for
`DiffCodec`) was confirmed by reading the actual derive target, not taken on faith.

One real gap found, independently confirmed (not just trusted from the self-report): **jpg/jfif-1.01
has zero 5-role `LanguageSpec` registration** (`dsl::register_language`/`dsl::LanguageSpec` — zero
hits anywhere in jpg's tree) and zero `register_schema_spec` calls. This matches what FG2's own jpg
report already flagged as a deviation ("a real, pre-existing gap... left untouched rather than
fabricated under time pressure"), so it is not a hidden defect, but it IS a real, unresolved
per-standard-checklist item that a follow-up wave should close.

Full crate suite: **1773 passed, 0 failed, 1 ignored** (up from the recipe's own ≥1714/0/1-ignored
floor — consistent with concurrent sibling-wave work landing in the same live tree during this
session, not a regression).

## Per-standard detail

### gif/87a
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::standards::v87a"` → all passing,
  including all 6 conformance laws run in isolation
  (`artifacts::gif::standards::v87a::engine::tests::conformance_laws::*`, 6/6 ok).
- `DiffCodec::encode_diff`/`decode_diff` (`🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:875-902`):
  real flag-per-field binary frame (`write_bin_option`/`write_bin_tri_flag` for the tri-state
  `gct`), confirmed by direct read — not text-as-binary.
- `OpBinary::encode_op`/`decode_op` (`🧬️mutations/🦀️component.rs:224-231`): pure forward to
  `dsl::variants_binary::encode_op`/`decode_op` — provably already-real, confirmed by reading the
  body (not assumed from a "derives cleanly" claim).
- Protocol file (`📸️snapshot/💾️binary/📡️component.protocol.semio`) honestly documents a NEW,
  live-confirmed mechanism gap (`protocol-framing-magic-fixed-8-bytes`): `framing magic` always
  reads/compares exactly 8 bytes (verified against `magic_bytes(value: u64) -> [u8; 8]`,
  `🧰️framework/…/📖️grammar/🦀️component.rs:983`), but GIF87a's real magic is 6 bytes — worked
  around honestly with `framing record` + individually-walked, protocol-layer-unvalidated
  `field magic fixed 6`. Header fields (magic/width/height/packed/bg/par, 13 bytes) checked
  field-for-field against `decode_gif` (`⚙️engine/🦀️component.rs:469-476`) — exact match.
- Grammar file uses png's own honest hex-dump precedent correctly (bare `hex` macro, not a
  hand-rolled `{INT|IDENT}*`).
- Fixtures real: `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio` present
  and non-trivial; `📚️examples/💃️dancing/🖼️assets/🖼️dancing.gif` real fixture, exercised by
  `decodes_real_fixture_with_nontrivial_invariants`/`decode_encode_decode_round_trip_is_stable`
  (both pass).
- Registration: `register_pilot_languages()` (5-role, all `dsl::passthrough_hooks`) +
  `register_schema_specs()` (`"stdio.gif"` → `GifSnapshot::__dsl_spec`) confirmed present and
  called from `register()`.
- Pitfalls: grepped for bare-paren grouping, hand-rolled hex productions, reserved-keyword
  collisions, multi-line alternation, unhandled `Prim::Ref` recursion — zero hits.

### gif/89a
- `cargo test … "artifacts::gif::standards::v89a"` → all passing (part of the same 75-test gif
  run), including all 6 conformance laws.
- `DiffCodec::encode_diff`/`decode_diff` (`🔺️diff/🦀️component.rs:1304-1337`): real, same
  flag-per-field shape as 87a, plus the tri-state `loop_count` field. Confirmed real by direct
  read.
- `OpBinary` (`🧬️mutations/🦀️component.rs:332-339`): pure forward to `dsl::variants_binary`,
  confirmed real.
- Protocol file documents the SAME `protocol-framing-magic-fixed-8-bytes` gap plus a genuinely
  NEW, correctly-identified one: cross-block state carry (GCE fields consumed by a later,
  different block) and packed-byte bitfield→bytelength computation — both correctly left as an
  honest opaque `chain rest bytes` past the 13-byte header, with the reasoning spelled out
  in-file rather than silently skipped.
- Fixtures, registration (`register_schema_specs` → `"stdio.gif.89a"`), pitfalls: all clean, same
  as 87a.

### jpg/jfif-1.01
- `cargo test … "artifacts::jpg"` → 40 passed, 0 failed, all 6 conformance laws included.
- `DiffCodec::encode_diff`/`decode_diff` (`🔺️diff/🦀️component.rs:1635-1701`): real 16-bit-flags +
  fixed-order-fields binary frame, confirmed real by direct read — genuinely no opaque tail (jpg's
  diff payloads are non-recursive).
- `OpBinary::encode_op`/`decode_op` (`🧬️mutations/🦀️component.rs:381-440`): real tag-dispatched
  binary frame (12 variants, `out[1] = <tag>` then per-variant field encoding), confirmed real.
- Protocol file (`📸️snapshot/💾️binary/📡️component.protocol.semio`) is a genuinely careful,
  marker-scan-based (`tag marker(0xFF)`) real byte layout, cross-checked against the actual
  encoder's marker bytes (`0xE0`/`0xDB`/`0xC0`/`0xC4`/`0xDD`/`0xDA`) — confirmed by grep against
  `⚙️engine/🦀️component.rs`. Two new, honestly-documented, non-blocking mechanism gaps
  (`protocol-repeat-length-inclusive-convention`, `protocol-array-count-arithmetic`), both
  correctly sidestepped by the artifact's own conformance fixture rather than silently mismodeled.
- **Gap confirmed**: zero `LanguageSpec`/`register_language` hits anywhere in jpg's tree (grepped
  independently) — no 5-role registration exists, and no `register_schema_spec` call exists
  either. This matches the jpg self-report's own "Deviations" section, which explicitly names this
  as a known, deliberately-left-open gap ("not part of this wave's specific brief... recorded here
  for the next agent/closer"). Confirmed genuine, not fabricated evasion — but it is a real,
  outstanding checklist item.
- Fixtures real, including `example.jpg` (previously 0 bytes per F3b's own report, now 801 bytes
  of real `encode_jpg` output — verified on disk).
- Pitfalls: none found by grep.

### bmp/v3
- `cargo test … "artifacts::bmp"` → 25 passed, 0 failed, all 6 conformance laws included, plus a
  `schema_spec_registration_resolves` test.
- `BmpDiff` derives `#[derive(..., dsl::DslDiff)]` (`🔺️diff/🦀️component.rs:257`) — no hand-rolled
  `DiffCodec` impl exists (confirmed: zero `fn encode_diff`/`impl DiffCodec` hits anywhere in
  bmp's diff tree) because the derive macro generates one, routing through the real
  `.spk`-container path. This is the "provably already-real" case, confirmed by finding the derive
  attribute directly, not assumed.
- `OpBinary` (`🧬️mutations/🦀️component.rs:236-243`): pure forward to `dsl::variants_binary`,
  confirmed real.
- Protocol file: real BITMAPFILEHEADER + BITMAPINFOHEADER-core field-by-field layout, confirmed
  against `decode_bmp`; correctly documents the same `protocol-magic-shorter-than-8-bytes` gap
  independently (2-byte "BM" magic) and two more honestly-scoped gaps (`Cond` field-vs-field,
  `Count::Field`-no-default-fallback), both non-blocking because the real committed fixture is
  24bpp/no-BITFIELDS.
- Registration: full 5-role `LanguageSpec` + `register_schema_spec` for BOTH `"stdio.bmp"` and
  `"stdio.bmp#diff"` (since `BmpSnapshot`/`BmpDiff` both genuinely derive) — confirmed present.
- Pitfalls: none found.

### tiff/6.0
- `cargo test … "artifacts::tiff"` → 39 passed, 0 failed, all 6 conformance laws included.
- `DiffCodec` (`🔺️diff/🦀️component.rs:1029-1069`): real flags-byte + fixed-order-fields frame,
  `ifds` recursing through `enc_ifds_diff_bin`/`dec_ifds_diff_bin` genuinely structured all the
  way down (not text-as-bytes) — confirmed real.
- `OpBinary` (`🧬️mutations/🦀️component.rs:232-270`): real tag-dispatched frame (8 variants),
  confirmed real.
- Protocol file genuinely and correctly uses M2's `endian {...}` construct
  (`field byte_order endian { "II"=le "MM"=be }`), cross-checked against the real
  `decode_tiff`'s own byte-order-mark read (`⚙️engine/🦀️component.rs:344-348`) — exact match.
  Correctly documents the IFD-chain/out-of-line-offset/array-of-records compound gap and stops the
  protocol description at the real, bounded 10-byte header rather than guessing at the rest.
- Registration: full 5-role `LanguageSpec`; `register_schema_spec` deliberately NOT called, with
  an honest, specific, correct reason documented in-file (`TiffValues` is a data-carrying enum
  that blocks `#[derive(dsl::DslRecord)]`/`DslDiff`/`DslOps` — same root cause json/csv/zip/png
  already hit).
- Fixtures real (`example.tiff`, `example.dsl.semio`, `example.pack.semio` present).
- Pitfalls: none found.

### deflate/rfc1950
- `cargo test … "artifacts::deflate"` → 26 passed, 0 failed, all 6 conformance laws included, plus
  a `schema_spec_registration_resolves` test.
- `DiffCodec` (`🔺️diff/🦀️component.rs:305-353`): real flags-byte binary frame, confirmed real.
- `OpBinary` (`🧬️mutations/🦀️component.rs:122-129`): pure forward to `dsl::variants_binary`,
  doc-commented as "Replaces the prior `serde_json` stub" — i.e. this WAS a real JSON-transfer
  violation before this wave and has been genuinely fixed, not merely left alone.
- Registration: full 5-role `LanguageSpec`; `register_schema_spec` called for `"stdio.deflate"`
  only (not `"#diff"`), with an honest, correct, in-file reason (`DeflateDiff` has no derivable
  spec) — matches the recipe's own "skip rather than fabricate" rule.
- Fixtures real (`example.dsl.semio`, `example.pack.semio`, `example.zz`).
- Pitfalls: none found.

### las/1.0
- `cargo test … "artifacts::las"` → 34 passed, 0 failed, all 6 conformance laws included.
- `DiffCodec` (`🔺️diff/🦀️component.rs:1540-1601+`): real header-mask + fixed-order-fields binary
  frame (26 header fields + vlrs/points), confirmed real by direct read.
- `OpBinary` (`🧬️mutations/🦀️component.rs:461-512`): real tag-dispatched frame (14 variants),
  confirmed real.
- Protocol file: real, fully field-by-field 227-byte fixed header, cross-checked against
  `read_u16`/`read_u32`/`read_f64` byte-offset helpers in `⚙️engine/🦀️component.rs` — exact
  match. Correctly stops at the header and treats VLRs+points as one opaque trailing chain,
  documenting the `protocol-array-of-records` root cause (count-from-earlier-field AND
  format-selects-shape, neither expressible).
- Registration: full 5-role `LanguageSpec`; `register_schema_spec` deliberately NOT called, with
  an honest, specific reason (two independently-confirmed compiler-error blockers documented:
  `LasPointDiff`'s tri-state `Option<Option<T>>` fields, and no blanket `DslField` impl for bare
  tuples like `(f64,f64,f64)`).
- Fixtures real (`example.las`, `example.dsl.semio`, `example.pack.semio`, `example.bin`).
- Pitfalls: none found.

### dwg/ac1018
- `cargo test … "artifacts::dwg::standards::v_ac1018"` → all passing (part of the combined
  50-test dwg run), all 6 conformance laws included.
- `DwgDiff` derives `#[derive(..., dsl::DslDiff)]` — zero hand-rolled `DiffCodec` impl (confirmed
  by grep, zero hits) — provably already-real, routes through `.spk` container.
- `OpBinary` (`🧬️mutations/🦀️component.rs:153-160`): pure forward to `dsl::variants_binary`,
  doc-commented "Replaces the prior `serde_json` stub" — a real fix, not a no-op.
- **Restraint check (the #1 dwg-specific concern)**: protocol file declares ONLY the plain,
  unencrypted 21-byte AC10xx preamble (6-byte version sentinel + 12-byte reserved span + 1-byte
  maintenance_version + 2-byte codepage), cross-checked byte-for-byte against
  `dwg_version_sentinel`/`parse_version_header_fields`
  (`🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:71-96`) — exact offset match (0x00, 0x12,
  0x13-0x14). Everything past offset 0x15 is one opaque `chain remainder bytes`. The agent did
  NOT attempt the two-level decrypt/decompress indirection — correctly respected the M2
  exclusions carve-out (§6 of the recipe).
- Registration: full 5-role `LanguageSpec` + `register_schema_spec` for both
  `"stdio.dwg.ac1018"`/`"stdio.dwg.ac1018#diff"` (standard-qualified id, since ac1024 also uses
  the bare `"stdio.dwg"` id — correctly disambiguated, confirmed by reading both).
- Fixtures real (`example.dwg`, `example.dsl.semio`, `example.pack.semio` under both the
  artifact-level and standard-level `📚️examples`).
- Pitfalls: none found.

### dwg/ac1024
- `cargo test … "artifacts::dwg::standards::v_ac1024"` → all passing (part of the combined
  50-test dwg run), all 6 conformance laws included, PLUS the dedicated D1/D2-pipeline tests
  (`lcg_decrypt_is_its_own_inverse`, `lz_round_trip_literal_only_stream`,
  `real_fixture_d1_locates_every_named_section`, `real_fixture_d2_decompresses_every_section`,
  `real_fixture_page_directory_matches_header_cross_check`) and the architectural-fixture tests
  (`fixture_is_real_ac1024_not_a_stub`, `real_decode_reaches_d2_with_every_named_section`,
  `real_decode_stays_lossless_on_reencode`) — all passing, confirming the real fixture and real
  Rust-side D1/D2 pipeline are genuinely exercised, independent of the protocol-description-layer
  scope cut.
- `DwgDiff` derives `dsl::DslDiff` — zero hand-rolled `DiffCodec`, confirmed real via the derive.
  Its own `.spk`-container protocol file (`🔺️diff/💾️binary/📡️component.protocol.semio`) is a
  verbatim, correctly-copied instance of the recipe's §2.4 worked example (magic
  `0x8953504B0D0A1A0A`, 24-byte header, 4-arm tag-dispatched `repeat segments`, 84-byte footer) —
  confirmed by direct comparison against the recipe text.
- `OpBinary`: pure forward to `dsl::variants_binary`, confirmed real.
- **Restraint check**: same shape as ac1018 — 21-byte plain preamble modeled field-by-field
  (cross-checked against `dwg_version_sentinel`/`parse_version_header_fields` at
  `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:132-158`, exact offset match), everything
  past offset 0x15 (the encrypted R2004+ header, compressed page-directory, compressed
  section-info directory, and every named section's own encrypted+compressed pages) is ONE opaque
  `chain remainder bytes`. The in-file comment explicitly cites the M2 "would mean a different
  kind of system" framing and does not attempt the two-level indirection. Correct restraint,
  confirmed.
- Registration: full 5-role `LanguageSpec` + `register_schema_spec` for `"stdio.dwg"`/
  `"stdio.dwg#diff"` — confirmed present.
- Fixtures real, including the byte-real `architectural.dwg` (`📚️examples/🏛️architectural/
  🖼️assets/📄️architectural.dwg`), exercised end-to-end by the tests named above.
- Pitfalls: none found.

## Full crate suite

```
cargo test -p semio-s-plugin-stdio --lib
test result: ok. 1773 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 8.68s
```

No failures anywhere in the crate at the time of this run — including P1-P3/FG1's own 13 prior
standards, confirming no regression from this wave's work landing in the shared live tree.

## Findings requiring follow-up

1. **jpg/jfif-1.01 has no 5-role `LanguageSpec` registration and no `register_schema_spec` call**
   — confirmed independently (grep for `LanguageSpec`/`register_language`/`register_schema_spec`
   across jpg's entire tree returns zero hits). This is self-disclosed in FG2's own jpg report as
   a deliberate, time-boxed deviation, not a hidden defect, but it leaves jpg's own per-standard
   checklist incomplete relative to every other FG2 standard (all of which have full 5-role
   registration). Recommend a small follow-up task scoped to exactly this gap — copy the shape
   from any sibling FG2 standard's own `register_pilot_languages()` (bmp/tiff/deflate/las/dwg all
   have a clean, copy-pasteable exemplar) plus a `register_schema_specs()` stub (jpg's own types
   are fully hand-rolled per the F6 recon, so — per the recipe's own rule — that stub should stay
   empty with a `mechanism_gaps`-citing doc comment, exactly like json/csv/zip/png/las/tiff already
   do, not attempt to fabricate a `RecordSpec`).

## No recurrence of the FG1 binary-frame shortfall

Explicitly re-stating the one thing this verification pass was most alert for: **every one of the
9 standards' `DiffCodec::encode_diff`/`decode_diff` and `OpBinary::encode_op`/`decode_op` bodies
was read directly** (not inferred from doc comments or trusted from the self-reports). None of them
is `Ok(self.print_diff().into_bytes())` / `Ok(self.print_op().into_bytes())` or an equivalent
`print_x_op`/`print_x_diff` text-as-binary shortcut. The two "already-real, no-op" cases (bmp's and
both dwg standards' `DiffCodec`, which derive `#[derive(dsl::DslDiff)]`) were confirmed real by
finding the derive attribute on the struct directly, not assumed from a claim that "it derives
cleanly." **The FG1-era shortfall does not recur in this wave.**
