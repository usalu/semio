# P2 FG2 — ☁️las (standard 1.0) — Real Grammar/Protocol + Binary-Frame Upgrade Report

Wave: FG2. Artifact: `☁️las` / standard `1.0`. Ownership boundary:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/**` (never touched `📦️glue.rs`, `📜️script.ts`, the SDK
traits, the schema/dsl/protocol/registry modules, the framework `🧪️fixture-sweep` graduation
list, or `🏪️store`).

## 1. Starting state

F2 (an earlier wave) had already brought `LasSnapshot`/`LasHeader`/`LasVlr`/`LasPoint`/`LasDiff`/
`LasMutation` to full LAS 1.0 completeness (25 real header fields, index-keyed `vlrs`/`points`
collections, 15 real mutation variants) and rewrote the snapshot facet's KSY/ABNF/Spicy leaves
honestly — but F2's own report explicitly deferred all grammar/protocol dialect work to a future
wave. On inspection at the start of this wave:

- `📸️snapshot/📝️text/📖️component.grammar.semio` and `📸️snapshot/💾️binary/📡️component.protocol.
  semio` contained pseudo-ABNF text (`%x`, single-quoted literals, `SP`/`NL`, bare `2OCTET`
  productions) entirely outside this framework's real `.grammar.semio`/`.protocol.semio` dialect
  (`dialect grammar\ngrammar <id>\n...` / `dialect protocol\nprotocol <id>\nversion <n>\n...`).
- `🔺️diff/📝️text/📖️component.grammar.semio`, `🔺️diff/💾️binary/📡️component.protocol.semio`,
  `🧬️mutations/📝️text/📖️component.grammar.semio`, `🧬️mutations/💾️binary/📡️component.protocol.
  semio` were all still the universal `{schema, payload=*OCTET}` F6-era stub — matching NEITHER
  the real dialect NOR `LasDiff`/`LasMutation`'s actual hand-rolled text shapes.
- `LasDiff::encode_diff`/`decode_diff` were `print_diff().into_bytes()` — the F6 text-as-binary
  shortcut.
- `LasMutation::encode_op`/`decode_op` were `print_las_mutation(self).into_bytes()` — same
  shortcut.
- `register_pilot_languages()` registered only 1 role (`stdio.las`, Document; `extension:
  Some("bin")`, a stale copy-paste from the binary artifact's own boilerplate).
- No conformance-law test module existed (`committed_facet_files_parse`,
  `grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
  `protocol_walk_law`, `fixture_honesty_law`).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was an 11-byte stub (`68656c6c6f`, literally
  binary's own "hello" fixture copy-pasted, no `semio stdio.las.dsl v1` preamble at all); no
  `🎒️example.pack.semio` existed.

## 2. What changed — Rust binary codecs (`DiffCodec`/`OpBinary`)

### `🔺️diff/🦀️component.rs` — `#region 🔖️BinaryDiffCodec`

Real LEB128-varint-framed binary, replacing the F6 shortcut:

- **Shared record binary primitives** (`pub(crate)`, reused by the mutations facet below):
  `write_bytes_lp`/`read_bytes_lp`/`write_str_lp`/`read_str_lp` (length-prefixed via
  `store::pack_rt::write_varint_u64`/`store::ByteReader::read_varint_u64`), `enc_header_bin`/
  `dec_header_bin` (full 25-field `LasHeader` record), `enc_vlr_bin`/`dec_vlr_bin` (full 4-field
  `LasVlr`), `enc_point_bin`/`dec_point_bin` (full 14-field `LasPoint`, incl. both `Option<T>`
  fields as an inline `0`/`1`-tagged presence byte).
- **Sparse diff binary codecs**: `enc_vlr_diff_bin`/`dec_vlr_diff_bin` (4-bit field-presence mask),
  `enc_point_diff_bin`/`dec_point_diff_bin` (14-bit mask, incl. both TRI-STATE fields —
  `Option<Option<f64>>`/`Option<Option<(u16,u16,u16)>>` — as mask-bit-gated inline `0`/`1`-tagged
  values), `enc_vlrs_diff_bin`/`dec_vlrs_diff_bin` and `enc_points_diff_bin`/`dec_points_diff_bin`
  (the two index-keyed collection triples: varint-counted `removed`/`modified`/`added` lists).
- **`LasDiff::encode_diff`/`decode_diff`**: `format u8 (=OP_BINARY_FORMAT) | header_mask u32 (bit
  i = header field i present, declaration order) | <only the present header fields' values, in
  order> | <vlrs diff if present> | <points diff if present>`.
- **`demo_diff_cases()`** (`pub(crate)`, `#[cfg(test)]`) — `LasDiff::default()` plus both
  directions of a real `between()` over two fully-populated snapshots (25/25 header fields
  differ, both point tri-states exercised both directions, both collections non-trivially
  changed) — single source of truth now shared by `diff_codec_text_binary_roundtrip_law` (this
  file) AND `diff_grammar_conformance_law`/`protocol_walk_law` (engine.rs). `base_point`/
  `base_vlr` were moved out of `mod tests` to module scope (`pub(crate)`, `#[cfg(test)]`) so both
  `demo_diff_cases()` and `mod tests` share one copy.

### `🧬️mutations/🦀️component.rs` — `#region 🔖️BinaryOpCodec`

- `enc_f64x3_bin`/`dec_f64x3_bin` (3×fixed-`f64`, for `SetScaleAndOffset`/`SetBounds`'s bare-tuple
  fields), `enc_snapshot_bin`/`dec_snapshot_bin` (a whole `LasSnapshot`: length-prefixed `schema`
  string + `diff::enc_header_bin` + varint-counted `vlrs`/`points` lists of full records, reusing
  the diff facet's own record encoders rather than a second, independently-drifting copy).
- **`LasMutation::encode_op`/`decode_op`**: `format u8 | tag u8 (0..14, declaration order) |
  <variant-specific fields, genuinely field-by-field for all 15 variants>`.
- **`demo_mutation_cases()`** (`pub(crate)`, `#[cfg(test)]`) — one case per variant (15 total),
  moved out of the pre-existing `op_text_binary_roundtrip_law` test's inline case list (now the
  single source of truth for that test AND `ops_grammar_conformance_law`/`protocol_walk_law`).
  `vlr`/`point`/`base_snapshot` fixture helpers were likewise moved from `mod tests` to module
  scope for the same reuse reason.

Both `DiffCodec`/`OpBinary` upgrades independently round-trip tested
(`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`) — both pass.

## 3. What changed — Grammar/protocol files (real dialect, replacing pseudo-ABNF placeholders)

All 6 files (3 grammar, 3 protocol) rewritten from scratch in the real M1/M2 dialect:

- **`📸️snapshot/📝️text/📖️component.grammar.semio`**: `document = artifact-mark hex-body`,
  `artifact-mark = "stdio.las"`, `hex-body = hex` — the same shape `stdio.binary`'s own hex-dump
  snapshot grammar uses (binary-native artifact, text facet is opaque-hex-by-design; the grammar
  never re-describes the byte-level LAS structure that hex run decodes to).
- **`📸️snapshot/💾️binary/📡️component.protocol.semio`**: `framing record` (NOT `framing magic` —
  `Framing::Magic` always consumes exactly 8 bytes at position 0, confirmed by direct read of
  `walk_protocol`, but LAS's real signature is only 4 bytes; the signature is instead modeled as
  the header's own first field, `field magic fixed 4`) + `header fixed 227` with all 31 real
  fields (magic, file_source_id, global_encoding, project_id_guid, version_major/minor,
  system_identifier, generating_software, creation_day_of_year/year, header_size,
  offset_to_point_data, number_of_vlrs, point_data_format_id, point_data_record_length,
  number_of_point_records, 5×points_by_return, 12×f64 scale/offset/bounds) + `chain payload
  bytes` (VLR list + point list combined, one opaque trailing tail — see §4's mechanism gap).
- **`🔺️diff/📝️text/📖️component.grammar.semio`**: real one-line diff-text grammar matching
  `print_las_diff`/`parse_las_diff` exactly — 25 optional `key=value`/`key=hex` parts (one
  production per header field, matching declaration order) + the two index-keyed collection-triple
  productions (`vlrs`/`points`, per the recipe's §1.4 copy-pasteable shape), tri-state `[0]`/
  `[1,<T>]` tags for `gps_time`/`rgb`, single-letter `tag:value` sparse-field productions for
  `LasVlrDiff`/`LasPointDiff`.
- **`🔺️diff/💾️binary/📡️component.protocol.semio`**: `framing record` + `field format u8` +
  `field header_mask u32` + `chain payload bytes` — the two real leading fields modeled precisely,
  the variable tail (which header values are present + both collection triples) stays opaque (see
  §4).
- **`🧬️mutations/📝️text/📖️component.grammar.semio`**: real one-line op-text grammar matching
  `print_las_mutation`/`parse_las_mutation` exactly — 15 variant productions, 14 modeled precisely
  field-by-field, `set-snapshot` modeled as `REST` (genuinely unbounded nested composite, same
  honest-boundary treatment zip's own `set-snapshot`/`add-entry` use).
- **`🧬️mutations/💾️binary/📡️component.protocol.semio`**: `framing record` + `field format u8` +
  `field tag u8` + `chain body bytes` — same two-real-fields-then-opaque-tail shape as `stdio.
  binary`'s own already-real mutations protocol (which documents the identical limitation for its
  own `dsl::variants_binary`-routed ordinal).

All 6 files verified via `committed_facet_files_parse` (parse under the real dialect) AND
`grammar_conformance_law`/`ops_grammar_conformance_law`/`diff_grammar_conformance_law` (the
`Recognizer` accepts real `print_dsl`/`print_op`/`print_diff` output) AND `protocol_walk_law`
(`walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, `consumed ==
bytes.len()` in every case).

## 4. Mechanism gaps hit (both already consolidated in the ticket's recipe §5, not new)

1. **`protocol-array-of-records`** — LAS's VLR list repeats `number_of_vlrs` times (a count
   sourced from an EARLIER-decoded header field, not a per-iteration tag/sentinel byte), and the
   point-record list is a FIXED shape chosen ONCE by `point_data_format_id` (an earlier header
   field selecting the shape for every subsequent iteration, not a per-iteration dispatch).
   `Block::Repeat`'s arms are tag-dispatched per iteration; `Prim::Array` only repeats one
   fixed-width scalar. Neither construct can express "repeat N times, N from an earlier field,
   each iteration a multi-field record" or "an earlier field selects the record shape for every
   following iteration." Same root cause hits the diff/mutations facets' `vlrs`/`points`
   collection triples (runtime-counted, index-keyed). **Non-blocking** — the real Rust
   `encode_las`/`decode_las`/`encode_diff`/`decode_diff`/`encode_op`/`decode_op` walk this
   genuinely, field-by-field, independently round-trip tested; only the `.protocol.semio`
   DESCRIPTION bottoms out in one opaque trailing chain past the real fixed leading fields —
   exactly the recipe's documented workaround (zip's own `entries` hits the identical gap).
2. **`register-schema-spec-needs-recordspec`** — `LasSnapshot`/`LasHeader`/`LasVlr`/`LasPoint`/
   `LasDiff` are all fully hand-rolled (confirmed by a real, reverted `#[derive(dsl::DslRecord)]`/
   `DslDiff`/`DslOps` probe — two independent, real compiler-error blockers: `LasPointDiff`'s
   tri-state `Option<Option<T>>` fields, and the complete absence of a blanket `DslField` impl for
   bare tuples like `(f64, f64, f64)`, hit by both `LasPointDiff::rgb` and
   `LasMutation::SetScaleAndOffset`/`SetBounds` — documented in `🔺️diff/🦀️component.rs`'s own
   `HandcraftedDiffCodec` doc comment, pre-existing from F2). No `fn() -> RecordSpec` exists to
   register. **Non-blocking** — `register_schema_specs()` is a documented no-op, per the recipe's
   own "skip and file, never fabricate" rule (json/csv/zip/png's own situation).

Neither gap required a mechanism-level fix — both are pre-existing, already-documented dialect
boundaries, exercised honestly.

## 5. Language registration (5-role, was 1)

`register_pilot_languages()` in `⚙️engine/🦀️component.rs` now registers all 5 roles:
`stdio.las` (Document, grammar+protocol = snapshot facet), `stdio.las.op` (Ops, grammar+protocol
= mutations facet), `stdio.las.diff` (Diff, grammar = diff facet, `protocol: None` per the
5-role scheme's own documented shape), `stdio.las.pack` (Pack, protocol = snapshot facet),
`stdio.las.spr` (Spr, protocol = mutations facet) — all `dsl::passthrough_hooks`. Also fixed the
Document role's stale `extension: Some("bin")` (an F2-era copy-paste from the binary artifact's
own boilerplate) to the artifact's real `Some("las")`.

`register_schema_specs()` added as a documented no-op (§4 item 2), called from `register()`.

## 6. Real fixtures regenerated

`demo_las_snapshot()` added to `⚙️engine/🦀️component.rs` (point-data-format-0, 1 VLR, 2 points —
small but genuinely representative, non-default in every field). Generated via a temporary
`#[ignore]` `debug_generate_demo_fixtures` test that called the REAL `store::ArtifactDsl::
print_dsl`/`store::ArtifactPack::encode_pack` directly, captured the printed bytes, wrote them to
disk, then deleted the temp test (per the recipe's own mandated procedure):

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — was an 11-byte fake stub
  (`68656c6c6f`, no preamble); now 681 bytes, genuine `semio stdio.las.dsl v1\n` + real hex body.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — did not exist; now 358 bytes, genuine
  `encode_pack` output (SEMIO envelope + real LAS binary buffer).

Both asserted byte-for-byte honest by `fixture_honesty_law` (parse-back AND re-encode equality).

## 7. Conformance-law test module

Added `mod conformance_laws` inside `⚙️engine/🦀️component.rs`'s existing `mod tests`, the
canonical 6-law shape: `committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`. All 6 pass.

## 8. Test results

`cargo test -p semio-s-plugin-stdio --lib "artifacts::las"` → **34 passed, 0 failed** (was 21
passed before this wave; +13 new: `demo_snapshot_round_trip`, the 6 conformance laws, plus the
existing `diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law` now exercising
REAL binary frames instead of the text-as-binary shortcut, and all pre-existing F2-era tests
still green unmodified in behavior).

`cargo test -p semio-s-plugin-stdio --lib` (whole crate, no filter) → **1747 passed, 8 failed, 2
ignored**. All 8 failures are in `bmp`/`gif`/`jpg` (`committed_facet_files_parse`,
`diff_grammar_conformance_law`, `ops_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law` — the exact same conformance-law names this wave introduces for `las`,
confirming these are OTHER sibling F2-wave agents' in-progress work on their own artifacts, not
fallout from `las`) — none reference `las` anywhere in the failure output; classified per the
ticket's own repo-rules digest ("sibling artifact within this same wave, mid-edit — wait/retry,
don't chase") and left untouched.

## 9. Policy gate (`bun ./📜️script.ts policy`)

Filtered the regenerated `.🦑️repo/⚡️cache/breaches/compose.json` for every breach mentioning
`☁️las`/`stdio/las` (80 hits total):

- `grammar-parseability` ×3, `protocol-parseability` ×3, `fixture-honesty` ×1,
  `language-registration` ×1 — **every one of these is `-stale-`**, its own summary text reading
  "already looks like…"/"already registers 5 >= 5 languages"/"fixtures are already genuine" —
  i.e. **zero real breaches for the 4 rule kinds this wave's scope covers**, allowlist entries
  ready for the ticket's periodic policy-shrink pass to prune (not touched myself, per the
  recipe's own rule).
- `grammar-honesty`, `facet-mirror-drift`, `diff-algebra`, `field-sweep-presence`,
  `json-transfer-ban` — **zero hits** (real or stale) for `las`.
- The remaining 68 hits (`taxonomy-dirs-artifact`, `semio-examples-tests`,
  `emoji-vs16`, `mutation-facet-missing`, `artifact-engine-folder-missing`,
  `artifact-schema-facet-missing`, `artifact-schema-prefix-unknown`,
  `stdio-artifacts/composer-trait`, `os-state-authority-static-OnceLock`) are all pre-existing,
  unrelated to grammar/protocol/diff/mutations/fixtures/registration (directory-taxonomy and
  composer-trait rules that predate this wave and are outside its checklist) — left untouched.

`grep -rn "serde_json::to_vec\|serde_json::from_slice\|serde_json::to_string\|serde_json::from_str\|serde_json::Value"` across every `☁️las` `.rs` file → **zero hits** (already clean).

## 10. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (new)

## 11. Deviations

1. Fixed `register_pilot_languages()`'s Document-role `extension: Some("bin")` → `Some("las")`
   (matches `store::ArtifactDsl::EXTENSION` for `LasSnapshot`) — a small in-scope correctness fix
   to a stale F2-era copy-paste, not a scope expansion.
2. The pre-existing orphan `📚️examples/🎬️demo/🖼️assets/🎒️example.bin` (5-byte ASCII `"hello"`,
   an apparent leftover copy from the `stdio.binary` artifact's own demo assets) is unreferenced
   by any `.rs` file in this tree and was left untouched — outside this wave's fixture checklist
   (`🗣️example.dsl.semio`/`🎒️example.pack.semio` only).
3. Diff/mutations facets' NESTED per-representation grammar leaf pairs that are NOT the live-wired
   `.grammar.semio`/`.protocol.semio` files (e.g. any sibling `.g4`/`.ebnf`/`.ksy`/`.abnf`/
   `.spicy` leaves under those same facet dirs, if present) were not touched — matches F2's own
   documented, accepted scope boundary (only `register_pilot_languages`-referenced files are
   live-wired; this wave's mandate was specifically the real-dialect `.grammar.semio`/`.protocol.
   semio` pair plus the binary-frame Rust upgrade, both fully delivered).
4. Whole-crate `cargo test` shows 8 pre-existing failures in `bmp`/`gif`/`jpg` — confirmed
   unrelated to `las` by file-path classification (§8) and left untouched per the ticket's repo
   rules.

## 12. Verification commands run

- `cargo check -p semio-s-plugin-stdio --lib` → clean (warnings only, pre-existing/unrelated).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::las"` → 34 passed, 0 failed.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → 1747 passed, 8 failed (bmp/gif/jpg,
  not mine), 2 ignored.
- `bun ./📜️script.ts policy` → regenerated `compose.json`, filtered for every `☁️las`/`stdio/las`
  breach (80 hits) → 0 real (non-stale) breaches for grammar-parseability/protocol-parseability/
  fixture-honesty/language-registration/json-transfer-ban/grammar-honesty/facet-mirror-drift/
  diff-algebra/field-sweep-presence.
- `grep -rn "serde_json::to_vec\|serde_json::from_slice\|serde_json::to_string\|serde_json::from_str\|serde_json::Value" ✏️s/…/☁️las --include="*.rs"` → 0 hits.
