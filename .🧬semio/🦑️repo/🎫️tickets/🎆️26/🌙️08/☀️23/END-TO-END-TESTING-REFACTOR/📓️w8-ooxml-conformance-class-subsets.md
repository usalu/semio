# Wave 8 — the six OOXML conformance-class subsets (xlsx / docx / pptx, ✳️strict + ✳️transitional)

Scope: the `📕️xlsx`, `📜️docx` and `🎞️pptx` non-`✳️any` subsets, which had no handcrafted mutations of
their own. All six now declare a vocabulary, register an oracle, and carry an exhaustive case on a
real committed package.

## Verified results

```
$ bun ./📜️script.ts contract --owner 🗄️stdio
0 high-priority breach(es) across 0 rule(s)

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-xlsx-ecma-376-strict
[test] level=exhaustive cases=1 executed=19 passed=19 failed=0 errored=0 parity=0/0
$ … --case mutate-xlsx-ecma-376-transitional
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
$ … --case mutate-docx-ecma-376-strict
[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0
$ … --case mutate-docx-ecma-376-transitional
[test] level=exhaustive cases=1 executed=13 passed=13 failed=0 errored=0 parity=0/0
$ … --case mutate-pptx-ecma-376-strict
[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0
$ … --case mutate-pptx-ecma-376-transitional
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0

$ cargo test --features oracles --lib     # 🧪️oracle/📦️packages/🦀️rust
test result: ok. 208 passed; 0 failed; 1 ignored
```

`parity=0/0` is the honest record that no subject ran: the plugin crate cannot compile in this
session (see "Honest limits").

## What genuinely distinguishes these subsets — the finding the vocabulary is built on

`✳️any` owns the DOCUMENT vocabulary (sheets/cells, blocks/runs, slides/shapes). Not one of its
mutations can move a package between conformance classes, because a conformance class is a property
of the OPC **package** and of no document object at all. Each subset's own
`check_<class>_conformance` was read line by line, and the vocabulary is **one kind per axis it
actually reads** — no more, no less:

| subset | axes its checker reads | kinds |
|---|---|---|
| xlsx ✳️strict | `xl/workbook.xml` root `xmlns`, root `xmlns:r`, root `conformance`, VML content type, worksheet content type | 9 |
| xlsx ✳️transitional | the same minus VML (Transitional retains VML — no rule exists) | 7 |
| docx ✳️strict | Strict main ns, Transitional ns anywhere, VML ns anywhere, `officeDocument` rel base, `conformance="strict"`, `mc:AlternateContent` | 10 |
| docx ✳️transitional | Transitional main ns, any strict-family ns, contradicting `conformance="strict"` | 6 |
| pptx ✳️strict | as docx ✳️strict **plus** the Transitional DrawingML namespace, which the pptx checker rejects separately | 11 |
| pptx ✳️transitional | as docx ✳️transitional, over two Transitional namespace families | 7 |

The sibling asymmetries are the specification's, not editorial: `✳️transitional` declares fewer
kinds precisely because ISO/IEC 29500-4 legalises VML and `mc:AlternateContent`, and `pptx`
declares one more than `docx` because a deck carries DrawingML.

## The reference

`quick-xml` 0.42 over `zip` 6 — a generic, namespace-agnostic OPC + XML implementation. It reads a
real package, performs every declared edit, and reassembles the whole container from the parts
alone, so every scenario is genuinely `@mode-differential`. Both crates are test-only and neither is
production-reachable: this repository's own OPC and XML codecs are hand-written and link neither.

**Rejected alternative, recorded.** The Python venv host has landed (`pypdf-pdf-1-4-text` is the
precedent), so `openpyxl` / `python-docx` / `python-pptx` were reachable. They were not chosen: each
is a document object model over the Transitional schema, exposing sheets/paragraphs/slides and no
API for the package's conformance class, so none could perform or observe a single kind in these
catalogs. That is a statement about the shape of their APIs. It was **not** verified against an
installed copy in this session — none of the three is importable on this machine.

## Defects this case found in the repository's own encoders

Found by designing the case, confirmed by reading the code, not by running it (see "Honest limits").

1. **`encode_xlsx` destroys the Strict stamp.** `encode_xlsx` calls `regenerate_workbook_parts`
   unconditionally, which drops and rebuilds `xl/workbook.xml`, `xl/worksheets/*` and
   `xl/sharedStrings.xml` from the typed model, with `SML_NS` and `R_NS` — the **Transitional**
   constants — hardcoded in `workbook_to_xml`/`worksheet_to_xml`. `XlsxStrictBuilderConstruction::
   stamp_strict_namespace` stamps `snapshot.opc`, and serialization throws that stamp away. An XLSX
   package this repository builds as ISO/IEC 29500-1 Strict is written to the wire as Transitional.
   All five of the xlsx conformance axes are on parts this function regenerates, so every xlsx
   `mutate-*` scenario will legitimately fail against the subject.
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🦀️component.rs:168,219`
2. **`encode_docx` has the same shape, narrower blast radius.** `sync_main_part` always rewrites
   `word/document.xml` from `snap.document` via `document_to_xml`, which hardcodes `W_NS` (the
   Transitional namespace). The main-namespace and `conformance` axes therefore cannot survive a
   docx encode either; the relationship-base axis can, because relationships are only added when
   missing, never rewritten. `.../📜️docx/.../🧵️serializers/🦀️component.rs:110,163,183`
3. **`pptx` is the one format where a Strict stamp survives.** `PptxSnapshot.xml_parts` carries
   every XML part as a typed `XmlDocument`, and `encode_pptx` regenerates `ppt/presentation.xml`
   only when the projected presentation actually changed. The pptx mutations therefore address
   `xml_parts` rather than `opc.parts` — `encode_pptx` explicitly rejects a package that stores an
   XML part as opaque OPC bytes.

The pattern: **all three `✳️any` encoders regenerate the main part from the typed model with the
Transitional namespaces baked in**, so a conformance-class stamp is a decode-time concept the write
path silently discards. That is a schema-completeness gap, not a coding slip, and it is recorded
rather than worked around.

## Facts about the corpus, recorded not papered over

Not one of the three committed OOXML fixtures carries VML markup, `mc:AlternateContent`, a
strict-family namespace or a `conformance` attribute — verified by unzipping all three and searching
every entry. The three removal kinds (`remove-conformance-attribute`, `remove-vml-part`,
`remove-alternate-content`) therefore run on the real package **after the same independent
implementation has inserted their target**. The mutation under test is still the removal, still
performed by the reference, still on a genuine OPC container; the arrangement is stated in every
affected Feature file and implemented once, in the shared engine's `conformance_arrange`.

## Evidence the oracle phase actually carries

An oracle-only run has no subject to compare against, so each handler is self-checking rather than a
recorded no-op:

* `mutate-<kind>` fails if the conformance-class projection did not move (except `no-mutation`).
* `inverse-<kind>` applies the mutation and its **independently computed** inverse — derived by
  reading the pre-state out of the bytes with `quick-xml`, never by calling this repository's own
  `Mutation::inverse` — and fails unless the projection returns to where it started.
* `identity-round-trip` fails on bit-identical output (byte pass-through) and on any projection
  drift across the reference's own container rebuild.

Thirty additional `#[test]`s in the six subset oracle modules prove the same laws directly against
the committed fixtures.

## Honest limits

* **No subject ran.** `cargo check --lib` on `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` stops at
  `semio-framework-job` with 6 errors from another session's live `RetainedJobPayload` refactor
  (`ManuallyDrop<Option<…>>` mismatches at :513, :523, :546, :650, a `generation` method that no
  longer exists at :1671, and a double mutable borrow at :489). Compilation never reaches this
  plugin, so **none of the six subject-side mutation modules has been compiled**, and neither has
  the `kinds_match_enum_and_catalog` test each of them carries. They are written to the same shape
  as the `✳️any` vocabularies and reviewed by hand; they are not verified.
* **`ArtifactBuilder::Mutation` still names `✳️any`'s document vocabulary** on all six subset
  builders. A builder has one associated mutation type and a Strict workbook still needs its cells
  edited, so unifying the document and conformance-class vocabularies behind one type is a real
  design question, not a mechanical edit. Left as a recorded open seam rather than guessed at while
  the workspace cannot compile.
* **`insert-alternate-content` appends to the part's ROOT element.** A word processor emits
  `mc:AlternateContent` deeper inside the body. The axis the conformance checkers read — does the
  part contain markup-compatibility markup — is exercised faithfully; the placement is the coarsest
  a package-level engine can address, and is documented in the shared engine.

## Files

Shared engine (new region in an existing shared family module — six subsets across three artifacts
genuinely share it):
* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs` — `//#region 🔖️OoxmlConformanceClass`

Oracle wiring:
* `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — six additive module entries beside
  each artifact's existing `any`. No `Cargo.toml` change: `zip` and `quick-xml` were already linked.

Per subset (`<artifact>/🏅️standards/🔖️ecma-376/🪆️subsets/✳️<class>/`):
* `🧬️schema/🧬️mutations/🦀️component.rs` — the vocabulary, `KINDS`, and the conformance `#[test]`
* `🧬️schema/🦀️component.rs` — mounts the above with `#[path]` (deliberately not through the
  contended shared `📦️glue.rs`)
* `🧪️oracle/🦀️component.rs` — dispatcher, projection, bridge, five `#[test]`s on the real fixture
* `🧪️oracle/🔣️component.json` — oracle, catalog and comparison profile

Per case (`<artifact>/🧪️tests/mutate-<fmt>-ecma-376-<class>/`):
* `component.feature`, `🦀️component.rs`
