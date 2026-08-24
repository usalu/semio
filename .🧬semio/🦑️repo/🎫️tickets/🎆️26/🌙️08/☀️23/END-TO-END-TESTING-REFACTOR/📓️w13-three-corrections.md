# Wave 13 — the three corrections the w11 audit named

Date 2026-08-24. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Successor to `📓️w11-verification.md`
§4 (42 vacuous scenarios), §5 (undocumented identical AP214 catalogs) and §6 (templated prose).
Every command quoted below was actually run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`
unless noted, and exit codes were read from the tool's own status, never through a pipe.

---

## 1. The 42 scenarios that asserted nothing — closed

All four dispatched cases now assert every law they claim, through the shared
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module rather than a hand-rolled local comparison, and each
subset oracle module gained the `every_declared_kind_is_observable…` test its conformance-class
siblings already had.

| case | before | now |
|---|---|---|
| `mutate-pdf-1-7` | 18 `mutate-<kind>` vacuous | 37/37 asserting, `executed=37 passed=37 failed=0` |
| `mutate-docx-ecma-376` | 13 vacuous | `executed=27 passed=27 failed=0` |
| `mutate-pptx-ecma-376` | 9 vacuous | `executed=19 passed=19 failed=0` |
| `mutate-pdf-1-4` | 2 vacuous | `executed=5 passed=5 failed=0` |

### The real work was in `📄️pdf 1.7 ✳️any`

Turning observability on exposed that `document::project_pdf`'s page-and-metadata surface cannot see
EIGHT of the eighteen declared kinds — the seven COS object-graph kinds plus `set-page-crop-box`.
Declaring eight kinds unobservable would have been shrinking the law to fit the projection, so the
projection was widened instead (the `🧊️obj` precedent). `project_pdf_1_7` now also reports:

* each page's `/CropBox`;
* an `objectGraph` member: the trailer minus `/Size`/`/Prev`/`/XRefStm` (bookkeeping `lopdf`
  recomputes on every save), and the catalog resolved three references deep with `/Pages` omitted
  (already projected in full by `pageCount`/`pages`). Object NUMBERS never appear — `semantic-pdf-v1`
  calls them writer freedom, and a resolved value is what a conforming reader sees (ISO 32000-1
  §7.3.10). Cycle-guarded, because `/Parent` back-pointers make the graph cyclic.

Seven of the eight became observable with no exemption: `set-dict-entry` moves `/PageMode`,
`remove-dict-entry` drops `/Outlines`, `remove-object` #3015 makes `/Outlines` resolve to `null`,
`set-object-value` #145 rewrites the `/OpenAction` the catalog resolves to, both trailer kinds move
trailer keys, and `set-page-crop-box` moves the new page field.

**One kind stays unobservable, and it is the vocabulary's fault, not the projection's.**
`insert-object` adds an indirect object and links it to nothing; ISO 32000-1 §7.5.4 makes an
unreferenced object unreachable. Measured on the real thesis with a throwaway `lopdf` probe
(`w13-pdf-object-probe/`): **3,173 objects, 3,173 references, ZERO orphans, ZERO dangling
references** — there is no id at which an insertion could land somewhere already pointed at.
`InsertObject` carries no reference site and only `SetDictEntry` can create one. Named as
`UNOBSERVABLE` in the subset's oracle module, argued in the feature, and pinned by
`insert_object_is_unobservable_only_because_nothing_can_reference_the_new_object`, which flips red
the moment the vocabulary or the fixture changes. Its INVERSE stays under the full law.

### The four failures the brief predicted

* `pdf-1-7` `inverse-append-page-content` / `inverse-remove-page` / `inverse-set-page-content` on
  `pages.N.contentOperators` — already triaged in w11 and still disposed of the same way: the axis is
  dropped for those three kinds only, on both sides, because `PdfPage`'s only content field is
  `text`. That carve-out now lives in the SUBSET's oracle module (`regenerates_page_content`,
  `without_content_operators`) so the case adapter and the module-level law test cannot exempt
  different things. `AppendPageContent`'s missing counterpart is the same gap and is named as such;
  the fix is widening `PdfPage` to retain a real content stream, which is a production-snapshot
  change (rust/ts/graphql/proto/json + io + diff + editor) belonging to that snapshot's owner and
  well outside these three corrections.
* `docx` `inverse-remove-style` on `styles.1.id` — already fixed in w11 and now pinned at unit level
  by `removing_an_interior_style_is_refused_because_append_cannot_put_it_back`: removing an INTERIOR
  style is refused outright (`InsertStyle` appends), removing the LAST one round-trips.

No assertion was weakened and no fixture was swapped to dodge anything.

## 2. `step-ap214-cc2/cc3/cc4/cc5` — documented and machine-checked

The four catalogs are identical **by ISO 10303-214 §4.3**, not by copying. §4.3 defines the six
classes as one monotone ladder over a single capability and varies nothing else, so a vocabulary
derived one kind per axis depends only on WHERE the ceiling sits, and there are three places:

* strictly inside the ladder (rungs 2–5) — a ceiling type exists to write AND a rung exists above to
  demote from, so both ladder verbs are declared: six kinds, identical for cc2–cc5;
* below it (cc1, rung 1) — `ceiling_type_of(1)` is `None`, nothing to write and nothing to demote
  onto, so `remove-shape-representation` replaces both verbs: five kinds;
* on top of it (cc6, rung 6) — nothing can be above the top rung, so no demotion has a subject: five
  kinds.

Written out once in the shared `📐️step/🏅️standards/🔖️ap214/🧪️oracle/🦀️component.rs` header (the named
module all seven subsets already share `part21`/`ladder` through), cited from each of the four
subsets' oracle headers, their `🔣️component.json` `_comment`s and their feature descriptions, and
asserted by
`the_four_interior_classes_share_one_vocabulary_because_their_ceilings_share_one_place`, which fails
if an interior class's list drifts or an edge class grows a verb its ceiling cannot support.

## 3. Templated prose — the six OOXML cases rewritten

Measured before and after over the six OOXML conformance features (sentences >70 chars appearing in
3 or more of the six): **was heavy, is now ZERO**. Every case now names its own real input, its own
distinguishing behaviour and its own evidence limits, for example:

* the DOCX/PPTX/XLSX ✳️strict cases start OUTSIDE their class (the committed packages are
  Transitional) and their ✳️transitional mirrors start INSIDE it — the same real bytes driven across
  the boundary in both directions;
* PPTX names its DrawingML axis (two namespace families, a kind no DOCX or XLSX subset has); XLSX
  names its per-worksheet `[Content_Types].xml` Override axis and states plainly that this case IS
  differential where `mutate-xlsx-ecma-376` is not (`calamine` reads, `rust_xlsxwriter` writes);
* the arranged pre-states are counted per case (3 of 10, 1 of 6, 3 of 11, 1 of 7, 2 of 9, 1 of 7)
  instead of one shared paragraph;
* each says what it CANNOT witness — document content, deck geometry, cell content, media parts.

The 14 sentences still shared are shared by exactly TWO files each, always the strict/transitional
pair of one artifact stating the same real fact about the same real fixture.

The six adapters' replicated law comparisons were replaced by calls into the shared `law` module
(shared helper code is the right kind of sharing), their duplicated `KINDS` mirrors now import the
one list their subset's oracle module declares, and every doc comment is per-subset.

## 4. Verification — real output

```
cargo test --features oracles --lib   (✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust)   exit 0
test result: ok. 367 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 92.48s
```

`oracle exhaustive --owner 🗄️stdio --case …`, every one exit 0:

```
mutate-pdf-1-7                      executed=37 passed=37 failed=0 errored=0 parity=0/0
mutate-pdf-1-4                      executed=5  passed=5  failed=0 errored=0 parity=0/0
mutate-docx-ecma-376                executed=27 passed=27 failed=0 errored=0 parity=0/0
mutate-pptx-ecma-376                executed=19 passed=19 failed=0 errored=0 parity=0/0
mutate-docx-ecma-376-strict         executed=21 passed=21 failed=0 errored=0 parity=0/0
mutate-docx-ecma-376-transitional   executed=13 passed=13 failed=0 errored=0 parity=0/0
mutate-pptx-ecma-376-strict         executed=23 passed=23 failed=0 errored=0 parity=0/0
mutate-pptx-ecma-376-transitional   executed=15 passed=15 failed=0 errored=0 parity=0/0
mutate-xlsx-ecma-376-strict         executed=19 passed=19 failed=0 errored=0 parity=0/0
mutate-xlsx-ecma-376-transitional   executed=15 passed=15 failed=0 errored=0 parity=0/0
mutate-step-ap214-cc2 / cc3 / cc4 / cc5   executed=13 passed=13 failed=0 errored=0 parity=0/0 each
```

`parity=0/0` is unchanged and expected: the Rust SUBJECT phase still cannot compile.

## 5. ⚠️ `contract` is RED, and it is not this work

`bun ./📜️script.ts contract --owner 🗄️stdio --case <any>` exits **1** for every case, always with the
same output:

```
70 high-priority breach(es) across 1 rule(s):
     70  testing/contract
```

All 70 come from ONE rule, *"A mutation vocabulary is declared here but no catalog registers it"*,
which the `--case` filter does not scope. The rule landed in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts` at commit
`dced3e93` **2026-08-24 15:00:40**, i.e. after the w11 audit and from another session. NONE of the 70
names a case touched here. Exactly **2 of the 70 are in 🗄️stdio**, and they are precisely the gap
w11 §0.2 already recorded:

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations
```

Both have a handcrafted 9-kind vocabulary and NO `🧪️oracle` directory, no manifest, no catalog and no
case. Closing them is a full end-to-end subset build each (oracle module + registration + catalog +
feature + adapter) and is a wave of its own, not a correction to these three. It is now a *blocking*
gate failure rather than an invisible gap, which is an improvement in itself.

## 6. Shared-module additions, declared as the brief requires

* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` gained `feature_rows(feature) -> Vec<(String,
  Json)>`, which reads a case's own `Examples` table out of the feature text. Genuinely shared by two
  subsets: `📜️docx ecma-376/✳️any` and `🎞️pptx ecma-376/✳️any` both carry `Examples` payloads (a whole
  replacement block tree, a whole replacement deck) too large to restate by hand, and both prove the
  same two laws over them. Reading the table rather than transcribing it is what makes the unit-level
  law test and the case provably the same rows. Covered by its own unit test. No existing function in
  any shared family module was changed.
