# 🧮 W5 — fem2d third-party SOLVER oracles

Scope: give `s.fem.fem2d` a real third-party FEM-solver oracle and discharge the
`noOracleDecisions/fem2d-non-geometry-mutation-semantics` debt that covered 22 of the artifact's 25
mutation kinds. Everything below was RUN; nothing Rust was compiled (host swap exhausted, per brief),
so the Rust half is verified by signature grep, not by `cargo`. §8 lists exactly what is unproven.

---

## 1. Dependency state — proven

Root `pyproject.toml` `[dependency-groups] test` already listed `anastruct>=1.6.1`,
`scikit-fem>=10.0.2`, `PyNiteFEA>=1.1.6` (added by the predecessor W5 before the fleet loss). No
change was needed.

```
$ uv sync --group test --group dev
Resolved 150 packages in 497ms
Checked 147 packages in 941ms

$ MPL_IGNORE_SYSTEM_FONTS=1 MPLBACKEND=Agg uv run python -c "import anastruct, skfem, Pynite, scipy, numpy; ..."
anastruct 1.7.0
scikit-fem 12.0.2
PyNiteFEA 3.0.0
scipy 1.18.0
numpy 2.5.0
```

### 1.1 A host blocker, and the fix that is now committed in the oracle

`import anastruct` pulls `matplotlib`, whose `font_manager` runs at import time and **fails outright
on this host**:

```
File ".venv/lib/python3.14/site-packages/matplotlib/font_manager.py", line 275, in _get_macos_fonts
    return [Path(entry["path"]) for entry in d["_items"]]
KeyError: '_items'
```

matplotlib 3.11 parses `system_profiler SPFontsDataType`, whose JSON on this macOS (Darwin 25.6 /
macOS 26) no longer carries `_items`. This is deterministic, not load-related — it reproduces on a
quiet run. matplotlib itself provides the escape hatch: `findSystemFonts` skips the host font
database entirely when `MPL_IGNORE_SYSTEM_FONTS` is set. The committed oracle therefore does, before
its `anastruct` import:

```python
os.environ.setdefault("MPL_IGNORE_SYSTEM_FONTS", "1")
os.environ.setdefault("MPLBACKEND", "Agg")
```

That is not a workaround smuggled into a test — the oracle draws nothing, and refusing the host font
database is the correct declaration for a headless numeric adapter. Without it, **every** anastruct
import on a macOS 26 machine dies. Worth telling W6/W7 if they import anastruct or matplotlib.

`PyNiteFEA` and `scikit-fem` import clean and fast; neither is wired into fem2d (see §7).

---

## 2. What was built

New language-agnostic case
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/`:

| file | lines | role |
|---|---|---|
| `🥒️.feature` | 161 | 35 scenarios, `@capability-fem2d-1-mutate`, `@oracle-anastruct-fem2d-solver`, `@comparison-semantic-fem2d-analysis-v1` |
| `🐍️.py` | 935 | the ORACLE — anastruct + `scipy.linalg` + closed forms |
| `🦀️.rs` | 505 | the SUBJECT — this repository's engine through the artifact's own bridges |
| `🧫️fixtures/` | 9 files | 7 snapshots + the mutation corpus + the committed reference values |

Plus, outside the case:

* `…/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs` — four new public test bridges appended to the
  existing `//#region 🌉️TestBridge` (§6).
* `…/🪆️subsets/🌐️any/🔮️oracle/🔣️.json` — two oracle entries, one comparison profile, 22 discharged
  `oracleRequirements`, one narrowed `noOracleDecisions` (§7).
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — one line:
  `"🧮️solves-fem2d-1-benchmarks"` added to
  `semanticDirectoryMemberKinds/members-of-tests/memberNames` (1136 → 1137 entries), the name-registry
  gate every discovery tool reads.
* `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs` — one expression, a real defect the oracle
  found (§5).

Ticket-folder inputs (kept): `🔨️make-fem2d-benchmarks.py`, `🔨️run-fem2d-oracle.py`,
`🔨️register-fem2d-solver-oracle.py`. Their run outputs live under `🗑️generated/fem2d-oracle/`.

---

## 3. The argument — why a solver CAN judge these 22 kinds

The registry's own words for declining a solver were:

> `code_aster`, `OpenSees`, `anastruct` and `PyNite` compute displacements and forces FROM a model,
> while every one of these twenty-two kinds edits the model DOCUMENT itself and none of them reads a
> solved result.

That is right about what a solver can **read** and wrong about what a solver can **judge**. A solver
cannot say whether `deleteSupport` removed the right record — `📈️mutate-fem2d-1-analysis` and its
four subset siblings already adjudicate that against a second implementation of the algebra. What a
solver can say, with complete independence, is **what the structure does once that record is gone**,
and that is precisely what all 22 kinds exist to change. A `replaceSection` landing on the wrong
slot, a `deleteSupport` removing the wrong support, an `addLoad` attaching to the wrong member: each
produces a different STRUCTURE, and a different structure answers differently under load.

So the case compares no documents at all. It compares ANALYSIS RESULTS, and it reaches every kind by
**mutate-then-solve**: the subject applies the mutation through production dispatch and solves what
it left behind; the reference solves the independently committed post-mutation snapshot from scratch.

---

## 4. Fixture corpus

All flat files in `🧫️fixtures/` (a case directory may hold exactly one `🧫️fixtures` and no nested
directories — hence one corpus file rather than fifty).

| fixture | what it is | why |
|---|---|---|
| `🏗️timber-portal-frame.snapshot.json` | the committed real-world timber portal frame, **verbatim copy** | provenance; judged by `derives-the-timber-frame-substructure` |
| `🪵️timber-frame-members.snapshot.json` | its FRAME SUBSTRUCTURE — identical nodes/members/materials/sections/supports, regions and area loads dropped | the committed document itself **cannot be solved** — see below |
| `📏️steel-cantilever.snapshot.json` | 6 m IPE 300 cantilever, 8 elements, 10 kN tip load | `PL³/(3EI)`; also the modal benchmark |
| `📐️steel-simple-beam.snapshot.json` | 8 m IPE 400, 12 kN/m UDL, 2 elements | `5wL⁴/(384EI)` |
| `🌉️concrete-two-span.snapshot.json` | 2 × 6 m continuous 300×600 RC beam, dead + live + ULS | `0.375wL : 1.25wL : 0.375wL` reaction split |
| `🏢️steel-frame-base.snapshot.json` | 3-bay (6 m) 2-storey (3.5 m) steel moment frame: 15 members, dead/live/wind/spare cases, ULS/SLS/spare combinations, a detached 4-corner-pinned plant-room slab, one spare record per `delete-`/`replace-` verb | the mutate-then-solve base |
| `🏛️steel-columns.snapshot.json` | 4 disjoint 6 m columns, one per effective-length factor, each with its own 1 MN reference case | Euler buckling at `K = 2.0 / 1.0 / 0.6992 / 0.5` |
| `🧬️mutated.snapshots.json` | per kind: the payload, the committed post-mutation snapshot, and the declared effect (`changes` / `invariant`) | 320 KB |
| `📊️expected.results.json` | the committed reference values (§6) | 150 KB |

### 4.1 A finding: the committed real-world timber portal frame is a MECHANISM

`🏗️timber-portal-frame.snapshot.json`'s `slab_spare` region has outline
`(0,0) (4,0) (4,2) (0,2)`. Exactly ONE of those vertices, `(0,0)`, coincides with a frame node (`n2`).
Since `build_nodes_and_elements` reuses a document node at an exact mesh-vertex position, the meshed
slab is attached to the rest of the model at a single point, and a meshed continuum attached at one
point can rotate rigidly about it — a zero-energy mode. `fem2d_solve_all` on the committed document
should therefore return `FemError::Singular`.

That region exists to give `delete-region`/`replace-region` a trailing target, not to be analysed.
The case states this as a **checkable geometric fact** rather than as prose: the
`derives-the-timber-frame-substructure` scenario counts the region's node-coincident outline vertices
and requires exactly one, and both implementations run that count. The mechanism it implies belongs
to the continuum, which a 2D frame package cannot see and therefore cannot adjudicate — so the
scenario asserts the attachment, not the singularity. The singularity itself is asserted, on a model
both sides CAN see, by `refuses-a-mechanism` (the cantilever with its only support deleted).

### 4.2 Why the cantilever is 6 m and not 3 m

At 3 m the first LONGITUDINAL mode sits at 431 Hz, between bending modes 2 (251 Hz) and 3 (703 Hz),
so `βₙL = 7.8548` would be compared against the wrong mode. At 6 m the bending modes are
10.0 / 62.8 / 175.8 Hz and the first axial mode is 215.5 Hz, above all three.

### 4.3 Why the mutation corpus is authored, and how it is trusted

`🔨️make-fem2d-benchmarks.py` carries a small independent Python transcription of the fem2d mutation
algebra. Before it authors a single fixture it **replays every committed `(before, mutation, after)`
specification vector on disk** — globbed, so the hash-truncated case directory names W1 is repairing
do not matter:

```
$ uv run python 🔨️make-fem2d-benchmarks.py
[make] replayed 25 committed specification vectors through this transcription — all agree
[make] wrote …/🧫️fixtures/🏗️timber-portal-frame.snapshot.json (6225 bytes)
… 8 more …
[make] wrote …/🧫️fixtures/🧬️mutated.snapshots.json (319738 bytes)
```

25/25 committed vectors reproduced exactly. Each authored payload is additionally required to MOVE
the base document, so no kind can be committed as a silent no-op.

---

## 5. 🚨 A real engine defect the oracle found, and fixed

`✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs` — `BeamEb2::recover`, station moments:

```rust
- crate::model::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: m1 + v1 * x + wy_local * x * x / 2.0 }
+ crate::model::BeamStation { x, n: -n1, v: v1 + wy_local * x, m: -m1 + v1 * x + wy_local * x * x / 2.0 }
```

`m1 = f_end[2]` is the **moment the node applies to the member**, not the internal moment on the
section; the internal progression is `−m1 + v1·x + w·x²/2`. Worked example, the 6 m tip-loaded
cantilever (`P = 10 kN`): `f_end = [·, +10000, +60000, ·, −10000, 0]`, so the old formula gives
`M(0) = 60 kN·m` and `M(L) = 60 + 10·6 = 120 kN·m` at a **free end**, where the moment must be zero.
The corrected form gives `M(0) = −60`, `M(L) = 0`. ✅

Why nobody noticed: the subset's own `two_span_beam_matches_analytical_midspan_deflection_and_moment`
starts its first element at a **pin**, where `m1 = 0` and the two formulas agree exactly (both give
the 45 kN·m midspan moment it asserts). The 3D `Frame3::recover` uses the same shape but a different
local dof pairing whose sign is already absorbed in its stiffness matrix — its own test
(`hex/frame` UDL cantilever, asserting `tip.m ≈ 0` with `m1 ≠ 0`) passes today, so **this is a 2D-only
defect and W6/fem3d is not affected.** Verified by reading both recoveries; not verified by cargo.

This is the whole point of adding an external oracle: the defect had been shipping under a green
suite for as long as the 2D beam recovery has existed.

---

## 6. Oracle design

### 6.1 Who judges what

| quantity | judge |
|---|---|
| nodal displacements, support reactions (all frame models, every load case) | **anastruct 1.7.0** — third-party 2D structural analysis package |
| member end forces `[N, V, M]` at both ends | a from-the-textbook Euler-Bernoulli recovery in the oracle, whose magnitude at BOTH ends anastruct's own `N`/`Q`/`M` sampling must reproduce |
| natural frequencies, buckling factors | **`scipy.linalg.eigh` / `eig`** on an independently assembled `K`, consistent `M`, consistent `K_g` |
| the singular condition | `numpy.linalg.eigvalsh` on the restrained stiffness matrix |
| `PL³/3EI`, `5wL⁴/384EI`, `0.375wL : 1.25wL`, `βₙL`, `π²EI/(KL)²` | closed form |

The textbook assembly is **never trusted alone**: `agree_with_anastruct` refuses to emit a single
number until anastruct has reproduced every displacement and every reaction of every load case to
1e-6 relative to that case's own peak response. A disagreement is a failure, never a fallback.

### 6.2 Conventions, pinned empirically

anastruct's `get_node_displacements` reports `ux`/`uy` physically but `phi_z` **clockwise**-positive,
and `get_node_results_system` reports the force the structure applies to the support. Established
against a fixed-end cantilever under a tip force, an axial force and a tip moment, and re-established
on every run: `rz = −phi_z`, `(Fx, Fy, Mz)_reaction = −(Fx, Fy, Tz)_system`. Support mapping:
`{Tx,Ty,Rz}` → `add_support_fixed`, `{Tx,Ty}` → `add_support_hinged`, `{Ty}[,Rz]` →
`add_support_roll(direction="x", rotate=…)`, `{Tx}[,Rz]` → `direction="y"`.

### 6.3 Three deliberate scope limits, each measured rather than assumed

1. **Regions are outside the projection.** anastruct expresses no meshed continuum. The projection
   carries exactly the nodes and members a frame solver can see (every node a `bar`/`beam` element
   references, in document order), so region kinds are judged on the frame INVARIANCE they must
   preserve; region geometry keeps `three-fem2d-mesh-reader` + `manifold-fem2d-mesh-measure`.
2. **Axial force is compared at the START end only.** The engine's `ElementResult::Beam` reports one
   constant `n` for all stations, which is the value at the start node. For a column carrying its own
   weight the true axial varies along the member — anastruct puts `c0_0`'s axial at 153985 N at the
   base and 149967 N at the head, a 4018.7 N difference that is exactly that column's own weight
   (`ρAL g = 7850 · 0.01491 · 3.5 · 9.81`). The engine's start-node value is right; its constancy is
   an approximation, so the gate compares the end anastruct and the engine both mean. **Recorded as a
   known engine approximation, not a defect** — but W7 may want a varying axial recovery.
3. **anastruct's per-element `M` maximum is not an end value.** Under a member UDL the largest moment
   is interior; the gate compares `result["M"][0]` and `[-1]`, not `max`.

### 6.4 `📊️expected.results.json` schema

```jsonc
{
  "schema": "semio.fem2d.analysis-reference/v1",
  "producedBy": { "oracle": "anastruct-fem2d-solver", "package": "anastruct",
                  "eigen": "scipy.linalg", "gravity": 9.81 },
  "fixtures": {
    "<snapshot filename>": {
      "scales":  { "translation": 1e-2, "rotation": 1e-2, "force": 1e5, "moment": 1e5 },
      "cases":   { "<case or combination id>": {
                     "displacements": { "<node id>": [ux, uy, rz] },
                     "reactions":     { "<node>.<dof>": value },
                     "elements":      { "<member id>": [n0, v0, m0, n1, v1, m1] } } },
      "closedForm": [ { "what": "PL³/(3EI) tip deflection", "case": "tip",
                        "quantity": "displacement"|"reactions", "node": "c8", "dof": "Ty",
                        "reactions": ["n2.Ty"], "value": -0.0410…, "tolerance": 1e-9 } ]
    }
  },
  "mutated": { "<kind>": { "scales": {…},
                           "summary": { "<case id>": { "peakTranslation", "peakRotation",
                                                       "reactionFx", "reactionFy",
                                                       "peakMoment", "peakAxial" } } } },
  "modal":    { "scale": 1000.0, "frequenciesHz": [10.0187…, 62.7911…, 175.9097…] },
  "buckling": { "scale": 100.0,  "factors": { "col_a": 1.202696, "col_b": 4.810932,
                                              "col_c": 9.842969, "col_d": 19.252952 } }
}
```

Everything is **SI, raw, reviewable**. The 25 mutated models carry the compact 6-scalar digest rather
than full results (full results ×25 would be ~700 KB of noise); the full per-value comparison for
those still happens, live, through the cross-language projection.

**Why `scales` exists.** A comparison profile carries ONE absolute tolerance, and a raw fem2d answer
mixes displacements at 1e-5 m with reactions at 1e5 N. Every projected value is divided by one of
four committed decades, read by BOTH implementations from this same file, which makes every projected
number order one and makes the profile's `tolerance: 1e-6` read as a fraction of the model's own peak
response. The same-implementation half is judged far harder: each side holds its own numbers to these
committed SI values at **1e-9 relative**, in role, before the projection is built.

### 6.5 Scenarios (35)

| id | n | what |
|---|---|---|
| `solves-<fixture>` | 6 | every load case and combination solved; committed values + closed forms |
| `solves-after-<kind>` | 25 | mutate-then-solve, all 25 kinds, plus the declared `changes`/`invariant` effect |
| `modal-cantilever-frequencies` | 1 | 3 modes vs `βₙL` at 2 % |
| `buckling-column-factors` | 1 | 4 columns vs `π²EI/(KL)²` at 2 % |
| `refuses-a-mechanism` | 1 | supports deleted → singular; supported → regular |
| `derives-the-timber-frame-substructure` | 1 | the derived fixture IS the committed document minus regions/area loads; `slab_spare` touches at exactly one node |

The `changes` / `invariant` declaration is itself evidence: a kind declared `changes` that answers
exactly as the base model does has not been exercised at all; a kind declared `invariant` that moves
the frame has corrupted something it was never meant to touch — the failure mode a document
comparison structurally cannot see. 16 kinds are `changes`, 9 `invariant` (isolated node create/
delete, unreferenced material/section create/delete, the three region kinds, and
`update-analysis-settings`, whose static answer cannot move).

**No `@mutations-<catalog>` tag.** Claiming one forces scenario ids `mutate-<kind>`/`inverse-<kind>`
over exactly that catalog's kinds and forbids any other `mutate-` id, which cannot express a case
that spans all five subsets' vocabularies from one subset's `🧪️tests`. The scenario ids here are
`solves-after-<kind>`, so `mutationCoverageBreaches` and `caseAboveSubsetBreaches` both early-return.
Coverage of the 25 kinds is asserted by the `KINDS` table in both adapters instead.

---

## 7. Registry — 22 discharged, 3 residues named

`🌐️any/🔮️oracle/🔣️.json`, via `🔨️register-fem2d-solver-oracle.py` (idempotent):

```
[register] oracles: ['fem2d-python-independent', 'three-fem2d-mesh-reader',
                     'manifold-fem2d-mesh-measure', 'serde-json-fem2d-carrier-reader',
                     'anastruct-fem2d-solver', 'scipy-fem2d-eigen-reference']
[register] comparison profiles: ['semantic-fem2d-carrier-v1', 'semantic-fem-mesh-manifold-v1',
                                 'semantic-fem2d-analysis-v1']
[register] attached anastruct-fem2d-solver to 22 mutation requirements
[register] mutation kinds still carrying an undischarged requirement: none
```

* **`anastruct-fem2d-solver`** — third-party-library, python, anastruct 1.7.0, **GPL-3.0-only**,
  `testOnly: true`, `productionReachable: false`, capabilities `["fem2d-1-mutate",
  "fem2d-1-analysis"]`, comparison profile `semantic-fem2d-analysis-v1`. The GPL is worth flagging to
  the coordinator: it is imported by a Python test adapter only, never linked and never redistributed.
* **`scipy-fem2d-eigen-reference`** — scipy 1.18.0, BSD-3-Clause, capability `["fem2d-1-analysis"]`,
  for the two eigenproblems anastruct does not express.
* **`semantic-fem2d-analysis-v1`** — `arrays: "ordered"`, `tolerance: 1e-6`, with the scale-
  normalisation rationale and the `ε·κ ≈ 1e-7` conditioning floor that sizes it.
* **`noOracleDecisions/fem2d-non-geometry-mutation-semantics`** — kept under its existing id (nothing
  references it by `@no-oracle-`), rewritten from a 22-kind debt to three named residues:

| still owed | why | who could pay it |
|---|---|---|
| `analysis.deformationScale` | scales a drawn deformed shape, enters no equation; no solver can witness it | already witnessed at carrier level by `serde-json-fem2d-carrier-reader` — scope note, not a gap |
| a `FemRegion`'s plane-stress response (CST stresses) | anastruct expresses no continuum | **`scikit-fem`**, already a declared test dependency — the obvious next ticket |
| `material.name` / `section.name` / `region.name` / `loadCase.name` / `combination.name` | a solver never reads a label | carrier-level, discharged by serde_json — scope note, not a gap |

`analysis.modalCount` / `bucklingCount` are NOT residues: they change how many modes an analysis
returns and the modal scenario compares that count and those frequencies.

---

## 8. Verification — what was run, what was not

### 8.1 The oracle, end to end, through the repository's REAL Python host

`🔨️run-fem2d-oracle.py run` builds a Protocol-v2 plan and executes it with
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` — the same host the
coordinator drives.

```
$ uv run python 🔨️run-fem2d-oracle.py run
  ✅ solves-steel-cantilever                          73.2 ms
  ✅ solves-steel-simple-beam                         21.7 ms
  ✅ solves-concrete-two-span                         23.3 ms
  ✅ solves-timber-frame-members                      26.6 ms
  ✅ solves-steel-frame-base                         143.6 ms
  ✅ solves-steel-columns                            195.5 ms
  ✅ solves-after-create-node … solves-after-update-analysis-settings   (25 scenarios, all ✅)
  ✅ modal-cantilever-frequencies                     34.4 ms
  ✅ buckling-column-factors                          57.9 ms
  ✅ refuses-a-mechanism                              13.9 ms
  ✅ derives-the-timber-frame-substructure             3.6 ms
[run] 35 passed, 0 failed (host exit 0)
```

Generation (`… generate`) reports `anastruct agrees` for all 6 benchmarks and all 25 mutated models —
31 independent third-party cross-checks, each covering every displacement, every reaction and every
member end force of every load case.

Closed-form agreement, measured:

| benchmark | closed form | reference | error |
|---|---|---|---|
| cantilever tip | `PL³/3EI` | exact at the node | < 1e-9 |
| simple beam midspan | `5wL⁴/384EI` | exact at the node | < 1e-9 |
| two-span reactions | `1.25wL` / `0.375wL` | exact | < 1e-9 |
| modal 1/2/3 | 10.0184 / 62.7838 / 175.8235 Hz | 10.0187 / 62.7911 / 175.9097 | ≤ 0.05 % |
| buckling `K` = 2.0/1.0/0.6992/0.5 | 1.20279 / 4.81116 / 9.84120 / 19.24463 | 1.202696 / 4.810932 / 9.842969 / 19.252952 | ≤ 0.05 % |

(The old in-repo modal/buckling tests use a 10 % tolerance. These are at 2 % against closed form and
1e-9 against the committed reference — W7's tolerance-tightening item is now backed by numbers.)

### 8.2 The Rust half — NOT compiled

Per the brief, no `cargo` was run. Instead every external symbol the new Rust names was grepped:

| symbol | file:line |
|---|---|
| `fem2d_solve_all(&Fem2dSnapshot) -> Result<HashMap<String, StaticResult>, Fem2dError>` | `⚙️engine/◻️2d/🦀️.rs:102` |
| `fem2d_modal(&Fem2dSnapshot) -> Result<ModalResult, Fem2dError>` | `⚙️engine/◻️2d/🎵️modal-buckling/🦀️.rs:37` |
| `fem2d_buckling(&Fem2dSnapshot, &str) -> Result<BucklingResult, Fem2dError>` | `⚙️engine/◻️2d/🎵️modal-buckling/🦀️.rs:92` |
| `StaticResult { displacements, reactions, elements, checks }` | `⚙️engine/🏗️model/🦀️.rs:310` |
| `NodeDisplacement.values: [f64;6]`, `Dof::index()` (`Tx=0, Ty=1, Rz=5`) | `⚙️engine/🏗️model/🦀️.rs:224, :30` |
| `ElementResult::{Bar{n}, Beam{stations}}`, `BeamStation{x,n,v,m}` | `⚙️engine/🏗️model/🦀️.rs:292, :239` |
| `reaction = K·u − f` at restrained dofs (physical sign) | `⚙️engine/🧮️analyses/🦀️.rs:2752` |
| `MutationMessage { level: Severity, code: FaultCode, … }` | `📡️replication/🎮️mutation/🦀️.rs:996` |
| `Scenario { id: String, steps: Vec<(String,String)> }`, `Context::fixture_bytes` | `🧪️test/📡️protocol/🦀️.rs:350,:355`, `🏃️runner/🦀️.rs:27` |
| `Json::{get,str,array,to_string}`, `Number(f64)`, `Outcome::with_raw` | `🧪️test/📡️protocol/🦀️.rs:9,:18,:536` |

Two borrow/shape hazards were found and fixed by re-reading rather than by the compiler:
`active.iter_mut().find(…)` with a `None` arm that pushes (E0502 — replaced with `position()`), and
`Json::str(key)` used on a bare `Json::String` (it reads an OBJECT member — replaced with a `label`
helper).

**Residual risk, honestly stated.** The Rust adapter and the four bridges have never been through
`rustc`. Expect ordinary type-level fixes on first compile. Two things are unproven at RUNTIME and
matter more than compile errors:

1. **`fem2d_solve_all` on `🏢️steel-frame-base`.** Its plant-room slab is pinned at all four corner
   nodes, which should make the meshed block regular, but no fem2d mesh has been solved here. If it
   comes back `Singular`, the slab (and the three region scenarios' `invariant` claim) needs a
   rethink, not the corpus.
2. **`refuses-a-mechanism`.** It asserts the engine reports the singular condition for a
   support-less cantilever. `FemError::Singular` exists and is returned on a zero pivot; whether the
   LDLT actually trips on this exact matrix is unverified.

Everything else the Rust asserts is held against numbers a third-party solver produced, so a
disagreement is information either way.

---

## 9. Exact commands

```bash
cd /Users/ueli/Documents/semio
uv sync --group test --group dev
MPL_IGNORE_SYSTEM_FONTS=1 MPLBACKEND=Agg uv run python -c "import anastruct, skfem"

uv run python .🧬semio/.../FEM-PLUGIN-END-TO-END/🔨️make-fem2d-benchmarks.py        # fixtures
uv run python .🧬semio/.../FEM-PLUGIN-END-TO-END/🔨️run-fem2d-oracle.py generate     # 📊️expected.results.json
uv run python .🧬semio/.../FEM-PLUGIN-END-TO-END/🔨️run-fem2d-oracle.py run          # 35/35 through the real host
uv run python .🧬semio/.../FEM-PLUGIN-END-TO-END/🔨️register-fem2d-solver-oracle.py  # registry
```

Regenerating the corpus is a two-step, in order: `make` then `generate` (the reference values are
derived from the fixtures, never the other way round).

## 10. Files touched

Created:
* `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/{🥒️.feature,🐍️.py,🦀️.rs}`
* the same case's `🧫️fixtures/` (9 files)
* `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/{🔨️make-fem2d-benchmarks.py,🔨️run-fem2d-oracle.py,🔨️register-fem2d-solver-oracle.py,📓️w5-fem2d-solver-oracles.md}`

Updated:
* `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs` — 4 bridges appended to `🌉️TestBridge`
* `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json` — oracles, profile, 22 requirements, narrowed decision
* `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs` — the beam moment sign (§5)
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — one member name

Nothing was deleted or renamed. `🎫️ticket.json`, `📓️status.md` and `AGENTS.md` untouched.
