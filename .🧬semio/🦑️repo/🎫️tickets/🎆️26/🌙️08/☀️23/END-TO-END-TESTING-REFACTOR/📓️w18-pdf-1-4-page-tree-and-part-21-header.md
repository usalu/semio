# Wave 18 — PDF 1.4's page tree, and ISO 10303-21's `FILE_DESCRIPTION`

Date 2026-08-25. Successor to `📓️w13-final-audit.md` §2.2(2) and §2.2(6), which are the two
divergence clusters this wave closes. Raw evidence and the ticket-local probe that produced every
number below live in `w18-pdf14-and-part21-header/`.

---

## 0. What was already in the tree when this wave started, and what was not

Head at start `9152d149d6`, dirty tree. A peer session had already written most of the PDF 1.4
snapshot/codec rewrite and the whole Part-21 header fix, then stopped mid-flight: **the stdio plugin
library did not compile** (15 `no field 'page' on PdfSnapshot` errors plus one
`PdfDiff::__dsl_diff_spec` that no longer exists). That is recorded rather than glossed, because the
before/after numbers below are only meaningful against a tree that builds.

This wave finished that migration, found and fixed a second real codec defect the half-migration hid
(§2), rewrote the `✳️any` case that was still written against the stub (§3), and measured all of it.

---

## 1. Cluster 1 — PDF 1.4 could not represent a real document

### The decision: implement, not remove

`📄️pdf` 1.4 is not a subset that should not exist. ISO 32000-1 §7.7.3 gives PDF a real page TREE —
a catalog pointing at `/Pages`, whose `/Kids` recursively resolve to `/Page` leaves, each with its
own inheritable `/MediaBox` and content stream — and PDF 1.4 is the version ISO 19005-1 (PDF/A-1)
and ISO 15930-1 (PDF/X-1a) are written against, which is exactly why the `✳️a` and `✳️x`
conformance subsets hang off it. Three committed cases and a real 65-page fixture depend on it.
Removing it would delete the only place in the repository where those two published conformance
standards are measured at all. It was implemented.

### What the snapshot and codec are now

* `PdfSnapshot { schema, pages: Vec<PageDoc> }` — the page tree walked flat, one
  `PageDoc { width, height, text }` per leaf, in reading order. `Default` is ONE blank US-Letter
  page, because ISO 32000-1 §7.7.3.2 gives `/Count` a lower bound of one.
* `decode_pdf` walks `/Root → /Pages → /Kids` with real `/MediaBox` inheritance (§7.7.3.4) and a
  cycle guard, decodes each leaf's content stream(s) through the shared 1.7 COS lexer, and extracts
  ISO 32000-1 §9.4.3's four text-showing operators (`Tj`, `TJ`, `'`, `"`).
* `encode_pdf` writes every page: catalog, one-level page tree, per-page FlateDecode content
  stream, classic `xref` table and trailer. Deterministic — no timestamps, no document id.
* `PdfDiff` is the index-keyed `pages` triple (`removed`/`modified`/`added`), so "delete page 12" is
  a three-byte diff. It no longer derives `dsl::DslDiff` (that shape is off the derive path), so
  `register_schema_specs` registers the snapshot spec only — the same position `gif` 87a/89a hold
  for the same reason.

**1.4 is not a copy of 1.7's vocabulary.** 1.7 models a page as
`PdfPage { media_box, crop_box, rotate, text }` over a retained COS object graph
(`objects: Vec<PdfIndirectObject>`); 1.4 keeps the resolved PAGE view only and has the classic
cross-reference TABLE, because cross-reference streams and object streams are PDF **1.5** features.
`../✳️a`/`../✳️x`'s `stdio.pdf.{a,x}.schema-gap-unverifiable` diagnostic still fires unconditionally
and still says so — that honest limit was not touched and was not weakened.

### The second defect, found by this wave and not by the audit

`literal_string` built its `(…)` operand by `out.push(byte as char)` into a `String`. Every byte
`≥ 0x80` was therefore re-encoded as the TWO UTF-8 bytes of the Latin-1 code point of the same
value, so `é` was written and `Ã©` read back, and `decode → encode → decode` was NOT stable. Page 1
of the committed thesis carries exactly one such byte (a glyph code with no Unicode reading, which
the lossy decode turns into `U+FFFD`) — enough to move the `✳️a` projection on its own. The function
now returns `Vec<u8>` and writes bytes verbatim, escaping only `\`, `(`, `)`, CR and LF per
§7.3.4.2. `codec_retention_law_round_trips_every_page` gained a page carrying `ü ß U+FFFD 中文 \ ( )`
to pin it.

---

## 2. Cluster 2 — ISO 10303-21's `LIST[1:?]` population constraint

`write_part21` emitted `FILE_DESCRIPTION((),'')`, which `ruststep` refuses while tokenizing
(`expected ')', found (`) and which the reference writes as `(''),'2;1'`. The standard's lower bound
of one is a POPULATION constraint, so `()` is not a legal spelling of "nothing to say" — `('')` is.

The fix is at the WRITER, which is the only layer that catches every caller: `write_header_record`
takes ISO 10303-21 §8.2's fixed attribute list per record and, position by position, replaces an
empty `LIST[1:?]` with `('')` and pads a missing attribute with that position's unpopulated
spelling. Values the caller DID populate go out verbatim, so nothing a real document carried is
normalized away.

**The same defect was checked for elsewhere and it was there.** `LIST[1:?]` appears three more
times in the Part-21 header: `FILE_NAME.author`, `FILE_NAME.organization` (§8.2.3) and
`FILE_SCHEMA.schema_identifiers` (§8.2.4). All four are covered. Measured, with the probe:

```
=== write_part21(StepSnapshot::default()) ===       === a COMPLETELY EMPTY Part21Header ===
ISO-10303-21;                                        ISO-10303-21;
HEADER;                                              HEADER;
FILE_DESCRIPTION((''),'2;1');                        FILE_DESCRIPTION((''),'');
FILE_NAME('','',(''),(''),'','','');                 FILE_NAME('','',(''),(''),'','','');
FILE_SCHEMA((''));                                   FILE_SCHEMA((''));
ENDSEC;                                              ENDSEC;

[step::cc1..cc6] ruststep ACCEPTS the header this writer emits
[step::empty-header] ruststep ACCEPTS it
```

The right-hand column is the one that matters for reach: `Ifc2x3Mutation::SetHeader` can carry a
`Part21Header` whose three fields are all `vec![]`, and the writer now makes even that conformant.

---

## 3. The `✳️any` case had to be rewritten, and why that is a finding rather than collateral

`mutate-pdf-1-4` was reported `5/5` in the audit. It scored 5/5 because BOTH halves were written to
mirror the stub: the oracle rebuilt every document as one synthetic page pinned to
`MediaBox [0 0 612 792]`, and the laws were measured against that REBUILD rather than against the
committed bytes — explicitly, in the feature's own prose, so that the geometry gap could not fail
anything. A green measured against a baseline chosen to make it green is not evidence.

It is now measured against the real document:

* the oracle round-trips the thesis through `lopdf`'s own object graph (all 65 pages survive) and
  builds a `set-snapshot` target page for page from `params.snapshot.pages`;
* `project_pdf_1_4` projects `pageCount` plus, per page, the `/MediaBox` extent and the shown text —
  everything `PdfSnapshot` carries, nothing more;
* a `set-snapshot` spec carrying the OLD single-page shape, or no page at all, is an ERROR
  (`a_set_snapshot_spec_without_a_page_list_is_refused`), so the failure mode this wave removed
  cannot come back silently.

The document VERSION is deliberately not projected, and the reason is stated in the feature: the
committed file declares `%PDF-1.5` while this standard's writer emits `%PDF-1.4`, so recording it
would report a divergence about a field neither producer was asked to carry. That is a projection
decision taken deliberately and written down, not an `ignoreKeys` entry.

### Nothing was weakened

No `ignoreKeys` entry was added, no `tolerance` widened, no `arrays` mode changed, no fixture
swapped, no scenario deleted, no law relaxed. `semantic-pdf-v1` and
`semantic-pdf-1-4-conformance-{a,x}-v1` are byte-identical to what they were. The `✳️any`
projection got STRICTER (65 pages compared where 1 page's `width`/`height`/`text` was compared
before) and its baseline got stricter still (the committed document, not a rebuild of it).

---

## 4. Assets and fixtures handcrafted in the same change

| file | why |
|---|---|
| `✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` | `print_dsl` output of the new codec; the old one was the stub's |
| `✳️any/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio` | ditto for `encode_pack` |
| `✳️any/📚️examples/🎬️demo/🖼️assets/📄️example.pdf` | was **0 bytes**; now the real 648-byte encoder output |
| `📄set-snapshot/🧪️tests/shrinks-the-page-to-a5-and-rewrites-its-text/` quintet | before/after/mutation/diff moved to the page tree — now TWO pages, so the committed diff proves the whole-snapshot replacement still produces a SPARSE `modified:[{index:0}]` delta and leaves page 1 alone |

`parse_dsl(print_dsl(demo)) == demo` and `decode_pack(encode_pack(demo)) == demo` both hold, checked
directly.

---

## 5. Before / after — the runner's own `[test]` lines, copied verbatim

Every line below is `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case <case>` run from
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, and every exit code is the tool's own, read from
its exit status and never through a pipe. Raw logs: `w18-pdf14-and-part21-header/` (`cases.txt`,
`retry.txt`, `retry2.txt`).

```
mutate-pdf-1-4-a       [test] level=exhaustive cases=1 executed=18 passed=18 failed=0 errored=0 parity=9/9    exit=0
mutate-pdf-1-4-x       [test] level=exhaustive cases=1 executed=18 passed=18 failed=0 errored=0 parity=9/9    exit=0
mutate-pdf-1-4         [test] level=exhaustive cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5    exit=0
extract-text-pdf-1-4   [test] level=exhaustive cases=1 executed=2  passed=2  failed=0 errored=0 parity=0/0    exit=0
mutate-step-ap214-cc1  [test] level=exhaustive cases=1 executed=22 passed=22 failed=0 errored=0 parity=11/11  exit=0
mutate-step-ap214-cc2  [test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=13/13  exit=0
mutate-step-ap214-cc3  [test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=13/13  exit=0
mutate-step-ap214-cc4  [test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=13/13  exit=0
mutate-step-ap214-cc5  [test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=13/13  exit=0
mutate-step-ap214-cc6  [test] level=exhaustive cases=1 executed=22 passed=22 failed=0 errored=0 parity=11/11  exit=0
```

| case | before (`📓️w13-final-audit.md`) | after |
|---|---|---|
| `mutate-pdf-1-4-a` | **0 / 9** | **9 / 9** |
| `mutate-pdf-1-4-x` | **0 / 9** | **9 / 9** |
| `mutate-pdf-1-4` (`✳️any`) | 5 / 5, `executed=5` — oracle half only, measured against a REBUILT baseline | **5 / 5, `executed=10`** — both halves, measured against the real 65-page document |
| `mutate-step-ap214-cc1` | 10 / 11 | **11 / 11** |
| `mutate-step-ap214-cc2` | 12 / 13 | **13 / 13** |
| `mutate-step-ap214-cc3` | 12 / 13 | **13 / 13** |
| `mutate-step-ap214-cc4` | 12 / 13 | **13 / 13** |
| `mutate-step-ap214-cc5` | 12 / 13 | **13 / 13** |
| `mutate-step-ap214-cc6` | 10 / 11 | **11 / 11** |
| `extract-text-pdf-1-4` | 0 / 2, both scenarios **errored** | `executed=2 passed=2 errored=0`, reported `no-subject-implementation` — see §6 |

**24 divergences closed** (18 PDF + 6 STEP). Against the audit's `parity=1012/1277` stdio baseline
and holding every other case fixed: **1036 / 1277 = 81.1 %** (81.3 % if the two oracle-only
`extract-text-pdf-1-4` rows leave the denominator, which the per-case run's `parity=0/0` suggests),
up from 79.2 %.

Cross-checked independently before the runner ever ran, by the ticket-local probe
(`w18-pdf14-and-part21-header/probe-crate`), which drives the real subject codec and the real oracle
modules through the same `law::divergence_within` the runner uses and reported the same 23 of 23 for
the three PDF cases. Both measurements agree.

**`bun ./📜️script.ts contract` — exit 1, 2 high-priority breaches, neither from this wave.** Both
are `testing/discovery/unmanaged-tests` ratchets (`🧰️framework` 42 vs 35, `✏️s` 4 vs 1). The four
`✏️s` files are `🎞️animate/…/🧪️index.test.ts` and three `🎬️sequence/…` `.test.js` files — no file
this wave touched is a `.test.*` file, and all twelve mutation/oracle/comparison rule ids are still
at zero.

---

## 5b. Infrastructure that ate five of the ten runs, recorded rather than glossed

Six of the ten cases failed their FIRST run for reasons that were not in the code, and every one of
them passed on retry with no source change in between. That is worth writing down, because the same
three causes will hit the next wave:

1. **The 900 s per-case budget, twice** (`mutate-pdf-1-4-x`, `mutate-step-ap214-cc4`). Eight
   concurrent `parity` runs were sharing one cargo target dir. Re-run with
   `SEMIO_BUILD_BUDGET_MS=5400000` and both passed. Wave 13's remedy #10 (a budget overrun should
   fail that CASE, not throw out of `runPhases`) is still open and still costing whole runs.
2. **A peer session mid-edit in `semio-framework-os-kernel`** (`mutate-pdf-1-4`,
   `mutate-step-ap214-cc2`, `-cc3`). The subject host exited 101 with
   `could not compile semio-framework-os-kernel — 10 previous errors` at 19:1x and `3 previous
   errors` at 19:3x; a direct `cargo check -p semio-framework-os-kernel --lib` at 19:35 was **exit
   0**. A changing error count across builds is the signature of a live edit, not of a defect in the
   thing being tested. Both host `Cargo.toml`s and both `Cargo.lock`s were byte-identical apart from
   the package name, which is what ruled the alternative out.
3. **The disk filled twice** (`mutate-step-ap214-cc4`, `-cc5`, `-cc6` died on `ENOSPC`).
   `⚡️cache/agents/local/cargo-test-hosts/debug/incremental` had grown to **126 GB** and the volume
   was down to 118 MB. Clearing that one directory (pure regenerable build cache) returned 90 GB.
   This wave's own probe target dir was 12 GB of it and was deleted first.

---

## 6. Left open, with the reason

* **`extract-text-pdf-1-4`.** Both scenarios are `@mode-conformance`/`@mode-property` on the
  REFERENCE: the case exists because text extraction is the one PDF capability with no Rust
  reference here, and its evidence is `pypdf`'s font-decoded reading. Our 1.4 subset extracts the
  RAW operand bytes by design (no `/ToUnicode`, no `/Differences` resolution), and this document's
  fonts are Type3 with synthetic subset-local glyph names, so our reading legitimately cannot
  contain `"Ueli Saluz"`. Registering a subject here would either fabricate a decoder or commit a
  guaranteed-red row; neither is better than the honest "this case has no second producer". The
  `errored` verdict is the parity phase demanding a subject from an oracle-only case — a runner
  concern, not a codec defect.
* **`contract` reports 2 high-priority `testing/discovery` breaches** (`🧰️framework` 42 vs 35,
  `✏️s` 4 vs 1). Not from this wave: the four `✏️s` files are
  `🎞️animate/…/🧪️index.test.ts` and three `🎬️sequence/…` `.test.js` files, none of them touched here.
  Recorded so the next audit does not attribute them to this change.
* **The `✳️a`/`✳️x` `schema-gap-unverifiable` diagnostic still fires on every document.** PDF 1.4's
  snapshot carries the resolved page view, not an object graph, so full ISO 19005-1 / ISO 15930
  conformance still cannot be checked from it. Unchanged by this wave and not weakened by it.

---

## 7. Files touched

Production (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/`):
`✳️any/🚪️io/🦀️component.rs`, `✳️any/🧬️schema/📸️snapshot/🦀️component.rs`,
`✳️any/🧬️schema/🔺️diff/🦀️component.rs`, `✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (+ its facet
files), `✳️a/🧬️schema/🦀️component.rs`, `✳️a/🧬️schema/🧬️mutations/🦀️component.rs`,
`✳️x/🧬️schema/🦀️component.rs`, `✳️x/🧬️schema/🧬️mutations/🦀️component.rs`.

Oracles: `✳️any/🧪️oracle/🦀️component.rs` (rewritten), `✳️a/🧪️oracle/🦀️component.rs`,
`✳️x/🧪️oracle/🦀️component.rs`.

Cases (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/`): `mutate-pdf-1-4/` (adapter + feature
rewritten), `mutate-pdf-1-4-a/component.feature`, `mutate-pdf-1-4-x/component.feature`.

STEP: `📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🚪️io/📐️part21/🦀️component.rs`,
`✳️any/🧬️schema/📸️snapshot/🦀️component.rs`.
