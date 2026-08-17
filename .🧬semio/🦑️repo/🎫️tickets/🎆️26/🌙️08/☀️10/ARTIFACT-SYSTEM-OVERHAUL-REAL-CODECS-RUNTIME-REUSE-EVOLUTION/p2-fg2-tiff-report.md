# FG2 — 🖼️tiff (standard 6.0) — Real Codecs / Runtime Reuse / Evolution

## Scope

Artifact: `stdio.tiff` (TIFF 6.0), path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/`.
Subset touched: `✳️any` (the only subset with schema/diff/mutations facets; `✳️baseline` is a
validate-only subset layered on top and was not touched).

Native-side classification (per `p2-w0-recon-report.md` §1b): **binary-native**. TIFF has no
textual syntax of its own — `ArtifactDsl::print_dsl`/`parse_dsl` hex-encode/decode the real TIFF
file bytes verbatim (the `HandcraftedArtifactCodecs` region of `📸️snapshot/🦀️component.rs`), so
the "grammar" file honestly models the DSL TEXT form (a hex-dump grammar, PNG's own precedent)
while the "protocol" file models the real byte layout.

## What was already real vs. what F6 left on the text-as-binary shortcut

Before this wave:

- `⚙️engine/🦀️component.rs`'s `decode_tiff`/`encode_tiff` were ALREADY a real, complete TIFF 6.0
  codec: runtime `II`/`MM` endianness, generic typed tag/type/value decode for all 12 TIFF field
  types, IFD-chain walking (cycle-guarded), inline-vs-out-of-line tag value resolution, and
  uncompressed + PackBits strip pixel decode/encode. Nothing needed fixing here.
- `TiffDiff`/`TiffMutation` were hand-rolled (a real compile-error citation in both files' own doc
  comments confirms `#[derive(dsl::DslDiff)]`/`#[derive(dsl::DslOps)]` fail — `TiffValues`, a
  12-variant data-carrying enum, has no `DslField` impl). Their TEXT codecs (`print_diff`/
  `parse_diff`, `print_op`/`parse_op`) were already real, genuine grammars.
- **`DiffCodec::encode_diff`/`decode_diff` and `OpBinary::encode_op`/`decode_op` were BOTH still on
  the F6 `print_diff()/print_op().into_bytes()` text-as-binary shortcut** — confirmed by direct
  read before touching anything, matching the P2-W0 census ("100% of stdio's `DiffCodec` impls
  were still on the text-as-binary shortcut"). This wave's main Rust-side deliverable was
  upgrading both to real binary frames.
- All 6 `.grammar.semio`/`.protocol.semio` files on disk (snapshot/diff/mutations × text/binary)
  were pre-Phase-2 placeholders in an ad-hoc ABNF-like dialect (`dialect grammar
  stdio.tiff.snapshot` on one line, no `extension`/`start` directives, hand-rolled `HEXDIG`/`WS`
  productions instead of the framework's `hex` macro, a binary protocol file that
  RE-DESCRIBED the SEMIO envelope instead of describing only the post-unwrap TIFF payload). All 6
  were rewritten from scratch in the real M1/M2 dialect.
- The shipped `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` fixture was a bare
  `hex("hello")` placeholder (68656c6c6f) with no `semio stdio.tiff.dsl v1` preamble — not real
  TIFF bytes at all. No `🎒️example.pack.semio` fixture existed on disk.
- No `register_pilot_languages()` / 5-role `LanguageSpec` registration existed for `stdio.tiff` at
  all.

## Deliverables landed this wave

### 1. Real binary-frame upgrade — `DiffCodec` (`🔺️diff/🦀️component.rs`)

Added a `🔖️ValueBinaryCodecs`/`🔖️DiffValueBinaryCodecs` region: real recursive binary twins of
every text-grammar helper (`enc_values_bin`/`dec_values_bin` — a 1-byte kind tag 0-11 + real
varint-length-prefixed/fixed-width-LE payload for all 12 `TiffValues` variants including
`Rational`/`SRational` pairs; `enc_tag_bin`/`dec_tag_bin`; `enc_ifd_bin`/`dec_ifd_bin`;
`enc_tags_diff_bin`/`dec_tags_diff_bin`; `enc_ifds_diff_bin`/`dec_ifds_diff_bin`), reusing
`store::pack_rt::write_varint_u64`/`store::ByteReader` (same convention `xml`'s own
`enc_xml_node_bin`/`dec_xml_node_bin` — read as the mandated reference — establishes). Also added
the previously-missing `write_bytes_lp`/`read_bytes_lp`/`write_str_lp`/`read_str_lp` LEB128-framed
primitives (this file did not have them at all before this wave, unlike what a first skim of a
sibling artifact might suggest).

`encode_diff`/`decode_diff` now emit/parse a REAL frame: `format u8 | flags u8 |
[byte_order][ifds][pixels]` — `flags` bits 0/1/2 mark the three independently-optional `TiffDiff`
fields present (same bitmask shape `XmlDiff`'s 3-optional-field frame uses), each present field's
real typed payload following in fixed order. `ifds` recurses genuinely through the IFD-index-keyed
triple → tag-id-keyed triple → 12-variant `TiffValues` union, all real binary, never
`print_diff().into_bytes()`.

### 2. Real binary-frame upgrade — `OpBinary` (`🧬️mutations/🦀️component.rs`)

Added `enc_snapshot_bin`/`dec_snapshot_bin` (reusing `TiffDiff`'s binary primitives). `encode_op`/
`decode_op` now emit/parse `format u8 | tag u8 | variant payload` — `tag` is the `TiffMutation`
variant ordinal (0-7, same order `print_tiff_mutation`'s keyword match uses), payload is the
variant's real fields via the same binary primitives (`SetSnapshot`→`enc_snapshot_bin`,
`InsertIfd`→varint index + `enc_ifd_bin`, `SetTag`→varint ifd_index + tag u16 + kind u8 +
`enc_values_bin`, etc.).

Both upgrades verified by the pre-existing `diff_codec_text_binary_roundtrip_law` and
`op_text_binary_roundtrip_law` tests (unchanged test bodies, now exercising real binary instead of
disguised text) plus the new `protocol_walk_law` conformance test.

### 3. Six grammar/protocol files rewritten in the real dialect

- `📸️snapshot/📝️text/📖️component.grammar.semio` — real header (`dialect grammar` / `grammar
  stdio.tiff.snapshot` / `extension tiff` / `start document`), `envelope-mark = "stdio.tiff"`,
  `document = envelope-mark hex-body`, `hex-body = hex` using the framework's `hex` macro (not a
  hand-rolled `HEXDIG`/`WS` ABNF fragment) — PNG's own hex-dump precedent, applied honestly.
- `📸️snapshot/💾️binary/📡️component.protocol.semio` — the REAL TIFF byte layout: `header fixed 8`
  (`field byte_order endian { "II"=le "MM"=be }` — M2 item 6's runtime-endianness construct, THE
  capability the P2-W0 recon's dedicated TIFF paragraph named this artifact as needing, now
  actually used; `field magic u16`; `field first_ifd_offset u32`), a second `header fixed 2` for
  the real, individually-walked `entry_count` field leading IFD 0, then `chain rest bytes` for
  everything past that point (every IFD entry, the next-IFD-offset chain, every out-of-line tag
  value, and the pixel strip data — see Mechanism Gaps below for exactly why).
- `🔺️diff/📝️text/📖️component.grammar.semio` — the real one-line `print_diff`/`parse_diff` shape:
  `document = byte-order-tok? ifds-tok? pixels-tok?`, the index-keyed `ifds-diff-body` /
  tag-id-keyed `tags-diff-body` collection-triple productions (recipe §1.4's shape), and
  `tiff-values` modeling the real single-letter-tag + bracketed-payload grammar `enc_values`
  emits (`B`/`A`/`S`/`L`/`R`/`E`/`U`/`H`/`G`/`Q`/`F`/`D`).
- `🔺️diff/💾️binary/📡️component.protocol.semio` — `format u8 | flags u8 | chain payload bytes`,
  documented per §2.5's opaque-tail pattern (the real header individually walked, the recursive/
  collection-shaped payload one opaque tail — mirrors `xml`'s own diff protocol file almost
  verbatim, adapted for TIFF's 3-flag bitmask and its own `protocol-array-of-records` root cause).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — the real one-line `print_op`/`parse_op` shape:
  one alternative per `TiffMutation` variant keyword, `snapshot-lit`/`ifd-lit`/`tag-lit`/
  `tiff-values` restated (self-contained leaf grammar, matching the repo's per-facet convention).
- `🧬️mutations/💾️binary/📡️component.protocol.semio` — `format u8 | tag u8 | chain payload bytes`.

All 6 verified to parse under the real dialect parser by `committed_facet_files_parse`.

### 4. `register_pilot_languages()` — 5-role `LanguageSpec` registration

Added to `⚙️engine/🦀️component.rs`, called from `register()`. `stdio.tiff` (Document),
`stdio.tiff.op` (Ops), `stdio.tiff.diff` (Diff, `protocol: None` per the 5-role scheme's own
documented shape), `stdio.tiff.pack` (Pack), `stdio.tiff.spr` (Spr) — all `dsl::passthrough_hooks`,
mirroring png's own `register_pilot_languages` exemplar exactly. `register_schema_spec` was
deliberately NOT called (filed below) — `TiffSnapshot`/`TiffDiff`/`TiffMutation` have no derivable
`RecordSpec` (same `TiffValues`-blocks-`DslField` root cause the diff file's own doc comment
already documents).

### 5. Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — REPLACED the `hex("hello")` placeholder
  with genuine `print_dsl(demo_tiff_snapshot())` output (`semio stdio.tiff.dsl v1` preamble +
  real hex-encoded TIFF bytes: II header, 10 real IFD entries incl. a carried `Artist` ASCII tag,
  real strip pixel data). Generated via a temporary `[DEBUG]`-prefixed `#[ignore]`d test that
  called the real `store::ArtifactDsl::print_dsl` directly; the temp test was deleted before
  finishing.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — NEW, genuine `encode_pack(demo_tiff_snapshot())`
  bytes (same generation method, temp test deleted).
- `demo_tiff_snapshot()` (new, `⚙️engine/🦀️component.rs`) is the single source of truth for both
  fixtures — deliberately built via a REAL `encode_tiff`/`decode_tiff` round trip (not
  hand-assembled core-tag values) so it is immune to `encode_tiff`'s own canonicalization
  self-correcting it on the first round trip (the same trap `png`'s own `demo_png_snapshot()` doc
  comment documents and guards against for its IHDR fields).
- `demo_mutation_cases()` (new, `🧬️mutations/🦀️component.rs`) / `demo_diff_cases()` (new,
  `🔺️diff/🦀️component.rs`) — representative case lists (every `TiffMutation` variant, every
  `TiffValues` field-type family, IFD-level AND tag-level removed/modified/added) reused by both
  the grammar-conformance and protocol-walk laws.

### 6. Six conformance-law tests (new `conformance_laws` submodule, `⚙️engine/🦀️component.rs`)

`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — copied from png's own
`conformance_laws` module shape, adapted for TIFF's own demo helpers. `fixture_honesty_law` checks
BOTH the `.dsl.semio` and `.pack.semio` fixtures (png's own fuller exemplar, since TIFF now ships
both).

### 7. One real protocol-dialect bug found and worked around

While drafting the snapshot protocol file, discovered that `walk_protocol`'s `Framing::Record`
handling special-cases any NAMED `Block::Record` (`record <name> { ... }`) by jumping `pos`
straight to `bytes.len()` on first sight — treating the WHOLE remaining buffer as that record's
body — rather than genuinely walking its declared fields (this is the exact mechanism the
"record IS the rest" text-native `framing record / chain payload utf8` exemplar relies on; it is
NOT a bug for that case, but it silently defeats field-level validation for any OTHER named
`record` block under `Framing::Record`). Worked around by using a second `header fixed N` block
(anonymous, unconditionally field-walked) instead of `record ifd0 { field entry_count u16 }` for
the real `entry_count` field — verified the difference directly by first landing the `record`
form, observing `protocol_walk_law` still pass but suspiciously not exercise the field, then
re-reading `walk_protocol`'s `Block::Record` arm to confirm the jump-to-EOF behavior, then fixing.
Not filed as a `mechanism_gaps` blocking entry (a correct workaround exists and this is a real,
usable framework mechanism, just one worth documenting precisely for the next FG-wave agent who
reaches for a named `record` block under `framing record` for anything other than "the rest").

## Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::tiff"` → **39 passed, 0 failed, 0 ignored**
(all pre-existing F6-era tests unchanged and passing, plus the 6 new conformance-law tests).

`cargo test -p semio-s-plugin-stdio --lib` (whole crate), run twice ~90s apart per the ticket's
own retry-once guidance:

- 1st run: **1754 passed, 1 failed** — `artifacts::gif::standards::v87a::…::protocol_walk_law`
  (magic-byte-order mismatch, classic mid-edit symptom). `git status` confirmed `🎞️gif` files
  under active concurrent edit by another session at the time.
- 2nd run (~90s later): **1759 passed, 1 failed** — the gif failure was GONE (more tests passing
  overall, confirming real concurrent progress), replaced by a DIFFERENT sibling-wave artifact,
  `artifacts::dwg::standards::v_ac1018::…::protocol_walk_law` (same magic-byte-order-swap
  symptom). Both `gif` and `dwg` are explicitly named in this ticket's own repo-rules sibling-wave
  list ("gif/jpg/bmp/tiff/deflate/las/dwg") — this is the documented "another agent in your own
  wave mid-edit" pattern, not chased further per instructions (retry once, don't chase).

Zero TIFF-related failures in either whole-crate run.

`grep serde_json::to_vec|from_slice|to_string|from_str|Value` over `🖼️tiff/**` → clean, no
JSON-transfer violations.

## Mechanism gaps (all non-blocking — real, honest opaque-tail workaround applied)

1. **`protocol-array-of-records`** (consolidated table, reused) — TIFF's IFD entries are a
   homogeneous but per-entry-VALUE-shape-varying 12-byte record (tag/type/count/value-or-offset,
   the value shape depending on the entry's own `type` field), repeated `entry_count` times.
   `Prim::Array` only repeats one fixed-width scalar, never a multi-field record — same root cause
   ZIP's/CSV's own nested-record-array gap. Workaround: the snapshot protocol models the real
   `entry_count` field, then treats the whole entry table (plus everything after it) as one opaque
   `chain`; the Rust `decode_tiff`/`encode_tiff` side stays fully, genuinely structured.
2. **`tiff-out-of-line-tag-value-offset`** (new, TIFF-specific instance of the ZIP-class
   offset-pointer gap) — each IFD entry's 4-byte value field is dual-meaning: an inline value OR a
   file offset to out-of-line value data, selected by `element_size(type) * count <= 4`
   (`read_tag_values`, TIFF6 §2's own documented rule). The protocol dialect has no primitive to
   conditionally dereference a just-decoded offset field mid-walk. Same honest opaque-tail
   workaround as above; Rust-side (`read_tag_values`) resolves it for real, independently tested.
3. **`tiff-ifd-chain-pointer-repeat`** (new) — the IFD chain itself is a linked list
   (`next_ifd_offset`, repeat-until-0 via an absolute file-offset jump decoded from EACH IFD, not a
   static/count-sourced repeat). `Block::Repeat` only supports tag-dispatched or
   byte-sentinel-terminated iteration, not "repeat by re-jumping to a pointer decoded from the
   PREVIOUS iteration's own body." Only the first IFD's `entry_count` field is protocol-modeled;
   the whole chain (2nd+ IFDs, if any) is inside the same opaque tail. `decode_tiff`'s own
   `read_ifd_chain` (cycle-guarded) walks the real chain in Rust, independently tested
   (`gradient_checkerboard_*_round_trip`, `carried_ascii_tag_round_trips`, etc. all exercise
   single- and would exercise multi-IFD files identically since the codec is chain-generic).

None of these are new discoveries beyond what the P2-W0 recon's own dedicated TIFF paragraph and
§1b table already anticipated ("needs 'select Prim endianness from an earlier field, apply to all
subsequent reads,' plus the offset/pointer-resolution primitive ZIP needs, plus 'repeat via
next-pointer until sentinel 0'") — the endianness capability (M2 item 6, `Prim::Endian`) was
already built and IS used for real in the snapshot protocol; the offset-resolution and
next-pointer-repeat capabilities were NOT built (correctly out of scope per this ticket's own
ownership rules) and are recorded here rather than worked around by inventing new dialect syntax.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/⚙️engine/🦀️component.rs` — added
  `register_pilot_languages()` (called from `register()`), `demo_tiff_snapshot()`, the
  `conformance_laws` test submodule (6 tests).
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added binary primitives
  (`write_bytes_lp`/`read_bytes_lp`/`write_str_lp`/`read_str_lp`), `enc_values_bin`/`dec_values_bin`/
  `enc_tag_bin`/`dec_tag_bin`/`enc_ifd_bin`/`dec_ifd_bin`/`enc_tags_diff_bin`/`dec_tags_diff_bin`/
  `enc_ifds_diff_bin`/`dec_ifds_diff_bin`, `demo_diff_cases()`; rewired `DiffCodec::encode_diff`/
  `decode_diff` to the real binary frame.
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — added `enc_snapshot_bin`/
  `dec_snapshot_bin`, `demo_mutation_cases()`; rewired `OpBinary::encode_op`/`decode_op` to the real
  binary frame; fixed the `TiffTag` import.
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten.
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten.
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — rewritten.
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten.
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten.
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  replaced fake fixture with real `print_dsl` output.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new,
  real `encode_pack` output.

Not touched: `📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol/registry framework
modules, `🧪️fixture-sweep`, `🏪️store`, `AGENTS.md`, `STATUS.md` — all out of this agent's
ownership boundary per the wave brief.

## Deviations from the literal brief

- Added `🎒️example.pack.semio` even though the checklist marks it required (not the optional
  `spr.semio`) — it did not exist on disk before this wave; regenerated as part of "real fixtures."
- Extended `fixture_honesty_law` to check both fixtures (png's own fuller exemplar) rather than
  just the `.dsl.semio` one, since both now exist.
