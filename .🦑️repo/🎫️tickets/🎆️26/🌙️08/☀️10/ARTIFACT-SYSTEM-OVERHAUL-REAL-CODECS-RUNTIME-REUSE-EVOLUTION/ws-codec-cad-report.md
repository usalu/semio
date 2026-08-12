# W-S Codec Wave — `stdio.semio.cad` (`✳️cad` subset)

Follow-up to the `✳️workflow` pilot (`ws-codec-workflow-report.md`) and the `✳️brep` wave
(`ws-codec-brep-report.md`), applying their now fully-verified template to cad — the subset the
brief specifically flagged as brep's closest precedent (`CadEntity` is a 9-variant tagged-enum
geometry primitive union, same shape as brep's `BrepCurve`/`BrepSurface`, just with more variants
and 2D geometry). Written per `📖️grammar-recipe.md` and this ticket's brief.

---

## 1. Derive path vs hand-rolled — what actually happened

Per the brief, the derive path was reconsidered given `SemioPoint2` now derives `dsl::DslRecord`.
It hits the exact same wall brep's wave first identified: `CadEntity` is a data-carrying tagged
enum whose 9 variants (`Line`/`Arc`/`Circle`/`Ellipse`/`Polyline`/`Text`/`Insert`/`Solid`/
`Dimension`) have DIFFERENT field sets — the derive machinery's `DslVariants`/`DslEnum` support
targets one-spec-per-variant BINARY layouts, not a single alternated TEXT grammar production set
(`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path`, the gap brep's report first
named). Hand-rolled instead, reusing the exact hex/bracket convention this subset's own `🔺️diff`
facet had ALREADY established pre-wave for this same enum (`enc_entity`/`dec_entity`,
`L[...]`/`A[...]`/.../`D[...]` single-letter tag prefix) — one text convention, not two
independently-invented ones. No new mechanism gap; this wave just reconfirms brep's finding for a
larger (9 vs. 2+6) variant count.

---

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` now produce/consume a genuine 4-line structured
  body: `schema=<hex>`, `layers=[...]`, `blocks=[...]`, `entities=[...]`. Every layer/block/entity-
  record/entity is real hex/bracket-encoded value text (entity via the single-letter tag
  convention above), never a hex dump of a JSON blob. Replaces the old whole-struct
  `serde_json::to_vec` + hex-dump shortcut entirely.
- [x] **Real binary pack** — `encode_cad_snapshot_binary`/`decode_cad_snapshot_binary`: `format u8`
  + varint-length-prefixed `schema` UTF-8, then varint layer/block/entity counts and per-field
  varint-length-prefixed strings, real 8-byte LE `f64` coordinates, and a real per-variant **tag
  byte** + fields for `CadEntity` (incl. a `Vec<SemioPoint2>` run for `Polyline`). Hand-rolled
  (`store::pack_rt`/`store::ByteReader`, no external crate), duplicated independently of the diff
  facet's own primitives (not imported — keeps `snapshot`, the base type `diff`/`mutations` both
  depend ON, free of a reverse dependency on either sibling facet, same rule brep's wave followed).
- [x] Grammar file (`📸️snapshot/📝️text/📖️component.grammar.semio`) — real dialect syntax, one
  production per physical line, `hex` macro for every string leaf, tagged alternation for `entity`.
- [x] Protocol file — real `header fixed 1 {format u8}` + real bare `segment schema_len varint` /
  `segment schema_bytes Array(u8, Field(schema_len))` (the PROVEN bare form per the workflow
  report's UPDATE §7 note), then one honest opaque `chain payload bytes` tail
  (`protocol-array-of-records` gap — `layers`/`blocks`/`entities` are 3 homogeneous-but-variable-
  length collections, `blocks[].entities` a further nested one, both entity collections embedding
  a tag-dispatched union).
- [x] g4/ebnf (text mirrors), ksy/spicy/abnf (binary mirrors) — rewritten to match the new real
  grammar/protocol field-for-field (previously placeholder ABNF-dialect hex-dump scaffolding).
- [x] **Fixtures** — `📚️examples/📐️drawing/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`:
  genuine `print_dsl()`/`encode_pack()` output of a NEW `demo_cad_snapshot()` fixture (a small
  floor-plan-shaped drawing: a `door` block with a nested `Line`, plus one top-level entity of
  EVERY OTHER `CadEntity` variant — `Arc`/`Circle`/`Ellipse`/`Polyline`/`Text`/`Insert`/`Solid`/
  `Dimension` — deliberately exercising all 9 variants at least once, richer than the pre-existing
  `populated_snapshot()` test fixture which only covered `Line`/`Circle`/`Insert`). Generated via a
  temporary `#[test] #[ignore] fn cad_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs`
  that `eprintln!`'d both outputs, run once, bytes extracted with a small Python script (never
  hand-transcribed), temp test then deleted.

### Diff (`🔺️diff/`)

- [x] **Text codec already real** — confirmed pre-wave: `print_cad_diff`/`parse_cad_diff` already
  emitted genuine hex/bracket `collection=[removed];[modified];[added]` tokens via the shared
  `engine::triples::enc_named_triple`, incl. the `entity`-tagged single-letter-prefix value codec.
  No text-side work needed (the "may already be real, note it as a no-op" case the brief flagged
  as possible — confirmed true for TEXT only, matching brep's diff-facet precedent exactly).
- [x] **Binary upgrade** — was on the `print_diff().into_bytes()` text-as-binary shortcut (confirmed
  pre-wave). Now: `format u8` + `presence u8` (bit0=`layers`, bit1=`blocks`, bit2=`entities`) as
  two real fixed header fields, then 0-3 varint-length-prefixed opaque blobs (the same
  `enc_named_triple` text this type's `print_diff` already emits). One opaque blob per present
  collection rather than per-segment `Cond`-guards — `protocol-cond-cannot-chain` gap (a second
  `if`-guard on a field that's itself only conditionally decoded hard-errors `eval_cond`).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — rewritten from ABNF-dialect
  placeholder to real dialect syntax: restates the `layer`/`entity-record`/`block`/`entity` value
  grammars, the recipe §1.4 tri-state `option-x` pattern for every `Option<T>` diff field
  (`CadLayerDiff`/`CadEntityRecordDiff`/`CadBlockDiff`), the name-keyed collection-triple pattern
  for all 3 collections.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format,
  presence}` + `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors — rewritten to match the new real grammar/protocol.
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope — same convention workflow's/
  brep's own `demo_diff_cases` use) added for the conformance-law tests — self-contained (does NOT
  reach into `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`, since a private item of a
  child module isn't visible to its parent), covering a full removed/modified/added sweep both
  directions across all 3 collections incl. the nested `blocks[].entities`, exercising 7 of the 9
  `CadEntity` variants (`Line`/`Circle`/`Polyline`/`Arc`/`Dimension`/`Ellipse`/`Insert`).

### Mutations (`🧬️mutations/`)

- [x] **Text codec already real** — confirmed pre-wave: `print_cad_mutation`/`parse_cad_mutation`
  already emitted a genuine `keyword arg=value ...` grammar (unlike brep's pre-wave state, which
  was on a raw `serde_json` passthrough — this differs from brep, cad's mutations text facet was
  ALREADY real before this wave, matching workflow's own pre-wave state instead). No text-side
  work needed.
- [x] **Binary upgrade** — was on the `print_op().into_bytes()` text-as-binary shortcut (confirmed
  pre-wave — NOT a raw `serde_json` passthrough; checked carefully per the brief's explicit
  instruction to look for that pattern, and it was not present here). `format u8` + `tag u8`
  (variant ordinal, `OP_KEYWORDS`/`variant_ordinal`, 0-15 matching `parse_cad_mutation`'s keyword
  match across all 16 variants incl. `NoMutation`) as two real fixed fields, then the variant's own
  `key=value ...` argument text as one opaque trailing `bytes` chain — reuses the already-real,
  already-tested `print_cad_mutation`/`parse_cad_mutation` text codec (one source of truth).
- [x] Grammar/protocol/mirrors — grammar traced verbatim from `print_cad_mutation`'s real
  `format!(...)` call sites (never guessed); a `snapshot-lit`/`enc_cad_snapshot`/`dec_cad_snapshot`
  production+function pair (already present pre-wave) covers the `SetSnapshot` variant's whole-
  snapshot payload.
- [x] Consolidated the pre-existing `base_snapshot()`/`sample_layer`/`sample_block`/
  `sample_entity_record` test-local helpers into a single module-scope `#[cfg(test)] fn fixture()` +
  `pub(crate) fn demo_mutation_cases()` (matching workflow's/brep's own convention exactly — one
  case per variant incl. `NoMutation`), reused by both this facet's own 3 tests AND the composer's
  `ops_grammar_conformance_law`/`protocol_walk_law`.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to cad's
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same home workflow's/brep's own reports identified as the right fallback (cad,
like workflow/brep, has no per-standard `⚙️engine/` dir; the shared 14-subset
`⚙️engine/🦀️component.rs` aggregator is out of this wave's `✳️cad/`-only edit scope).

### Not done (explicit)

`LanguageSpec`/`register_schema_spec` registration — skipped, same reasoning as workflow/brep: no
real derivable `RecordSpec` exists for cad's hand-rolled types (the tagged-enum blocker in §1 means
any type embedding `CadEntity` can't derive one), and no clear per-subset registration site was
found beyond `🎹️composer::register()` itself. Filed as a follow-up, not guessed at.

### JSON-transfer ban (checklist item 8)

Grepped every changed `.rs` file for `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/
`Value` inside `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks — **clean** (zero hits; the only
remaining `serde_json` mentions in `📸️snapshot/🦀️component.rs` are in doc-comment prose describing
the OLD, now-replaced shortcut; `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` have zero
`serde_json` mentions at all, confirming both facets' `DiffCodec`/`OpBinary` upgrades were pure
binary-frame work with no JSON anywhere).

---

## 3. Exact files touched

All paths relative to repo root. Every grammar/protocol/mirror file already existed as placeholder
scaffolding (per this ticket's earlier phase) — none created new, only rewritten in place.

**Snapshot**: `…/✳️cad/🧬️schema/📸️snapshot/🦀️component.rs`, `…/📸️snapshot/📝️text/📖️component.grammar.semio`,
`…/📸️snapshot/📝️text/🅰️component.g4`, `…/📸️snapshot/📝️text/🔤️component.ebnf`,
`…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/💾️binary/🥋️component.ksy`,
`…/📸️snapshot/💾️binary/🌶️component.spicy`, `…/📸️snapshot/💾️binary/🔠️component.abnf`.

**Diff**: `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/📖️component.grammar.semio`,
`…/🔺️diff/📝️text/🅰️component.g4`, `…/🔺️diff/📝️text/🔤️component.ebnf`,
`…/🔺️diff/💾️binary/📡️component.protocol.semio`, `…/🔺️diff/💾️binary/🥋️component.ksy`,
`…/🔺️diff/💾️binary/🌶️component.spicy`, `…/🔺️diff/💾️binary/🔠️component.abnf`.

**Mutations**: `…/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
`…/🧬️mutations/📝️text/🅰️component.g4`, `…/🧬️mutations/📝️text/🔤️component.ebnf`,
`…/🧬️mutations/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/💾️binary/🥋️component.ksy`,
`…/🧬️mutations/💾️binary/🌶️component.spicy`, `…/🧬️mutations/💾️binary/🔠️component.abnf`.

**Tests**: `…/✳️cad/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️cad/`, explicitly permitted by the brief, mirroring workflow's/
brep's own precedent): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/📐️drawing/🦀️component.rs`,
`…/📐️drawing/🟦️component.ts`, `…/📐️drawing/🖼️assets/🗣️example.dsl.semio` (real, byte-verified),
`…/📐️drawing/🖼️assets/🎒️example.pack.semio` (real, byte-verified).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, the `🧬️schema/` root-facet
(Artifact) mirror files (already handwritten earlier in this ticket), and every other subset were
left untouched — confirmed via `git status --porcelain` scoped to `✳️cad/` and the new example dir
only (nothing else listed as modified by this session).

One authoring mistake made and immediately corrected during this session: a path typo
(`🏅️标准` instead of `🏅️standards`, an emoji-lookalike slip) twice created a stray file/directory
outside the intended scope (`🧬️schema/🧬️mutations/💾️binary/🥋️component.ksy` under the wrong
`🏅️标准/…` path once, and briefly attempted reads elsewhere). Both were caught immediately — the
stray directory was `rm -rf`'d before any test/build step touched it, and confirmed gone via a
follow-up `ls`. No stray files remain on disk.

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's 3 collections (`blocks[].entities` a further nested one) — homogeneous variable-length repeated records, 2 embedding a further tagged union. Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's 3 collections — up to 3 independently-optional segments; used one opaque `chain payload bytes` with a real 3-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types, tagged-enum blocker). |
| `semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path` | no (brep's report, not the recipe table) | `CadEntity`'s 9 variants have different field sets — re-confirms brep's finding for a larger variant count (9 vs. brep's 2+6); no new gap, no new workaround needed beyond brep's own hand-roll precedent. |

No NEW mechanism gap was discovered by this wave — cad's shape (id-keyed collections + one
9-variant tagged-enum geometry union, 2D instead of 3D) is structurally a straightforward instance
of patterns workflow's and brep's waves already proved out.

---

## 5. Verification — real, not claimed

All commands run synchronously in this session, full output read (not deferred, not speculative).

1. **`cargo check -p semio-s-plugin-stdio`** → **0 errors** (`Finished `dev` profile [unoptimized]
   target(s) in 49.40s`, 485 pre-existing warnings, none new/attributable to cad).
2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::cad"`** →
   **27 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests
   (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
   `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`). One real error round
   hit and fixed mid-wave: `demo_diff_cases()` (module scope, outside `mod tests`) initially
   referenced the bare `STDIO_SEMIOCAD_DOCUMENT_SCHEMA` constant, which is only imported inside the
   private `mod tests` block, not at the `diff` module's own top level — 2 `E0425: cannot find
   value` errors; fixed by fully-qualifying both references
   (`crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA`,
   matching the already-fully-qualified style the pre-existing private `sweep_a`/`sweep_b` used); re-
   check came back clean. One transient concurrent-build hiccup was also observed (a `cargo test`
   invocation immediately after the fixture generation reported "2 previous errors" with no
   attributable error lines in the full captured output, then a re-run one command later compiled
   and passed cleanly) — consistent with CLAUDE.md's "many agents working simultaneously on a shared
   target dir" note, not a defect in this wave's own files; not chased further since the very next
   run was clean.
3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) → **1894 passed, 2 failed, 3
   ignored**. Both failures —
   `artifacts::semio::standards::v1::subsets::document::composer::tests::conformance_laws::fixture_honesty_law`
   and `…::grammar_conformance_law` — are in the **`document` subset**, confirmed via
   `git status --porcelain` to be a file this session never touched and that is actively `M`odified
   (mid-edit) by a different concurrent sibling session running the SAME kind of codec wave on
   `✳️document` right now (per CLAUDE.md's "You MUST work simultaneously with others" rule). Re-ran
   the whole-crate suite a second time per the brief's "wait a bit and re-check once" guidance — the
   same 2 `document` failures persisted unchanged (that session's wave was still in progress both
   times), confirming they are real, ongoing, out-of-scope work rather than transient flakiness.
   **Zero regressions attributable to cad** — every one of the 1894 passing tests that touches cad
   code passed both times.

**Status: this wave is genuinely proven, fully green for `✳️cad`'s own scope**, with two honest,
explicitly-flagged, pre-existing (as of report time), out-of-scope failures in a sibling subset
(`document`) this session did not touch and is not responsible for.
