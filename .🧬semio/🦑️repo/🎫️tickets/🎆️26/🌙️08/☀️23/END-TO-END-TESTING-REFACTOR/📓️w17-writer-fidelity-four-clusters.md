# w17 — the four writer-fidelity clusters of the first full differential run

Successor note to `📓️w13-final-audit.md` §2.2. Scope: the four clusters that audit attributed to
this repository's own code — XLSX transitional relationship loss (15 divergences), DOCX transitional
conformance attribute (5), Markdown `htmlBlock` (1), TIFF `insert-ifd` tag 278 (1).

Working logs: `w17-writer-fidelity/`. **Read §3 first: the parity ratios were NOT obtainable in this
session** — every cargo invocation on this machine was serialised behind other sessions' builds — so
no after-number is claimed anywhere in this note. What was verified is stated there, exactly.

**Nothing was weakened.** No `ignoreKeys` entry, no widened tolerance, no changed `arrays` mode, no
swapped or normalised fixture, no deleted scenario, no relaxed law. Every change is in a writer, a
parser test, or a schema. Verify with
`git diff -- '*🔣️component.json'` (the only profile touched anywhere in this tree is another
session's `✳️image` oracle registration, which ADDS a reference implementation where there was none)
and `git diff -- '*component.feature'` (the only feature line changed by this work is the TIFF
KNOWN-OPEN-DIVERGENCE paragraph, rewritten to record that the schema change it prescribed was
actually carried out — no scenario, parameter or `Examples` row moved).

---

## 0. The headline

Three of the four clusters were real defects in our writers and are fixed at the cause. The fourth
was **misattributed**: `md-commonmark`'s divergence is the reference library injecting content, and
the raw bytes of both sides prove it (§4). Two further defects of the same family, which no scenario
was measuring, were found and fixed while fixing the mandated ones (§1.3, §5.2).

---

## 1. XLSX + DOCX + PPTX — one defect, three formats

`XlsxSnapshot`, `DocxSnapshot` and `PptxSnapshot` are each `opc` (every part verbatim, plus the two
typed metadata channels) **plus** a typed semantic VIEW (`workbook` / `document` / `presentation`).
The view is a projection of part of the package, never the whole of it. All three writers treated
the view as the authority over an entire part or an entire relationship list, and so destroyed, on
every single write, everything the view does not model.

`pptx` was already half-immune by accident: `encode_pptx` regenerates only when the typed
presentation actually differs from the authoritative XML parts. That guard is the shape of the DOCX
fix; preserve-what-you-read is the shape of the XLSX one.

### 1.1 XLSX — `regenerate_workbook_parts` replaced the workbook's whole relationship list

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`

The old line was:

```rust
opc.relationships.insert(WORKBOOK_PART.to_string(), workbook_rels);
```

where `workbook_rels` held exactly N worksheet pointers plus one `sharedStrings` pointer. Every
other relationship `xl/_rels/workbook.xml.rels` was READ with — `styles`, `theme`, `calcChain`,
`printerSettings`, `externalLink` — was overwritten out of existence. The committed fixture
`🧫️fixtures/📕️reuse-marketplaces.xlsx` declares five workbook relationships (2 × worksheet, theme,
styles, sharedStrings); we wrote three. Package-wide, the oracle's projection saw 7 distinct
relationship TYPES and ours 5, on `mutate-no-mutation` — an identity-level divergence, which is why
all 15 comparisons of the case failed.

Two further defects in the same six lines, neither of them measured by any scenario:

* the regenerated pointers were spelled with the **Transitional** type URIs unconditionally
  (`REL_TYPE_WORKSHEET`/`REL_TYPE_SHARED_STRINGS`), so writing a genuinely Strict package injected
  `schemas.openxmlformats.org` types into a `purl.oclc.org/ooxml` package;
* the package-root `officeDocument` check compared against the Transitional constant only, so a
  Strict package (whose root relationship is the purl one, which `decode_xlsx` explicitly
  recognises) gained a SECOND, contradictory root relationship on write.

The fix is `workbook_relationships`: only the `worksheet` and `sharedStrings` pointers are
regenerated (they are the two whose TARGETS this codec owns), each reusing the id and the declared
type URI of the pointer it replaces; every other relationship is preserved in its original position;
ids are minted only from those not already spoken for. Relationship types are matched by SUFFIX
(`/worksheet`, `/sharedStrings`, `/officeDocument`) — the convention `docx`'s `main_part_path`
already used — so a Strict and a Transitional package are handled by one code path. On the committed
fixture with no mutation the rebuilt list is now byte-identical to the one that was read.

### 1.2 DOCX — `sync_main_part` re-rendered `word/document.xml` from a view that cannot hold it

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs`

`sync_main_part` unconditionally did

```rust
let bytes = xml_document_to_text(&document_to_xml(&snap.document)).into_bytes();
snap.opc.set_part(MAIN_DOCUMENT_PART, &content_type, bytes);
```

and `document_to_xml` builds a fresh root: `elem("w:document", vec![attr("xmlns:w", W_NS)], …)`.
So every root attribute the package carried was discarded on write — including the very
`conformance` attribute the `✳️transitional` conformance-class vocabulary had just set. The mutation
reported `applied` and the written package carried nothing; the projection read
`mainRootAttributes` = `[xmlns:w]` where the oracle read `[conformance, xmlns:w]`. `word/styles.xml`
had the identical defect through `styles_to_xml`'s hardcoded `xmlns:w`, which is why
`set-main-namespace` and `set-snapshot` diverged on the `namespaces` axis too.

The fix has two halves, and both are needed:

1. **Do not rewrite a part that already projects to the view.** `part_already_projects` re-parses
   the part and compares; an unchanged `word/document.xml`/`word/styles.xml` is left byte-for-byte
   alone. This is the guard `encode_pptx` already applied to `ppt/presentation.xml`.
2. **When the part MUST be rewritten, keep its own shape.** `document_into_part` takes the root
   element's real name and attributes, the XML declaration, the doctype, the prolog, and every
   `w:body` child that is neither `w:p` nor `w:tbl` (`w:sectPr` above all) from the package that was
   READ, and regenerates only the `w:p`/`w:tbl` sequence `DocxDocument::body` is the view of.
   `styles_into_part` does the same for `w:styles`, and merges `w:name`/`w:basedOn` INTO the existing
   `w:style` element rather than rebuilding it, so a style keeps its real definition.

### 1.3 PPTX — the same relationship loss, found by inspection rather than by a failing case

`regenerate_presentation_parts` did `opc.relationships.insert(PRESENTATION_PART, pres_rels)` with
`pres_rels` = one slide-master pointer plus N slide pointers. On the committed
`🧫️fixtures/🎞️semio-talk.pptx` that discards `presProps`, `viewProps`, `tableStyles`, `notesMaster`
and `theme`. It has never shown up in parity because `encode_pptx` only calls it when the typed
presentation actually changed, and no conformance-class scenario changes it — the defect is real and
simply unmeasured. Fixed the same way as XLSX (`presentation_relationships`).

**Left open, reported rather than fixed:** the per-slide relationship lists are still wiped
(`opc.relationships.retain(|owner, _| !owner.starts_with("ppt/slides/"))`) and replaced with a single
`slideLayout1` pointer, which on the committed fixture destroys slide 23's `image` relationship and
rewrites every slide's layout target to layout 1. Preserving them by path is only correct while the
slide list is unchanged — an insert renumbers the parts, so a preserved list would be
mis-associated rather than merely lost. Closing it properly means keying slide relationships by
slide IDENTITY rather than by index, which is a `PptxPresentation` schema question, not a writer
one. Named here so it is not rediscovered as a surprise.

---

## 2. TIFF — `insert-ifd` omitted tag 278 because the snapshot could not hold a second raster

The `mutate-insert-ifd` row hands both sides an `ifd` param carrying six entries **and** a real
`pixels` strip. The oracle backs the page with those bytes, which forces
`RowsPerStrip = ImageLength` (TIFF6 §Strips: one combined strip, or a reader computing
`ceil(height / RowsPerStrip)` expects more `StripOffsets` than exist), so its IFD 2 projects seven
entries. Our subject adapter parsed the strip and threw it away —
`let (ifd, _pixels) = ifd_from_json(ifd_json)?;` — because `TiffIfd` had nowhere to put it, and the
encoder therefore wrote the six declared entries and no strip tags at all: six entries, one
divergence.

The feature file's own KNOWN-OPEN-DIVERGENCE paragraph named the remedy exactly and forbade the
shortcut: *"not a tolerance, an `ignoreKeys` entry or a cosmetic `RowsPerStrip` our encoder would
have nothing to back."* The remedy has been carried out rather than the shortcut taken.

`TiffIfd` now carries its own `pixels` — RAW STRIP BYTES, in the layout that directory's own
`BitsPerSample`/`SamplesPerPixel`/`Compression` entries declare, not the canonical RGBA
`TiffSnapshot::pixels` holds. Raw is the honest choice: strip bytes are lossless and
layout-agnostic, so a secondary page whose photometric layout this codec cannot decode (palette,
CMYK, tiled) still round-trips, where a decode-to-RGBA field would have to fail or fabricate. IFD 0
is the documented exception — the primary raster IS the document's image, is the field `SetPixels`
addresses, and stays the single authority — so `ifds[0].pixels` is always empty by construction.

Threaded through: snapshot (`TiffIfd`), diff (a new `TiffIfdDiff { entries, pixels }` replacing the
bare `TiffTagsDiff` in `TiffIfdModified`, with its absorb/apply/between/validate arms), the text and
binary diff codecs, and all four mirrors (`🔣️component.json`, `🛰️component.proto`,
`🔗️component.graphql`, `🟦️component.ts`) for both snapshot and diff.

`decode_tiff` now folds each secondary directory's strips into `pixels` **and drops
`StripOffsets`/`StripByteCounts` from its `entries`** — they are layout, not content, and keeping the
source file's byte offsets in a snapshot no longer bound to that file makes `decode(encode(x)) == x`
impossible. `encode_tiff_with` writes the strips back and recomputes the required triple, with
`RowsPerStrip` forced to `ImageLength`. A directory carrying no strip bytes is still metadata-only
and still gets no invented `RowsPerStrip`.

**A second defect the same change closes, which no scenario was measuring.** Before it, the encoder
omitted `StripOffsets`/`StripByteCounts` for every directory beyond the first, so every round trip of
the committed two-page fixture `🖼️abbau-aufbau-masterarbeit-grundriss.tiff` silently destroyed page
2's 768-byte raster. The semantic projection only decodes IFD 0's raster, so nothing ever saw it.
A new in-crate test pins it:
`secondary_ifd_raster_and_its_required_strip_tags_survive_the_codec`
(`🏅️standards/🔖️6.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`).

### 2.1 Which required baseline tags a synthesised IFD still omits

TIFF6 §Baseline requires, of a strip-organised bilevel/grayscale/palette/RGB image directory:
`ImageWidth` (256), `ImageLength` (257), `BitsPerSample` (258, RGB/palette), `Compression` (259),
`PhotometricInterpretation` (262), `StripOffsets` (273), `SamplesPerPixel` (277, RGB),
`RowsPerStrip` (278), `StripByteCounts` (279), `XResolution` (282), `YResolution` (283),
`ResolutionUnit` (296), and `ColorMap` (320) for palette images.

After this change the encoder emits 273/278/279 for any directory it has strip bytes for, and
carries 256/257/258/259/262/277 and everything else through verbatim from what the caller declared.
It does **not** synthesise 282/283/296 for a directory whose caller did not declare them, and does
not synthesise 320. That is deliberate and is the same policy IFD 0 has always had: this codec
writes the fields it can back with a fact, and never invents a resolution the document does not
claim. It is recorded here as a known, bounded gap rather than closed by fabrication.

---

## 3. What the parity numbers did — NOT MEASURED IN THIS SESSION, AND THAT IS SAID PLAINLY

**No `[test] level=…` line was obtained for any of the six cases, so no after-ratio is claimed.**
Not one number in this note is a prediction dressed as a measurement.

What WAS obtained, and is real:

* `cargo check -p semio-s-plugin-stdio --lib` — **exit 0, zero errors** with every change in this
  note applied. (An earlier attempt was red on `📄️pdf` 1.4 only, another session's in-flight
  §2.2(2) fix; it went green on its own and none of its errors were in a file this work touches.)
* **All 14 generated test hosts BUILD, every one exit 0** — `mutate-tiff-6-0`,
  `mutate-xlsx-ecma-376{,-transitional}`, `mutate-docx-ecma-376{,-transitional}`,
  `mutate-md-commonmark`, `mutate-pptx-ecma-376`, oracle AND subject halves, the subject halves with
  `--features sut` (log: `w17-writer-fidelity/prebuild5.log`). That link-checks the whole plugin plus
  every test adapter this work edited, including the `TiffIfd`/`TiffIfdDiff` schema change across the
  diff algebra and both codecs.
* The Markdown finding in §4 is not a prediction at all: it is read out of the two sides' committed
  raw output bytes from the run the audit itself measured.

Why the ratios are missing: the runner shells `cargo run` per case, and every `cargo` on this machine
serialises on the global `~/.cargo/.package-cache` lock. With 8–23 concurrent `cargo` processes from
other sessions throughout this window, the `mutate-docx-ecma-376-transitional` subject phase sat at
**0.2 s of CPU across 45+ minutes of wall clock** — blocked, not slow. Its ORACLE half completed
normally in 3 minutes once it got the lock, which is what pins the cause on contention rather than on
anything in this change. §5.1 records the prebuild workaround that does help and the one that does
not.

**To finish the measurement**, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, when the
machine is quiet:

```
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-xlsx-ecma-376-transitional
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-docx-ecma-376-transitional
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-tiff-6-0
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-md-commonmark
# regression guards for the three cases these writers also serve:
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-xlsx-ecma-376        # was 21/21
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-docx-ecma-376        # was 27/27
bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-pptx-ecma-376        # was 19/19
```

Baselines to compare against, from `📓️w13-final-audit.md` §2.1: `mutate-xlsx-ecma-376-transitional`
**0/15**, `mutate-docx-ecma-376-transitional` **8/13**, `mutate-tiff-6-0` **16/17**,
`mutate-md-commonmark` **12/13** (and expected to STAY 12/13 — see §4).

---

## 4. Markdown — the audit's attribution is wrong, and the raw bytes say so

`📓️w13-final-audit.md` §2.2(11) reads: *"We do not recognise an HTML comment as a CommonMark HTML
block, so every block after it shifts one place."* That is not what happens, and no change to this
repository's parser is warranted.

Both halves of a differential scenario project their OWN output bytes through the same
`comrak`-backed `project_md`. Read out of the run's own result directory
(`.🧬semio/🦑️repo/⚡️cache/tests/results/…-md-commonmark-{oracle,subject}-rust/mutate-set-snapshot.*.raw`):

```
ORACLE                                       SUBJECT
- First replacement item                     - First replacement item
- Second replacement item, …                 - Second replacement item, …
                                             
<!-- end list -->                            ```bash
                                             echo "…"
```bash                                      ```
```

The oracle's own output CONTAINS `<!-- end list -->`; ours does not. `comrak`'s WRITER injects that
separator between a list and a following code block (a conservative guard against an indented code
block being absorbed — unnecessary for the fenced block it itself always writes), and `comrak`'s
reader then reports it as a sixth document block. There is no `htmlBlock` in our output for our
parser to drop; our renderer reproduces exactly the five blocks it was given, which is the correct
CommonMark rendering. `parse(render(x)) != x` for `comrak` here.

This is precisely what `mutate-md-commonmark`'s own feature file already said (its
*"One scenario is left RED rather than tuned away, and it is the reference library's"* paragraph).
The audit contradicted the feature; the feature is right.

The claim that our parser handles HTML comments is now executable rather than argued:
`html_comment_between_a_list_and_a_code_block_is_an_html_block`
(`📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`) feeds
the parser the oracle's own byte sequence and asserts the `htmlBlock` appears in position 1 of 3.

**The obvious wrong fix, named so nobody reaches for it:** teaching our renderer to emit
`<!-- end list -->` too. That would be copying a reference library's non-normative quirk into our
writer — injecting document content no specification asks for — to make a number go green. It is the
same category of move as an `ignoreKeys` entry and is refused for the same reason.

### 4.1 What HTML block forms we actually do not handle

Audited against CommonMark §4.6's seven start conditions. `html_block_start`
(deserializers, ~line 121) accepts `<` + `!`/`?`/`/`+letter/letter at indent ≤ 3, and EVERY block
ends at the next blank line. Measured against the spec:

| type | start condition | our start | our end |
|---|---|---|---|
| 1 | `<script`/`<pre`/`<style`/`<textarea` | ✅ | ❌ ends at blank line; spec ends at the closing tag |
| 2 | `<!--` | ✅ | ❌ ends at blank line; spec ends at `-->` |
| 3 | `<?` | ✅ | ❌ ends at blank line; spec ends at `?>` |
| 4 | `<!` + letter | ✅ | ❌ ends at blank line; spec ends at `>` |
| 5 | `<![CDATA[` | ✅ | ❌ ends at blank line; spec ends at `]]>` |
| 6 | a known block tag name | ✅ | ✅ |
| 7 | a complete open/close tag ALONE on its line, not interrupting a paragraph | ⚠️ over-accepts: any letter-initial tag, no alone-on-line test, and it can interrupt a paragraph | ✅ |

Types 1–5 are only wrong for a construct that spans a blank line; type 7 is wrong in the other
direction (a paragraph beginning `<span>…` becomes an HTML block for us and a paragraph for the
spec). The module already declares this as a documented scope cut in place; none of it is reached by
the committed README fixture, which is why `md-commonmark` is otherwise 12/12. No change made: there
is no failing test driving one, and rewriting a parser's block grammar with nothing measuring the
result is exactly the un-test-driven move this repository forbids.

---

## 5. Notes for whoever runs the next full sweep

### 5.1 The 900-second budget is a lock-contention artefact, not a slow case

`bun ./📜️script.ts parity … --case <c>` shells `cargo run --features sut`, which for a subject host
is a from-scratch build of the whole `semio-s-plugin-stdio` crate into that host's OWN `target/`.
With several sessions building concurrently that exceeds the per-case 900 s budget and — because
`runProbe` throws out of `executeOne` — kills the entire run with no partial report (wave 13's
remedy, still open). The workaround used here: pre-build every host OUT of band first, with the
SAME feature set the runner will use —

```
cargo build --manifest-path <host>/Cargo.toml --features sut     # subject hosts
cargo build --manifest-path <host>/Cargo.toml                    # oracle hosts
```

— after which `cargo run` is a no-op and the case completes in seconds. Building subject hosts
WITHOUT `--features sut` is worthless: the feature set differs, so the runner rebuilds from scratch
anyway. That mistake cost this session a full 900 s timeout.

### 5.2 Concurrent-session state observed during this run

`cargo check -p semio-s-plugin-stdio --lib` was red for a stretch on `📄️pdf` 1.4 alone
(`no field 'page' on PdfSnapshot`, `PdfDiff::__dsl_diff_spec` missing) — another session landing
exactly the `📓️w13-final-audit.md` §2.2(2) fix, the 65-page-thesis-into-one-page defect. Unrelated
to this work and left alone. `✳️image`'s oracle manifest also changed under a third session, from a
`noOracleDecisions` entry to a real Pillow + independent-Python pairing; that is a strengthening, not
a weakening, and is noted only so the profile diff is not misread.

---

## 6. Before / after

| case | before (w13) | after | status |
|---|---|---|---|
| `mutate-xlsx-ecma-376-transitional` | 0/15 | not measured | fix landed, compiles, host builds |
| `mutate-docx-ecma-376-transitional` | 8/13 | not measured | fix landed, compiles, host builds |
| `mutate-tiff-6-0` | 16/17 | not measured | fix landed, compiles, host builds |
| `mutate-md-commonmark` | 12/13 | not measured | **no fix warranted** — §4 |
| `mutate-xlsx-ecma-376` | 21/21 | not measured | regression guard |
| `mutate-docx-ecma-376` | 27/27 | not measured | regression guard |
| `mutate-pptx-ecma-376` | 19/19 | not measured | regression guard |

Anyone continuing this: run §3's seven commands and fill this table from the runner's own
`[test] level=…` lines. Do not fill it from anything else.
