# Wave 16 — converting no-oracle cases to cross-language differentials

Scope assigned: the recorded `noOracleDecision` cases in `🧱️block`, `🧩️puzzle`, `🌀️procedural`,
`🪐️space`, `🔱️trinity`, `🏗️fem`, `🌍️gis`, `🪵️sourcing`, `🗒️note`, `🖨️raster` — **20 cases, 362
declared mutation kinds**, not the ~18 the brief estimated.

Date 2026-08-25. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Nothing here closes the ticket.

---

## 1. What was converted

Nine cases, each from `@no-oracle-…` to a real second implementation in Python registered as the
oracle, with the `noOracleDecisions` entry deleted from the subset manifest.

| Case | Kinds | Scenarios | Oracle phase | Second producer | Real artifact |
|---|---:|---:|---|---|---|
| `mutate-gismap-1` | 12 | 37 | 37/37 ✅ | Python, from-spec, reads the carrier | committed Liège map, carrier-parsed in BOTH languages |
| `mutate-gisterrain-1` | 2 | 7 | 7/7 ✅ | Python, from-spec | derived once from the committed terrain + gismap examples |
| `mutate-curate-1` | 3 | 10 | 10/10 ✅ | Python, from-spec | derived once from the committed ten-entry timber kit |
| `mutate-raster-1` | 12 | 37 | 37/37 ✅ | Python, from-spec | derived once from the committed demo carrier + a committed group node |
| `mutate-jack-1` | 8 | 25 | 24/25 ⚠️ | Python, from-spec, reads the carrier | committed **Nakagin Capsule Tower**, carrier-parsed in BOTH languages |
| `mutate-rewrite-1` | 7 | 22 | 22/22 ✅ | Python, from-spec | derived once from the committed Nakagin ground-floor rule |
| `mutate-fem2d-1` | 25 | 76 | 76/76 ✅ | Python, from-spec | derived once from the committed twelve-node timber portal frame |
| `mutate-fem3d-1` | 25 | 76 | 76/76 ✅ | Python, from-spec | derived once from the committed sixteen-node two-storey steel frame |
| `mutate-block-2d-1` | 26 | 79 | 79/79 ✅ | Python, from-spec | derived once from the committed *Hexagonal Cut Concrete Forest Left* node kind |

Totals: **120 of 362 kinds (33%) now have a second implementation**; 369 scenarios execute in the
oracle role where 0 did before.

Every one of the nine is a from-spec independent Python implementation, not a third-party library.
Section 4 records what was surveyed and declined per artifact and why — in every case the answer is
that the format is semio-native and the thing a foreign library would adjudicate is not the thing
the vocabulary edits.

## 2. Findings — the point of the exercise

### 2.1 `gismap`: `create-<noun>` silently drops its declared insertion index (OUR CODEC)

The grammar writes `create-position SP number`, the mutation payload carries `index`, and
`CreatePosition { index, item }` holds it. But the sparse delta
(`🧬️schema/🧬️mutations/🆕create-position/🔺️diff/🦀️component.rs`) records only `added: [item]`, and
`apply_features_delta` (`🧬️schema/🔺️diff/📝️text/🦀️component.rs`) applies additions with `push`. So
**every `create-<noun>` appends and the declared index is discarded.**

Knock-on: `delete-<noun>`'s inverse is `create-<noun>` at the captured index, so undoing the deletion
of any NON-TRAILING feature puts it back in the wrong place — the inverse law is violated.

Why the committed vectors could not catch it: all three `create-` vectors insert at index 1 into a
ONE-element collection, where append and insert-at-1 coincide. The real Liège document has two.

`parity exhaustive --case mutate-gismap-1 --implementation rust` → **`parity=30/37`, exit 1**, with
exactly seven red: `mutate-create-{position,route,region}`, `inverse-create-{position,route,region}`
and `inverse-delete-position`. **Left red on purpose.** No parameter was softened, no fixture was
swapped, no assertion relaxed. The fix belongs in `GisMapFeaturesDelta`, which cannot express an
insertion position at all; adding one has to keep the diff-absorb law, which the obvious
`reordered`-based workaround breaks (`d1` and `d2` are both computed against `base`).

### 2.2 `jack`: the committed specification vectors are not self-contained (COMMITTED FIXTURES)

All eight `jack` vectors carry a before-scene of the shape
`{schema, name, manifest, camera, content: <composed child handle>}` — the scene content is inside a
composed child the snapshot does not expand, so **the entities every vector addresses are absent from
the bytes that are committed.**

Seven still replay, because what the outcomes really specify turns out to be that the four in-place
verbs answer an absent target with an accepted `mutation.no-op` while the two `delete-` verbs reject
one. The eighth cannot: `rejects-a-node-id-the-scene-already-holds` commits a scene with NO nodes, so
the id it calls a duplicate is not there. `spec-vector-create-node` is **left red**; the fixture is
what needs fixing.

### 2.3 `jack`: no committed vector exercises an accepting mutation (COVERAGE GAP)

All eight are negative — three rejections, five accepted no-ops. Before this conversion the accepting
direction of the whole eight-kind vocabulary had **no committed evidence at all**. The sixteen
real-document scenarios against the Nakagin tower are the first.

### 2.4 `raster`: the snapshot schema describes a different artifact (SPECIFICATION)

`🖨️raster/…/🧬️schema/📸️snapshot/🔣️component.json` is a verbatim copy of `s.stdio.json`'s
`JsonSnapshot` — `{schema, value}` — carrying the wrong `$id`. The mutation schema's
`RasterLayerNode` `$ref`s it and therefore refs nothing. The document shape was read off the twelve
committed vectors instead, which agree with one another on every field.

### 2.5 A FAMILY of committed schemas describes the wrong thing (SPECIFICATION)

Three separate instances, found by trying to implement from them:

* `🔌️jack/…/🧬️schema/🧬️mutations/🔣️component.json` — the SNAPSHOT schema with `title` changed to
  `JackMutation`.
* `🗺️gismap/…/🧬️schema/🧬️mutations/🔣️component.json` — the same defect, `title: "GisMapMutation"`,
  body = `GisMapSnapshot`.
* `◻2d` (fem2d) `…/🧬️schema/🧬️mutations/🔣️component.json` — the same defect again,
  `title: "Fem2dMutation"`, body = `Fem2dSnapshot`.

None of the three describes a mutation. Whatever generates these files is copying the snapshot
schema when it should be emitting the mutation union, and it has done so at least four times counting
`🖨️raster`'s snapshot schema in §2.4.

A fifth, of a different flavour, found while surveying the remaining work:
`🌀️procedural/🌀️procedural2d`'s snapshot schema declares `Widget` as
`{"type": "string", "contentMediaType": "application/json"}` while every committed vector writes a
widget as an OBJECT (`{"kind": "inputNote", "id": …, "text": …}`). That subset was not converted
partly because of it: the real committed example's widget kinds (`input-slider`, `neuron`,
`output-preview`) have shapes no vector demonstrates and no schema states, so a derived fixture
would have had to guess them.

A related, separate gap: `◻2d`'s SNAPSHOT schema declares its nine collections but every one of
`FemNode`, `FemElement`, `FemRegion`, `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` and
`FemCombination` is an EMPTY `{"title": …, "type": "object"}` with no properties at all. The record
shapes exist only in the committed vectors.

### 2.5b `block2d`: the subject phase never ran the implementation (OUR TEST CODE, pre-existing)

`mutate-block-2d-1/🦀️component.rs` did not link the plugin crate at all. It read the committed
vectors and asserted laws OVER THEM — so the subject phase exercised the committed JSON, never this
subset's own codec. The cause is that the subset ships **no test bridge**: every other converted
subset has one (`gis_map_mutation_report_json`, `fem2d_mutation_report_json`,
`gis_terrain_mutation_report_json`), `🧱️block` has none. `block2d_mutation_report_json` was added to
`◻2d/…/🧬️schema/🧬️mutations/🦀️component.rs` in the same shape, and the adapter now drives it. The
same gap almost certainly holds for `block-3d`, `block-5d` and the three `🧩️puzzle` subsets, which
were not reached.

### 2.5c `block2d`: the carrier codec is unreachable from a test case (OUR CODE)

This subset's `store::ArtifactDsl` impl is handwritten `async` (`📸️snapshot/🦀️component.rs:57`,
`async fn print_dsl`), while the generated test host is synchronous. `parse_dsl`/`print_dsl` are
therefore unreachable from a case adapter without an async bridge. `mutate-block-2d-1` consequently
asserts NO `.dsl.semio` carrier law at all, and says so in its feature rather than shipping a
scenario that claims more than it checks. Compare `🗺️gismap`, whose `ArtifactDsl` is synchronous and
whose identity scenario does hold the committed example to byte exactness.

### 2.6 `curate`: our own parser rejects our own committed example (OUR CODEC, pre-existing)

`subject exhaustive --case mutate-curate-1` at session start, before any edit:
`executed=7 passed=6 failed=1`, the failure being
`identity-round-trip → TextError { message: "expected Text, found Absent", line 1, column 1 }`.
`parse_curate_dsl` cannot read
`🗂️curate/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`. Kept, not routed around; it is why that
case's fixture was derived by reading the carrier in a recorded script rather than by running the
subset's codec.

### 2.7 `gisterrain` and `fem2d`/`fem3d`: the subject phase had never compiled (OUR TEST CODE, pre-existing)

`mutate-gisterrain-1/🦀️component.rs:244` bound `let text = …`, shadowing the `fn text` declared at
:128, so `:248` failed with two `E0618`s. The `mutate-fem2d-1` and `mutate-fem3d-1` adapters carried
the identical shadowing bug in the same handler. All three are fixed here; those three cases' Rust
halves have never run before this session.

### 2.8 Vocabulary limits that a differential ALONE would have reported green

Two vocabularies cannot express the inverse of a non-trailing delete, because their `create-` verb
carries no index:

* `curate` — `create-curated-item "object-id" "=" IDENT "count" "=" INT`
* `jack` — `create-node id text text number number number number port-table`

Both implementations share the limit, so the comparison would agree while the law was violated. It is
caught only because BOTH sides assert the restoring law in role, position for position. Recorded in
both features; the `mutate-` and `inverse-` tables deliberately address different entries and say so.

### 2.9 Naming inconsistencies in committed wire forms

`assembly`'s `ChangeWeight`/`RemoveWeight` use `module_id` while its snapshot uses `moduleId`.
`jack` mixes camelCase discriminators with snake_case arguments (`new_name`, `new_value`).

## 3. Blockers — why eleven cases are not converted

### 3.1 The Rust subject phase does not build for four of the ten plugins

Measured by `subject exhaustive --case <one case per plugin>`:

| Plugin | Result |
|---|---|
| `🪵️sourcing` | builds ✅ |
| `🌍️gis` | builds ✅ (after the `E0618` fix in 2.7) |
| `🔱️trinity` | `semio_framework_job` unlinked, 1289 errors — **fixed here**, see below |
| `🖨️raster` | `semio_framework_job` unlinked, 764 errors — **fixed here** |
| `🗒️note` | 745 errors, `E0053` async-signature mismatches across viewer/editor/schema — another session's async-convention refactor, NOT fixable from a test case |
| `🪐️space` | `semio-framework-os-kernel` fails: 10 errors in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` (`attach_backbone`/`detach_backbone`/`tick`/`dispatch` trait bounds, plus four `future cannot be sent between threads safely`), committed 2026-08-25 14:57 by another session |
| `🌀️procedural`, `🏗️fem`, `🧱️block`, `🧩️puzzle` | not reached before the probe was stopped to free the cargo lock |

The `semio_framework_job` gap was a one-line omission: the generated
`🧬️mutations/💾️binary/🦀️component.rs` yields through `semio_framework_job::StepOutcome`, and
`🌍️gis`'s `📦️packages/🦀️rust/Cargo.toml` links the crate while `🖨️raster`'s and `🔱️trinity`'s did
not. Added to both, mirroring gis exactly. **This is the one place this work stepped outside a test
case**, and it is recorded here rather than buried.

### 3.2 Cargo target-dir lock contention

Several sibling sessions are running the same wave. A `parity` run was killed at the 900 s budget
with `Likely shared cargo target-dir lock contention from another concurrent session`. Plugin cold
builds ran 20–40 minutes each. This, more than anything else, is what bounded how many cases could be
verified end to end.

### 3.2b Two more subsets ship no test bridge, so their subject phase runs nothing (surveyed)

The `🧱️block` gap in §2.5b is not isolated. `🌀️procedural/🌀️procedural2d`'s adapter also does not link
the plugin crate and its subset also exports no `*_report_json`. On the evidence of these two, the
same is likely true of `block-3d`, `block-5d`, the three `🧩️puzzle` subsets and
`🌀️procedural/🧊️procedural3d` — that is 178 of the 268 remaining kinds whose "subject phase" today
reads committed JSON and never runs the implementation. Converting them needs the same four-line
bridge `block2d_mutation_report_json` now shows, added per subset.

### 3.3 `🌀️procedural/🧩️assembly` has no written specification at all

No `🧬️schema/📸️snapshot/🔣️component.json`, no `🧬️schema/🧬️mutations/🔣️component.json`, no grammar,
and **no committed example document of any kind**. The only written specification is the nine
handcrafted vectors. A second implementation is still possible from them, but "a real-world complex
artifact" is not available for this artifact at all.

### 3.4 Specification completeness, surveyed across the remaining subsets

| Subset | snapshot schema | mutation schema | grammar | richest committed example |
|---|---|---|---|---|
| `puzzle-5d` | ✅ | ✅ | 1 | **3.0 MB** `🗣️dream.dsl.semio` |
| `puzzle-3d` | ✅ | ✅ | 1 | 129 KB `🗣️tower.dsl.semio` |
| `puzzle-2d` | ✅ | ✅ | 1 | 94 KB `🗣️tower.dsl.semio` |
| `fem3d` / `fem2d` | ✅ | ✅ | 2 | 2.9 KB / 2.1 KB |
| `block-3d` / `-2d` / `-5d` | ✅ | ✅ | 2 | 1.8 / 1.3 / 1.0 KB |
| `proc-3d` / `proc-2d` | ✅ | ✅ | 2 | 1.6 KB / 0.6 KB |
| `note` | ✅ | ✅ | 1 | 0.5 KB |
| `space` | ❌ | ❌ | 1 | 0.2 KB, and its `artifacts` table is EMPTY |
| `home` | ✅ | ✅ | 1 | 40 bytes — a two-field document; "complex artifact" is not meaningful here |
| `assembly` | ❌ | ❌ | 0 | none |

`🧩️puzzle` is where the real-world-complex bar is genuinely met by committed content, and it is the
highest-value remaining work: 89 kinds across three subsets against a 3 MB real document.

**Remaining, in priority order, with what each needs:**

| Subset | Kinds | Needs |
|---|---:|---|
| `block-5d` | 41 | derivation from its committed carrier + a test bridge (§2.5b); same shape as `block-2d` |
| `block-3d` | 37 | same, plus the `catalog`/`vortexKindExtra` split the old decision names |
| `puzzle-3d` | 35 | derivation from the 129 KB `🗣️tower.dsl.semio` table carrier + a test bridge |
| `note` | 33 | plugin does not compile (§3.1) |
| `puzzle-5d` | 28 | derivation from the 3 MB `🗣️dream.dsl.semio` + a test bridge |
| `puzzle-2d` | 26 | derivation from the 94 KB tower + a test bridge |
| `proc-2d`, `proc-3d` | 14 + 14 | derivation + a test bridge each |
| `assembly` | 9 | no schema, no grammar, no committed example — §3.3 |
| `space`, `home` | 4 + 1 | plugin does not compile (§3.1); `home` is a two-field document |

## 4. Method, for whoever continues

The recipe in `📓️w13-cross-language-recipe.md` holds. Three additions from this wave:

**A. Two carrier families, and how to tell them apart.** Some `.dsl.semio` carriers write every
member as hex of UTF-8 (`gismap`, `jack`, `raster`) and can be read by a from-spec Python
implementation whose reading is then PINNED by byte-exact re-encoding of the committed file — that is
the strongest form, and `gismap` and `jack` use it. Others are structured documents mixing quoted
strings, braced blocks and fenced code (`curate`, `rewrite`, `note`, `procedural`, `puzzle`, `block`)
whose encoding rules cannot be read off one example. **Do not guess those.** Read them ONCE in a
recorded derivation script in the ticket folder, commit the result as a `local://` snapshot fixture,
and leave the carrier's own laws asserted in role on the Rust side.

**B. A derived fixture is an INPUT, never an expectation.** Both implementations read it; neither
side's ANSWER comes from it. Every derivation script here states, file by file, which committed bytes
each value came from, and refuses to run if the committed input is not what it expects.

**C. What to hold back from the cross-language projection, and what not to.** Only values no second
implementation can reproduce: `std::hash::DefaultHasher` digests (`gismap`'s `drawing`/`value`,
`gisterrain`'s `mesh`, `jack`'s `content`). Those stay asserted exactly, IN ROLE, on the Rust side —
`jack` in the sharpest available form, requiring the digest to MOVE on an apply and come BACK on an
undo. Nothing else was held back: no comparison profile was changed, no `ignoreKeys` added, no
committed fixture edited or removed.

**D. Keep the old evidence.** Every converted case keeps a `spec-vector-<kind>` family replaying the
committed handcrafted triples through BOTH implementations, plus the Rust-side committed `🔺️diff`
and `🎯️outcome` checks where the case already made them.

## 5. Verification

Verbatim log: `w16-cross-language/🧪️w16-verification.txt`.

```
oracle exhaustive --case mutate-gismap-1        executed=37 passed=37 failed=0 errored=0   exit 0
oracle exhaustive --case mutate-gisterrain-1    executed=7  passed=7                       exit 0
oracle exhaustive --case mutate-curate-1        executed=10 passed=10                      exit 0
oracle exhaustive --case mutate-raster-1        executed=37 passed=37                      exit 0
oracle exhaustive --case mutate-jack-1          executed=25 passed=24 failed=1             exit 1  (§2.2)
oracle exhaustive --case mutate-rewrite-1       executed=22 passed=22                      exit 0
oracle exhaustive --case mutate-fem2d-1         executed=76 passed=76                      exit 0
oracle exhaustive --case mutate-fem3d-1         executed=76 passed=76                      exit 0
oracle exhaustive --case mutate-block-2d-1      executed=79 passed=79                      exit 0

parity exhaustive --case mutate-gismap-1 --implementation rust
                                                executed=74 passed=73 failed=1 parity=30/37 exit 1  (§2.1)

contract --case <each of the nine>              2 breaches, both testing/discovery, owned by
                                                🧰️framework and ✏️s (other plugins' .test.ts files),
                                                present before this work. ZERO breaches name any of
                                                the nine cases; testing/contract, testing/oracle,
                                                testing/fixture and testing/taxonomy are all at zero.

dependency                                      ecosystems=4 entries=232 production-reachable=151
                                                test-oracle=30 — UNCHANGED. The nine oracle entries
                                                declare package "" and contribute nothing.
```

**Parity beyond `mutate-gismap-1` could not be measured in this session, and the reason is
environmental, not a property of the cases.** Every `parity` run needs a full plugin build, and
several sibling sessions are running the same wave against the same cargo target directory. Two
`mutate-curate-1` attempts were killed by the runner's own 900 s budget with

```
[budget] cargo run … exceeded 900000ms — killed. Likely shared cargo target-dir lock contention
from another concurrent session — investigate before retrying.
```

Two `mutate-curate-1` attempts and one `mutate-gisterrain-1` attempt were killed this way — three
timeouts, ~45 minutes of wall clock, no result. A fourth run, a re-verification of
`mutate-gismap-1`, was killed at 7 minutes with SIGTERM (`exit=143`) by something outside this
session. `mutate-gismap-1` got through earlier, when
contention was lower; its `parity=30/37` is real recorded output and is the finding in §2.1. A queue
for the remaining eight is left running; each is a single command:

```
bun ./📜️script.ts parity exhaustive --case <case> --implementation rust
```

`--implementation rust` is required: without it the framework runs the oracle-only Python adapter in
the SUBJECT role too and reports one `errored` per scenario. Pre-existing, documented as trap 1 in
`📓️w13-cross-language-recipe.md`, not fixed here.

Three plugins additionally cannot produce a Rust subject at all right now — see §3.1: `🗒️note`
(745 async-signature errors from another session's refactor) and `🪐️space` (`semio-framework-os-kernel`,
10 errors in `🏪️store/🔄️sync`). Neither is in the nine.

## 6. Files

Converted cases (`component.feature` + `🐍️component.py` + `🦀️component.rs` + the subset's
`🧪️oracle/🔣️component.json`):

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/mutate-gismap-1/` (+ `🧫️fixtures/🗺️liege-with-derived-regions.dsl.semio`)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🧪️tests/mutate-gisterrain-1/` (+ `🧫️fixtures/🏔️liege-terrain.snapshot.json`)
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🧪️tests/mutate-curate-1/` (+ `🧫️fixtures/🗂️timber-kit.snapshot.json`)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🧪️tests/mutate-raster-1/` (+ `🧫️fixtures/🖨️semio-demo-board.snapshot.json`)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🧪️tests/mutate-jack-1/`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🧪️tests/mutate-rewrite-1/` (+ `🧫️fixtures/♻️nakagin-ground-floor.snapshot.json`)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🧪️tests/mutate-fem2d-1/` (+ `🧫️fixtures/🏗️timber-portal-frame.snapshot.json`)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/mutate-fem3d-1/` (+ `🧫️fixtures/🧊️steel-frame.snapshot.json`)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🧪️tests/mutate-block-2d-1/` (+ `🧫️fixtures/🧱️hexagonal-cut-concrete-forest-left.snapshot.json`)

Production code touched, and only where a subset could not be tested at all without it (§2.5b, §3.1):

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/…/🧬️schema/🧬️mutations/🦀️component.rs` — added
  `block2d_mutation_report_json`, the test bridge every other converted subset already ships.
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml`,
  `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` — added the `semio-framework-job` workspace
  dependency their generated `🧬️mutations/💾️binary` decoders already reference, mirroring `🌍️gis`.
- `✏️s/🔌️plugins/🖨️raster/…/🧬️schema/🧬️mutations/🦀️component.rs` — one stale docstring line that still
  named the deleted no-oracle decision.

Derivation scripts and logs, ticket folder — `w16-cross-language/`:

- `🐍️derive-gismap-regions.py`, `🐍️derive-gisterrain-imports.py`, `🐍️derive-curate-selection.py`,
  `🐍️derive-raster-board.py`, `🐍️derive-rewrite-rule.py`, `🐍️derive-fem2d-frame.py`,
  `🐍️derive-fem3d-frame.py`, `🐍️derive-block2d-kind.py`, `🧪️w16-verification.txt`

Not touched: the framework, the taxonomy, any shared manifest, any comparison profile, any committed
specification vector, `🔒️dependencies.json`, `project.json`, `launch.json`.
