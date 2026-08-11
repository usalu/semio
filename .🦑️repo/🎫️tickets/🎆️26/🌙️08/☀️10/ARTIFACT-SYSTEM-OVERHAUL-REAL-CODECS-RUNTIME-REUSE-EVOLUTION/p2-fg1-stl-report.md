# FG1 — 🟪️stl (standard ascii) — Grammar & Protocol Overhaul Report

Ticket: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`. Artifact: `🟪️stl`
standard `ascii`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl/🏅️standards/🔖️ascii/`.

## 1. Starting point

Per W0's own recon row (`p2-w0-recon-report.md` line 83), stl/ascii's real keyword-line grammar
(`solid <name>` / `facet normal x y z` / `outer loop` / `vertex x y z`×3 / `endloop` / `endfacet` /
`endsolid <name>`) is "mostly sufficient" for the dialect — the only gap is the same one obj's `o`/
`g` has: `solid_name`/`endsolid`'s name token is genuine "rest of line, arbitrary text," solved by
M1's `LINE` raw-span terminal (in fact `📖️grammar/🦀️component.rs`'s own `RawSpanEnd` doc comment
names stl's `solid <name>`/`endsolid <name>` as one of the two worked citations for that exact
primitive). F2 (schema overhaul) and F6/F6b (OpText/OpBinary + DiffCodec) had already landed real,
hand-rolled Rust codecs for all three facets before this wave started — F6's report
(`f6-stl-report.md`) documents that `StlDiff`/`StlMutation` are **hand-rolled, not derive-based**,
because a real, reproduced `dsl`-grammar bug (nested `Shape::Tuple` — `vertices: [[f64;3];3]`
prints/parses flat with no depth marker) blocks the derive path. This wave's job was purely the
text-dialect/protocol-dialect *description* layer: 6 `.grammar.semio`/`.protocol.semio` files were
still on the pre-M1 ABNF/JSON-placeholder shape (unparseable by the real `parse_grammar`/
`parse_protocol`), 5-role `LanguageSpec` registration only covered the Document role, and the
`example.dsl.semio`/`example.pack.semio` fixtures were literal `"Hello, stdio.stl!"` stubs.

## 2. Grammar files (3 rewritten, real M1 dialect syntax)

- **Snapshot** (`📸️snapshot/📝️text/📖️component.grammar.semio`): `document = artifact-mark solid`,
  `solid = "solid" LINE facet* "endsolid" LINE`, with `facet`/`vertex`/`vec3` modeling
  `decode_stl_ascii`/`encode_stl_ascii` (⚙️engine/component.rs:20-89) exactly. `solid`/`endsolid`'s
  name uses the `LINE` raw-span terminal per W0's own citation. `number = INT | FLOAT` (json's own
  idiom) because Rust's `f64` `Display` never emits a trailing `.0` for whole-numbered coordinates.
- **Mutations** (`🧬️mutations/📝️text/📖️component.grammar.semio`): traced verbatim from F6's real
  `print_stl_op`/`parse_stl_op` (`🧬️mutations/🦀️component.rs`) — 7 keyword-line alternatives
  (`no-mutation`, `set-snapshot snapshot=...`, `set-solid-name name=<hex>`, `insert-triangle
  index=<INT> triangle=...`, `remove-triangle index=<INT>`, `set-triangle-normal ...`,
  `set-triangle-vertices ...`), the mandatory `hex` macro for `name`/`solid_name`/`schema` (never a
  hand-rolled `{INT|IDENT}*`), and the same `[...]`-bracketed `vec3-value`/`vertices-value`/
  `triangle-value`/`snapshot-value` shape F6's `enc_vec3`/`enc_vertices`/`enc_triangle`/
  `enc_snapshot` actually emit (every array level individually bracketed — the exact mechanism that
  sidesteps the `dsl`-derive nested-tuple bug).
- **Diff** (`🔺️diff/📝️text/📖️component.grammar.semio`): traced verbatim from F6's real
  `print_stl_diff`/`parse_stl_diff` (`🔺️diff/🦀️component.rs`) — sparse `solid-name-field?
  triangles-field?`, `triangles` as an index-keyed (INT-keyed) collection-triple
  (`"triangles" "{" "[" removed-list? "]" ";" "[" modified-list? "]" ";" "[" added-list? "]" "}"`),
  same shape as png's own `text-chunks-clause` (recipe §1.4). The one genuinely new shape beyond
  the recipe's own worked examples: `triangle-diff-value`'s sparse `N:<vec3>`/`V:<vertices>` tag
  list (F6's own per-field-tag sparse patch encoding for `StlTriangleDiff`).

**Real pitfall hit and fixed** (recipe §3 pitfall #4 — "keep every production on ONE physical
line"): the mutations grammar's `document` alternation was originally wrapped across 2 source
lines for readability; `parse_grammar`'s `parse_sequence` stops at the first `Newline` token, so
this silently truncated and the continuation line mis-parsed as an invalid new production
(`"expected Ident, found Pipe"`). Caught by `committed_facet_files_parse`/`ops_grammar_
conformance_law` on the first real test run, fixed by collapsing to one physical line — exactly
the same class of bug csv/png hit in the recipe's own documented history.

## 3. Protocol files (3 rewritten)

stl is **text-native** for the pack/op/diff wire (per W0's own classification and confirmed by
direct read of `📸️snapshot/🦀️component.rs`'s `ArtifactPack` impl): `encode_pack_with` wraps
`encode_stl_ascii(self)` verbatim UTF-8 ASCII-STL text inside the SEMIO binary envelope; F6's
`OpBinary::encode_op`/`DiffCodec::encode_diff` are `print_stl_op(self).into_bytes()`/
`print_stl_diff(self).into_bytes()` **verbatim, with no envelope, no header at all** (the same
"binary = text bytes verbatim" simplification `WriterDiff`/gif89a/svg's hand-rolled `DiffCodec`s
use, per F6's own doc comments). All three protocol files are therefore the same minimal, honest
shape: `framing record` + `chain payload utf8` — the snapshot one models the bytes as starting
right after the envelope's token (per M3's own "don't re-describe the envelope" guidance, matching
json's own worked §2.1 example exactly); the mutations/diff ones need no envelope-unwrap at all
since there is none. Replaced the old placeholders, which described the real *separate* binary-STL
byte layout (`decode_stl_binary`/`encode_stl_binary`) — a genuinely different codec this artifact
exposes only for cross-artifact export to `stdio.binary`
(`🚪️io/📤️export/…/💾️binary/🔖️raw/✳️any/🦀️component.rs`), never reachable through `ArtifactPack`
at all, so that content was honestly wrong for what a *pack-facet* protocol file must describe (I
did not touch that separate real binary-STL codec or its still-implicit description — out of this
wave's scope, that codec has no `.protocol.semio` of its own to begin with).

`diffcodec_binary_upgraded`/`opbinary_binary_upgraded`: **both false** — F6 already made
`OpBinary`/`DiffCodec` real (not a JSON-transfer shortcut); this wave only rewrote the *description*
files to honestly match that already-real Rust code, no Rust-side wire-format change was made or
needed.

## 4. Real fixtures

`demo_stl_snapshot()` added to `⚙️engine`'s `DocumentHelpers` region: `solid_name: "demo"`, two
triangles with distinct normals (`[0,0,1]`/`[0,0,-1]`), deliberately avoiding the degenerate empty-
`solid_name` case (see §6). Generated via a temporary `#[ignore]`d test (`zzz_generate_fg1_fixtures`,
deleted after one run, per the checklist's own instructions) that called the real
`store::ArtifactDsl::print_dsl`/`store::ArtifactPack::encode_pack` directly:

- `🗣️example.dsl.semio`: `semio stdio.stl.dsl v1\nsolid demo\n  facet normal 0 0 1\n...endsolid demo\n`
  (281 chars).
- `🎒️example.pack.semio`: the real SEMIO envelope (`8953454D0D0A1A0A` magic, LE u32 token length
  `0x11`=17, token `"stdio.stl.pack v1"`) wrapping the identical ASCII-STL body, 287 bytes.

Both asserted byte-for-byte against `demo_stl_snapshot()` by `fixture_honesty_law` (passing).

## 5. Conformance-law tests (all 6, in `⚙️engine`'s existing test region)

Added a `conformance_laws` submodule inside the existing `#[cfg(test)] mod tests` (extending the
file, not creating a new one, per CLAUDE.md): `committed_facet_files_parse`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law`,
`protocol_walk_law`, `fixture_honesty_law` — copied structurally from `binary`'s own pilot
(`💾️binary/…/⚙️engine/🦀️component.rs`), the closest precedent (also hand-rolled `OpBinary`/
`DiffCodec` for its mutations facet). Added `pub(crate) fn demo_mutation_cases()` (7 cases, one per
`StlMutation` variant) to `🧬️mutations/🦀️component.rs` and `pub(crate) fn demo_diff_cases()` (3
cases: empty, a full removed+modified+added triple, a `V`-tag-only sparse patch) to
`🔺️diff/🦀️component.rs`, both reused by the new conformance tests AND refactored into the
pre-existing `op_text_binary_roundtrip_law`/`diff_codec_text_binary_roundtrip_law` tests (removing
duplicated literal case lists — same DRY reuse pattern `binary`'s own precedent establishes).

## 6. `register_pilot_languages()` — 5-role `LanguageSpec` registration

Was Document-only before this wave. Added `stdio.stl.op` (Ops), `stdio.stl.diff` (Diff, `protocol:
None` per the exemplar's own shape), `stdio.stl.pack` (Pack), `stdio.stl.spr` (Spr) — all
`dsl::passthrough_hooks`, matching json/binary's own exemplar exactly. 5 roles total.

`register_schema_spec`: **not called**, and documented inline as intentional (not an oversight) —
`StlSnapshot`/`StlDiff`/`StlMutation` carry zero real `dsl::DslRecord`/`DslDiff` derives (F6's
hand-roll, see §1), so no `fn() -> RecordSpec` genuinely exists to register. Filed as a
`mechanism_gaps` entry (`register-schema-spec-needs-recordspec`) — a pre-existing, already-catalogued
gap from the recipe's own table (§5), not new to this wave.

## 7. Mechanism gaps

1. **`register-schema-spec-needs-recordspec`** (pre-existing, recipe §5) — hand-rolled types have no
   `RecordSpec`; `register_schema_spec` correctly skipped. Non-blocking.
2. **New, narrow observation** (not in the recipe's table, filed here for future readers): M1's
   `LINE` raw-span terminal's `match_raw_span` (`📖️grammar/🦀️component.rs`) starts its span at
   `tokens[pos]`'s byte offset — when the captured name is EMPTY (e.g. `solid \nendsolid \n`, real
   output of `encode_stl_ascii(&StlSnapshot::default())`), the next real token is on a LATER
   physical line (whitespace/newlines are lexer trivia), so `LINE` would swallow that later line's
   content as if it were the name. This is a real, narrow edge of the shared `match_raw_span`
   primitive itself (framework-level, out of this wave's ownership boundary), not a bug in this
   artifact's grammar. Worked around by design: `demo_stl_snapshot()` deliberately uses a
   non-empty `solid_name`, matching every other pilot's own "model realistically, avoid the
   degenerate corner your grammar's primitive doesn't cleanly cover" convention. Non-blocking — the
   REAL `decode_stl_ascii`/`encode_stl_ascii` Rust codec (used by `ArtifactDsl`/`ArtifactPack`, not
   the `Recognizer`) already round-trips the empty-name case correctly and is covered by the
   pre-existing `ascii_solid_name_round_trips_including_empty` test.

## 8. JSON-transfer elimination check

`grep -rn "serde_json::" ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🟪️stl` → zero hits. Already clean (F6's
hand-roll never used `serde_json`).

## 9. Verification

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::stl::"
  → 34 passed; 0 failed; 0 ignored (up from 21 before F6, 23 after F6, now +11 from this wave:
    5-role registration is not itself tested but its consts are exercised by the new law tests;
    the 6 new conformance-law tests plus demo_mutation_cases/demo_diff_cases reuse account for
    the net delta over the F6 baseline).

cargo test -p semio-s-plugin-stdio --lib   (whole crate, no filter)
  → 1703 passed, 13 failed, 3 ignored. All 13 failures independently confirmed to belong to OTHER,
    concurrently-active FG1-wave sessions on sibling standards — dxf (2: fixture_honesty_law +
    inverse_law), ifc (1: fixture_honesty_law), md (4: all 4 conformance laws), obj (3: 1 law +
    2 debug-dump tests), xml (3: 3 conformance laws) — every one confirmed via the failure text
    itself (missing/malformed fixtures, grammar bugs in THEIR OWN files, none touching anything
    under 🟪️stl/). Zero stl failures anywhere in the whole-crate run. Per this ticket's own
    "concurrent-churn" note (poll rather than chase), classified and not chased — these are each
    other FG1 agents' own in-flight work landing at different points in time, confirmed via
    `git status` showing those exact files as concurrently modified outside this session.
```

Real compile churn during this session (repo-wide, not caused by this session): `obj` (a syntax
error from a mid-edit concurrent session — resolved itself), `xml`/`md` (unresolved-import errors
from concurrent renames of hand-rolled binary-codec helper functions — resolved themselves),
`step`/`ifc` (unrelated missing-function errors from other concurrent FG1 agents — resolved
themselves). Each was independently re-confirmed via `git status`/error-text classification before
retrying, per this ticket's repo rules — none required or received any fix from this session.

## Files touched

- `📸️snapshot/📝️text/📖️component.grammar.semio` — rewritten, real M1 dialect.
- `📸️snapshot/💾️binary/📡️component.protocol.semio` — rewritten, real M2 dialect (pack-container
  shape).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten, real M1 dialect.
- `🧬️mutations/💾️binary/📡️component.protocol.semio` — rewritten, real M2 dialect.
- `🔺️diff/📝️text/📖️component.grammar.semio` — rewritten, real M1 dialect.
- `🔺️diff/💾️binary/📡️component.protocol.semio` — rewritten, real M2 dialect.
- `🧬️mutations/🦀️component.rs` — added `pub(crate) fn demo_mutation_cases()`, refactored
  `op_text_binary_roundtrip_law`/`diff_codec_text_binary_roundtrip_law` to reuse
  `demo_mutation_cases()`/`diff::demo_diff_cases()`.
- `🔺️diff/🦀️component.rs` — added `pub(crate) fn demo_diff_cases()`.
- `⚙️engine/🦀️component.rs` — added `demo_stl_snapshot()`, expanded `register_pilot_languages()`
  to 5 roles, added a documented note on why `register_schema_specs()` doesn't exist for this
  artifact, added the `conformance_laws` test submodule (6 tests).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated, real `print_dsl` output.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — regenerated, real `encode_pack` bytes
  (binary file, new).
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/p2-fg1-stl-report.md`.

No shared/off-limits files touched: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema`
framework crates, `🧪️fixture-sweep`, `🏪️store` were all read-only this session.
`StlSnapshot`/`StlDiff`/`StlMutation`'s Rust SHAPE and every `Encode`/`decode` function BODY are
byte-for-byte unchanged from before this session (verified: only `.grammar.semio`/`.protocol.semio`
content, 2 new `pub(crate)` demo-case functions, 1 new `demo_stl_snapshot()` helper, the
`register_pilot_languages()` body, and the new test module were touched).

## Deviations from the brief

None substantive. The brief's own framing ("one of the simplest remaining pilots... don't
overbuild it") was followed: no new mechanism-gap workaround needed beyond `LINE` (already
anticipated by the brief and M1's own worked citation); `hex`/collection-triple/`INT|FLOAT`
patterns applied exactly per the recipe, no local inventions beyond the diff grammar's
`triangle-diff-value` sparse tag list (which itself is a direct, mechanical transcription of F6's
real `enc_triangle_diff`, not a new pattern).
