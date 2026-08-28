# 📓️ `semio@v1/✳️document` — carrier verdicts, oracle choice, per-kind witnessability

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document`
Capability: `semio-v1-document-mutate` · artifact `s.stdio.semio` · standard `v1` · subset `document` · 18 kinds.

---

## 1. Carrier verdicts — all three export serializers are REAL, read in full

Step 0 of `📓️pilot-playbook.md` asks whether the subset is reachable at all. It is: none of the three
serializers is one of the three disqualifying stub shapes (`print_dsl(..).into_bytes()`,
`encode_pack`→`decode_pack` type-confusion, or `serialize_text`-only). Each was read body-first.

### docx — REAL

`…/✳️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs`

It builds a real `DocxDocument` value; `DocxSnapshot::from_parts` hands the OPC container to docx's own
`engine::encode_docx`, so no codec is reimplemented here (`:71-75`):

```rust
    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let styles = from.styles.iter().map(|s| DocxStyle { id: s.id.clone(), name: s.name.clone(), based_on: s.based_on.clone() }).collect();
        let body = from.blocks.iter().flat_map(map_semio_block).collect();
        Ok(DocxSnapshot::from_parts(OpcPackage::default(), DocxDocument { body, styles }))
    }
```

The block mapping is `map_semio_block` (`:39-59`), and its LOSSES are the witnessability spec. They are
larger than the prior research pass recorded — this is a correction, not a restatement (`:46,:55,:54,:56,:57`):

```rust
        DocBlock::List { items, .. } => items.iter().flat_map(|item| item.blocks.iter().flat_map(map_semio_block)).collect(),
        DocBlock::Quote { blocks } => blocks.iter().flat_map(map_semio_block).collect(),
        DocBlock::Code { text, .. } => vec![DocxBlock::paragraph(text.clone())],
        DocBlock::Image { alt, .. } => vec![DocxBlock::paragraph(alt.clone())],
        DocBlock::PageBreak => Vec::new(),
```

* `List` and `Quote` **FLATTEN** — the file's own header says so (`:7-9`): "docx's block model only has
  `Paragraph`/`Table` — `List`/`Quote` FLATTEN". **`ordered` has no docx representation whatsoever**, so
  docx cannot witness `set-list-ordered`. The worklist's summary ("only `PageBreak` and inline
  colour/font/link dropped") understated this.
* `Code` keeps only its text (the `language` tag is gone), `Image` keeps only its `alt` (never fabricated
  image bytes), `PageBreak` drops entirely.
* Heading level survives only through the `HeadingN` style-id convention, and only when no explicit
  `style_id` was set (`:43`): `let style = style_id.clone().or_else(|| Some(format!("Heading{level}")));`
* Runs carry exactly three character properties (`:27`); `size`/`font`/`colour`/`link` are dropped (`:16`).
* Styles survive whole: `DocxStyle { id, name, based_on }`.

### md — REAL

`…/📝️md/🔖️commonmark/✳️any/🦀️component.rs:50-61` maps every `DocBlock` onto a real `MdBlock`:

```rust
        DocBlock::Heading { level, runs, .. } => vec![MdBlock::Heading { level: *level, inlines: runs_to_inlines(runs) }],
        DocBlock::List { ordered, items } => vec![MdBlock::List { ordered: *ordered, start: None, tight: true, items: … }],
        DocBlock::Code { language, text } => vec![MdBlock::CodeBlock { info: language.clone(), literal: text.clone() }],
        DocBlock::Image { image_id, alt, .. } => vec![MdBlock::Paragraph { inlines: vec![MdInline::Image { alt: alt.clone(), url: image_id.clone(), title: None }] }],
        DocBlock::PageBreak => Vec::new(),
```

Losses, from the file's own header (`:5-15`): **`styles`/`style_id` are dropped entirely** ("CommonMark
has no named-style concept"), `underline` is dropped, `Table` flattens to plain paragraphs, and image
BYTES/mime are dropped while `image_id` is reused verbatim as the emitted URL. `ordered`, heading `level`,
code `language`, blockquote nesting and inline bold/italic/link all survive.

### pdf — REAL

`…/📄️pdf/🔖️1.7/✳️any/🦀️component.rs:60-74` splits blocks into pages on `PageBreak` and flattens each
page to text:

```rust
            for block in &from.blocks {
                if matches!(block, DocBlock::PageBreak) { pages.push(make_page(&current)); current.clear(); }
                else { current.extend(block_to_lines(block)); }
            }
```

Header (`:9-12`): "ALL formatting/structure inside a page collapses to plain joined lines … `RunStyle`,
heading level, code language, list/table structure are all dropped — only visible text survives."

All three ship `#[cfg(test)]` round trips through the real downstream codec
(`maps_heading_paragraph_and_table`, `list_and_quote_flatten_image_and_pagebreak_drop`,
`maps_headings_lists_code_quotes_and_flattens_tables`, `splits_pages_on_pagebreak`).

---

## 2. Outcome classes — read from the code, not the doc comments

`…/✳️document/🧬️schema/🧬️mutations/🦀️component.rs:326-450`. Every arm of
`impl Mutation<SemioDocumentSnapshot> for SemioDocumentMutation::diff` wraps its result in
`protocol::MutationOutcome::new(..)`. **`MutationOutcome::error` and `::fatal` are never constructed
anywhere in this vocabulary**, so `rejected` is UNREACHABLE for all 18 kinds — do not declare it.

A mutation that cannot apply (path misses, value already equal) returns `SemioDocumentDiff::default()` —
an empty diff, i.e. a `no-op` in effect. The only place a no-op is signalled explicitly is the
`set-snapshot` leaf (`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs:6-11`):

```rust
    if base == snapshot {
        return protocol::MutationOutcome::new(SemioDocumentDiff::default()).warn("mutation.no-op", "set-snapshot: new snapshot is identical to the current one");
    }
```

Consequently:

| outcome | kinds |
| --- | --- |
| `applied` only | `insert-block`, `remove-block`, `insert-style`, `remove-style`, `insert-image`, `remove-image` (their diff arms are unconditional and always non-empty) |
| `applied` + `no-op` | `set-snapshot`, `set-block-content`, `set-paragraph-style`, `set-heading-level`, `set-list-ordered`, `set-run-text`, `set-run-style`, `set-image-block`, `set-style-name`, `set-style-based-on`, `set-image-bytes` |
| `no-op` only | `no-mutation` (`SemioDocumentMutation::NoMutation => SemioDocumentDiff::default()`) |
| `rejected` | **none — unreachable in this vocabulary** |

Structural note confirmed: production dispatch today exposes ONE generic entry point, `set-snapshot`
(`🧬️mutations/📄set-snapshot/`), with a single committed scenario
(`🧪️tests/bolds-the-body-paragraph-and-finalizes-its-copy/`). The other 17 names are enum variants of
`SemioDocumentMutation` and members of the `KINDS` roster (`🧬️mutations/🦀️component.rs:234-252`).

---

## 3. Oracle choice

### What was surveyed and declined, on the merits

* **A standalone Rust oracle crate** (`zip` 6 + `quick-xml` 0.42 + `comrak` 0.54 + `lopdf` 0.44 — all four
  already vetted `test-oracle` entries in `🔒️dependencies.json`, all four present in the local cargo
  registry so it builds `--offline`). Written, then withdrawn: the finished pilots run their oracle in
  TypeScript against a vendored library, and the Rust workspace is mid-refactor. Recorded here because it
  remains the obvious route if a Rust oracle is ever wanted for this subset.
* **Predicting the mutated result in our own code.** Out, and structurally so — the
  `reimplementation-registered-as-third-party` gate is now blocking. Neither registered oracle here
  computes what a mutation ought to produce: the libraries WRITE a before and an after document, and the
  probes READ files and report what the bytes encode. There is no expected-value computation anywhere in
  this registration.
* **`pdfjs-dist` as a third carrier oracle.** `pdfjs-dist@5.4.296` is vendored and is a genuine
  independent PDF reader. It is **not registered**, because no vendored library WRITES a PDF. The only
  ways to obtain a `third-party-generated` pdf fixture would be to hand-roll PDF bytes (the banned
  pattern in a different costume) or to drive Playwright's Chromium print path (non-deterministic, and
  not "the library wrote the format"). A reader with no library-authored artifact behind it is a coverage
  claim with nothing under it. Nothing is lost in coverage terms: pdf was the sole witness of exactly one
  thing — a `PageBreak` inserted by `insert-block` — and `insert-block` is witnessed by docx and md
  anyway. `pdfjs-dist` is also `production-runtime` in `🔒️dependencies.json`, so registering it would
  have required recording production debt as well.
* **`mdast-util-from-markdown` / `micromark` as the CommonMark reader.** Declined in favour of
  `markdown-it`: the first is the same unified family as the writer, so the reading would have confirmed
  its own serializer's family rather than checked it.

### What is registered

Two qualifying oracles (the WRITERS, which is what `generator.oracle` means) plus two probes (the
READERS). The split is deliberate and is what keeps the reading independent of the writing.

| role | id | packages | engine family |
| --- | --- | --- | --- |
| oracle (writer) | `jszip-xmldom-docx-carrier` | `jszip@3.10.1` (OPC container) + `@xmldom/xmldom@0.9.10` (WordprocessingML) | `jszip` |
| oracle (writer) | `mdast-to-markdown-md-carrier` | `mdast-util-to-markdown@2.1.2` | `unified-mdast` |
| probe (reader) | `semio-document-carrier-read` | `fflate@0.8.3` + `fast-xml-parser@5.11.1` + `markdown-it@14.3.0` | `fast-xml-parser` / `markdown-it` |
| probe (reader) | `semio-document-carrier-property` | same | same |

* **docx**: written by `jszip` + `@xmldom/xmldom`, read by `fflate` + `fast-xml-parser`. Fully disjoint
  package sets on the two sides — container and XML layer both.
* **md**: written by `mdast-util-to-markdown` (unified/mdast), read by `markdown-it` — an unrelated
  CommonMark implementation with its own parser. This is the reader/judge separation the mesh pilot gets
  from `three` versus `manifold-3d`.

**Cross-family invariant `document-text-agrees-across-every-carrier`**: on a flat document
`fast-xml-parser` and `markdown-it` must recover the same block-text sequence. They share no ancestry, so
their agreement is a real check rather than one library nodding at itself.

**One isolation fact recorded rather than hidden.** `markdown-it` has exactly one import site outside a
test-owned directory: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️artifact-support-leaf-authority/🟦️.test.ts`,
a framework library test that parses its own README prose. It is registered on the PROBE rather than on an
oracle, which is both where a measurement tool belongs and what keeps the `oracle-in-production` gate
truthful. `jszip` and `fast-xml-parser` are already used the same way by `s.stdio.bcf@2.1`'s probes and
generator; none of the six libraries is declared by any production `package.json`.

---

## 4. Per-kind witnessability — 15 witnessed / 3 uncarried

`✔` = the carrier ENCODES the property the mutation writes, so a change to it is visible in the bytes.

| # | kind | docx | md | verdict |
| --- | --- | --- | --- | --- |
| 1 | `no-mutation` | ✔ | ✔ | witnessed — both readers must recover an unchanged block-text sequence |
| 2 | `set-snapshot` | ✔ | ✔ | witnessed — whatever the carrier encodes of the replaced snapshot |
| 3 | `insert-block` | ✔ | ✔ | witnessed as a block count. A `PageBreak` insert is invisible in both (`PageBreak => Vec::new()` in each) |
| 4 | `remove-block` | ✔ | ✔ | witnessed as a block count |
| 5 | `set-block-content` | ✔ | ✔ | witnessed as block text |
| 6 | `set-paragraph-style` | ✔ | ✘ | docx only — `w:pStyle`; md drops `style_id` entirely |
| 7 | `set-heading-level` | ✔ | ✔ | md carries `level` directly; docx via the `HeadingN` style-id convention |
| 8 | `set-list-ordered` | ✘ | ✔ | **md only** — docx flattens lists, so `ordered` has no docx encoding at all |
| 9 | `set-run-text` | ✔ | ✔ | witnessed as run/inline text |
| 10 | `set-run-style` | ✔ | ✔ | docx witnesses `w:b`/`w:i`/`w:u`; md witnesses Strong/Emphasis/Link. Neither witnesses `size`/`font`/`colour` |
| 11 | `set-image-block` | ✔ | ✔ | md keeps a real image node with `alt` + url; docx keeps `alt` as ordinary paragraph text — a change is still visible, the fact that it IS an alt is not |
| 12 | `insert-style` | ✔ | ✘ | docx only — the named-style table |
| 13 | `remove-style` | ✔ | ✘ | docx only |
| 14 | `set-style-name` | ✔ | ✘ | docx only — `w:name` |
| 15 | `set-style-based-on` | ✔ | ✘ | docx only — `w:basedOn` |
| 16 | `insert-image` | ✘ | ✘ | **UNCARRIED** |
| 17 | `remove-image` | ✘ | ✘ | **UNCARRIED** |
| 18 | `set-image-bytes` | ✘ | ✘ | **UNCARRIED** |

**Why the three image-store kinds are uncarried, stated as a limit.** `SemioDocumentSnapshot::images` is an
id-keyed store of `DocImage { id, mime, bytes }`. The docx serializer writes no OPC media part at all —
its own header says "real image BYTES are never fabricated into a fake OPC media part" — and the md
serializer's header says "`DocBlock::Image::bytes`/`mime` are dropped — md images carry a URL, not raw
bytes". Neither export path ever emits the store, so a reader cannot witness a change to something the
bytes never carried. These three carry an `oracleRequirement` naming
`semio-v1-document-mutate-uncarried` with **no `oracle` field**, the `sequence@1/✳️any` convention
verbatim, so they report honestly as un-oracled rather than being absorbed into a capability-level pass.

MEASURED, so this is not an assertion: the probe answers `status: "unsupported"` — never an empty `ok` —
for every one of these.

```
$ bun …/✳️document/🔬️probes/📜️script.ts document-property --property image-store --input before.docx --input after.docx
  status=unsupported {"carrier":"docx","property":"image-store","reason":"docx does not encode image-store"}
$ … --property image-store --input before.md --input after.md
  status=unsupported {"carrier":"md","property":"image-store","reason":"md does not encode image-store"}
$ … --property list-ordered --input before.docx --input after.docx
  status=unsupported {"carrier":"docx","property":"list-ordered","reason":"docx does not encode list-ordered"}
$ … --property style-table --input before.md --input after.md
  status=unsupported {"carrier":"md","property":"style-table","reason":"md does not encode style-table"}
$ … --property paragraph-style --input before.md --input after.md
  status=unsupported {"carrier":"md","property":"paragraph-style","reason":"md does not encode paragraph-style"}
```

---

## 5. What was built

```
✳️document/🧪️oracle/🔣️.json          + 2 oracles, 2 probes, 1 comparison profile, 1 tolerance profile,
                                        1 mutationManifest (18 kinds), 24 fixtureManifests
✳️document/🔬️probes/📜️script.ts      4 reading probes; computes nothing, predicts nothing
✳️document/🏭️generator/📜️script.ts   + 🧪️document / 🧪️blocks / 🧪️runs / 🧪️styles family modules
✳️document/🧫️fixtures/<recipe>/…      15 recipes → 50 files → 24 per-carrier fixture bundles
```

The pre-existing `semio-document-python-independent` entry (`kind: cross-semio-implementation`) is left
untouched. It remains a required SUPPLEMENTAL oracle; the two new entries are what discharge the
`third-party-library` requirement for the 15 witnessed kinds.

24 fixtures rather than 15 because a fixture's authority is the SINGLE library that wrote its bytes: a
recipe with two carriers yields one docx bundle attributed to `jszip-xmldom-docx-carrier` and one md
bundle attributed to `mdast-to-markdown-md-carrier`. 14 docx + 10 md = 24.

---

## 6. The gate — validated BOTH ways, with real numbers

The gate recipe is `set-run-text-rewrites-the-body-copy`, which commits a deliberately wrong after
alongside the right one: a single character changed, `revised` → `rev1sed`. Close enough that only a
reader that genuinely decoded the text can tell them apart.

**ACCEPT (known-good):**

```
carrier-agreement  after.docx vs after.md
  → totalDisagreements 0, allEqual true
    docx: ["The Report Title","The revised body paragraph.","The closing paragraph."]
    md:   ["The Report Title","The revised body paragraph.","The closing paragraph."]
document-property --property block-text  after.docx vs after.docx
  → differingEntries 0, equal true
```

**REJECT (known-bad):**

```
carrier-agreement  counterexample.docx vs after.md
  → totalDisagreements 1, differingIndices [1], allEqual false
    docx: ["The Report Title","The rev1sed body paragraph.","The closing paragraph."]
document-property --property block-text  after.docx vs counterexample.docx
  → differingEntries 1, differingIndices [1], equal false
document-property --property block-text  after.md   vs counterexample.md
  → differingEntries 1, differingIndices [1], equal false
```

Separation is 0 against 1 on every one of the three measurements, in both engine families independently.

**Witness sweep — every fixture is witnessed by its own carrier**, run over all 24 bundles with the
recipe's own property (`no-mutation` inverted, since its correct reading is *no* change):

```
[witness sweep] 24/24 fixtures witnessed by their own carrier
```

with, for example, `set-snapshot` at `differingEntries=2` (two blocks rewritten, the code block
untouched), `no-mutation` at `differingEntries=0`, and every other recipe at `differingEntries=1`.

---

## 7. Harness runs — real output

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture reproduce --subset document
[fixture reproduce] 24 generated fixture(s), 0 problem(s)

$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture verify --subset document
[fixture verify] 24 fixture(s), 0 file problem(s)
```

`fixture reproduce` re-runs each fixture's own recorded `generator.command` into a scratch directory and
compares digests against the committed manifest; it never writes into the committed bundle. 24/24 pass.
An independent double-generation check agrees: 50/50 fixture files byte-identical on a second run
(`@xmldom/xmldom` and `mdast-util-to-markdown` write no timestamp, and JSZip is pinned to a fixed
1980-01-01 stamp instead of `new Date()` — without that pin every regeneration would differ).

```
$ bun …/🧪️test/📜️script.ts matrix --subset document          # repo-wide denominators, document rows filtered
[matrix] fixtureReproducibilityCoverage   100.00%  348/348
[matrix] fixtureProvenanceCoverage         99.71%  347/348
[matrix] dependencyIsolationCoverage      100.00%  190/190
[matrix] fixtureClassCoverage             100.00%  3/3
[report] Which fixtures are not reproducible?  none
29 coverage rows for subset `document`, 0 with a fixture-provenance problem.
```

The matrix attributes every capability-covered row to the first oracle registering that capability, so all
15 witnessed kinds show `jszip-xmldom-docx-carrier` in the row dump even where the manifest names only the
md oracle (`set-list-ordered`). That is the framework's row-building, not the manifest — the manifest's
own per-kind requirements are the table in §4.

### `contract --subset document` — breach accounting

`contract` is repo-wide; the honest way to read it is the delta attributable to this subset.

**Fixed by this work (was a real breach before):**

```
testing/contract | capability-without-manifest |
  Catalog semio-v1-document declares capability semio-v1-document-mutate (18 kind(s)) and no mutation manifest owns it
```

Gone: the capability now has a manifest and contributes rows to the release gate instead of being
invisible to it.

**Attributable to `✳️document` afterwards — 8, and every one is either pre-existing, universal, or the
intended honest report:**

| n | breach | reading |
| --- | --- | --- |
| 3 | `testing/oracle / missing-external-oracle` on `semio-v1-document-mutate-uncarried` for `insert-image`, `remove-image`, `set-image-bytes` | **INTENDED.** This is exactly what the uncarried convention is for, and `sequence@1/✳️any` produces the same four. The alternative — pointing them at a real oracle id — would be a green result standing on evidence that does not exist. |
| 3 | `testing/contract / duplicate-mutation-owner` for `no-mutation`, `set-snapshot`, `remove-block` against `✳️cad` | **A FRAMEWORK KEY COLLISION, reported not worked around.** The ownership key is `artifact@standard::mutation` with no subset segment, so two subsets of `s.stdio.semio@v1` that both have a `no-mutation` kind collide by name. This is pre-existing in kind: `s.stdio.semio@v1::move-vertex` already collides between `✳️mesh` and `✳️brep` for the same reason. Dropping the three names from this manifest would silence it by making the subset's own vocabulary incomplete, which is worse. |
| 1 | `testing/contract / runtime-inventory-missing` | **UNIVERSAL.** All 20 manifested subsets carry it, `✳️mesh`, `✳️brep` and `step@ap214/cc6` included — it clears only when `test inventory` runs an owner's production `🏭️bridge`, and `semio-s-plugin-stdio` does not currently compile (a peer's in-flight refactor, untouched here). |
| 1 | `testing/contract / binary-protocol-drift` | **PRE-EXISTING**, present in the baseline before this work and unrelated to it (one mutation kind has no wire record in `🧬️mutations/💾️binary/📡️component.protocol.semio`). |

Zero new `testing/fixture` breaches: the 24 bundles resolve their `../🧫️fixtures/<recipe>/<file>` paths,
match their digests, name a registered generating oracle of a qualifying kind, and name the
`semantic-document-carrier-v1` / `document-text-exact` profiles this contribution defines. Zero
`oracle-in-production`, zero `reimplementation-registered-as-third-party`.

---

## 8. What is still owed

* **`insert-image`, `remove-image`, `set-image-bytes`** — no exported carrier encodes the image store.
  Closing them needs either a docx serializer that emits real OPC media parts, or a carrier that does.
* **A production `🏭️bridge`** for this subset, so `test inventory` can prove manifest = runtime. Blocked
  on `semio-s-plugin-stdio` compiling.
* **pdf** — a vendored PDF *writer* would make `pdfjs-dist` a third reader oracle and would restore the
  one thing pdf alone can witness, a `PageBreak`.
* **`Table`** is exercised by no recipe. Both carriers encode it (docx as `w:tbl`, md by flattening), so
  it is reachable; it was left out to keep the first corpus flat enough that the cross-family agreement
  check is meaningful.
