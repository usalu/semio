# P2-FG3 — ☁️ply 1.0 — real grammar/protocol dialect, real binary diff/op frames, fixtures, conformance laws

## Summary

`stdio.ply` (standard 1.0) entered this wave with: (a) both `.grammar.semio`/`.protocol.semio`
files for every facet (snapshot, diff, mutations) still written in the pre-Phase-2 ABNF-flavored
placeholder syntax (`dialect protocol stdio.ply.diff` / `magic = %x00` / `payload = *OCTET` for
diff and mutations; `%x`-hex-literal ABNF for the snapshot pair) — never the real
`grammar-recipe.md` dialect; (b) `DiffCodec::encode_diff`/`decode_diff` and
`OpBinary::encode_op`/`decode_op` still on F6's `print_diff()/print_op().into_bytes()` text-as-
binary shortcut; (c) only ONE `LanguageSpec` role registered (`stdio.ply`, Document) instead of
the full 5; (d) zero conformance-law tests; (e) placeholder demo fixtures (`example.dsl.semio` =
literal string `"Hello, stdio.ply!"`, `example.ply` = `"Hello, stdio.txt!"` — copy-paste garbage,
not real `print_dsl` output). All five are closed by this wave. Two genuine, in-scope Rust bugs
were also found and fixed along the way (see "Bugfixes" below).

## Grammar files rewritten (3)

- `🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` — the real PLY 1.0 ASCII
  header + data-row syntax: magic line, `format` line (all 3 format-kind alternatives modeled,
  even though `print_dsl` only ever emits `ascii`), any number of `comment`/`element`/`property`
  declarations, `end_header`, then the body. The body is modeled as `number*` (a flat run of real
  `INT`/`FLOAT` tokens) rather than per-row/per-element structure, because per-row boundaries are
  schema-external (only the ASCII header, parsed at runtime, says how many rows an element has
  and how many cells each row's list properties carry) and `Newline` is always lexer trivia (no
  structural line-boundary terminal exists) — documented in-file, same root cause as the
  protocol-side carve-out (see Mechanism gaps).
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio` — real one-line `print_diff`/
  `parse_diff` grammar (space-separated `name=value` tokens; `elements`'s real NAME-keyed
  removed/modified + INDEX-keyed added triple; `PlyProperty`/`PlyValue` data-carrying-enum tag
  grammars; the `hex` macro for every string field).
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` — real one-line
  `print_op`/`parse_op` grammar (kebab-case keyword + `arg=value` tokens, one alternative per
  `PlyMutation` variant), restating (not `use`-ing — cross-facet `use` doesn't resolve at walk
  time yet) the diff grammar's `ply-value`/`ply-property`/`element-value`/`row-value` shapes.

One real authoring bug caught and fixed during this pass, matching the recipe's own documented
pitfall #4 (`committed_facet_files_parse`/`grammar_conformance_law` caught it immediately): the
snapshot grammar's `scalar-type` production was originally wrapped across two physical lines for
readability — `parse_sequence` stops at the first `Newline` token, so the wrap silently truncated
the production and the continuation line was mis-parsed as a new, invalid production (`expected
Ident, found Pipe`). Fixed by putting it back on one physical line.

## Protocol files rewritten (3)

- **Snapshot (Pack facet)** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: `framing record`,
  `header fixed 4` with one real field `magic fixed 4` (the universal `"ply\n"` 4-byte magic,
  true across all 3 format variants), then `chain body bytes` — one opaque trailing chain for
  everything else (the `format`/`comment`/`element`/`property` header lines, `end_header`, and
  the ascii-text-or-binary-cell body). This is the honest, pre-approved M2 boundary
  (`📖️grammar-recipe.md` §6 names `ply` explicitly, alongside `pdf/1.7`, for exactly this
  reason): verified live (not assumed) that the real `Prim` primitive set has no "scan forward to
  this literal multi-byte string" construct (`MarkerScan(u8)` only scans a run of ONE repeated
  fill byte; `backward magic` only locates a FIXED-width magic from EOF) that could locate the
  unboundedly-long `end_header\n` terminator, and no construct lets a value the GRAMMAR side
  already parsed parameterize a PROTOCOL field's width/count — the same root cause PDF 1.7's own
  `/W`-array gap documents. `framing record` (not `framing magic ...`) deliberately, matching DWG
  ac1018/ac1024's own documented `protocol-framing-magic-fixed-8-bytes` reason (`Framing::Magic`
  always reads/compares exactly 8 raw bytes; ply's real magic is only 4).
- **Diff** — `🔺️diff/💾️binary/📡️component.protocol.semio`: real flag-per-field binary frame.
  `PlyDiff` has 3 PLAIN `Option<T>` fields (`format`/`comments`/`elements`, struct order) and NO
  tri-state fields, so every field uses a simple 2-way presence flag. `format` (1 byte, fixed
  width) is a plain conditional field; `comments`/`elements` are length-prefixed opaque blobs
  (`Prim::Ref` to a struct/enum still unconditionally errors at walk time —
  `protocol-prim-ref-recursion`, re-confirmed live by reading `walk_fields`, not assumed).
- **Mutations** — `🧬️mutations/💾️binary/📡️component.protocol.semio`: `header fixed 2` (`format
  u8`, `tag u8` — the real `PlyMutation` variant ordinal 0-9) + `chain payload bytes`, same shape
  as json's own hand-rolled `JsonMutation` frame (§2.5's worked example) — `PlyMutation` is
  HAND-ROLLED (confirmed un-derivable, see the module doc comment's real `cargo check` citation),
  so there's no `dsl::variants_binary`-generated frame to forward to.

## DiffCodec / OpBinary real binary upgrade

Both were confirmed, by direct reading (not assumed), still on the F6-era
`print_diff()/print_op().into_bytes()` text-as-binary shortcut. Both upgraded to real binary
frames this wave, following gif89a's own `RealBinaryPrimitives`/`RealBinaryDiffFrame` pattern
(`dsl::ByteWriter`/`dsl::ByteReader`, real LEB128 varints, real length-prefixed blobs):

- `🔺️diff/🦀️component.rs` — new `RealBinaryPrimitives` region (`write_bin_blob`/`write_bin_str`/
  `write_bin_vec`/`write_bin_option`/`write_bin_format`/`write_bin_scalar_type`/`write_bin_value`
  [handles `PlyValue`'s own self-recursion via `List`]/`write_bin_property`/`write_bin_row`/
  `write_bin_element`/`write_bin_snapshot`, all `pub(crate)` so `🧬️mutations/🦀️component.rs` can
  reuse them exactly like it already reuses the text-codec primitives) + new `RealBinaryDiffFrame`
  region (`write_bin_row_diff`/`write_bin_rows_diff`/`write_bin_element_diff`/
  `enc_elements_diff_bin`, each producing one opaque blob matching the protocol file's
  `Array(u8, Field(<name>_len))` fields). `impl protocol::DiffCodec for PlyDiff`'s `encode_diff`/
  `decode_diff` rewritten to the real frame; `print_diff`/`parse_diff` (text form) unchanged.
- `🧬️mutations/🦀️component.rs` — new `RealBinaryOpFrame` region (`op_tag`, `op_pack_err`) +
  rewritten `impl protocol::OpBinary for PlyMutation` (`encode_op`/`decode_op`), reusing the diff
  file's binary primitives for every variant's own recursive payload (`SetSnapshot`'s whole
  `PlySnapshot`, `AddElement`'s `PlyElement`, `InsertRow`'s `PlyRow`, `SetRowProperty`'s bare
  `PlyValue` incl. the recursive `List` case).

Real captured examples: `set-row-property` op binary — `format(01) tag(09) element_name_len(08)
"vertex"(hex bytes) row_index(varint 00) property_name_len(01) "x" value_tag(06=Float)
f32_le_bytes`; `PlyDiff::between(a,b)` binary — `format_flag(00) comments_flag(01)
comments_len(varint) comments_blob elements_flag(01) elements_len(varint) elements_blob` (exact
byte content depends on the two snapshots, verified round-trip in
`diff_codec_text_binary_roundtrip_law`/`op_text_binary_roundtrip_law`, both PASS).

## 5-role LanguageSpec registration

`⚙️engine/🦀️component.rs`'s `register_pilot_languages()` rewritten from registering only
`stdio.ply` (Document) to the full 5: `stdio.ply` (Document), `stdio.ply.op` (Ops),
`stdio.ply.diff` (Diff, `protocol: None` per the 5-role scheme's own shape), `stdio.ply.pack`
(Pack), `stdio.ply.spr` (Spr) — all `dsl::passthrough_hooks`, mirroring png's own exemplar.
`register_schema_spec` deliberately NOT called (see Mechanism gaps — no derivable `RecordSpec`
exists for any of `stdio.ply`'s hand-rolled types).

## Fixtures regenerated (both, genuinely real)

- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — was the literal fake string `"Hello,
  stdio.ply!"`; now genuine `print_dsl(demo_ply_snapshot())` output (captured via a temporary
  `#[ignore]`d `[DEBUG]`-prefixed test that called the real Rust encoder directly, then deleted
  before finishing — per the checklist's own mandated method). Real content: the `semio
  stdio.ply.dsl v1` preamble, a genuine ply header (magic, `format ascii 1.0`, one `comment`
  line, a `vertex` element with 3 scalar `float` properties, a `face` element with one `list
  uchar int vertex_indices` property), `end_header`, and 3 real ASCII data rows (one of them the
  list-property row: count `2` then 2 index values).
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` (**new** — did not exist before this wave)
  — genuine `encode_pack(demo_ply_snapshot())` bytes: real SEMIO envelope magic
  (`8953454d0d0a1a0a`) + token-length + `stdio.ply.pack v1` token, then the real ascii ply file
  bytes verbatim.
- `demo_ply_snapshot()` (new, `⚙️engine/🦀️component.rs`) is the single source of truth every
  conformance law and both fixtures are built from — `format: PlyFormat::Ascii` deliberately (the
  DSL/text facet always normalizes to ascii regardless of `format`, so a non-ascii demo would
  make `fixture_honesty_law`'s round-trip assertion fail; the Pack facet's genuine BINARY-variant
  behavior is instead exercised directly in `protocol_walk_law` by calling
  `encode_ply_with_format` with `BinaryLittleEndian`/`BinaryBigEndian` explicitly).

## The 6 conformance-law tests (new `conformance_laws` submodule, `⚙️engine/🦀️component.rs`)

`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law` — all added, all PASS.
`protocol_walk_law` additionally proves the SAME snapshot protocol (magic + opaque tail)
genuinely spans all 3 format variants by calling `encode_ply_with_format` directly with
`BinaryLittleEndian`/`BinaryBigEndian` (not just the ascii demo path) and asserting full
byte-consumption for each.

## Bugfixes (in-scope, real, found while implementing the above)

1. **`encode_pack_with` silently discarded `self.format`.** `🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s
   `ArtifactPack::encode_pack_with` called the ascii-forcing `engine::encode_ply(self)`
   unconditionally instead of `encode_ply_with_format(self, self.format)` — meaning
   `decode_pack(encode_pack(snap))` for any snapshot whose `format` was `BinaryLittleEndian`/
   `BinaryBigEndian` would silently come back with `format: Ascii`. The Pack facet is supposed to
   be the artifact's real, byte-exact, format-respecting on-disk representation (unlike the
   DSL/text facet, which deliberately normalizes to ascii) — fixed to call
   `encode_ply_with_format(self, self.format)`. No pre-existing test exercised `ArtifactPack`
   with a non-ascii format, so nothing broke; this is exactly the bug `protocol_walk_law`'s new
   binary-variant assertions now guard against.
2. **`header_text` never re-emitted `comments`.** `⚙️engine/🦀️component.rs`'s `header_text` was
   previously a documented scope cut ("comments are NOT re-emitted into the header on encode").
   This silently broke `decode_ply(encode_ply(snap))`'s own round-trip for any snapshot with
   non-empty `comments`, and made the new snapshot grammar's `comment-line` production
   permanently unreachable by this artifact's real `print_dsl` output. Closed: `header_text` now
   takes `comments: &[String]` and emits a real `comment <text>\n` line per entry (the decode
   side, `parse_header_text`, already handled comment lines correctly — this was purely a
   one-directional encode gap). `demo_ply_snapshot()`'s one comment (`"semio demo"`) is real,
   round-trip-exercised proof this now works.

## Mechanism gaps (new/reconfirmed for `ply`)

| gap id | engine area | symptom | blocking |
|---|---|---|---|
| `ply-ascii-header-schema-external` | `.grammar.semio`/`.protocol.semio`, both facets | The ASCII header's own `element`/`property` declarations dynamically choose the body's row/cell shape (grammar side) and the binary body's per-field width/count (protocol side) — no construct in either dialect lets a value the OTHER side already parsed parameterize a later shape, and no primitive can locate the unboundedly-long literal `end_header\n` terminator that bounds the header region. Confirmed live by reading `Prim`'s real variant set and `walk_fields`, not assumed. | No — pre-approved, documented M2 exclusion carve-out (`📖️grammar-recipe.md` §6 names `ply` explicitly, alongside `pdf/1.7`). Grammar honestly models the header structurally + the body as a flat real-token vocabulary (`number*`); protocol honestly models the universal 4-byte magic + one opaque trailing chain, same DWG-precedent treatment. |
| `protocol-prim-ref-recursion` | diff/mutations protocol files | `Prim::Ref` to a struct/enum unconditionally errors at `walk_protocol` time — re-confirmed live for `ply` (`PlyElement`/`PlyRow`/`PlySnapshot`/`PlyValue`/`PlyProperty` all reachable from some diff/op field). | No — real fixed leading fields (flags/tags) individually walked; recursive payload one opaque length-prefixed blob/tail, Rust side genuinely fully structured and round-trip-tested independently. |
| `register-schema-spec-needs-recordspec` | `dsl::registry::register_schema_spec` | Requires `fn() -> RecordSpec`; `PlySnapshot`/`PlyDiff`/`PlyMutation` have none (all hand-rolled — `PlyProperty`/`PlyValue` are data-carrying enums with no derivable `DslField`). | No — `register_schema_spec` simply not called, same treatment json/csv/zip/png's own reports document. |

## Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` → **36/36 passed, 0 failed** (23
  pre-existing F1-F6 tests + 2 pre-existing codec-round-trip tests refactored to reuse new
  `demo_diff_cases()`/`demo_mutation_cases()` + 6 new conformance-law tests +
  `demo_source_nonempty` + 4 semio-mesh-serializer tests unaffected). Full output:
  `p2-fg3-ply-scoped-test-final.txt` in this folder.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1796 passed, 4 failed, 1 ignored**.
  All 4 failures are entirely inside `📄️pdf` (`artifacts::pdf::standards::v1_4::…`/
  `artifacts::pdf::standards::v1_7::…`), a different, concurrently-running FG3 session's
  in-progress work on the pdf artifact — confirmed by (a) file path being entirely inside
  `🗿️artifacts/📄️pdf/**`, explicitly outside `ply`'s ownership boundary, and (b) repeated
  `cargo check`/`cargo test` runs during this session flipped between "pdf pack fixture missing"
  and "pdf write_varint_i64 not found" and clean, several times, with zero edits from this
  session to any pdf file — the documented "Concurrent Cargo Workspace Churn" pattern. The 1
  ignored test (`csv::…::zzz_generate_p2p1_fixtures`) is pre-existing, unrelated to `ply`. Per the
  ownership boundary, the pdf failures were left untouched. Full tail output:
  `p2-fg3-ply-full-crate-test-final.txt` in this folder.
- `bun run ./📜️script.ts policy` — ran read-only (not modified). Zero mentions of `ply` anywhere
  in the breach output (grep-verified); zero mentions of
  `POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`/`POLICY_FIXTURE_HONESTY`/
  `POLICY_LANGUAGE_REGISTRATION`/`POLICY_STDIO_JSON_TRANSFER_BAN` at all in the printed breach
  set — every breach shown belongs to unrelated `os-state-authority`/`budget` categories in other
  subsystems. Full output too large to inline; not copied into the ticket folder (ran read-only,
  policy allowlists are outside this session's ownership to edit per the ticket's own rules).
- JSON-transfer elimination: grepped `☁️ply`'s diff/mutations files for
  `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` inside `ArtifactPack`/
  `OpBinary`/`DiffCodec` impl blocks — zero hits (the only `serde_json` mentions left anywhere are
  doc-comment prose explaining what the OLD stale grammar placeholder used to describe, not real
  code).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs` — 5-role
  `register_pilot_languages`; new `demo_ply_snapshot()`; new `conformance_laws` test submodule (6
  laws); `header_text` bugfix (comments re-emission) and its call-site update.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`
  — rewritten, real dialect.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio`
  — rewritten, real dialect, honest magic+opaque-tail boundary.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `encode_pack_with` bugfix.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/📖️component.grammar.semio`
  — rewritten, real dialect.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`
  — rewritten, real binary diff frame.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — new `RealBinaryPrimitives`/`RealBinaryDiffFrame` regions; `DiffCodec::encode_diff`/
  `decode_diff` upgraded to real binary; `sweep_a`/`sweep_b`/new `demo_diff_cases()` promoted to
  module scope (`#[cfg(test)]`) so the engine's conformance tests can reuse them; `codec_tests`
  simplified to iterate `demo_diff_cases()`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  — rewritten, real dialect.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio`
  — rewritten, real binary op frame.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — new `RealBinaryOpFrame` region; `OpBinary::encode_op`/`decode_op` upgraded to real binary;
  new `demo_mutation_cases()` (`#[cfg(test)]`, module scope); `codec_tests` simplified to iterate it.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` —
  regenerated, genuine `print_dsl` output (was the fake string `"Hello, stdio.ply!"`).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` —
  **new**, genuine `encode_pack` bytes (did not exist before this wave).
- Ticket-folder scratch (kept, `.txt`): `p2-fg3-ply-scoped-test-final.txt`,
  `p2-fg3-ply-full-crate-test-final.txt`.

**No shared files touched**: `📦️glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema`/`store`
framework crates, `🧪️fixture-sweep/🦀️component.rs`'s `STDIO_CONFORMANCE_GRADUATED` list, and every
other artifact's own files were all left untouched (only read, to confirm real framework
behavior — `Prim`'s variant set, `walk_protocol`/`walk_fields`, `dsl::ByteWriter`/`ByteReader`
re-export path, `store::pack_rt::OP_BINARY_FORMAT` — never assumed).

## Deviations from a completely literal reading of the artifact brief

1. **Snapshot protocol is magic+opaque-tail, not a field-by-field `repeat`/`Count::Field`
   per-element/per-property walk**, despite the brief's own enthusiastic framing ("use M2's
   repeat/Count::Field for this, it's the textbook case... don't fall back to opaque unless you
   hit a genuine wall"). A genuine wall WAS hit and is documented in detail above and in the
   protocol file's own doc comment: the dialect has no "scan to this literal multi-byte string"
   primitive to locate `end_header\n`, and no "a value the grammar side parsed parameterizes a
   protocol field" mechanism — confirmed live by reading `Prim`'s real variant set and
   `walk_fields`, not assumed. This is not a local invention: the SAME ticket's own
   `📖️grammar-recipe.md` §6 and `p2-w0-recon-report.md` already name `ply` explicitly, alongside
   `pdf/1.7`, as a pre-approved M2 exclusion for exactly this reason, "already decided, don't
   reopen." `repeat`/`Count::Field`/`endian` remain fully real, working, exercised mechanisms in
   this dialect (see gif89a's/png's own protocol files) — they simply don't apply to a body whose
   very shape is chosen by ASCII text parsed at a different layer.
2. **Fixed two real, pre-existing Rust bugs** (`encode_pack_with`'s format-discarding, and
   `header_text`'s comment-dropping) that are outside the literal "grammar/protocol files +
   DiffCodec/OpBinary" checklist scope but squarely inside the `☁️ply/**` ownership boundary and
   directly load-bearing for the facets this wave is asked to model honestly (a "Pack facet
   covers both format variants" claim would have been false without fix #1; the snapshot
   grammar's own `comment-line` production would have been permanently dead code without fix #2).
   Both are additive, non-breaking (verified: no pre-existing test exercised either code path the
   old way), and documented in-line at the fix site.
3. **`example.ply`** (a stray, unreferenced asset under the same `📚️examples/🎬️demo/🖼️assets/`
   directory, containing the leftover copy-paste string `"Hello, stdio.txt!"`) was left untouched
   — confirmed by grep it is not `include_str!`/`include_bytes!`-referenced anywhere in the
   crate, so it is dead weight, not a fixture this wave's checklist requires regenerating (the
   checklist names `example.dsl.semio`/`example.pack.semio` specifically).

## Report path

`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg3-ply-report.md`
(this file).
