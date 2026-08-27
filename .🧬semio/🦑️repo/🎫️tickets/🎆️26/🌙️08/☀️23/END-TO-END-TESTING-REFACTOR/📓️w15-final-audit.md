# Wave 15 — final audit of the evidence floor

> **THE OWNER'S BAR:** *every single mutation exercised on a REAL-WORLD COMPLEX artifact, with a
> SECOND INDEPENDENT IMPLEMENTATION producing the same result.*

Date 2026-08-26 (08:50–11:15 CEST). HEAD `8d9b51f081f42b36722b54f80a5c502d6322f9ca`
(2026-08-25 15:02:47 +0200) — the same baseline `📓️w14-final-audit.md` measured against — **dirty
tree**, 4,497 working-tree paths. Toolchain `rustc 1.99.0-nightly (c4af71034 2026-07-06)`,
`sccache 0.10.0`. Raw logs and scripts in `w15-audit/` — the six commands' own stdout, the 36-owner
`oracle exhaustive` sweep, the captured failing-scenario records, the fixture and population surveys,
and the scripts that produced every number below.

This audit judges the QUALITY of the evidence, not the size of the totals. Every `[test]` line below
is copied verbatim from the tool's own stdout; every exit code was read from the tool's own exit
status, never through a pipe.

## 0. The most misleading thing a reader would otherwise believe

**That there are five red scenarios. There are thirty-one, and the difference is not noise — it is
the single most valuable output this ticket has produced.**

The brief carries "five". `📓️w14-final-audit.md` §1.2 says five. `📓️w15-specification-defects.md`,
written today, is titled *"the five red scenarios"* and analyses exactly five as though they were the
set. A per-owner `oracle exhaustive` sweep of the 36 non-`🗄️stdio` owners, run here today and
reproduced case by case, reports:

```
sum of 36 `[test] level=exhaustive …` lines — NOT a verbatim tool line; the 36 are in w15-audit/oracle-sweep/
cases=63  executed=2319  passed=2288  failed=31  errored=0  parity=0/0  not-exercised=21
```

Nine owners are red, not two:

```
🧱️block 6    🧩️puzzle 6    🏛️architect 5    📕️norm 4    ✒️writer 3
🗒️note 3     📋️forms 2     🔱️trinity 1     📖️playbook 1
```

**The other twenty-six are not undiscovered — they are documented, in a different note.**
`📓️w22-group-a-second-implementations-2026-08-26.md` §3 analyses them under the heading *"The
twenty-six red scenarios"*, and its per-owner run log (`w22-group-a/🧪️oracle-converted-owners.txt`)
carries figures byte-identical to the ones measured here. So the finding is not that the work was not
done. **It is that this ticket's record has split into notes that each read as complete, and neither
of the two written today states the total.** A reader who opens the note whose title promises the red
scenarios gets five of thirty-one; a reader who opens the other gets twenty-six of thirty-one; the
brief for this audit inherited the five.

**Thirty-one is a floor, not a total.** `🗄️stdio` — 2,177 scenarios, 44% of the repository — could
not be measured today at all, because its oracles are Rust crates and every attempt to build one hit
the contention described below. Whatever `🗄️stdio` is carrying is not in this number.

### What the thirty-one actually are — and it is four causes, not thirty-one findings

Classified from each scenario's own diagnostic, read out of the run reports here:

| cause | red scenarios | cases |
|---|---:|---|
| **A. A composed child's `childId` is a content address no document states** | **16** | `mutate-program-1` (4), `mutate-block-3d-1` (6), `mutate-note-1` (2), `mutate-writer-1` (2), `mutate-en1990-1` (2) |
| **B. The `.dsl.semio` carrier has no usable grammar** | **7** | `identity-round-trip` in `iso16757`, `vdi3805`, `program-1`, `note-1`, `writer-1`, `playbook-1`, `forms-1` |
| **C. A committed vector under-determines the verb it pins** | **7** | `mutate-puzzle-2d-1` (3), `mutate-puzzle-3d-1` (3), `mutate-forms-1` (1) |
| **D. A committed vector is not self-contained** | **1** | `mutate-jack-1 :: spec-vector-create-node` |

**Sixteen red scenarios across five cases in four different plugins are one unpublished function.**
Whatever else this ticket has produced, that is the finding: it is not a test defect, it is a
specification hole with a measured blast radius of **58 `DefaultHasher::new()` call sites in 46
files**, and every artifact in that list whose snapshot embeds a composed child can never meet the
raised bar until it is closed (§6.1).

And Defect B is worse than "the grammar is a placeholder". Three of its seven are not placeholders at
all — `📋️forms` and `📖️playbook` commit a grammar that **describes a different document** (the
generic `family-scene` canvas grammar, `doc-body = schema-line layers-block`, over artifacts that
carry no `layers` block at all — shared verbatim by `📏️layout`, `🖍️draw` and `🖨️raster` too), and
`🗒️note` commits a **real** grammar that covers three of the six block kinds its own vocabulary
declares and names fields the artifact never writes (§6.2).

### The second thing a reader would get wrong: parity is no longer zero, and the blocker is no longer the peer's

Two audits in a row led with `error[E0432]: unresolved import component::component_persistent_local`
and reported repo-wide parity as *blocked by someone else*. **That excuse expired this morning.** The
macro now exists (`💻️os/🔨️modules/🔌️plugin/🦀️component.rs:84-86`, `#[macro_export]`), the module
beside it exists (`:57`), `📦️glue.rs:15`'s `pub use component::component_persistent_local;` resolves,
and this audit watched a generated **Rust subject host compile and run**: `mutate-zip-2-0`'s subject
emitted all fifteen of its scenarios, and a `📕️norm` subject host — a different plugin — got as far
as compiling too (§1.3). It cleared no later than **03:16 today**, the timestamp of the first
per-case parity log with a live subject.

Parity is being measured, one case at a time, by a concurrent session in this same ticket
(`w16-audit/percase/*.txt`, 23 cases with a summary line). Summed: **244 of 246 scenarios compared.**
The two failures are `mutate-bmp-v3 :: mutate-set-pixel-data` and
`mutate-gif-87a :: mutate-set-global-color-table`, both declared `⚠️ KNOWN OPEN DIVERGENCE` in their
own features (§8.4). Cross-checked here: the parity denominator is the case's **full** scenario count,
not its differential count — `mutate-avi-1-0` is 27 scenarios and reports `parity=27/27`,
`mutate-bmp-v3` is 15 and reports `14/15`.

**But read what that 246 is.** It is **5.0% of the repository's 4,933 scenarios**. Eleven of the 23
cases are `create-*` synthesis cases contributing 2–3 scenarios each — cases that build a document
from nothing, which is the opposite of "a real-world complex artifact". Only twelve are `mutate-*`,
contributing 218. It took roughly six hours of wall clock to get that far, one `--case` at a time.

### What stands between this repository and its headline number now is its own test harness

Five of this audit's runs failed today. Four were killed by the 900-second per-case budget while up
to four cargo builds contended over one shared target directory — and because `runProbe` throws on
`ETIMEDOUT` and nothing catches it, each of those four **discarded everything it had already measured
and printed no summary line at all**. The fifth failed on a build artifact another session deleted
mid-link:

| run | wall clock | outcome |
|---|---|---|
| `oracle exhaustive` (repo-wide, attempt 1) | 42 min | ETIMEDOUT on `mutate-csv-rfc4180`'s oracle host — **no summary line** |
| `oracle exhaustive` (repo-wide, attempt 2) | 15 min | ETIMEDOUT on `mutate-avi-1-0`'s oracle host — **no summary line** |
| `parity … --case mutate-en1990-1` | 15 min | ETIMEDOUT on the subject host — **no summary line** |
| `parity … --case mutate-zip-2-0` (attempt 1) | 20 min | ran; the **oracle** host failed to build because a `-L native=…/onig_sys-…/out` path vanished under a concurrent build → `parity=0/0` |
| `parity … --case mutate-zip-2-0` (retry) | 18 min | ETIMEDOUT on the subject host — **no summary line** |

Four of the five name the cause in the harness's own budget message: *"Likely shared cargo target-dir
lock contention from another concurrent session."* **The repo-wide `oracle exhaustive` of §1.2 had to
be replaced by a 36-invocation per-owner sweep** so that one case's timeout would lose one owner
instead of everything. This is w13 remedy #10 / w14 remedy #6, and it has gone from an inconvenience
to **the** binding constraint the moment the real blocker cleared.

### Four further things the headline hides

1. **Half the bar — *a real-world complex artifact* — is unmet for 53% of the mutation evidence, and
   nobody has counted it since the bar was raised.** 56 of the 145 `mutate-*` cases (**2,592 of 4,862
   scenarios**) run on no artifact, on an artifact under 4 KiB, or on a generic
   `📚️examples/🎬️demo/…/🗣️example.dsl.semio` placeholder (§2.3). Ten of the eleven cases with *no*
   artifact got a second implementation this wave and not one of them got a document. And the wave's
   flagship conversion, `mutate-program-1` — 533 scenarios, 10.8% of the repository — runs all of them
   on a 28,538-byte synthetic demo whose project is `"Sample Clinic"` for `"Sample Health"`, code
   `CLN-001`, timestamped at the Unix epoch. Counted in bytes it looks like a real artifact; it is
   schema at scale.
2. **The twelve `🏗️ifc`/`📐️step` cases were not fixed; two new cases were added beside them.** All
   twelve still declare **zero** `@mode-differential` scenarios, because `ruststep 0.4` still has no
   writer. What the wave did — and did very well — is add `differential-ifc-2x3` (8 scenarios) and
   `differential-ifc-4` (15) backed by IfcOpenShell 0.8.4.post1, which genuinely re-serializes through
   its own C++ Part-21 writer (§4). The whole +23 scenarios in the repository is those two cases.
3. **Seventeen of the 28 remaining `@no-oracle-` decisions still inherit the argument their siblings
   falsified** — but they now say so themselves, in the registry, in these words: *"THIS DECISION IS A
   DEBT, NOT A JUDGEMENT"* (§3.1). That is the right correction and it is not the same as fixing it.
4. **The scrub of measured ratios was incomplete and it damaged the prose.** Seven sites still assert
   a measured result, the worst being `mutate-pdf-1-7/🦀️component.rs:19`'s present-tense
   `` `parity` is now 34/37 `` — **five lines below the line the same scrub edited** — and seven of
   the thirteen scrubbed sentences no longer parse as English (§8).

### What genuinely improved, and is not overstated

* **The thirty-one refusals are all refusals BY CLAUSE.** Every one arrives as a `failed` result
  carrying a paragraph that names the document it consulted, quotes the bytes it read, and says what
  would close it — *"Read 269 bytes of the committed artifact and refused to guess their meaning"*.
  None is a crash, a timeout or a silent skip. This is what a second implementation is for, and it is
  the strongest evidence in the repository.
* **The fifteen `📕️norm` oracles are one module now, and the refactor states its own limits.** They
  were not "0.672–0.875 similar" as w14 measured — reproduced independently here from the preserved
  pre-refactor adapters, with docstrings, comments and the per-subset data block stripped, all fifteen
  hash to **one hash** (§5). They are now one 874-line engine imported by fifteen 69–173-line data
  files, whose docstring says: *"A shared bug in a copied oracle agrees with itself in every case that
  shares it, and fifteen copies hide that where one import states it."*
* **`mutate-program-1` converted** — 533 scenarios, 45% of the entire remaining no-oracle surface, and
  w14's remedy #1 (§3).
* **Fifteen conversions plus two new cases — 1,141 + 23 scenarios — and zero scenarios lost anywhere.**
* **A known divergence was closed at the cause, not at the profile.** `mutate-tiff-6-0`'s
  `mutate-insert-ifd` paragraph went from `⚠️ KNOWN OPEN DIVERGENCE` to `✅️ CLOSED, AT THE CAUSE` by
  doing the schema-first change the old paragraph had itself named and declined (§7).
* **`ifcopenshell` was adopted properly** — declared with a real package name and a real pinned
  version, through a shared-manifest edit that also unblocked `📕️norm`'s shared Python module (§4.2).
* **A fifth consecutive audit finds zero weakening** (§7).

---

## 1. The six commands, verbatim

Run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` except #5 (in `📦️packages/🟦️typescript`)
and #6 (in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`).

> **Machine conditions, stated because they determine two of the six.** Throughout this audit a
> concurrent session was running `parity exhaustive --owner 🗄️stdio` one `--case` at a time (pids
> 68682, then 7179), a second was running `📜️script.ts plugin generator`, and a third holds the
> `💻️os/🔨️modules/🔌️plugin` refactor. All of them write the **same**
> `⚡️cache/agents/local/cargo-test-hosts` target directory and the **same** per-case
> `⚡️cache/tests/{work,results}` directories as this audit. By the end only **15** work directories
> survived repo-wide, the rest having been overwritten mid-audit.

### 1.1 `bun ./📜️script.ts contract` — **exit 1**, 2 breaches across 1 rule id

```
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery

  testing/discovery  🧰️framework  44 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  5 executable test file(s) outside the canonical owner-root test tree, baseline allows 1

full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`testing.json` read straight afterwards holds exactly those two records and nothing at a lower
priority. **Breaches by rule id:** the testing domain declares **30** rule ids across six kinds
(enumerated from `📦️index.ts`'s own `breach(...)` call sites: 14 `testing/contract`, 7
`testing/oracle`, 4 `testing/taxonomy`, 2 `testing/discovery`, 2 `testing/fixture`, 1
`testing/dependency`). **Twenty-nine are at zero. One fires:**

| rule id | kind | count |
|---|---|---|
| `unmanaged-tests` | `testing/discovery` | **2** |
| all 29 others | — | **0** |

Byte-identical to w14. Not this ticket and not a lowered ratchet: `./🔒️migration.json` is unchanged
since `a2746cd371` and clean in the working tree; the counts grew, the allowance did not move.

### 1.2 `bun ./📜️script.ts oracle exhaustive` (repo-wide) — **the command did not survive; the same command per owner did**

**Two repo-wide attempts, both exit 1, neither producing a `[test] level=…` line** (§0). Attempt 1
ran 42 minutes and died on `mutate-csv-rfc4180`'s oracle host; attempt 2 ran 15 minutes and died on
`mutate-avi-1-0`'s. Both ended the same way, and the harness itself names the cause:

```
[budget] cargo run --quiet --manifest-path …-oracle-rust/Cargo.toml -- --plan … exceeded 900000ms — killed.
         Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying.
error: spawnSync cargo ETIMEDOUT
      at runProbe (…/📚️library/📦️packages/🟦️typescript/📦️index.ts:1699:18)
      at executeOne (…/🧪️test/📜️script.ts:529:17)
```

Raw: `w15-audit/02-oracle-repowide.txt`, `02-oracle-repowide-attempt2.txt`.

**So the same command was run scoped, one owner at a time, over all 36 non-`🗄️stdio` owner groups**
(`w15-audit/sweep.sh`, per-owner logs in `w15-audit/oracle-sweep/`). Every one of the 36 produced a
summary line — none needed cargo, because every non-`🗄️stdio` oracle in this repository is Python or
TypeScript. Aggregated:

```
sum of the 36 lines — NOT a verbatim tool line; each owner's own line is in w15-audit/oracle-sweep/
cases=63  executed=2319  passed=2288  failed=31  errored=0  parity=0/0  not-exercised=21
```

| owner | cases | executed | passed | failed |
|---|---:|---:|---:|---:|
| `📕️norm` | 15 | 799 | 795 | **4** |
| `🏛️architect` | 1 | 533 | 528 | **5** |
| `🧱️block` | 3 | 237 | 231 | **6** |
| `🧩️puzzle` | 3 | 181 | 175 | **6** |
| `🏗️fem` | 2 | 152 | 152 | 0 |
| `🌀️procedural` | 3 | 77 | 77 | 0 |
| `🗒️note` | 1 | 67 | 64 | **3** |
| `🔱️trinity` | 2 | 47 | 46 | **1** |
| `🌍️gis` | 2 | 44 | 44 | 0 |
| `📐️cad` | 1 | 41 | 41 | 0 |
| `🖨️raster` | 1 | 37 | 37 | 0 |
| `💠️lowpoly` | 1 | 35 | 35 | 0 |
| `📋️forms` | 1 | 21 | 19 | **2** |
| `📖️playbook` | 1 | 19 | 18 | **1** |
| `🪵️sourcing` | 1 | 10 | 10 | 0 |
| `✒️writer` | 1 | 9 | 6 | **3** |
| `🖱️ui` / `🎠️kernel` | 5 | 10 | 10 | 0 |
| 19 no-oracle-only owners | 20 | 0 | 0 | 0 (`not-exercised=21`) |

**`🗄️stdio` — 2,177 scenarios, 44% of the repository — could not be measured today**, because its
oracles are Rust crates and every attempt to build one hit the contention above. That is the one gap
in this audit's coverage and it is stated rather than papered over.

**The population checks out exactly.** The repository's own `discoverTestCases`/`parseFeature` put
**63 cases and 2,756 scenarios** outside `🗄️stdio`, of which **42 cases / 2,319 scenarios** are
oracle-backed and **21 cases / 437 scenarios** carry a recorded no-oracle decision. The sweep reports
`cases=63`, `executed=2319` and `not-exercised=21` — every oracle-backed scenario ran, and every
no-oracle case is accounted for as un-exercised, with nothing missing and nothing double-counted.

**Two things the aggregate must not be read as.** First, `parity=0/0` on every line is by
construction: `oracle exhaustive` executes the reference side alone and forms no comparison — the
number that answers the raised bar is §1.3's. Second, a per-owner sweep is not identical to one
repo-wide invocation: it re-reads the registry 36 times and writes 36 reports, and it cannot see a
cross-owner problem. It is the same command, the same level and the same population; it is not the
same run.


### 1.3 `bun ./📜️script.ts parity exhaustive` — **unmeasurable today, and not for the reason the last two audits gave**

**Repo-wide parity could not be measured.** Not because no subject host links — subject hosts link now
— but because every attempt was killed by the shared-target-dir contention described above. Reported
as unmeasurable rather than as a number.

**Probe 1 — `parity exhaustive --owner 🗄️stdio --case mutate-zip-2-0`, exit 1.** This is the run that
proves the blocker cleared: the **subject** host compiled and emitted all fifteen scenarios; it is the
**oracle** host that failed, and it failed on a build-cache artifact, not on a compile error in this
repository:

```
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
[test] ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/mutate-zip-2-0: rust oracle host exited 101 without emitting results
error: could not compile `semio-test-host-mutate-zip-2-0` (bin "host")
Caused by:
  could not execute process `sccache …rustc --crate-name host …-L 'native=…/cargo-test-hosts/debug/build/onig_sys-68d65bfd7fa1f850/out'` (never executed)
Caused by:
  No such file or directory (os error 2)
[test] ✏️s/…/mutate-zip-2-0: no result stream at …/mutate-zip-2-0-oracle-rust/📤️results.jsonl
[test] no oracle result to compare against: …::mutate-zip-2-0::mutate-no-mutation::rust::subject
   … (fifteen such lines, one per scenario, all `::rust::subject`)
```

`sccache 0.10.0` is installed and on `PATH` (`/Users/ueli/.local/bin/sccache`); what vanished is the
`onig_sys` native output directory inside the shared `cargo-test-hosts` target dir, removed by another
session's build while this one was still linking against it.

**Note the shape of that summary line.** `executed=15 passed=15 failed=0 errored=0 parity=0/0` reads
green for a case that formed **zero** comparisons, and the only signals are the trailing `problems`
lines and the exit code. That is w12 remedy #7 / w13 remedy #2, still open at `📜️script.ts:520-541`.

**Probes 2–4 — killed at the 900 s budget, no summary line at all.** `--owner 📕️norm --case
mutate-en1990-1`, and a retry of `--owner 🗄️stdio --case mutate-zip-2-0`, both ended:

```
[budget] cargo run --quiet --manifest-path …-subject-rust/Cargo.toml --features sut -- --plan … exceeded 900000ms — killed.
         Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying.
error: spawnSync cargo ETIMEDOUT
      at runProbe (…/📚️library/📦️packages/🟦️typescript/📦️index.ts:1699:18)
      at executeOne (…/🧪️test/📜️script.ts:529:17)
      at runPhases (…/🧪️test/📜️script.ts:593:25)
```

The en1990 probe's subject host is a `📕️norm` host — a **different plugin** from `🗄️stdio`, and it
too got as far as compiling rather than failing on `component_persistent_local`. Nothing in this
repository refused to build today.

**Two parity numbers this audit measured itself, on the owners whose subjects are TypeScript and
therefore need no cargo at all — both exit 0:**

```
$ bun ./📜️script.ts parity exhaustive --owner 🖱️ui        # exit 0
[test] level=exhaustive cases=3 executed=17 passed=17 failed=0 errored=0 parity=7/7

$ bun ./📜️script.ts parity exhaustive --owner 🎠️kernel     # exit 0
[test] level=exhaustive cases=2 executed=7 passed=7 failed=0 errored=0 parity=3/3
```

Ten comparisons, all equal. They are small and they are not the raised bar — no mutation, no
real-world artifact — but they establish that the platform itself forms and reports comparisons
correctly today, so the failures above are the cargo path and nothing else.

**Corroborating evidence from a concurrent session, attributed as such.** A peer session in this same
ticket has been running the same command one `--case` at a time since 03:16 and its per-case logs are
in `w16-audit/percase/`. Twenty-three carry a summary line; summed, **244/246**. Verbatim samples:

```
mutate-dxf-r12       [test] level=exhaustive cases=1 executed=78 passed=78 failed=0 errored=0 parity=39/39
mutate-bcf-2-1       [test] level=exhaustive cases=1 executed=58 passed=58 failed=0 errored=0 parity=29/29
mutate-docx-ecma-376 [test] level=exhaustive cases=1 executed=54 passed=54 failed=0 errored=0 parity=27/27
mutate-avi-1-0       [test] level=exhaustive cases=1 executed=54 passed=54 failed=0 errored=0 parity=27/27
mutate-gif-87a       [test] level=exhaustive cases=1 executed=50 passed=49 failed=1 errored=0 parity=24/25
mutate-bmp-v3        [test] level=exhaustive cases=1 executed=30 passed=29 failed=1 errored=0 parity=14/15
mutate-binary-raw    [test] level=exhaustive cases=1 executed=20 passed=20 failed=0 errored=0 parity=0/0   (no-oracle: expected)
extract-text-pdf-1-4 [test] no-subject-implementation … (adapters python host references only)
                     [test] level=exhaustive cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0
```

These are another session's measurements and are reported as such; what this audit verified
independently is that the denominators equal the cases' full scenario counts, and that both failures
are declared divergences (§8.4).

### 1.4 `bun ./📜️script.ts dependency` — **exit 0**

```
[dependency] ecosystems=4 entries=233 production-reachable=151 test-oracle=31
[dependency] test-oracle python:ifcopenshell@0.8.4.post1 (ifcopenshell-ifc-2x3-any-differential,ifcopenshell-ifc-4-any-differential)
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/…/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

**`entries=232 → 233`, `test-oracle=30 → 31`, and the diff against w14's log is exactly one line: the
`ifcopenshell` registration.** After four waves of a byte-identical figure, the number finally moved —
and it moved for the one dependency this wave declared honestly. The three `production-debt` records
are unchanged. Full listing in `w15-audit/04-dependency.txt`.

### 1.5 `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — **exit 1**

```
 65 pass
 4 fail
 2058 expect() calls
Ran 69 tests across 1 file. [192.45s]
```

69 tests, unchanged; `expect()` calls 2,057 (w14) → **2,058**. Two failures are §1.1's breach
surfacing inside the suite. **Two are new, and both are the same real defect:**

```
(fail) 🔒️ dependency ratchet > the committed baseline classifies every ecosystem it tracks and keeps oracles out of production
  error: ifcopenshell is linked by oracle ifcopenshell-ifc-2x3-any-differential but is absent from the dependency baseline
(fail) 🧩️ cross-language oracle hosts > the committed baseline classifies every external host package as a test-only dependency
  error: python:ifcopenshell is on a generated host's import path but is absent from the dependency baseline
```

`🔒️dependencies.json` was not regenerated after the adoption: it holds **232** entries with no
`ifcopenshell`, against the live scan's 233. It is also stale on `png` — `0.17.16` in the baseline
against `0.18` in both `Cargo.toml` and the oracle registration. **These two failures are a gate doing
its job**, and they matter for §5.3: the same gate is structurally blind to Pillow and three.js.

### 1.6 `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — **exit 0**

```
running 374 tests
test result: ok. 372 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 173.24s
```

Identical to w14: 374 / 372 / 2 ignored (the same one-shot `#[ignore]`d `bmp v3` and `tiff 6.0`
fixture-derivation helpers, not skipped assertions). The Rust oracle crate was untouched by this wave,
whose fifteen conversions are all Python.

---

## 2. Fixture complexity — the `mutate-*` population, case by case

Measured with the repository's own `fixtureUrisIn` + `resolveFixtures`, run from the repository root
so paths resolve, and stat-ed on disk (`w15-audit/fixtures.ts` → `fixtures.json` →
`fixture-rows.json`). A fixture whose resolved path passes through `🧬️mutations/<kind>/…/🧪️tests/`
is a **specification vector** — a handcrafted `(before, mutation, after, outcome)` quartet, a third
statement of the verb's meaning — and is NOT counted as a real-world artifact. **No fixture is
missing anywhere in the repository (0 unresolved URIs) and no fixture resolves to a zero-byte file.**

The full 145-row table is `w15-audit/mutate-fixture-table.md` (and `.tsv`), one row per `mutate-*`
case: bytes, scenario count, case, largest real artifact URI, oracle or no-oracle decision. The
distribution:

| largest real artifact the case reads | cases | scenarios |
|---|---:|---:|
| **none at all** (specification vectors only) | **11** | **497** |
| < 1 KiB | 26 | 885 |
| 1–4 KiB | 12 | 469 |
| 4–16 KiB | 17 | 668 |
| 16–64 KiB | 10 | 716 |
| 64 KiB – 1 MiB | 50 | 1,164 |
| ≥ 1 MiB | 19 | 463 |
| **total `mutate-*`** | **145** | **4,862** |

**The two numbers the brief asked for.**

* **`mutate-*` cases running on under 4 KiB or on nothing: 49 cases / 1,851 scenarios.** Of those,
  **38 cases / 1,354 scenarios** read a real artifact that is under 4 KiB, and **11 cases /
  497 scenarios** read no real artifact at all.
* **Repo-wide cases that read no real artifact at all: 27 cases / 538 scenarios — unchanged from
  w14.** The eleven `mutate-*` members are unchanged too, and that is the finding: *every one of the
  eleven got a second implementation in this wave and not one of them got an artifact.*

```
   83 sc  mutate-block-5d-1        205 vectors, 0 artifacts   ORACLE block-5d-python-independent
   75 sc  mutate-block-3d-1        185 vectors, 0 artifacts   ORACLE block-3d-python-independent
   71 sc  mutate-puzzle-3d-1       175 vectors, 0 artifacts   ORACLE puzzle-3d-python-independent
   57 sc  mutate-puzzle-5d-1       140 vectors, 0 artifacts   ORACLE puzzle-5d-python-independent
   53 sc  mutate-puzzle-2d-1       130 vectors, 0 artifacts   ORACLE puzzle-2d-python-independent
   41 sc  mutate-cad-1             100 vectors, 0 artifacts   ORACLE cad-python-independent
   35 sc  mutate-lowpoly-1          85 vectors, 0 artifacts   ORACLE lowpoly-python-independent
   29 sc  mutate-procedural-2d-1    70 vectors, 0 artifacts   ORACLE procedural-2d-python-independent
   29 sc  mutate-procedural-3d-1    70 vectors, 0 artifacts   ORACLE procedural-3d-python-independent
   19 sc  mutate-assembly-1         45 vectors, 0 artifacts   ORACLE assembly-python-independent
    5 sc  mutate-os-config-opening   0 fixtures               no-oracle
```

Ten of the eleven moved from "one implementation, no artifact" to "two implementations, no
artifact". That is half the bar met and the other half untouched, and the wave's own notes do not
say so.

### 2.1 What genuinely improved, measured against w14's own fixture survey

Comparing `w14-audit/fixtures2.json` to `w15-audit/fixtures.json` case by case
(same script, same classifier):

* **No case's largest artifact shrank. Not one.** Zero fixtures deleted, zero renamed — and the
  eleven upgraded cases **added** their new artifact rather than replacing the old one: every URI each
  of them resolved at w14 still resolves today (checked per case, `old→new` fixture-set sizes 1→2,
  2→3, 2→4, 10→11, 11→12). Nothing was swapped for something the caveat happens to fit.
* **Eleven cases got a substantially larger real artifact**, and two new cases arrived with big ones:

```
  mutate-rewrite-1          2,454 →   246,269   local://♻️nakagin-capsule-tower.snapshot.json
  mutate-semio-video        1,815 →   220,106   local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio
  mutate-svg-1-1-basic      1,463 →   138,219   shared://🎨️semio-brand-and-onboarding.svg
  mutate-semio-graph          297 →   131,964   local://🗣️nakagin-capsule-tower.dsl.semio
  mutate-gif-87a            2,936 →   117,704   shared://🖼️dancing-87a-large.gif
  mutate-xml-1-0              747 →    92,873   shared://📰️ooxml-readme-document.xml
  mutate-semio-brep           537 →    90,063   local://🗣️hexagonal-cut-concrete-forest-left.dsl.semio
  mutate-semio-kit             734 →    78,066   local://🗣️nakagin-capsule-tower.dsl.semio
  mutate-semio-audio        1,145 →    72,341   local://🗣️bauen-mit-bestand-ausschnitt.dsl.semio
  mutate-semio-text           203 →    70,816   local://🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio
  mutate-xml-1-0-valid        631 →    40,440   shared://📰️reuse-marketplaces-plist.xml
  differential-ifc-4          NEW  → 2,496,437  shared://🏗️nakagin-capsule-tower.ifc
  differential-ifc-2x3        NEW  →   193,915  shared://🏗️wellness-center-sama-street-level.ifc
```

  These are genuine derivations from real committed material, documented in place. `mutate-semio-text`
  is the model: its 70,816-byte DSL was derived once by `w22-fixture-upgrade/🐍️derive-text-fixture.py`
  from `🌐️zukunft-bau-entwerfen-mit-bestand.html`, the real 150 KB TYPO3-published German page
  already committed as this repository's own HTML 5 fixture, read with Python's stdlib `html.parser`;
  384 runs and 344 marks are real text nodes and real `<a href>`s of that page. The feature states the
  limit of the source rather than hiding it (the page uses only `<strong>` and `<a>`, so the artifact
  carries only the `bold` and `link` arms).

* **But only two cases in the whole repository re-derive their fixture on every run** —
  `mutate-semio-table` and `mutate-semio-value`, through a `payload-fidelity` scenario. The eleven
  upgrades above are one-shot derivations with the script in the ticket folder and no tripwire, so
  they can drift from the real source silently. That is the difference between a real artifact and a
  real artifact you can still trust in six months.

### 2.2 Seventeen zero-byte committed example assets, unchanged for a third audit

12 × `🎒️example.pack.semio` plus `🎞️example.gif`, `🎞️example.pptx`, `💬️example.bcf`,
`📕️example.xlsx`, `📷️example.png` are committed at **0 bytes**. No case resolves a fixture to any of
them, so no law is passing on empty bytes — but they are dead files shipped as examples and nothing
reports them. Flagged at w14 §4.4; untouched.

---

### 2.3 Size is not the same as real, and the largest conversion in the wave shows it

The bucket table above flatters the surface, because it measures bytes. **28 of the 134 `mutate-*`
cases that read an artifact at all (1,393 scenarios, 28.6% of that population) read a generic
`📚️examples/🎬️demo/…/🗣️example.dsl.semio` — the subset's own schema-shaped placeholder, not a
document anybody made.**

The largest is the wave's flagship. `mutate-program-1` — 533 scenarios, 10.8% of the repository, the
conversion this audit praises most in §3 — runs every one of them on a **28,538-byte synthetic demo**:

```
meta { schema=architect.program document-id=document-3 title="Sample Clinic"
       industry-sector=healthcare project-type="" locale=en revision="0" purpose=text=""
       terminology=[ ] classification=[ ] author-ids=[ ]
       timestamps=created="1970-01-01T00:00:00Z" updated="1970-01-01T00:00:00Z" }
project { id=project-1 code=CLN-001 client-name="Sample Health" owner-organization=""
          funding-model="" brief-summary=text="" problem-statement=text="" …
```

`"Sample Clinic"`, `"Sample Health"`, `CLN-001`, epoch timestamps, empty strings throughout. It is
28 KB of schema at scale, which is a genuinely useful thing to exercise 266 verbs against — and it is
the exact opposite of *"a REAL-WORLD COMPLEX artifact"*. Counted by bytes it lands in the 16–64 KiB
bucket and looks like progress; counted by what the bar asks, it is in the same category as the eleven
cases with no artifact at all.

Seven of the 28 demo-backed cases are above 4 KiB and therefore escape the size table entirely:
`mutate-program-1` (533 sc, 28,538 B), `mutate-draw-1` (29, 31,179 B), `mutate-layout-1` (51, 4,625 B),
`mutate-iso16757-1` (43, 4,128 B), `mutate-process3d-1` (33, 12,438 B), `mutate-epw-energyplus`
(27, 6,124 B), `mutate-jack-1` (25, 10,742 B) — 741 scenarios.

Combining the two criteria: **`mutate-*` cases whose evidence rests on no artifact, an artifact under
4 KiB, or a generic demo placeholder: 56 cases / 2,592 of 4,862 scenarios — 53%.** More than half of
this repository's mutation evidence still does not run on a real-world document, and no number in any
wave note says so.

---

## 3. `@no-oracle-`: 43 → 28 cases, 1,719 → 578 scenarios

Measured with the repository's own `discoverTestCases` + `parseFeature` (`w15-audit/survey.ts` →
`survey.json`), not by grep. Full table with substitutes: `w15-audit/no-oracle-table.tsv`; full
rationale text: `w15-audit/no-oracle-rationales.txt`.

| | w13 | w14 | **now** |
|---|---|---|---|
| cases | 164 | 164 | **166** |
| scenarios | 4,564 | 4,910 | **4,933** |
| `@oracle-` cases / scenarios | 79 / 1,331 | 121 / 3,191 | **138 / 4,355** |
| `@no-oracle-` cases / scenarios | 85 / 3,233 | 43 / 1,719 | **28 / 578** |
| oracle registrations | 80 | 122 | **139** (138 claimed, 1 orphan `csv`) |
| recorded no-oracle decisions | — | 44 | **29** (28 claimed, 1 orphan) |

Fifteen cases converted (1,141 scenarios), and the biggest was the one w14 named as remedy #1:
**`mutate-program-1`, 533 scenarios, 45% of the entire remaining no-oracle surface, now carries
`architect-program-python-independent`** — a 788-line Python reference written from the 266 committed
payload shapes, the snapshot register list, three of `📓️derivation-rules.md`'s rules and all 266
committed vectors, replayed offline before registration. It reproduces Defect A independently (§6)
and it found a new discrepancy on its own: *"the snapshot's 47th register is serialized `artifacts`
even though the committed JSON Schema requires `documents`."*

### 3.1 For each of the 28: argued from a clause, or inherited?

**17 cases / 425 scenarios inherit. 11 cases / 153 scenarios argue from a specific clause.**

The seventeen all carry the identical paragraph — *"THAT SURVEY STANDS, AND IT IS NOT WHAT THIS
DECISION NOW RESTS ON. Surveying and declining a third-party LIBRARY is a different judgement from
declining a SECOND IMPLEMENTATION … The sentence it used to lean on … is refuted inside this
repository"* — and all seventeen then say, in these words, **"THIS DECISION IS A DEBT, NOT A
JUDGEMENT"**. Seventeen of the twenty-eight also name what a second implementation would be written
from (18 of the 28 rationales do) and the nearest transferable recipe (17 of 28). Mean rationale
length is 2,719 characters — these are arguments, not one-liners.

```
TEMPLATE + self-labelled DEBT   argued from a specific clause
   71  mutate-remodel-1           43  mutate-semio-any
   63  mutate-shooting-1          24  mutate-txt-utf-8
   51  mutate-layout-1            21  mutate-jpg-jfif-1-01-baseline
   33  mutate-process3d-1         20  mutate-binary-raw
   31  mutate-mathematical-1      19  mutate-tiff-6-0-baseline
   29  mutate-dag-1                7  mutate-dwg-ac1018
   29  mutate-draw-1               7  mutate-dwg-ac1024
   21  mutate-flow-1               5  mutate-os-config-opening
   21  mutate-wires-1              3  merge-conflicting-utilities
   19  mutate-present-1            3  host-protocol-parity
   17  mutate-sequence-1           1  reject-malformed-version-input
   13  mutate-vcs-1
    9  mutate-imperative-1        11 cases / 153 scenarios
    9  mutate-s-space-1
    3  mutate-playground-1
    3  mutate-energy-model-1
    3  mutate-s-home-1
17 cases / 425 scenarios
```

**This is the right correction and it is not the same as fixing it.** w14's complaint was that 34
decisions read as verdicts when they were absences. They no longer do: every one of the seventeen now
states, in its own registry entry, that it is a debt and names the sibling case that refuted its old
argument, and most of them name the committed documents a reference would be written from and the
nearest transferable recipe. What has not
changed is that the reference does not exist. A reader who takes "28 no-oracle cases" as "28 formats
that genuinely have no second producer" is wrong about seventeen of them, and now the registry says so
out loud — which is exactly the improvement, and exactly the limit of it.

The eleven that argue from a clause are the same population w14 judged correct, minus the ones that
converted: `mutate-dwg-*` (LibreDWG is GPL-3.0 C, `dxf 0.6` reads DXF and explicitly not DWG),
`mutate-jpg-jfif-1-01-baseline` and `mutate-tiff-6-0-baseline` (the vocabulary addresses frame-header
and IFD class-membership axes `image 0.25`'s API cannot express, cited by ITU-T T.81 and TIFF 6.0
clause), `mutate-binary-raw` (a raw buffer has no grammar), `mutate-semio-any` (the envelope's own
committed grammar declares the wrapped arm OPAQUE, so an envelope-level reference can reproduce both
carriers byte for byte and still say nothing about a delegated verb), and the four non-format cases.

### 3.2 The weak point inside the "correct" eleven is unchanged

`mutate-binary-raw` and `mutate-txt-utf-8` still name, as their substitute, *"an independently
hand-written reference implementation … in this subset's own oracle module"* — our code, our
language, our crate, our authors. Both are honestly typed `@no-oracle-` so no false differential is
claimed, and neither decision says the thing out loud: under the raised bar that is not a second
producer.

### 3.3 One orphan on each side, still unreported by any rule

`csv` (oracle registration) and `os-config-merge-policy-unmounted-facet` (no-oracle decision) are
declared and claimed by nothing. The contract checks the reverse direction only (`unknown-oracle`,
`unknown-no-oracle-decision`), so a decision can outlive the case that justified it and nothing
notices. Unchanged from w14 §2.3.

---

## 4. The twelve `🏗️ifc`/`📐️step` cases still declare ZERO differential scenarios

This was the brief's third question and the answer is a plain **no — they were not fixed, they were
side-stepped, and the side-step is excellent.**

```
differential-ifc-2x3     8 sc   modes: differential          ifcopenshell 0.8.4.post1   NEW CASE
differential-ifc-4      15 sc   modes: differential          ifcopenshell 0.8.4.post1   NEW CASE
mutate-ifc-2x3          11 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-ifc-2x3-cobie    15 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-ifc-2x3-cv20     13 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-ifc-2x3-sav      13 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-ifc-4            23 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-step-ap214       23 sc   modes: property, round-trip  ruststep 0.4               unchanged
mutate-step-ap214-cc1…6 74 sc   modes: property, round-trip  ruststep 0.4               unchanged
```

All twelve keep zero `@mode-differential` scenarios, because `ruststep 0.4` still has no writer. What
the wave did instead was add two **new** cases beside them. The repository went 164 → 166 cases and
4,910 → 4,933 scenarios, and **the entire +23 is those two cases** — no existing case gained or lost
a single scenario.

### 4.1 Is `ifcopenshell` genuinely writing? Yes — verified in the adapter, not in the prose

`differential-ifc-4/🐍️component.py:484` and `:639` both end in `model.to_string().encode("utf-8")` —
IfcOpenShell's own C++ Part-21 writer re-serializing the whole exchange structure. The mutated bytes
are then read back by a from-scratch ISO 10303-21 reader written in the same file from clause 6
(`§6.4.2` control directives, `§6.2` doubled apostrophe) and clause 8 (`§8.2.2`/`§8.2.3` header
attribute order), and `semantic-ifc-v1` — the **same** profile the `ruststep` siblings use, not a
looser one — compares the two sides. The fixture is `shared://🏗️nakagin-capsule-tower.ifc`, the real
**2,496,437-byte, 24,792-entity** `FILE_SCHEMA(('IFC4'))` export. Roles are clean: the Rust adapter
registers 3 × `.subject(` and no oracle; the Python adapter registers 3 × `.oracle(` and no subject.
Observability is asserted on the oracle side (`observable()`, `🐍️component.py:558`) and
non-pass-through on the subject side, so a kind whose parameters no-op cannot pass.

The header is unusually honest about what IfcOpenShell *cannot* do, each point confirmed against this
exact fixture rather than assumed: `set-entity-name` is refused on creation and a hand-written file
carrying the retype reads back **16,975 of 24,792** entities with no error raised; `insert-entity-arg`
raises `IndexError` and a hand-written file silently drops the tenth argument; `remove-entity-arg`
cannot reduce arity; and `ifcopenshell.file.remove` performs reference repair while
`IfcMutation::RemoveEntity` deliberately does not — *"two implementations of two different verbs are
not a differential"*. Those four keep their `ruststep`-backed scenarios in `../mutate-ifc-4`
unchanged. The removal primitive is used only as the inverse of `insert-entity`, behind an explicit
`get_total_inverses(...) == 0` guard.

### 4.2 The governance rule w14 called "one manifest edit" was lifted — properly

`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json` gained exactly six lines
(`{"implementation": "python", "package": "ifcopenshell", "version": "0.8.4.post1", "module":
"ifcopenshell"}`), and both new registrations declare `"ecosystem": "python", "package":
"ifcopenshell", "version": "0.8.4.post1"` — a real package name and a real pinned version, not the
`"package": ""` convention. The same mechanism was used to give `📕️norm` its shared Python module
through its own plugin-root `oracleHostPackage`.

**But the dependency baseline was not regenerated, and that is two live red tests** (§1.5). `entries`
went 232 → 233 in the live scan and stayed 232 in the committed `🔒️dependencies.json`.

---

## 5. Oracle diversity — the fifteen `📕️norm` oracles were refactored, and the refactor is the best piece of work in the wave

The brief's fourth question. w14 measured the fifteen at 0.672–0.875 mutual 5-gram Jaccard and called
them "one generic verb engine instantiated fifteen times". That was generous, and this audit
**reproduced the stronger measurement independently** rather than taking the wave's word for it — the
fifteen pre-refactor adapters are preserved at `w15-work/old-adapters/`, so strip each one's
`# region 🔖️Vocabulary … # endregion 🔖️Vocabulary` data block, its docstrings and its comments, and
hash what is left:

```
f523f953027267ad  15 files  [din16798, din18599, din4108, en1990, en1991, en1992, en1993,
                             en1994, en1995, en1996, en1997, en1998, en1999, iso16757, vdi3805]
distinct engines among the 15 OLD adapters: 1
```

**One hash, fifteen files.** They were not similar; outside the per-subset data they were the same
~800 lines fifteen times. The reference surface read as fifteen independent implementations and was
one.

They are now **one named module**, `✏️s/🔌️plugins/📕️norm/🧪️oracle/📦️packages/🐍️python/semio_norm_vocabulary.py`
(874 lines), imported by fifteen adapters that shrank to **69–173 lines each** and contain nothing but
data: `KINDS`, `VECTORS`, `DSL_ASSET`, `ENVELOPE`, and a three-line `adapter()` that calls
`build_adapter(Subset(...))`.

Re-measured here with the same method w14 used (5-gram Jaccard, docstrings stripped,
`w15-audit`-side script):

| group | files | pairs | max | median | min |
|---|---:|---:|---:|---:|---:|
| the fifteen `📕️norm` adapters | 15 | 105 | **0.244** | 0.085 | 0.042 |
| the seventeen `🧿️semio` adapters | 17 | 136 | 0.454 | 0.106 | 0.053 |
| all 61 Python adapters | 61 | 1,830 | 0.958 | — | — |

And the module says the thing that matters out loud, in its own docstring:

> *"Writing them out fifteen times would have produced fifteen files whose engines were byte-identical
> … and it made the reference surface look fifteen times larger than the evidence it carries. **A
> shared bug in a copied oracle agrees with itself in every case that shares it**, and fifteen copies
> hide that where one import states it. So: one engine, one bug surface, declared."*

It also states the honest boundary on the carrier — that `.dsl.semio` has no published grammar, that
`identity_handler` therefore reads the artifact at the carrier level and PINS the reading by
re-emitting each file byte for byte, that it deliberately does not map carrier tokens onto snapshot
enum spellings because that mapping is stated nowhere, and that it **refuses** `iso16757` and
`vdi3805` rather than guessing (§6, Defect B). That refusal is a `raise AssertionError`, so the two
scenarios stay red; it was not converted into an expected outcome.

### 5.1 The honest count of distinct second implementations

61 committed Python adapter files. Clustering at ≥ 0.60 Jaccard and collapsing the fifteen norm files
(which share one module by construction) gives **43 clusters**. Of those, one cluster wraps
`ifcopenshell` (`differential-ifc-2x3` + `differential-ifc-4`), two singletons wrap `pypdf` and
`simplejson`, and one singleton (`host-protocol-parity`) is a subject adapter rather than an oracle.
**Distinct in-repo second implementations: 39 Python + 1 TypeScript (`mutate-semio-mesh`) = 40**,
standing behind 57 registrations that declare `package: ""`. w14's honest count was ≈ 27 out of 42
files. The surviving multi-file clusters are all declared as siblings in their own headers:

```
  0.958  mutate-procedural-2d-1 :: mutate-procedural-3d-1   (declared; the 3d header lists the three
                                                             differences from the 2d sibling by name)
  0.832  mutate-fem2d-1 :: mutate-fem3d-1                   (declared 2D/3D sibling pair)
  0.737  differential-ifc-2x3 :: differential-ifc-4         (declared: "a deliberate, stated
                                                             duplication … rather than a shared module",
                                                             with the reason)
  0.713  mutate-forms-1 :: mutate-playbook-1                (declared cross-case divergence)
```

### 5.2 Mechanical independence — clean across all 68 adapter files

Over every committed `🧪️tests/**/🐍️component.py` (61) and `🟦️component.ts` (7), with docstrings and
comments stripped: **zero** hits for `subprocess`, `os.system`, `popen`, `ctypes`, `cffi`, `dlopen`,
`importlib`, `child_process`, `execSync`, `spawnSync`, and **zero** imports of any semio production
module (the only `semio_*` imports are the host facade `semio_repo_test` and the declared
`semio_norm_vocabulary`). Every Python adapter is oracle-only — the single exception is
`host-protocol-parity`, which is a five-implementation cross-subject case by design, so the reference
can never become its own subject.

The complete third-party import census across all adapters is `PIL` (1), `ifcopenshell` (2), `pypdf`
(1), `simplejson` (1), `semver` (1), `three` (1).

---

### 5.3 Two genuinely third-party references that still no manifest declares — and the gate that would catch them is blind by construction

This is w14 §3.3, unchanged in substance and now sharper, because the missing rule has a working
counterpart.

* **Pillow.** `mutate-semio-image`'s reference does `from PIL import Image` (`🐍️component.py:520`).
  Its oracle id was renamed to `semio-image-python-pillow-independent` — it now *names* Pillow — but
  its registration still declares `"ecosystem": "python", "package": ""`, with no version, and
  **Pillow appears nowhere in `🔒️dependencies.json`**. It resolves out of the developer's own user
  site-packages because the runner's venv is created `--system-site-packages`. On a devcontainer, on
  CI, on a second machine, that import raises `ModuleNotFoundError` and the case's oracle half
  produces nothing.
* **three.js.** `mutate-semio-mesh`'s reference does `import * as THREE from "three"`
  (`🟦️component.ts:48`) and builds real `THREE.BufferGeometry` objects. Its registration is
  `semio-mesh-typescript-three-independent`, also `"package": ""`. The adapter's own prose says
  **r185** (`🟦️component.ts:13`); what resolves is `node_modules/three` at **0.182.0** — r182.
  `three` *is* in `🔒️dependencies.json`, declared `"kinds": ["production-runtime"]`, `"^0.182.0"`,
  used by five `package.json` files — so an oracle reads a production library, which is exactly the
  `production-debt` pattern the gate reports for `png`, `zip` and `image`, and it reports nothing.

**Why it reports nothing is now demonstrable rather than inferred.** The rule exists and it works: it
fired this morning on `ifcopenshell` in two TypeScript tests (§1.5). It is keyed on
`oracleLinkedPackages` (`📦️index.ts:538-543`), whose body is

```ts
for (const linked of [primary, ...(entry.packages ?? [])])
  if (linked.package.length > 0 && !byName.has(linked.package)) byName.set(linked.package, linked);
```

— a registration declaring `package: ""` contributes **nothing at all**. **57 of the 138 oracle
registrations in use declare an empty package.** For 55 that is the honest "second implementation, no
distribution" convention. For these two it hides a real third-party dependency from the only gate that
would have caught it, and there is still no rule for *"an oracle adapter imports a distribution no
manifest declares"* — the check would have to read the adapter's imports, and `importProbe`
(`📦️index.ts:1631`) only ever scans for names the registration already gave it.

---

## 6. The red scenarios — thirty-one, not five, and four causes behind them

The brief asks whether the five specification-defect scenarios are still failing for their stated
reasons. **All five are. And they are five of thirty-one.**

The five, re-measured here directly and scoped so the repo-wide run's abort could not lose them, exit
status read from the tool:

```
$ bun ./📜️script.ts oracle exhaustive --owner 📕️norm                          # exit 1
[test] level=exhaustive cases=15 executed=799 passed=795 failed=4 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🔱️trinity --case mutate-jack-1   # exit 1
[test] level=exhaustive cases=1 executed=25 passed=24 failed=1 errored=0 parity=0/0
```

| # | case :: scenario | the second implementation's own words |
|---|---|---|
| 1 | `mutate-en1990-1 :: mutate-insert-variable-action` | *"the committed vector declares this mutation applied, yet this implementation refused it: `insert-variable-action` would seed the composed child slot `'qK'`, whose childId is content-addressed by a function no specification in this repository states"* |
| 2 | `mutate-en1990-1 :: inverse-insert-variable-action` | *"the forward mutation could not be applied to its own committed before-snapshot: …"* (same cause) |
| 3 | `mutate-iso16757-1 :: identity-round-trip` | *"this artifact's carrier cannot be read by a second implementation. `'}'` is not a `key=value` field … and this repository publishes no grammar for it"* |
| 4 | `mutate-vdi3805-1 :: identity-round-trip` | same message, same cause |
| 5 | `mutate-jack-1 :: spec-vector-create-node` | *"the committed vector declares a refusal, but the mutation applied"* |

The other twenty-six were captured the same way, by re-running each red owner and reading its report
before the shared `reports/latest` could be overwritten (`w15-audit/failures/*.jsonl`, 26 records;
`w15-audit/report-norm/` holds the norm run's). Every one of the thirty-one is a **refusal by
clause** — a `failed` result carrying a paragraph naming the document consulted and the bytes read —
not a crash, a timeout or a silent skip.

| cause | red | cases |
|---|---:|---|
| **A** — a composed child's `childId` is a content address no document states | **16** | `program-1` 4, `block-3d-1` 6, `note-1` 2, `writer-1` 2, `en1990-1` 2 |
| **B** — the `.dsl.semio` carrier has no usable grammar | **7** | `identity-round-trip` in `iso16757`, `vdi3805`, `program-1`, `note-1`, `writer-1`, `playbook-1`, `forms-1` |
| **C** — a committed vector under-determines the verb it pins | **7** | `puzzle-2d-1` 3, `puzzle-3d-1` 3, `forms-1` 1 |
| **D** — a committed vector is not self-contained | **1** | `jack-1 :: spec-vector-create-node` |

Every one of these was re-derived from source here rather than taken from any note — and the
classification reconciles exactly with the two notes that each hold a piece of it: 26 (the w22 note's
count, and its per-owner log's figures are byte-identical to those measured here) + 5 (the norm and
jack scenarios) = 31.

### 6.1 Defect A — a composed child's `childId` is a digest from a hasher its own vendor refuses to specify

Sixteen scenarios across five cases in four plugins. `mutate-jack-1 :: spec-vector-create-node` is
classified separately (§6.4) because its immediate cause is out-of-band fixture state, but it is the
same hole seen from the other side — the same `DefaultHasher` digest is why jack has only one vector
at all.

w14 read these as two findings ("a mutation whose result our specification cannot state" and "a
committed vector disagrees with the vocabulary"). They are one, they are now sixteen scenarios rather
than two, and the cause is worse than "unpublished". `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🦀️component.rs:87-93`:

```rust
fn en1990_qk_scene_id(entries: &[En1990QkEntry]) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(entries).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("en1990-qk-{:016x}", hasher.finish())
}
```

The committed `insert-variable-action` vector's **entire** before→after difference is one field —
`"childId": "en1990-qk-7904dd65836c8ff4"` → `"en1990-qk-69c0017661d2372c"`. `Q_snow` and `12.5` never
appear in either snapshot; the entry list they belong to lives in a process-local
`EN1990_QK_SCRATCH: RefCell<HashMap<…>>` thread-local. So an implementation holding the committed
before-snapshot and the committed mutation holds everything the specification gives it and still
cannot compute the after-snapshot, because the only field that moves is a hash of a value the
specification never handed it. `std`'s own documentation says the `DefaultHasher` algorithm *"is not
specified, and so it and its hashes should not be relied upon over releases"* — so the committed
after-snapshot is pinned to a value the Rust project explicitly refuses to guarantee, and a toolchain
bump can turn this case red with nothing in this repository changing.

`mutate-jack-1 :: spec-vector-create-node` is the same defect from the other side. Verified directly
in the fixture, not from the note: `…/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🦀️component.rs:41`
calls `cache_jack_content(&snapshot.content.child_id, vec![payload.node.clone()], Vec::new())` — the
node the declared `mutation.duplicate-id` refusal collides with is **injected at test time from
Rust** and is in no committed file. A second implementation reading the committed quartet sees an
empty scene and legitimately applies the mutation. The vector is not wrong about the verb; **it is not
self-contained**, and no contract rule requires that it be. The same file's own header explains why
jack has only this one vector: any state-*changing* branch would need a hand-forged `DefaultHasher`
digest in its `➡️after`.

**It is now sixteen red scenarios, not two.** Writing fifteen more second implementations found the
same hole four more times, each refusing by clause and each naming the handle it could not compute:

```
mutate-program-1     create-knowledge-record, create-benchmark-record   knowledge, benchmarks → s.stdio.semio@v1/table
mutate-block-3d-1    create/delete/rename-vortex-kind                   catalog               → s.stdio.semio@v1/kit
mutate-note-1        edit-block-text                                    a text block content  → s.stdio.semio@v1/text
mutate-writer-1      edit-text                                          document              → s.stdio.semio@v1/document
mutate-en1990-1      insert-variable-action                             the qK child slot
```

— each counted in both the `mutate-` and the `inverse-` role: 4 + 6 + 2 + 2 + 2 = **16**. In every one
the verb's whole observable effect is that address changing (`catalog-a602…` → `catalog-69f2…`,
`note-text-eea4…` → `note-text-9382…`); **every other verb over the same record is implemented and
green**, which is what makes the diagnosis unambiguous. Verbatim from `mutate-program-1`'s run report:

> *"`create-benchmark-record`: this implementation refuses this kind rather than guessing it. The
> 'benchmarks' register is carried as a COMPOSED CHILD HANDLE ({childId, target}), not as an array, so
> this verb's whole observable effect is …"*

**One published addressing rule closes sixteen red scenarios across five cases in four plugins, and
no comparison profile moves.**

**Scale, verified independently:** `DefaultHasher::new()` has **58 non-test call sites** across
`✏️s` and `🧰️framework`, in **46 distinct `🦀️component.rs` files**, spanning ~30 artifacts including
`🔌️jack`, `📘️en1990`, `📙️din18599`, `🧊️process3d`, `🖨️raster`, `🗺️gismap`, `🏛️program`,
`🎬️present`, `📐️cad`, `📋️forms`, `🕸️dag` and `📸️remodel`. **Every artifact in that list whose
snapshot embeds a composed child has the same ceiling: no second implementation can ever reproduce a
mutation that touches the child slot.** `mutate-program-1`'s brand-new Python reference hit it
independently on its own two composed-child registers (`knowledge`, `benchmarks`) and refuses them by
clause. This is not five red scenarios; it is a bound on how far the raised bar can be taken at all
as things stand.

### 6.2 Defect B — the `.dsl.semio` text carrier writes a notation for which no grammar exists

Scenarios: `mutate-iso16757-1 :: identity-round-trip`, `mutate-vdi3805-1 :: identity-round-trip`.

Both subsets' committed
`🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` is, in full and byte-identically:

```
document = header body
header = "schema" SP "stdio.json" NL
body = payload NL?
payload = OCTET+
```

The artifact it is supposed to describe is 4,128 bytes and **opens `semio norm.iso16757.dsl v1`** —
so the grammar's own `header` production is factually wrong about the file, before one gets to the
three things `payload = OCTET+` does not describe: nested records flattened into `key=key=value` runs
with no delimiter (`names=preferred=locale=en text="…"`, ambiguous on its face — `a=b=c` could be
`a.b = c` or `a = "b=c"`), typed table blocks `name [col:TYPE …] { rows }` over an unenumerated type
vocabulary (`TEXT UINT REC TABLE LIST MAP`), and a `_` sentinel for an absent optional field.

**Verified independently: `payload = OCTET+` is the committed body of 46 of the 112 text-snapshot
grammars under `✏️s`, including all fifteen `📕️norm` subsets.** The other thirteen norm cases'
`identity-round-trip` passes only because their documents happen to be flat `key=value` lines a reader
can guess. `iso16757` and `vdi3805` are simply where the absence became load-bearing.

**And five more carriers were refused this wave, three of them for a worse reason than a
placeholder.** Verbatim from the run reports:

* `mutate-writer-1` and `mutate-program-1` — the same `payload = OCTET+` placeholder, with the same
  factually wrong header production: *"whose header production declares `"schema" SP "stdio.json"`,
  contradicted by the artifact's own first line `semio writer.writer.dsl v1`"*. `mutate-writer-1`'s
  refusal ends *"Read 269 bytes of the committed artifact and refused to guess their meaning."*
  `mutate-program-1` adds a second, independent gap: *"all seventy record `$defs` of
  `🧬️schema/📸️snapshot/🔣️component.json` are `{"type": "object", "additionalProperties": true}`
  with no `properties`"* — so the JSON Schema does not bound the notation either.
* **`mutate-forms-1` and `mutate-playbook-1` commit a grammar that describes a DIFFERENT document.**
  *"Its committed grammar describes a DIFFERENT document — the generic `family-scene` canvas grammar,
  `doc-body = schema-line layers-block` with shape/path/text layers and `id`/`x`/`y`/`fill`/`stroke`/
  `opacity` fields — while the committed artifact carries no `layers` block at all."* The same canvas
  grammar is committed over four more unrelated documents: `📏️layout`, `🖍️draw` and `🖨️raster`.
  A wrong grammar is worse than an absent one, because it passes every gate an absent one passes and
  additionally looks finished.
* **`mutate-note-1` commits a REAL grammar and it still cannot be read.** *"the grammar's
  `block = text-block | image-block | shape-block` covers three of the SIX block kinds this vocabulary
  declares, leaving stroke, table, math and group with no production at all; its `block-field` list
  names `paragraphs` and `asset-id` while the committed artifact writes neither."*

Today `payload = OCTET+` — and a grammar for the wrong document, and a grammar covering half its own
vocabulary — satisfy every rule the testing domain has: **30 rule ids across six kinds, none of which
is "a committed grammar must parse its subset's committed examples."**

### 6.3 Defect C — seven scenarios where a committed vector does not determine the verb it pins

`mutate-puzzle-2d-1 :: replace-node-handle` and `mutate-puzzle-3d-1 :: replace-object-vortex` (both
roles each) have exactly one committed vector, that vector supplies a genuinely different record
(`handle-1` moves from `handle-kind-a` to `handle-kind-c`), and the committed outcome declares
`mutation.no-op` with an unchanged after-snapshot. Three different rules produce exactly that and
nothing distinguishes them — the verb is unimplemented, or it refuses an attached port, or it refuses
a kind the compatibility relation does not admit. `mutate-puzzle-2d-1 :: inverse-replace-kind-catalogs`
(and its 3d twin) and `mutate-forms-1 :: inverse-change-form-title` are the same shape from the other
side: the vector INSTALLS a member the before-snapshot did not carry, so undoing it means REMOVING
one, and nothing committed says whether the verb accepts a null.

**This class is invisible to a single implementation and invisible to the subject half**, which
asserts a footprint and never applies the inverse. It is exactly the category of finding the raised
bar exists to produce, and it needs one more committed vector per verb, not a code change.

### 6.4 Defect D — a committed vector that is not self-contained

`mutate-jack-1 :: spec-vector-create-node` is one scenario and one rule. Verified directly in the
fixture: `…/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🦀️component.rs:41` calls
`cache_jack_content(&snapshot.content.child_id, vec![payload.node.clone()], Vec::new())`, so the node
the declared `mutation.duplicate-id` refusal collides with is injected at test time from Rust and is
in no committed file. A second implementation reading the committed quartet sees an empty scene and
applies the mutation. **The vector is not wrong about the verb; it is not self-contained, and no
contract rule requires that it be.**

### 6.5 Nothing was weakened to make any of them go away — including under a refactor

The refusal path is a `raise AssertionError`, not a declared expected outcome: the carrier refusal is
raised at `semio_norm_vocabulary.py:695`; the child-slot refusal raises `Refused` at `:424`/`:432` and
surfaces as an `AssertionError` at `:801`/`:826`. So the scenarios **fail**. Both `📕️norm` carrier
scenarios keep the assertion verbatim —

```gherkin
  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed ISO 16757 document from the parsed carrier
    Given the real committed text artifact asset://…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then the Rust side reproduces the committed file byte for byte and the Python side refuses, because this carrier's notation is specified nowhere
```

— the refusal is **asserted**, not tolerated.
The four `en1990` vector files, the two `.dsl.semio` example artifacts and the four `jack` vector
files are all unmodified in the working tree.

The wave rewrote all fifteen `📕️norm` adapters underneath the norm scenarios (§5), which is exactly when
a red scenario quietly turns green. The wave's own harness replayed every planned scenario through the
old and the new adapters and reported `[equivalence] scenarios=799 identical=799 mismatched=0`.
**This audit could reproduce only 21 of those 799** — the replay reads plan files from
`⚡️cache/tests/work`, and by the time it was re-run only 15 work directories survived, the rest having
been overwritten by three concurrent sessions (§9, last bullet). What was reproduced was clean:

```
[equivalence] scenarios=21 identical=21 mismatched=0
```

That is en1990 only, including its two red scenarios, projection-identical and message-identical
before and after the refactor. The other 778 rest on the wave's own run, which is a weaker standing
than it should be and is the harness's fault, not the wave's.

**What this audit could and did check independently for all fifteen: the post-refactor result itself.**
`oracle exhaustive --owner 📕️norm` was run twice here, hours apart and under different machine load,
and both times returned `cases=15 executed=799 passed=795 failed=4` — the same total, the same four
scenarios, the same four messages. A refactor that had quietly turned a red scenario green would have
shown as `failed=3`.

---

## 7. Weakening check — a fifth consecutive audit finds none

Checked eight ways against `HEAD` (`8d9b51f081`), which is the correct baseline because every
conversion in both waves is uncommitted working-tree work.

1. **Comparison profile knobs.** `git diff HEAD -- '*🔣️component.json'` filtered to
   `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"` returns **nothing**. 98 manifest files changed,
   +2,145/−548 lines, and not one line is a profile knob.
2. **`ignoreKeys` / `"tolerance"` / `"arrays"` repo-wide**, excluding this ticket's own notes: **10
   hits in the entire diff, and not one is a knob.** Six are prose *asserting* that nothing was
   relaxed (*"no `ignoreKeys`, no relaxed profile, and the payload was not swapped for one the caveat
   happens to fit"*); three are the DWG/tessellation `tolerance` *field name* in unrelated production
   code; one is the removal of a paragraph that was replaced by a stronger one (the `mutate-tiff-6-0`
   closure below).
3. **`@comparison-` tags.** Zero added, zero removed, across all 106 changed feature files. No case
   moved to a looser profile — including the two brand-new IFC cases, which reuse the `ruststep`
   siblings' own `semantic-ifc-v1`.
4. **`@mode-` tags.** 165 `@mode-differential` lines added; **zero removed**. Removals are 80
   `@mode-property`, 60 `@mode-conformance`, 1 `@mode-error` — every mode change is an upgrade. Repo
   totals: oracle-backed `differential` 2,356 → **3,728**, `property` 703 → 480, `round-trip`
   130 → 145.
5. **Scenarios.** 166 features, `git diff --name-status HEAD -- '*component.feature'` = **106 M, 0 A,
   0 D, 0 R** (the two new cases are untracked additions). Re-parsed with the repository's own
   `parseFeature` and compared case by case against `w14-audit/survey.json`: **zero scenarios lost on
   any existing case, zero cases regressed from oracle to no-oracle, and the entire +23 is the two new
   IFC cases.**
6. **Fixtures.** Over `*🧫️fixtures*` and `*📚️examples*`: **88 A, 69 M, 0 D, 0 R**, plus 12 untracked
   new fixture files. Per-case: **no case's largest artifact shrank** (§2.1). The working tree does
   carry 1,178 deletions — **every one of them is another session's ticket notes under
   `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`; zero deletions touch a test, feature, fixture,
   example or oracle** (`git status --porcelain | grep '^ D' | grep -icE '🧪️tests|component.feature|🧫️fixtures|📚️examples|🧪️oracle'` = 0).
7. **The migration ratchet.** `./🔒️migration.json` is unchanged since `a2746cd371`
   (2026-08-23 20:01) and clean in the working tree: `{"total": 48, "byArea": {".storybook": 10,
   "✏️s": 1, "🌎️hub": 2, "🧰️framework": 35}}`. Nobody raised the allowance to make §1.1 green.
8. **`law::` call sites in case adapters.** Counted per file over all 55 changed case adapters. The
   delta set is **byte-for-byte identical to the one w14 recorded** — gis 11→13, fem 11→12, tiff
   6→7, fourteen `mutate-semio-*` 2→3, `mutate-semio-value` 1→3, the fifteen norm 12→10 (removing
   this repository's own answer from the oracle's chair when a Python reference took the role — a
   strengthening), `mutate-block-2d-1` 3→0 (documented in place). **Wave 15 added and removed no
   `law::` call anywhere.** The two brand-new IFC cases carry 0 `law::` calls by design: they are pure
   differentials whose assertions are the agreement plus the Python side's own `observable()` and the
   Rust side's own byte-pass-through guard.

**One thing that looks like a loss and is not.** Two feature files lost `Then/And` step *lines*
(`mutate-semio-image` 14 → 10, `mutate-semio-value` 13 → 11) while the changed features overall went
506 → 638. Reading the diff: the vector-only scenarios were kept as their own `spec-vector-<kind>`
scenarios and the mutate/inverse scenarios moved onto a real derived artifact, with several separate
`Then`s consolidated into one that asserts cross-producer agreement. Scenario counts rose; nothing was
dropped.

**A fix at the cause, not at the profile — worth naming.** `mutate-tiff-6-0`'s
`⚠️ KNOWN OPEN DIVERGENCE — mutate-insert-ifd` paragraph is now `✅️ CLOSED, AT THE CAUSE`. The remedy
the old paragraph itself prescribed and rejected as too expensive — *"giving `TiffIfd` its own strip
bytes — a schema-first change across the snapshot, diff, mutation, proto/graphql/ts mirrors and the
binary protocol — not a tolerance, an `ignoreKeys` entry or a cosmetic `RowsPerStrip`"* — was carried
out. `TiffIfd` now has its own `pixels` field threaded through the snapshot, `TiffIfdDiff`, the text
and binary diff codecs and the proto/graphql/ts/json mirrors.

---

## 8. Measured ratios in source: seven sites still assert one, and the scrub damaged seven case narratives

The brief's last question. The scrub reached **thirteen** source sites in twelve files, not eight —
searched wrap-tolerantly (the phrase straddles a line break in five of them, which is why a naive
line-oriented grep undercounts it):

```
🎞️gif/🧪️tests/mutate-gif-87a/component.feature          🖊️dwg/🧪️tests/mutate-dwg-ac1018/component.feature
🎞️gif/🧪️tests/mutate-gif-89a/component.feature          🖊️dxf/🧪️tests/mutate-dxf-r12/component.feature
🎞️pptx/🧪️tests/mutate-pptx-ecma-376/component.feature   🖼️bmp/🧪️tests/mutate-bmp-v3/component.feature
🏗️ifc/🧪️tests/mutate-ifc-4/component.feature            🖼️tiff/🧪️tests/mutate-tiff-6-0/component.feature
📄txt/🧪️tests/mutate-txt-utf-8/component.feature        📕️xlsx/🧪️tests/mutate-xlsx-ecma-376/component.feature
📄️pdf/🧪️tests/mutate-pdf-1-7/component.feature          📐️step/🧪️tests/mutate-step-ap214/component.feature
📄️pdf/🧪️tests/mutate-pdf-1-7/🦀️component.rs
```

**8.1 — the scrub missed a ratio in the very file it edited.** In
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7/🦀️component.rs`, line 14 was changed
from `` `parity=24/37` `` to *"its parity ratio (recorded in the ticket, not here)"*. **Line 19, five
lines below, was not touched:**

```rust
//! `parity` is now 34/37 with no comparison profile touched, no `ignoreKeys` added and no fixture
```

That is the worst of the seven remaining sites — a **present-tense** assertion of a current measured
ratio, in the one file the scrub demonstrably had open. The same case's `component.feature` was
rewritten by hand in the same wave and reads cleanly (*"it scored the ratio recorded in the ticket,
not here"*), so one file of the pair was edited by a person and the other by a substitution.

**8.2 — six more measured results survive in source.** Five are historical rather than present-tense,
but all are still "what it last scored" rather than "what the case asserts":

```
📄️pdf/🧪️tests/mutate-pdf-1-4-a/component.feature:36       "This case scored 0 of 9 the first time"
📄️pdf/🧪️tests/mutate-pdf-1-4-x/component.feature:36       "This case scored 0 of 9 the first time"
📄️pdf/…/🔖️1.4/✳️any/…/📸️snapshot/🦀️component.rs:10        "mutate-pdf-1-4-a/mutate-pdf-1-4-x scored 0/9 apiece"
📄️pdf/…/🔖️1.4/✳️x/🧪️oracle/🦀️component.rs:149             "this case scored 0 of 9 on a document typeset at A4"
📐️step/🧪️tests/mutate-step-ap214/component.feature:50     "Both roles passed their own laws (`executed=46 passed=46`)"
📄txt/🧪️tests/mutate-txt-utf-8/component.feature:64        "`subject exhaustive … mutate-txt-utf-8` → `executed=24 passed=24`"
```

**8.3 — the scrub was mechanical in seven of the thirteen sites, and it broke the prose in all seven.**
The substitution replaced a noun phrase with a differently-shaped one and left the surrounding
sentence un-repaired. Verbatim:

```
dxf-r12   THE FIRST DIFFERENTIAL RUN OF THIS CASE WAS its parity ratio (recorded in the ticket, not
          here), AND IT FOUND THREE REAL DEFECTS THAT
ifc-4     …unsupported escape start Some('\')`, its parity ratio (recorded in the ticket, not here).
          Byte 138718 is
step      Both roles passed their own laws (`executed=46 passed=46`) and yet its parity ratio
          (recorded in the ticket, not here):
txt       while proving only that the codec did not panic. its parity ratio (recorded in the ticket,
          not here) is the correct and permanent reading
pdf-1-7   //! …run of this case scored //! its parity ratio (recorded in the ticket, not here), and
   (.rs)  ten of the thirteen failures were ONE production defect: `encode_pdf`
xlsx      All six pool scenarios diverged structurally (its parity ratio (recorded in the ticket, not
          here)): the oracle emitted `{"sharedStringCount":…
pptx      all 19 subject scenarios red, its parity ratio (recorded in the ticket, not here), while
          the oracle composition read
```

The other six — `gif-87a`, `gif-89a`, `pdf-1-7`'s feature, `dwg-ac1018`, `bmp-v3`, `tiff-6-0` — were
rewritten properly and read as English:
*"⚠️ KNOWN OPEN DIVERGENCE — `mutate-set-pixel-data` (this case's parity ratio is recorded in the
ticket, not here)."* That is the pattern the other seven should have followed. Seven case narratives —
several of them the best forensic writing in this repository, explaining a real defect a differential
run found — no longer parse. The rule was obeyed and the documentation was damaged; both facts belong
in the record.

**8.4 — the two divergences the parity runs found are declared, not hidden.** `mutate-bmp-v3 ::
mutate-set-pixel-data` and `mutate-gif-87a :: mutate-set-global-color-table` — the only two failures
in the 246 comparisons measured so far (§1.3) — each have a `⚠️ KNOWN OPEN DIVERGENCE` paragraph in
their own feature that argues the disagreement from the format and names what resolving it would
require. Neither was closed by widening a profile.

**8.5 — `mutate-zip-2-0`'s docstring did it right, and still overstates one thing.** Its
`parity=15/15` claim is gone, replaced by an explicit statement of the rule
(*"a measured parity ratio recorded in source is a claim about a moment, and it becomes false the
moment anything moves"*). But the same paragraph still asserts *"the subject phase RUNS"* and cites
`cargo check -p semio-framework-os-kernel --lib` (the **default** feature set) as proof. The subject
phase does now run (§1.3) — but that sentence has been true-by-luck for three audits and its cited
evidence is still the root-workspace check, not the generated host's own `[workspace]`, which is the
thing `📓️w21-four-blockers.md` §1 showed can disagree.

---

## 9. Stale claims, a split record, and harness defects still standing

* **The ticket's own record has split, and each fragment reads as complete.** Two notes written today
  report incompatible totals for the same measurement: `📓️w15-specification-defects.md` is titled
  *"the five red scenarios"* and analyses five; `📓️w22-group-a-second-implementations-2026-08-26.md`
  §3 is headed *"The twenty-six red scenarios"* and analyses twenty-six. **Neither states the total,
  which is thirty-one** (§1.2, §6), and the brief for this audit inherited the five. Both notes are
  individually excellent and individually misleading about scope. Nothing in the repository aggregates
  a red-scenario count, because the only command that would — repo-wide `oracle exhaustive` — has not
  completed on this machine in three audits.
* **`✏️s/…/🖼️tiff/🧪️tests/mutate-tiff-6-0-baseline/🦀️component.rs:19`** still says
  `identity-round-trip` goes through `law::reparsed_not_copied`. The handler calls
  `law::carrier_is_exact` (`:244`) and the file's own comment at `:226-231` explains why the old law
  was replaced. Flagged at w16, at w14, and again here — **third audit, unfixed.**
* **`📋️status.md` is untouched since `215e369d07` (2026-08-23 18:01, file mtime 14:04)** — three
  days and two conversion waves stale. Its dashboard says **"Test cases discovered: 11 across 8
  owners", "Scenarios declared / executed at quick: 32 / 30", "Registered oracles: 10", "External
  dependencies classified: 212", "Recorded no-oracle decisions: 2"** and **"Parity comparisons:
  37 / 37 equal"**; live it is **166 cases / 4,933 scenarios / 139 oracles / 233 dependencies / 28
  no-oracle decisions**, and the parity line is a measured ratio frozen in prose — the exact thing
  this wave spent effort scrubbing out of source. Its owner table still says
  *"every other non-`compose` owner: discovered / surveyed"* and its explanatory paragraph still says
  the six artifact owners are `oracle-green` because *"a concurrent session's os-kernel refactor is in
  flight"* — a sentence that stopped being true this morning. `📋️contract.md` (10:53) and
  `📋️architecture.md` (15:00) are frozen the same day. **A reader who starts at the ticket's own
  status document is misled before reaching any audit.**
* **A case whose host fails to build still contributes `executed=0 passed=0 failed=0`.**
  `executeOne` (`📜️script.ts:520-541`) returns `{ results: [], problems }` when the host exits
  non-zero, so the case is invisible in the summary line. This is w12 remedy #7 / w13 remedy #2 and it
  is exactly what produced §1.3's green-looking `executed=15 passed=15 failed=0 errored=0 parity=0/0`
  for a case whose oracle half did not exist.
* **`runProbe` still throws on `ETIMEDOUT`** (`📚️library/📦️packages/🟦️typescript/📦️index.ts:1697-1719`)
  and nothing catches it before `run`, so one slow case still discards a whole repo-wide run with no
  summary line. w13 remedy #10, w16 §1.3, w14 remedy #6.
* **`⚡️cache/tests/reports/latest` is still shared, not per-run** (`📜️script.ts:75-78` — the path is
  literally `reports/latest`, with no run id). Two sessions running the platform at once overwrite
  each other's `📊️summary.json`, `📤️results.jsonl`, `📋️junit.xml` and `📈️metrics.json`.
* **`not-exercised` still merges a policy decision with a crash** in the summary line. The per-case
  `[test] not-exercised …` line now distinguishes the two reasons in its parenthetical, and a new
  `[test] no-subject-implementation …` line was added for host-reference-only cases — both real
  improvements — but the counter in `[test] level=… not-exercised=N` still adds them together.
* **The shared build and result cache is a live hazard, observed twice during this audit.** A
  concurrent session's `parity exhaustive --owner 🗄️stdio` (pid 68682) writes the **same**
  `⚡️cache/tests/{work,results}/test-s-plugins-stdio-artifacts-zip-c00596-mutate-zip-2-0-*`
  directories and the **same** `⚡️cache/agents/local/cargo-test-hosts` target directory as this
  audit's probes. §1.3's oracle-host failure is a direct consequence: a `-L native=…/onig_sys-…/out`
  path vanished under a build that was still using it.

---

## 10. Remedies, in order of leverage

1. **Make the harness survivable, because it is now the binding constraint and it was not before.**
   Three things, all open since w13, and together they are why this audit could not produce a
   repo-wide number on a machine that had cleared its only real blocker:
   * `runProbe` throws on `ETIMEDOUT` and nothing catches it — one slow case discards the entire run
     with no summary line. Observed **four times today**, and it is what forced §1.2's repo-wide
     command to be replaced by a per-owner sweep.
   * `⚡️cache/tests/{work,results,reports/latest}` and `⚡️cache/agents/local/cargo-test-hosts` are
     shared across sessions with no run id, so concurrent runs clobber each other's plans, results,
     reports and build artifacts. Observed live (§9).
   * `executeOne` returns `results: []` for a host that failed to build, so the case contributes
     `0/0/0` and the summary line stays green.
   Fixing these is worth more than any number of new oracles, because without them no number can be
   produced at all.
2. **Publish the composed-child addressing rule, and stop using `DefaultHasher` for it** (§6.1). This
   is the single highest-value change available: it closes **16 of the 31 red scenarios**, across five
   cases in four plugins, and 58 call sites across 46 files put a ceiling on how much of the
   repository can *ever* meet the raised bar until it lands. A specified digest over a specified canonicalisation —
   the test protocol already has `digest` = truncated sha256 — makes the value computable in any
   language. Add a contract rule that a specification vector must be self-contained; `mutate-jack-1`'s
   is not, and nothing reports that.
3. **Publish one grammar for the `.dsl.semio` notation, and gate it** (§6.2). It closes 7 more red
   scenarios. It is one notation shared by every subset that emits `ArtifactDsl`; 46 of 112 committed
   text-snapshot grammars are the `payload = OCTET+` placeholder whose header production is factually
   wrong about its own artifact's first line, and five more subsets (`📋️forms`, `📖️playbook`,
   `📏️layout`, `🖍️draw`, `🖨️raster`) commit a grammar that describes a **different document**
   altogether. Add the rule that has no counterpart today: *a committed grammar must parse its
   subset's committed examples* — a wrong grammar currently passes every one of the 30 rule ids.
4. **Add one more committed vector per under-determined verb** (§6.3, 7 red scenarios) and make
   specification vectors self-contained (§6.4, 1 more). Neither needs a code change; both need a
   committed file. Together with remedies 2 and 3 they close **31 of 31**.
5. **Give the eleven artifact-less `mutate-*` cases an artifact** (497 scenarios). They all now have a
   second implementation and none has a document. `mutate-semio-text`'s derivation is the recipe, and
   `mutate-semio-table`/`mutate-semio-value`'s `payload-fidelity` scenario is the part the eleven
   upgrades of this wave skipped — derive from a real committed artifact **and re-derive on every
   run**, so the fixture cannot drift.
6. **Declare Pillow and three.js, and close the gate's blind spot** (§5.3). The gate that would have
   caught them exists and works — it fired on `ifcopenshell` in the TypeScript suite this morning —
   but `oracleLinkedPackages` skips any registration whose `package` is `""`, which is exactly the
   convention those two use. Two fixes: give them real `package`/`version` entries, and add the rule
   *an oracle adapter that imports a distribution no manifest declares is a breach*, keyed on the
   adapter's imports rather than on its registration.
7. **Regenerate `🔒️dependencies.json`.** It is 232 entries against a live scan of 233, missing
   `ifcopenshell` entirely and carrying `png 0.17.16` where both `Cargo.toml` and the registration say
   `0.18`. One red test today, and it will stay red.
8. **Convert the seventeen self-declared debts** (425 scenarios). They now name what a reference would
   be written from and the nearest transferable recipe; the largest four — `mutate-remodel-1` (71),
   `mutate-shooting-1` (63), `mutate-layout-1` (51), `mutate-process3d-1` (33) — are 218 of the 425.
9. **Repair the seven damaged case narratives** (§8.3) and delete the seven surviving measured results
   (§8.1–8.2), starting with `mutate-pdf-1-7/🦀️component.rs:19`'s present-tense `34/37`.
10. **Split `not-exercised`** in the summary line into "recorded no-oracle decision" and "host failed".
   The per-case lines now distinguish them; the counter still does not.
11. **Reconcile the ticket's own record, and refresh `📋️status.md`, `📋️contract.md`,
    `📋️architecture.md`.** Two notes written the same day report five red scenarios and twenty-six;
    neither states thirty-one (§9). The status document is three days and two conversion waves stale. The status document is three
    days and two conversion waves stale and its central explanatory sentence — that a peer's os-kernel
    refactor is blocking every subject — stopped being true this morning. Delete the two stale
    docstring claims in §9 and the 17 zero-byte example assets.

---

## 11. Totals

| | w12 | w13 | w14 | **w15 (now)** |
|---|---|---|---|---|
| cases | 164 | 164 | 164 | **166** |
| scenarios | 4,562 | 4,564 | 4,910 | **4,933** |
| `@oracle-` cases / scenarios | 79 / 1,331 | 79 / 1,331 | 121 / 3,191 | **138 / 4,355** |
| … of which `@mode-differential` | — | — | 2,356 | **3,728** |
| `@no-oracle-` cases / scenarios | 85 / 3,231 | 85 / 3,233 | 43 / 1,719 | **28 / 578** |
| … of which inherit the falsified argument | — | — | 34 / 1,633 | **17 / 425** (all self-labelled DEBT) |
| oracle registrations | — | 80 | 122 | **139** (138 used, 1 orphan) |
| … declaring a third-party package | — | 79 | 79 | **81** (29 distinct packages) |
| … declaring `package: ""` | — | — | 42 | **57** |
| distinct in-repo second implementations | — | 0 | ≈27 | **40** (39 py + 1 ts, clustered ≥0.60) |
| `mutate-*` cases on < 4 KiB or nothing | — | — | — | **49 / 1,851 sc** |
| … or on a generic `🎬️demo` placeholder | — | — | — | **56 / 2,592 sc (53%)** |
| cases reading no real artifact at all | — | — | 27 / 538 | **27 / 538** |
| ifc/step cases with a differential scenario | — | — | 0 of 12 | **0 of 12** (+2 new cases, 23 sc) |
| oracle-phase scenarios executed | 1,331 | 1,331 | 3,148 | **2,319 over 36 non-`🗄️stdio` owners; the repo-wide command aborted twice** |
| **oracle-vs-subject comparisons (`parity`)** | **0/0** | 1,012/1,277 (stdio) | **0/0 — no subject host linked** | **unmeasurable repo-wide** (5 runs aborted). Measured here: **10/10** on the two TypeScript-subject owners. Measured per-case by a concurrent session: **244/246** over 23 of 166 cases = **5.0% of scenarios** |
| open oracle-phase divergences | 0 | — | 5 | **≥ 31** (5 re-confirmed + 26 measured here) across 9 owners, in 4 causes; `🗄️stdio` unmeasured |
| contract breaches | 0 | 0 | 2 | **2** (`unmanaged-tests`, other sessions) |
| TS suite | 69 / 0 fail | 69 / 0 | 67 / 2 | **65 / 4**, 2,058 `expect()` |
| stdio oracle crate `cargo test` | 369 / 367 ok | — | 374 / 372 ok | **374 / 372 ok** |
| dependency entries / test-oracle | 232 / 30 | 232 / 30 | 232 / 30 | **233 / 31** |
| fixtures deleted / scenarios deleted / profiles loosened | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 | **0 / 0 / 0** |

**One sentence.** The wave converted 1,141 more scenarios to a second implementation, collapsed
fifteen byte-identical oracles into one honestly-labelled module, adopted the exact IFC reference the
owner named, closed a known divergence at its cause and weakened nothing — and the thirty-one refusals
those references now raise by clause, sixteen of them one unpublished addressing function, are worth
more than every count in this table; meanwhile the blocker two audits blamed for zero parity quietly
cleared, leaving a test harness that discards a forty-minute run when one case runs slow, and half the
bar — *a real-world complex artifact* — unmet for 2,592 of 4,862 mutation scenarios that nobody has
counted since it was raised.

---

## 12. Method


* Population, tags, modes, oracle/no-oracle attribution and adapters: the repository's own
  `discoverTestCases` and `parseFeature`, via `w15-audit/survey.ts` → `survey.json` and
  `w15-audit/modes.ts`. Not grep.
* Fixtures: the repository's own `fixtureUrisIn` + `resolveFixtures`, via `w15-audit/fixtures.ts` →
  `fixtures.json` → `fixture-rows.json`, run from the repository root so relative paths resolve;
  sizes stat-ed on disk; any URI whose resolved path passes through `🧬️mutations/…/🧪️tests/`
  classified as a specification vector, never as an artifact.
* w14-vs-w15 deltas: the two waves' own `survey.json` / `fixtures*.json`, compared case by case by id.
* Similarity: 5-gram Jaccard over token streams with docstrings and comments stripped; clustering by
  union-find at ≥ 0.60.
* Registry orphans: `w15-audit/orphan.ts`.
* Red scenarios: the repository's own `oracle exhaustive`, run per owner (`w15-audit/sweep.sh` →
  `oracle-sweep/`), then each red owner re-run and its `⚡️cache/tests/reports/latest/📤️results.jsonl`
  copied out immediately (`w15-audit/failures.sh` → `failures/*.jsonl`, 26 records; plus
  `failures/…` for norm and jack). Every quoted diagnostic is a `diagnostics[0].message` from those
  records, not from any note.
* The pre-refactor `📕️norm` engine-identity claim: `w15-audit/🐍️engine-identity.py`, run over the
  fifteen adapters preserved at `w15-work/old-adapters/`.
* Every exit code was read from the tool's own exit status. No number in this document came from a
  pipeline's exit code, and no `[test]` line was paraphrased. Where a figure is a sum of several
  tool lines rather than one tool line, it is labelled as such at the point of use (§0, §1.2).
* ⚠️ **A note on `w15-audit/` itself.** At `HEAD` this directory held ~60 logs from an earlier
  session (`chk-*.txt`, `run-all.sh`, `parity-report/`). Every one of them was already **deleted from
  the working tree before this audit began** — the ticket folder listing taken at 08:50 contained
  `w15-work` and no `w15-audit`. This audit recreated the directory; the only filename that collides
  with a deleted predecessor is `01-contract.txt`. Nothing was removed here.
