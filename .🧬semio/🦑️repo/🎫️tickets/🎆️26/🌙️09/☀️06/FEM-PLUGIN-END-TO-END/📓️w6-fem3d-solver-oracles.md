# 🧮 W6 — fem3d third-party FEM solver oracles

Scope: give `🗿️artifacts/🧊️3d` real third-party SOLVER oracles and retire the
`fem3d-non-geometry-mutation-semantics` / `fem3d-1-mutate-uncarried` debt honestly. No Rust or TS
compiler was run (host swap exhausted, per brief); everything Python was run end to end.

**Headline: 25/25 fem3d mutation kinds now carry a third-party `fem3d-1-mutate` oracle (was 3/25);
`noOracleDecisions` is empty (was 1). 35 oracle scenarios run green, every one of them additionally
held to a closed form and/or to global equilibrium in role.**

---

## 1. Dependencies

`pyproject.toml`'s `test` group already declared `PyNiteFEA>=1.1.6`, `scikit-fem>=10.0.2` and
`anastruct>=1.6.1` (added by the lost 02:30 fleet). `uv sync --group test --group dev` installed
`jsonpatch` and `networkx` and resolved 150 packages; nothing else was missing.

Proven present, by import:

| Package | Import | Version |
|---|---|---|
| PyNiteFEA | `from Pynite.FEModel3D import FEModel3D` | 3.0.0 |
| scikit-fem | `import skfem` | 12.0.2 |
| SciPy | `import scipy` | 1.18.0 |
| NumPy | `import numpy` | 2.5.0 |

**One fallback, documented.** `import Pynite` (the package root) pulls `Pynite.ShearWall`, which
imports `matplotlib.pyplot` at module level; on this host `matplotlib.font_manager` raised
`KeyError: '_items'` on first import because `system_profiler SPFontsDataType -json` was returning
garbage under the load (a bare `system_profiler` call did not return inside 300 s). The oracles
therefore import `from Pynite.FEModel3D import FEModel3D` and `from Pynite.PhysMember import
PhysMember` directly, never the package root — which is correct on its own merits (a solver oracle
has no business pulling a plotting stack) and which removes the whole failure mode. The font cache
has since been built, so `import Pynite` also works again on this machine; the direct imports stay.

## 2. The reference corpus — three cases, not one

The test platform binds exactly ONE `@oracle-<id>` per feature file (`🟦️.ts:415`, `tagValue`, plus
the `oracle-capability-mismatch` / `oracle-profile-mismatch` gates at `:1650-1653`), so three
libraries need three cases. They sit side by side under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/`:

| Case | Oracle | Scenarios | What it adjudicates |
|---|---|---|---|
| `🧮️solves-fem3d-1-benchmarks` | `pynite-fem3d-solver` | 29 | linear static on real frames, four closed-form cases, mutate-then-solve × 22 kinds |
| `🎵️solves-fem3d-1-eigen` | `scipy-fem3d-eigen` | 5 | modal frequencies, Euler buckling at K = 0.5/0.7/1.0/2.0 |
| `🧱️solves-fem3d-1-solid` | `skfem-fem3d-solid` | 1 | 3D continuum elasticity on the `Tet4` solid path |

There is **no duplicated code** between them: the beam `K`/`M`/`Kg` assembly lives only in the eigen
case that needs it, the scikit-fem elasticity only in the solid case, the PyNite bridge only in the
benchmarks case. Each carries its own `🥒️.feature`, `🐍️.py` (oracle), `🦀️.rs` (subject) and
`🧫️fixtures/`.

### Fixtures (all authored by `🔨️author-fem3d-benchmark-fixtures.py`, in the artifact's own schema)

**Real-world models** — `🧊️steel-frame.snapshot.json` (the existing shared 16-node two-storey steel
frame, copied in verbatim), `🏢️space-frame-2x2-bay.snapshot.json` (2 × 2-bay, 2-storey, 6 m × 5 m
grid; 27 nodes, 42 members, HEB 240 columns under an IPE 400/IPE 330 beam grid, floor dead + live
UDLs, wind as storey forces, ULS + SLS combinations), `🏛️timber-roof-truss.snapshot.json` (12 m C24
king-post truss, 13 pin-jointed `bar` members braced out of plane, panel-point snow + roof build-up).

**Closed-form models** — `📏️cantilever-tip-load-local-z` (bends on `iy`), `📐️cantilever-tip-load-local-y`
(bends on `iz`), `🌀️cantilever-tip-torsion` (`TL/GJ`), `🌉️simply-supported-udl` (two elements, node at
midspan).

**Solid model** — `🧱️prismatic-solid-column.snapshot.json`, a 2 × 2 × 4 m C30/37 prism. Its `meshSize`
is deliberately larger than the footprint diagonal so the footprint triangulates into exactly its
four corners; every base node is then a DOCUMENT node the fixture can restrain, which is what lets
the restraint set be the one the uniform-stress solution itself satisfies (`Tz` at four corners,
`Tx`+`Ty` at the origin, `Ty` at +x, `Tx` at +y). Under uniform top pressure the answer is then
exactly `−pH/E`.

**Eigen models** — `🎵️modal-cantilever` (8 elements, `iy ≠ iz` so no mode is a double root) and four
`🏛️buckling-column-*` fixtures (8 elements, unit axial reference load so the lowest factor IS the
critical load in newtons, `iz < iy` so the first mode is unambiguous).

**Mutate-then-solve corpus** — `🦠️mutation-base.snapshot.json` plus one `🦠️mutation-after-<kind>` per
kind (22 files). See §3 for why these exist rather than the committed specification vectors.

## 3. Why the mutate-then-solve scenarios carry their own pair

The committed `🧫️fixtures/<kind>/{⏮️before,⏭️after}.json` pairs of the five mutation subsets — 22 of
them, exactly the 22 non-geometry kinds — all share one small model: 3 nodes, one `bar`, one `frame`,
one `FemSolid`. **That model's line-element sub-system is a mechanism.** Node `n2` carries the frame
`e2` and the bar `e1`; the bar resists only along global X, `sup2` fixes only `Tz`, and nothing
restrains the `{n2, n3}` sub-assembly against a rigid rotation about `n2` — the solid's `Tet4`
elements have no rotational degrees of freedom to supply it. PyNite reports
`The stiffness matrix is singular… The structure is unstable` for all 22, and this repository's own
`solve_linear_static` would return `FemError::Singular` for the same reason. Solving them would prove
nothing on either side.

So the case commits its own solvable pair: a single-bay 6 m × 5 m steel portal frame with four fixed
feet, a closed eave ring and a diagonal `bar` brace, carrying the same spares the mutation corpus
needs (an unreferenced material, an unreferenced section, an unattached node, a redundant roller at
one eave, a load case no combination cites). The typed payload relating each pair is stated in the
feature's Examples table in the vocabulary's own grammar; the subject reaches its after-model through
PRODUCTION dispatch (`fem3d_mutation_report_json`) and is held to the committed after-model by
`law::divergence` before anything is solved. The ALGEBRA of those payloads stays where it belongs,
with the subset cases that own it.

Mechanism table the reference produced (both implementations must report it identically):

| Verdict | Kinds |
|---|---|
| `changed` (every combination) | create/delete/replace-element, replace-material, replace-section, create/delete/replace-support |
| `changed` (only the cases that cite the edited record) | add-load, remove-load, change-load-case-self-weight |
| `unchanged` | create/delete-material, create/delete-section, update-analysis-settings |
| `reshaped` | create-node, delete-node (the node set itself moves) |
| `appeared` / `disappeared` | create/delete-load-case, create/delete-combination |

`update-analysis-settings` reporting `unchanged` under linear static is the correct answer and is
asserted as such; the kind's own solver relevance (mode and factor counts) is carried by the second
oracle attached to it, `scipy-fem3d-eigen`.

## 4. What the oracles assert in role, before they project anything

* **Coordinate reconciliation, checked not assumed.** This artifact is Z-up and PyNite is Y-up, and
  the two build a member's local triad from different references (`+Z` with a `+X` fallback near
  vertical, vs `+Y`). The reference maps frames with the proper rotation `(X, Y, Z)ₚ = (x, z, −y)`
  (determinant +1), computes this artifact's own triad from its documented rule, solves for the
  PyNite roll angle that makes them coincide, and then **verifies the result against PyNite's own
  `T()` to `1e-12` per member**. If that fails the scenario stops before comparing anything.
* **Global equilibrium** — `Σ reactions + Σ applied = 0` to `1e-9` relative — on every load case and
  every combination of every model, including the ones with no closed form.
* **Closed forms** — `PL³/3EI`, `PL²/2EI`, `PL`, `TL/GJ`, `5wL⁴/384EI`, `wL/2`,
  `βₙ²/(2πL²)·√(EI/ρA)`, `π²EI/(KL)²`, `−pH/E`, `−ρgH²/2E`.

Two integration findings, reported rather than worked around:

1. **PyNite's `add_member` builds a PHYSICAL member that silently subdivides itself at any model node
   lying on its axis.** On the shared `🧊️steel-frame` fixture that turns the spare node `n3` (0, 0, 3.5)
   — which sits on upper column `col_00_u` but is named by no element — into a mid-height support
   carrying **8271.7 N**, which this artifact's own model never puts there. Caught by the equilibrium
   assertion (reactions summed 30 838.7 N against an applied 39 110.5 N). The reference defines
   `LiteralMember`, a `PhysMember` whose subdivision search is narrowed to its own two end nodes, so
   PyNite's element set is this artifact's element set.
2. **A `bar` is not a PyNite member.** `FemElement::Bar` has three translational degrees of freedom
   and no bending stiffness; PyNite's `Spring3D` is exactly that, so a bar maps to
   `add_spring(ks = EA/L)` and every rotational freedom at a node only bars touch is restrained —
   mirroring `crate::model::build_dof_map`'s union-of-incident-elements rule.

## 5. Verification — the oracle run

`uv run python .🧬semio/…/FEM-PLUGIN-END-TO-END/🔨️run-fem3d-oracle.py`, final run:

```
== 🧮️solves-fem3d-1-benchmarks (29 scenarios)   29 ok   → 📊️expected.results.json  491 605 bytes
== 🎵️solves-fem3d-1-eigen        (5 scenarios)    5 ok   → 📊️expected.results.json    2 556 bytes
== 🧱️solves-fem3d-1-solid        (1 scenario)     1 ok   → 📊️expected.results.json      751 bytes
```

Selected reference values (all produced by the third-party libraries, all inside their stated
tolerance of the closed form):

| Benchmark | Reference | Closed form | Error |
|---|---|---|---|
| modal cantilever mode 1 | 9.01442 Hz | 9.01442 Hz | 0.000 % |
| modal cantilever mode 3 | 56.4969 Hz | 56.4924 Hz | 0.008 % |
| Euler column K = 2.0 | 201 668.1 N | 201 667.7 N | 0.000 % |
| Euler column K = 1.0 | 806 697.2 N | 806 670.8 N | 0.003 % |
| Euler column K = 0.7 | 1 650 469.5 N | 1 646 266.9 N | 0.255 % |
| Euler column K = 0.5 | 3 228 335.7 N | 3 226 683.2 N | 0.051 % |
| prism under 0.5 MPa | −6.060606e-5 m | −6.060606e-5 m | 8e-16 |
| prism under self weight | −5.692536e-6 m | −5.707636e-6 m | 0.265 % |

The timber truss's bar forces are textbook-correct as a sanity signal: bottom chord +37.5/+26.3 kN
tension, rafters −40.4 kN compression, and `vt2` (the king post under symmetric loading) exactly a
zero-force member.

`📊️expected.results.json` carries both halves per scenario: the `projection` the coordinator compares
across languages, and the full-precision `reference` the Rust subject is held to numerically.

## 6. The Rust side (written, NOT compiled — per brief)

Three subject adapters (`🦀️.rs` per case), registered like every sibling through
`semio_repo_test_host::Adapter`, SUBJECT role only. They are **not** mounted in the crate entry: the
subset-level `🧪️tests/*/🦀️.rs` adapters are compiled by the generated cache-local host crate
(`🧪️test/📜️script.ts:418-470`), which links `semio-repo-test-host`, the SUT behind `--features sut`,
and the owner's contributed oracle host package — the crate entry has no mount for
`📈️mutate-fem3d-1-analysis` either, so none was added for these. **No edit was made to
`📦️packages/🦀️rust/🦀️.rs`.**

Engine API re-read against the source, file:line:

| Call | Definition |
|---|---|
| `fem3d_solve_all(&Fem3dSnapshot) -> Result<HashMap<String, StaticResult>, Fem3dError>` | `⚙️engine/🧊️3d/🦀️.rs:82` |
| `fem3d_modal(&Fem3dSnapshot) -> Result<ModalResult, Fem3dError>` | `⚙️engine/🧊️3d/🎵️modal-buckling/🦀️.rs:34` |
| `fem3d_buckling(&Fem3dSnapshot, &str) -> Result<BucklingResult, Fem3dError>` | `⚙️engine/🧊️3d/🎵️modal-buckling/🦀️.rs:66` |
| `meshing::resolve_geometry(&Fem3dSnapshot) -> Result<ResolvedGeometry, Fem3dError>` | `⚙️engine/🧊️3d/🕸️meshing/🦀️.rs:43` |
| `Node { pub id, pub pos: [f64;3] }` | `⚙️engine/🏗️model/🦀️.rs:46` |
| `ModalResult { pub frequencies_hz, pub shapes }` | `⚙️engine/🧮️analyses/🦀️.rs:405` |
| `BucklingResult { pub factors, pub shapes }` | `⚙️engine/🧮️analyses/🦀️.rs:412` |
| `Dof::index(self) -> usize` | `⚙️engine/🏗️model/🦀️.rs:30` |
| `fem3d_mutation_report_json(&str, &str, &str) -> Result<String, String>` | `🌐️any/🧬️schema/🧬️mutations/🦀️.rs:519` |
| `law::divergence(&Json, &Json) -> Option<String>` | `🗄️stdio/🧪️oracle/⚖️law/🦀️.rs:77` |
| `Adapter::subject`, `Context::{fixture_json, fixture_bytes, doc_string, scenario}` | `🧪️test/🏃️runner/🦀️.rs:121, :22-79` |

The adapters read the snapshot with a small literal JSON reader rather than the DSL codec, on
purpose: the subject here is the SOLVER, and what the DSL/pack/store encodings do with these bytes is
the `🌐️any` round-trip cases' business. That also keeps the generated host's dependency graph to the
crates it already links (adding `semio-framework-schema` would have meant editing the host
generator).

**Compile-verification is owed.** The adapters were written against the signatures above and reviewed
for the usual traps (`&f64` arithmetic, `ok_or` error types, `Copy` on `Dof`, match ergonomics in
`for` patterns over `&[&str]`), but they have never been through `rustc`. First compile should be
`cargo run --manifest-path <generated host> --features sut`.

## 7. Registry — what changed in `🌐️any/🔮️oracle/🔣️.json`

Applied by `🔨️register-fem3d-solver-oracles.py`, which refuses to run unless the file round-trips
byte-identically first, so a concurrent editor's work in the same file survives.

* **`oracles` 4 → 7**: `pynite-fem3d-solver` (MIT, PyNiteFEA 3.0.0, engine family `pynite`),
  `scipy-fem3d-eigen` (BSD-3-Clause, SciPy 1.18.0, family `lapack`), `skfem-fem3d-solid`
  (BSD-3-Clause, scikit-fem 12.0.2, family `scikit-fem`). All `testOnly`, none production-reachable,
  none needing the network, none sharing an engine family with `crate::sparse`.
* **`comparisonProfiles` 2 → 5**: `semantic-fem3d-analysis-v1`, `semantic-fem3d-eigen-v1`,
  `semantic-fem3d-solid-v1`. Each describes exactly what its projection compares and why the full
  field is not in it.
* **`oracleRequirements`**: the 22 non-geometry kinds' unfilled
  `{capability: fem3d-1-mutate, qualifyingKind: third-party-library}` now names
  `pynite-fem3d-solver`; `update-analysis-settings` additionally carries `scipy-fem3d-eigen`; the
  three solid kinds additionally carry `skfem-fem3d-solid` alongside their existing `three` +
  `manifold-3d` geometry pair. **25/25.**
* **`fixtureManifests` 37 → 40**: one `third-party-generated` entry per case pinning its
  `📊️expected.results.json` by sha256 and byte count, with the generator command and the licence
  attribution. Digests verified fresh against disk after the final oracle run.
* **`noOracleDecisions` 1 → 0**: `fem3d-non-geometry-mutation-semantics` removed. Its survey was
  right that a solver cannot adjudicate document ALGEBRA; it asked only half the question. A solver
  adjudicates what the edited document MEANS, and that half is now discharged.

`🔣️taxonomy.json` `members-of-tests` gained the three new case directory names beside
`📈️mutate-fem3d-1-analysis` (line 10218ff), so discovery's name registry stays set-equal with disk.

## 8. Finding for W7 / the kernel — `Frame3::local_mass` has the wrong sign in the y-bending plane

`✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs` builds the consistent mass with the SAME 156/22L
block in both bending planes:

```rust
let block = [[156.0, 22.0*l, 54.0, -13.0*l], …];
for … in [1usize, 5, 7, 11] { m.set(gi, gj, factor * block[bi][bj]); }   // z-plane, iz
for … in [2usize, 4, 8, 10] { m.set(gi, gj, factor * block[bi][bj]); }   // y-plane, iy — same block
```

But the file's own stiffness (`local_stiffness`) and its own UDL vector (`local_udl`, `f(4) =
-wz·L²/12`, `f(10) = +wz·L²/12`) are written in the `θy = −∂w/∂x` convention, which flips the sign of
the `L` terms in that plane — formally `S·M·S` with `S = diag(1, −1, 1, −1)`. The mass matrix is the
only one of the three that does not carry the flip, so it disagrees with the element's own UDL path.

Measured, on the committed `🎵️modal-cantilever` fixture (8 elements) and on a one-element cantilever
under self weight, with everything else held equal:

| | mode 1 | mode 2 | mode 3 | mode 4 |
|---|---|---|---|---|
| closed form | 9.01442 | 14.98530 | 56.49241 | 93.91127 Hz |
| flipped (correct) | +0.000 % | +0.000 % | +0.008 % | +0.008 % |
| **as implemented** | +0.000 % | **−0.541 %** | +0.008 % | **+1.056 %** |

Modes 2 and 4 are the y-plane bending family; they are the only ones affected, which is exactly the
signature of this defect. It passes the kernel's own 10 % modal tolerance and fails a 1 % one.

The static consequence is larger, because `solve_multi_case` builds self weight as
`element.mass() · gravity_pattern` (`🧮️analyses/🦀️.rs:2583-2591`). For a horizontal member the local
triad puts gravity entirely in the y-bending plane, so the fixed-end moments come out with the wrong
sign. One-element cantilever, self weight only, tip deflection against `wL⁴/8EI`:

| | tip w | error |
|---|---|---|
| flipped (correct) | −1.709976e-3 m | 0.00 % |
| **as implemented** | −2.849959e-3 m | **−66.67 %** |

The error shrinks with mesh refinement but does not vanish: the sign is wrong per element.
`self_weight_matches_total_mass_times_gravity` cannot see it, because only the ROTATIONAL rows are
wrong and the translational row sums — which is what a reaction total measures — are untouched.

**Fix:** apply `S·M·S` to the `[2, 4, 8, 10]` block in `Frame3::local_mass` (and check
`BeamEb2`/`elements2d` for the same shape). Until it lands, the `🎵️solves-fem3d-1-eigen` modal
scenario and any self-weight case on a horizontal frame member are expected to FAIL against the
committed reference — which is the correct behaviour of a real oracle, and the reason this ticket
added one.

## 9. Files

Created:
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧮️solves-fem3d-1-benchmarks/{🥒️.feature,🐍️.py,🦀️.rs}` + `🧫️fixtures/` (30 fixtures + `📊️expected.results.json`)
- `…/🧪️tests/🎵️solves-fem3d-1-eigen/{🥒️.feature,🐍️.py,🦀️.rs}` + `🧫️fixtures/` (5 fixtures + reference)
- `…/🧪️tests/🧱️solves-fem3d-1-solid/{🥒️.feature,🐍️.py,🦀️.rs}` + `🧫️fixtures/` (1 fixture + reference)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🔨️author-fem3d-benchmark-fixtures.py`
- `…/FEM-PLUGIN-END-TO-END/🔨️run-fem3d-oracle.py`
- `…/FEM-PLUGIN-END-TO-END/🔨️register-fem3d-solver-oracles.py`
- `…/FEM-PLUGIN-END-TO-END/📓️w6-fem3d-solver-oracles.md` (this file)

Updated:
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`

Untouched, deliberately: `📦️packages/🦀️rust/🦀️.rs` (no mount was needed), every file W1/W3/W4/W5/W7
owns, `🗑️generated/`, `🎫️ticket.json`.

## 10. Owed

1. `cargo` compile of the three new subject adapters (native, then `wasm32-wasip2`).
2. A coordinator run of the repo test platform over the three cases, both roles, to confirm the
   cross-language projections match — the Python halves are proven, the Rust halves are not.
3. The `Frame3::local_mass` fix (§8) before the eigen and self-weight scenarios can be green.
4. The identical treatment for `◻️2d` — `anastruct` is installed and W5 owns it; the debt entry
   `fem2d-non-geometry-mutation-semantics` is still open in the 2d registry.
