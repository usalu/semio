# Wave 15 — how many second implementations there really are, and what `@mode-` actually gates

Date 2026-08-26. HEAD `8d9b51f081f42b36722b54f80a5c502d6322f9ca` (2026-08-25 14:57:24 +0200), dirty
tree — everything below is working-tree work. Raw scripts, logs and the pre-refactor adapters are in
`w15-work/`. Every `[test]` line is copied verbatim from the tool's own stdout; every exit code was
read from the tool's own exit status, never through a pipe.

Answers `📓️w14-final-audit.md` §3.2 finding 1 (*"42 registrations are not 42 implementations"*) and
§3.4 (*"only 2,356 of 3,191 oracle scenarios are `@mode-differential`"*).

> **Repo-wide parity could not be measured, and no number here is a parity number.** No generated
> Rust subject host links — `error[E0432]: unresolved import `component::component_persistent_local``
> in `semio-framework-plugin`, a live peer session's in-flight refactor under
> `💻️os/🔨️modules/🔌️plugin/⚛️reactor`. Every measurement below is either the oracle phase alone, a
> static survey through the repository's own `discoverTestCases`/`parseFeature`, or an offline
> replay harness. Nothing here claims a subject ran.

---

## 1. The fifteen `📕️norm` oracles were not "0.672–0.875 similar". They were one file.

### 1.1 The real similarity, and why the audit's figure was low

Re-measured with the audit's own method — 5-gram Jaccard over the token stream, docstrings stripped —
but stripping the docstrings through an AST round-trip so that indentation and line-wrapping cannot
inflate the *difference* (`w15-work/🐍️similarity.py`):

```
pairs=105 min=0.7972 max=0.9458 mean=0.8923
```

against the audit's reported 0.672–0.875. The gap is method, not drift: comparing raw text counts
re-wrapped prose as different tokens.

### 1.2 The measurement that settles it

Similarity was the wrong instrument. Strip each file's module docstring, strip its
`# region 🔖️Vocabulary … # endregion 🔖️Vocabulary` block, and normalise the one docstring sentence
that names the standard, then hash what is left:

```
d8ea262f6b2f3e71 mutate-iso16757-1     d8ea262f6b2f3e71 mutate-en1994-1
d8ea262f6b2f3e71 mutate-vdi3805-1      d8ea262f6b2f3e71 mutate-en1995-1
d8ea262f6b2f3e71 mutate-din4108-1      d8ea262f6b2f3e71 mutate-en1996-1
d8ea262f6b2f3e71 mutate-din16798-1     d8ea262f6b2f3e71 mutate-en1997-1
d8ea262f6b2f3e71 mutate-en1990-1       d8ea262f6b2f3e71 mutate-en1998-1
d8ea262f6b2f3e71 mutate-en1991-1       d8ea262f6b2f3e71 mutate-en1999-1
d8ea262f6b2f3e71 mutate-en1992-1       d8ea262f6b2f3e71 mutate-din18599-1
d8ea262f6b2f3e71 mutate-en1993-1
```

**One hash, fifteen files.** Outside the per-subset data block, all fifteen adapters were
byte-identical — not similar, not near-copies, the same 800 lines fifteen times. The audit's honest
count of *"≈27 distinct implementations, not 42"* was itself generous: for these fifteen it is one.

### 1.3 What was done: one named module, imported

The engine now lives once, at

```
✏️s/🔌️plugins/📕️norm/🧪️oracle/📦️packages/🐍️python/semio_norm_vocabulary.py     (874 lines)
```

reached by all fifteen generated Python hosts through a declared `oracleHostPackage` in
`✏️s/🔌️plugins/📕️norm/🧪️oracle/🔣️component.json` — the same mechanism the plugin already used to
share the Rust `law` module, on the Python side for the first time in this repository:

```json
{
  "implementation": "python",
  "package": "semio_norm_vocabulary",
  "path": "✏️s/🔌️plugins/📕️norm/🧪️oracle/📦️packages/🐍️python"
}
```

An entry that carries `path` is in-repo source: `materializePythonHost` puts the directory on
`PYTHONPATH` and provisions nothing, so no distribution is added and the dependency ratchet is
untouched.

Each of the fifteen adapters is now its subset's DATA and nothing else — the closed kind list its
committed catalog declares, the committed specification vector per kind, the real committed example
document, and the envelope token — plus a four-line `adapter()`:

```python
def adapter():
    return build_adapter(Subset("EN 1990", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
```

```
15 adapters before:  13,534 lines
15 adapters now:      1,519 lines  +  874 lines of shared engine
```

Similarity after, same measurement:

```
pairs=105 min=0.0180 max=0.0807 mean=0.0426
```

The duplication is not gone — a shared bug in that engine still agrees with itself in all fifteen
cases. It is now **visible**: one import instead of fifteen copies, and the module's own docstring
says so in the second paragraph.

### 1.4 Proof the refactor changed no evidence

Not asserted — replayed. `w15-work/🐍️equivalence.py` loads the pre-refactor adapter (recovered from
the git index into `w15-work/old-adapters/`) and the post-refactor adapter side by side, drives both
through the fifteen plans the runner itself wrote into
`.🧬semio/🦑️repo/⚡️cache/tests/work/*norm*oracle-python/📋️plan.json`, and compares, per scenario, the
serialized projection, the `digest` of the raw bytes, and — where the handler raises — the exception
type and its full message:

```
[equivalence] scenarios=799 identical=799 mismatched=0
EXIT=0
```

And through the platform itself, before and after, same command, exit status read from the tool:

```
before  [test] level=exhaustive cases=15 executed=799 passed=795 failed=4 errored=0 parity=0/0   EXIT=1
after   [test] level=exhaustive cases=15 executed=799 passed=795 failed=4 errored=0 parity=0/0   EXIT=1
```

The same four scenarios are red, with the same messages, for the same reasons — see
`📓️w15-specification-defects.md`.

### 1.5 What the repo-wide duplication picture looks like now

5-gram Jaccard over all **44** committed Python case adapters, docstrings stripped, after the
refactor. Top pairs:

```
0.8204  mutate-fem2d-1  mutate-fem3d-1
0.4154  mutate-semio-kit  mutate-semio-object
0.4132  mutate-semio-audio  mutate-semio-video
0.3615  mutate-semio-animation  mutate-semio-audio
```

**Exactly one high-similarity pair is left**, and it was read line by line rather than scored:
`mutate-fem2d-1` / `mutate-fem3d-1` are a declared 2D/3D sibling pair and they are **genuinely
differentiated per artifact**, not copies —

* different members (`regions` ↔ `solids`), different record shapes (`nodes` gains `z`, `sections`
  gain `iz`/`j`, `materials` gain `g`),
* an `ELEMENTS` variant table that exists only in 3D (`frame` carries a `roll` about its own axis,
  `bar` does not) and a corresponding extra invariant in the document check,
* a different combination-term shape — 2D commits a list of `{caseId}` records, 3D commits a
  case-keyed map, and each implementation validates its own,
* different load variants (`regionId` ↔ `solidId`, `memberUdl` gains `wz`),
* different fixtures (`timber-portal-frame` ↔ `steel-frame`) and different structural invariants
  (2D asserts a ridge above both eaves; 3D asserts four ground corners with two storeys above).

That is the shape a legitimate sibling has. Everything else in the repository now sits at ≤ 0.42.

**Honest count of distinct second implementations: 28** — 42 registrations, minus the fifteen norm
copies, plus the one shared engine they now call.

---

## 2. `@mode-differential` does not gate the comparison. All 3,191 oracle-backed scenarios are compared.

This is the load-bearing correction to §3.4 of the audit, and it is measured, not read.

### 2.1 The mechanism

`evaluateParity` (`📦️packages/🟦️typescript/📦️index.ts:1819`) pairs every **subject** result with the
**oracle** result of the same `owner::case::scenario` and compares them. It never looks at the
scenario's mode. `mode` is consulted in exactly two places in the whole platform, both of which are
about `@no-oracle-` cases: the `differential-without-evidence` contract rule (`index.ts:1150-1158`)
and the substitute check at `📜️script.ts:632`.

### 2.2 The measurement

From `📓️w21-four-blockers.md`'s own verbatim run lines, cross-checked against this survey's scenario
counts:

| case | total scenarios | of which `@mode-differential` | measured parity denominator |
|---|---|---|---|
| `mutate-docx-ecma-376` | 27 | 13 | **27** (`parity=27/27`) |
| `mutate-json-rfc8259-i-json` | 22 | 10 | **22** (`parity=22/22`) |
| `mutate-xlsx-ecma-376-strict` | 19 | 9 | **19** (`parity=13/19`) |

In every case the denominator is the **total** scenario count, not the differential count.

**So the audit's "only 2,356 of 3,191 claim a second implementation produced the same result" is a
statement about the tags, not about the evidence.** When a subject links, all 3,191 are compared
against the second producer. The `@mode-` tag says what law the scenario states in its own right; it
has never decided what gets compared.

### 2.3 The 835, classified

Every non-differential oracle-backed scenario, grouped by what its own `Then` steps assert
(`w15-work/retype-survey.ts`):

| | scenarios | |
|---|---|---|
| in the twelve `🏗️ifc` + `📐️step` cases | **172** | a peer session owns these; untouched here |
| assert cross-producer agreement and nothing else | **290** | the tag is wrong |
| assert cross-producer agreement **plus** an inverse or round-trip law | **68** | the tag is right — the law is real |
| assert a law "in role" / through an independently read projection | **6** | genuinely not a two-producer claim |
| assert a law without naming either role (inverse restores, decode-encode identity) | **299** | the tag is right |
| | **835** | |

### 2.4 What was retyped, and what deliberately was not

Retyped `@mode-property` → `@mode-differential`: **223 scenarios, 21 outline blocks, 21 feature
files**, and only where *every* assertion of the block is a cross-producer agreement, so that the
`@mode-property` tag named a law the scenario does not state. Each of the 21 is backed by a real
third-party distribution with a pinned version — `lopdf 0.44`, `quick-xml 0.42`, `image 0.25`,
`dxf 0.6`, `las 0.11`, `zip 6`, `csv 1`, `calamine 0.36`, `html5ever 0.39`, `riff 2.0`, `json 0.12`,
`simplejson 4.1.1`, `stl_io 0.8`, `gif 0.13`:

```
mutate-las-1-0  mutate-html-5  mutate-gif-87a  mutate-svg-1-1  mutate-svg-1-1-basic
mutate-svg-1-1-tiny  mutate-bcf-2-1  mutate-pdf-1-4  mutate-pdf-1-7  mutate-csv-rfc4180
mutate-tsv-iana  mutate-xlsx-ecma-376  mutate-docx-ecma-376  mutate-xml-1-0
mutate-jpg-jfif-1-01  mutate-avi-1-0  mutate-json-rfc8259  mutate-json-rfc8259-i-json
mutate-dxf-r12  mutate-tiff-6-0  mutate-stl-ascii
```

Result, over the same 164-case population the audit measured (excluding the peer's brand-new
`differential-ifc-4`, created at 06:09 today):

```
oracle-backed scenarios      3191   3191
@mode-differential           2356 → 2579
non-differential              835 →  612
```

**Deliberately NOT retyped, with the reason:**

* **`mutate-epw-energyplus` (22 scenarios).** Its `Then` says the oracle and the subject agree, but
  its feature file states the opposite in prose and the prose is right: *"this subset's oracle module
  writes the bytes itself (hand-rolled, independent of the subject crate — the oracle role never
  links it) with no third-party validation of EPW grammar to lean on, so they are typed
  `@mode-property` … never claimed as agreement between two independent producers it does not have."*
  Its registered oracle is `csv 1` — a generic comma-grid reader, not an EPW producer. Retyping it
  would have inflated a count against the case's own honest reasoning.
* **The 68 that assert a law *and* the agreement**, and the 299 that assert a law alone. The tag
  names the law; the differential is measured anyway (§2.2). Flattening them would have traded one
  misleading number for another and erased the inverse/round-trip claim from the tag.
* **The 172 ifc/step scenarios.** A peer session is landing `differential-ifc-4` right now.
* **The 6 in-role / independent-reader scenarios.** These genuinely are a single producer plus a
  second reader, and they say so.

### 2.5 The contract, before and after

`bun ./📜️script.ts contract`, run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, exit
status read from the tool:

```
EXIT=1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery

  testing/discovery  🧰️framework  44 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  5 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

Byte-identical to the audit's §1.1 — the same two pre-existing `unmanaged-tests` breaches from other
sessions' test files, and **not one new breach** from 223 retypes, the new Python host package or the
fifteen rewritten adapters. `🔒️migration.json` was not touched.

(A later run showed four extra `testing/contract` breaches in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🧪️tests/differential-ifc-4/component.feature`, *"Step at
line 30 is outside a Background or Scenario"*. That directory was created at 06:09–06:13 today and
its file count changed between two runs six minutes apart: it is a peer session mid-write, not this
work.)

Spot-checks of two retyped cases through the platform, exit status read from the tool:

```
[test] level=exhaustive cases=1 executed=13 passed=13 failed=0 errored=0 parity=0/0    (mutate-csv-rfc4180)  EXIT=0
[test] level=exhaustive cases=1 executed=39 passed=39 failed=0 errored=0 parity=0/0    (mutate-dxf-r12)
```

### 2.6 The dependency gate, and proof the new Python host package adds no dependency

`bun ./📜️script.ts dependency` — **exit 0**:

```
[dependency] ecosystems=4 entries=233 production-reachable=151 test-oracle=31
```

`entries` 232 → 233 and `test-oracle` 30 → 31 after four waves of byte-identical figures, and the
one new row is **not this work**:

```
[dependency] test-oracle python:ifcopenshell@0.8.4.post1 (ifcopenshell-ifc-2x3-any-differential,ifcopenshell-ifc-4-any-differential)
```

That is the audit's remedy #4 — adopting `ifcopenshell` for the IFC cases — landing from the peer
session, on the same clock as `differential-ifc-4`. The three `production-debt` records (`png`,
`zip`, `image`) are unchanged.

**`semio_norm_vocabulary` appears nowhere in that listing, and that is correct**: an
`oracleHostPackage` that carries `path` is in-repo source put on `PYTHONPATH`, not a distribution to
provision, so it is neither a production nor a third-party dependency and the ratchet has nothing to
count.

### 2.7 The TypeScript suite

`bun test 🧪️index.test.ts` in `📦️packages/🟦️typescript` — **exit 1**, `65 pass / 4 fail`, 1,974
`expect()` calls, 69 tests. All four failures were attributed:

| failing test | cause | whose |
|---|---|---|
| `every committed case satisfies the frozen contract` | the two pre-existing `unmanaged-tests` breaches + the peer's `differential-ifc-4` step-placement breaches | pre-existing / peer |
| `the migration backlog is a shrink-only ratchet` | `Expected <= 1, Received 5` — other sessions' test files | pre-existing |
| `dependency ratchet … keeps oracles out of production` | `ifcopenshell is linked by oracle ifcopenshell-ifc-2x3-any-differential but is absent from the dependency baseline` | peer |
| `cross-language oracle hosts … every external host package` | `python:ifcopenshell is on a generated host's import path but is absent from the dependency baseline` | peer |

The peer has registered `ifcopenshell` as an oracle and put it on the host import path but has not
yet added it to `🔒️dependencies.json`; both failures name only that package. A fifth failure,
`🚫️ oracle purity > no production source imports a registered oracle`, appeared in one run and
**passes in isolation** (`2 pass / 0 fail`) — its 67-second repository walk was racing the peer's
writes.

`expect()` calls 2,057 (w14) → 1,974: the contract test's own breach list shrank by two records and
the two ifcopenshell assertions abort their loops early. 69 tests, unchanged. No test was skipped,
weakened or removed.

---

## 3. Measured ratios removed from source

Source says what a case ASSERTS, never what it last scored. Five feature files still carried a dated
parity figure in their prose; the finding each one records was kept word for word and only the number
moved out:

| file | was | now |
|---|---|---|
| `mutate-bmp-v3` | `(parity 14/15, 2026-08-24)` | `(this case's parity ratio is recorded in the ticket, not here)` |
| `mutate-tiff-6-0` | `(was parity 16/17, 2026-08-24; fixed 2026-08-25)` | `(the ratios before and after are recorded in the ticket, not here)` |
| `mutate-gif-89a` | `(parity 41/43, 2026-08-24)` | `(this case's parity ratio is recorded in the ticket, not here)` |
| `mutate-gif-87a` | `(parity 24/25, 2026-08-24)` | `(this case's parity ratio is recorded in the ticket, not here)` |
| `mutate-pdf-1-7` | `it scored parity 24 of 37` | `it scored the ratio recorded in the ticket, not here` |

`grep -rnE "parity [0-9]+/[0-9]+|parity [0-9]+ of [0-9]+" --include=component.feature` now returns
nothing outside `./compose`.

**Two files still carry a measured run line and were left alone deliberately**, because a peer
session is mid-sweep in them (mtime 05:51 today, and its automated substitution has already garbled
the surrounding sentences):

* `mutate-step-ap214/component.feature:50` — `` Both roles passed their own laws (`executed=46 passed=46`) ``
* `mutate-txt-utf-8/component.feature:53` — `` → `executed=24 passed=24` ``

---

## 4. Totals

| | audit (w14) | now |
|---|---|---|
| oracle registrations | 122 | 122 |
| … in-repo second implementations | 42 | 42 |
| **distinct second implementations** | ≈27 (est.) | **28 (measured)** |
| fifteen `📕️norm` adapters, mutual 5-gram Jaccard | 0.672–0.875 (reported) | **0.018–0.081** (was 0.797–0.946 by this method, and byte-identical outside the data block) |
| highest similarity between any two Python adapters | — | **0.820** (`fem2d`/`fem3d`, a genuinely differentiated sibling pair) |
| oracle-backed scenarios | 3,191 | 3,191 |
| … `@mode-differential` | 2,356 | **2,579** |
| … non-differential | 835 | **612** |
| … non-differential outside ifc/step | 663 | **440** |
| scenarios a linking subject would compare | *(read as 2,356)* | **3,191 — mode never gated it** |
| oracle phase, `--owner 📕️norm` | 799 / 795 / 4 | **799 / 795 / 4** |
| contract breaches attributable to this work | — | **0** |
| comparison-profile knobs changed | 0 | **0** |
| scenarios deleted | 0 | **0** |
| assertions removed | 0 | **0** |

**One sentence.** The fifteen `📕️norm` "independent references" were one 800-line file copied
fifteen times and are now one module imported fifteen times with all 799 scenarios proved
bit-identical across the change; and the 835 "non-differential" oracle scenarios were never excluded
from the comparison — `parity` has always compared every scenario both roles produced, 223 of them
were simply mistagged, and the number that actually cannot be measured today is still zero, because
no Rust subject links.
