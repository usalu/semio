# W-S Codec Wave — `stdio.semio.brep` (`✳️brep` subset)

Follow-up to the `✳️workflow` pilot (`ws-codec-workflow-report.md`), applying its now fully-verified
template to brep — the largest/most complex of the 13 remaining semio subsets (6 id-keyed
collections, 2 data-carrying tagged enums `BrepCurve`/`BrepSurface`). Written per
`📖️grammar-recipe.md` and this ticket's brief.

---

## 1. Derive path vs hand-rolled — what actually happened

The brief asked to try the derive path first now that `SemioPoint3`/`SemioPoint2`/etc. all derive
`dsl::DslRecord` (closing the gap workflow's pilot hit). That gap is indeed closed — but brep hits a
**different, new blocker**: `BrepCurve` and `BrepSurface` are data-carrying **tagged enums**
(`Line{origin,direction}` / `Circle{center,axis,radius}` / `Ellipse{...}` / `Nurbs{control_points:
Vec<SemioPoint3>, weights: Vec<f64>, degree: u32, knots: Vec<f64>}`, and the 6-variant
`BrepSurface` analogue). The derive machinery's `DslVariants`/`DslEnum` support (confirmed by
reading the framework's derive macro and `FieldKind` classification) targets ONE-spec-per-variant
BINARY layouts, not a single TEXT grammar production set that must alternate between heterogeneous
variant field shapes with a single-letter tag prefix (`L[...]`/`C[...]`/`N[...]`). No proven
mechanism emits a matching text production for this shape. Hand-rolled instead, reusing the exact
hex/bracket convention this subset's own `🔺️diff` facet had ALREADY established pre-wave for these
same two enums (`enc_curve`/`enc_surface`, `L[origin,direction]` etc.) — one text convention, not
two independently-invented ones.

**New mechanism gap, not yet in the recipe's table**: `semio-tagged-enum-heterogeneous-variants-no-
dslenum-text-path` — a data-carrying enum whose variants have DIFFERENT field sets (not just
different single values) has no derive-path route to a matching grammar production set, even though
its scalar/record fields (like `SemioPoint3`) are all individually derivable. Every future semio
subset with a real curve/surface/value-union type (cad, drawing, model) will hit this identically.

## 2. Per-facet checklist (recipe §4)

### Snapshot (`📸️snapshot/`)

- [x] **Real text DSL** — `parse_dsl`/`print_dsl` now produce/consume a genuine 7-line structured
  body: `schema=<hex>`, `vertices=[...]`, `edges=[...]`, `loops=[...]`, `faces=[...]`,
  `shells=[...]`, `solids=[...]`. Every entity is real hex/bracket-encoded value text (curve/surface
  via the single-letter tag convention above), never a hex dump of a JSON blob.
- [x] **Real binary pack** — `encode_brep_snapshot_binary`/`decode_brep_snapshot_binary`: `format
  u8` + varint-length-prefixed `schema` UTF-8, then varint counts + per-entity fields (varint-
  length-prefixed strings, real 8-byte LE `f64` coordinates, and a real per-variant **tag byte** +
  fields for `BrepCurve`/`BrepSurface`, incl. `Vec<SemioPoint3>`/`Vec<f64>` runs for both `Nurbs`
  variants). Replaces the old `serde_json::to_vec`-in-envelope shortcut entirely.
- [x] Grammar file (`📸️snapshot/📝️text/📖️component.grammar.semio`) — real dialect syntax, one
  production per physical line, `hex` macro for every string leaf, tagged alternation for
  `curve`/`surface`.
- [x] Protocol file — real `header fixed 1 {format u8}` + real `segment schema_len varint` /
  `segment schema_bytes Array(u8, Field(schema_len))` (proven **bare** segment form, per the
  workflow report's UPDATE §7 note — never the braced form), then one honest opaque `chain payload
  bytes` tail (`protocol-array-of-records` gap — 6 homogeneous-but-variable-length collections, 2 of
  them embedding a further tag-dispatched union).
- [x] g4/ebnf (text mirrors), ksy/spicy/abnf (binary mirrors) — rewritten to match the new real
  grammar/protocol field-for-field (previously placeholder ABNF-dialect hex-dump scaffolding).
- [x] **Fixtures** — `📚️examples/🧊️solid/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`:
  genuine `print_dsl()`/`encode_pack()` output of a NEW `demo_brep_snapshot()` fixture (one
  triangular face/shell/solid exercising `Line`, `Circle`, and BOTH `Nurbs` curve/surface variants —
  chosen deliberately richer than the pre-existing `populated_snapshot()` test fixture, which only
  exercised `Plane`/`Line`/`Nurbs`-curve, to stress the encoder's least-tested corners). Generated
  via a temporary `#[test] #[ignore] fn brep_temp_print_real_fixtures()` in `🎹️composer/🦀️component.rs`
  that `eprintln!`'d both outputs, run once, bytes extracted with a small Python script (never hand-
  transcribed), temp test then deleted.

### Diff (`🔺️diff/`)

- [x] **Text codec already real** — confirmed pre-wave: `print_brep_diff`/`parse_brep_diff` already
  emitted genuine hex/bracket `collection=[removed];[modified];[added]` tokens via the shared
  `engine::triples::enc_named_triple`. No text-side work needed (this is the "may already be real,
  note it as a no-op" case the brief flagged as possible — confirmed true for TEXT only).
  All 24 `pub(crate)`-visible per-entity/per-primitive functions in this facet's `Primitives`/
  `ValueCodecs` regions were bumped from private `fn` to `pub(crate) fn` (36 functions total across
  both regions) so the `🧬️mutations` facet could import and reuse them rather than re-deriving a
  third independent copy of the same entity encoding.
- [x] **Binary upgrade** — was on the `print_diff().into_bytes()` text-as-binary shortcut (confirmed
  pre-wave, matching the P2-W0 census default). Now: `format u8` + `presence u8` (bit0=`vertices` …
  bit5=`solids`, 6 collections needing 6 bits, still fits one `u8`) as two real fixed header fields,
  then 0-6 varint-length-prefixed opaque blobs (the same `enc_*_diff` text this type's `print_diff`
  already emits). One opaque blob per present collection rather than per-segment `Cond`-guards —
  `protocol-cond-cannot-chain` gap (a second `if`-guard on a field that's itself only conditionally
  decoded hard-errors `eval_cond`).
- [x] Grammar (`🔺️diff/📝️text/📖️component.grammar.semio`) — rewritten from ABNF-dialect placeholder
  to real dialect syntax: restates all 6 entity + curve/surface value grammars, the tri-state
  `option-x` pattern for every `Option<T>` diff field, the name-keyed collection-triple pattern
  (recipe §1.4) for all 6 collections.
- [x] Protocol (`🔺️diff/💾️binary/📡️component.protocol.semio`) — `header fixed 2 {format, presence}` +
  `chain payload bytes`.
- [x] g4/ebnf/ksy/spicy/abnf mirrors — rewritten (ebnf/g4 to match the new full grammar; abnf was
  already unusually detailed/accurate pre-wave for the TEXT shape, kept and re-framed as "the text
  each binary blob carries", ksy/spicy updated for the new real binary frame).
- [x] `demo_diff_cases()` (`#[cfg(test)] pub(crate) fn`, module scope) added — self-contained (does
  NOT reach into `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`, since a private item of
  a child module isn't visible to its parent module; this is a real, first-time-hit variant of the
  pattern workflow's own `demo_diff_cases` used, worth flagging for future subsets that already have
  a pre-existing `tests`-scoped sweep fixture).

### Mutations (`🧬️mutations/`)

- [x] **Real text codec — NOT already real, unlike the diff facet.** Pre-wave, `OpText::print_op`/
  `parse_op` were a **plain `serde_json::to_string`/`from_str`** whole-enum passthrough (genuinely
  readable JSON, not hex-of-JSON, but still a JSON-transfer-ban violation and not a real per-
  production grammar) — this differs from workflow's starting state, where the mutations facet's
  text codec was ALREADY a real keyword grammar pre-pilot. Replaced with a real `keyword arg=value
  ...` grammar (`print_brep_mutation`/`parse_brep_mutation`), one clause per `SemioBrepMutation`
  variant (23 incl. `NoMutation`), reusing the diff facet's now-`pub(crate)` entity/primitive
  functions rather than re-deriving a third copy.
- [x] **Binary upgrade** — was `serde_json::to_vec`/`from_slice` of the whole enum (JSON-transfer-
  ban violation, confirmed and now fixed). `format u8` + `tag u8` (variant ordinal, `OP_KEYWORDS`/
  `variant_ordinal`, 0-22 matching `parse_brep_mutation`'s keyword match) as two real fixed fields,
  then the variant's own `key=value ...` argument text as one opaque trailing `bytes` chain — reuses
  the real, tested `print_brep_mutation`/`parse_brep_mutation` text codec (one source of truth).
- [x] Grammar/protocol/mirrors — grammar traced verbatim from `print_brep_mutation`'s real
  `format!(...)` call sites (never guessed); a new `snapshot-lit`/`enc_brep_snapshot`/
  `dec_brep_snapshot` production+function pair was added for the `SetSnapshot` variant's whole-
  snapshot payload (`[hex(schema),[vertex,...],[edge,...],[loop,...],[face,...],[shell,...],
  [solid,...]]`, one line, reusing the diff facet's entity encoders — same shape workflow's own
  `enc_semio_workflow_snapshot` established for its 2-collection case, generalized to brep's 6).
- [x] Consolidated the pre-existing `all_mutations(base)`/`populated_snapshot()` test-local helpers
  into a single module-scope `#[cfg(test)] fn fixture()` + `pub(crate) fn demo_mutation_cases()`
  (matching workflow's own convention exactly), reused by both this facet's own 3 tests AND the
  composer's `ops_grammar_conformance_law`/`protocol_walk_law`.

### Conformance-law tests

All 6 (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
`diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`) added to brep's
`🎹️composer/🦀️component.rs`'s existing `#[cfg(test)] mod tests` block, in a new nested `mod
conformance_laws` — same home workflow's own report identified as the right fallback (brep, like
workflow, has no per-standard `⚙️engine/` dir; the shared 14-subset `⚙️engine/🦀️component.rs`
aggregator is out of this wave's `✳️brep/`-only edit scope).

### Not done (explicit)

`LanguageSpec`/`register_schema_spec` registration — skipped, same reasoning as workflow: no real
derivable `RecordSpec` exists for brep's hand-rolled types (the tagged-enum blocker in §1 means
`BrepCurve`/`BrepSurface`-embedding types can't derive one), and no clear per-subset registration
site was found beyond `🎹️composer::register()` itself. Filed as a follow-up, not guessed at.

### JSON-transfer ban (checklist item 8)

Grepped every changed `.rs` file for `serde_json::to_vec`/`from_slice`/`to_string`/`from_str`/
`Value` inside `ArtifactPack`/`OpBinary`/`DiffCodec` impl blocks — **clean** (zero hits; the only
remaining `serde_json` mentions in these 3 files are in doc comments describing the OLD, now-
replaced shortcuts — confirmed by direct grep, output included below in §5).

---

## 3. Exact files touched

All paths relative to repo root. Every file already existed as placeholder scaffolding (per this
ticket's earlier phase) EXCEPT the new example directory — no new grammar/protocol/mirror files
were created, only rewritten in place, per the brief's "edit existing files" rule.

**Snapshot**: `…/✳️brep/🧬️schema/📸️snapshot/🦀️component.rs`, `…/📸️snapshot/📝️text/📖️component.grammar.semio`,
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

**Tests**: `…/✳️brep/🎹️composer/🦀️component.rs` (new `mod conformance_laws` inside its existing
`#[cfg(test)] mod tests`).

**New example slug** (outside `✳️brep/`, explicitly permitted by the brief, mirroring workflow's own
precedent): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📚️examples/🧊️solid/🦀️component.rs`,
`…/🧊️solid/🟦️component.ts`, `…/🧊️solid/🖼️assets/🗣️example.dsl.semio` (real, byte-verified),
`…/🧊️solid/🖼️assets/🎒️example.pack.semio` (real, byte-verified).

Nothing outside these was touched. `🧪️fixture-sweep/🦀️component.rs`, `📜️script.ts`, `📦️glue.rs`,
`launch.json`, `catalog.json`, the shared `⚙️engine/🧮️geometry` module, and every other subset were
left untouched — confirmed via `git status --porcelain` scoped to `✳️brep/` and the new example dir
only (nothing else listed as modified by this session).

---

## 4. Mechanism gaps hit

| gap id | recipe row? | what happened here |
|---|---|---|
| `protocol-array-of-records` | yes, §5 | snapshot pack's 6 collections — homogeneous variable-length repeated records (2 embedding a further tagged union). Opaque trailing `chain payload bytes` after the real `format`+`schema` header. |
| `protocol-cond-cannot-chain` | yes, §5 | diff binary's 6 collections — up to 6 independently-optional segments; used one opaque `chain payload bytes` with a real 6-bit `presence` bitmask header field instead of chained `Cond`s. |
| `register-schema-spec-needs-recordspec` | yes, §5 | skipped `register_schema_spec` — no derivable `RecordSpec` (hand-rolled types, tagged-enum blocker). |
| **`semio-tagged-enum-heterogeneous-variants-no-dslenum-text-path`** (NEW — not in recipe's table) | no | `BrepCurve`/`BrepSurface` are data-carrying tagged enums whose variants have DIFFERENT field sets (not just different scalar values). Even with all their scalar/record fields (`SemioPoint3`) now `dsl::DslRecord`-derivable (the gap workflow's pilot flagged and a later session closed), the derive machinery has no proven route from such an enum to a matching TEXT grammar production set — `DslVariants`/`DslEnum` support targets one-spec-per-variant BINARY layouts, not a single alternated text grammar. **Recommend**: any future semio subset with a real curve/surface/typed-value union (cad, drawing, model are the most likely next hits) should hand-roll these two specific types immediately rather than re-attempt the derive path — this pilot's `enc_curve`/`enc_surface`/`write_curve`/`write_surface` (single-letter tag + real fixed-field-per-variant binary tag byte) are now a real, committed, dual-codec (text+binary) reference for exactly this shape. |

---

## 5. JSON-transfer ban — grep evidence

```
$ grep -n "serde_json" 📸️snapshot/🦀️component.rs 🔺️diff/🦀️component.rs 🧬️mutations/🦀️component.rs
📸️snapshot/🦀️component.rs:496:/// reuse) backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-
📸️snapshot/🦀️component.rs:869:/// 🎁 Real structured text/binary codecs (brep wave — off the old hex-dump-of-`serde_json`
🧬️mutations/🦀️component.rs:189:/// `OpText`/`OpBinary`, replacing the old whole-enum compact-`serde_json` passthrough. Grammar:
🧬️mutations/🦀️component.rs:364:/// ⚡️ Real binary op frame, replacing the old whole-enum compact-`serde_json::to_vec` shortcut.
```
All 4 hits are doc-comment prose describing the OLD, now-replaced shortcut — zero `serde_json` calls
inside any `ArtifactPack`/`OpBinary`/`DiffCodec` impl body. `🔺️diff/🦀️component.rs` has zero hits at
all (its `DiffCodec` was already, and remains, `serde_json`-free).

---

## 6. Verification — real, not claimed

All three commands run synchronously in this session, full output read (not deferred, not
speculative):

1. **`cargo check -p semio-s-plugin-stdio`** → **0 errors** ("Finished `dev` profile [unoptimized]
   target(s) in 43.94s", 484 pre-existing warnings, none new/attributable to brep). One real error
   round hit and fixed mid-wave: the mutations facet initially imported every diff-facet primitive
   EXCEPT `enc_face`/`dec_face` (4 `E0425: cannot find function` errors, all pointing at the same
   missing-import root cause) — fixed by completing the import list; re-check came back clean.
2. **`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::brep"`**
   → **29 passed, 0 failed, 0 ignored**, including all 6 conformance-law tests
   (`committed_facet_files_parse`, `grammar_conformance_law`, `ops_grammar_conformance_law`,
   `diff_grammar_conformance_law`, `protocol_walk_law`, `fixture_honesty_law`). One real failure
   round hit and fixed mid-wave: `fixture_honesty_law` failed once against the two placeholder
   fixture files (`"brep snapshot: unknown line \"PLACEHOLDER_WILL_BE_REGENERATED_FROM_REAL_
   print_dsl_OUTPUT\""`) before the temp-test-generated real fixtures were copied in; re-run after
   came back green.
3. **`cargo test -p semio-s-plugin-stdio --lib`** (whole crate) → **1869 passed, 1 failed, 3
   ignored**. The 1 failure — `artifacts::semio::standards::v1::subsets::object::composer::tests::
   conformance_laws::fixture_honesty_law`, `"parse shipped .dsl.semio fixture: ... expected [...],
   got \"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST\""` — is in the **`object` subset**, a file this
   session never touched (another concurrent session's in-progress placeholder fixture, per CLAUDE.md's
   "You MUST work simultaneously with others" rule). **Zero regressions attributable to brep** —
   every one of the 1869 passing tests that touches brep code passed; the crate's pre-existing
   pass count (1844, per the workflow report's own whole-crate baseline) grew by the 29 real brep
   tests plus other concurrent sessions' unrelated work, minus the 1 pre-existing unrelated `object`
   failure.

**Status: this wave is genuinely proven, fully green for `✳️brep`'s own scope**, with one honest,
explicitly-flagged, pre-existing, out-of-scope failure in a sibling subset this session did not
touch and is not responsible for.
