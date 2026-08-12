# W-S Codec Closer — `stdio.semio` (`✳️any` subset, the envelope union over all 13 domain subsets)

Closing wave for **semio** (`🧿️semio`): all 13 domain subsets (`brep`, `mesh`, `model`, `object`,
`document`, `cad`, `drawing`, `image`, `video`, `audio`, `animation`, `presentation`, `workflow`)
now have real codecs. This wave upgrades the last remaining subset — `✳️any`, the tagged-union
envelope wrapping the other 13 — from a hex-of-`serde_json` passthrough to a real codec that
**delegates** to each subset's own now-real codec, per the brief's explicit requirement (never
reinvent, never re-derive any of the 13 subsets' own grammars/binary layouts).

**Status: fully verified green, in this session, synchronously — no deferred/unverified claims.**
See §4 for real, observed command output.

---

## 1. What was replaced

Before this wave, `📸️snapshot/🦀️component.rs`'s `ArtifactDsl`/`ArtifactPack` impls for
`SemioSnapshot` were a hex-of-`serde_json` passthrough (`serde_json::to_vec`/`from_slice` of the
WHOLE `SemioSnapshot` struct, including its 13-arm `#[serde(tag="subset")]` enum). Same for
`🧬️mutations/🦀️component.rs`'s `OpText`/`OpBinary` (`serde_json::to_string`/`from_str` of the
whole 15-variant `SemioMutation` enum). `🔺️diff/🦀️component.rs`'s `print_diff`/`parse_diff` (text)
were ALREADY real (delegating to each wrapped subset's own `print_diff`/`parse_diff` — confirmed by
reading, per the brief's own note that these "may have been kept up to date automatically") — only
`Replace`'s payload (`enc_replace_snapshot`/`dec_replace_snapshot`) and the binary
`DiffCodec::encode_diff`/`decode_diff` were still on `serde_json`/text-as-binary shortcuts.

## 2. What's real now — genuine delegation, not reinvention

### Snapshot (`📸️snapshot/`)

- **Real delegating text DSL** (`📸️snapshot/🦀️component.rs`, new `🔖️SubsetDispatch`/
  `🔖️TextPrimitives` regions): body is exactly 2 header lines (`subset=<tag>`, `schema=<hex>`)
  followed by the WRAPPED subset's own real `<SubsetSnapshot as store::ArtifactDsl>::print_dsl()`
  output, with THAT subset's own preamble line stripped (`strip_inner_preamble`, via
  `store::semio_format::split_text_preamble`) — this envelope already carries its own `semio
  stdio.semio.dsl v1` preamble via `wrap_text`, so embedding a second one would double up.
  `parse_dsl` hands the un-prefixed remainder straight to the matching subset's own real
  `parse_dsl` — every subset's own `parse_dsl` already tolerates a missing preamble (falls back to
  treating the whole text as body), the exact same convention this envelope's own `parse_dsl`
  itself relies on one level up. Confirmed **zero** `serde_json` calls in the impl (§4).
- **Real delegating binary pack** (`🔖️BinaryPrimitives`): `format u8` + `tag u8` (real
  [`subset_ordinal`], 0-12) + varint-length-prefixed `schema` UTF-8 (`store::pack_rt`/
  `store::ByteReader`, same primitives every prior pilot uses), then the WRAPPED subset's own full,
  already-real `<SubsetSnapshot as store::ArtifactPack>::encode_pack()` bytes as one opaque
  trailing payload. That call already applies THAT subset's own `semio_format` envelope internally
  — a genuine, honest DOUBLE envelope (delegation, not a re-derivation of any subset's own binary
  layout).
- **Grammar file** (`📸️snapshot/📝️text/📖️component.grammar.semio`): real header —
  `artifact-mark = "stdio.semio"` + `subset-tag = "brep" | "mesh" | ... | "workflow"` (13
  alternatives, one physical line) + `subset-line = "subset" "=" subset-tag` — then the OPAQUE
  remainder as `document = artifact-mark subset-line REST` (the raw-span terminal,
  `📖️grammar-recipe.md` §1.3(b), confirmed positionable mid-sequence by the framework's own test
  `doc = "BODY" REST`). The wrapped subset's own real, already-conformance-tested grammar is the
  sole authority past this point — re-describing all 13 full sub-grammars inline would be pure
  duplication.
- **Protocol file** (`📸️snapshot/💾️binary/📡️component.protocol.semio`): `header fixed 2 {format u8,
  tag u8}` + bare `segment schema_len varint` / `segment schema_bytes Array(u8, Field(schema_len))`
  (the proven bare-segment form per the recipe's own workflow-pilot addendum) + `chain payload
  bytes` (the wrapped subset's own opaque `encode_pack()` output).
- `🅰️component.g4`/`🔤️component.ebnf`/`🥋️component.ksy`/`🌶️component.spicy`/`🔠️component.abnf` —
  descriptive mirrors, same production/field names.

### Diff (`🔺️diff/`)

- **Text**: `print_diff`/`parse_diff` were already real for the 13 same-kind arms; this wave
  upgraded `Replace`'s payload from hex(`serde_json`) to hex(`SemioSnapshot::print_dsl`) — real
  delegation to this envelope's own now-real `ArtifactDsl`, applied one level in.
- **Real delegating binary** (`impl protocol::DiffCodec`, replacing the old
  `print_diff().into_bytes()` text-as-binary shortcut): `format u8` + `tag u8` (real [`diff_tag`],
  0=NoChange, 1-13=subset kind, 14=Replace) as two genuine, individually protocol-walkable fixed
  header fields, then ONE opaque trailing payload: for the 13 same-kind variants, the wrapped
  subset's OWN real `DiffCodec::encode_diff()`/`decode_diff()` bytes (every one of the 13 subsets
  already has a real binary `DiffCodec` from its own prior wave — confirmed by grep before writing
  any code, see §5); for `Replace`, the wrapped snapshot's own real
  `ArtifactPack::encode_pack()`/`decode_pack()` bytes (📸️snapshot's real binary delegation, reused
  one level deeper); `NoChange` carries no payload.
- Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`): `tag = "replace" | "brep" | ... |
  "workflow"` + `tagged-diff = tag ":" REST` + `diff = "noChange" | tagged-diff` — same tag +
  opaque-REST treatment as the snapshot facet, for the same "don't duplicate 13 already-real
  grammars" reason.
- Protocol: `header fixed 2 {format u8, tag u8}` + `chain payload bytes`.
- `demo_diff_cases()` promoted to module scope (`#[cfg(test)] pub(crate) fn`) — `NoChange`, all 13
  same-kind (empty-but-genuinely-tagged) nested diffs, and one `Replace` — reused by this file's
  own test and by `🎹️composer`'s conformance-law tests.

### Mutations (`🧬️mutations/`)

The biggest lift: `OpText`/`OpBinary` were a full whole-enum `serde_json` passthrough, not a
partial shortcut. New real text form: `noMutation` | `<tag>:<payload>` where `tag` is one of
`setSnapshot`/13 subset names, and `payload` for the 13 wrapped variants is exactly that subset's
OWN real `OpText::print_op()`/`parse_op()` output (every one of the 13 subsets already has a real
`OpText`/`OpBinary` — confirmed by grep before writing any code, see §5); for `setSnapshot`,
hex(`SemioSnapshot::print_dsl`) — same real delegation pattern as diff's `Replace`. New real binary
form (`impl protocol::OpBinary`): `format u8` + `tag u8` (real [`mutation_tag`], 0=NoMutation,
1=SetSnapshot, 2-14=subset kind) + one opaque trailing payload — the wrapped subset's own real
`OpBinary::encode_op()`/`decode_op()` bytes, or (`SetSnapshot`) the wrapped snapshot's own real
`ArtifactPack::encode_pack()`/`decode_pack()` bytes. Grammar/protocol mirror the diff facet's shape
exactly (tag alternation + opaque REST / opaque `chain payload bytes`).
`demo_mutation_cases()` (module scope) covers all 15 top-level tags (`NoMutation`, `SetSnapshot`,
and each of the 13 wrapped-kind `NoMutation`-equivalent variants) for full dispatch-table grammar
coverage.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests`, in a new nested `mod
conformance_laws` — `any` has no per-subset `⚙️engine/` dir (only `📸️snapshot`/`🔺️diff`/
`🧬️mutations`/`🎹️composer`/`🏗️builder`/`🚪️io`/`🧐️analyzer`), same situation every prior semio pilot
was in; `🎹️composer` is the closest "engine-equivalent" home, matching the established precedent.

### Real fixtures

New example slug `🧿️semio/📚️examples/🌐️envelope/` (outside `✳️any/`, explicitly permitted by the
brief, same treatment `workflow`'s `🌊️pipeline`/`object`'s `🕸️graph` slugs got): `🦀️component.rs`,
`🟦️component.ts`, `🖼️assets/🗣️example.dsl.semio`, `🖼️assets/🎒️example.pack.semio`. The demo snapshot
(`snapshot::demo_semio_snapshot()`) wraps `workflow`'s own real demo snapshot (2 nodes, 1 edge,
incl. a negative coordinate) — chosen as the representative subset per the brief's own suggestion
("e.g. mesh or workflow") since it's a well-understood, already-nontrivial nested payload. The two
`🖼️assets/*.semio` fixtures are the GENUINE `print_dsl()`/`encode_pack()` output of
`demo_semio_snapshot()` — generated via a temporary `#[test] fn ws_temp_print_real_fixtures()`
added to composer's `conformance_laws` module, run once with `--nocapture`, the real stdout
hex/text captured and converted to the exact file bytes via a small Python script (never
hand-transcribed), then the temp test deleted. `fixture_honesty_law` asserts these fixtures decode
back to `demo_semio_snapshot()` AND that re-encoding it reproduces the shipped bytes exactly.

### JSON-transfer ban (deliverable 7)

```
grep -n "serde_json::to_vec\|serde_json::from_slice\|serde_json::to_string\|serde_json::from_str\|serde_json::Value" \
  📸️snapshot/🦀️component.rs 🔺️diff/🦀️component.rs 🧬️mutations/🦀️component.rs
```
→ **zero hits** in all three files.

---

## 3. Exact files touched

All paths relative to
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/`.

**Snapshot**: `📸️snapshot/🦀️component.rs`, `📸️snapshot/📝️text/📖️component.grammar.semio`,
`📸️snapshot/📝️text/🦀️component.rs`, `📸️snapshot/📝️text/🅰️component.g4`,
`📸️snapshot/📝️text/🔤️component.ebnf`, `📸️snapshot/💾️binary/📡️component.protocol.semio`,
`📸️snapshot/💾️binary/🦀️component.rs`, `📸️snapshot/💾️binary/🥋️component.ksy`,
`📸️snapshot/💾️binary/🌶️component.spicy`, `📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/📖️component.grammar.semio`,
`🔺️diff/📝️text/🦀️component.rs`, `🔺️diff/📝️text/🅰️component.g4`, `🔺️diff/📝️text/🔤️component.ebnf`,
`🔺️diff/💾️binary/📡️component.protocol.semio`, `🔺️diff/💾️binary/🦀️component.rs`,
`🔺️diff/💾️binary/🥋️component.ksy`, `🔺️diff/💾️binary/🌶️component.spicy`,
`🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `🧬️mutations/🦀️component.rs`, `🧬️mutations/📝️text/📖️component.grammar.semio`,
`🧬️mutations/📝️text/🦀️component.rs`, `🧬️mutations/📝️text/🅰️component.g4`,
`🧬️mutations/📝️text/🔤️component.ebnf`, `🧬️mutations/💾️binary/📡️component.protocol.semio`,
`🧬️mutations/💾️binary/🦀️component.rs`, `🧬️mutations/💾️binary/🥋️component.ksy`,
`🧬️mutations/💾️binary/🌶️component.spicy`, `🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️any/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️any/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🌐️envelope/🦀️component.rs`,
`…/🌐️envelope/🟦️component.ts`, `…/🌐️envelope/🖼️assets/🗣️example.dsl.semio` (real),
`…/🌐️envelope/🖼️assets/🎒️example.pack.semio` (real).

Nothing outside these was touched — `script.ts`, `🧪️fixture-sweep/🦀️component.rs`, `📦️glue.rs`,
`catalog.json`, `launch.json`, the shared `⚙️engine/🧮️geometry` module, and every one of the 13
domain subsets' own files were left untouched (only ever CALLED via their existing public
`ArtifactDsl`/`ArtifactPack`/`DiffCodec`/`OpText`/`OpBinary` trait impls, never edited).

---

## 4. Verification — real, observed output (synchronous, this session)

```
cargo check -p semio-s-plugin-stdio
```
→ **0 errors** ("Finished `dev` profile [unoptimized] target(s) in 0.24-16.96s" across repeated
runs, 493 pre-existing warnings, confirmed none attributable to this wave's files by checking every
warning whose FIRST location line falls under `✳️any/` — none found).

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::any"
```
→ **27 passed, 0 failed, 0 ignored** (first run: 26 passed / 1 failed — `fixture_honesty_law`,
against the placeholder fixture text; fixed by generating real fixtures per §2's "Real fixtures"
step; final run: 27/27 green), including all 6 conformance-law tests individually confirmed `ok`:
`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`.

```
cargo test -p semio-s-plugin-stdio --lib
```
→ **1930 passed, 0 failed, 3 ignored, 0 filtered out** — zero regressions anywhere in the whole
crate (this session's baseline, per the brief, was 1922 passed/0 failed; the `any` wave net-added 8
tests — 27 new/updated `any`-scoped tests replacing 19 pre-existing ones — with zero failures
introduced anywhere else).

---

## 5. Real work verifying "genuine reuse" before writing any dispatch code

Before writing the delegation dispatch tables, grepped every one of the 13 domain subsets' own
`🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` for `fn encode_diff`/`fn decode_diff` and
`fn encode_op`/`fn decode_op` respectively — confirmed **all 13** already have real, non-scaffold
binary codecs (brep, mesh, model, object, document, cad, drawing, image, video, audio, animation,
presentation, workflow), consistent with the other 12 waves' own closing reports. This meant the
`any` envelope's own binary upgrade could be a pure, honest delegation (opaque-tail-to-the-real-
per-subset-codec) rather than needing to fall back to a text-bytes-behind-a-header shortcut for any
subset.

---

## 6. Mechanism notes (not blocking, filed for reference)

- **Ordinal tables**: `subset_ordinal` (snapshot, 0-12), `diff_tag` (diff, 0=NoChange/1-13=subset/
  14=Replace), `mutation_tag` (mutations, 0=NoMutation/1=SetSnapshot/2-14=subset) are three
  independent, explicitly-matched (no arithmetic-offset cleverness) functions — each facet's own
  wire tag space differs in shape (snapshot has no "no value" case; diff/mutations each have 1-2
  reserved non-subset tags), so a single shared numbering scheme across all three would need
  awkward offset math for no real benefit. All three preserve the enum's own declaration order for
  the 13 subset arms specifically, so they read as one consistent convention despite being separate
  functions.
- **Snapshot pack is a genuine double envelope**: `<SemioSnapshot as ArtifactPack>::encode_pack()`
  wraps a `format+tag+schema+payload` frame in ITS OWN `semio_format` envelope (magic + "stdio.
  semio.pack v1" token), and `payload` is itself the wrapped subset's full `encode_pack()` output —
  ANOTHER complete `semio_format` envelope (magic + e.g. "stdio.semio.workflow.pack v1" token)
  nested inside. Confirmed real and round-trip-correct by `pack_and_dsl_round_trip_the_demo_snapshot`
  and `all_thirteen_subset_tags_round_trip_text_and_binary` — a deliberate choice: it is the
  MOST honest form of delegation (literally calling the subset's own public codec end-to-end,
  bytes-out to bytes-in, no partial reach-in), at the cost of ~30 bytes of nested-envelope overhead
  per snapshot. A future optimization could strip the inner envelope and re-derive it from the
  outer `tag` byte on decode, but that would mean reaching into subset-internal wire details this
  ticket's `✳️any/`-only edit scope should not assume stay stable.
- **`register_pilot_languages()`/`register_schema_spec`** — not added, consistent with every prior
  semio pilot's own precedent (`SemioSnapshot`/`SemioDiff`/`SemioMutation` are fully hand-rolled, no
  derivable `RecordSpec`); filed as a follow-up rather than guessed at, per the recipe's own
  guidance.

**Status: `stdio.semio`'s envelope union (`✳️any`) now has a genuinely real, fully delegating,
fully green codec across all 3 facets — closing the loop on
ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's semio real-codec program (all 14
subsets — 13 domain + the envelope union — now real).**
