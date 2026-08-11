# FG2 — 🖊️dwg (ac1018 + ac1024) — grammar/protocol real-codec upgrade report

Ticket: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`, wave FG2.
Scope: both DWG standards, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/` and
`.../🔖️ac1024/`, one combined report per the brief's "double-artifact slot" instruction.

## 1. Starting state (read before assuming)

The brief's own citation (`p2-w0-recon-report.md` §1b) described ac1018 as having "no real
byte-level codec" and ac1024 as unassessed-against-the-dialect due to its decrypt/decompress
depth. **That recon predates this repo's current state.** By the time this FG2 session started,
an earlier wave (self-documented in-repo as "F5"/"F6") had already:

- Given ac1018 a real, honest byte-level codec: `dwg_version_sentinel` (generic `AC` + 4-digit
  check), `parse_version_header_fields` (`maintenance_version`/`codepage` at the real LibreDWG
  `header.spec` offsets 0x12/0x13), and a best-effort section-name substring/offset-table scan.
- Given ac1024 the full real R2004+ D1 (file-header decrypt, section/page-directory walk) + D2
  (bespoke LZ77-variant decompression) pipeline, validated end-to-end against the real ~145KB
  `architectural.dwg` fixture, with its own committed unit + real-fixture test suite already
  passing (`⚙️engine/🦀️component.rs`, 38 baseline tests, all green before this session touched
  anything).
- Given both standards' `DwgSnapshot`/`DwgDiff`/`DwgMutation` real `#[derive(dsl::DslRecord)]`/
  `dsl::DslDiff`/`dsl::DslOps)]` derives — meaning **both standards' `DiffCodec` (via the full
  `.spk` document container) and `OpBinary` (via `dsl::variants_binary`) were ALREADY REAL binary
  frames before this session started** — the exact "already real, upgrading is a no-op" case the
  recipe's checklist calls out for zip/binary, confirmed here by direct read of both standards'
  `🔺️diff/🦀️component.rs`/`🧬️mutations/🦀️component.rs` (module docs explicitly document the F6
  derive-eligibility decision) and independently re-verified by inspecting the generated
  `encode_diff`/`encode_op` code paths.

What was genuinely still a placeholder, for **both** standards, in **all 6 facets**: every
`.grammar.semio`/`.protocol.semio` file was the old pre-Phase-2 ABNF-shaped stub (`dialect
grammar stdio.dwg.snapshot` / `%x00` / `*OCTET` — 2 tokens on one header line, rejected outright
by the real dialect parser). This FG2 wave's actual job was therefore 100% "write the real
grammar/protocol files + registration + fixtures + conformance tests", 0% "upgrade an already-real
binary frame" — despite the brief's FG1-lesson warning, there was no F6 text-as-binary shortcut to
find on `DiffCodec`/`OpBinary` for either standard; both were already correct.

## 2. Real byte-level layout modeled (both standards, identical shape)

Both standards share the same plain, unencrypted AC1015+ preamble (LibreDWG `header.spec`), so
both snapshot protocol files model the identical real fields, verified against `⚙️engine`'s own
doc comments and independently re-verified with `xxd` against the real 148,638-byte
`architectural.dwg` fixture (offset 0x12 == `0x02`, offset 0x13 == `0x1E`/30 — exact match):

- `field magic fixed 6` — the `AC10xx` version sentinel (GENERIC across every AC10xx version;
  `encode_dwg`'s own test fixture uses "AC1032" and the codec accepts it — not hardcoded to the
  standard's own literal digits).
- `field reserved fixed 12` — real, positionally-fixed DWG header bytes (the redundant
  `zero_one_or_three` byte, a preview-thumbnail file address, per LibreDWG's own field order) that
  this artifact's own Rust codec never reads/interprets — modeled as one opaque span rather than
  fabricated field-by-field detail the codec doesn't actually have.
- `field maintenance_version u8` at 0x12, `field codepage u16` (LE) at 0x13-0x14 — both genuinely,
  individually read by `parse_version_header_fields`.
- `chain remainder bytes` — everything from 0x15 onward. For ac1018 this is the standard's own
  best-effort substring/offset-table section-name scan (never positionally decoded further, per
  Decision #5's documented frozen scope). For ac1024 this is the ENTIRE decrypt+decompress+
  two-level-indirection D1/D2 pipeline — the pre-approved M2 scope boundary (§6 of
  `📖️grammar-recipe.md`, restated verbatim in this FG2 brief): "an imperative transform pipeline,
  not a declarative layout description... would mean a different kind of system." Not attempted;
  not a shortfall.

## 3. New mechanism gap found and worked around: `protocol-framing-magic-fixed-8-bytes`

First hit (and already correctly diagnosed/fixed) by `stdio.gif`'s own 87a/89a sibling standards,
independently re-confirmed live here: `Framing::Magic` unconditionally reads/compares EXACTLY 8
raw bytes regardless of the declared hex literal's own width. A 6-hex-byte `0x414331303138`
("AC1018") declaration still got read as an 8-byte, left-zero-padded `[0,0,65,67,49,48,49,56]`
against the real on-disk 6-byte magic immediately followed by 2 real header bytes — a genuine
`walk_protocol` failure, caught by this wave's own `protocol_walk_law` test (not assumed, not
guessed at from reading the dialect source — hit for real, then fixed). Fix, matching GIF's own
precedent exactly: `framing record` (not `framing magic ...`) + a real, individually-walked
`field magic fixed 6` inside the header block — genuinely walked but not byte-validated at the
protocol-description layer (the real Rust `dwg_version_sentinel` already validates the `AC` +
4-digit shape). Applied identically to both standards' snapshot protocol files.

## 4. Diff/mutations facets — real text form captured from live `print_op`/`print_diff`, never
guessed

Every grammar production was written from real, directly-observed output of a temporary
`#[test]` calling the actual `print_op()`/`print_diff()`/`print_dsl()`/`encode_pack()` functions
(never hand-derived independently of the real code path, per the recipe's mandatory method) — the
temp tests were run once via `cargo test ... -- --nocapture`, their output copied into the
grammar files' own doc comments verbatim, then the temp tests were either deleted or converted
into permanent `pub(crate) fn demo_mutation_cases()`/`demo_diff_cases()` helpers (mirroring
`stdio.binary`'s own established pattern, incl. its `#[cfg(test)]`-gating on the function itself
so a normal `cargo build` doesn't warn about a test-only helper as dead code).

Confirmed print conventions, both standards (general `keyed_field_rank` derive behavior, not
DWG-specific): scalar fields (incl. `#[dsl(base64)]` `Vec<u8>` fields directly on a struct, which
DO get the compact quoted-base64 form) print first, in declaration order; `List`/nested-record
fields print after, also in declaration order; `Option<Vec<u8>>` does NOT get the base64 form
(the derive's own documented "peels one `Option` layer before checking `attrs.base64`" quirk —
`DwgDiff.bytes` falls back to a verbose bracketed decimal list, confirmed by direct observation:
`bytes=[ 170 187 0 255 ]`); `#[dsl(block)]` fields print `key { ... }`; plain nested-record fields
print `key=<nested fields, flat, no delimiter>`; list-of-record items have no separator, the next
item's own leading key is the structural boundary.

ac1024's richer `sections`/`SetSnapshot.sections` shape (a name-keyed
`removed`/`modified`/`added` triple, generic-derive-driven rather than hand-rolled — the first
GENUINELY DERIVED instance of this collection-triple shape this program has produced, all
prior examples being zip/png's own hand-rolled equivalents) was captured and modeled the same
way, including a `DwgSectionPage.error: Some(String)` case to prove the single-layer
(non-tri-state) `Option<String>` path.

## 5. A genuine pre-existing bug fixed in scope: wrong-type shim usage in `⚙️engine`

ac1018's `⚙️engine/🦀️component.rs` (BEFORE this session) imported `DwgSnapshot`/`DwgDiff`/
`DwgMutation`/`register_pilot_languages`'s grammar/protocol constants via the bare
`crate::artifacts::dwg::{...}`/`crate::artifacts::dwg::schema::...` paths — which the top-level
`📦️glue.rs` shim (an off-limits file, read-only for this wave) aliases to the CANONICAL ac1024
standard, per its own documented comment: "default standard switched ac1018 -> ac1024... ac1018
stays mounted above, fully untouched". ac1018's own `🔺️diff/🦀️component.rs`/`🧬️mutations/
🦀️component.rs` files already explicitly warn against this exact pitfall in their own module
docs ("NOT `crate::artifacts::dwg::DwgSnapshot`... must be reached through its own
fully-qualified standard path") but `⚙️engine/🦀️component.rs` itself had NOT been updated to
follow that same rule — meaning its `register_pilot_languages()`/(now new) `register_schema_specs()`
would have silently pointed at ac1024's grammar/protocol/types instead of ac1018's own. Fixed by
switching `register_pilot_languages()`/`register_schema_specs()`/the new `demo_dwg_snapshot()`/the
new `conformance_laws` test module to the fully-qualified
`crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::...` path, matching the
sibling files' own established convention. Left `DwgEngine`/`DwgArtifact`/the pre-existing
`codec_round_trip` test's shim-aliased imports untouched — `DwgArtifact` (this standard's
artifact-level wrapper, `🪆️subsets/✳️any/🧬️schema/🦀️component.rs`) is a SEPARATE, deliberate
design choice (not part of this FG2 wave's checklist) to route through the canonical richer
`DwgSnapshot` type for its own `to_snapshot`/`from_snapshot` conversions; redesigning that is out
of scope and risks a wider, unrelated refactor.

Note: `register_schema_specs()` on ac1018 registers under a standard-qualified id
(`"stdio.dwg.ac1018"`/`"stdio.dwg.ac1018#diff"`), NOT the bare `"stdio.dwg"` the pre-existing
`register()`/`register_pilot_languages()` already used before this session — deliberately, since
ac1018's `register()` is dead code (never invoked from the real plugin bootstrap; only the
top-level `dwg::engine::register()` — i.e. ac1024's — is called, per `📦️glue.rs`'s own comment
about "several other plugins... target `Dialect{standard: StandardId("ac1018")}` directly"),
and using the bare id would collide with ac1024's own identically-shaped, ALREADY-live
registration under the same string if ac1018's `register()` were ever wired live. This is new
code with full latitude to be collision-safe; the pre-existing `register_pilot_languages()`/
`register()` id scheme (`"stdio.dwg"` etc., also unchanged and dead-code) was left as originally
authored per the "edit existing files, don't redesign architecture" rule — see `deviations`.

## 6. Fixtures

- ac1024 (the CANONICAL standard, per the shim): the pre-existing SHARED artifact-level
  `📚️examples/🎬️demo/🖼️assets/` was already an AC1018-shaped 22-byte stub with a stale,
  non-preamble, wrong-hex `🗣️example.dsl.semio` and no `🎒️example.pack.semio` at all. Switched to
  a genuine AC1024-shaped 22-byte stub (`b"AC1024" + 16×0x00`, matching this standard's own
  pre-existing `codec_round_trip` test fixture exactly) and regenerated both `.dsl.semio`/
  `.pack.semio` from the REAL `print_dsl()`/`encode_pack()` output (captured via a temp test, then
  written verbatim — never hand-derived).
- ac1018 (the legacy/frozen standard, Decision #5): given its OWN dedicated fixture location,
  `🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets/{🗣️example.dsl.semio,🎒️example.pack.semio}`
  (new directory, within this artifact's ownership boundary) — reusing the EXISTING, unchanged
  `📚️examples/🎬️demo/🖼️assets/example.dwg` raw bytes (the exact `b"AC1018"+16×0x00` stub, already
  byte-identical to this standard's own pre-existing `codec_round_trip` test), with real,
  freshly-captured `.dsl.semio`/`.pack.semio` output.
- Neither standard shipped a `📡️example.spr.semio` (optional/bonus per the checklist, not
  blocking) — skipped to keep scope tight; the `protocol_walk_law` test already exercises real
  `encode_op` bytes for every `demo_mutation_cases()` variant directly, which is the actual
  functional coverage that fixture would otherwise round-trip-prove.

`bun run ./📜️script.ts policy` flags both new `🎒️example.pack.semio` fixtures under
`handcrafted-grammar/empty-example` (51 bytes ≤ the 64-byte threshold). Confirmed this is a
pre-existing, REPO-WIDE backlog (86 total instances at time of this report, incl. `stdio.binary`'s
own 37-byte `🎒️example.pack.semio` and `stdio.txt`'s own 47-byte one — both explicitly cited by
this program as "already real, pilot-proven" reference implementations) — not something any
single FG-wave standard is expected to individually clear; matches the recipe's own "you do NOT
edit these policy rules' allowlists yourself" guidance. Not fixed; recorded here for visibility,
not as a blocker.

## 7. Conformance laws

All 6 laws, per standard, in `⚙️engine`'s own `conformance_laws` test module (never a framework
file): `committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`. ac1024's
`grammar_conformance_law`/`protocol_walk_law` additionally exercise the REAL ~145KB
`architectural.dwg` fixture (not just the minimal demo stub) — a second, genuinely non-trivial
real-fixture recognition/walk, on top of the demo case every other standard's own laws use.

## 8. Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::dwg"` → **50 passed, 0 failed** (up from
  the pre-existing 38; all 38 original tests still pass unmodified, plus 12 new: 6 conformance
  laws × 2 standards — `ops_grammar_conformance_law`/`diff_grammar_conformance_law`'s own loop
  bodies additionally exercise every `demo_mutation_cases()`/`demo_diff_cases()` case
  individually, not counted as separate `#[test]` fns).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1773 passed, 0 failed, 1 ignored**
  (exceeds the recipe's own "expect ≥1714/0/1-ignored" bar).
- `bun run ./📜️script.ts policy` → 0 new breaches in the families this recipe names
  (`POLICY_GRAMMAR_PARSEABILITY`/`POLICY_PROTOCOL_PARSEABILITY`/`POLICY_FIXTURE_HONESTY`/
  `POLICY_LANGUAGE_REGISTRATION`/`POLICY_STDIO_JSON_TRANSFER_BAN`) — the two dwg-specific NEW
  findings (`handcrafted-grammar/empty-example` ×2) are a different, pre-existing repo-wide
  family (§6 above); the `stdio-artifacts/codec-id-uniqueness` finding on both `⚙️engine` files
  pre-dates this session (confirmed via `git diff -U0` — the flagged `register_document_codec`
  line was never touched by this wave).
- Two transient, unrelated concurrent-session compile breaks were hit mid-session (`☁️las` missing
  fixture asset, `🎞️gif/87a` missing fixture asset) — both classified by file path as sibling
  in-progress work per the repo rules, waited out, both cleared on retry without any action from
  this session.

## 9. Files touched (all within `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/**`)

Rewritten (real dialect syntax, from ABNF placeholder):
- `🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/{📝️text/📖️component.grammar.semio,💾️binary/📡️component.protocol.semio}` (6 files)
- `🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/{📝️text/📖️component.grammar.semio,💾️binary/📡️component.protocol.semio}` (6 files)

Extended (new regions/functions, existing structure preserved):
- `🏅️standards/🔖️ac1018/⚙️engine/🦀️component.rs` — `demo_dwg_snapshot()`, `register_schema_specs()`,
  5-role `register_pilot_languages()` (fully-qualified, fixing the shim-type bug), full
  `conformance_laws` test module.
- `🏅️standards/🔖️ac1024/⚙️engine/🦀️component.rs` — same additions, shim references already correct
  for this standard (it IS the canonical one).
- `🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — new
  `#[cfg(test)] pub(crate) fn demo_mutation_cases()`, `op_text_binary_roundtrip_law` refactored to
  call it (dedup, per CLAUDE.md).
- `🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — new
  `#[cfg(test)] pub(crate) fn demo_diff_cases()`, `diff_codec_text_binary_roundtrip_law` refactored.
- `🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — new
  `#[cfg(test)] pub(crate) fn demo_mutation_cases()`; `op_text_binary_roundtrip_law` now extends it
  with one extra `error:Some(...)` case rather than duplicating the whole list.
- `🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — new
  `#[cfg(test)] pub(crate) fn demo_diff_cases()`; local test-only `page`/`section` helpers removed
  (now dead after the extraction).

Fixtures:
- `📚️examples/🎬️demo/🖼️assets/example.dwg` — content switched AC1018→AC1024 stub (22 bytes).
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated from real `print_dsl()`.
- `📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new, from real `encode_pack()`.
- `🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — new.
- `🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` — new.

Not touched (out of ownership boundary / off-limits, confirmed by name): `📦️glue.rs`,
`📜️script.ts`, SDK traits, schema/dsl/protocol/registry modules, `🧪️fixture-sweep` graduation
list, `🏪️store`. Not touched (in-boundary but deliberately out of THIS wave's checklist):
`🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (`DwgArtifact`'s own
canonical-type routing decision) and the pre-existing `register()`/`register_pilot_languages()`
bare-`"stdio.dwg"` id scheme in both `⚙️engine` files.
