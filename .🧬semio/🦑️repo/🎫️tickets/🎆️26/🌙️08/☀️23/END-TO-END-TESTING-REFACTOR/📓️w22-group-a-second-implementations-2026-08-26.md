# Wave 22 — the 34 Group-A refusals, re-examined one by one

> **The task, as set:** *34 of the 43 remaining `@no-oracle-` cases (1,633 scenarios, 95%) rest on an
> argument their own siblings falsified 41 times in the same wave. For each of the 34: re-examine the
> refusal against what a sibling already achieved over the same carrier. Where a sibling's approach
> transfers, write the second implementation. Where the refusal genuinely holds, rewrite it to argue
> from the specific clause or missing artifact that blocks it.*

Date 2026-08-26. HEAD `8d9b51f081f42b36722b54f80a5c502d6322f9ca` (2026-08-25 15:02:47 +0200), **dirty
tree** — every change this note records is uncommitted working-tree work. Successor to
`📓️w14-final-audit.md`, whose §2.1 Group A and §9 remedy #1 are what it answers.

Raw logs and the authoring aids that produced this: `w22-group-a/`.

---

## 0. The headline, and the thing a reader would otherwise get wrong

**Fifteen of the 34 got a real second implementation. Eighteen got a rewritten refusal that argues
from what actually blocks them. One was left alone because it was already argued.** In scenarios:

| | before | after |
|---|---|---|
| `@no-oracle-` cases repo-wide | 43 | **28** |
| `@no-oracle-` scenarios repo-wide | 1,719 | **578** |
| `@oracle-` cases repo-wide | 121 | **138** |
| `@oracle-` scenarios repo-wide | 3,191 | **4,355** |
| … of which `@mode-differential` | 2,356 | **3,728** |
| registered oracle entries | 122 | **139** |

**The thing not to get wrong: this is still reference-side coverage, not `parity`. `parity` was NOT
measured, and this pass did not even manage to re-verify why.** The single-case probe
`bun ./📜️script.ts parity exhaustive --owner 🗒️note --case mutate-note-1` ran for the runner's full
900 s per-case budget WITHOUT FINISHING THE COMPILE of the generated subject host, was killed by the
runner's own budget check — whose message names the cause, *"Likely shared cargo target-dir lock
contention from another concurrent session"*, and a concurrent session's own
`parity exhaustive --owner 🗄️stdio --case differential-ifc-4` was indeed running throughout — and then
threw `spawnSync cargo ETIMEDOUT` out of `runProbe` with no `[test] level=…` line at all. So the probe
answered nothing. `📓️w14-final-audit.md` §5.3 measured the underlying blocker one day earlier
(`unresolved import component::component_persistent_local` in `semio-framework-plugin`, in every
generated host's graph); **this note does not repeat that as current fact, because it was not
re-measured here.**

What WAS measured about the crates, and it is not one state: `semio-s-plugin-block` reports **1,522**
errors of the form `expected X, found future` (a peer session's in-flight async refactor), while
`semio-s-plugin-puzzle` and `semio-s-plugin-cad` both compile clean at `cargo check --lib`, exit 0,
zero errors. What every number in this note rests on instead
is the ORACLE phase run through the real runner, plus an offline replay of each reference against its
own case plan (`w22-group-a/🐍️replay.py`, which loads the repository's real Python host so the
adapter sees the same `Adapter`/`Context`/`Outcome` it will see under `oracle exhaustive`). **Parity
could not be measured. That is stated here once and not softened anywhere below.**

Three further things a reader should not infer:

1. **Ten of the fifteen conversions do not yet compare our codec against anything.** For
   `block-3d`, `block-5d`, `puzzle-{2d,3d,5d}`, `cad`, `lowpoly`, `procedural-{2d,3d}` and `assembly`
   the SUBJECT adapter links no plugin crate at all: it replays the committed vectors and projects
   them. So what those comparisons establish today is that an independent implementation of the
   specification computes the committed after-snapshots — a real check of the vectors, and the class
   of check that found `mutate-jack-1`'s wrong vector — but not our codec against a second producer.
   Each of those ten says so in its own registration AND in its feature, and each names the missing
   bridge (`block3d_mutation_report_json`, `puzzle2d_mutation_report_json`, …) that closes it. They
   are also exactly the ten cases the w14 audit §4.1 listed as reading **no real-world artifact at
   all** — the same ten, which is itself a finding: the cases with no artifact and the cases with no
   subject are one set.
2. **`≈27 distinct implementations` was the honest count before; it is now ≈39, not 42+15.**
   `procedural-2d`/`procedural-3d` are ONE implementation instantiated twice, and so are
   `playbook`/`forms`. Both pairs say so in their own docstrings, and in both cases the *pair* is
   what produced the finding.
3. **Twenty-six scenarios across the fifteen conversions are RED, deliberately** — 26 of 1,141, or
   2.3%. Every one is a refusal argued by clause inside the reference, in the shape
   `mutate-iso16757-1` established in the previous wave; the runner reports them as `failed` with the
   clause as the message, and `errored=0` everywhere. They are listed in §3 and they are the most
   valuable output here.

### 0.1 The repo-wide `oracle exhaustive` command still cannot produce a headline number here

Attempted twice, and separately a single-case `parity` probe was attempted once — all three died the
same way. The first oracle attempt died at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/…/mutate-avi-1-0`'s RUST oracle host with
`error: spawnSync cargo ETIMEDOUT`, thrown out of `runProbe`
(`📚️library/📦️packages/🟦️typescript/📦️index.ts:1699`) through `executeOne`
(`🧪️test/📜️script.ts:529`) with nothing catching it — **no `[test] level=…` line at all, and every
case measured before it discarded**. That is w13 remedy #10 / w16 §1.3 / w14 §9 remedy #6, still
open, and this is the first time it has been recorded biting the ORACLE phase rather than parity: the
900 s per-case budget is applied to `cargo run`, which includes compiling that case's host from
scratch. The second attempt was killed by this machine before it wrote anything. The `parity` probe
(§0) hit the same wall on `mutate-note-1`'s subject host. **Three runs, three ETIMEDOUTs, no summary
line from any of them — the defect is not intermittent, it is what this machine does whenever two
sessions build Rust at once.**

**So the numbers in §1 are per-owner runs of the same command**, scoped so the answer cannot be lost —
the same approach `📓️w14-final-audit.md` §5.3 fell back to. `bun ./📜️script.ts contract` was run
whole and is exit 1 with exactly the **2** `testing/discovery:unmanaged-tests` breaches the w14 audit
recorded, both other sessions' files; **zero** breaches in the `testing/oracle` domain.

---

## 1. What was converted, and what each second producer is

Fifteen cases, **1,141 scenarios**, each with a Python second implementation in the case directory,
each registered as a `python` / `package: ""` / `testOnly: true` oracle, each replacing the
`@no-oracle-` tag with `@oracle-`, each retagging its mutation rows `@mode-differential`.

| case | scenarios | reference | offline replay | refused, by clause |
|---|---:|---|---|---:|
| `mutate-program-1` | 533 | `architect-program-python-independent` | 528 / 533 | 5 |
| `mutate-block-5d-1` | 83 | `block-5d-python-independent` | 83 / 83 | 0 |
| `mutate-block-3d-1` | 75 | `block-3d-python-independent` | 69 / 75 | 6 |
| `mutate-puzzle-3d-1` | 71 | `puzzle-3d-python-independent` | 68 / 71 | 3 |
| `mutate-note-1` | 67 | `note-python-independent` | 64 / 67 | 3 |
| `mutate-puzzle-5d-1` | 57 | `puzzle-5d-python-independent` | 57 / 57 | 0 |
| `mutate-puzzle-2d-1` | 53 | `puzzle-2d-python-independent` | 50 / 53 | 3 |
| `mutate-cad-1` | 41 | `cad-python-independent` | 41 / 41 | 0 |
| `mutate-lowpoly-1` | 35 | `lowpoly-python-independent` | 35 / 35 | 0 |
| `mutate-procedural-2d-1` | 29 | `procedural-2d-python-independent` | 29 / 29 | 0 |
| `mutate-procedural-3d-1` | 29 | `procedural-3d-python-independent` | 29 / 29 | 0 |
| `mutate-forms-1` | 21 | `forms-python-independent` | 19 / 21 | 2 |
| `mutate-assembly-1` | 19 | `assembly-python-independent` | 19 / 19 | 0 |
| `mutate-playbook-1` | 19 | `playbook-python-independent` | 18 / 19 | 1 |
| `mutate-writer-1` | 9 | `writer-python-independent` | 6 / 9 | 3 |
| **total** | **1,141** | | **1,115 / 1,141** | **26** |

The `offline replay` column is `w22-group-a/🐍️replay.py`, run before any registration was written; the
runner's own per-owner numbers below agree with it scenario for scenario.

**Every one of the fifteen was then confirmed through the real runner, not only the offline replay.**
`bun ./📜️script.ts oracle exhaustive --owner <owner>`, run for all ten owners the conversions touch,
verbatim from each run's own `[test]` line (raw log: `w22-group-a/🧪️oracle-converted-owners.txt`):

```
🏛️architect  cases=1 executed=533 passed=528 failed=5 errored=0 parity=0/0
🧱️block      cases=3 executed=237 passed=231 failed=6 errored=0 parity=0/0
🧩️puzzle     cases=3 executed=181 passed=175 failed=6 errored=0 parity=0/0
📐️cad        cases=1 executed=41  passed=41  failed=0 errored=0 parity=0/0
💠️lowpoly    cases=1 executed=35  passed=35  failed=0 errored=0 parity=0/0
🌀️procedural cases=3 executed=77  passed=77  failed=0 errored=0 parity=0/0
🗒️note       cases=1 executed=67  passed=64  failed=3 errored=0 parity=0/0
✒️writer     cases=1 executed=9   passed=6   failed=3 errored=0 parity=0/0
📖️playbook   cases=1 executed=19  passed=18  failed=1 errored=0 parity=0/0
📋️forms      cases=1 executed=21  passed=19  failed=2 errored=0 parity=0/0
```

**1,220 executed, 1,194 passed, 26 failed, 0 errored.** The 1,220 is the fifteen conversions' 1,141
plus the 79 of `mutate-block-2d-1`, which shares the `🧱️block` owner and was converted in the
previous wave. The 26 failures are exactly the 26 clause-grounded refusals of the offline replay,
case for case — no run produced a failure the replay did not predict, and none produced an `errored`.
Note the `parity=0/0` on every line: the oracle phase forms no comparison by construction, and no
Rust subject host links, so it stays zero. **That is the honest ceiling on all of this.**

**None of the fifteen imports anything from this repository.** No `subprocess`, no FFI, no shell-out,
no Rust identifier smuggled in; the verbs were written from each subset's committed
`📸️snapshot/🔣️component.json`, from the rules of `📓️derivation-rules.md`, and from that subset's own
committed `(before, mutation, after, outcome)` vectors, and were then checked by replaying every
vector before the oracle was registered.

### 1.1 Why each refusal fell, named against the sibling that falsified it

The brief's charge was that these 34 all leaned on one sentence. They did, and here is the sentence
each was replaced with — the sibling whose approach transfers, named case by case:

* `mutate-block-3d-1`, `mutate-block-5d-1` — **`mutate-block-2d-1`**, the same plugin, the same
  carrier, the same kind-definition document shape. The 3d decision claimed its split vortex-kind
  vocabulary "IS this subset's specification"; the 5d decision claimed the same of its
  `2d`/`3d` → `part2d`/`part3d` diff renaming. A split is something a second implementation MODELS
  and a renaming is something it resolves through a declared alias table; both do.
* `mutate-puzzle-{2d,3d,5d}-1` — **`mutate-fem2d-1`, `mutate-fem3d-1`, `mutate-gismap-1`**. Two-level
  connectivity (ports owned by nodes) is not an obstacle; the references model both cascades.
* `mutate-cad-1` — **`mutate-gismap-1`**. This decision was HALF right and the half that was right is
  kept: registering a general-purpose CAD or JSON-patch crate really would be worse than no oracle.
  It was wrong that the only alternative is none.
* `mutate-lowpoly-1`, `mutate-assembly-1`, `mutate-procedural-{2d,3d}-1` — **`mutate-fem3d-1`,
  `mutate-gisterrain-1`**.
* `mutate-note-1`, `mutate-writer-1`, `mutate-playbook-1`, `mutate-forms-1` — **`mutate-semio-drawing`,
  `mutate-semio-mesh`, the fifteen `📕️norm` and nineteen `🧿️semio` references**.
* `mutate-program-1` — **the fifteen `📕️norm` references**, which were written from
  `📓️derivation-rules.md` and `📓️taxonomy.md`, the exact two documents this case's rationale called
  un-adjudicable. Its 266 kinds were re-derived from `📸️snapshot/🔣️component.json`'s register list
  and rules 1, 2 and 4, and the resulting engine reproduces **264 of the 266** committed vectors
  exactly, on both the forward and the inverse side, before a single scenario was registered.

---

## 2. What was refused, and the clause each refusal now cites

Eighteen cases keep a `@no-oracle-` decision. **Not one of them keeps the falsified sentence.** Each
rationale and each feature's opening now says the same three things in the same order: what the
third-party survey found (kept where it was good), that DECLINING A LIBRARY IS NOT DECLINING A SECOND
IMPLEMENTATION and only the first was ever judged here, and what blocks a second implementation
*today*, concretely.

**One blocker accounts for seventeen of the eighteen, and it is one edit per case.** Their committed
specification vectors are **not declared as `asset://` fixtures**: the `Examples` table carries the
payloads inline and the Rust adapter reaches the committed files through `include_str!`. Two things
follow, and both are defects in their own right — the execution plan pins none of those files'
digests, so a silently edited vector changes the RESULT rather than the PLAN; and a Python reference
cannot read them at all, because the Python host resolves a fixture only through the plan and raises
on an undeclared URI (`🐍️host.py`, `Context.fixture`). `mutate-program-1`, `mutate-note-1`,
`mutate-block-3d-1` and every case converted here declare them; these seventeen do not.

| case | scenarios | nearest transferable recipe | second blocker |
|---|---:|---|---|
| `mutate-remodel-1` | 71 | `mutate-cad-1` | carrier grammar is the `payload = OCTET+` placeholder |
| `mutate-shooting-1` | 63 | `mutate-puzzle-3d-1` | same placeholder |
| `mutate-layout-1` | 51 | `mutate-note-1` | carrier grammar is the generic `family-scene` canvas grammar |
| `mutate-process3d-1` | 33 | `mutate-assembly-1` | same placeholder |
| `mutate-mathematical-1` | 31 | `mutate-puzzle-2d-1` | same placeholder |
| `mutate-dag-1` | 29 | `mutate-puzzle-2d-1` | same placeholder |
| `mutate-draw-1` | 29 | `mutate-note-1` | canvas grammar |
| `mutate-txt-utf-8` | 24 | — (see §2.1) | the oracle role is already occupied |
| `mutate-flow-1` | 21 | **`mutate-procedural-2d-1`, the same document** | same placeholder |
| `mutate-wires-1` | 21 | `mutate-puzzle-2d-1` | same placeholder |
| `mutate-present-1` | 19 | `mutate-note-1` | same placeholder |
| `mutate-sequence-1` | 17 | `mutate-puzzle-2d-1` | same placeholder |
| `mutate-vcs-1` | 13 | `mutate-writer-1` | same placeholder |
| `mutate-imperative-1` | 9 | `mutate-playbook-1` | same placeholder |
| `mutate-s-space-1` | 9 | `mutate-cad-1` | — |
| `mutate-playground-1` | 3 | `mutate-writer-1` | same placeholder |
| `mutate-energy-model-1` | 3 | `mutate-cad-1` | same placeholder |
| `mutate-s-home-1` | 3 | `mutate-writer-1` | same placeholder |

`mutate-flow-1` deserves its own line: its document is not merely *like* `mutate-procedural-2d-1`'s,
it IS the same document. The procedural subsets' `fixture` half is literally a `flow.fixture`
widget/synapse graph with a camera and a sparse layout map, and the Python reference written for them
in this wave already models the widget discriminant, the port pair and the fact that `delete-widget`
does NOT cascade. Twenty-one scenarios sit behind one fixture declaration.

`mutate-semio-any` (43) was left untouched: it is the one decision in the repository that already
carried a genuine re-survey with two nameable blockers, and this pass found nothing to correct in it.

### 2.1 `mutate-txt-utf-8` is a different refusal and now says so

The w14 audit §2.2 named it and `mutate-binary-raw` together: both declare, as their substitute, *"an
independently hand-written reference implementation … in this subset's own oracle module"*. That
description is accurate — `oracle_apply_mutation`, `independent_split`, `independent_render` in
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle` never call the production `TxtSnapshot`/`TxtMutation` code — and it
is our code, in our language, in our crate, by our authors. Its decision now records that as a debt,
and names the two edits that would close it. First, its oracle role is already occupied, and occupied
differently from every other case in the group: this is the one whose reference half is a genuinely
separate hand-written implementation living in ANOTHER OWNER'S CRATE, so converting means MOVING the
reference role out of `🗄️stdio/🧪️oracle` rather than adding one beside it. Second, the compared
projection's `schema` member is the Rust constant `crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA`,
whose VALUE is stated in no committed document, so a second implementation cannot emit the projection
without being told it.

**And a finding this comparison turned up on the way.** THIRTEEN of the eighteen remaining Group-A
cases register Rust `.oracle(...)` handlers — `mutate-remodel-1`, `mutate-layout-1`,
`mutate-process3d-1`, `mutate-mathematical-1`, `mutate-dag-1`, `mutate-draw-1`, `mutate-wires-1`,
`mutate-vcs-1`, `mutate-imperative-1`, `mutate-s-space-1`, `mutate-playground-1`,
`mutate-energy-model-1`, `mutate-s-home-1` — and a recorded no-oracle case dispatches NO oracle role,
so every one of those registrations is dead code. Worse than dead: each handler reads the committed
vector through `include_str!` and hands it straight back, so if the role were ever dispatched the
comparison would be a fixture against itself. `mutate-program-1`'s own adapter doc comment warned
about exactly this shape — *"Registering an oracle handler here would be dead code that reads as
coverage in every listing"* — and thirteen cases do it anyway. The five that do not are
`mutate-shooting-1`, `mutate-flow-1`, `mutate-present-1`, `mutate-sequence-1` and `mutate-txt-utf-8`
(whose four are real and live in another crate). Everything else
about the case is unusually good: it reads a REAL 27,471-byte German interview transcript with 80 real
blank lines, and its nine literal `@id-spec-vector` byte vectors pin the split-and-render contract in
any language.

`mutate-binary-raw` was left alone, as instructed.

---

## 3. The twenty-six red scenarios, and the findings behind them

Every conversion's refusals are raised in role with the clause that blocks them, so they arrive as
`failed` with a message rather than as silence. **They are new evidence, not regressions**: before
this wave none of these scenarios had a second producer to disagree with.

### 3.1 One unstated function accounts for fourteen of the twenty-six

**A composed child slot's `childId` is a CONTENT ADDRESS of the child document, and no document in
this repository states the addressing function or the child's canonical encoding.** The w14 audit
found this once, on `mutate-en1990-1`, and recommended publishing the rule (§9 remedy #3). Writing
fifteen second implementations found it four more times:

| case | kinds refused | the handle |
|---|---|---|
| `mutate-program-1` | `create-knowledge-record`, `create-benchmark-record` (×2 roles) | `knowledge`, `benchmarks` → `s.stdio.semio@v1/table` |
| `mutate-block-3d-1` | `create-vortex-kind`, `delete-vortex-kind`, `rename-vortex-kind` (×2 roles) | `catalog` → `s.stdio.semio@v1/kit` |
| `mutate-note-1` | `edit-block-text` (×2 roles) | a text block's `content` → `s.stdio.semio@v1/text` |
| `mutate-writer-1` | `edit-text` (×2 roles) | `document` → `s.stdio.semio@v1/document` |
| `mutate-en1990-1` (w14) | `insert-variable-action` (×2 roles) | a composed child slot |

In every one of them the verb's WHOLE observable effect is that address changing — `catalog-a602…`
→ `catalog-69f2…`, `note-text-eea4…` → `note-text-9382…`. Every OTHER verb over the same record is
implemented and green. **One published rule closes sixteen red scenarios across five cases** — `mutate-program-1`'s two
kinds, `mutate-block-3d-1`'s three, `mutate-note-1`'s one, `mutate-writer-1`'s one and
`mutate-en1990-1`'s one, each counted in both the `mutate-` and the `inverse-` role: 4 + 6 + 2 + 2 + 2.
Fourteen of those sixteen are inside the fifteen cases converted here; `mutate-en1990-1`'s two came
from the previous wave and are still red. **No comparison profile moves.**

### 3.2 Seven more are the vocabulary being under-determined by its own vectors

* `mutate-puzzle-2d-1 :: replace-node-handle` and `mutate-puzzle-3d-1 :: replace-object-vortex`
  (×2 roles each). Each has exactly ONE committed vector; each vector supplies a genuinely different
  record (`handle-1` moves from `handle-kind-a` to `handle-kind-c`) and each committed outcome
  declares `mutation.no-op` with an unchanged after-snapshot. Three different rules produce exactly
  that and nothing distinguishes them: the verb is unimplemented, or it refuses a port an edge is
  attached to, or it refuses a kind the compatibility relation does not admit.
  **Their 5d sibling settles two thirds of it**: `mutate-puzzle-5d-1`'s `replace-part-grip` vector
  really does rekind `grip-1`, on a grip a fastener IS attached to, and the document moves. So the
  verb is implemented and attachment does not block it. One more vector, on a port whose kind the
  relation admits, decides the rest.
* `mutate-puzzle-2d-1 :: inverse-replace-kind-catalogs` and `mutate-puzzle-3d-1 ::` the same. The
  vector INSTALLS a catalogue where the before-snapshot had none, so undoing it means REMOVING the
  member — and the 5d sibling's `null-catalogs-is-noop` vector proves a null argument is accepted and
  does NOTHING. The gap is in the VOCABULARY, and it is invisible to the subject half, which asserts
  only a footprint and never applies an inverse.
* `mutate-forms-1 :: inverse-change-form-title`. Same shape: the vector ADDS `title` to a snapshot
  that carried none, and nothing says whether the verb takes a null. Its `📖️playbook` sibling has no
  such gap, because there `title` is always present and nullable.

### 3.3 The remaining five are the carrier — and the carrier is undocumented nearly everywhere

`identity-round-trip` is refused by clause in `mutate-program-1`, `mutate-note-1`, `mutate-writer-1`,
`mutate-playbook-1` and `mutate-forms-1`. **14 + 7 + 5 = 26**, which is the whole of §1's
failure column and the whole of the ten owner runs' `failed` totals. Counted across the whole semio-native surface this pass
touched:

* **The repository-wide PLACEHOLDER** — a grammar whose whole body is `payload = OCTET+` and whose
  header production declares `"schema" SP "stdio.json"` — is committed for the snapshot text carrier of
  **41 distinct semio-native artifacts** across 46 grammar files. Measured, not estimated, over every
  `🚪️io/📸️snapshot/📝️text/` and `🧬️schema/📸️snapshot/📝️text/` grammar in `✏️s/🔌️plugins`: it covers
  `🏛️program`, `✒️writer`, `🌿️vcs`, `🔌️wires`, `📸️remodel`, `🎥️shooting`, `🧊️process3d`,
  `➗️mathematical`, `🕸️dag`, `🌊️flow`, `🎬️present`, `🎬️sequence`, `📜️imperative`, `🎪️playground`,
  `🔋️model`, `🏠️home`, `🔌️jack`, `🗂️curate`, `♻️rewrite`, `🗺️gismap`, `🏔️gisterrain`, all fifteen
  `📕️norm` subsets, and — a detail worth naming — `◻2d`, `🧊️3d`, `🖐️5d`, `🌀️procedural2d` and
  `🧊️procedural3d`, five of the cases converted in this pass, whose `identity-round-trip` never touches
  the carrier and so never met the gap. Every one of those artifacts contradicts that header on its own
  first line (`semio vcs.vcs.dsl v1`, `semio architect.program.dsl v1`, …).
* **A generic `family-scene` CANVAS grammar** — `doc-body = schema-line layers-block`,
  `layer = shape-layer | path-layer | text-layer` — is committed by `📖️playbook`, `📋️forms`,
  `📏️layout`, `🖍️draw` and `🖨️raster`, over five documents that carry no `layers` block at all. The
  five copies differ from each other only in their `grammar`, `extension` and `artifact-mark` lines;
  `diff` between the `📖️playbook` and `📋️forms` copies is three lines. `📖️playbook`'s artifact is five
  hex-encoded scalars and two `[hex,hex]` child-handle pairs; `📋️forms`'s is a nested
  `steps=[ … blocks=[ … ] … ]` tree.
* **Only 39 artifacts in `✏️s/🔌️plugins` commit a grammar that is neither** — and 34 of those 39 are
  third-party formats (`📄️pdf`, `🎞️gif`, `🏗️ifc`, `📐️step`, `🎒️zip`, …) where the grammar was written
  from someone else's specification. The semio-native surface is where the carrier is undocumented.
* **`🗒️note` commits a REAL grammar, and it is incomplete** — which is what makes its gap citable
  rather than vague. `block = text-block | image-block | shape-block` covers three of the SIX block
  kinds the vocabulary declares (stroke, table, math and group have no production at all); its
  `block-field` list names `paragraphs` and `asset-id` while the artifact writes neither and writes
  `content=child_id=… target="…"`, a flattened nested record nothing bounds; and its
  `artifact-mark = "note.note"` is contradicted by `semio note.note.dsl v1`.

`mutate-program-1`'s refusal adds the sharpest single clause in the set: **all seventy record `$defs`
in its committed `📸️snapshot/🔣️component.json` are `{"type": "object", "additionalProperties": true}`
with no `properties` at all**, and 133 of its mutation payload objects likewise — so even the schema
cannot bound the flattened nested records (`ownership=consultant-ids=[ ] participant-ids=[ ] tags=[ ]
notes=[ ]`) the carrier writes. That same row header writes `tags` and `notes`, which **no committed
snapshot vector carries on any record of any register**.

---

## 4. Findings a second implementation produced that no single-implementation case could

These are the reason the exercise is worth doing, and none of them is a red scenario — every one is a
green row that means something different than it used to.

1. **Three subsets' `🧬️mutations/🔣️component.json` is not a mutation schema at all.**
   `mutate-puzzle-{2d,3d,5d}-1`'s are titled `Puzzle2dMutation`/`Puzzle3dMutation`/`Puzzle5dMutation`
   and declare the SNAPSHOT's members — the pre-migration whole-snapshot-shaped generic schema that
   `s.architect.program`'s own mutation schema records itself as superseding. They were never
   replaced, so the verbs and their argument lists had to be read off the committed payloads.
2. **`mutate-note-1`'s `duplicate-blocks` places its second copy BEFORE its own source.** The
   insertion index is computed against the pre-mutation list and never re-based as earlier copies
   land: duplicating `blk-ink` (root index 1) and `blk-table` (root index 2) in one mutation yields
   `blk-text, blk-ink, blk-ink-copy, blk-table-COPY, blk-table, …` in the committed after-snapshot.
   The singular `duplicate-block` places its copy after the source. Both implementations reproduce the
   committed order; naming it is what stops it passing as intent.
3. **One diagnostic code means opposite things inside one plugin.** `mutation.cascade` at level `info`
   announces that `delete-slot` really DID remove the edges naming it (`🧩️assembly`) and that
   `delete-widget` LEFT a dangling synapse standing (`🌀️procedural2d`).
4. **`s.forms.form` and `s.playbook.playbook` are the same document with the same verbs and answer
   two situations differently.** A duplicate step id is a REJECTED `mutation.duplicate-id` in forms
   (`create-step`) and an APPLIED `mutation.no-op` in playbook (`add-step`). A block added to a step
   that does not exist is `mutation.invariant` in forms (`create-block`) and
   `mutation.target-missing` in playbook (`add-block`). Neither divergence is stated anywhere; both
   are visible only because one reference was written against both surfaces.
5. **`mutate-procedural-2d-1` and `mutate-procedural-3d-1` differ only in names — and disagree on a
   diagnostic.** Same document shape, same fourteen verbs; three kind names and four argument names
   differ, and `delete-widget` raises `mutation.cascade` in 2d and NOTHING in 3d, for an effect that
   is byte-for-byte identical in both committed vectors. The 2d subset also spells one argument
   `question_id`, the only snake_case identifier in either document model.
6. **`mutate-forms-1`, `mutate-playbook-1` and `mutate-writer-1` carry almost no exercised mutation.**
   Nine of forms' ten committed vectors, eight of playbook's nine and one of writer's four leave the
   snapshot BYTE-IDENTICAL, because those kinds address records that live in a composed child. What
   those vectors pin is a DIAGNOSTIC — and the references DERIVE it from the `scene` array in each
   scenario's doc string rather than reading it off the committed outcome, which is the only way the
   comparison says anything. **No committed vector in either case exercises a create/add/move/replace
   that SUCCEEDS.** That is a real gap in those cases' fixtures and is now stated in both of them.
7. **`connect-` verbs in `s.architect.program` NORMALISE the edge they append.** The
   `connects-reception-to-waiting` vector's payload carries `normalized: false` and its after-snapshot
   carries `true`, with nothing else moved. It is the only statement of the rule in the repository.
8. **The `documents` register is serialized `artifacts`.** `s.architect.program`'s committed JSON
   Schema requires `documents`; every committed vector carries `artifacts`, and
   `create-document`'s payload key is `document`.

---

## 5. Was any evidence weakened?

**No.** Checked against `HEAD` (`8d9b51f081`), which is the correct baseline because everything here
is uncommitted working-tree work:

1. **Comparison profiles.** `git diff HEAD -- '*🔣️component.json'` filtered to
   `^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"` returns **nothing**.
2. **Comparison tags.** `git diff HEAD -- '*component.feature'` filtered to `^[+-]@comparison-`
   returns **nothing** — all fifteen conversions kept `@comparison-ordered-json-v1`.
3. **Scenarios.** Over all 106 features changed in the working tree, re-parsed with the repository's
   own `parseFeature` and compared by scenario **id**: **3,772 → 4,118, 346 gained, ZERO LOST**
   (`w22-group-a/cmp.ts`). This pass added none and removed none; the 346 are the previous wave's.
4. **Fixtures.** `git diff --name-status HEAD` over `*🧫️fixtures*` and `*📚️examples*`: **88 A, 69 M,
   0 D, 0 R** — byte-identical to what `📓️w14-final-audit.md` §6.5 measured before this pass began.
   No fixture was added, swapped, normalised or deleted here.
5. **Assertions.** Nothing was removed. Every conversion ADDS assertions the case did not have: the
   full inverse law (apply, then apply the verb's OWN computed inverse, and require the committed
   before-snapshot back, index for index) where the subject half asserted only the weaker footprint
   precondition; a single-member footprint law; and per-case structural laws in the reference's own
   validator.
6. **Migration ratchet.** `./🔒️migration.json` untouched.
7. **Contract.** `bun ./📜️script.ts contract` → the same **2** breaches the w14 audit recorded, both
   `testing/discovery:unmanaged-tests` and both attributable to other sessions' test files. **Zero**
   `testing/oracle` breaches: no `unknown-oracle`, no `oracle-capability-mismatch`, no
   `oracle-profile-mismatch`, no `differential-without-evidence`.
8. **No production code was touched.** No Rust was added or changed except ONE doc comment
   (`mutate-program-1/🦀️component.rs`, which claimed the oracle role is never dispatched for this
   case — now false). No test bridge was added to any plugin crate, deliberately: those crates do not
   compile today and unverifiable Rust would have been worse than none.

---

## 6. What is left, in order of leverage

1. **Declare the specification vectors as `asset://` fixtures in the seventeen cases of §2.** One
   edit each. It pins their digests whether or not a reference follows, and it is the only thing
   standing between `mutate-flow-1` (21 scenarios) and a reference that already exists for the same
   document.
2. **Publish the composed-child addressing rule** (§3.1). Sixteen red scenarios across five cases —
   fourteen of them inside this pass's conversions — and no comparison profile moves.
3. **Publish a real grammar for the `.dsl.semio` snapshot carrier** (§3.3). One `payload = OCTET+`
   placeholder stands in for 41 semio-native artifacts, and five more carry a canvas grammar for a
   document they do not hold. `🗒️note` shows what a real one looks like and needs three more
   productions.
4. **Add the ten missing `<subset>_mutation_report_json` bridges** so the ten vector-only cases of §0.1
   compare a codec rather than a fixture. Not done here because each is PRODUCTION code in a crate
   this test-side pass deliberately does not touch, and because none of them could be verified end to
   end while the framework blocker stands. Two of the four crates involved (`puzzle`, `cad`) compile
   clean today; `block` does not (1,522 errors, another session's refactor).
5. **Add one `replace-<container>-<port>` vector on an unattached port** to `s.puzzle.2d` and
   `s.puzzle.3d`, and one `remove-catalogue` vector or a nullable argument (§3.2).
6. **Give `s.forms.form` and `s.playbook.playbook` one vector each that SUCCEEDS** (§4.6). Between
   them those two cases carry 40 scenarios and exercise one applied mutation.
7. **Reconcile the four sibling divergences of §4.3–4.5** — or record each as intended.
