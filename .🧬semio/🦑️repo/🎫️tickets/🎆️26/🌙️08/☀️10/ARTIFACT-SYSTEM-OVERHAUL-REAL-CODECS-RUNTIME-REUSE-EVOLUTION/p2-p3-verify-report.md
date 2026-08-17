# P2-P3 Independent Verification — txt & binary Smoke Pilots

Scope: independent re-verification of the P3 wave's two self-reports (`p2-p3-txt-report.md`,
`p2-p3-binary-report.md`). Nothing taken on trust from either report — every claim below was
re-derived from disk (re-reading the actual committed `.grammar.semio`/`.protocol.semio` files,
re-reading the actual `#[derive(...)]`/`impl protocol::...` Rust source, re-running the test
filters and the full crate suite myself).

## Verdict: both pilots PASS. One non-blocking documentation defect found in binary's diff protocol
file; one avoidable registration-completeness gap found in binary (both flagged below, neither
blocks the wave).

---

## 1. Own-scope test filters (re-run myself)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::txt"` → **30 passed, 0 failed** — matches
  the self-report exactly.
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::binary"` → **25 passed, 0 failed** —
  matches the self-report exactly.

## 2. Grammar/protocol files — real dialect, real form (all 12 read in full)

All 6 files per artifact (`snapshot/text`+`snapshot/binary`, `mutations/text`+`mutations/binary`,
`diff/text`+`diff/binary`) use the real `dialect grammar`/`dialect protocol` header syntax with
correct `extension`/`start`/`schema`/`version` directives — genuinely parseable, not the old
one-line-header-plus-ABNF-body fossils. Content genuinely matches the artifacts' own real codec
output (cross-checked against the Rust source, not just the files' own comments):

- **txt**: snapshot grammar uses M1's `REST` raw-span terminal for the whole-body capture (`body =
  REST`) — correct, this is exactly the documented gold-standard case. Mutations/diff grammars
  match `TxtMutation`/`TxtDiff`'s real derive-generated `print_op`/`print_diff` shapes
  field-for-field (verified against the actual `keyed_field_rank`/`to_kebab` derive conventions in
  `🗣️dsl/✨️derive/🦀️component.rs`, not just asserted). Protocol files correctly model `framing
  record`/`chain utf8` for the pack payload, `field format u8`/`field ordinal varint`/`chain bytes`
  for `OpBinary`, and the real 24-byte-post-magic `.spk` superblock header for `DiffCodec` (`header
  fixed 24` — verified against `HEADER_SIZE=32` in `🎒️pack/📐️format/🦀️component.rs`, minus the
  8-byte magic already consumed by `framing magic`: 32−8=24, and the 6 listed fields
  `u16+u16+u32+u32+u32+fixed(8)` sum to exactly 24 — numerically self-consistent).
- **binary**: snapshot grammar correctly uses the framework's `hex` **macro** (not a hand-rolled
  `{INT|IDENT}*` production) for the whole-body hex dump, matching the mandatory P1-fix-report
  convention. Protocol/grammar files for mutations and diff correctly reflect the real
  `dsl::variants_binary`/`.spk`-container wire shapes.

### Finding (non-blocking, binary only): diff protocol file's declared header size is internally
inconsistent

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
declares `header fixed 32` immediately followed by the SAME 6 fields txt's own diff protocol file
lists (`version_major u16`, `version_minor u16`, `required_flags u32`, `optional_flags u32`,
`header_crc32 u32`, `reserved fixed 8`) — which sum to **24 bytes**, not 32. txt's own
byte-identical file correctly declares `header fixed 24` for the same field list after the same
8-byte magic. Root cause: the binary agent's own comment cites "the framework's own canonical
worked example" (`protocol_parse_print_round_trip_retains_body`, `📖️grammar/🦀️component.rs`
line ~2851) as justification for the `32` — but that framework test itself declares `header fixed
32` for a field list that sums to only 12 bytes, i.e. the `32` was copied from an unrelated
example without recomputing against binary's own real field list.

**Confirmed non-blocking**: read `parse_grammar`'s "header" directive handler directly
(`📖️grammar/🦀️component.rs` line 1067-1068) — `let _size = parse_u64_literal(&mut cursor)?;` — the
declared numeral is parsed and **immediately discarded**, never stored on `Block::Header` (which
holds only `Vec<Field>`), never validated against the summed field widths, and never consulted by
`walk_protocol` (`walk_fields` walks the field list directly). This is why `protocol_walk_law`
passes despite the mismatch — the walker never reads the declared `32`. It is nonetheless a real
self-consistency/documentation-accuracy defect (the file's own prose calls it "the 32-byte
superblock header," which conflates the header's overall on-disk size — 32, including the 8-byte
magic — with the number of bytes this block itself claims to consume after the magic, which should
read 24 to be internally consistent with its own field list, exactly as txt's sibling file does).
Recommend a 1-line fix (`header fixed 32` → `header fixed 24`) in a future pass; does not affect
correctness of any test or of `walk_protocol`'s real behavior.

## 3. DiffCodec/OpBinary — confirmed genuinely NOT the F6 text-as-binary shortcut (both artifacts)

Read the actual `#[derive(...)]` attributes and `impl protocol::...` bodies directly, not the
reports' claims:

- `TxtMutation`/`BinaryMutation` both derive `dsl::DslOps`; both `impl protocol::OpBinary` bodies
  are pure forwards to `dsl::variants_binary::encode_op`/`decode_op` (confirmed by line-reading
  both files — `🧬️mutations/🦀️component.rs` for each artifact). Neither calls `print_op()` anywhere
  in the encode path.
- `TxtDiff`/`BinaryDiff` both carry `#[derive(Clone, Debug, Default, PartialEq, Serialize,
  Deserialize, ArtifactSchema, dsl::DslDiff)]` (grepped the struct declarations directly) — the
  derive-generated `DiffCodec::encode_diff`/`decode_diff` route through
  `store::pack_rt::encode_document` (confirmed by the derive macro's own module, not just the
  artifacts' comments).

Both artifacts: `diffcodec_binary_upgraded = false`, `opbinary_binary_upgraded = false` are
correctly reported — nothing needed upgrading, both were real before this wave, self-reports are
accurate here.

## 4. Fixtures — confirmed real (byte-level inspection, not trusted from either report)

- **txt** `🗣️example.dsl.semio`: `semio stdio.txt.dsl v1\nHello, stdio.txt!\n` — genuine preamble
  line present. `🎒️example.pack.semio` (hex-dumped myself): `8953 454d 0d0a 1a0a` (real SEMIO
  binary magic) + `u32le` token length `0x11`=17 + `"stdio.txt.pack v1"` token + the raw UTF-8
  payload `"Hello, stdio.txt!\n"` verbatim — genuinely matches the snapshot grammar's own claimed
  form field-for-field.
- **binary** `🗣️example.dsl.semio`: `semio stdio.binary.dsl v1\n68656c6c6f` — `68656c6c6f` decodes
  to ASCII `"hello"`. `🎒️example.pack.semio` (hex-dumped myself): same real SEMIO magic + token
  length `0x14`=20 + `"stdio.binary.pack v1"` token + payload bytes `68 65 6c 6c 6f` = literally
  `"hello"` — matches the dsl fixture's hex content exactly (round-trip consistent).

Both fixtures are genuine, non-fake, and internally consistent with each other.

## 5. Registration

- **txt**: 5-role `LanguageSpec` (`Document`/`Ops`/`Diff`/`Pack`/`Spr`) confirmed present, grepped
  directly. `register_schema_specs()` confirmed called from `register()`, registering
  `"stdio.txt"` (→ `TxtSnapshot::__dsl_spec`) and `"stdio.txt#diff"` (→ `TxtDiff::__dsl_diff_spec`)
  — both genuinely derivable (`TxtSnapshot` derives `dsl::DslRecord`). `schema_spec_registration_resolves`
  test passed in the own-scope run above. **Fully confirmed, no gaps.**
- **binary**: 5-role `LanguageSpec` confirmed present, grepped directly (all 5 roles, correct
  grammar/protocol wiring per role). **`register_schema_spec` was NOT called at all** — confirmed
  by reading `register()`'s body (only `register_pilot_languages()`, no `register_schema_specs()`
  function exists in this artifact). The report's own justification ("three independently-derived
  specs... no single canonical choice the API expects") is only actually true for the **mutations**
  facet (one spec per `BinaryMutation` variant via `DslVariants`) — `BinarySnapshot` derives
  `dsl::DslRecord` and `BinaryDiff` derives `dsl::DslDiff` (confirmed by grep of both structs'
  `#[derive(...)]` lines), meaning both genuinely have a single canonical `__dsl_spec`/
  `__dsl_diff_spec` exactly like `TxtSnapshot`/`TxtDiff` do — txt registered exactly these same two
  under `"stdio.txt"`/`"stdio.txt#diff"` in the SAME wave, with no API obstacle. Binary could have
  done the identical two calls (`"stdio.binary"`, `"stdio.binary#diff"`) and only genuinely needed
  to skip the mutations-facet registration for the stated multiplicity reason. **This is a real,
  avoidable partial gap** — registration_confirmed should be read as "5-role LanguageSpec: yes;
  register_schema_spec: no (0 of 2 feasible calls made, despite the artifact's own report
  correctly identifying that RecordSpecs exist)."

## 6. P1 pitfalls — grepped all 12 committed files directly, none recur

- **Bare `(...)` grouping**: zero bare-paren grouping in any production (the only `(` occurrences
  are legitimate `Array(u8, Field(seg_len))` macro calls in binary's diff protocol repeat-block,
  which is correct macro-call syntax, not grouping).
- **Hand-rolled `{INT|IDENT}*` hex production**: zero occurrences in any of the 12 files. binary
  correctly uses the bare `hex` macro reference with no matching production; txt has no hex-shaped
  content at all (its raw payload is UTF-8 text via `REST`, not hex).
- **Reserved-keyword production names** (`extension`/`use`/`start`/`comment`/`string`): zero
  productions named any of the five reserved words in either artifact's files.
- (4th pitfall, framework comment-scanning bug): already fixed at the framework level per
  `p2-p1-fix-report.md`; not re-testable per-artifact, but neither artifact's files depend on any
  comment-adjacent `?`/`|`/`"` construct that would have re-triggered it, and both artifacts'
  `committed_grammar_and_protocol_files_parse`/`committed_facet_files_parse` tests pass, which
  transitively exercises the fixed lexer path.

## 7. Full crate suite (run once, at the end)

`cargo test -p semio-s-plugin-stdio --lib` → **1671 passed, 0 failed, 1 ignored** — matches both
self-reports exactly. No failures to classify as external churn; the tree was quiet during this
verification pass. This count covers all 6 artifacts touched so far in the P-ladder (json, csv,
zip, png, txt, binary) plus every other stdio artifact, with zero regressions.

---

## Per-artifact summary

| Artifact | tests | real_dialect | binary_frame | fixtures_real | registration | p1_pitfalls_avoided |
|---|---|---|---|---|---|---|
| txt | 30/0 | yes | yes | yes | yes (full 5-role + 2/2 feasible schema_spec calls) | yes |
| binary | 25/0 | yes (1 non-blocking numeral inconsistency, see §2) | yes | yes | partial (5-role yes; schema_spec 0/2 feasible calls made) | yes |

## Recommendation

Neither finding blocks closing this wave — both are cosmetic/completeness gaps, not correctness
bugs, and both are cheap fixes for whoever next touches `stdio.binary`:
1. `🔺️diff/💾️binary/📡️component.protocol.semio`: `header fixed 32` → `header fixed 24`.
2. Add a `register_schema_specs()` call to binary's `⚙️engine/🦀️component.rs`, mirroring txt's exact
   pattern, for `"stdio.binary"` (`BinarySnapshot::__dsl_spec`) and `"stdio.binary#diff"`
   (`BinaryDiff::__dsl_diff_spec`) — leaving only the genuinely-unregistrable mutations facet as
   the documented gap.
