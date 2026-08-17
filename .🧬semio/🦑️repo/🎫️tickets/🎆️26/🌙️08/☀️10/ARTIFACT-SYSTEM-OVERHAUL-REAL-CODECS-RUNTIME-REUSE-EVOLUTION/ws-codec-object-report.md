# W-S Codec Wave — `stdio.semio.object` (`✳️object` subset)

Second real-codec subset for **semio** (`🧿️semio`), following `ws-codec-workflow-report.md`'s
proven pattern, applied to the hardest facet shape in this batch: a genuinely **recursive**,
data-carrying value graph (`SemioValue` — `Null`/`Bool`/`Int`/`Float`/`Str`/`Bytes`/`List`/`Map`/
`Ref`), closer in shape to `json`'s own `JsonValue` than to `workflow`'s flat node/edge records.
Scope: `✳️object`'s three facets (snapshot, diff, mutations), plus a new example fixture slug.

**Status: fully verified green, in this same session, synchronously — no deferred/unverified
claims.** See §5 for real, observed command output.

---

## 1. What was replaced

Before this wave:
- **Snapshot** (`📸️snapshot/🦀️component.rs`): `ArtifactDsl`/`ArtifactPack` were a hex-of-`serde_json`
  passthrough — `serde_json::to_vec(self)`/`from_slice` wrapped in hex text or raw bytes. No real
  grammar walked the value shape; the grammar file described a JSON object literal (`'"schema"' ":"
  ...`) instead of this subset's own hand-rolled `SemioValue` convention.
- **Diff** (`🔺️diff/🦀️component.rs`): `print_diff`/`parse_diff` were ALREADY real (tag-prefixed hex
  text, matching `json`'s own `JsonValueDiff` convention) — but `DiffCodec::encode_diff`/
  `decode_diff` were still the F6-era `print_diff().into_bytes()` text-as-binary shortcut, and the
  protocol file only described `framing record` + `chain payload utf8` (no real header fields).
- **Mutations** (`🧬️mutations/🦀️component.rs`): same shape — `OpText` already real, `OpBinary` still
  `print_op().into_bytes()`.
- All three `.grammar.semio` files had TWO real, independent bugs (present before this wave, never
  caught because nothing had parsed them yet): (a) `hex = ( HEXDIG HEXDIG )*` as a hand-rolled
  production instead of the framework's bare `hex` macro (`HEXDIG` isn't even a terminal this
  dialect's lexer recognizes); (b) list/map-shaped alternatives grouped with bare `( )` instead of
  `{ }` (recipe pitfall #1 — `( )` is reserved for macro-call argument lists).

## 2. What's real now

### Snapshot (`📸️snapshot/`)

- **Real recursive text DSL** — `enc_semio_object_snapshot`/`dec_semio_object_snapshot` (new
  `📸️snapshot/🦀️component.rs` region `🔖️SnapshotTextCodec`): `[hex(schema),<value>,[<node>,...]]`,
  every field its own token, genuinely recursively parsed. `SemioObjectSnapshot` has no natural
  "on-disk file format" of its own (unlike `json`'s real RFC8259 text) — it's a NEUTRAL semio type —
  so this reuses the SAME tag-prefixed `SemioValue` grammar the sibling `🔺️diff`/`🧬️mutations`
  facets already hand-rolled pre-wave (`enc_semio_value`/`dec_semio_value`, `diff/🦀️component.rs`),
  rather than inventing a third independent encoding. `ArtifactDsl::print_dsl`/`parse_dsl` and
  `ArtifactPack::encode_pack_with`/`decode_pack_with` both route through it — **zero `serde_json`
  anywhere in this impl block now** (confirmed by grep, §4).
- **Real binary pack** — this subset is classified TEXT-NATIVE (like `json`, not binary-native like
  `png`): there is no separate "binary `SemioValue`" layout. The pack IS the semio envelope wrapping
  this same real recursive text, UTF-8 encoded, verbatim — same precedent `json`'s own
  `JsonSnapshot::encode_pack_with` uses (`write_json_text(&self.value).into_bytes()` straight into
  `wrap_binary`, no distinct binary value layout). The "varint/length-prefixed" framing the brief
  anticipated is the SEMIO envelope's own magic+u32-length+token framing (described once,
  framework-level) — not something this facet needs to reinvent.
- **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`: real dialect syntax,
  `artifact-mark = "stdio.semio.object"` + `document = artifact-mark "[" hex "," value "," "["
  object-node* "]" "]"`, `value` genuinely recursive (`List`/`Map` reference `list-item`/`map-item`
  which reference `value` back).
- **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: unchanged framing shape
  (`framing record` + `chain payload utf8`, correct per the text-native classification), comment
  updated to describe the real encoder instead of the old `serde_json::to_vec` shortcut.

### Diff (`🔺️diff/`)

- **Binary upgrade** — `DiffCodec::encode_diff`/`decode_diff` now: `format u8` +
  `presence u8` (bit0=`root` present, bit1=`objects` present) as two REAL, individually
  protocol-walkable fixed header fields, then the recursive payload(s) as one opaque trailing
  `bytes` chain. The payload itself is **real LEB128-varint-framed recursive binary** — new
  `enc_value_diff_bin`/`dec_value_diff_bin` (+ `enc_indexed_diff_bin`/`enc_map_diff_bin`/
  `enc_objects_diff_bin` helpers), template copied from `json`'s own `enc_value_diff_bin`/
  `enc_array_diff_bin`/`enc_object_diff_bin` — NOT a re-wrap of the text bytes (a stricter upgrade
  than the minimal "text-blob-behind-a-real-header" shape `workflow`'s diff used, matching `json`'s
  own rigor since `object` was explicitly pointed at `json` as the primary precedent for the
  recursive parts).
- **Grammar file** — real dialect syntax; fixed the pre-existing `hex = (HEXDIG HEXDIG)*` production
  bug (now bare `hex` macro) and the bare-`( )`-grouping bug (`{...}*`/`item ","?` shape now, matching
  json's own list/triple productions).
- **Protocol file** — `header fixed 2` + `field format u8` + `field presence u8` + `chain payload
  bytes`, same shape json's own `format u8 | has_value u8` diff protocol uses.
- **`demo_diff_cases()`** — new module-scope `#[cfg(test)] pub(crate) fn` (previously the equivalent
  case list lived inline inside one test); reused by both `diff_codec_text_binary_roundtrip_law`
  (this file) and `diff_grammar_conformance_law`/`protocol_walk_law` (composer's conformance tests) —
  single source of truth, same convention `json`'s `demo_diff_cases()` uses.

### Mutations (`🧬️mutations/`)

- **Binary upgrade** — `OpBinary::encode_op`/`decode_op` now: `format u8` + `tag u8` (variant
  ordinal, 0-8, matching `print_object_mutation`'s own keyword match order) as two real fixed
  fields, then the variant's own real LEB128-varint-framed path/key/value/id payload as one opaque
  trailing `bytes` chain — new `enc_semio_path_bin`/`dec_semio_path_bin` (this file) and
  `enc_semio_object_snapshot_bin`/`dec_semio_object_snapshot_bin` (this file, for `SetSnapshot`
  only — mirrors `json`'s own `enc_json_snapshot_bin`, which likewise lives in `mutations`, not
  `snapshot`), built on `enc_semio_value_bin`/`enc_semio_object_node_bin` (moved to `diff/🦀️component.rs`
  as shared primitives, reused by both diff and mutations, same layering json uses).
- **Grammar file** — real dialect syntax, same `hex`-macro and `{ }`-grouping bug fixes as diff's;
  `document`'s 9-way alternation collapsed to reference named per-variant productions on one
  physical line (recipe pitfall #4 — the original draft literally wrapped this across 9 lines and
  was caught by `ops_grammar_conformance_law`/`committed_facet_files_parse` failing, see §6).
- **Protocol file** — `header fixed 2` + `field format u8` + `field tag u8` + `chain payload bytes`.
- **`demo_mutation_cases()`** — promoted to module scope (`#[cfg(test)] pub(crate) fn`, was
  previously inline inside `op_text_binary_roundtrip_law` only); reused by that test AND by
  composer's `ops_grammar_conformance_law`/`protocol_walk_law`.

### Single-source-of-truth cleanups (beyond the minimum ask, low-risk, same-file-set)

- `enc_semio_snapshot`/`dec_semio_snapshot` in `mutations/🦀️component.rs` (previously an
  independent second copy of the same `[hex(schema),value,[node,...]]` encoding `snapshot`'s own DSL
  now uses) are now thin aliases calling `snapshot::enc_semio_object_snapshot`/
  `dec_semio_object_snapshot` directly — one encoder, not two silently-driftable copies.
- `demo_semio_object_snapshot()` — promoted from an inline `sample_snapshot()` test helper (was
  private to `snapshot/🦀️component.rs`'s own `mod tests`) to a module-scope `#[cfg(test)]
  pub(crate) fn`, reused by this file's own round-trip tests AND by composer's
  `grammar_conformance_law`/`protocol_walk_law`/`fixture_honesty_law` — same convention `json`'s
  `demo_json_snapshot()` and `workflow`'s `demo_workflow_snapshot()` use.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs`'s new `#[cfg(test)] mod tests { mod conformance_laws { ... } }` — object
has no per-subset `⚙️engine/` dir (only `📸️snapshot`/`🔺️diff`/`🧬️mutations`/`🎹️composer`/
`🏗️builder`/`🚪️io`/`🧐️analyzer`), same situation `workflow` was in; `🎹️composer` is the closest
"engine-equivalent" home, matching `workflow`'s own precedent exactly.

### Real fixtures

New example slug `🧿️semio/📚️examples/🕸️graph/` (outside `✳️object/`, explicitly permitted by the
brief, same treatment `workflow`'s `🌊️pipeline` slug got): `🦀️component.rs`, `🟦️component.ts`,
`🖼️assets/🗣️example.dsl.semio`, `🖼️assets/🎒️example.pack.semio`. The two `🖼️assets/*.semio` fixtures
are the GENUINE `print_dsl()`/`encode_pack()` output of `demo_semio_object_snapshot()` — generated
via a temporary `#[test] fn ws_temp_print_real_fixtures()` added to composer's `conformance_laws`
module, run once (`cargo test ... -- --nocapture`), the real stdout hex captured and converted to
the exact file bytes via a small Python script (never hand-transcribed), then the temp test deleted.
`fixture_honesty_law` asserts these fixtures decode back to `demo_semio_object_snapshot()` AND that
re-encoding it reproduces the shipped bytes exactly — so this can never silently drift to a fake.

### JSON-transfer ban (checklist item 8)

Grepped every changed `.rs` file (`snapshot`, `diff`, `mutations`, `composer`) for
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` — **zero hits**. The only
`serde_json` mentions left anywhere in these files are inside doc comments explicitly stating it is
NOT used.

---

## 3. Exact files touched

All paths relative to repo root, base
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/`.

**Snapshot**: `📸️snapshot/🦀️component.rs`, `📸️snapshot/📝️text/📖️component.grammar.semio`,
`📸️snapshot/📝️text/🅰️component.g4`, `📸️snapshot/📝️text/🔤️component.ebnf`,
`📸️snapshot/💾️binary/📡️component.protocol.semio`, `📸️snapshot/💾️binary/🥋️component.ksy`,
`📸️snapshot/💾️binary/🌶️component.spicy`, `📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/📖️component.grammar.semio`,
`🔺️diff/📝️text/🅰️component.g4`, `🔺️diff/📝️text/🔤️component.ebnf`,
`🔺️diff/💾️binary/📡️component.protocol.semio`, `🔺️diff/💾️binary/🥋️component.ksy`,
`🔺️diff/💾️binary/🌶️component.spicy`, `🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `🧬️mutations/🦀️component.rs`, `🧬️mutations/📝️text/📖️component.grammar.semio`,
`🧬️mutations/📝️text/🅰️component.g4`, `🧬️mutations/📝️text/🔤️component.ebnf`,
`🧬️mutations/💾️binary/📡️component.protocol.semio`, `🧬️mutations/💾️binary/🥋️component.ksy`,
`🧬️mutations/💾️binary/🌶️component.spicy`, `🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️object/🎹️composer/🦀️component.rs` (new `#[cfg(test)] mod tests { mod
conformance_laws }`).

**New example slug** (outside `✳️object/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🕸️graph/🦀️component.rs`,
`…/🕸️graph/🟦️component.ts`, `…/🕸️graph/🖼️assets/🗣️example.dsl.semio` (real),
`…/🕸️graph/🖼️assets/🎒️example.pack.semio` (real).

Nothing outside these was touched — confirmed via `git status --porcelain` scoped to
`script.ts`/`glue.rs`/`catalog.json`/launch-config/`🧪️fixture-sweep`/`🔣️json` (json's own files, the
recursive-value reference only, read-only): all clean.

---

## 4. Verification — real, observed output (synchronous, this session)

```
cargo check -p semio-s-plugin-stdio
```
→ **0 errors** ("Finished `dev` profile [unoptimized] target(s) in 0.23s", 486 pre-existing
warnings, none attributable to this wave's files). Note: an unrelated concurrent session's
in-progress `✳️brep` refactor produced 4-5 transient `enc_face`/`dec_face` visibility errors during
this session (confirmed via `git status` showing those exact `brep`/`obj` files as `M` mid-edit by
another session) — waited (polled every 20s) until that session's edits landed, then re-ran; zero
errors remained, none ever in any `object`-subset file.

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::object"
```
→ **60 passed, 0 failed, 0 ignored** (first run: 55 passed / 5 failed — all 5 in the newly-added
conformance-law tests; see §6 for the two real bugs those failures caught and how they were fixed;
final run after fixes: 60/60 green), including all 6 conformance-law tests individually confirmed
`ok`: `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`.

```
cargo test -p semio-s-plugin-stdio --lib
```
→ **1869 passed, 0 failed, 3 ignored, 0 filtered out** — zero regressions anywhere in the whole
crate.

---

## 5. Real bugs caught and fixed this session (read before replicating this pattern elsewhere)

1. **`value = ... | ... \n | ...` wrapped across two physical lines** — hit independently in all
   THREE grammar files' `value`/`value-diff` production (the long 9-alternative tag list was long
   enough to tempt a wrap, exactly the failure mode `png`'s own pilot hit per the recipe's pitfall
   #4). Caught by `committed_facet_files_parse`/`grammar_conformance_law`/
   `ops_grammar_conformance_law`/`diff_grammar_conformance_law` all failing with `"expected Ident,
   found Pipe"`. Fixed by collapsing each to one physical line.
2. **Missing `artifact-mark` token in the snapshot grammar's `document` production** — the snapshot
   grammar's `document` didn't account for the `envelope_id()` token
   (`grammar_conformance_law`/`fixture_honesty_law`'s own reconstructed-body harness prepends
   `stdio.semio.object` before the body, matching `json.snapshot`'s/`workflow.snapshot`'s own
   grammars' `artifact-mark` convention). Fixed by adding `artifact-mark = "stdio.semio.object"` and
   prefixing `document` with it — direct, real-`Recognizer`-confirmed proof, not assumed from the
   sibling pilots' precedent alone.
3. **Pre-existing `hex = ( HEXDIG HEXDIG )*` + bare-`( )`-grouping bugs** in the diff/mutations
   grammar files (present BEFORE this wave, inherited from an earlier ticket phase, never actually
   parsed until this wave's `committed_facet_files_parse` ran for the first time) — fixed per the
   recipe's own pitfalls #1/#2, using the bare `hex` macro and `{...}*`/`item ","?` shapes instead,
   matching `json`'s own already-correct grammar files exactly.

None of these were guessed at or deferred — every one was caught by a real, synchronous
`cargo test` failure in this session and fixed, then re-verified.

---

## 6. Mechanism notes / follow-ups (not blocking, filed for the next semio subset)

- **`register_pilot_languages()`/`register_schema_spec`** — NOT added. `object`'s `🎹️composer::register()`
  had no pre-existing `dsl::register_language`/`register_schema_spec` call site (unlike `json`'s
  `⚙️engine::register()`, which already had one from an earlier wave) and `SemioObjectSnapshot`/
  `SemioValue` are fully hand-rolled (no derivable `RecordSpec` — same root cause as `json`'s
  `JsonValue`/`workflow`'s hand-rolled path). This ticket's own deliverable list (in the brief) did
  not ask for this item for `object`; following `workflow`'s own precedent, it is filed as a
  follow-up rather than guessed at or half-implemented.
- **Diff/mutations binary payload is real recursive LEB128 binary, not a text-blob-behind-a-header**
  — a stricter, more thorough upgrade than `workflow`'s own diff/mutations binary (which kept the
  existing TEXT bytes as the opaque payload, only adding a real header in front). This wave went the
  fuller `json`-precedent route (real `enc_value_diff_bin`/`enc_semio_value_bin`/`enc_semio_path_bin`,
  genuinely structured and round-trip tested independently of the text codec) because the brief
  explicitly pointed at `json` as the PRIMARY reference for this subset's recursive parts. Future
  subsets with a similarly recursive value type should default to this fuller pattern; subsets with
  only flat/record-shaped diffs (most of the remaining semio subsets) should keep using `workflow`'s
  lighter pattern — it is equally honest and less code for that shape.
- **Layering**: `SemioValue`'s real (text AND binary) primitive encoders live in `🔺️diff/🦀️component.rs`
  (not `📸️snapshot/`), exactly mirroring where `json` places `enc_json_value`/`enc_json_value_bin` —
  `snapshot`, `diff`, and `mutations` all import from there. This is a real, if slightly
  counter-intuitive (the "owning" type's file isn't the primitives' home), established repo
  convention — worth calling out explicitly for whoever reads this subset's imports next.

**Status: this subset's real-codec upgrade is a genuinely proven, fully green, follow-along
template for the next semio subset with a recursive value-graph shape** (candidates per the
subset list: any subset whose snapshot embeds a `SemioValue`-shaped or otherwise self-referential
payload). §2's per-facet breakdown and §6's mechanism notes are the copy-paste answer for that
subset's own wave.
