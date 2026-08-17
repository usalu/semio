# W-S Codec Wave — `stdio.semio.video` (`✳️video` subset)

Real-codec wave for a **semio** subset (`🧿️semio`), following the proven, fully-verified
`✳️workflow` pilot (`ws-codec-workflow-report.md`) and `✳️image` wave (`ws-codec-image-report.md` —
closest precedent per the brief: video, like image, is a "typed-raw-retention" subset — real
container-level metadata, compressed sample bytes stored honestly as opaque buffers, never
decoded) templates and `📖️grammar-recipe.md`. Scope: `✳️video`'s three facets (snapshot, diff,
mutations), plus a new example fixture slug.

**Status: fully verified green in this session — real command output for every claim below, no
deferral.**

---

## 1. Derive path vs hand-rolled — what actually happened

Per this wave's brief, the `#[derive(dsl::DslArtifact)]` path was checked first. It is blocked by
a sharper instance of the `derive-nested-multi-buffer-record` gap mesh's own wave first named:
`SemioVideoStream.samples: Vec<SemioVideoSample>` nests a `Vec<u8>` opaque buffer field (`data`)
inside a `Vec<T>`-of-struct field (`streams`) itself nested inside the snapshot — the derive's
`#[dsl(table)]`/`Vec<Record>` support (confirmed by reading the framework's `SceneDocument`/
`TableDocument` worked examples) covers one level of id-keyed `Vec<Record>`; it has no tested path
for a nested buffer-bearing leaf record two collections deep. Hand-rolled immediately, reusing
video's own `SemioVideoSample.data: Vec<u8>` buffer field the same hex/bracket-encoded way
mesh's/image's own buffer fields are handled.

**Decision**: hand-rolled `ArtifactDsl`/`ArtifactPack` for the snapshot (never regressing to
hex-of-JSON). Going into this wave, `🔺️diff`/`🧬️mutations` ALREADY had real, hand-rolled TEXT
codecs (`print_semio_video_diff`/`parse_semio_video_diff`, `print_semio_video_mutation`/
`parse_semio_video_mutation`) — confirmed by reading the pre-wave files — but BOTH were still on
the F6 `print_diff().into_bytes()`/`print_op().into_bytes()` text-as-binary shortcut for their
`encode_diff`/`encode_op` binary faces (checked both carefully per the brief's item 4 — found, not
assumed already-real). Both upgraded to real binary frames, matching workflow's/mesh's own
upgraded shape (format+presence header for diff, format+tag header for mutations).

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` in `📸️snapshot/🦀️component.rs` now produce/
  consume a genuine 2-line structured body: `schema=<hex>`, `streams=[<stream>,...]` — every
  `stream`/`sample` value a real bracket-nested token tree (hex for `schema`/`codec`/`data` via the
  `hex` macro, a single-letter tag for `kind`, plain integers for `width`/`height`/`pts`, `[num,den]`
  for `rate`) — not a hex dump of a JSON blob. Preamble handling unchanged
  (`store::semio_format::split_text_preamble`/`wrap_text`).
- [x] **Real binary pack** — `encode_pack_with`/`decode_pack_with` now call
  `encode_video_snapshot_binary`/`decode_video_snapshot_binary`: `format u8` + varint-length-
  prefixed `schema` UTF-8, then varint stream count and per-stream `kind` tag `u8`, length-prefixed
  `codec`, `width`/`height` `u32` LE, `rate.num`/`rate.den` 8-byte LE `i64` each, varint sample
  count and per-sample `pts` `u64` LE + `key` `u8` (0/1) + length-prefixed opaque `data`
  (`store::pack_rt::write_varint_u64`, `store::ByteReader` — same primitives workflow's/mesh's/
  image's own upgraded facets use). Replaces the old `serde_json::to_vec`-in-envelope shortcut
  entirely. Hand-rolled, not `store::pack_rt::encode_document` (needs a derived `RecordSpec`, which
  — per §1 — doesn't exist here).
- [x] **Grammar file** — `📸️snapshot/📝️text/📖️component.grammar.semio`, real dialect syntax
  (bare `hex` macro, one production per line), matching `print_video_snapshot_body` field-for-field.
- [x] **Protocol file** — `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `header fixed 1
  {field format u8}` + real bare `segment schema_len varint` / `segment schema_bytes
  Array(u8, Field(schema_len))` (the proven bare form — auto-merges into one segment), then one
  honest opaque `chain payload bytes` tail for `streams` (`protocol-array-of-records` gap —
  homogeneous-but-variable-length repeated records, doubly so since each stream's own `samples`
  sub-collection carries an opaque `data` buffer). Real Rust encode/decode stays fully structured
  past that point.
- [x] `🅰️component.g4`/`🔤️component.ebnf` (text mirrors), `🥋️component.ksy`/`🌶️component.spicy`/
  `🔠️component.abnf` (binary mirrors) — rewritten from the OLD ABNF-style envelope-header-plus-
  hex(JSON) placeholder description to real, descriptive (not test-parsed) mirrors of the new
  grammar/protocol shape.
- [x] **Fixtures** — `📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
  generated via the prescribed temporary-test method (§4 below) — genuine `print_dsl()`/
  `encode_pack()` bytes, never placeholder text.

### Diff (`🔺️diff/`)

- [x] **Already-real text codec confirmed unchanged** — `print_semio_video_diff`/
  `parse_semio_video_diff` were ALREADY a real hand-rolled hex/bracket grammar going into this wave
  (index-keyed, via `engine::triples::enc_indexed_triple`/`dec_indexed_triple` — no natural id for
  `streams`/`samples`) — verified by reading the pre-wave file.
- [x] **Binary upgrade** — was on the pre-wave `print_diff().into_bytes()` text-as-binary shortcut
  (confirmed by reading the pre-wave file). Now: `format u8` + `presence u8` (bit0 = `streams`
  present) as two real fixed header fields, then (when present) one varint-length-prefixed opaque
  text blob (the same `enc_streams_diff` text `print_diff` already emits). One opaque trailing
  `payload` chain in the protocol description, for the `protocol-cond-cannot-chain` reason
  workflow's/mesh's own diff protocols document (a second `if`-guard on a conditionally-decoded
  field hard-errors `eval_cond`).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — rewritten with the `[removed];
  [modified];[added]` index-keyed collection-triple pattern, recursive one level down for each
  modified stream's own `samples` field, and the tri-state `[0]`/`[1,<value>]` pattern for every
  `Option<T>` diff field.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf mirrors rewritten to the real shape; ksy/spicy/abnf mirrors rewritten from the old
  "identical to text, `print_diff().into_bytes()`" description to the real binary frame shape.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — reuses the existing
  test module's `snapshot_a()`/`snapshot_b()` fixtures (promoted to `pub(crate)`) rather than
  duplicate them, covering the empty (no-op) diff and both `a↔b` directions (`streams.removed`/
  `.modified`/`.added` AND, within a modified stream, nested `samples.removed`/`.modified`/`.added`).

### Mutations (`🧬️mutations/`)

- [x] **Already-real text codec confirmed unchanged** — `print_semio_video_mutation`/
  `parse_semio_video_mutation` (`keyword arg=value ...` shape) were ALREADY real going into this
  wave.
- [x] **Binary upgrade** — same shortcut, same treatment. `format u8` + `tag u8` (variant ordinal,
  new `OP_KEYWORDS`/`variant_ordinal`, 0-8 across all 9 `SemioVideoMutation` variants) as two real
  fixed fields, then the variant's own argument text (the keyword prefix stripped via new
  `print_semio_video_mutation_args`) as one opaque trailing `bytes` chain.
- [x] Grammar/protocol/mirrors, same treatment — grammar traced verbatim from
  `print_semio_video_mutation`'s real `format!(...)` call sites.
- [x] Added module-scope `demo_mutation_cases()` (`#[cfg(test)] pub(crate) fn`) for the
  conformance-law tests; delegates to the existing test module's own `sample_mutations()` (promoted
  to `pub(crate)`) rather than keep an independent copy (byte-identical, per the dedupe convention
  image's/mesh's own waves establish).

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` (video's composer already had a `mod tests` for its own `SubsetValidator`/
invariant-check tests, but no conformance-law submodule) — same location/shape workflow's/mesh's/
image's own reports identify as the right home (video likewise has no per-standard `⚙️engine/` test
module of its own; `🎹️composer` is the closest "engine-equivalent").

### JSON-transfer ban (checklist item 8)

Grepped every changed `.rs` file for `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/
`Value` — **clean** (zero real hits; the only two remaining mentions are doc comments describing
the OLD, now-replaced shortcut, in `📸️snapshot/🦀️component.rs` and `📸️snapshot/💾️binary/
🦀️component.rs`).

---

## 3. A real mechanism-gap bug found and fixed mid-wave: combined-bracket option-tag literals

**Symptom**: after the initial diff binary upgrade + real grammar rewrite, `cargo test`'s
`diff_grammar_conformance_law` failed to recognize genuine `print_diff()` output for a modified
stream (`recognizer.recognize(...)` returned `Ok(false)`), even though `committed_facet_files_parse`
(the grammar file itself parses) and `diff_codec_text_binary_roundtrip_law` (the Rust encode/decode
round-trips) both passed — i.e. the TEXT was correct, only the grammar's RECOGNITION of it was
broken.

**Root-caused** by reading `match_symbol_tracked`'s `Symbol::Literal` arm directly
(`🧰️framework/…/🗣️dsl/📖️grammar/🦀️component.rs`): a quoted grammar literal like `"[0]"` requires an
EXACT match against a SINGLE input token's text. Since the shared lexer tokenizes `[` (`LBracket`)
and `]` (`RBracket`) as their OWN separate tokens (confirmed by reading the lexer's bracket-handling
arm directly), a combined literal `"[0]"` can **never** match — no single input token's text is ever
literally `"[0]"`. Confirmed empirically with a throwaway minimal-grammar test (`x = "[0]"` failed to
recognize `"[0]"`; `x = "[" "0" "]"` — three separate literal tokens — succeeded).

My first draft of `opt-kind`/`opt-codec`/.../`opt-samples` copied `workflow`'s own
`option-hex = "[0]" | "[1," hex "]"` shape verbatim (combined-literal form) — this is the SAME
latent bug, just never exercised by workflow's own `diff_grammar_conformance_law` test cases
(apparently none of workflow's demo diffs ever produce a genuinely-unchanged optional field inside
a modified item, so the `"[0]"` branch was never recognized against real text in that wave's test
suite — an untested-path gap, not a proven-working pattern). **Mesh's own diff grammar already
uses the CORRECT three-separate-literal form** (`"[" "0" "]" | "[" "1" "," ... "]"`) — re-reading it
closely after the failure surfaced the fix immediately. Rewrote every `opt-*` production in video's
diff grammar to the three-token form; `diff_grammar_conformance_law` went green immediately after.

**Recommend**: any future semio subset (or a return pass over workflow's own committed grammar
file) should audit every `"[0]"`/`"[1,"`-shaped combined literal for this exact bug — it is silent
(no parse error, just `recognize() == false`) and will only surface if the demo/fixture diff cases
happen to exercise the "unchanged optional field inside a changed item" case, which is easy to miss
by construction. Filed as a new mechanism-gap entry below.

---

## 4. Exact files touched

All paths relative to repo root. 26 files modified inside `✳️video/`, plus one new example slug
outside it (explicitly permitted by the brief).

**Snapshot**: `…/✳️video/🧬️schema/📸️snapshot/🦀️component.rs`,
`…/📸️snapshot/📝️text/📖️component.grammar.semio`, `…/📸️snapshot/📝️text/🦀️component.rs`,
`…/📸️snapshot/📝️text/🅰️component.g4`, `…/📸️snapshot/📝️text/🔤️component.ebnf`,
`…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/💾️binary/🦀️component.rs`,
`…/📸️snapshot/💾️binary/🥋️component.ksy`, `…/📸️snapshot/💾️binary/🌶️component.spicy`,
`…/📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🅰️component.g4`, `…/🔺️diff/📝️text/🔤️component.ebnf`,
`…/🔺️diff/💾️binary/📡️component.protocol.semio`, `…/🔺️diff/💾️binary/🦀️component.rs`,
`…/🔺️diff/💾️binary/🥋️component.ksy`, `…/🔺️diff/💾️binary/🌶️component.spicy`,
`…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/💾️binary/🦀️component.rs`,
`…/🧬️mutations/💾️binary/🥋️component.ksy`, `…/🧬️mutations/💾️binary/🌶️component.spicy`,
`…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️video/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️video/`, explicitly permitted by the brief):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🎥️clip/🦀️component.rs`,
`…/🎥️clip/🟦️component.ts`, `…/🎥️clip/🖼️assets/🗣️example.dsl.semio` (genuine `print_dsl` output),
`…/🎥️clip/🖼️assets/🎒️example.pack.semio` (genuine `encode_pack` bytes). Note: like the `🧊️cube`/
`🖼️swatch` precedent examples, this slug is not wired into any `mod` tree from outside its own
directory (no such registration site was found for the prior examples either) — it is not compiled
as part of the crate; its own `#[cfg(test)] mod tests { fn clip_source_nonempty() }` therefore does
not run under `cargo test -p semio-s-plugin-stdio`, consistent with cube's/swatch's own committed,
accepted state.

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, and every other subset were
left untouched, per the brief.

---

## 5. Fixture generation method (recipe's prescribed procedure, followed exactly)

Added a temporary `#[test] fn ws_temp_print_real_fixtures()` to `🎹️composer/🦀️component.rs`'s new
`conformance_laws` module that called the real `store::ArtifactDsl::print_dsl(&demo)` /
`store::ArtifactPack::encode_pack(&demo)` for `snapshot::demo_video_snapshot()` and `eprintln!`'d
both outputs (DSL as UTF-8 text, pack as a hex dump). Ran it once with `cargo test ...
ws_temp_print_real_fixtures -- --nocapture`, captured the real stdout, then used a small Python
script to write the DSL text verbatim (`newline="\n"`, no extra trailing newline — matching
`store::semio_format::wrap_text`'s real implementation, which does not append one) and decode the
hex dump into the real pack bytes (`bytes.fromhex(...)`) — never hand-transcribed. Deleted the
temporary test immediately after. `fixture_honesty_law` is the independent proof this worked (run
before real fixtures: failed on the placeholder text exactly as expected; re-run after fixture
generation: green, see §6).

---

## 6. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's `streams` (each with a nested `samples` sub-collection and an opaque `data` buffer) — homogeneous variable-length repeated records. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's `streams` — ONE independently-optional top-level field (video's `SemioVideoDiff` has only `streams: Option<...>`, unlike workflow's two or mesh's/image's three-plus); same `presence`-bitmask (1 meaningful bit) + opaque-tail treatment for consistency with the sibling waves' shape, even though a bare flag byte would have sufficed for one field. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types), same as workflow/mesh/image. |
| `derive-nested-multi-buffer-record` (mesh's own gap, re-hit) | no, but same root cause as an existing recipe row | `SemioVideoStream.samples: Vec<SemioVideoSample>` nests a `Vec<u8>` buffer field inside a `Vec<T>`-of-struct field inside the snapshot — the exact shape mesh's own report first named (there: five sibling buffer fields on one leaf record; here: one buffer field on a leaf record one level shallower, same wall). Confirms the gap generalizes to ANY leaf record with a `Vec<u8>` buffer field nested inside a `Vec<T>` collection, not just mesh's specific five-buffer shape. |
| **`grammar-combined-bracket-literal-never-matches`** (NEW — not in recipe's table) | no | A quoted grammar literal spanning multiple lexer tokens with no whitespace between them (e.g. `"[0]"`, `"[1,"`) can **never** match, because `Symbol::Literal` requires an EXACT single-input-token match and the shared lexer tokenizes `[`/`]`/digits/commas as separate tokens. Silent failure mode: the grammar FILE still parses fine (`parse_grammar`/`committed_facet_files_parse` never sees the problem — it's a recognition-time issue, not a parse-time one), and `recognize()` just returns `Ok(false)` with no diagnostic pointing at the cause. Hit here because video's `diff_grammar_conformance_law` happened to exercise a genuinely-unchanged optional field inside a modified item; **workflow's own committed diff grammar has the same latent bug** (`option-hex = "[0]" | "[1," hex "]"`) but it was never caught because that wave's own demo diff cases never happened to hit the "[0]" branch during recognition. Mesh's diff grammar already used the SAFE three-separate-literal form (`"[" "0" "]" | "[" "1" "," ... "]"`) — apparently independently arrived at, not documented as a fix for this specific issue. **Recommend**: (a) every future subset's tri-state/option-tag productions MUST use the three-separate-literal form, never a combined multi-char bracket literal; (b) a follow-up pass should audit workflow's own `📝️text/📖️component.grammar.semio` files (snapshot's `option-hex` pattern doesn't exist there since workflow's snapshot has no tri-state fields, but its diff grammar's `option-hex`/`option-params-diff`/`option-point2` all use the combined-literal form) and fix them the same way, adding a demo-diff-case fixture that actually exercises the "[0]" branch during recognition to catch a regression. |

---

## 7. Verification — real, not claimed (this session, foreground, actually observed)

All commands below were run directly in the foreground in this session and their real output was
read before writing this report.

1. **`cargo check -p semio-s-plugin-stdio`** →
   ```
   warning: `semio-s-plugin-stdio` (lib) generated 491 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 180 suggestions)
       Finished `dev` profile [unoptimized] target(s) in 9.07s
   ```
   **0 errors.**

2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::video"`**
   → **32 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests individually
   confirmed `ok`: `committed_facet_files_parse`, `grammar_conformance_law`,
   `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
   `fixture_honesty_law`. (Along the way: first run of `diff_grammar_conformance_law` after the
   initial grammar draft genuinely failed — a real bug, §3 above — fixed and re-confirmed green
   before this final run; first run of `fixture_honesty_law` before fixture generation also
   correctly failed on the placeholder text, exactly as expected.)

3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) →
   ```
   test result: ok. 1922 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 13.23s
   ```
   **Zero failures, zero regressions anywhere** — no concurrent-session churn to note this time
   (whole-crate run was clean on the first attempt).

4. **JSON-transfer ban grep** (`serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/`Value`
   across every changed `.rs` file) → zero real hits, two doc-comment-only mentions of the old,
   now-replaced shortcut.

**Status: this wave is genuinely proven, fully green for `✳️video`.** No unrelated failures to
report from sibling agents (audio/animation/presentation) — the whole-crate run was clean.
