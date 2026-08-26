# Final end-to-end audit of `26/08/23/END-TO-END-TESTING-REFACTOR`

Date 2026-08-25. Run window 05:25–16:50 CEST. Head at start
`18adc8cce3d223c1898c7543fa461928a81fe38f` (2026-08-25 01:06:10 +0200) with a dirty tree (164
tracked files modified by concurrent sessions); head moved to `9ed590cd8749af38dab141723300f9f91120cfad`
(2026-08-25 09:16:05 +0200) **during** the run, which forced a mid-run rebuild — recorded here rather
than glossed. Successor to `📓️w12-final-audit.md`, `📓️w12-what-the-numbers-mean.md`,
`📓️w13-stale-claims-and-the-semio-byte-law.md`, `📓️w13-three-corrections.md`,
`📓️w14-subject-parity-container-tabular-slice.md`.

Raw unfiltered logs: `w15-audit/`. Every `[test]` line below is copied verbatim from the tool's own
stdout; every exit code was read from the tool's own exit status, never through a pipe.

---

## 0. The single most misleading thing a reader would otherwise believe

**Every prior report in this ticket read the oracle phase's `failed=0` as evidence that the
implementations are correct. The first full differential run says otherwise: 265 of 1,277
oracle-versus-subject comparisons DISAGREE, and 51 scenarios fail or error outright.**

The same 101 stdio cases, the same tree, the same afternoon — `parity` finished 09:25, the
repo-wide `oracle` run 12:36:

```
[test] level=exhaustive cases=164 executed=1331 passed=1331 failed=0 errored=0 parity=0/0 not-exercised=85
[test] level=exhaustive cases=101 executed=3205 passed=3154 failed=27 errored=24 parity=1012/1277
```

The first line is `oracle exhaustive` repo-wide. The second is `parity exhaustive --owner 🗄️stdio`.
Nothing regressed between them. The oracle phase never compared two producers, so it could not see
any of it.

Concretely, and none of it was known before this run:

* **`mutate-pdf-1-4-a` and `mutate-pdf-1-4-x` score 0 of 9 and 0 of 9.** Our PDF 1.4 subject turns
  the 65-page, 6,346,331-byte bachelor thesis into a **607-byte, one-page skeleton** whose only text
  is `SemIO`. The oracle projects `pageCount: 65`; we project `pageCount: 1`. This is not a
  formatting difference — it is 64 pages and 136,000 characters of a real document destroyed on
  write, in cases that have been reported green since wave 7.
* **The six PDF 1.7 conformance classes (`✳️a ✳️e ✳️h ✳️ua ✳️vt ✳️x`) score 0 of 33, 0 of 29, 0 of 25,
  0 of 27, 0 of 41 and 0 of 33** — 188 comparisons, every single one, including `mutate-no-mutation`.
* **`mutate-xlsx-ecma-376-transitional` scores 0 of 15**, and `mutate-json-rfc8259-i-json` **0 of 22**.
* **Five stdio cases have never had a subject half that compiles** — `mutate-docx-ecma-376-strict`,
  `mutate-xlsx-ecma-376-strict`, `create-and-read-jpeg`, `create-and-round-trip-stl`,
  `mutate-json-rfc8259-i-json` — each failing on a single one-line adapter error, and each reported
  green in every wave of this ticket.
* **Both `✳️baseline` cases that wave 12 created to drive `unregistered-mutation-vocabulary` from 70
  to 0 fail their own subject phase** — `mutate-tiff-6-0-baseline` twice, `mutate-jpg-jfif-1-01-baseline`
  once. The evidence those cases promised instead of an oracle is red.
* **`mutate-dwg-ac1018` and `mutate-dwg-ac1024` fail all 14 of their scenarios**: `R2004 entity 0x239
  type 77: dwg bitstream underflow`. Our DWG reader cannot read the fixture the case commits.

The good news is real and belongs in the same breath: **nothing was weakened to produce those
numbers** (§3), the two codec fixes hold (§7), and coverage went from 29% of the repository's
scenarios to 50% (§4). But the headline of this ticket is not "the platform is green." It is "the
platform finally has a differential number, and that number is 79%."

---

## 1. The eight commands, verbatim

All run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` unless stated.

### 1. `bun ./📜️script.ts contract` — **exit 0**

```
0 high-priority breach(es) across 0 rule(s):


full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` read directly after the run: content `[]`, 3 bytes.
So no non-blocking priority hides behind the high-priority count.

**Breach count by rule id.** The validator raises fourteen distinct rule ids
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts`):
`unknown-mutation-catalog`, `mutation-catalog-capability-mismatch`, `mutation-kind-uncovered`,
`mutation-inverse-uncovered`, `mutation-kind-undeclared`, `mutation-kinds-deferred`, `no-adapter`,
`feature-syntax`, `missing-capability`, `missing-comparison`, `unknown-comparison`, `no-scenarios`,
`unregistered-mutation-vocabulary`, `mutation-catalog-unclaimed`. **Every one of the fourteen is at
zero.**

### 2. `bun ./📜️script.ts oracle exhaustive` (repo-wide) — **exit 0**

```
[test] level=exhaustive cases=164 executed=1331 passed=1331 failed=0 errored=0 parity=0/0 not-exercised=85
```

Preceded by exactly 85 `[test] not-exercised …` lines. Identical to wave 12's figure: 164 cases,
1,331 executed, 85 not exercised. The oracle-reachable population has not moved at all this wave.

### 3. `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio`

**This command did not complete, in three separate attempts, always the same way, and that is itself
a finding.** Attempt 1 (11:55) died at `mutate-tiff-6-0`; attempt 2 was killed at 13:13 while
crawling at one case per five to nine minutes; attempt 3 (16:50) died at `mutate-pdf-1-7-vt`:

```
[budget] cargo run --quiet --manifest-path …/mutate-pdf-1-7-vt-subject-rust/Cargo.toml --features sut -- --plan … exceeded 900000ms — killed. Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying.
error: spawnSync cargo ETIMEDOUT
      path: "cargo"
   syscall: "spawnSync cargo"
      code: "ETIMEDOUT"
```

Exit 1, no `[test] level=…` line at all. Two things are true here and both belong in the record.
First, another session was rebuilding the workspace throughout (head moved at 09:16, and stdio
`oracle`/`subject` result directories were being written by a process that is not mine). Second —
and this is ours — **a per-case 900 s budget overrun aborts the WHOLE run**: `runProbe` throws out of
`executeOne`, `runPhases` never reaches its summary, and the ninety cases that already passed are
discarded with no partial report. A per-case timeout should fail that case and continue.

**The subject-phase numbers below are therefore taken from the `parity` run, which executes the
identical subject phase**, read out of its own result stream before the shared cache was rotated (per-case table in
`w15-audit/parity-per-case.txt`, failure list in `w15-audit/failed-errored-51.txt`, run report in
`w15-audit/parity-report/`):

| subject-role results | count |
|---|---|
| rust passed | 1,833 |
| rust failed | 27 |
| python errored | 24 |
| **total subject results** | **1,884** |
| distinct stdio cases with a subject result | 97 of 101 |
| distinct stdio scenarios with a subject result | 1,884 of 1,928 |

The 44 scenarios with no subject result are the four cases whose rust host does not compile
(§2.4). **All 607 no-oracle stdio scenarios ran their subject phase.**

### 4. `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio` — **exit 1**

```
[test] level=exhaustive cases=101 executed=3205 passed=3154 failed=27 errored=24 parity=1012/1277
```

**This is the headline number of the entire ticket.** Full run report snapshotted to
`w15-audit/parity-report/` (`📈️metrics.json`, `📊️summary.json`); the run's own metrics file:

```json
"parityCoverage": {
  "rust":   { "compared": 1253, "equal": 1012, "ratio": 0.807662 },
  "python": { "compared": 24,   "equal": 0,    "ratio": 0 }
}
```

### 5. `bun ./📜️script.ts dependency` — **exit 0**

```
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

`entries=232`, `test-oracle=30`, the same three pre-existing `production-debt` records as wave 11 and
wave 12. Unchanged in every figure.

### 6. `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — **exit 0**

```
 69 pass
 0 fail
 1823 expect() calls
Ran 69 tests across 1 file. [115.26s]
```

Run under heavy concurrent load (a cargo build and the repo-wide parity run were both live).
`discovery is idempotent` did not flake — wave 13 gave it an explicit `30_000` budget
(`🧪️index.test.ts:239`), and wave 12's 2.6%-margin flake is closed. **But the suite gained nothing:
1,823 `expect()` calls, the same figure wave 12 measured.**

### 7. `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — **exit 0**

```
running 371 tests
test result: ok. 369 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 94.25s
```

371 tests, up from 369 at wave 12. The 2 ignored are still the same one-shot fixture-derivation
helpers (`artifacts::bmp::…::derive_real_world_fixture`, `artifacts::tiff::…::derive_real_world_fixture`);
neither is a skipped assertion.

### 8. `cargo check -p semio-s-plugin-stdio --lib`, from the repo root — **exit 0**

```
warning: `semio-s-plugin-stdio` (lib) generated 108 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 103 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 19.13s
```

---

## 2. The repo-wide parity ratio, and what every remaining failure means

### 2.1 The ratio

| owner | comparisons | equal | ratio |
|---|---|---|---|
| `🗄️stdio` (76 oracle-dispatched cases) | 1,277 | 1,012 | **79.2 %** |
| `🧰️framework` differential cases (`satisfy-version-requirements` 3/3, `compile-style-variants` 3/3, `flatten-class-name-inputs` 4/4) | 10 | 10 | 100 % |
| `🧰️framework` cross-subject (`host-protocol-parity` 30/30) | 30 | 30 | 100 % |
| every other plugin (56 cases, 2,614 scenarios) | 0 | 0 | — no oracle registered |
| **repo-wide** | **1,317** | **1,052** | **79.9 %** |

Excluding the cross-subject comparisons, the differential oracle-versus-subject ratio is
**1,022 / 1,287 = 79.4 %**.

Per-case, the 20 cases that are not perfect (full table in `w15-audit/`, derived from the run's own
`📊️summary.json`):

```
 14/15   mutate-bmp-v3          8/13   mutate-docx-ecma-376-transitional
 24/25   mutate-gif-87a        41/43   mutate-gif-89a
  0/22   mutate-json-rfc8259-i-json     12/13  mutate-md-commonmark
  0/2    extract-text-pdf-1-4   0/9    mutate-pdf-1-4-a      0/9    mutate-pdf-1-4-x
 34/37   mutate-pdf-1-7         0/33   mutate-pdf-1-7-a      0/29   mutate-pdf-1-7-e
  0/25   mutate-pdf-1-7-h       0/27   mutate-pdf-1-7-ua     0/41   mutate-pdf-1-7-vt
  0/33   mutate-pdf-1-7-x
 10/11   mutate-step-ap214-cc1  12/13  cc2   12/13  cc3   12/13  cc4   12/13  cc5   10/11  cc6
 16/17   mutate-tiff-6-0        0/15   mutate-xlsx-ecma-376-transitional
```

### 2.2 Every failing scenario, its diff, and its attribution — verified, not taken on report

**(1) 188 failures — the six PDF 1.7 conformance classes. Attribution: OUR PROJECTION, and it is
the one place where the deliberate fix belongs in the profile.**

`mutate-pdf-1-7-a::mutate-no-mutation::rust::subject`, profile `semantic-pdf-conformance-a-v1`:

```
$.fontPrograms[18].programBytes: numbers differ by more than 0
  oracle:  6849      subject: 6848
$.fontPrograms[19].programBytes   oracle: 7462   subject: 7463
$.fontPrograms[20].programBytes   oracle: 8162   subject: 8160
$.fontPrograms[21].programBytes   oracle: 6851   subject: 6849
$.fontPrograms[22].programBytes   oracle: 6860   subject: 6858
```

Five of twenty-three embedded font programs, off by one or two bytes, on **`no-mutation`** — so this
is an identity-level divergence, and it is why all 188 comparisons in the family fail.

I did not take the obvious reading on trust. `font_program`
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs:1283`) measures `stream.content.len()` —
the **FlateDecode-compressed** stream, not the font program. Decompressing every `/FontFile*` object
out of both sides' raw output (probe recorded at `w15-audit/pdf-font-program-probe.txt`):

```
oracle font-file objects: 23   subject: 23   same object numbers: True
compressed length differs: 5
   obj 3002 compressed 6849 -> 6848  decompressed 7281 -> 7281  contentSame True
   obj 3004 compressed 7462 -> 7463  decompressed 7912 -> 7912  contentSame True
   obj 3006 compressed 8162 -> 8160  decompressed 8645 -> 8645  contentSame True
   obj 3008 compressed 6851 -> 6849  decompressed 7274 -> 7274  contentSame True
   obj 3010 compressed 6860 -> 6858  decompressed 7290 -> 7290  contentSame True
decompressed CONTENT differs: 0
```

**All 23 embedded font programs are byte-identical after decompression.** What differs is deflate
output length — encoder freedom, by exactly the reading `semantic-pdf-v1` already committed to when
it listed `streamLength` in its `ignoreKeys`. The conformance profile's own description asserts the
opposite of the truth: *"an embedded font program's byte length … all of them exact integers or
literal coordinates a writer must reproduce exactly; hence no tolerance."*

This is the one divergence in the whole run that is legitimate writer freedom the profile should
already tolerate, and the fix is **not** an `ignoreKeys` entry: it is to make `font_program` report
the DECODED program length, which is what the profile description literally says it measures ("how
many bytes that program is"). That keeps the axis strict — a corrupted font program still fails —
while removing the compressed-length noise. Whoever does it must say so in the profile description,
because the sentence quoted above is currently false.

**(2) 18 failures — `mutate-pdf-1-4-a` (9/9) and `mutate-pdf-1-4-x` (9/9). Attribution: OURS, and it
is the most serious defect in the run.**

```
$.pageCount: numbers differ by more than 0
  oracle:  65     subject: 1
$.pageText: values differ
  oracle:  "SemIOAdomain-specicprogramminglanguageforarchitectsbyUeliSaluz…PhilippGeyer"
  subject: "SemIO"
```

The subject's whole output is 607 bytes:

```
%PDF-1.4
1 0 obj  << /Type /Catalog /Pages 2 0 R >>
2 0 obj  << /Type /Pages /Kids [3 0 R] /Count 1 >>
3 0 obj  << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R … >>
```

Root cause verified in the schema: the PDF **1.4** subset's snapshot
(`📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:19`) is

```rust
pub struct PdfSnapshot { pub schema: String, pub page: PageDoc }
```

— exactly **one** page, `{ width, height, text }`. All three PDF 1.4 cases feed it
`asset://…/📄️bachelor-thesis.pdf`, a 65-page document. The snapshot structurally cannot hold it, so
decode→encode discards 64 pages. `mutate-pdf-1-4` (the `✳️any` case) passes 5/5 only because its
profile does not project the page count. This is neither the reference library's fault nor the
fixture's: it is a case built on a snapshot that cannot carry its own input.

**(3) 22 failures + 22 errors — `mutate-json-rfc8259-i-json`, 0/22. Attribution: THE ADAPTER.**

```
errored … mutate-no-mutation -> adapter has no subject registration for scenario mutate-no-mutation
```

…and the same for all 22 scenarios. The Python adapter registers oracle handlers only. The Rust half
does not build either:

```
error[E0603]: crate `protocol` is private
error: could not compile `semio-test-host-mutate-json-rfc8259-i-json` (bin "host") due to 1 previous error
```

This case has never had a subject, in any language, at any point in this ticket.

**(4) 15 failures — `mutate-xlsx-ecma-376-transitional`, 0/15. Attribution: OURS (OOXML writer).**

```
$.relationshipTypes: array length differs   oracle: 7   subject: 5
$.relationshipTypes[3]  oracle: ".../relationships/styles"     subject: ".../relationships/worksheet"
$.relationshipTypes[4]  oracle: ".../relationships/theme"      subject: ".../metadata/core-properties"
$.relationshipTypes[5]  oracle: ".../relationships/worksheet"  subject: undefined
```

On `mutate-no-mutation`. Our writer **drops the `styles` and `theme` relationships** from the
package on every round trip.

**(5) 5 failures — `mutate-docx-ecma-376-transitional`, 8/13. Attribution: OURS.**

```
$.mainRootAttributes: array length differs   oracle: 2   subject: 1
$.mainRootAttributes[0].name   oracle: "conformance"  subject: "xmlns:w"
$.mainRootAttributes[0].value  oracle: "strict"       subject: "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
$.mainRootAttributes[1]        oracle: {"name":"xmlns:w",…}   subject: undefined
```

`set-conformance-attribute` reports applied and writes nothing to the document.

**(6) 6 failures — `mutate-step-ap214-cc1…cc6 :: mutate-set-snapshot`. Attribution: OURS, against the
specification.**

```
ruststep could not parse the input: Error while tokenizing STEP input
0: at line 3:
FILE_DESCRIPTION((),'');
                 ^
expected ')', found (
```

The oracle's own output for the same row, read from its raw file:

```
FILE_DESCRIPTION((''),'2;1');
```

ISO 10303-21 declares `FILE_DESCRIPTION.description` as `LIST[1:?] OF STRING` and
`implementation_level` as a defined value. `write_part21` emits an **empty** list and an **empty**
implementation level. The reference reader is right to refuse it; the header we write is
non-conformant.

**(7) 3 failures — `mutate-pdf-1-7 :: inverse-{remove-page, append-page-content, set-page-content}`,
34/37. Attribution: OURS (`PdfPage` snapshot), and already argued in the feature.**

Divergent on `pages.N.contentOperators` and on nothing else (295, 149 and 290 differences, all in
that one array). The case's own description states the position in full and refuses to drop the
axis; its subject handler holds all eighteen kinds to the inverse law with no carve-out at all and
passes. I verified the mechanism rather than trusting it: the oracle's `inverse_spec`
(`📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs:466, 495, 561`) rebuilds prior text
from `page_text`, which filters `operation.operator == "Tj"` (line 223), and this thesis sets its
type with `TJ`. The claim in the feature checks out. **The underlying cause is still ours**: the
vocabulary carries a page's content as a single extracted `text` field, so neither side can round-trip
a 294-operator content stream. Widening `PdfPage` is the fix, and the feature says so.

**(8) 3 failures — GIF. Attribution: an under-specified verb in OUR vocabulary. Documented open
divergences.**

```
mutate-gif-87a :: mutate-set-global-color-table -> gif87a: image 0 has an index past the end of its color table
mutate-gif-89a :: mutate-set-screen-size        -> encode_gif failed: gif89a: frame 0 region exceeds the logical screen
mutate-gif-89a :: mutate-set-frame-geometry     -> encode_gif failed: gif89a: frame 0 indices length mismatch
```

Both features carry a `⚠️ KNOWN OPEN DIVERGENCE` paragraph naming the exact rows, the exact numbers,
and the instruction *"Do not weaken the profile, the row's parameters or the fixture to close it."*
The reference silently normalizes; our encoder refuses. Neither is wrong until the verb says what it
means.

**(9) 2 errors — `extract-text-pdf-1-4`, 0/2. Attribution: THE ADAPTER.** `adapter has no subject
registration for scenario every-page-yields-text` / `…declared-pages-carry-their-printed-text`.

**(10) 1 failure — `mutate-bmp-v3 :: mutate-set-pixel-data`, 14/15. Attribution: under-specified verb,
documented.**

```
encode_bmp failed: bmp: pixel (0,2559) is rgb(200,40,40), which has no matching entry in the declared 240-entry palette — cannot encode as 8-bit indexed without narrowing
```

The feature already records `parity 14/15` and forbids closing it by weakening anything.

**(11) 1 failure — `mutate-md-commonmark :: mutate-set-snapshot`, 12/13. Attribution: OURS
(CommonMark parser).**

```
$.blocks: array length differs   oracle: 6   subject: 5
$.blocks[3].kind   oracle: "htmlBlock"   subject: "codeBlock"
$.blocks[3].raw    oracle: "<!-- end list -->\n"   subject: undefined
```

We do not recognise an HTML comment as a CommonMark HTML block, so every block after it shifts one
place.

**(12) 1 failure — `mutate-tiff-6-0 :: mutate-insert-ifd`, 16/17. Attribution: OURS.**

```
$.ifds[2].entries: array length differs   oracle: 7   subject: 6
$.ifds[2].entries[6]   oracle: {"tag":278,"type":4,"values":[8]}   subject: undefined
```

Our inserted IFD omits tag 278 (`RowsPerStrip`).

### 2.3 The 51 scenarios that failed or errored outright

27 failed + 24 errored. Beyond the parity divergences above, three groups are new:

| scenario | message | attribution |
|---|---|---|
| `mutate-dwg-ac1018` ×7 and `mutate-dwg-ac1024` ×7 — **every scenario of both cases** | `R2004 entity 0x239 type 77: dwg bitstream underflow` | **Ours.** Our DWG reader cannot decode the fixture the case commits. Both are `@no-oracle-` cases whose stated evidence IS the subject phase. |
| `mutate-jpg-jfif-1-01-baseline :: identity-round-trip` | `identity law violated: decoding and re-encoding moved the semantic projection — $.components[0] is "1:2x2", expected "1:1x1"` | **Ours.** Re-encoding changes chroma subsampling. |
| `mutate-tiff-6-0-baseline :: mutate-remove-tile-tags` | `this row moves its axis in the direction that stays inside the class, so the verdict must not change, but it went from ["stdio.tiff.baseline.tiled-not-baseline"] to []` | **Ours.** |
| `mutate-tiff-6-0-baseline :: identity-round-trip` | `identity law violated … $.stripOffsets is "388", expected "412"` | **Ours.** |

**The last three are the two `✳️baseline` subsets wave 12 created to drive
`unregistered-mutation-vocabulary` from 70 to 0.** Wave 12 closed those breaches by declaring a
no-oracle case whose evidence "is discharged by the subject phase." The subject phase has now run,
and for both of them it is red.

### 2.4 Five cases whose subject half has never compiled

```
[test] …/mutate-docx-ecma-376-strict: rust subject host exited 101 without emitting results
[test] …/create-and-read-jpeg:        rust subject host exited 101 without emitting results
[test] …/mutate-json-rfc8259-i-json:  rust subject host exited 101 without emitting results
[test] …/create-and-round-trip-stl:   rust subject host exited 101 without emitting results
[test] …/mutate-xlsx-ecma-376-strict: rust subject host exited 101 without emitting results
```

One error each:

```
error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::mutations::vml_markup`
error[E0432]: unresolved import `semio_s_plugin_stdio::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::mutations::vml_markup`
error[E0433]: cannot find `jfif_1_01` in `standards`
error[E0433]: cannot find `ascii` in `standards`
error[E0603]: crate `protocol` is private
```

This is the exact class of defect wave 13 found once, in `create-and-round-trip-bmp` (`v3` where the
module is `v_v3`), and named as *"green and wrong at the same time"*. There were five more. **The
runner does not count them**: a case whose host fails to build contributes `executed=0 passed=0
failed=0 errored=0` to the summary line (`📜️script.ts:497-503` returns `results: []`), so only the
`problems` list and the exit code show it. Wave 12's remedy #7 is still open, and it cost this ticket
five silently-unmeasured cases.

---

## 3. Did anyone weaken evidence to get parity up? — checked six ways, one finding

**Comparison profiles.** `git diff <ticket-start|w11|w12> -- '*🔣️component.json'` filtered to
`ignoreKeys|tolerance|arrays|"mode"` returns **only `+` lines, all of them inside newly added
profiles**. Not one existing profile gained an `ignoreKeys` entry, loosened a `tolerance`, or changed
its `arrays` mode at any point in this ticket. `semantic-pdf-v1` still declares the same 11 keys and
`0.0001`; `semantic-pdf-conformance-a-v1` still declares `"tolerance": 0, "ignoreKeys": []` — which
is why it is red in §2.2(1) rather than quietly green.

**Fixtures.** `git diff … -- '*component.feature'` shows **no removed or changed `asset://` /
`shared://` / `local://` line** anywhere in the ticket. `git diff --name-status` over `*🧫️fixtures*`
and `*📚️examples*` shows **zero deletions**; every entry is `A` except fourteen `M`s, thirteen of
which are `.rs` demo modules.

**The one fixture that was edited — named, because it is exactly the pattern to watch for.**
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🌐️example.html`
was changed in commit `18adc8cce3`, in the **same commit** as the HTML5 parser fix, from

```html
<html lang="en">
<head>
…
</body>
</html>
```

to `<html lang="en"><head>` … `</body></html>` with no trailing newline. The in-crate test
`codec_retention_law` asserts `write_html_document(parse_html_document(FIXTURE)) == FIXTURE`; after
the (correct, WHATWG §13.2.6.4.3) fix that drops whitespace before `<head>`, the old bytes could no
longer round-trip. **The fixture was normalised into a fixpoint of our own writer so that a
byte-identity assertion would pass.** Three mitigations, all verifiable: the parser change is a
genuine conformance fix found by the parity phase and documented at length; this asset is used by no
test case (only that unit test); and `mutate-html-5`'s real fixture,
`🧫️fixtures/🌐️zukunft-bau-entwerfen-mit-bestand.html`, was **not** touched (last modified
`ede955d5a2`, 2026-08-24 01:10, before the HTML5 parity work) — its parity went 13/21 → 21/21 purely
through the parser fix. Still: the honest form of that assertion is idempotence
(`write(parse(write(parse(x)))) == write(parse(x))`) on the real bytes, not equality against a
fixture edited to satisfy it. And the test cannot run at all (§6).

**Law calls.** `git diff c3a79bd4ce -- '*🧪️tests/*🦀️component.rs' | grep '^-.*law::'` returns
**nothing**. No assertion was deleted from any adapter since the wave-12 audit.

**Exemption lists.** Every `UNOBSERVABLE` / `GUARD_VECTORS` / `*_WRITER_FREEDOM` diff in the ticket is
an addition to a new case; none of the pre-existing lists grew. `mutate-pdf-1-7`'s subject handler
went the other way — wave 13 gave it `mutation_is_observable_within` and `inverse_restores_within`
with **no carve-out at all**, stricter than the oracle half.

**Scenarios.** No case was deleted; the case count is 164 at both wave 12 and now, and the scenario
count went **up** by 2 (`mutate-semio-brep` and `mutate-semio-table` each gained an
`identity-round-trip`).

**Two exemption lists that are honest but hollow, and that no gate measures.** Not weakening — they
predate the parity work and are documented in place — but they belong in a final audit:

* `mutate-energy-model-1` lists **1 of 1** kinds as `UNOBSERVABLE`. Its only committed vector is a
  documented no-op, so the plugin's only mutation kind has **no forward evidence at all**. Its feature
  says so in a paragraph headed *"A gap, stated plainly rather than papered over."*
* `mutate-mathematical-1` lists **13 of 15**. Nine carry rejection vectors and four are
  `applied`-but-`mutation.no-op`, "because the graph and the point cloud they address live in composed
  children no fixture can resolve."

Wave 12's "63 of 63 asserting, zero vacuous" was measured over the 63 oracle-dispatched cases only.
Nobody has ever measured the 85 no-oracle cases the same way.

---

## 4. How many scenarios execute in at least one phase

**2,284 of 4,564 (50.0 %)** — up from **1,331 of 4,562 (29.2 %)** at wave 12.

| population | cases | scenarios | executed in ≥1 phase |
|---|---|---|---|
| `🗄️stdio` | 101 | 1,928 | **1,928 (100 %)** — 1,321 oracle, 1,884 subject, 607 subject-only |
| `🧰️framework` | 7 | 22 | **22 (100 %)** |
| non-stdio plugins whose subject phase runs | 12 | 372 | **334** |
| non-stdio plugins whose subject phase cannot run | 44 | 2,242 | **0** |
| **total** | **164** | **4,564** | **2,284** |

Measured, not assumed. The repo has 164 feature files expanding to 4,564 scenarios (79 carry
`@oracle-` → 1,331 scenarios; 85 carry `@no-oracle-` → 3,233 scenarios; the split is unchanged from
wave 12 apart from the +2). The stdio figure is the distinct `(case, scenario)` set in the parity
run's own result stream. The framework figure is seven `parity exhaustive --case …` runs, all exit 0.
The non-stdio figure is seven `subject exhaustive --owner …` runs:

```
=== 🌀️procedural ===  [test] level=exhaustive cases=3 executed=77 passed=77 failed=0 errored=0 parity=0/0
=== 🌍️gis ===         [test] level=exhaustive cases=2 executed=25 passed=25 failed=0 errored=0 parity=0/0 not-exercised=1
                      error: could not compile `semio-test-host-mutate-gisterrain-1` (bin "host") due to 2 previous errors
=== 🎪️demonstrator === [test] level=exhaustive cases=1 executed=3 passed=3 failed=0 errored=0 parity=0/0
=== 🏭️process ===     [test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
                      error: could not compile `semio-test-host-mutate-process3d-1` (bin "host") due to 2 previous errors
=== 📐️cad ===         [test] level=exhaustive cases=1 executed=41 passed=41 failed=0 errored=0 parity=0/0
=== 🧩️puzzle ===      [test] level=exhaustive cases=3 executed=181 passed=181 failed=0 errored=0 parity=0/0
=== 🪵️sourcing ===    [test] level=exhaustive cases=1 executed=7 passed=6 failed=1 errored=0 parity=0/0
```

**One more real failure in there**, and it is the first evidence anyone has ever collected for that
plugin:

```
mutate-curate-1 :: identity-round-trip -> TextError { message: "expected Text, found Absent", span: TextSpan { line: 1, column: 1, length: 0 } }
```

Both host-build failures are one-line adapter bugs (`error[E0618]: expected function, found
std::string::String`, twice each) — the same category as §2.4.

**2,280 scenarios — 50 % of the repository, across 46 cases — still execute in no phase at all and
have no phase that could produce any.** 44 of those cases sit in the 26 plugins whose library does
not compile; the other two (`mutate-gisterrain-1`, 5 scenarios, and `mutate-process3d-1`, 33) sit in
plugins that DO compile but whose generated test hosts do not. That is down from 3,231 at wave 12,
and it is still the largest single number in this ticket.

---

## 5. How many plugins can run a subject phase at all

**7 of 34, up from 1 of 34.**

`cargo check -p <plugin> --lib` for all 33 plugin crates (34 plugin directories; `🗟️artifacts` has no
Rust package). Raw logs `w15-audit/chk-*.txt`, matrix `w15-audit/08-plugin-matrix.txt`.

| verdict | plugins |
|---|---|
| **lib compiles (8)** | `stdio`, `cad`, `demonstrator`, `gis`, `procedural`, `process`, `puzzle`, `sourcing` |
| **subject phase actually runs (7)** | the eight above **minus `process`**, whose lib compiles but whose test host does not |
| blocked purely upstream (1) | `space` — its own code has zero errors |
| blocked purely by a peer plugin (1) | `writer` — fails on its `trinity` dependency (1,289 errors), its own code never reached |
| blocked by its own async-convention debt (23) | `norm` 6,082 · `architect` 2,591 · `mathematical` 2,432 · `block` 1,522 · `trinity` 1,289 · `raster` 764 · `note` 745 · `forms` 739 · `lowpoly` 706 · `layout` 653 · `draw` 533 · `shooting` 517 · `flow` 499 · `dag` 482 · `reasoning-mindmap` 438 · `sequence` 412 · `remodel` 411 · `playbook` 270 · `vcs` 166 · `energy` 162 · `fem` 79 · `animate` 57 · `imperative` 11 |

### The correction that matters: "the os-kernel blocker is gone" is only true for one feature set

`cargo check -p semio-framework-os-kernel --lib` is **exit 0** — confirmed again here. But
`cargo check -p semio-s-plugin-space --lib` is **exit 101**, and every error is in os-kernel, not in
`space`:

```
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🔄️sync/🦀️component.rs:889:20: error[E0599]: the method `attach_backbone` exists for struct `os_store::component::ArtifactStore<P, Mutation>`, but its trait bounds were not satisfied
…:900 detach_backbone   …:929 tick   …:940 dispatch
…:2241:21: error: future cannot be sent between threads safely: future created by async block is not `Send`
error: could not compile `semio-framework-os-kernel` (lib) due to 10 previous errors; 28 warnings emitted
```

`cargo tree -e features` explains it: `space` pulls `semio-framework-os` with `os-host-full`, which
unifies os-kernel's `sync` feature (`tokio-tungstenite`, `semio-framework-os-services`, `tokio/rt`,
`tokio/time`); `stdio` reaches os-kernel through `semio-framework-plugin`'s `component-guest` and
never enables it. **The store blocker earlier waves declared cleared is still standing on the `sync`
path**, and every case file and note that says "the os-kernel blocker was cleared on 2026-08-24"
should say "cleared for the default feature set."

---

## 6. Stubs, placeholders, `todo!`, templated prose, undocumented sibling copies

* **`todo!` / `unimplemented!`**: **zero** across every `🧪️oracle` and `🧪️tests` tree under
  `✏️s/🔌️plugins`.
* **Stale "peer-blocked" prose**: down to two lines, and both now read correctly
  (`mutate-semio-kit/🦀️component.rs:31`, `mutate-zip-2-0/🦀️component.rs:11`, each of the form *"…that
  blocker was cleared on 2026-08-24"*). `grep -rn "OPEN, and left red\|FAILS on the ORACLE" ✏️s`
  returns nothing. Wave 12 items 4 and the 15-adapter claim are genuinely closed. **They will need
  editing again for §5.**
* **The 1.000-similarity sibling pair is gone.** `create-and-round-trip-bmp` and
  `create-and-round-trip-tiff` are now written against each format's own layout, and the BMP case's
  second fixture moved `8×4 → 5×3` so that a 15-byte scanline actually exercises BMP's 4-byte padding
  rule — a strengthened fixture, and the strongest single piece of work in wave 13. Re-measured over
  all 164 features with 5-gram Jaccard: **maximum pairwise similarity is now 0.806**
  (`mutate-pdf-1-4-a` / `-x`), and every pair above 0.65 is a documented conformance-class family.
* **Templated prose is still heavy repo-wide, and no gate sees it.** Sentences over 70 characters
  appearing in three or more feature descriptions: **78 distinct sentences, touching 122 of the 164
  features.** One sentence appears in **39**. Wave 13 measured "ZERO" — but only over the six OOXML
  cases it rewrote. Most of the 78 are shared platform-law boilerplate rather than silent copies of a
  sibling's content, and the largest genuinely-similar families (`step-ap214-cc1…cc6` at 20 shared
  sentences each, `mutate-pdf-1-7-{a,e,h,ua,vt,x}` at 15–17) each carry a paragraph naming what makes
  that subset different. It is not the standard's forbidden silent copy; it is not "zero" either.
* **The highest-similarity ADAPTER pair is 0.939** —
  `mutate-procedural-2d-1/🦀️component.rs` vs `mutate-procedural-3d-1/🦀️component.rs`. Their features
  state the relationship (`"This subset shares a snapshot SHAPE with 🧊️procedural3d"`); neither
  adapter header mentions the other.
* **A real broken `include_str!`**, found by building the stdio lib's test target:
  `🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:163` reads
  `"../../../../🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/🔣️component.json"`, which resolves inside
  `🔖️ac1024/` and needs one more `../`. The file it wants exists. That `#[cfg(test)]` catalog
  cross-check has never compiled.
* **`cargo check -p semio-s-plugin-stdio --lib --profile test` — exit 101, `913 previous errors`.**
  Wave 13 called this "the biggest thing this audit found that is still open"; it is unchanged. By
  artifact: `🧿️semio` 238, `🏗️ifc` 118, `📄️pdf` 97, `🎞️pptx` 87, `🟪️stl` 85, `🧊️gltf` 50, `🎥️mp4` 46,
  `🎨️svg` 44, `📜️docx` 43, `📐️step` 31, and 12 more. The damage is the automated async sweep —
  `error[E0425]: cannot find value 'exp'` (from `exp.awaitected`), `invalid format string`,
  `#[async_test] can only be applied to an 'async fn'`. **No `#[test]` inside the production stdio
  plugin can be run today**, which includes every unit test written for the two codec fixes in §7.

---

## 7. Are both codec fixes confirmed by a passing parity or subject run?

**Yes for both, and it matters that the answer is not "the unit test passes" — those unit tests
cannot run at all (§6).**

**`🧊️obj RemoveFace` membership.** The repair is in production
(`🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:185` `restore_face_at`,
`:199` `restore_group_at`, `:208` `restore_object_at`, routed at `:307`, `:320`, `:328`).
`mutate-obj-3-0` is **`45/45` parity, `executed=45 passed=45 failed=0`** in the full parity run —
oracle against subject, on the real document, `inverse-remove-face` included. `create-and-round-trip-obj`
is `2/2`. Confirmed end to end.

**`📄txt` injectivity.** `non_canonical_reason` gates both the oracle
(`📄txt/…/🧪️oracle/🦀️component.rs:74`, applied at `:171`) and **production**
(`📄txt/…/🧬️schema/🧬️mutations/🦀️component.rs:104`). `mutate-txt-utf-8` is a recorded no-oracle case,
so its evidence is the subject phase: **all 24 of its scenarios executed and passed** in the parity
run's subject half (`executed=24 passed=24 failed=0`, reproduced three times in the wave-13 logs).
Confirmed end to end.

Neither fix is among the 265 parity divergences or the 51 failures.

---

## 8. Totals

| | wave 12 | now |
|---|---|---|
| contract breaches, all 14 rule ids | 0 | **0** |
| cases / scenarios in the repository | 164 / 4,562 | 164 / **4,564** |
| oracle phase | 1,331 executed, 0 failed | **1,331 executed, 0 failed** |
| **oracle-vs-subject comparisons, repo-wide** | **0 / 0** | **1,052 / 1,317 (79.9 %)** |
| … stdio only | 0 / 0 | **1,012 / 1,277 (79.2 %)** |
| scenarios failing or erroring | 0 (nothing compared) | **51** |
| scenarios executing in ≥ 1 phase | 1,331 of 4,562 (29.2 %) | **2,284 of 4,564 (50.0 %)** |
| scenarios executing in NO phase | 3,231 (70.8 %) | **2,280 (50.0 %)** |
| plugins whose subject phase runs | 1 of 34 | **7 of 34** |
| plugin libs that compile | 1 of 33 | **8 of 33** |
| stdio cases whose subject host does not compile | unknown | **5** |
| stdio oracle unit tests | 369 (367 pass, 2 ignored) | **371 (369 pass, 0 fail, 2 ignored)** |
| stdio plugin `lib test` target | 914 errors | **913 errors** |
| TypeScript suite | 69 pass / 1,823 `expect()` | **69 pass / 1,823 `expect()`** |
| dependency ledger | 232 entries, 30 test-oracle, 3 production-debt | **identical** |
| comparison profiles weakened | 0 | **0** |
| fixtures swapped to dodge a failure | 0 | **0** (one unit-test example normalised — §3) |

### What should happen next, in order of how much evidence it buys

1. **Fix the five one-line adapter errors in §2.4.** Five cases, roughly 44 scenarios, five lines of
   code, and every one of them has been reported green for the whole ticket.
2. **Make the runner count a case whose host failed to build as `errored`, not as `executed=0
   failed=0`** (`📜️script.ts:497`). Wave 12's remedy #7. Without it, item 1 will happen again.
3. **`mutate-pdf-1-4-a` / `-x`: decide whether the PDF 1.4 subset is a one-page format or whether its
   snapshot must carry the whole document.** Today it silently destroys 64 of 65 pages.
4. **Correct `font_program` to report the decoded program length** and rewrite the sentence in the
   six conformance profiles that calls a compressed byte count writer-invariant. 188 comparisons —
   72 % of all divergences — with proof in §2.2(1) that the font programs themselves are identical.
5. **`write_part21`'s `FILE_DESCRIPTION` header is non-conformant** (`LIST[1:?]`, ISO 10303-21). Six
   cases.
6. **The `xlsx` transitional writer drops `styles` and `theme` relationships; the `docx` transitional
   writer drops `conformance`.** 20 comparisons, real package data loss.
7. **`mutate-dwg-ac1018` / `-ac1024` cannot decode their own fixture** — 14 scenarios, and the only
   phase those cases have.
8. **Add a contract rule for "a case declares `@no-oracle-` and its owning plugin (or its host) does
   not compile."** Wave 12's remedy #8, and §2.3 is what it would have caught.
9. **Re-word every "the os-kernel blocker was cleared" claim to name the feature set** (§5), and give
   `🪐️space` — whose own code is clean — the ten-error `sync` fix that is the cheapest plugin in the
   fleet to unblock.
10. **Make a per-case budget overrun fail that case, not the whole run** (`📜️script.ts:499`,
    `runProbe`). Three attempts at `subject exhaustive --owner 🗄️stdio` were thrown away at case 90-odd
    of 101 by one slow case under a peer session's cargo lock (§1.3). Combined with item 2 the runner
    has two ways to lose a result: silently (host will not build) and catastrophically (one case runs
    long).
