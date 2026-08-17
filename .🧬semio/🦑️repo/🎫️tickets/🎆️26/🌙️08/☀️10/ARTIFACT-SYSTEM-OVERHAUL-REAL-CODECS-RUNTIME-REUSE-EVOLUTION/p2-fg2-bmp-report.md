# P2-FG2 — 🖼️bmp/v3 — real grammar/protocol + fixture honesty + full language registration

## Summary

`stdio.bmp` v3 is binary-native per the P2-W0 census (§1b) and this wave's own brief. Its real
byte-level codec (`decode_bmp`/`encode_bmp`, `⚙️engine/component.rs`) was already fully real from
an earlier "schema overhaul" wave (F2) — full BITMAPFILEHEADER + BITMAPINFOHEADER decode, indexed
1/4/8bpp palette, 16/32bpp `BI_BITFIELDS`, 24/32bpp `BI_RGB`, all honestly typed on `BmpSnapshot`.
What THIS wave found: **all six `.grammar.semio`/`.protocol.semio` facet files were still pre-M1/M2
ABNF-flavored pseudo-syntax** (the current dialect header alone — `dialect protocol
stdio.bmp.snapshot` on one line — doesn't parse under the real parser at all), the mutations/diff
facets' protocol files described a fictional raw-JSON payload that neither codec ever emits, the
demo fixture (`🗣️example.dsl.semio`) was a literal fake `"hello"` placeholder (hex for the ASCII
string "hello", not real BMP bytes at all — no `.pack.semio` fixture existed), the artifact only
registered 1 of the mandatory 5 `LanguageSpec` roles, no `register_schema_specs` call existed
despite `BmpSnapshot`/`BmpDiff` both carrying genuine derived `RecordSpec`s, and no conformance-law
test module existed. All five are now fixed.

## What was real already (confirmed by direct reading, not assumed)

- **`OpBinary` for `BmpMutation`**: already real — `encode_op`/`decode_op`
  (`🧬️mutations/🦀️component.rs`) forward straight to `dsl::variants_binary::encode_op`/`decode_op`.
  Confirmed via a `[DEBUG]` probe against a real `BmpMutation` value: `encode_op(NoMutation) ==
  [1, 0, 0, 0]` (format=1, ordinal=0, symbol-count=0). **No Rust change needed here.**
  `opbinary_binary_upgraded: false` reflects this correctly.
- **`DiffCodec` for `BmpDiff`**: already real — `BmpDiff` derives `#[derive(dsl::DslDiff)]`
  (`🔺️diff/🦀️component.rs`), whose derive-generated `encode_diff`/`decode_diff` call
  `store::pack_rt::encode_document`/`decode_document` (the framework-generic `.spk` document
  container), never `print_diff().into_bytes()`. Confirmed via direct reading of
  `✨️derive/🦀️component.rs`'s `DslDiff` region AND a `[DEBUG]` probe: `encode_diff` output starts
  with the real `.spk` magic `[137, 83, 80, 75, 13, 10, 26, 10]` (`0x8953504B0D0A1A0A`).
  **`diffcodec_binary_upgraded: false`** — the facet was already real, this wave only had to
  correctly *describe* it (see §2 below); this is a no-op per the recipe's own explicit wording,
  not a mistake.
- **`BmpSnapshot`'s own `store::ArtifactDsl`/`store::ArtifactPack`**: real hand-rolled hex-dump text
  + real binary pack codecs, calling the real `encode_bmp`/`decode_bmp` under the shared `.semio`
  envelope — unchanged this wave, correctly documented by the rewritten snapshot facet files.

## What was NOT real, and what changed

### 1. Grammar files (rewritten, 3 total — all now real M1 dialect)

- `📸️snapshot/📝️text/📖️component.grammar.semio` — honest hex-dump grammar (`envelope-mark
  hex-body`, `hex-body = hex` using the framework's built-in `hex` macro — never a hand-rolled
  `{INT|IDENT}*` production, per this ticket's own recipe §3 pitfall #2), mirroring `stdio.png`'s
  own already-real snapshot/text grammar exactly (same "BMP has no textual syntax of its own"
  boundary). Replaced a header that didn't even parse under the real dialect
  (`dialect protocol stdio.bmp.snapshot` on one physical line — the real parser expects `dialect
  grammar` / `grammar <id>` / `start <production>` as separate directives).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — real one-line `OpText` grammar for
  `BmpMutation`'s GENERIC `dsl::DslOps`-derived `print_op`/`parse_op` (kebab-case keyword +
  `keyed_field_rank`-ordered `key=value` fields). Traced from a REAL `cargo test` run of a
  temporary `[DEBUG]`-prefixed probe against every `BmpMutation` variant (added, run once,
  captured real `print_op`/`encode_op` output, deleted before closing) — NOT guessed. Replaced a
  stub describing a fictional `serde_json`-tagged wire the real derive-driven printer never emits
  (real output has no `"mutation":` JSON tag at all; it's kebab-case bare keywords like
  `set-header-fields header-size=56 width=9 ...`).
- `🔺️diff/📝️text/📖️component.grammar.semio` — real grammar for `BmpDiff`'s derive-driven
  `print_diff`/`parse_diff` (`dsl::print(..., JoinMode::Inline)` over `__dsl_diff_spec()` — no
  container keyword, bare `key=value`/`key { ... }` fields in rank-then-declaration order). Traced
  from a real `[DEBUG]` probe: `BmpDiff::between(a, b).print_diff()` produced `"header-size=56
  width=8 ... palette { removed=[ ] modified=[ index=1 entry { b=99 g=88 r=77 reserved=1 } ]
  added=[ ... ] } pixels=[ 3 10 17 ... ]"` — confirming `pixels` (a plain `Option<Vec<u8>>` that
  the `#[dsl(base64)]`-peels-`Option`-first derive quirk falls back to a bracketed `INT` list for)
  prints AFTER `palette` despite being declared after it in the struct too (both rank 1, tie broken
  by declaration order) — replaced a stub describing camelCase JSON (`"headerSize"`) the real
  derive-driven printer never emits either.

### 2. Protocol files (rewritten, 3 total — all now real M2 dialect)

- `📸️snapshot/💾️binary/📡️component.protocol.semio` — real field-by-field BITMAPFILEHEADER (14
  bytes, unbraced `header fixed 14` block) + the 40-byte BITMAPINFOHEADER core (11 real fields, an
  unconditional `segment info_header { ... }`) + conditional BI_BITFIELDS masks (`segment
  bitfield_masks if compression eq 3 { ... }`, M2's conditional-presence construct) + conditional
  palette (`segment palette if bits_per_pixel le 8 { entries Array(Fixed(4), Field(colors_used))
  }`, a real count-from-field repeated structure using M2's cross-block field-env threading — NOT
  opaque, per the brief's explicit instruction) + trailing pixel data (`chain pixel_data bytes`,
  opaque — the row-padding stride formula is documented in a comment rather than modeled, since
  `Count`/`Array` have no computed-arithmetic-over-two-fields primitive).
- `🧬️mutations/💾️binary/📡️component.protocol.semio` — `format u8 | ordinal varint | chain bytes`,
  copied in shape from `stdio.txt`'s own already-real mutations protocol (same underlying
  `os_pack::encode_record_body` framework-generic wire, since `BmpMutation` derives `dsl::DslOps`
  the same way `TxtMutation` does).
- `🔺️diff/💾️binary/📡️component.protocol.semio` — the real `.spk` document-container shape (§2.4
  of this ticket's own recipe): `framing magic 0x8953504B0D0A1A0A` + `header fixed 24` (6 real
  fields: version_major/minor, required/optional flags, header_crc32, an 8-byte reserved span) +
  `chain bytes` (the segment table/chunk table/symbol table/compressed record body/84-byte footer
  all stay one opaque tail — a framework-level container, not artifact-specific, same treatment
  `stdio.txt`'s own diff protocol already gives it). Replaced a stub describing raw JSON bytes the
  real derive-driven `encode_diff` never emits.

### 3. Fixtures regenerated (both, `fixtures_regenerated: {dsl: true, pack: true}`)

The committed `🗣️example.dsl.semio` was `68656c6c6f` — the raw ASCII bytes of the literal string
"hello", not real BMP bytes, no `semio stdio.bmp.dsl v1` preamble line at all, and no
`🎒️example.pack.semio` file existed under `📚️examples/🎬️demo/🖼️assets/` (the `m5` protocol
auto-discovery harness's own `find_example_semio` silently found nothing for this facet). Both
regenerated via a temporary `[DEBUG]`-prefixed test in `⚙️engine/🦀️component.rs` that built a real
4x2 24-bit `BI_RGB` `BmpSnapshot` (8 distinct non-solid RGBA pixels, deliberately safe against
`encode_bmp`'s own canonicalization — `header_size`/`planes`/`bits_per_pixel`/`compression` set to
exactly what encode always hardcodes), called the REAL `store::ArtifactDsl::print_dsl`/
`store::ArtifactPack::encode_pack` functions, wrote the bytes straight to both fixture files, and
asserted a full round-trip before the test was deleted. `demo_bmp_snapshot()` (the same value,
kept as a real, non-`#[cfg(test)]` function so `fixture_honesty_law` can call it) now lives in
`⚙️engine/🦀️component.rs`.

### 4. `register_pilot_languages()` — 1 role → the full 5 (`registration_roles: 5`)

Was `stdio.bmp` (Document) only. Added `stdio.bmp.op` (Ops), `stdio.bmp.diff` (Diff,
`protocol: None` — the 5-role scheme has no dedicated "diff binary" role even though a real diff
protocol file now exists, per the recipe's own explicit note), `stdio.bmp.pack` (Pack, pointing at
the same real snapshot protocol), `stdio.bmp.spr` (Spr, pointing at the same real mutations
protocol). Also fixed the Document role's stale `extension: Some("bin")` → `Some("bmp")` while
touching this function (a real, in-scope correctness fix, matching `stdio.png`'s own precedent).

### 5. `register_schema_specs()` — new, real (not fabricated)

`BmpSnapshot` derives `#[derive(dsl::DslRecord)]` (real `__dsl_spec`) and `BmpDiff` derives
`#[derive(dsl::DslDiff)]` (real `__dsl_diff_spec`) — both genuine `fn() -> RecordSpec`
constructors, so per the recipe's own checklist item this wave registers both:
`dsl::registry::register_schema_spec("stdio.bmp", BmpSnapshot::__dsl_spec)` and
`("stdio.bmp#diff", BmpDiff::__dsl_diff_spec)`, `#[cfg(not(target_arch = "wasm32"))]`-gated to
match `os_dsl::registry`'s own gate, following `stdio.txt`'s own exemplar pattern (the pilot with a
genuinely single derivable spec per facet, unlike json/csv/zip/png's hand-rolled types or binary's
too-many-specs situation). `BmpMutation`'s own mutations facet is correctly skipped (`DslOps` gives
per-variant specs via `DslVariants`, no single canonical id to register a mutations facet under).

### 6. The 6 conformance-law tests + `demo_mutation_cases()`/`demo_diff_cases()` — new

Added `mod conformance_laws` to `⚙️engine/🦀️component.rs`'s test module (copied in shape from
`stdio.png`'s own `conformance_laws`, verbatim in structure): `committed_facet_files_parse`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
`protocol_walk_law`, `fixture_honesty_law`, plus a bonus `schema_spec_registration_resolves`
(matching `stdio.deflate`'s own precedent for this wave). These need real, exhaustive per-facet
case lists (`mutations::demo_mutation_cases()`, `diff::demo_diff_cases()`) — I found the mutations
and diff test modules had 3 near-duplicate inline `vec![...]` case lists apiece (one per existing
law test), a real pre-existing inconsistency this repo's CLAUDE.md directs refactoring rather than
leaving alone. Consolidated each into ONE module-level, `#[cfg(test)]`-gated case function
(matching `stdio.png`'s own `demo_mutation_cases`/`demo_diff_cases` placement — module scope, not
nested inside `mod tests`, so the engine's separate test file can reach them too), and refactored
`mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`/
`diff_codec_text_binary_roundtrip_law` to call the single source of truth instead of duplicating
fixture literals. Added two out-of-range no-op mutation cases (`RemovePaletteEntry`/
`SetPaletteEntry` at index 99) to `demo_mutation_cases()` for extra coverage, matching `stdio.png`'s
own precedent.

## A real authoring pitfall hit and fixed (worth flagging for future FG-wave agents)

**Inside a BRACED protocol block (`segment name { ... }`/`record name { ... }`/an `arm` body),
fields are written WITHOUT the leading `field` keyword** — `parse_braced_fields` calls
`parse_field_pair` directly (`name` then `type`, no `field` prefix), unlike an UNBRACED
`header fixed N` / top-level field list, which DOES use `field name type` per line (dispatched via
the main loop's `"field"` directive branch). My first draft used `field header_size u32` inside
`segment info_header { ... }` and got `"expected a protocol type, found Newline"` — the parser
correctly parsed `field` as the record's own FIELD NAME, then choked trying to parse `header_size`
as a `Prim` type. Fixed by dropping `field` inside all three braced segments (`info_header`,
`bitfield_masks`, `palette`). This wasn't spelled out explicitly in the recipe's own worked
examples (which only show `arm`/`record` bodies, not a `segment { ... }` with several fields) — the
zip/`.spk` `arm 3 { flags u8 seg_len varint ... }` example IS the same convention, just not labeled
as a general "braced blocks never take `field`" rule. Also independently hit this wave's own
documented pitfall #4 (a multi-symbol production wrapped across two physical lines silently
truncates) in both the mutations `op = ...` alternation and the diff `diff = ...?...?...`
optional-field run — caught immediately by `committed_facet_files_parse`/
`diff_grammar_conformance_law` and fixed by collapsing each to one physical line.

## Mechanism gaps found (new — not in the ticket's consolidated table; recorded here)

| gap id | engine area | symptom | blocking |
|---|---|---|---|
| `protocol-magic-shorter-than-8-bytes` | `Framing::Magic`/`magic_bytes` | `Framing::Magic` always reads and compares exactly 8 bytes (`walk_protocol`'s own `need(bytes, 0, 8, "magic")`); BMP's real on-disk magic is only 2 bytes ("BM") immediately followed by a real, non-zero `file_size` field — there is no way to zero-pad "BM" out to 8 bytes and have it match real file bytes 2..8 (unlike PNG, whose own 8-byte signature happens to fill the primitive exactly). Every OTHER short-magic binary-native format in this same wave (gif87a/89a's 6-byte "GIF87a"/"GIF89a", jpg's 2-byte 0xFFD8, tiff's 2-byte "II"/"MM") hits the identical wall. | No — worked around honestly with `framing record` + a genuine, individually-walked (content-unvalidated by this dialect) `fixed 2` field; deep magic-content validation stays Rust-side, same division of labor the dialect already gives every checksum/CRC. |
| `protocol-cond-field-vs-field` | `Cond`/`eval_cond` | `Cond` only compares one earlier-decoded field against a FIXED CONSTANT (`eq/ne/lt/le/gt/ge value`); BMP's real optional 4th (alpha) `BI_BITFIELDS` mask is gated by a POSITION comparison against another field (`cursor + 4 <= data_offset`), not a field-vs-constant test — genuinely inexpressible. | No — the real committed `.pack.semio` fixture is 24bpp `BI_RGB` (`compression != 3`), so the whole `bitfield_masks` segment is absent and this case never reaches `protocol_walk_law`; documented in the protocol file's own comment rather than silently modeled wrong. |
| `protocol-count-field-no-default` | `Count::Field`/`resolve_count` | `Count::Field(name)` only resolves a DIRECT field value; BMP's real palette-entry count formula is `if colors_used != 0 { colors_used } else { 1 << bits_per_pixel }` (the common "full default palette" case) — no conditional-default arithmetic is expressible. | No — the real committed fixture has no palette at all (`bits_per_pixel == 24`), so the `palette` segment is absent and never reaches `protocol_walk_law`; documented in the protocol file's own comment. Would need a real fixture with `bits_per_pixel <= 8 && colors_used == 0` to expose an actual mismatch (not attempted — out of this wave's fixture-generation scope). |

## Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::bmp"` → **25 passed, 0 failed, 0 ignored**
(all pre-existing F2-era laws — `mutation_diff_law`, `inverse_law`, `absorb_law`,
`between_roundtrip_law`, `field_sweep_covers_every_mutable_field`, `op_text_binary_roundtrip_law`,
`diff_codec_text_binary_roundtrip_law`, `codec_retention_law`, `codec_round_trip`,
`row_bytes_padding_is_exact`, `gradient_checkerboard_24bit_round_trip`,
`indexed_4bit_palette_round_trip`, `bitfields_16bit_555_round_trip`, `sniff_rejects_non_bmp_bytes`,
`empty_snapshot_matches_schema`, `demo_source_nonempty`, plus the semio-image io serializer/
deserializer tests that exercise this codec — PLUS all 6 new conformance-law tests
(`committed_facet_files_parse`/`grammar_conformance_law`/`ops_grammar_conformance_law`/
`diff_grammar_conformance_law`/`protocol_walk_law`/`fixture_honesty_law`) and
`schema_spec_registration_resolves` — all green.

`cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1769 passed, 3 failed, 1 ignored**
(retried once, stable both times). All 3 failures classified by file path, none touching
`artifacts::bmp::*`: `artifacts::dwg::standards::v_ac1018::engine::tests::conformance_laws::
protocol_walk_law`, `artifacts::dwg::standards::v_ac1024::engine::tests::conformance_laws::
protocol_walk_law`, `artifacts::gif::standards::v89a::engine::tests::conformance_laws::
ops_grammar_conformance_law` — every one a sibling artifact explicitly named in this ticket's own
repo-rules digest as within THIS SAME wave (`gif`/`dwg` both listed alongside `bmp`), confirmed via
direct read of the dwg failure (`magic mismatch: expected [0, 0, 65, 67, 49, 48, 49, 56], got [65,
67, 49, 48, 49, 56, 0, 0]` — the SAME `Framing::Magic`-always-8-bytes issue this report's own
`protocol-magic-shorter-than-8-bytes` gap names, hit by a concurrent agent still mid-edit on their
own file at the time of this run) — not chased, per the ticket's own explicit instruction.

`cargo check -p semio-s-plugin-stdio --lib` (non-test) → clean, zero errors, only pre-existing,
unrelated warnings (confirmed this artifact's own Rust changes are not a compile blocker).

`bun run ./📜️script.ts policy` → 11 total lines mention `🖼️bmp` in the full repo-wide breach
output, ALL pre-existing and unrelated to this wave's scope (`taxonomy/emoji-prefix`,
`mutation-migration/triad-completeness`, `mutation-migration/artifact-engine`,
`artifact-schema/facet-completeness` ×3, `artifact-schema/type-name-parity`,
`stdio-artifacts/composer` ×2, `os-state-authority/item-scope-global` ×2 — none reference
`POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`/`POLICY_FIXTURE_HONESTY`/
`POLICY_LANGUAGE_REGISTRATION`/`POLICY_STDIO_JSON_TRANSFER_BAN`, the 5 rules this wave's own
deliverables actually target — confirmed by an explicit grep for each rule slug intersected with
`bmp`, zero hits). Zero new breaches introduced by this wave's own changes.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/⚙️engine/🦀️component.rs` — added
  `demo_bmp_snapshot`, `register_schema_specs` (+ wired into `register()`), full 5-role
  `register_pilot_languages` (+ fixed stale `extension: Some("bin")` → `Some("bmp")`), `mod
  conformance_laws` (7 tests: the 6 required + `schema_spec_registration_resolves`). Temporary
  `[DEBUG]`-prefixed fixture-generation and parse-error-probe tests added then removed.
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — moved `entry`/`base_snapshot`/
  `sweep_a`/`sweep_b` fixture helpers to module scope (out of `mod tests`), added
  `demo_mutation_cases()` (single source of truth), refactored `mutation_diff_law`/`inverse_law`/
  `op_text_binary_roundtrip_law` to use it instead of 3 near-duplicate inline lists.
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — moved `entry` to module scope, added
  `demo_snap_a`/`demo_snap_b`/`demo_diff_cases()` (single source of truth), refactored
  `diff_codec_text_binary_roundtrip_law` to use it.
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten (real
  hex-dump grammar, real M1 dialect header).
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten (real
  BITMAPFILEHEADER/BITMAPINFOHEADER/conditional-masks/conditional-palette/pixel-data layout, real
  M2 dialect).
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten (real
  op-text grammar, traced from real `print_op` output).
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten (real
  format/ordinal/opaque-record-body layout).
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten (real diff
  grammar, traced from real `print_diff` output).
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten (real
  `.spk`-container header layout, magic + 24-byte header + opaque tail).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  regenerated (real `print_dsl` output, real preamble line, real BMP bytes — previously a fake
  "hello" placeholder).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new
  (real `encode_pack` output for the demo snapshot; this fixture did not exist before this wave).

No files outside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/**` and this report were touched.
`📦️glue.rs`, `📜️script.ts`, the SDK traits, schema/dsl/protocol/registry modules, the framework
`🧪️fixture-sweep` graduation list, and `🏪️store` were all left untouched, per the ownership
boundary.

## Deviations from the literal brief

- The brief's suggested BITFIELDS mask + palette-count modeling ("thread `Field(name)` resolution
  across blocks") was followed as literally as the dialect allows, but two genuine sub-cases
  (the optional 4th alpha mask's position-comparison gate; the palette count's
  field-value-with-fallback formula) are NOT expressible even with M1/M2's full construct set —
  recorded as `mechanism_gaps` above rather than fabricated or silently modeled wrong. Neither
  blocks the real committed fixture, which exercises neither code path.
- `opbinary_binary_upgraded: false` and `diffcodec_binary_upgraded: false` — both facets were
  confirmed real (not the F6 text-as-binary shortcut) via direct reading BEFORE any Rust change was
  made; this wave's actual Rust-side work was limited to `register_pilot_languages`/
  `register_schema_specs`/the conformance-law test module/moving 5 existing fixture helpers to
  module scope — the grammar/protocol/fixture rewrite was the load-bearing deliverable, exactly as
  the brief's own "binary-frame lesson from FG1" section anticipates for a standard that turns out
  to already be real.

## Standards / laws / facets

- Standard: `bmp v3` (BITMAPINFOHEADER-based, Windows/OS2 BMP).
- Laws present: `mutation_diff_law`, `inverse_law`, `absorb_law`, `between_roundtrip_law`,
  `field_sweep_covers_every_mutable_field`, `op_text_binary_roundtrip_law`,
  `diff_codec_text_binary_roundtrip_law`, `codec_retention_law` (pre-existing, all still green) +
  `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
  `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`,
  `schema_spec_registration_resolves` (new, this wave).
- Facets updated: snapshot (text+binary grammar/protocol), diff (text+binary grammar/protocol,
  Rust fixture helpers), mutations (text+binary grammar/protocol, Rust fixture helpers), engine
  (demo snapshot, 5-role language registration, schema-spec registration, conformance laws), demo
  example fixtures (dsl + pack).
