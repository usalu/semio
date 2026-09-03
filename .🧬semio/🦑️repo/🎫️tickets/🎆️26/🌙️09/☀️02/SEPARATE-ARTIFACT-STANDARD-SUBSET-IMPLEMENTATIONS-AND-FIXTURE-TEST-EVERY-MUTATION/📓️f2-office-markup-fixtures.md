# F2 — Office/Markup Fixtures: All 115 `mutation-without-fixture` Breaches Closed

Shard F2. Territory: `mutation-without-fixture` across `🎞️pptx`, `📕️xlsx`, `🎨️svg`, `📜️docx`,
`🎒️zip`, `📷️jpg`, `🌐️html`, `📰️xml` under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`.

## Headline

The brief's counts (23/22/19/14/12/9/8/8) summed to **115**, not 107 — confirmed by re-reading
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` before touching anything (`🔎` below). Closed **all
115**, zero remainder, zero itemised debt.

| artifact | subsets | before | after |
| --- | --- | --- | --- |
| `🎞️pptx` | base, strict, transitional | 23 | **0** |
| `📕️xlsx` | base, strict, transitional | 22 | **0** |
| `🎨️svg` | basic, tiny | 19 | **0** |
| `📜️docx` | strict, transitional | 14 | **0** |
| `🎒️zip` | base, iso21320 | 12 | **0** |
| `📷️jpg` | baseline | 9 | **0** |
| `🌐️html` | any | 8 | **0** |
| `📰️xml` | valid | 8 | **0** |
| **total (my territory)** | | **115** | **0** |

Repo-wide `mutation-without-fixture`: 361 (ticket baseline) → 247 (after xml/html/svg/zip/jpg, mid-run
re-check) → **84** (final, other shards' concurrent work included). No other breach id moved in a way
attributable to me. Confirmed by two live `bun ./📜️script.ts test contract` runs in the FOREGROUND
(one mid-way after closing 56 of 115, one final after all 115), both read back fresh from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`.

| id | before | after |
| --- | --- | --- |
| `mutation-without-fixture` (mine) | 115 | **0** |
| `missing-fixture` | 0 | 0 |
| `orphan-fixture` | 0 | 0 |
| `fixture-digest-mismatch` | 0 | 0 |
| `fixture-manifest-invalid` | 0 | 0 |
| `fixture-generated-by-non-qualifying-oracle` | 0 | 0 |
| `fixture-generator-unregistered` | 0 | 0 |
| `oracle-in-production` | 314 | **314 (not risen)** |
| repo-wide TOTAL | 1186 (baseline) | 908 (final; other shards' concurrent work) |

`✏️s/🎫️tickets/…/🔍️check-mutation-leaf-ownership.py` re-run after all edits: zero findings anywhere
under my 8 artifacts (all findings in the output belong to `procedural/generation3d` and
`architect/program`, neither touched by me). I never moved, renamed or created a mutation directory —
every edit is a `fixtureManifests[]` array plus (for zip) two new sibling oracle entries, all inside
each subset's own `🧪️oracle/🔣️.json`, plus new files under each subset's own `🧫️fixtures/`.

## Method

For every mutation I built a genuine before/after pair, then registered it as a `fixtureManifests[]`
entry (schema `semio.repository-test.fixture/v2`) in the owning subset's `🧪️oracle/🔣️.json`, matching
the shape of the PNG exemplar (`png@1.2/✳️any`'s `change-header-applied` etc.) — `target`, `mutation`,
`files[]` with real `sha256`, `provenance`, `comparisonProfile`, and (for `third-party-generated`
fixtures) a `generator` block naming an already-registered qualifying oracle.
`mutationFixtureBreaches` (the rule's own implementation, `🟦️.ts:5398`) accepts exactly this form — a
v2 fixture whose `target` names the mutation's own artifact/standard/subset and whose `mutation` field
names the mutation's own id — which is the PNG exemplar's own registered form, so I matched that
directly rather than also duplicating the exemplar's separate `🧬️mutations/<id>/🧪️tests/<case>/`
physical-vector scaffolding (a different, already-passing mechanism `mutationVectorRegistryBreaches`
owns, not required by this specific breach and out of scope for 115 mutations under this ticket's time
budget).

**Comparison profile**: every fixture cites a `CORE_COMPARISON_PROFILES` entry (`utf8-text-v1` for
text formats, `exact-bytes-v1` for binary/zip-based ones) rather than a new custom profile — avoids
`fixture-comparison-profile-unknown` without inventing profile machinery this ticket doesn't ask for.

**Where the reference reached the mutation**, the fixture is `class: "third-party-generated"` and its
bytes are the library's own writer output, never hand-typed:

| artifact | reference used | how |
| --- | --- | --- |
| `xml`/`svg` | lxml 6.1.3 | `etree.parse` -> `etree.tostring` round-trip; SVG has no DOCTYPE so lxml carries every one of its 19 mutations |
| `html` | html5lib 1.1 | `html5lib.parse(treebuilder="dom")` -> `HTMLSerializer` (the `dom` builder, not `etree`, because html5lib's `etree` tree silently drops the DOCTYPE node — verified live) |
| `zip` | **yazl 2.5.1** (write) + yauzl 3.4.0 (read-verify) | yazl builds the archive, yauzl re-reads every entry/comment back before commit (both real, both run) |
| `jpg` | Pillow 12.2.0 | `Image.save(subsampling=, progressive=)` for sampling/SOF-marker/component-count kinds; component-count change (L vs RGB) is also the genuine cause of DHT-table-count change, so it doubles as `insert-huffman-table`/`remove-huffman-table` evidence |
| `pptx` (base) | python-pptx 1.0.2 | `Presentation`/`slides`/`shapes`/`text_frame` object model + the slide-id list for reordering |
| `xlsx` (base) | openpyxl 3.1.5 | `Workbook`/`Worksheet` object model |
| `docx`/`pptx`/`xlsx` `set-snapshot` (all subsets) | the same three libraries | a whole different real document, both sides genuine library output |

**yazl was newly registered** (`🩹️f2-register-yazl-oracle.py`) as
`yazl-zip-2-0-{base,iso21320}-mutate-writer`, `third-party-library`, `productionReachable: false` — the
already-registered `yauzl` reader has no writer API at all, so crediting it as the *generator* of a
`third-party-generated` fixture would have been a misattribution; this mirrors the reader/writer split
E2 already used for `wav` (`hound`+`riff`).

### Handcrafted, and exactly why (per artifact)

- **`xml`/`declare-entity`, `xml`/`set-internal-subset`**: verified live that lxml's own
  `docinfo.doctype` drops internal-DTD-subset content on serialization — round-tripping
  `<!DOCTYPE root [<!ENTITY foo "bar">]>` through `etree.parse`+`tostring` yields bare
  `<!DOCTYPE root>`, the entity silently gone. No lxml path reaches these two.
- **`jpg`/`set-sample-precision`, `jpg`/`set-arithmetic`**: Pillow's encoder always writes baseline
  8-bit Huffman JPEG and exposes no parameter for precision or entropy-coding method (matches E2's own
  registry rationale for this reader). Both are a single marker byte, hand-patched on top of a genuine
  Pillow file (precision byte in SOF0; SOF0→SOF9 marker code) — documented as NOT a spec-conformant
  12-bit/arithmetic bitstream, only a structural marker-vocabulary vector.
- **`xlsx`/`insert-shared-string`, `remove-shared-string`, `set-shared-string`**: grepped openpyxl
  3.1.5's own `writer/excel.py` — no `sharedStrings` reference exists anywhere; its writer always emits
  inline strings (`t="inlineStr"`), never an `xl/sharedStrings.xml` part. Each fixture starts from a
  genuine openpyxl package, is zip-patched to use the real OOXML shared-strings mechanism, and is READ
  BACK AND CONFIRMED with `openpyxl.load_workbook` before commit (live output captured in the
  generator script, e.g. `set-shared-string: before A1='Hello' after A1='World'`).
- **`docx`/`pptx`/`xlsx` strict+transitional structural mutations** (conformance attribute, main
  namespace, relationship(s) namespace, drawing namespace, VML part presence, `mc:AlternateContent`
  presence, worksheet content-type) — 8 kinds across 44 fixtures. None of python-docx / python-pptx /
  openpyxl's public object models expose a `conformance` attribute, a part's namespace URI, or
  VML/AlternateContent editing (verified by inspecting each library's real object surface). Every one
  is a zip/XML patch on top of a genuine library-written base package: namespace pairs are the real
  ISO/IEC 29500 Part 1 Transitional↔Strict URIs (`schemas.openxmlformats.org/...` ↔
  `purl.oclc.org/ooxml/...`, not fabricated); every patched main-part XML is re-parsed with lxml for
  well-formedness and every resulting archive's zip integrity is re-checked, both live, both asserted
  in code (`🔨️f2_ooxml_common.py`), not merely claimed.

## Registrations

New `fixtureManifests[]` entries written directly into each subset's own `🧪️oracle/🔣️.json` (no other
existing field touched — no oracle kind reclassified, no mutation manifest edited, no existing fixture
moved):

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✳️valid/🧪️oracle/🔣️.json        (8)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧪️oracle/🔣️.json           (8)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/{✳️basic,✳️tiny}/🧪️oracle/🔣️.json  (10+9)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/{✳️base,✳️iso21320}/🧪️oracle/🔣️.json (5+7, plus 1 new yazl oracle entry each)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json  (9)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/{✳️base,✳️strict,✳️transitional}/🧪️oracle/🔣️.json (7+10+6)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/{✳️base,✳️strict,✳️transitional}/🧪️oracle/🔣️.json (8+8+6)
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/{✳️strict,✳️transitional}/🧪️oracle/🔣️.json (9+5)
```

Every fixture's real files (before/after pairs) were written under the matching subset's own
`🧫️fixtures/<mutation>-applied/{before,after}.<ext>`, exactly the PNG exemplar's own naming.

## Generator scripts (kept, idempotent, all in this ticket folder)

- `🔨️f2-gen-xml-fixtures.py`, `🔨️f2-gen-html-fixtures.py`, `🔨️f2-gen-svg-fixtures.py` (Python/lxml/html5lib)
- `🔨️f2-gen-zip-fixtures.cjs` + `🩹️f2-register-yazl-oracle.py` (Node/yazl+yauzl)
- `🔨️f2-gen-jpg-fixtures.py` (Python/Pillow)
- `🔨️f2-gen-pptx-base-fixtures.py`, `🔨️f2-gen-xlsx-base-fixtures.py` (content-level, real object model)
- `🔨️f2_ooxml_common.py` (shared zip/XML patch helpers) + `🔨️f2-gen-docx-structural-fixtures.py`,
  `🔨️f2-gen-pptx-structural-fixtures.py`, `🔨️f2-gen-xlsx-structural-fixtures.py` (strict/transitional)

All are re-runnable (they overwrite their own prior `fixtureManifests` array and fixture files).

## Final answer

- **Closed all 115 of the 115 `mutation-without-fixture` breaches** the brief's counts named (the
  brief's stated "107" undercounted its own listed per-artifact numbers, which sum to 115 — confirmed
  against the live breach cache before starting).
- **All 8 artifacts complete**: `🎞️pptx`, `📕️xlsx`, `🎨️svg`, `📜️docx`, `🎒️zip`, `📷️jpg`, `🌐️html`,
  `📰️xml` — zero remainder, nothing itemised as left open.
- **Before/after**: `mutation-without-fixture` in my territory 115 → 0; `missing-fixture`,
  `orphan-fixture`, `fixture-digest-mismatch`, `fixture-manifest-invalid`,
  `fixture-generated-by-non-qualifying-oracle`, `fixture-generator-unregistered` all held at 0 (no new
  provenance defects introduced); `oracle-in-production` 314 → 314, **not risen**. Repo-wide
  `mutation-without-fixture` 361 → 84 over the session (the remainder is other shards' concurrent
  territory, untouched by me).
- One new oracle package registered: `yazl` 2.5.1 (MIT), as a writer sibling to the already-registered
  `yauzl` reader, in both zip subsets — `testOnly: true`, `productionReachable: false`.
- This file: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️f2-office-markup-fixtures.md`.
