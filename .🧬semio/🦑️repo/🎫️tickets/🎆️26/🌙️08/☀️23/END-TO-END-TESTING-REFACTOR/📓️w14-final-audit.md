# Wave 14 — final audit against the raised bar

> **THE STANDARD, as raised:** *every single mutation exercised on a REAL-WORLD COMPLEX artifact, with a
> SECOND INDEPENDENT IMPLEMENTATION producing the same result.*

Date 2026-08-26. HEAD `8d9b51f081f42b36722b54f80a5c502d6322f9ca` (2026-08-25 14:57:24 +0200), **dirty
tree** — every conversion this audit judges is uncommitted working-tree work, so all diffs below are
`git diff HEAD`. Successor to `📓️w16-final-audit.md` (2026-08-25 23:31), whose §1.2, §1.4, §1.7–1.9,
§6.1–6.3, §9 and §11 were left **(pending)** and are answered here.

Raw logs: `w14-audit/`. Every `[test]` line is copied verbatim from the tool's own stdout. Exit codes
were read from the tool's own exit status, never through a pipe.

---

## 0. The most misleading thing a reader would otherwise believe

**That `oracle exhaustive` reporting `executed=3148 passed=3143 failed=5` means 3,143 mutations were
confirmed by a second implementation. It means the opposite: in every one of those 3,143 scenarios
the second implementation ran ALONE.** The oracle phase prints `parity=0/0` by construction — it
executes only the reference side. The number that answers the raised bar is `parity`, and today it is
**zero across the whole repository**, because no generated Rust subject host links:

```
error[E0432]: unresolved import `component::component_persistent_local`
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error; 103 warnings emitted
```

Measured on two different owners as isolated single cases (§5.3). `semio-framework-plugin` sits in
every Rust subject host's dependency graph, so this is one framework-level break, not 34 plugin ones —
and it belongs to a **live peer session** refactoring `💻️os/🔨️modules/🔌️plugin` (its files were last
written at 05:18, 04:36, 04:33 and 01:12 today), not to this ticket. But the consequence stands:
**right now, not one mutation in this repository is being checked against a second producer.** The
worst part is how it reads — `mutate-zip-2-0` prints
`executed=15 passed=15 failed=0 errored=0 parity=0/0` for a case whose subject half never ran.

Four further things a reader would get wrong from the headline:

1. **An oracle registration is not a comparison, and not every oracle is a second producer.** The 42
   conversions moved the ORACLE role from Rust to Python/TypeScript in 42 cases, taking oracle-backed
   cases 79 → 121 and oracle-tagged scenarios 1,331 → 3,191. That is a real and large gain in
   *reference* coverage — the biggest in this ticket's history — and it is worth nothing under the
   raised bar until a subject runs beside it. Of those 3,191, only **2,356 are `@mode-differential`**;
   the other 835 use the reference as an independent *reader*, which is honestly typed but is not the
   bar. The twelve `🏗️ifc` + `📐️step` cases declare **zero** differential scenarios because
   `ruststep` has no writer — and `ifcopenshell`, which reads *and* writes IFC and is the reference
   the owner named, **is installed on this machine (0.8.4.post1)** and was rejected for a governance
   reason, not a technical one (§3.4).
2. **The count of remaining `@no-oracle-` cases is 43, not 81.** The brief's 81 is stale (w13 measured
   85; the conversion wave took it to 43 before this audit began). What has *not* changed is the
   argument underneath them: **34 of the 43 (1,633 of 1,719 scenarios, 95%) still rest on the sentence
   "no third-party library reads or writes semio's own `.dsl.semio`/`.pack.semio` envelope, and our
   vocabulary IS the specification" — the exact argument this same wave falsified 41 times.**
   `mutate-block-2d-1` got a Python second implementation; its siblings `mutate-block-3d-1` and
   `mutate-block-5d-1` (158 scenarios) still decline one, in the same plugin, over the same carrier,
   citing the same reason.
3. **"Real-world complex artifact" does not hold for most of the newly-oracled surface.** Of the 42
   conversions, **30 cases / 1,321 of 1,860 scenarios exercise their mutations on an artifact smaller
   than 4 KiB** — `mutate-semio-text` on a 203-byte demo note, `mutate-en1990-1` on a 215-byte
   `.dsl.semio`. Only 7 of the 42 read an artifact above 64 KiB. Repo-wide, **27 cases / 538
   scenarios read no artifact at all** beyond handcrafted per-kind specification vectors.
4. **The five red scenarios are the most valuable output of the whole wave**, and they are all
   semio-native: two say our own specification cannot state the result of a mutation, two say the
   `.dsl.semio` carrier has no grammar for what it writes, one says a committed vector disagrees with
   the vocabulary (§8). Not one is on a third-party-backed case.

### What genuinely improved, and is not overstated

* The 42 conversions are **real second implementations, not transliterations** — audited file by file
  in §3: no `import`, no FFI, no `subprocess`, no shell-out to cargo, no Rust identifier smuggled in.
* **The oracle phase now reaches 3,148 of 4,910 scenarios (64.1%)**, against 1,331 of 4,562 (29.2%)
  at w12 and the identical 1,331 at w13.
* **Zero scenarios were deleted**; the 58 changed feature files went **2,086 → 2,432 scenarios**, all
  346 of them added.
* **No comparison profile knob changed anywhere**: `git diff HEAD -- '*🔣️component.json'` filtered to
  `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"` returns **nothing**.
* **No fixture was deleted or swapped.** 88 added, 69 modified, 0 deleted — and 66 of the 69
  modifications are `.rs` files; the only three non-`.rs` modifications are PDF 1.4 *demo* assets that
  no case reads.
* **The migration ratchet was not lowered.** `🔒️migration.json` is unchanged since `a2746cd371`
  (2026-08-23 20:01) and clean in the working tree; the contract is red because other sessions' test
  files grew the legacy backlog.
* `cargo test --features oracles --lib` is **exit 0 with 374 tests, 372 passed, 0 failed** (w12: 369 /
  367).

---

## 1. The six commands, verbatim

All six were run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` except #5 (in
`📦️packages/🟦️typescript`) and #6 (in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`).

> **Machine conditions, stated because they affect two of the six.** Throughout this audit a
> *concurrent session* was running `bun ./📜️script.ts parity exhaustive --owner 🗄️stdio` (pid 41654,
> started 00:11; a second run, pid 25299, started 03:42). Both sessions write the **same** per-case
> directories under `.🧬semio/🦑️repo/⚡️cache/tests/{work,results}` and the **same**
> `⚡️cache/tests/reports/latest`. That clobbering was observed live: `reports/latest/📊️summary.json`
> changed twice under this audit's feet, to `cases=1 … create-and-edit-archive` and then to
> `cases=1 … create-and-read-jpeg`, neither of them this audit's run. **The run report is not
> per-run isolated, and two sessions running the platform at once silently overwrite each other's
> report.** That is a defect of the harness, recorded here because it also means any `reports/latest`
> a reader finds may belong to a different run than the summary line they are reading.

### 1.1 `bun ./📜️script.ts contract` — **exit 1**, 2 breaches across 1 rule id

```
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery

  testing/discovery  🧰️framework  44 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  5 executable test file(s) outside the canonical owner-root test tree, baseline allows 1

full breach set (including non-blocking priorities): /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/breaches/testing.json
```

`testing.json` read straight afterwards holds exactly those two records and nothing at a lower
priority.

**Breaches by rule id.** The domain declares **30** rule ids across six kinds. Twenty-nine are at
zero; one is not:

| rule id | kind | count |
|---|---|---|
| `unmanaged-tests` | `testing/discovery` | **2** |
| `unregistered-mutation-vocabulary`, `mutation-kind-uncovered`, `mutation-inverse-uncovered`, `mutation-kind-undeclared`, `mutation-kinds-deferred`, `mutation-catalog-unclaimed`, `mutation-catalog-capability-mismatch`, `unknown-mutation-catalog`, `unknown-comparison`, `missing-comparison`, `missing-capability`, `feature-syntax`, `no-adapter`, `no-scenarios` | `testing/contract` | 0 |
| `missing-oracle`, `unknown-oracle`, `unknown-no-oracle-decision`, `oracle-capability-mismatch`, `oracle-profile-mismatch`, `differential-without-evidence`, `claimed-implementations-missing` | `testing/oracle` | 0 |
| `missing-fixture`, `orphan-fixture` | `testing/fixture` | 0 |
| `case-slug`, `case-in-language-package`, `unknown-adapter-filename`, `unknown-case-child` | `testing/taxonomy` | 0 |
| `oracle-in-production` | `testing/dependency` | 0 |
| `excluded-path-leak` | `testing/discovery` | 0 |

**Attribution: not this ticket, and not a lowered ratchet.** The baseline
(`./🔒️migration.json`, `{"total": 48, "byArea": {".storybook": 10, "✏️s": 1, "🌎️hub": 2,
"🧰️framework": 35}}`) is unchanged since `a2746cd371` (2026-08-23 20:01) and clean in the working
tree — the counts grew, the allowance did not move. Exactly **four** of the overage files did not
exist at HEAD, and none of them touches a format, an oracle, a comparison profile or a case:

```
✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🧪️sequence-browser-consumer.test.js
✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🧪️sequence-protocol-oracle.test.js
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧬️flow-schema-oracle.test.js
```

w16 measured 42/4 here; it is 44/5 now. The ratchet is doing its job against neighbouring sessions.

### 1.2 `bun ./📜️script.ts oracle exhaustive` (repo-wide) — **exit 1**

```
[test] level=exhaustive cases=164 executed=3148 passed=3143 failed=5 errored=0 parity=0/0 not-exercised=44
```

Preceded by 44 `[test] not-exercised …` lines, followed by the problem list. Run wall clock: **86
minutes** under the concurrent load described above. Full log in `w14-audit/02-oracle-repowide.txt`.

**`executed=3148` is exactly `3191 − 43`** — every oracle-tagged scenario in the repository except
the 43 of `mutate-gif-89a`, whose host did not build:

```
[test] ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-89a: rust oracle host exited 101 without emitting results
error: could not compile `semio-test-host-mutate-gif-89a` (bin "host")
Caused by:
  could not execute process `sccache … rustc --crate-name host …` (never executed)
[test] ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🧪️tests/mutate-gif-89a: no result stream at …/📤️results.jsonl
```

That is an `sccache` failure under concurrent load, **not a defect in the case** — but note how it is
reported: `not-exercised=44` puts a case whose oracle host CRASHED in the same count, under the same
word, as the 43 cases that deliberately have no oracle. **A crash and a policy decision are
indistinguishable in the summary line.**

**`parity=0/0`.** The oracle phase alone forms no comparison, by construction. That is expected here;
what matters is §5.

#### The five failures are all genuine, unweakened divergences — and they are exactly what the raised bar is for

Recovered per-case (the run's own `reports/latest` had already been overwritten by a concurrent
session before it could be read — §7):

| case :: scenario | what the second implementation says |
|---|---|
| `mutate-en1990-1 :: mutate-insert-variable-action` | *"the committed vector declares this mutation applied, yet this implementation refused it: `insert-variable-action` would seed the composed child slot `'qK'`, **whose childId is content-addressed by a function no specification in this repository states**"* |
| `mutate-en1990-1 :: inverse-insert-variable-action` | same cause, reached from the committed before-snapshot |
| `mutate-iso16757-1 :: identity-round-trip` | *"this artifact's carrier cannot be read by a second implementation. `'}'` is not a `key=value` field: the notation nests records and tables and flattens nested records into `key=key=…`"* |
| `mutate-vdi3805-1 :: identity-round-trip` | same cause |
| `mutate-jack-1 :: spec-vector-create-node` | *"the committed vector declares a refusal, but the mutation applied"* |

Three distinct findings, none of them tuned away:
1. **A mutation whose result our own specification cannot state.** `insert-variable-action` seeds a
   composed child whose `childId` is content-addressed by an unstated function. Our Rust can produce
   it because it *is* the function; a second implementation cannot, and says so. That is a
   specification hole the single-implementation era could never have surfaced.
2. **The `.dsl.semio` carrier has no grammar for nested records.** Two `📕️norm` subsets write a
   notation that nests records and tables and flattens nested records into `key=key=…`, and the
   committed `📖️component.grammar.semio` for those subsets is the repository-wide placeholder whose
   whole body is `payload = OCTET+`. A second reader cannot parse what no document describes.
3. **A committed specification vector disagrees with the vocabulary.** `mutate-jack-1`'s
   `create-node` vector declares a refusal; both implementations apply it.

**None of these was closed by widening a profile, and all five are still red.** Verified per case:
`oracle exhaustive --owner 📕️norm` → `cases=15 executed=799 passed=795 failed=4`, and
`--case mutate-en1990-1` → `cases=1 executed=21 passed=19 failed=2`.

### 1.3 `bun ./📜️script.ts parity exhaustive` (repo-wide) — the headline command

**It did not produce a headline number, and §5.3 shows the number it would produce is zero.** Two
isolated single-case runs of the same command are quoted verbatim in §5.3; both end in

```
rust subject host exited 101 without emitting results
error[E0432]: unresolved import `component::component_persistent_local`
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error; 103 warnings emitted
```

and both print `parity=0/0`. The repo-wide invocation was started detached and was still walking its
first handful of cases after 25 minutes — 13 of them inside `mutate-os-config-opening` alone, against
a fixed 900 s per-case `cargo run` budget that includes compiling the host from scratch — before
reaching `mutate-present-1`, the exact case whose 900 s overrun destroyed the w16 attempt with
`spawnSync cargo ETIMEDOUT` and no summary line. Details and the two probes in §5.3–5.4.

### 1.4 `bun ./📜️script.ts dependency` — **exit 0**

```
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
```

Byte-for-byte the same summary figures as waves 11, 12, 13 and 16: `entries=232`, `test-oracle=30`,
the same three `production-debt` records. **That stability is a finding, not a reassurance** — see
§3.3. Full listing in `w14-audit/04-dependency.txt`.

### 1.5 `bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — **exit 1**

```
 67 pass
 2 fail
 2057 expect() calls
Ran 69 tests across 1 file. [119.72s]
```

Both failures are §1.1's breach surfacing inside the suite; **no load-induced timeout this time**
(w16 had a third failure at 58,547 ms):

```
(fail) 🔍️ discovery and contract > every committed case satisfies the frozen contract [25464.47ms]
  - []
  + [
  +   "testing/discovery:🧰️framework:44 executable test file(s) outside the canonical owner-root test tree, baseline allows 35",
  +   "testing/discovery:✏️s:5 executable test file(s) outside the canonical owner-root test tree, baseline allows 1",
  + ]
(fail) 🔍️ discovery and contract > the migration backlog is a shrink-only ratchet, never a growing allowlist [5344.53ms]
error: expect(received).toBeLessThanOrEqual(expected)
Expected: <= 1
Received: 5
```

`expect()` calls 2,061 (w16) → **2,057**; the four-assertion drop is the contract test's own breach
list shrinking by two records × two assertions, not a removed test. 69 tests, unchanged.

### 1.6 `cargo test --features oracles --lib` in `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — **exit 0**

```
running 374 tests
test result: ok. 372 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 106.73s
```

347 (w11) → 369 (w12) → **374**. The 2 ignored are the same one-shot `#[ignore]`d fixture-derivation
helpers (`bmp v3`, `tiff 6.0`); neither is a skipped assertion.

---

## 2. How many cases still carry `@no-oracle-`, and what was tried

**43 cases / 1,719 scenarios**, out of 164 cases / 4,910 scenarios. Measured with the repository's own
`discoverTestCases` + `parseFeature` (`w14-audit/survey.ts`, raw in `survey.json`), not by grep.

| | w12 | w13 | now |
|---|---|---|---|
| cases | 164 | 164 | **164** |
| scenarios | 4,562 | 4,564 | **4,910** |
| `@oracle-` cases / scenarios | 79 / 1,331 | 79 / 1,331 | **121 / 3,191** |
| `@no-oracle-` cases / scenarios | 85 / 3,231 | 85 / 3,233 | **43 / 1,719** |
| registered oracle entries | — | 80 | **122** |
| registered `noOracleDecision`s | — | — | **44** (43 claimed, 1 orphan) |

**The brief's "81" is stale in both directions**: `w13` measured 85, and the conversion wave that
followed it took the number to 43 before this audit began. Nothing in this audit's window changed it.

### 2.1 The 43, and why each says no second producer exists

Full table in `w14-audit/no-oracle-table.tsv`. Every one of the 43 declares substitutes; none is
empty (42 declare `specification-vectors, metamorphic-laws`, one declares `specification-vectors`
alone). Grouped by the *kind* of argument, and judged against the raised bar:

**Group A — "no third party speaks `.dsl.semio`, and our vocabulary IS the specification": 34 cases,
1,633 scenarios (95% of the remaining surface). This argument is now refuted inside this very
repository.** Exactly one of the 34 (`mutate-semio-any`, 43 scenarios) also carries a genuine
re-survey and names concrete blockers — it is broken out as Group B below, leaving **33 cases /
1,590 scenarios that carry the argument and nothing else**. The same sentence appears in the `noOracleDecision` of `mutate-block-3d-1` and
`mutate-block-5d-1` while their sibling `mutate-block-2d-1` — same plugin, same `.dsl.semio` carrier,
same "kind definition" document shape — carries `block-2d-python-independent`, a 513-line Python
implementation written from the committed snapshot schema, the committed mutations grammar and the
26 committed specification vectors. The same holds for `mutate-puzzle-{2d,3d,5d}-1` (181 scenarios)
against `mutate-fem{2d,3d}-1`, and for `mutate-cad-1` (41) against `mutate-gismap-1`.

The single largest case in the repository is in this group. `mutate-program-1` (**533 scenarios**,
`🏛️architect`) argues that its 266 kinds "are mechanically derived from this subset's own 66
registers by `📓️derivation-rules.md` … That derivation is a specification, not a fact an external
library could confirm or refute." **`📓️derivation-rules.md` and `📓️taxonomy.md` are precisely what
the fifteen `📕️norm` Python oracles were written from** — their headers say so verbatim — and those
fifteen are registered, dispatched second producers today. The premise is identical; the conclusion
is opposite. Nothing was *tried* and reported for `mutate-program-1`: its rationale contains no
survey of candidate implementations and no record of an attempt.

**Group B — a genuine, nameable technical blocker: 1 case, 43 scenarios (a subset of Group A's 34).** `mutate-semio-any`
(`semio-envelope-routing`) is the only decision in the repository that carries a **WAVE-14
RE-SURVEY** and rejects a second implementation on two concrete blockers rather than on absence:
(i) the envelope's own committed grammar declares the wrapped arm OPAQUE
(`document = artifact-mark subset-line REST`, `op = "noMutation" | tag ":" REST`), so an
envelope-level second implementation can reproduce both carriers byte for byte and still say nothing
about whether a *delegated* verb changed the arm it reached — which is what 18 of its 20 kinds
measure; (ii) the subset exports no JSON bridge for `SemioSnapshot`/`SemioMutation`, so the arm
snapshots can only be constructed inside the Rust subject crate. It names the two changes that would
unblock it. **This is the shape every one of Group A's 34 owes and does not have.**

**Group C — the format genuinely has no second producer, argued in detail: 5 cases, 74 scenarios.**

* `mutate-dwg-ac1018` / `mutate-dwg-ac1024` (7 + 7). DWG is proprietary; the only implementation of
  weight is **LibreDWG, GPL-3.0 C**, which cannot be linked into this test host, and `dxf 0.6` reads
  DXF and explicitly not DWG. The substitute is an independently hand-written reader of the one
  publicly specified part (the R13+ preamble at 0x00–0x05 / 0x12 / 0x13–0x14) against the real
  148,638-byte committed architectural drawing. Narrow and says so. **This is a correct decision.**
* `mutate-jpg-jfif-1-01-baseline` (21) and `mutate-tiff-6-0-baseline` (19). The vocabulary addresses
  frame-header/IFD *class-membership* axes (SOFn discriminant, DAC presence, per-class DHT count,
  sampling factors / `Compression`, `PhotometricInterpretation`, `BitsPerSample`, tile tags) that
  `image 0.25` cannot express, let alone write; and four to five of the kinds are normalised away by
  our own encoder, which is documented in the vocabulary's module docstring. Substitutes are ITU-T
  T.81 §4.2/§B.2.2/§B.2.4.2/Annex F and Adobe TIFF 6.0 Part 1 tag tables, cited by clause.
  **Correct decisions.**
* `mutate-binary-raw` (20). A raw byte buffer has no grammar. **Correct** — with one caveat below.

**Group D — not an interchange format at all: 4 cases, 12 scenarios.** `host-protocol-parity` (3),
`merge-conflicting-utilities` (3), `mutate-os-config-opening` (5), `reject-malformed-version-input`
(1). Out of scope for the bar.

Group A (34 / 1,633) and Groups C+D (9 / 86) partition the 43; Group B is the one member of Group A
that did the work the other 33 owe.

### 2.2 The weak point inside the "correct" decisions

`mutate-binary-raw` and `mutate-txt-utf-8` both name, as their second producer, *"an independently
hand-written reference implementation … in this subset's own oracle module"* — that is **our code, in
our language, in our crate, by our authors**. It is a genuinely separate implementation from the
production one, and both cases are honestly typed `@no-oracle-` so no false differential is claimed.
But under the raised bar it is not a second producer, and neither case's decision says so.

### 2.3 One orphan

The registry holds **44** `noOracleDecision`s and 43 are claimed by a feature; the orphan is
`os-config-merge-policy-unmounted-facet`. It also holds **122** oracle entries of which 121 are
claimed; the orphan is `csv`. No contract rule reports an unclaimed registry entry — only the reverse
(`unknown-oracle`, `unknown-no-oracle-decision`) is checked — so a decision can outlive the case that
justified it without anything noticing.

---

## 3. Are the 42 new oracles genuinely independent?

This is the failure mode the wave most invites, and it would make the parity number meaningless. Every
one of the 42 was read.

### 3.1 The mechanical checks — all clean

Over all 44 committed `🧪️tests/**/🐍️component.py` and the one `🟦️component.ts` reference:

* **Imports.** The complete import census is `json` (41), `copy` (21), `re` (15), `struct` (10),
  `io` (2), `os`, `decimal`, `csv`, plus `from semio_repo_test import Adapter, Context, Outcome[,
  digest]` (44) and exactly three third-party imports: `from PIL import Image`, `import pypdf`,
  `import simplejson`. **Nothing imports a semio production module.**
* **No FFI, no shell-out.** `subprocess`, `os.system`, `popen`, `ctypes`, `cffi`, `dlopen`,
  `importlib`, `cargo`, `wasm`: **zero hits** in any adapter. The only regex hits for `.rs` are
  docstring citations of which specification document was read.
* **No transliteration tells.** Function inventories are Python-idiomatic (`apply_mutation`,
  `inverse_mutation`, `read_varint`, `write_blob`, `column_index`, `derive_document_from_csv`), not
  Rust names in snake case. None of the 42 contains a per-kind dispatch table copied from
  `🧬️mutations/<kind>/…/🦀️component.rs`; `mutate-semio-table`'s header states outright that *"no file
  under `🧬️schema/🧬️mutations/<kind>/{🦠️mutation,↩️inverse,🔺️diff}/🦀️component.rs` was read"*, and the
  code bears that out — its verbs are driven off the committed DSL grammar's own productions.
* **They implement the carrier, not just the algebra.** 41 of the 42 read a real `.dsl.semio` /
  `.pack.semio` / `.dsl` file and parse it themselves — varint readers, magic-byte checks, bracketed
  record grammars, value-tag ordinals derived from the grammar's own `Z B I F S Y L M R` order and
  then pinned by re-encoding a committed example **byte for byte**. Only `mutate-block-2d-1` never
  crosses the carrier: its only artifact is a `local://…snapshot.json`.

**Verdict: none of the 42 merely re-expresses our Rust.** No case has to be named as a wrapper.

### 3.2 Where "independent" is nevertheless carrying less weight than the count suggests

Three real qualifications, none of which makes an oracle fake, all of which shrink the number:

1. **42 registrations are not 42 implementations.** 5-gram Jaccard over the committed Python
   adapters, docstrings stripped: the **fifteen `📕️norm` oracles are 0.672–0.875 mutually similar**,
   with the top pairs at 0.875 / 0.861 / 0.854. They are one generic verb engine — `normalised`,
   `singular`, `plural`, `collection_key`, `target_slot`, `derived_view` — instantiated fifteen
   times. `mutate-fem2d-1` / `mutate-fem3d-1` are a declared 2D/3D sibling pair. The **nineteen
   `🧿️semio` oracles are genuinely distinct** (max pair 0.479, min 0.057). Honest count of distinct
   second implementations: **≈27, not 42.**
2. **For the semio-native subsets, the "specification" the second implementation was written from is
   partly our own Rust.** Every `🧿️semio` header cites
   `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`'s envelope region as the carrier's
   *normative description*. That is defensible — the envelope has no other spec, and the Python was
   written against the described format rather than the code path — but a reader should know that
   agreement on the envelope is agreement between two readings of one Rust file, whereas agreement on
   the body is agreement against a committed `.grammar.semio` / `.protocol.semio` and pinned by exact
   byte reproduction of a committed example. Only the second half is fully independent evidence.
3. **The `📕️norm` engine and our Rust are both derived from the same two documents.** Both were
   written from `📓️taxonomy.md`'s closed verb table and `📓️derivation-rules.md`'s shape rules. Two
   correct derivations of one generative rule agreeing is weaker evidence than an unrelated library
   agreeing — it catches implementation slips, not a misreading of the rule itself. The decisions say
   so; the parity number does not.

### 3.3 Two genuinely third-party references that no manifest declares — still open from w16

* **Pillow.** `mutate-semio-image`'s reference does `from PIL import Image`
  (`🐍️component.py:520`) and its registration
  (`🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧪️oracle/🔣️component.json`) declares
  `"ecosystem": "python", "package": ""`. **Pillow appears nowhere in `./🔒️dependencies.json`** — that
  file has `pypdf 6.14.2` and `simplejson 4.1.1` and no Pillow. Re-verified in this audit against the
  runner's own venv, which is created `--system-site-packages` (`📜️script.ts:381`):

  ```
  $ .🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-cacdfc3fbe53ad5f7f5baff78016644f/bin/python \
      -c "import PIL; print(PIL.__version__, PIL.__file__)"
  11.3.0 /Users/ueli/Library/Python/3.9/lib/python/site-packages/PIL/__init__.py
  ```

  The one conversion with a *real* third-party producer resolves it out of **the developer's own user
  site-packages**. On a devcontainer, on CI, on a second machine, that import raises
  `ModuleNotFoundError` and the case's oracle half produces nothing.
* **three.js.** `mutate-semio-mesh`'s reference does `import * as THREE from "three"` and builds real
  `THREE.BufferGeometry` objects (`🟦️component.ts:830-837`) — a genuine second producer for the
  geometry half. Its registration also declares `"package": ""`. The adapter's prose and its
  registration's `rationale` both say **r185**; what resolves is `node_modules/three@0.182.0`, **r182**.
  `three` *is* in `🔒️dependencies.json` — as a `production-runtime` js dependency of five
  `package.json` files — so an oracle now reads a production library, which is exactly the
  `production-debt` pattern the gate reports for `png`, `zip` and `image`, and it reports nothing,
  because no oracle registration names the package.

**42 of the 122 oracle registrations declare an ecosystem with `"package": ""`.** For 40 that is the
honest "second implementation, no distribution" convention. For these two it hides a real third-party
reference from the only gate that would have caught it, and there is still no contract rule for *"an
oracle adapter imports a package no manifest declares."* This is why §1.4's `entries=232 /
test-oracle=30` has been byte-identical for four consecutive waves while the reference surface grew.

### 3.4 Not every oracle-backed scenario claims a second PRODUCER — 835 of them claim only a second READER

Counted by scenario mode (`w14-audit/modes.ts`):

| | differential | round-trip | property | conformance | error |
|---|---|---|---|---|---|
| oracle-backed (3,191) | **2,356** | 130 | 703 | 2 | — |
| `@no-oracle-` (1,719) | 1 | 40 | 826 | 845 | 7 |

Only the 2,356 `@mode-differential` scenarios claim "a second implementation produced the same
result". The other 835 use the registered reference as an **independent reader** that projects both
sides — the fleet brief's §6 route for a library that parses but cannot re-serialize. That is
honestly typed, and the twelve `🏗️ifc` + `📐️step` cases are the clearest example: **`ruststep 0.4`
has no writer at all** (`ast::ser::to_record` only builds an in-memory `Record`; no `Display` impl on
`Exchange`/`DataSection`/`Record`/`Parameter`), so those cases declare **zero** differential
scenarios. Under the raised bar they are not yet at it, and they say so.

**And the fix the owner named is already installed on this machine.** All three IFC 2x3 MVD manifests
carry a paragraph headed *"STRONGER REFERENCE EVALUATED AND NOT ADOPTED"*: `ifcopenshell` **reads AND
writes** IFC and would make a genuine `@mode-differential` possible for every IFC case. Re-verified
here:

```
$ .🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-…/bin/python -c "import ifcopenshell; print(ifcopenshell.version)"
0.8.4.post1
```

Their stated reason for not adopting it is **not a capability gap**: putting a Python distribution on
a generated host's import path needs an `oracleHostPackages` entry in the *shared*
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json` (host packages resolve by owner prefix, and a
subset manifest is not a prefix of the case owner `🗿️artifacts/🏗️ifc`), and the wave brief forbade an
executor from editing that shared file. **A governance rule, written for a parallel-execution wave, is
the only thing standing between five IFC cases (75 scenarios) and the exact oracle the owner asked
for.** That is one manifest edit.

---

## 4. Is every fixture genuinely real and complex?

**No.** Measured by resolving every `asset://` / `shared://` / `local://` URI in every feature through
the repository's own `fixtureUrisIn` + `resolveFixtures` and stat-ing the result
(`w14-audit/fixtures.ts`, raw in `fixtures2.json`). *Specification vectors* — fixtures whose URI runs
through `🧬️mutations/<kind>/🧪️tests/` — are excluded from "artifact" size, because a handcrafted
per-kind `(before, mutation, after)` triple is a third statement of the verb's meaning, not a
real-world document. **No fixture is missing and none resolves to a zero-byte file.**

### 4.1 Cases that read no real-world artifact at all — 27 cases, 538 scenarios

Eleven of them are `mutate-*` cases whose *entire* fixture set is handcrafted specification vectors:

```
   83 sc  mutate-block-5d-1        (205 vectors, 0 artifacts)
   75 sc  mutate-block-3d-1        (185 vectors, 0 artifacts)
   71 sc  mutate-puzzle-3d-1       (175 vectors, 0 artifacts)
   57 sc  mutate-puzzle-5d-1       (140 vectors, 0 artifacts)
   53 sc  mutate-puzzle-2d-1       (130 vectors, 0 artifacts)
   41 sc  mutate-cad-1             (100 vectors, 0 artifacts)
   35 sc  mutate-lowpoly-1         ( 85 vectors, 0 artifacts)
   29 sc  mutate-procedural-2d-1   ( 70 vectors, 0 artifacts)
   29 sc  mutate-procedural-3d-1   ( 70 vectors, 0 artifacts)
   19 sc  mutate-assembly-1        ( 45 vectors, 0 artifacts)
    5 sc  mutate-os-config-opening (  0 fixtures)
```

That is **497 scenarios of `mutate-*` evidence with no real-world artifact anywhere in the case.**
All eleven are `@no-oracle-`. The remaining sixteen are the eleven `create-*` synthesis cases (which
build a document from nothing by design — `create-minimal-pdf`, `create-and-round-trip-{bmp,gif,obj,
png,stl,tiff}`, `create-and-read-jpeg`, `create-and-retune-wave`, `create-and-edit-archive`,
`zlib-round-trip`) plus five non-format cases.

### 4.2 The newly-oracled surface is overwhelmingly small artifacts

Of the 42 conversions (1,860 scenarios), by largest real artifact the case reads
(`w14-audit/conversions.tsv`):

| largest artifact | cases | scenarios |
|---|---|---|
| < 4 KiB | **30** | **1,321** |
| < 16 KiB | 34 | 1,541 |
| < 64 KiB | 35 | 1,567 |
| ≥ 64 KiB | **7** | **293** |

Named, because it matters which:

```
  172 B   28sc  mutate-semio-video        📚️examples/🎥️clip/🗣️example.dsl.semio
  203 B   22sc  mutate-semio-text         📚️examples/📃️note/🗣️example.dsl.semio
  215 B   21sc  mutate-en1990-1           📚️examples/📕️high-consequence-office/…
  226 B   31sc  mutate-semio-audio        📚️examples/🎵️tone/🗣️example.dsl.semio
  278 B   40sc  mutate-semio-animation    📚️examples/🚶️walk/🗣️example.dsl.semio
  297 B   34sc  mutate-semio-graph        📚️examples/🕸️wires/🗣️example.dsl.semio
  …  (all 15 📕️norm cases: 215 B – 4,128 B)
  380 B   28sc  mutate-semio-object       537 B  40sc  mutate-semio-brep
  560 B   49sc  mutate-semio-cad          610 B  55sc  mutate-semio-document
  734 B   46sc  mutate-semio-kit
```

The seven that do meet the bar, and meet it well:

```
   85,791 B  52sc  mutate-semio-drawing
  119,066 B  34sc  mutate-semio-model         (Nakagin Capsule Tower)
  131,252 B  40sc  mutate-semio-flow          (Nakagin Capsule Tower)
  183,293 B  46sc  mutate-semio-presentation
  188,746 B  52sc  mutate-semio-mesh          (derived from a real glTF: 271 meshes, 459 primitives)
  391,703 B  40sc  mutate-semio-image         (frames decoded by Pillow from a real animated GIF)
  433,268 B  29sc  mutate-semio-value
```

`mutate-semio-table` (24,399 B) deserves its own note as the best-documented derivation in the
repository: its `.dsl.semio` was derived once from the real committed 50-row German
building-material-reuse survey `📊️reuse-marketplaces.csv`, and a `payload-fidelity` scenario
**re-derives it on every run through Python's own `csv` module**, so the fixture cannot silently
drift from the real data it claims to carry.

### 4.3 Repo-wide, the same shape

Over all 105 oracle-backed `mutate-*` cases (3,150 scenarios): **26 cases / 1,088 scenarios** rest on
an artifact under 2 KiB. Counting all 121 oracle-backed cases: 34 cases / 809 scenarios under 1 KiB,
46 / 1,300 under 4 KiB, 66 / 1,924 under 64 KiB.

The old third-party-backed half of the repository is where the real artifacts are, and they are
genuinely real: a 6.3 MB LaTeX bachelor thesis (10 PDF cases), a 17.5 MB scanned TIFF, a 6.0 MB BMP,
a 4.4 MB GIF, a 2.7 MB MP4, a 2.5 MB Nakagin Capsule Tower IFC 4, a 1.6 MB ZIP, a 1.4 MB OBJ, an
873 KB PLY, a 484 KB JPEG floor-plan scan, a 425 KB model JSON, 194 KB IFC 2x3, 150 KB HTML.

### 4.4 Seventeen zero-byte committed example assets, unchanged since w16

12 × `🎒️example.pack.semio` plus `🎞️example.gif`, `🎞️example.pptx`, `💬️example.bcf`,
`📕️example.xlsx`, `📷️example.png` are committed at **0 bytes**. No case resolves a fixture to any of
them (the smallest fixture any case reads is 40 B), so no law is quietly passing on empty bytes — but
they are dead files shipped as examples, and nothing reports them.

---

## 5. Coverage and the parity ratio

### 5.1 What `parity` counts, and why it is the only number that answers the raised bar

`evaluateParity` (`📦️index.ts:1819-1835`) keys oracle results by `owner::case::scenario` and walks
the **subject** results; a scenario enters the ratio only when **both** producers emitted a
projection for it. So the denominator is not "scenarios that exist" and not "scenarios with an
oracle" — it is **scenarios where two producers actually both ran**. That is precisely THE STANDARD's
"a second independent implementation produced the same result", and it is why the oracle phase's
`executed=3148` must not be read as coverage of the bar: `oracle exhaustive` reports `parity=0/0` by
construction.

### 5.2 Scenarios that execute in at least one phase

The oracle phase alone now reaches **3,148 of 4,910 scenarios (64.1%)**, up from w12's
**1,331 of 4,562 (29.2%)** and w13's identical 1,331. That is the single largest genuine improvement
in this ticket's history, and it is what the 42 conversions bought.

The remaining **1,762 scenarios (35.9%)** reach no oracle: 1,719 in the 43 `@no-oracle-` cases, whose
stated fallback is the subject phase, plus the 43 of `mutate-gif-89a` whose host failed to build.

**Does the subject phase rescue any of them? Almost none.** Of the 43 `@no-oracle-` cases, 40 have a
Rust subject and no Rust subject host links today (§5.3). The three with a non-Rust subject —
`merge-conflicting-utilities` (3), `host-protocol-parity` (3), `reject-malformed-version-input` (1) —
do execute. So **scenarios executing in at least one phase: 3,148 measured, at most 3,155 of 4,910
(64.3%)**, against w12's **1,331 of 4,562 (29.2%)**. The 1,716 scenarios of the 40 remaining
`@no-oracle-` cases execute in **no phase at all** — the stated fallback for all of them is a phase
that cannot run.

### 5.3 The repo-wide parity ratio — and it is not a percentage, it is a blocker

**Measured directly, twice, on two different owners: no Rust subject host in this repository links
today, so no oracle-versus-subject comparison can be formed for any Rust case.** Both probes were run
as single cases so the result is unambiguous:

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-zip-2-0      # exit 1
[test] level=exhaustive cases=1 executed=15 passed=15 failed=0 errored=0 parity=0/0
[test] ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/mutate-zip-2-0: rust subject host exited 101 without emitting results
error[E0432]: unresolved import `component::component_persistent_local`
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error; 103 warnings emitted
[test] …/mutate-zip-2-0: no result stream at …/test-…-mutate-zip-2-0-subject-rust/📤️results.jsonl
```

```
$ bun ./📜️script.ts parity exhaustive --owner 📕️norm --case mutate-en1990-1      # exit 1
[test] level=exhaustive cases=1 executed=21 passed=19 failed=2 errored=0 parity=0/0
[test] ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990/🧪️tests/mutate-en1990-1: rust subject host exited 101 without emitting results
error[E0432]: unresolved import `component::component_persistent_local`
error: could not compile `semio-framework-plugin` (lib) due to 1 previous error; 103 warnings emitted
```

**One framework-level error, not a per-plugin one.** `semio-framework-plugin` is in every generated
Rust subject host's dependency graph, so `🗄️stdio` and `📕️norm` fail identically and so will the
other 32 plugins. **Attribution: a live peer session, not this ticket.** `component_persistent_local`
IS defined (`💻️os/🔨️modules/🔌️plugin/🦀️component.rs:57`, `pub mod`, plus a macro at `:86`), and the
four files that name it were last written at **05:18, 04:36, 04:33 and 01:12 today** — the module is
being refactored underneath the build while this audit runs. This is w21 §1's finding again, at a new
line.

**So the honest answer to "what is the repo-wide parity ratio" is: `0` comparisons over `0`
attempted, for every Rust-subject case in the repository.** The only cases that can currently
contribute a non-zero parity are the four with a non-Rust subject
(`satisfy-version-requirements`, `flatten-class-name-inputs`, `compile-style-variants`,
`host-protocol-parity`) — 13 scenarios out of 4,910.

Two consequences that must not be smoothed over:

1. **The summary line for `mutate-zip-2-0` reads `executed=15 passed=15 failed=0 errored=0
   parity=0/0` — green — for a case whose subject half did not exist.** The only signals are the
   trailing `problems` lines and the exit code. This is w12 remedy #7 / w13 remedy #2, still open at
   `📜️script.ts:520-541`, and it is the mechanism that lets a reader mistake §1.2's `passed=3143` for
   evidence under the raised bar.
2. **`mutate-zip-2-0`'s own adapter docstring (`🦀️component.rs:10-13`) states `parity=15/15`.** It is
   `0/0` today. A number frozen into prose is not evidence.

### 5.4 Where the repo-wide command stood

`bun ./📜️script.ts parity exhaustive` (repo-wide) was attempted twice here, both times detached.

* **Attempt 1** (04:43 → 05:08). Reached four cases in 25 minutes: `merge-conflicting-utilities`,
  then **≈13 minutes inside `mutate-os-config-opening` alone**, then `host-protocol-parity`, then
  `mutate-present-1`. Killed at 05:08 by an external `SIGTERM` (`PARITY_EXIT=143`) that also took
  down a *concurrent session's* `parity exhaustive --owner 🗄️stdio` — so the pause is this machine's,
  not the command's. No summary line.
* **Attempt 2** (from 05:09, still running as this is written). With the machine no longer
  contended it cleared `mutate-present-1` — the exact case whose 900 s overrun destroyed the
  `📓️w16-final-audit.md` attempt with `spawnSync cargo ETIMEDOUT` and no `[test] level=…` line at
  all — and was at `mutate-dag-1` after 25 minutes, roughly the eighth of 164 cases. It is walking
  the plugins alphabetically and will not reach `🗄️stdio`, the 94-case bulk, for hours.
  `runProbe` still throws on `ETIMEDOUT` and nothing catches it
  (`📚️library/📦️packages/🟦️typescript/📦️index.ts:1699-1712`, `🧪️test/📜️script.ts:529`), so any
  single slow case still discards the whole run.

The per-case budget is applied to `cargo run`, which **includes compiling the host from scratch**, so
a cold subject host for a plugin nobody has built before can consume the entire 900 s without
anything being wrong. At the observed pace a repo-wide pass is a multi-hour operation that can abort
at any case and discard everything measured so far.

**The prescribed headline command still cannot be relied on to produce the headline number on this
machine — and §5.3 shows that even if it completed today, the number would be zero.** The two
isolated single-case runs in §5.3 are what this audit stands on instead: same command, same phases,
scoped so the answer cannot be lost.

---

## 6. Did anyone weaken evidence to raise a number?

**No.** Checked eight ways against `HEAD` (`8d9b51f081`), which is the correct baseline because every
conversion in this wave is uncommitted working-tree work.

1. **Comparison profiles.** `git diff HEAD -- '*🔣️component.json'` filtered to lines that actually
   declare a knob — `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"` — returns **nothing**. 61 manifest
   files changed, +1,651/−375 lines, and not one of them is a profile knob.
2. **`ignoreKeys` / tolerance repo-wide.** The full `git diff HEAD` contains **no** added or removed
   `ignoreKeys` or `"tolerance"` line outside (a) prose inside `description`/`rationale` strings and
   this ticket's own notes, and (b) two `json!({ … "tolerance": tolerance … })` request payloads in
   unrelated UI code.
3. **Scenarios.** 164 features before, 164 after. `git diff --name-status HEAD -- '*component.feature'`
   shows **58 M, 0 A, 0 D, 0 R**. Re-parsed with the repository's own `parseFeature` and compared by
   scenario **id**: across those 58 files, **2,086 → 2,432 scenarios, 346 gained, ZERO lost**
   (`w14-audit/cmp.ts`). Every gain is a `spec-vector-<kind>` scenario — the committed-vector claim
   the conversions moved out of the differential path was re-added as its own scenario, not dropped.
4. **Comparison tag per case.** All 42 conversions kept `@comparison-ordered-json-v1`; not one moved
   to a looser profile.
5. **Fixtures.** Over `*🧫️fixtures*` and `*📚️examples*`: **88 added, 69 modified, 0 deleted, 0
   renamed.** 66 of the 69 modifications are `.rs` files. The only three non-`.rs` modifications are
   `📄️pdf/…/📚️examples/🎬️demo/🖼️assets/{📄️example.pdf,🎒️example.pack.semio,🗣️example.dsl.semio}`,
   regenerated after the PDF 1.4 page-tree rewrite — **and no case reads any of them**; all ten PDF
   cases read `📚️examples/🎓️bachelor-thesis/…/📄️bachelor-thesis.pdf`, which is untouched. Every
   fixture a case actually reads is byte-identical to HEAD.
6. **The migration ratchet.** `./🔒️migration.json` unchanged since `a2746cd371` (2026-08-23 20:01),
   clean in the working tree. Nobody raised the allowance to make §1.1 green.
7. **Law call sites in case adapters.** Counted per file over the changed case adapters:
   * `mutate-gisterrain-1` / `mutate-gismap-1` **11 → 13**, `mutate-fem2d-1` / `mutate-fem3d-1`
     **11 → 12**, `mutate-tiff-6-0-baseline` **6 → 7**, and fourteen `mutate-semio-*` cases **2 → 3**
     (`mutate-semio-value` **1 → 3**). Assertions **added**.
   * The fifteen `📕️norm` cases went **12 → 10**, and the removed lines are
     `law::mutation_is_observable(...)` and the `law::divergence(...)` rejection arm **inside the
     Rust ORACLE handler** — this repository's own answer standing in the oracle's chair. Removing
     them when a Python reference took the role is a **strengthening**, and the Python side restates
     both by hand in its own `🔖️Laws` region (`observable` / `untouched` / `restores`).
   * **One genuine removal:** `mutate-block-2d-1` **3 → 0**. `law::mutation_is_observable`,
     `law::reparsed_not_copied` and `law::round_trip_preserves` are gone from its adapter. The first
     two are restated by hand and by the Python reference; the third — the `.dsl.semio` **carrier**
     round-trip law — is genuinely no longer asserted anywhere, and the adapter says so in three
     places with the reason (the subset's `store::ArtifactDsl` impl is handwritten `async` while the
     generated host is synchronous). It is a real loss, stated, and it is the same case that in
     exchange went from not linking the plugin crate at all to driving the real
     `block2d_mutation_report_json`.
8. **A caveat about the Python law surface.** The Python host
   (`🧰️framework/…/🧪️test/📦️packages/🐍️python/🐍️host.py`, 212 lines) exposes `Adapter`, `Context`,
   `Outcome`, `digest` — and **no `law` module**. So the 42 conversions cannot call the shared
   `⚖️law` module at all; each restates the laws by hand (`raise AssertionError(...)`, 20–47 per
   file; the 15 `📕️norm` and `mutate-block-2d-1`/`fem`/`gis`/`raster` files carry an explicit
   observability check). Nothing was weakened, but **the single source of truth for what a law means
   now has 42 hand-written cross-language copies and no gate compares them.**

**Nothing found that raises a number by lowering a bar.** The one assertion genuinely removed
(`mutate-block-2d-1`'s carrier law) is documented in place and is not attached to any number that
went up.

---

## 7. Stale claims and housekeeping still standing in the tree

* **`mutate-tiff-6-0-baseline/🦀️component.rs:19-20`** still says `identity-round-trip` goes through
  `law::reparsed_not_copied`. The handler calls `law::carrier_is_exact` (`:244`);
  `reparsed_not_copied` appears nowhere in the file. w16 flagged this; nobody fixed it.
* **`🎒️zip/🧪️tests/mutate-zip-2-0/🦀️component.rs:10-13`** asserts *"the subject phase RUNS … this
  case's own `parity exhaustive --owner 🗄️stdio --case mutate-zip-2-0` reports `parity=15/15`"* and
  cites `cargo check -p semio-framework-os-kernel --lib` (the **default** feature set) as proof the
  blocker is cleared. `📓️w21-four-blockers.md` §1 records that the identical crate fails to compile
  **inside the generated test host's own `[workspace]`** (`error: future cannot be sent between
  threads safely` at `💻️os/🔨️modules/🏪️store/🦀️component.rs:8321`) while the root-workspace check is
  exit 0. A number frozen into a docstring is not evidence, and this one is feature-set-dependent in
  exactly the way the line does not say.
* **`📋️status.md` is two orders of magnitude stale** — untouched since `215e369d07` (2026-08-23
  18:01, file mtime 14:04). Its dashboard says **11 cases / 32 scenarios / 212 dependencies / 10
  registered oracles** and its owner table says *"every other non-`compose` owner: discovered /
  surveyed"*. Live: **164 cases / 4,910 scenarios / 232 dependencies / 122 oracle registrations across
  37 owners.** Its explanatory paragraph — *"All six artifact owners are `oracle-green` rather than
  `parity-green` … because a concurrent session's os-kernel refactor is in flight"* — describes a
  2026-08-23 state. `📋️contract.md` (10:53) and `📋️architecture.md` (15:00) are frozen the same day.
  A reader who starts at the ticket's own status document is misled before reaching any audit.
* **`create-and-round-trip-bmp` and `create-and-round-trip-tiff` still have byte-identical feature
  descriptions** — the only 1.000 similarity pair in the repository, named at w12 and untouched since.
* **Orphan registry entries are unreported.** `csv` (oracle) and
  `os-config-merge-policy-unmounted-facet` (no-oracle decision) are declared and claimed by nothing.
* **`runProbe` still throws on `ETIMEDOUT`** and nothing catches it between there and `run`
  (`📚️library/📦️packages/🟦️typescript/📦️index.ts:1699-1712`, `🧪️test/📜️script.ts:529`). This is
  w13 remedy #10 and w16 §1.3; it is what turns one slow case into a whole run with no summary line.
* **A case whose host fails to build still contributes `executed=0 passed=0 failed=0`**
  (`📜️script.ts:520-541`). This is w12 remedy #7 / w13 remedy #2, still open, and it is the single
  mechanism that lets §0's "oracle coverage up, comparisons flat" stay invisible in the summary line.
* **`⚡️cache/tests/reports/latest` is shared, not per-run.** Two sessions running the platform at the
  same time overwrite each other's `📊️summary.json`, `📤️results.jsonl`, `📋️junit.xml` and
  `📈️metrics.json`. Observed live during this audit (§1). There is no run id in the path.

---

## 8. What each of the five oracle-phase divergences means

Asked as the brief asks it — *our codec, their library, or the fixture?*

| divergence | ours / theirs / fixture | why |
|---|---|---|
| `en1990 insert-variable-action` (×2) | **ours — a specification hole, not a codec bug** | Our Rust can produce the composed child's `childId` because it *is* the content-addressing function. No committed document states that function, so no second implementation can reproduce it. The remedy is to publish the addressing rule, not to relax the comparison. |
| `iso16757 / vdi3805 identity-round-trip` (×2) | **ours — a missing grammar** | Both subsets write a `.dsl.semio` notation that nests records and tables and flattens nested records into `key=key=…`; their committed `📖️component.grammar.semio` is the repository-wide placeholder `payload = OCTET+`. The carrier is unreadable by anyone who was not told, in code, how it is shaped. |
| `jack spec-vector-create-node` | **the fixture** | The committed vector declares a refusal; both producers apply the mutation. Either the vector is wrong or the vocabulary is — and this is the one class of finding a specification vector can produce that neither implementation could. |

None is attributable to a third-party library, because none of the five is on a third-party-backed
case. That is itself informative: **the reference libraries are not where this repository's
divergences are.** They are in the semio-native carriers, where the specification is thinner than the
implementation.

---

## 9. Remedies, in order of leverage

1. **Convert the 34 Group-A `@no-oracle-` cases** (1,633 scenarios) using the recipe this wave proved:
   a second implementation in Python or TypeScript written from the committed
   `📖️component.grammar.semio` + `📡️component.protocol.semio` + `🧬️mutations/🔣️component.json` +
   per-kind specification vectors, pinned by re-encoding a committed example byte for byte. Start with
   `mutate-program-1` (533 scenarios, 45% of the remaining no-oracle surface) — the `📕️norm` verb
   engine was written from the same two documents its rationale cites as un-adjudicable.
2. **Replace the toy artifacts.** 30 of the 42 conversions run on artifacts under 4 KiB, and 11
   `mutate-*` cases (497 scenarios) read no artifact at all. `mutate-semio-mesh`,
   `mutate-semio-model`, `mutate-semio-flow` and `mutate-semio-table` show how it is done — derive
   once from a real committed artifact and re-derive on every run.
3. **Publish the two specifications the second implementations proved missing** (§8): the
   content-addressing function for composed child slots, and a real grammar for the nesting
   `.dsl.semio` notation that `iso16757`/`vdi3805` write. Four of the five red scenarios close at the
   cause, and no comparison profile moves.
4. **Adopt `ifcopenshell` for the five IFC cases and lift the shared-manifest rule** (§3.4). It is
   installed, it reads and writes, and it is exactly the reference the owner named. Twelve
   `🏗️ifc` + `📐️step` cases currently declare zero differential scenarios because `ruststep` cannot
   write.
5. **Declare Pillow and three.js.** Add them as `test-oracle` entries with real `package`/`version`,
   and add the contract rule that has no counterpart today: *an oracle adapter that imports a package
   no manifest declares is a breach.* Without it the dependency gate will keep printing `entries=232`
   forever.
6. **Fix the two runner defects that hide evidence** — `runProbe` throwing on `ETIMEDOUT`
   (§7), and `executeOne` returning `results: []` for a host that failed to build so the case
   contributes `0/0/0`. Both are open since w13.
7. **Give `reports/latest` a run id.** Two sessions overwrite each other's report today (§1, §7).
8. **Split `not-exercised`** into "recorded no-oracle decision" and "host failed" — `mutate-gif-89a`
   is currently counted with the 43 policy decisions.
9. **Refresh `📋️status.md`, `📋️contract.md`, `📋️architecture.md`**, and delete the two stale claims
   in §7 and the 17 zero-byte example assets.

---

## 10. Totals

| | w12 (`c3a79bd4ce`) | w13 (`9ed590cd87`) | w16 | **now** |
|---|---|---|---|---|
| cases | 164 | 164 | 164 | **164** |
| scenarios | 4,562 | 4,564 | 4,910 | **4,910** |
| `@oracle-` cases / scenarios | 79 / 1,331 | 79 / 1,331 | 121 / 3,191 | **121 / 3,191** |
| … of which `@mode-differential` scenarios | — | — | — | **2,356** |
| `@no-oracle-` cases / scenarios | 85 / 3,231 | 85 / 3,233 | 43 / 1,719 | **43 / 1,719** |
| oracle registrations | — | 80 | 122 | **122** |
| … of which a third-party package | — | 79 | 79 | **79** |
| … of which an in-repo second implementation | — | 0 | 42 | **42** |
| distinct second implementations (5-gram Jaccard) | — | 0 | — | **≈27** |
| scenarios executed by the oracle phase | 1,331 | 1,331 | *(unmeasured)* | **3,148** |
| scenarios executing in **any** phase | 1,331 / 4,562 | 1,331 / 4,564 | *(unmeasured)* | **≤ 3,155 / 4,910 (64.3%)** |
| oracle-phase failures | 0 | — | *(unmeasured)* | **5** |
| **oracle-vs-subject comparisons (`parity`)** | **0 / 0** | 1,012 / 1,277 (stdio) | *(unmeasured)* | **0 / 0 — no Rust subject host links** |
| contract breaches | 0 | 0 | 2 | **2** (`unmanaged-tests`, other sessions) |
| TS suite | 69 / 0 fail | 69 / 0 | 66 / 3 | **67 / 2**, 2,057 `expect()` |
| stdio oracle crate `cargo test` | 369 / 367 ok | — | *(unmeasured)* | **374 / 372 ok** |
| dependency entries / test-oracle | 232 / 30 | 232 / 30 | 232 / 30 | **232 / 30** |

**One sentence.** The wave bought a genuine, large, honestly-built reference surface — 42 real second
implementations in two other languages, 346 new scenarios, five findings that only a second producer
could have surfaced, and not one weakened assertion — and it bought **zero** additional evidence
under the raised bar, because the raised bar is measured by `parity`, `parity` needs a subject, and no
Rust subject in this repository compiles today.

---

## 11. Method

* Population, tags, oracle/no-oracle attribution, adapters: the repository's own `discoverTestCases`
  and `parseFeature`, via `w14-audit/survey.ts` → `survey.json`. Not grep.
* Fixtures: the repository's own `fixtureUrisIn` + `resolveFixtures`, via `w14-audit/fixtures.ts` →
  `fixtures2.json`; sizes stat-ed on disk; `🧬️mutations/<kind>/🧪️tests/` URIs classified as
  specification vectors, not artifacts.
* Scenario gain/loss: `w14-audit/cmp.ts` re-parses each changed feature at `HEAD` and in the working
  tree and diffs the scenario **id** sets.
* Similarity: 5-gram Jaccard over token streams with docstrings stripped.
* Registry orphans: `w14-audit/orphan.ts`.
* Every exit code was read from the tool's own exit status. No number in this document came from a
  pipeline's exit code, and no `[test]` line was paraphrased.
