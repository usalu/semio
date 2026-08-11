# P2-FG3 — `📄️pdf` (standards 1.4 and 1.7) — Real Grammar/Protocol Report

Combined agent per the plan's own S6 pairing precedent. Owns exactly
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/` and `.../🔖️1.7/` plus this report.
No other file outside that boundary was touched (verified via `git status --porcelain` scoped to
the artifact directory before/after).

## Baseline

`cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"` at HEAD (before any edit): **142
passed, 0 failed**. `git status` showed only 1.7's `⚙️engine/🦀️component.rs` locally modified —
read as this program's own prior F4/F6c work per the brief's own note, treated as the real
starting point (not touched further except the two `#[test] fn debug_*` scratch additions used to
extract real `print_dsl`/`encode_pack` bytes, all removed before the final run).

## What was read first (never guessed)

- `📖️grammar-recipe.md` in full (dialect syntax, 5 pitfalls, per-standard checklist, mechanism-gap
  table) and `p2-w0-recon-report.md`'s pdf/gltf/ply hybrid-classification rows.
- Both engines in full: `🏅️standards/🔖️1.4/⚙️engine/🦀️component.rs` (112-line frozen stub —
  `encode_pdf` writes a fixed 5-object template, `decode_pdf` does a raw forward
  `stream`/`endstream` substring search, no xref/object-graph parsing) and `🏅️standards/🔖️1.7/
  ⚙️engine/🦀️component.rs` (1787 lines — real `Lexer`, classic+stream+hybrid+brute-force xref,
  page-tree inheritance, ToUnicode-aware text extraction, a minimal multi-page writer).
- Both `🧬️mutations/🦀️component.rs` and `🔺️diff/🦀️component.rs` for both standards, in full, to
  trace the REAL `print_op`/`parse_op`/`print_diff`/`parse_diff`/`encode_op`/`encode_diff` shapes
  (never invented). 1.4's `PdfMutation`/`PdfDiff` derive `dsl::DslOps`/`dsl::DslDiff` for real
  (confirmed via their own doc comments citing a real `cargo check`); 1.7's are fully hand-rolled
  (confirmed rejected for real — `PdfObject`/`PdfValueDiff` are genuine data-carrying enums with no
  `DslField` impl).
- Real fixture generation via temporary `#[test] fn debug_dump_real_shapes`/`debug_generate_
  fixtures` (eprintln + `std::fs::write` writing straight to the target fixture paths), run once,
  output captured, temp tests deleted before the final test run — never hand-derived.
- Precedents read in full before drafting: `💾️binary/🏅️standards/🔖️raw` (hex-body DSL-text
  precedent, matches PDF's own real `print_dsl` shape exactly), `📄txt` (already-real derive-path
  `OpBinary`/`DiffCodec` `.spk`-container shape, matches 1.4's exactly), `📰xml` (hand-rolled
  recursive binary-frame upgrade template, matches 1.7's exactly), `🎞️gif` 87a/89a (two-standards-
  one-artifact registration pattern AND the `framing magic`-fixed-8-bytes gap + its `header fixed
  N { field magic fixed N }` fix), `🎒️zip` (`backward` block worked example).

## Deviations from the brief's classification (documented, with justification)

1. **Snapshot GRAMMAR (DSL text) is `hex-body`, not real COS text syntax**, for BOTH 1.4 and 1.7.
   The brief's own classification describes "grammar models the real COS text syntax" — but a
   direct read of `impl store::ArtifactDsl for PdfSnapshot` on BOTH standards (confirmed further
   by a real `[DEBUG]` dump of `print_dsl`, not assumed) shows `print_dsl`/`parse_dsl` hex-encode
   the RAW `encode_pdf(self)` byte output verbatim — the same choice `binary/raw`'s own DSL-text
   codec makes, NOT a reserialized-native-format text form the way json/xml/csv's own `print_dsl`
   is real native text. Since the mandate is to match the REAL `print_dsl`/`parse_dsl` function,
   not an aspirational one, the snapshot grammar is `document = artifact-mark hex-body` (verbatim
   binary/raw's own precedent) for both standards. The REAL COS structure instead lives — and IS
   modeled — in the PACK/PROTOCOL facet below, where `encode_pack_with`'s payload genuinely is the
   raw `%PDF-...` file bytes.
2. **`framing magic` cannot express PDF's real 4-/5-byte magics.** `Framing::Magic` unconditionally
   reads/compares EXACTLY 8 raw bytes (`🗣️dsl/📖️grammar/🦀️component.rs`'s own `magic_bytes(u64) ->
   [u8;8]`), confirmed live by a real `walk_protocol` failure (`expected [0,0,0,0,37,80,68,70],
   got [37,80,68,70,45,49,46,52]`) before the fix. Same `protocol-framing-magic-fixed-8-bytes` gap
   gif87a's own protocol file already documents (its own 6-byte `GIF87a` signature hit the exact
   same wall). Fixed the same way: `framing record` + a real, individually-read `field magic fixed
   N` inside a `header` block — genuinely walked but not byte-validated at the protocol layer (the
   real Rust `decode_pdf` already validates it byte-for-byte).
3. **`backward`'s own `magic` literal is ALSO u64-capped** (`trim_be_bytes(parse_u64_literal(...))`
   — same 8-byte ceiling). PDF 1.7's real anchor keyword `startxref` is 9 ASCII bytes, one over the
   cap; confirmed live by a real parse failure (`invalid hex literal 0x737461727478726566`, 18 hex
   digits). Anchors on the first 8 bytes, `startxre` (`0x7374617274787265`), documented as unique
   enough in a well-formed PDF and harmless (the dropped trailing `f` simply becomes the first byte
   of the already-opaque `tail`).
4. **`demo_pdf17_snapshot()` is the real `decode_pdf(encode_pdf(seed))` fixed point, not a
   hand-built struct.** `encode_pdf` only ever reads `pages`/`info` (the writer's own doc comment:
   "the original `objects` graph is deliberately NOT re-emitted") and regenerates a fresh
   Catalog/Pages/Font/Content-stream graph every call; a hand-built demo with `objects: vec![]`
   made `parse_dsl(print_dsl(demo)) != demo` (confirmed live — decode always returns 6 populated
   objects + a real trailer). `pages`/`info` DO survive losslessly (same as the bachelor-thesis
   example's own `decode_encode_decode_is_structurally_equal_at_page_level` test already proves at
   scale); only `objects`/`trailer` needed the fixed-point construction. 1.4's own
   `demo_pdf_snapshot()` similarly uses `width: 612.0, height: 792.0` — NOT an arbitrary choice:
   `decode_pdf` (1.4) hardcodes those two literals unconditionally regardless of input, so any
   other value would break the same round-trip law.

## Per-standard deliverables

### `stdio.pdf` (1.4 — frozen stub, S6/F6 already on the DERIVE path)

- **Snapshot**: grammar = `hex-body` (real `print_dsl` shape). Protocol = `framing record; header
  fixed 4 { field magic fixed 4 }; chain payload bytes` (real 4-byte `%PDF` magic, walked not
  validated; rest opaque — 1.4's own decoder does a raw forward substring search, no xref/trailer
  structure to model, the documented "1.4 stays a frozen stub" boundary).
- **Mutations**: `PdfMutation` derives `dsl::DslOps` for real — `OpBinary::encode_op`/`decode_op`
  were ALREADY real (`dsl::variants_binary::encode_op`/`decode_op`), confirmed empirically
  (`encode_op(NoMutation) == [1,0,0,0]`, byte-identical to `stdio.txt`'s own documented example) —
  **no upgrade needed**, only real grammar/protocol files written to match. Grammar traces the
  real derive-driven `print_op` output (`"set-snapshot snapshot { schema=stdio.pdf page {
  width=300.5 height=400.25 text=\"hello world\" } }"`, `"no-mutation"`, confirmed by a real
  `[DEBUG]` dump). Protocol = `framing record; field format u8; field ordinal varint; chain
  bytes` (matches `stdio.txt`'s own already-real mutations protocol shape exactly).
- **Diff**: `PdfDiff` derives `dsl::DslDiff` for real — `DiffCodec::encode_diff`/`decode_diff` were
  ALREADY real (`store::pack_rt::encode_document`, the full `.spk` container), confirmed
  empirically (real magic `0x8953504B0D0A1A0A` at byte 0 of a real dump) — **no upgrade needed**.
  Grammar traces the real flat `key=value` diff form (`"width=300.5 height=400.25 text=\"changed
  text\""` / `""`). Protocol = the same 24-byte `.spk` superblock-header shape `stdio.txt`'s own
  diff protocol documents, field-for-field confirmed against the real dump's own bytes.

### `stdio.pdf.1.7` (real hand-rolled object-graph codec)

- **Snapshot**: grammar = `hex-body` (real `print_dsl` shape, artifact-mark `"stdio.pdf.1.7"`).
  Protocol = `framing record; header fixed 5 { field magic fixed 5 }` (real `%PDF-` magic) +
  `backward xref_trailer_region magic 0x7374617274787265 { tail bytes }` (real backward-scan
  anchor mirroring `find_last_subslice(data, b"startxref")`) — models the container framing per
  the M2/PDF-1.7 exclusion exactly: header + bounded xref/trailer region located via backward
  scan, full indirect-object-graph resolution stays Rust-side.
- **Mutations**: `PdfMutation` (15 variants, confirmed `#[derive(dsl::DslOps)]` rejected for real)
  — grammar traces every real `print_pdf_mutation`/`parse_pdf_mutation` keyword and value
  micro-syntax verbatim (the full recursive `pdf-object` COS grammar: `Z`/`B`/`I`/`R`/`S`/`N`/`A`/
  `D`/`F`/`T` tags, `path`/`objref`/`page`/`info`/`snapshot` literals). **`OpBinary` genuinely
  upgraded** from F6's `print_op().into_bytes()` shortcut to a real `format u8 | tag u8 | variant
  payload` frame, following `📰xml`'s own hand-rolled binary-upgrade template exactly: new
  `pub(crate)` recursive binary primitives (`enc_pdf_object_bin`/`enc_pdf_snapshot_bin`/
  `enc_pdf_page_bin`/`enc_pdf_info_bin`/`enc_objref_bin`/`enc_path_bin`/`enc_box_bin`, all
  LEB128-varint/length-prefixed, `store::ByteReader`/`store::pack_rt::write_varint_u64`-based,
  zero text-as-bytes reuse) added to the diff module and `pub(crate)`-reused by mutations, same
  intra-artifact reuse convention the text codecs already use.
- **Diff**: `PdfDiff` (5 top fields, genuinely recursive `PdfValueDiff` payload, confirmed
  `#[derive(dsl::DslDiff)]` rejected for real) — grammar traces every real `print_pdf_diff`/
  `enc_pages_diff`/`enc_dict_diff`/`enc_array_diff`/`enc_value_diff`/`enc_objects_diff` tag and
  shape verbatim. **`DiffCodec` genuinely upgraded** from the F6 shortcut to a real `format u8 |
  flags u8 | [declared_version][info][pages][objects][trailer]` frame (5-bit presence mask, fixed
  field order matching `print_pdf_diff`'s own emission order), backed by the same real recursive
  binary primitives (`enc_pages_diff_bin`/`enc_dict_diff_bin`/`enc_array_diff_bin`/
  `enc_value_diff_bin`/`enc_objects_diff_bin`, all newly written, genuinely recursive).

## Registration

Both standards got a real 5-role `LanguageSpec` registration (`register_pilot_languages()`, wired
into each standard's own `register()`): `stdio.pdf`/`.op`/`.diff`/`.pack`/`.spr` for 1.4,
`stdio.pdf.1.7`/`.op`/`.diff`/`.pack`/`.spr` for 1.7 — 10 roles total, all `dsl::passthrough_hooks`.

`dsl::registry::register_schema_spec` was called for 1.4 ONLY (`"stdio.pdf"` ->
`PdfSnapshot::__dsl_spec`, `"stdio.pdf#diff"` -> `PdfDiff::__dsl_diff_spec`) — genuinely real specs
exist there via the derive path. It was deliberately NOT called for 1.7 — no `__dsl_spec`/
`__dsl_diff_spec` exists (fully hand-rolled types, confirmed rejected for real, documented above)
— filed as `mechanism_gaps` rather than fabricated.

## Real fixtures

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (1.4, existing file REWRITTEN — the prior
  content, `68656c6c6f` with no preamble line, was a stale pre-Phase-2 placeholder, not genuine
  `print_dsl` output) and `🎒️example.pack.semio` (1.4, NEWLY added) — both genuine
  `print_dsl(demo_pdf_snapshot())`/`encode_pack(demo_pdf_snapshot())` output, asserted equal by
  `fixture_honesty_law`.
- `🏅️standards/🔖️1.7/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio`
  (NEW folder, mirroring gif 87a/89a's own per-standard-fixture-folder precedent exactly) — genuine
  `print_dsl(demo_pdf17_snapshot())`/`encode_pack(demo_pdf17_snapshot())` output.

Both generated via a temporary `#[test] #[ignore] fn debug_generate_fixtures` calling the REAL
Rust encoders directly and writing straight to the target paths (`std::fs::write`), run once via
`cargo test ... -- --ignored --nocapture`, then deleted before the final verification run.

## Conformance laws

All 6 laws (`committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`) added to both `⚙️engine/🦀️component.rs`'s own `tests::conformance_laws`
module (never a framework file), plus an extra `op_diff_codec_binary_roundtrip_law` for 1.7
specifically exercising the newly-upgraded binary frames independently of the dialect-level
`protocol_walk_law` check. 1.7's `protocol_walk_law` asserts a bounded `consumed` (not `==
bytes.len()`) for the snapshot/pack facet only, per M2's own documented `backward`-block exception
— mutations/diff frames still assert exact consumption.

## JSON-transfer census

`grep -rn "serde_json::to_vec\|from_slice\|to_string\|from_str\|::Value"` across
`✏️s/…/📄️pdf/**/*.rs` — **zero hits**. Nothing to fix; both standards were already clean (matches
the P2-W0 census, which did not list pdf among its 4 literal-JSON violators).

## Verification (real commands run, not assumed)

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"
  -> 157 passed, 0 failed, 0 ignored (up from the 142/0 baseline; +15 net: 12 new conformance-law
     tests [6 laws x 2 standards] + 2 demo_snapshot_round_trip + 1 op_diff_codec_binary_roundtrip_law)

cargo test -p semio-s-plugin-stdio --lib
  -> 1800 passed, 6 failed, 1 ignored. All 6 failures are in artifacts::gltf::standards::v2_0::
     engine::tests::conformance_laws::* (a DIFFERENT FG3 sibling agent's in-progress gltf work,
     confirmed via git status --porcelain showing gltf files modified by that other session, not
     this one) — classified by file path per the ticket's own standing "classify, don't chase"
     guidance, not investigated further; zero pdf-scoped failures.

bun run ./📜️script.ts policy
  -> zero hits for pdf under POLICY_GRAMMAR_PARSEABILITY / POLICY_PROTOCOL_PARSEABILITY /
     POLICY_FIXTURE_HONESTY / POLICY_LANGUAGE_REGISTRATION / POLICY_STDIO_JSON_TRANSFER_BAN (the
     5 policies this ticket's own recipe names as the relevant gate). Other pdf-tagged findings in
     the full policy output (composer ComposerEntry-impl detection, OnceLock item-scope, artifact-
     schema type-name-prefix mapping, emoji-variation-selector taxonomy) belong to unrelated policy
     categories outside this wave's scope and pre-date this session's edits (confirmed: none of
     them cite any file this session touched).

git status --porcelain -- "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf"
  -> only files under 🏅️standards/🔖️1.4/, 🏅️standards/🔖️1.7/, and the two 📚️examples/🎬️demo fixture
     paths — confirms the ownership boundary was respected throughout.
```

## Mechanism gaps (recorded, not locally "fixed")

| id | engine area | symptom | standard(s) hit | honest workaround |
|---|---|---|---|---|
| `protocol-framing-magic-fixed-8-bytes` | `Framing::Magic`, `magic_bytes(u64)->[u8;8]` | Unconditionally reads/compares exactly 8 bytes; PDF's real magic is 4 (1.4) / 5 (1.7) bytes | both | `framing record` + `header fixed N { field magic fixed N }` — real field, walked not byte-validated at the protocol layer (same fix gif87a's own file already documents) |
| `protocol-backward-magic-fixed-8-bytes` | `Block::BackwardScan`'s `magic`, also u64-parsed via `trim_be_bytes(parse_u64_literal(...))` | Real anchor keyword `startxref` is 9 bytes, 1 over the u64 cap | 1.7 | Anchor on the first 8 bytes (`startxre`), documented as unique-enough and harmless given the trailing byte lands inside the already-opaque tail |
| `protocol-prim-ref-recursion` | `walk_protocol`'s `Prim::Ref` arm | Unconditionally errors — `PdfObject`/`PdfValueDiff`/`PdfSnapshot` are genuinely recursive/data-carrying, can't be field-by-field protocol-described | 1.7 mutations + diff | Real fixed header (`format`/`tag` or `format`/`flags`) individually walked; the recursive payload is one opaque trailing `bytes` chain — Rust side stays genuinely, fully recursive binary, round-trip tested independently |
| `protocol-cos-text-not-byte-fixed-width` | protocol `Prim` family | PDF's own internal COS structure (object numbers, `/Length` values, xref offsets, dictionaries) is ASCII-decimal/text with no fixed byte width anywhere — no "read decimal digits to a delimiter" `Prim` exists | 1.7 (and would hit 1.4 too if it grew real xref parsing) | Everything past the fixed 5-byte header magic (until the backward-scan anchor) stays one opaque region; only the header magic and the backward anchor point are individually protocol-walked — matches the ticket's own binding M2/PDF-1.7 exclusion precisely |
| `register-schema-spec-needs-recordspec` | `dsl::registry::register_schema_spec` | Requires a real `fn() -> RecordSpec`; 1.7's types are fully hand-rolled (no `DslField` impl for `PdfObject`/`PdfValueDiff`/`PdfPathSegment`) | 1.7 only (1.4 IS registered — real derive-path specs exist there) | Skipped for 1.7, documented in `register_pilot_languages`'s own doc comment rather than fabricating an unrelated spec |
| `txt-opbinary-record-body-wire-is-framework-generic` | protocol/pack | Past `format`/`ordinal`, the record-body wire (`os_pack::encode_record_body`) is framework-generic, not `Array`/`Ref`/`repeat`-expressible | 1.4 mutations (inherits the exact gap `stdio.txt`'s own identical protocol file already documents) | `format`/`ordinal` genuinely walked; record-body bytes stay one opaque tail |
| `txt-diffcodec-spk-container-is-framework-level` | protocol/pack | `#[derive(dsl::DslDiff)]` routes through the full `.spk` document container, framework-generic, not per-artifact | 1.4 diff (same gap `stdio.txt`'s own file documents) | 24-byte superblock header genuinely walked; everything past it (segment/chunk/symbol tables, compressed body, 84-byte footer) stays one opaque tail |

## Files touched (all within `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/**`)

**Rewritten (real dialect content, were pre-Phase-2 ABNF/hex-dump placeholders before):**
- `🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/{📝️text/📖️component.grammar.semio,💾️binary/📡️component.protocol.semio}` (6 files)
- `🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/{📝️text/📖️component.grammar.semio,💾️binary/📡️component.protocol.semio}` (6 files)

**Rust — modified (real binary upgrade + conformance laws + registration):**
- `🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — added `BinaryPrimitives`/
  `ObjectValueBinaryCodecs`/`DiffValueBinaryCodecs` regions; replaced `DiffCodec::encode_diff`/
  `decode_diff` with the real binary frame.
- `🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — replaced
  `OpBinary::encode_op`/`decode_op` with the real binary frame, reusing the diff module's new
  binary primitives.
- `🏅️standards/🔖️1.4/⚙️engine/🦀️component.rs` — added `register_pilot_languages`/
  `register_schema_specs`/`demo_pdf_snapshot` + `tests::conformance_laws` module.
- `🏅️standards/🔖️1.7/⚙️engine/🦀️component.rs` — added `register_pilot_languages`/
  `demo_pdf17_snapshot` + `tests::conformance_laws` module.
- `🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/{🧬️mutations,🔺️diff}/🦀️component.rs` — temporary
  debug-dump test additions removed after use (verified via final `git status`/diff review).

**Fixtures:**
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (rewritten, was stale) +
  `🎒️example.pack.semio` (new) — 1.4.
- `🏅️standards/🔖️1.7/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` + `🎒️example.pack.semio`
  (new folder) — 1.7.

No `📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol/registry modules,
`🧪️fixture-sweep` graduation list, or `🏪️store` file was touched.
