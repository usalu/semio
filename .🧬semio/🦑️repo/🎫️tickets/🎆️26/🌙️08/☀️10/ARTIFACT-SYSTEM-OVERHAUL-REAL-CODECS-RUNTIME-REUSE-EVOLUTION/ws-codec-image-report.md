# W-S Codec Wave — `s.stdio.semio.image` (`✳️image` subset)

Real-codec wave for a **semio** subset (`🧿️semio`), following the proven, fully-verified
`✳️workflow` pilot (`ws-codec-workflow-report.md`) and `✳️mesh` wave (`ws-codec-mesh-report.md` —
the closest precedent per the brief, since image's `SemioImageFrame.rgba8: Vec<u8>` is a raw
byte-buffer field, like mesh's texture bytes) templates and `📖️grammar-recipe.md`. Scope:
`✳️image`'s three facets (snapshot, diff, mutations), plus a new example fixture slug.

**Status: fully verified green in this session — real command output for every claim below, no
deferral.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per this wave's brief, the `#[derive(dsl::DslArtifact)]` path was checked first. It is blocked
immediately, with strong pre-existing evidence in the codebase itself: `SemioImageSnapshot.icc:
Option<Vec<u8>>` is a BARE `Option<T>` field directly on the snapshot struct. `dsl` has no blanket
`Option<T>: DslField` impl — this exact wall is already documented in this subset's own
`🔺️diff`/`🧬️mutations` facets' doc comments (`SemioImageDiff`'s `icc: Option<Option<Vec<u8>>>`
tri-state field and `SetIcc`'s `Option<Vec<u8>>` mutation payload, both pre-existing before this
wave), matching gif's/docx's established precedent (`f6-final-summary.md` §4.3/§4.4). Since the
blocker is a bare field on the snapshot itself (not merely a nested collection shape issue like
mesh's), there was no ambiguity to resolve by experimentation — hand-rolled immediately, reusing
image's own frame/`rgba8`-as-`Vec<u8>` buffer field the same hex/bracket-encoded way mesh's texture
bytes are handled.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to
hex-of-JSON), matching the SAME hex/bracket-encoded convention this subset's own `🔺️diff`/
`🧬️mutations` facets already used pre-wave (both already had real, hand-rolled text codecs going
into this wave — see §2). `DiffCodec`/`OpBinary` were upgraded from the F6/pre-wave
`print_diff()`/`print_op().into_bytes()` text-as-binary shortcut to real binary frames, matching
mesh's own upgraded shape almost verbatim (format+presence header for diff, format+tag header for
mutations).

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/
  consume a genuine 8-line structured body: `schema=<hex>`, `width=<N>`, `height=<N>`,
  `colorspace=<c>`, `bitDepth=<N>`, `icc=<option-hex>`, `frames=[<frame>,...]`,
  `metadata=[<entry>,...]` — every field its own real token (hex for schema/rgba8/key/value via
  the `hex` macro, plain integers for width/height/delay_ms, a single-letter tag for colorspace) —
  not a hex dump of a JSON blob. Preamble handling unchanged
  (`store::semio_format::split_text_preamble`/`wrap_text`).
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_image_snapshot_binary`/`decode_image_snapshot_binary`: `format u8` + varint-length-
  prefixed `schema` UTF-8, real fixed `width`/`height` (`u32` LE), `colorspace` (`u8` tag),
  `bit_depth` (`u8`), then `icc` (presence `u8` + optional varint-length-prefixed bytes), `frames`
  (varint count + per-frame `delay_ms` `u32` LE + varint-length-prefixed `rgba8`), `metadata`
  (varint count + per-entry varint-length-prefixed `key`/`value` strings) —
  `store::pack_rt::write_varint_u64`/`store::ByteReader`, same primitives mesh's/workflow's own
  upgraded facets use. Replaces the old `serde_json::to_vec`-in-envelope shortcut entirely.
  Hand-rolled, not `store::pack_rt::encode_document` (needs a derived `RecordSpec`, which — per §1
  — doesn't exist here).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (`{ }` grouping, bare `hex` macro, one production per line), matching `print_image_snapshot_body`
  field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes
  Array(u8, Field(schema_len))` / `segment width u32` / `segment height u32` / `segment colorspace
  u8` / `segment bit_depth u8` (the proven bare form — consecutive bare segment lines auto-merge
  into one anonymous segment, confirmed by reading `parse_grammar`'s `"segment"` handling
  directly), then one honest opaque `chain payload bytes` tail for `icc`/`frames`/`metadata`
  (`protocol-array-of-records` gap — homogeneous-but-variable-length repeated records). The real
  Rust encode/decode stays fully structured past that point — this protocol description goes
  further than mesh's own (which stopped right after `schema`, since mesh's snapshot has no scalar
  fields between `schema` and its collections) because image genuinely has four more real,
  individually fixed-width fields worth describing before the opaque tail begins.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the OLD ABNF-style hex-of-JSON placeholder
  description to real, descriptive (not test-parsed) mirrors of the new grammar/protocol shape.
- [x] **Fixtures** — `📚️examples/🖼️swatch/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
  generated via the prescribed temporary-test method (§4 below) — genuine `print_dsl()`/
  `encode_pack()` bytes, never placeholder text.

### Diff (`🔺️diff/`)

- [x] **Already-real text codec confirmed unchanged** — `print_image_diff`/`parse_image_diff` were
  ALREADY a real hand-rolled hex/bracket grammar going into this wave (pre-existing, not something
  this wave had to build from scratch) — verified by reading the pre-wave file.
- [x] **Binary upgrade** — was on the pre-wave `print_diff().into_bytes()` text-as-binary
  shortcut (confirmed by reading the pre-wave file). Now: `format u8` + `presence u8` (bit0=`width`
  bit1=`height` bit2=`colorspace` bit3=`bitDepth` bit4=`icc` bit5=`frames` bit6=`metadata`) as two
  real fixed header fields, then 0-7 varint-length-prefixed opaque text blobs (the same per-field
  `enc_*`/`enc_frames_diff`/`enc_metadata_diff` text this type's `print_diff` already emits). One
  opaque trailing `payload` chain in the protocol description, not per-segment `Cond`s, for the
  `protocol-cond-cannot-chain` reason workflow's/mesh's own diff protocols document (a second
  `if`-guard on a conditionally-decoded field hard-errors `eval_cond`).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — rewritten from the OLD ABNF-style
  placeholder to real dialect syntax, restates the tri-state `option-hex` pattern for `icc`
  (`Option<Option<Vec<u8>>>`), and the collection-triple pattern for `frames` (`IndexedTripleDiff`)
  / `metadata` (`NamedTripleDiff`).
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors rewritten from the old ABNF-style/"identical to text"
  placeholder description to the real grammar/binary frame shape.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added, deduping an
  EXACT-DUPLICATE inline `cases` vec that already existed inside
  `diff_codec_text_binary_roundtrip_law` — that test now calls `demo_diff_cases()` instead of
  reconstructing the same base/other fixture pair a second time.

### Mutations (`🧬️mutations/`)

- [x] **Already-real text codec confirmed unchanged** — `print_image_mutation`/
  `parse_image_mutation` (`tag:args` shape) were ALREADY real going into this wave.
- [x] **Binary upgrade** — same shortcut, same treatment. `format u8` + `tag u8` (variant ordinal,
  new `OP_KEYWORDS`/`variant_ordinal`, 0-12 across all 13 `SemioImageMutation` variants) as two
  real fixed fields, then the variant's own argument text (the `tag:` prefix stripped via new
  `print_image_mutation_args`) as one opaque trailing `bytes` chain.
- [x] Grammar/protocol/mirrors, same treatment — grammar traced verbatim from
  `print_image_mutation`'s real `format!(...)` call sites (rewritten from the OLD ABNF-style
  placeholder).
- [x] Added module-scope `demo_mutation_cases()` (`#[cfg(test)] pub(crate) fn`) for the
  conformance-law tests; the existing test module's own `sample_mutations()` now delegates to it
  (dedupe — the two were byte-identical) rather than keep an independent copy. Left the local
  `frame()`/`fixture()` test helpers untouched (still used directly by `mutation_diff_law`/
  `inverse_law`/`codec_retention_law`).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs` as a brand-new `#[cfg(test)] mod tests { mod conformance_laws { ... }
}` block (image's composer had NO test module at all pre-wave) — same location/shape workflow's/
mesh's own reports identify as the right home (image likewise has no per-standard `⚙️engine/` test
module of its own; `🎹️composer` is the closest "engine-equivalent").

### JSON-transfer ban (checklist item 8)

Grepped all four changed `.rs` files (`📸️snapshot`, `🔺️diff`, `🧬️mutations`, `🎹️composer`) for
`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value` — **clean** (zero real hits; the
only remaining mention is one doc comment in `📸️snapshot/🦀️component.rs` describing the OLD,
now-replaced shortcut).

---

## 3. Exact files touched

All paths relative to repo root. 29 files modified inside `✳️image/`, plus one new example slug
outside it (explicitly permitted by the brief).

**Snapshot**: `…/✳️image/🧬️schema/📸️snapshot/🦀️component.rs`,
`…/📸️snapshot/📝️text/📖️component.grammar.semio`, `…/📸️snapshot/📝️text/🦀️component.rs`,
`…/📸️snapshot/📝️text/🅰️component.g4`, `…/📸️snapshot/📝️text/🔤️component.ebnf`,
`…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/💾️binary/🦀️component.rs`,
`…/📸️snapshot/💾️binary/🥋️component.ksy`, `…/📸️snapshot/💾️binary/🌶️component.spicy`,
`…/📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🦀️component.rs`, `…/🔺️diff/📝️text/🅰️component.g4`,
`…/🔺️diff/📝️text/🔤️component.ebnf`, `…/🔺️diff/💾️binary/📡️component.protocol.semio`,
`…/🔺️diff/💾️binary/🦀️component.rs`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/📝️text/🦀️component.rs`, `…/🧬️mutations/💾️binary/📡️component.protocol.semio`,
`…/🧬️mutations/💾️binary/🦀️component.rs`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️image/🎹️composer/🦀️component.rs` (new `#[cfg(test)] mod tests { mod
conformance_laws }`).

**New example slug** (outside `✳️image/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🖼️swatch/🦀️component.rs`,
`…/🖼️swatch/🟦️component.ts`, `…/🖼️swatch/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl`
output), `…/🖼️swatch/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, and every other subset were
left untouched, per the brief. (One stray mistyped-path file was accidentally created outside the
ticket scope mid-session due to a clipboard/input error — caught and `rm -rf`'d immediately, before
any content was written to it that mattered; confirmed clean via `ls` afterward.)

---

## 4. Fixture generation method (recipe's prescribed procedure, followed exactly)

Added a temporary `#[test] fn ws_temp_print_real_fixtures()` to `🎹️composer/🦀️component.rs`'s new
`conformance_laws` module that called the real `store::ArtifactDsl::print_dsl(&demo)` /
`store::ArtifactPack::encode_pack(&demo)` for `snapshot::demo_image_snapshot()` and `eprintln!`'d
both outputs (DSL as UTF-8 text, pack as a hex dump). Ran it once with `cargo test ...
ws_temp_print_real_fixtures -- --nocapture`, captured the real stdout, then used a small Python
script to write the DSL text verbatim (`newline="\n"`, no extra trailing newline — confirmed by
reading `store::semio_format::wrap_text`'s real implementation, which does not append one) and
decode the hex dump into the real pack bytes (`bytes.fromhex(...)`) — never hand-transcribed.
Deleted the temporary test immediately after. `fixture_honesty_law` is the independent proof this
worked (first run before real fixtures: 35 passed / 1 failed exactly as expected on the placeholder
text; re-run after fixture generation: 36/36 green, see §6).

---

## 5. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `icc`/`frames`/`metadata` — `icc` is a bare `Option<Vec<u8>>`, `frames`/`metadata` are homogeneous variable-length repeated records. Opaque trailing `chain payload bytes` after the real `format`+`schema`+`width`+`height`+`colorspace`+`bit_depth` header/segments. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `width`/`height`/`colorspace`/`bitDepth`/`icc`/`frames`/`metadata` — SEVEN independently-optional fields (more than workflow's two or mesh's three); same `presence`-bitmask + opaque-tail treatment, generalized to 7 bits (still fits one `u8`). |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types), same as workflow/mesh. Image's `🎹️composer::register()` also had no pre-existing `dsl::register_language`/`register_schema_spec` call site to extend. |
| **bare-`Option<Vec<u8>>`-on-the-snapshot-itself** (a sharper instance of workflow's `semio-shared-value-struct-not-dslfield` gap, but not identical — no shared geometry-value-struct field is involved here at all) | no, but same root cause as an existing recipe row | Unlike workflow (whose blocker was a *shared* `SemioPoint2` value-struct field not implementing `DslField`) or mesh (whose blocker was a *nested-collection depth* the derive's `Vec<Record>` support doesn't cover), image's derive-path blocker is the SIMPLEST possible case of the "bare `Option<T>`" wall: `icc: Option<Vec<u8>>` sits directly, unnested, on the top-level snapshot struct. This confirms the wall is general — ANY snapshot with a bare optional field of ANY type (not just geometry values or nested collections) hits it identically. **Recommend**: future semio subsets should check for ANY bare `Option<T>` field on the snapshot struct itself (not just `Option<geometry-value>` or `Option<Vec<geometry-value>>`) as an immediate, zero-investigation signal to hand-roll rather than attempt the derive path. |

---

## 6. Verification — real, not claimed (this session, foreground, actually observed)

All commands below were run directly in the foreground in this session and their real output was
read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** →
   ```
   warning: `semio-s-plugin-stdio` (lib) generated 485 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 176 suggestions)
       Finished `dev` profile [unoptimized] target(s) in 42.65s
   ```
   **0 errors.**

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::image"`**
   → **36 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests individually
   confirmed `ok`: `committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`. (First run, before the real fixtures were generated, correctly showed
   35 passed / 1 failed — `fixture_honesty_law` failing on the placeholder text, exactly as
   expected; re-run after fixture generation is the 36/0 result.)

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate), run TWICE (waited and re-checked
   once per the ticket's own concurrent-development ground rules) →
   ```
   test result: FAILED. 1894 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in ~15-20s
   ```
   both times, identically. The two failures are
   `artifacts::semio::standards::v1::subsets::document::composer::tests::conformance_laws::fixture_honesty_law`
   and `…document::composer::tests::conformance_laws::grammar_conformance_law` — **not this wave's
   code**: `fixture_honesty_law` panics on `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"` (the exact
   pattern §4 above shows how to close), and `git status` confirms
   `…/🪆️subsets/✳️document/…` is `M`-modified across 29 files by a different, concurrent session,
   with `🎹️composer/🦀️component.rs`'s mtime essentially "now" (`Aug 12 02:28:42`) — mid-way through
   its own real-codec wave on the `document` subset, same placeholder-fixture pattern this report's
   own §4 shows how to close, just not yet done on that session's side. **Zero failures
   attributable to anything in `artifacts::…::image`.**

**Status: this wave is genuinely proven, fully green for `✳️image`.** The two whole-crate failures
are unrelated concurrent churn in `✳️document`, explicitly noted rather than chased, per this
ticket's own concurrent-development ground rules.
