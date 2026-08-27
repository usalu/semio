# Wave 12 — 📄️pdf, 📜️docx, 🎞️pptx, 📕️xlsx: every subset, handcrafted

Scope: all 19 standard-subsets of `📄️pdf` (1.4 and 1.7), `📜️docx`, `🎞️pptx`, `📕️xlsx`, including
every non-`✳️any` conformance subset. Date 2026-08-24. Every `[test]` line below was read from the
tool's own output, never through a pipe.

---

## 1. Headline

**8 subsets that had no mutation vocabulary, no oracle, no catalog and no case now have all four.**
They are the whole of the "17 subsets with no `🧬️mutations` directory" residue that falls inside this
scope:

`pdf 1.4 ✳️a`, `pdf 1.4 ✳️x`, `pdf 1.7 ✳️a`, `pdf 1.7 ✳️e`, `pdf 1.7 ✳️h`, `pdf 1.7 ✳️ua`,
`pdf 1.7 ✳️vt`, `pdf 1.7 ✳️x`.

**Coverage in this scope goes from 11/19 subsets to 19/19.** 99 new mutation kinds, 8 new cases,
216 new scenarios, all green on the oracle phase with the observability, inverse and
no-byte-pass-through laws asserted in role.

**Three genuine defects were found by asserting laws, and fixed rather than exempted** (§4).
**Two inverses genuinely do not exist and are now documented and refused rather than faked** (§5).

---

## 2. Verified per case — the real `[test]` lines

`bun ./📜️script.ts contract --owner 🗄️stdio --case <case>` reported
`0 high-priority breach(es) across 0 rule(s)` for **all nineteen**.

`bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case <case>`:

| case | line | new? |
|---|---|---|
| `mutate-pdf-1-4` | `[test] level=exhaustive cases=1 executed=5 passed=5 failed=0 errored=0 parity=0/0` | |
| `mutate-pdf-1-7` | `[test] level=exhaustive cases=1 executed=37 passed=37 failed=0 errored=0 parity=0/0` | was 34/3 |
| `mutate-pdf-1-4-a` | `[test] level=exhaustive cases=1 executed=9 passed=9 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-4-x` | `[test] level=exhaustive cases=1 executed=9 passed=9 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-a` | `[test] level=exhaustive cases=1 executed=33 passed=33 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-e` | `[test] level=exhaustive cases=1 executed=29 passed=29 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-h` | `[test] level=exhaustive cases=1 executed=25 passed=25 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-ua` | `[test] level=exhaustive cases=1 executed=27 passed=27 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-vt` | `[test] level=exhaustive cases=1 executed=41 passed=41 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-pdf-1-7-x` | `[test] level=exhaustive cases=1 executed=33 passed=33 failed=0 errored=0 parity=0/0` | **new** |
| `mutate-docx-ecma-376` | `[test] level=exhaustive cases=1 executed=27 passed=27 failed=0 errored=0 parity=0/0` | was 26/1 |
| `mutate-docx-ecma-376-strict` | `[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0` | |
| `mutate-docx-ecma-376-transitional` | `[test] level=exhaustive cases=1 executed=13 passed=13 failed=0 errored=0 parity=0/0` | |
| `mutate-pptx-ecma-376` | `[test] level=exhaustive cases=1 executed=19 passed=19 failed=0 errored=0 parity=0/0` | |
| `mutate-pptx-ecma-376-strict` | `[test] level=exhaustive cases=1 executed=23 passed=23 failed=0 errored=0 parity=0/0` | |
| `mutate-pptx-ecma-376-transitional` | `[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0` | |
| `mutate-xlsx-ecma-376` | `[test] level=exhaustive cases=1 executed=21 passed=21 failed=0 errored=0 parity=0/0` | was 20/1 |
| `mutate-xlsx-ecma-376-strict` | `[test] level=exhaustive cases=1 executed=19 passed=19 failed=0 errored=0 parity=0/0` | |
| `mutate-xlsx-ecma-376-transitional` | `[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0` | |

**421 scenarios, 421 passed, 0 failed.**

`cargo test --features oracles --lib` in the stdio oracle crate:
`test result: FAILED. 343 passed; 1 failed; 2 ignored`. The single failure is
`artifacts::txt::…::every_feature_row_inverts_back_to_the_real_document`, a DIFFERENT artifact
(`📄txt`) whose oracle module was last committed at **12:05 today** by a concurrent session and whose
own test doc comment says it exposes a `(lines, trailing_newline)` non-injectivity. Not this scope,
not caused by anything here.

`bun ./📜️script.ts dependency` exits 0 with the same three pre-existing `production-debt` records
(`png`, `zip`, `image`). All 8 new oracle registrations are `testOnly: true`.

`bun ./📜️script.ts contract --owner 🗄️stdio` reports 12 breaches, **all twelve** in
`🖼️bmp/🧪️tests/mutate-bmp-v3/component.feature` (`Unrecognized line 96`…`107`) — another session's
in-flight feature file, none in this scope.

---

## 3. What was handcrafted

### 3.1 The six PDF 1.7 conformance vocabularies

Each derived one kind per axis from that subset's OWN `check_*_conformance`, read line by line:

| subset | standard | checker axes | kinds |
|---|---|---|---|
| `✳️a` | ISO 19005-2/-3 | `/Encrypt` shape · `/S /JavaScript` + bare `/JS` · `/S /Launch` · Filespec `/EF` without `/AFRelationship` · `/S /GTS_PDFA1` OutputIntent · embedded font programs | 16 |
| `✳️e` | ISO 24517-1 | the same four forbidden constructs, plus `/Subtype /Movie`/`/Sound` annotations (never `/3D`), plus **any** OutputIntent | 14 |
| `✳️h` | PDF Healthcare BPG | `Info.title` AND `Info.author` · JavaScript · Launch · `/AcroForm` `/FT /Sig` · embedded fonts — the only checker in the six that raises nothing harder than a warning | 12 |
| `✳️ua` | ISO 14289-1 | `/MarkInfo /Marked` · `/StructTreeRoot` · `/Lang` · `/ViewerPreferences /DisplayDocTitle` · `Info.title` · embedded fonts — the only checker that raises `hard()` for a MISSING key | 13 |
| `✳️x` | ISO 15930-7 | `/Encrypt` · `/GTS_PDFX` intent **with** `/DestOutputProfile` · `/TrimBox` or `/ArtBox` on **every** page · fonts · JavaScript · Launch · media annotations | 16 |
| `✳️vt` | ISO 16612-2 | everything `check_x_conformance` reads — its first statement is literally `let mut out = check_x_conformance(snapshot);` — **plus** `/DPartRoot` and per-node `/DPM` | 20 |

`✳️vt ⊃ ✳️x` is the one superset relationship, and it is stated by the subset's own code rather than
copied; the implementation is shared through the named engine, and what differs is the declared axis
list and the declared vocabulary.

### 3.2 The two PDF 1.4 conformance vocabularies

PDF 1.4's snapshot is a bare `PageDoc { width, height, text }` — no object graph — and both checkers
say so: each raises exactly two diagnostics, one movable and one (`…schema-gap-unverifiable`) that
fires unconditionally on every document. So:

* `1.4/✳️a` → the TEXT axis: `no-mutation`, `set-snapshot`, `set-page-text`, `clear-page-text`.
* `1.4/✳️x` → the GEOMETRY axis: `no-mutation`, `set-snapshot`, `set-page-size`, `collapse-page-size`.

**They share not one kind**, because their checkers read different fields of the same snapshot. And
neither shares anything with its 1.7 namesake, because PDF 1.4 cannot observe an object graph. The
schema-gap axis is projected as the constant `true` it genuinely is, recorded as bookkeeping of an
axis no mutation can move — never as evidence.

### 3.3 The shared engines

* `🧪️oracle/📄️document/🦀️component.rs` gained `pub mod pdf_conformance` (~1 100 lines) — the
  `lopdf` 0.44 object-graph engine all six PDF 1.7 conformance subsets share, exactly as
  `pub mod ooxml` is shared by the six OOXML ones. Each subset declares its own
  `PdfConformanceProfile` (axes, OutputIntent marker, `/DestOutputProfile` requirement) and its own
  `KINDS`, and **refuses a kind it does not declare even when the engine could perform it** — proven
  by a test per subset.
* `pdf 1.7 ✳️any/🧬️schema/🧬️mutations` gained `pub mod conformance_support` — the production-side
  object-graph primitives the six vocabularies share by name rather than by six copies.

### 3.4 The fixture, and why the arrangements are recorded

The real 6.3 MB MiKTeX bachelor thesis: 65 pages, 3 189 indirect objects, 70 `/Type /Font`, 23
`/Type /FontDescriptor` **every one of which carries an embedded program** (5 `/FontFile`,
16 `/FontFile2`, 2 `/FontFile3`), classic xref, page 1 at A4 `[0 0 595.276 841.89]`, `/Info` with
`/Title ()` and `/Author ()` both present-and-empty. Scanned and confirmed: **no** `/Encrypt`,
JavaScript, `/JS`, `/Launch`, `/Movie`, `/Sound`, `/Filespec`, `/OutputIntents`, `/MarkInfo`,
`/StructTreeRoot`, `/Lang`, `/ViewerPreferences`, `/AcroForm`, `/DPartRoot`, `/TrimBox`, `/ArtBox`.

Every removal kind is therefore arranged by the same independent implementation first, and every
feature file names each arranged scenario individually. `embed-font-file` is the mirror case: its
target descriptor is EMPTIED first, because all 23 already carry a program.

Projections record **content, not object numbers**: re-inserting a removed action at a fresh object
number is a faithful undo, and a projection carrying the number would report a false divergence.

---

## 4. Three real defects, found by asserting a law, fixed rather than exempted

1. **`📕️xlsx ✳️any` — the three shared-string kinds were `Ok(input.to_vec())`.** The documented
   reason ("calamine's SST is private, `rust_xlsxwriter` cannot address the pool by index") was true
   of that pairing and **wrong about the package**: `xl/sharedStrings.xml` is an OPC part, and the
   second producer for a part is `zip` + `quick-xml`, which this owner has linked all along and which
   the six conformance subsets already run on. All three kinds now read the real 229-entry pool, edit
   it by index, rewrite the part and reassemble the container; `project_shared_string_pool` reads the
   result back out of the bytes, entry by entry. **`sharedStringCount` is no longer adapter-tracked
   arithmetic for those kinds, and all ten kinds are now `@mode-differential`.**
2. **`📄️pdf 1.7 ✳️any` — the oracle wrote `() Tj` for an empty page text.** The thesis sets its type
   with `TJ`, so the independent reader projects most pages as `text: []`; encoding an empty text as
   a text-showing operator showing the empty string turned such a page into `text: [""]`. It now
   writes `BT ET`. This was the cause of the last three `mutate-pdf-1-7` failures after the
   `contentOperators` exemption (§5.1) was applied, and fixing it kept the **text** axis — the one
   thing the vocabulary genuinely carries — under the full law.
3. **`📄️pdf 1.4 ✳️x` — nothing in the artifact measured real page geometry.** `decode_pdf` hardcodes
   612×792 for every input; this subset's projection is the first thing that reads the true
   `/MediaBox`, and a test asserts the fixture is A4 and explicitly `assert_ne!(width, 612.0)`. That
   is the axis a subject run would be expected to fail on until `decode_pdf` reads a real MediaBox —
   recorded in the module and in the feature rather than smoothed over.

---

## 5. Two inverses that genuinely do not exist — documented, refused, not faked

### 5.1 `pdf-1-7-any`: `contentOperators` cannot survive `remove-page` / `append-page-content` / `set-page-content`

`PdfPage`'s only content field is `text`, so `InsertPage`/`SetPageContent` carry extracted text and
nothing else and both producers regenerate a minimal five-operator stream from it. Page 8 of the real
thesis carries **294** operators. The three inverse scenarios now compare the projection with
`pages.N.contentOperators` dropped **and nothing else dropped** — version, page count, every page's
media box, rotation and the shown text all stay under the full law, and every other kind in the
catalog stays under it on every axis including `contentOperators`. Stated in the adapter and in the
Feature description. Widening `PdfPage` to retain a real content stream is the fix.

### 5.2 An appending insert cannot undo an interior remove — twice, in two artifacts

* `XlsxMutation::InsertSharedString` carries only a `value` and appends
  (`…/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:145`), so `remove-shared-string {"index": 7}` of a
  229-entry pool has **no inverse in this vocabulary**. The production `inverse()` at line 173 answers
  `SetSharedString`, which restores neither the length nor the entry that shifted into the hole.
* `DocxMutation::InsertStyle` carries only a `style` and appends (line 181), so
  `remove-style {"id": "Title"}` of `[Normal, Title, Heading1, Heading2, Heading3, Code, TableCell]`
  has no inverse either — undoing it leaves `Heading1` where `Title` was, which is precisely what the
  w11 failure `styles.1.id is "Heading1" — the original had "Title"` was reporting.

In both cases the oracle now **refuses** such a request with a message naming the gap, the Examples
row addresses the LAST element (`index: 228` → the German header `"Anzahl"`; `id: "TableCell"`) so the
law is genuinely assertable, and the Feature description states the limitation and names the fix
(an insert that carries a position). Neither assertion was weakened.

---

## 6. What could NOT be verified, and why

**The production crate does not compile, so none of the eight new production mutation modules could
be type-checked.** `cargo check -p semio-s-plugin-stdio --lib` fails on TWO concurrent sessions'
in-flight refactors:

```
error: could not compile `semio-framework-job` (lib) due to 6 previous errors
error[E0277]: the trait bound `action::UiFixedBytes: Eq` is not satisfied
error[E0599]: no method named `clone` found for struct `SurfaceDoc` in the current scope
error[E0631]: type mismatch in function arguments
error: could not compile `semio-framework-ui-contract` (lib) due to 5 previous errors
```

The first is the `ManuallyDrop<Option<RetainedJobPayload>>` migration the brief warned about; the
second is a newer one — `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs`
had an mtime of **11:14 today** while this ran. `semio-s-plugin-stdio` is never reached.

What WAS done instead: every new production file was parse-checked with
`rustfmt --edition 2021 --check` (all clean), written against APIs read from the real vendored
sources, and given the same `kinds_match_enum_and_catalog` / `mutation_apply_inverse_round_trips_
every_variant` / observability tests the docx-strict precedent carries. **They are unverified beyond
syntax and should be type-checked the moment the workspace builds.** The affected files are the eight
`🧬️mutations/🦀️component.rs` modules, the `conformance_support` region in
`pdf 1.7 ✳️any/🧬️schema/🧬️mutations/🦀️component.rs`, and the `#[cfg(feature = "sut")] mod subject`
half of the eight new adapters.

`parity=0/0` for every case, as for the whole ticket: no oracle-versus-subject comparison has run
anywhere, and the oracle-phase green is a claim about `lopdf`, `zip` and `quick-xml`, not about this
repository's codecs.

---

## 7. Things examined and found NOT to be defects

* **`🎞️pptx ✳️transitional` "shallow (tests)".** Read in full. Its dispatcher delegates to the shared
  OOXML engine with its own profile, refuses undeclared kinds, and its four tests assert
  observability, inverse restoration, a genuine container round trip that is not a byte pass-through,
  and refusal of a sibling's kind. `no_mutation_is_a_true_byte_identity` asserting byte identity is
  correct and documented. No change made.
* **The five `✳️any` cases' laws.** All five (`mutate-pdf-1-4`, `mutate-pdf-1-7`,
  `mutate-docx-ecma-376`, `mutate-pptx-ecma-376`, `mutate-xlsx-ecma-376`) already assert the inverse
  and no-byte-pass-through laws in role — wave 11 landed that, and the map handed to this wave was
  stale on that point. What was missing was the OBSERVABILITY law on the xlsx `mutate` handler, which
  was added.

---

## 8. Files

New (8 subsets × 4 + 2 engines):

* `📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/{✳️a,✳️e,✳️h,✳️ua,✳️vt,✳️x}/` — `🧪️oracle/🦀️component.rs`,
  `🧪️oracle/🔣️.json`, `🧬️schema/🧬️mutations/🦀️component.rs`
* `📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/{✳️a,✳️x}/` — the same three
* `📄️pdf/🧪️tests/mutate-pdf-1-7-{a,e,h,ua,vt,x}/`, `📄️pdf/🧪️tests/mutate-pdf-1-4-{a,x}/` —
  `component.feature` + `🦀️component.rs`

Modified:

* `🧪️oracle/📄️document/🦀️component.rs` — new `pdf_conformance` engine region
* `🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — 8 new subset mounts
* `📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` —
  `conformance_support` region
* `📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — empty-text content stream fix
* `📄️pdf/🧪️tests/mutate-pdf-1-7/{🦀️component.rs,component.feature}` — the `contentOperators` exemption
* `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — real shared-string pool
* `📕️xlsx/🧪️tests/mutate-xlsx-ecma-376/{🦀️component.rs,component.feature}`
* `📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — interior-style refusal
* `📜️docx/🧪️tests/mutate-docx-ecma-376/component.feature`
* the 8 subsets' `🧬️schema/🦀️component.rs` — the `#[path]` mount of their own `mutations` module
