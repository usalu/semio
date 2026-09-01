# 📖️ Canvas-family grammar rewrite — the 13 stale `.grammar.semio` leaves, fixed at the source

Scope: the 13 "canvas-family" leaves the sibling report `📓️grammar-envelope-transcription.md`
identified as a byte-identical, cross-plugin copy-pasted "canvas layer" template
(`add-layer`/`set-stroke`/`move-layer`/`set-fill`) disjoint from their own Rust parsers —
`raster`/`forms`/`layout`/`draw`/`playbook` × snapshot/mutations/diff. Unlike the sibling's own
27-leaf pass (transcribe-only, `.semio` untouched), this ticket's brief was to **rewrite the
normative `📖️component.grammar.semio` itself** from each artifact's real `🦀️component.rs`, then
re-transcribe into `🅰️component.g4`/`🔤️component.ebnf` so all three agree.

All 13 `.semio` files were rewritten. Of the 13, **8 got a real grammar** (their Rust genuinely
has a text parser) and **5 got an honest "no text codec exists" notice** (their Rust genuinely
does not — see below). All 39 files (13 × 3) were re-transcribed/rewritten; nothing else touched.

## The 13, what the old template claimed vs. what the code does

| # | grammar id | facet | old template (fictional) | real codec found |
|---|---|---|---|---|
| 1 | `raster.document` | snapshot | `layers{shape/path/text{canvas-field*}}` | hand-rolled hex/bracket `key=value` lines |
| 2 | `raster.op` | mutations | `add-layer`/`set-stroke`/`move-layer`/`set-fill` | 12 real verbs, generic `dsl::print`/`parse` engine |
| 3 | `raster.document.diff` | diff | `add-layer`/`remove-layer`/`move id= dx= dy=` | **no text codec exists** |
| 4 | `forms.forms` | snapshot | same stale canvas template | hand-rolled hex/bracket `key=value` lines |
| 5 | `forms.forms.op` | mutations | same stale canvas template | 10 real verbs, hand-rolled `keyword key=value...` |
| 6 | `forms.forms.diff` | diff | same stale canvas template | **no text codec exists** |
| 7 | `layout.document` | snapshot | same stale canvas template | hand-rolled `key=<hex-or-json-hex>` lines |
| 8 | `layout.diff` | diff | same stale canvas template | **no text codec exists** |
| 9 | `draw.document` | snapshot | same stale canvas template | real `dsl::DslRecord` derive, `RecordLayout::Lines` |
| 10 | `draw.op` | mutations | same stale canvas template | 14 real verbs, generic `dsl::print`/`parse` engine |
| 11 | `draw.diff` | diff | same stale canvas template | **no text codec exists** |
| 12 | `playbook.playbook` | snapshot | `layers-block{shape/path/text}` (sibling's own example) | hand-rolled hex/bracket `key=value` lines |
| 13 | `playbook.playbook.diff` | diff | same stale canvas template | **no text codec exists** |

## The 5 "no text codec exists" leaves — honest boundary, not invented grammar

For every `*.diff` facet in this set, the Rust `*Diff` struct derives only
`#[derive(... ArtifactSchema)]` — **never** `dsl::DslDiff`. The two derive expansions are easy to
tell apart by what they emit:

- `expand_dsl_diff` (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1419-1473`)
  generates a real `print_diff`/`parse_diff`/`encode_diff`/`decode_diff` (`DiffCodec`) impl.
- `expand_artifact_schema` (`🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs:201-260`)
  generates only `field_states()` — schema introspection, no serialization at all.

Confirmed for each of the 5 (`RasterDiff` `🧬️schema/🔺️diff/🦀️component.rs:9-46`; `FormsDiff`
`🧬️schema/🔺️diff/🦀️component.rs:17-19`; `LayoutDiff` `🧬️schema/🔺️diff/🦀️component.rs:9`; `DrawDiff`
`🧬️schema/🔺️diff/🦀️component.rs:15-17`; `PlaybookDiff` `🧬️schema/🔺️diff/🦀️component.rs:16-18`) —
none derive `DslDiff`, and none of their sibling `📝️text/🦀️component.rs` files (14-463 lines each)
contain a manual `impl store::ArtifactDsl`/`OpText`/`DiffText` either — only in-memory Rust builder
functions (e.g. raster's `diff_add_layer`/`diff_remove_layer`/…) and the semantic
`MutationDiff::apply`/`absorb` trait impl (programmatic merge logic, not text I/O).

Per the brief's honest-boundaries rule, each of these 5 `.semio` files now states this plainly
(with the file:line evidence above) instead of transcribing the old, equally-fictional
`add-layer`/`remove-layer`/`move id= dx= dy=` template — a real production
(`diff = unimplemented-notice` / `unimplemented-notice = "NO-TEXT-CODEC-EXISTS-FOR-THIS-FACET"`)
so the file stays syntactically well-formed rather than an empty/broken rule.

## The 8 real grammars — how they were derived

### Snapshot facets (4): `raster.document`, `forms.forms`, `layout.document`, `playbook.playbook`

All four are hand-rolled hex/bracket `key=value`-per-line codecs (`print_*_snapshot_body`/
`parse_*_snapshot_body` in each artifact's `📝️text/🦀️component.rs`), the same family as the
already-fixed `stdio.semio.cad` pair — real preamble `semio <plugin>.<artifact>.dsl v1`
(`store::semio_format::wrap_text`/`SemioEnvelope::preamble_line`), then N `key=value` lines.

- **raster.document** (5 fields: schema/id/title/layers/assets) — layers/assets use a recursive
  `p[...]`/`g[...]`/`a[...]` tagged-bracket layer codec (`enc_layer`/`dec_layer`,
  `🧬️schema/📸️snapshot/🦀️component.rs:182-259`). **Verified byte-for-byte against the real
  committed fixture** `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — every field name/shape
  matches exactly.
- **forms.forms** (6 fields: schema/id/version/title/structure/results) — `title`'s absent form
  is a bare `-` (not `[0]`/`[1,...]`), a distinct hand-rolled convention from raster's own
  `enc_opt_str`. No usable fixture: `BUILDING_COMPONENT_EXAMPLE_TEXT`/`DEFAULT_EXAMPLE_TEXT`/
  `ONBOARDING_EXAMPLE_TEXT` are explicitly documented (own doc comment,
  `🚪️io/📸️snapshot/📝️text/🦀️component.rs:138-141`) as the DIFFERENT "playbook kernel step/block
  DSL" used to author the `structure` child's own content — **not** `FormsSnapshot`'s own wire
  format — so cross-checking against them would have been checking the wrong grammar entirely.
- **layout.document** (14 fields) — every collection/nested field is JSON-serialized then
  hex-encoded (`enc_json`), not the `[0]`/`[1,...]` option-tag convention. **Verified byte-for-byte
  against the real committed fixture** `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — all 14
  field names/order match exactly.
- **playbook.playbook** (6 fields: schema/id/version/title/document/flow) — **verified
  byte-for-byte against the real committed fixture** (the sibling report already quoted this one).

### Mutation facets (3): `raster.op`, `forms.forms.op`, `draw.op`

- **forms.forms.op** — hand-rolled `keyword key1=value1 key2=value2...` (own doc comment,
  `🚪️io/🧬️mutations/📝️text/🦀️component.rs:3-4`), 10 verbs matching `FormMutation` exactly:
  create-step/delete-step/reorder-step/rename-step/change-step-description/create-block/
  delete-block/move-block-to-step/replace-block/change-form-title. **10/10 verbs match.**
- **raster.op**/**draw.op** — both go through the SHARED generic `dsl::parse`/`dsl::print` record
  engine (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`), not hand-rolled.
  raster.op: 12 verbs matching `RasterMutationDsl` exactly (create-layer/delete-layer/
  reorder-layers/rename-layer/change-layer-visible/change-layer-opacity/change-layer-blend-mode/
  move-layer/resize-layer/change-layer-adjustment-kind/add-layer-asset/remove-layer-asset).
  **12/12 verbs match.** draw.op: 14 verbs matching `DrawMutation` exactly (set-layer-visible/
  set-layer-locked/set-layer-opacity/set-layer-blend-mode/rename-layer/update-layer-transform/
  replace-layer-fill/replace-layer-stroke/set-layer-boolean-operation/update-layer-trace-params/
  create-layer/duplicate-layer/delete-layer/reorder-layer). **14/14 verbs match.**

### `draw.document` (snapshot, real `dsl::DslRecord`) — the one fixture caught real mistakes in

`DrawSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs:10-32`) is a genuine
`#[derive(dsl::DslRecord)] #[dsl(id="draw.draw", layout="lines")]` record, not a hand-rolled
codec — the real wire format comes from the shared engine's default field-printing rules, which
are NOT obvious from reading the struct alone. The real, committed emblem fixture
(`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 99 lines) exists and was read in full — cross-
checking `draw.document`'s first draft against it caught and fixed **6 real mistakes**, all now
corrected in both `draw.document` and its `draw.op` sibling (which reuses the same nested
`layer`/`fill`/`transform` shapes):

1. **Field-key casing**: the derive's default key for an unattributed field is the Rust field name
   **kebab-cased**, not camelCase — `blend-mode=`/`scale-x=`/`scale-y=`/`shape-kind=`/
   `image-key=`/`source-key=`/`simplify-epsilon=`, confirmed against the fixture's own
   `blend-mode=normal scale-x=1 scale-y=1`.
2. **Angle unit suffix**: `#[dsl(angle="rad")]` on `DrawTransform.rotation` appends a bare unit
   suffix directly onto the number with no separator (`rotation=0rad`), not a plain number.
3. **`attributes.fill` is ALWAYS a block**, even when no variant is set (`fill { }` empty),
   while `attributes.stroke` is genuinely omitted when absent — an asymmetry the fixture shows
   directly (both fields carry similar-looking `#[dsl(...)]` attrs) that would not have been
   guessed correctly from the struct definitions alone.
4. **Fixed-size arrays print bracket-less**: `[f64;2]`/`[f64;4]` (`point`, `color`) print as a
   bare comma-joined tuple (`M 1.25,196.933`, `color=0.98,0.584,0,1`), not `[x,y]`/`[r,g,b,a]`.
5. **Bare-vs-quoted strings**: a string prints unquoted when it lexes back as one bare identifier
   (`id=emblem-group`, `mime=image/png`), quoted+escaped only when it needs to be
   (`name="Semio Emblem"`) — `text` is now `bare-text / quoted-text`, not quoted-only.
6. **Top-level line structure**: `schema=`/`id=`/`title=` glue onto ONE line (space-separated),
   only the block-shaped fields (`layers`/`assets`/`artboard`) each start a new line — not one
   field per line as first drafted.

`FillStyle`'s gradient variants and `Vec<T>` list-shape bracketing (`point-list`/`number-list`/
`string-list`) are **not** fixture-verified (the emblem fixture has no polygon layer, dashed
stroke, or boolean layer) — inferred by extension of the same list-shape convention used
elsewhere in this ticket's other grammars, and flagged as such directly in the `.semio` file.

## Verb/production match summary

| leaf | match |
|---|---|
| raster.document | 5/5 snapshot fields match `RasterSnapshot`; fixture-verified byte-for-byte |
| raster.op | 12/12 verbs match `RasterMutationDsl` |
| raster.document.diff | N/A — no text codec (honest notice) |
| forms.forms | 6/6 snapshot fields match `FormsSnapshot` |
| forms.forms.op | 10/10 verbs match `FormMutation` |
| forms.forms.diff | N/A — no text codec (honest notice) |
| layout.document | 14/14 snapshot fields match `LayoutSnapshot`; fixture-verified byte-for-byte |
| layout.diff | N/A — no text codec (honest notice) |
| draw.document | 6/6 top-level fields + 7/7 layer kinds match `DrawSnapshot`/`DrawLayerNode`; fixture-verified byte-for-byte (after 6 fixes above) |
| draw.op | 14/14 verbs match `DrawMutation` |
| draw.diff | N/A — no text codec (honest notice) |
| playbook.playbook | 6/6 snapshot fields match `PlaybookSnapshot`; fixture-verified byte-for-byte |
| playbook.playbook.diff | N/A — no text codec (honest notice) |

## Transcription into `.g4`/`.ebnf`

Same conventions the sibling's 27-leaf pass established (verified by reading several of its
outputs first, plus the already-fixed `stdio.semio.cad` pair as the canonical precedent for a
`hex`-macro-based grammar): kebab-case → camelCase for `.g4` rule names, kebab-case → "space case"
for `.ebnf`; `(...)` native grouping in both; `(...)?`/`(...)*` in `.g4` map to EBNF-native
`[ ... ]`/`{ ... }` in `.ebnf`; quoted literals `"x"` → `'x'`; the pre-existing `grammar Name;` +
`DOCUMENT: 'schema' [ ]+ '<slug>'` (`.g4`) and `document = header, body ;` + `header = 'schema',
space, '<slug>', newline ;` (`.ebnf`) lines preserved byte-for-byte, unreferenced by the new
transcription below them (same "predates this transcription" convention the sibling used) — the
`grammar`/slug identity line itself was never touched, per the brief (a sibling agent already
corrected those).

Built a small scratch transcriber (`transcribe2_ueli.py`) that parses each rewritten `.semio`
body into a real alternation/sequence/group AST (not a blind regex replace), then renders both
mirrors from that one AST — the same source of truth feeds both files, so they cannot drift from
each other by construction. Bare framework terminals (`hex`, `SP`, `NL`, `OCTET`, `IDENT`, `TEXT`,
`INT`, `FLOAT`, `DQUOTE`) are given one concrete definition each at the bottom of both mirrors,
same "ticket-reported deviation" treatment the sibling's own 27-leaf pass used.

## Verification (actually run)

**1. Structural well-formedness** — scratch script `verify_ueli.py`: for all 13 leaves, (a)
balanced `()/{}/[]` in both `.g4`/`.ebnf` after stripping comments-then-string-literals (in that
order — an earlier pass stripped literals first and broke on the prose apostrophe in "facet's"
inside the preserved header comment, found and fixed); (b) the pre-existing `DOCUMENT:`/`header =`
line is still present byte-for-byte; (c) every `.semio` production name has a matching `.g4` rule
and `.ebnf` rule (name-set parity).

```
13/13 leaves pass structural verification (balanced + header preserved + rule-name parity)
```

**2. `git status` scope** — `git status --porcelain` over the exact 39 touched paths (13 leaves ×
`{.semio, .g4, .ebnf}`) → **39** lines, all `M` (modified, none added/deleted), matching exactly;
confirmed via a Python `subprocess.run(['git','status','--porcelain','--', *paths])` call (a plain
shell glob over the emoji paths silently produced zero args and a false-empty result — caught and
fixed before trusting the count). No other file touched.

**3. Runtime harness check — a REAL harness was found, but did not finish running.** Unlike the
`.g4`/`.ebnf` mirrors (confirmed, again, to have zero `include_str!` grammar-parsing consumers —
only lint-test string checks), `.semio` grammar files DO have a real consumer:
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` auto-discovers every
`🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` under `✏️s/🔌️plugins` and round-trips it
through `os_dsl::parse_grammar`/`Recognizer` against each artifact's own committed `.dsl.semio`
fixture (`m5_auto_discovery`/`m6_recognizer_round_trip`-style tests, `🦀️component.rs:1119-1420`).
I ran `cargo test -p semio-framework-os-kernel --features dsl-fixture-sweep-full --lib
os_dsl::fixture_sweep` myself (not merely citing the sibling's prior claim) — **it produced zero
output after 45+ minutes** and I did not wait longer past that. This is consistent with, and
independently reproduces the symptom of, the sibling report's finding that this crate/feature
combination does not currently build for reasons unrelated to this change (missing dev-dependencies
+ async-convention debt in `🧪️fixture-sweep` itself). **I cannot claim this harness ran clean
against my rewritten grammars** — I confirmed it exists and what it does, not that it passes.
Given this, the byte-for-byte fixture cross-checks I did by hand (§ above, for the 4 snapshot
grammars with a real committed fixture) are the strongest verification actually available this
session.

**4. Fixture opcode/field-name cross-check** — see the per-leaf table above; 4 of the 8 real
grammars (raster.document, layout.document, draw.document, playbook.playbook) have a real
committed `.dsl.semio` fixture and were checked against it byte-for-byte, catching 6 real defects
in `draw.document`'s first draft (all fixed). `forms.forms` has no applicable fixture (its only
`example.dsl.semio`-named constants belong to a different grammar entirely, evidenced above).
`raster.op`/`forms.forms.op`/`draw.op` have no committed op-text fixture (mutations are exercised
via in-Rust `demo_mutation_cases()`/round-trip tests, not committed text files) — verified
directly against the Rust enum/struct definitions instead.

## Anything unfinished

Nothing left undone within the 13-leaf scope: all 13 `.semio` files rewritten (8 real grammars, 5
honest no-parser notices), all 26 `.g4`/`.ebnf` siblings re-transcribed to match. The one honest
gap is #3 above — the real `fixture-sweep` recognizer test exists but did not finish running this
session, so its pass/fail status against these specific rewritten grammars is unconfirmed by
cargo, only by hand cross-check against the 4 available real fixtures.

## Files touched (39)

13 × `{📖️component.grammar.semio, 🅰️component.g4, 🔤️component.ebnf}` under:
- `✏️s/🔌️plugins/🖨️raster/…/🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/📝️text/`
- `✏️s/🔌️plugins/📋️forms/…/🚪️io/{📸️snapshot,🧬️mutations,🔺️diff}/📝️text/`
- `✏️s/🔌️plugins/📏️layout/…/🧬️schema/{📸️snapshot,🔺️diff}/📝️text/`
- `✏️s/🔌️plugins/🖍️draw/…/🚪️io/{📸️snapshot,🧬️mutations,🔺️diff}/📝️text/`
- `✏️s/🔌️plugins/📖️playbook/…/🧬️schema/{📸️snapshot,🔺️diff}/📝️text/`
