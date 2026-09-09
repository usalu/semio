@capability-fem2d-1-mutate
@oracle-anastruct-fem2d-solver
@comparison-semantic-fem2d-analysis-v1
Feature: Solve real fem2d models, and every mutation of one, against a third-party structural solver

  THE DEBT THIS CASE PAYS. `../../../🌐️any/🔮️oracle/🔣️.json` recorded, under
  `noOracleDecisions/fem2d-non-geometry-mutation-semantics`, that twenty-two of this artifact's
  twenty-five mutation kinds owed a qualifying third-party reference and had none. Its reasoning was
  that "code_aster, OpenSees, anastruct and PyNite compute displacements and forces FROM a model,
  while every one of these twenty-two kinds edits the model DOCUMENT itself". That is right about
  what a solver can READ and wrong about what a solver can JUDGE. A solver cannot say whether
  `deleteSupport` removed the right record — the sibling case `📈️mutate-fem2d-1-analysis` and its
  four siblings already adjudicate that, against a second implementation of the algebra. What a
  solver can say, with total independence, is what the structure DOES once that record is gone, and
  that is the thing all twenty-two kinds exist to change.

  So this case compares no documents. It compares ANALYSIS RESULTS — nodal displacements, support
  reactions and member end forces, per load case and per combination — and it reaches every mutation
  kind by MUTATE-THEN-SOLVE: the subject applies the mutation through production dispatch and solves
  the mutated model with this repository's engine, while the reference solves the independently
  committed post-mutation snapshot from scratch. A mutation that lands on a different model than the
  one committed produces a different structure, and a different structure answers differently.

  THE REFERENCE is `🐍️.py` beside this file. Every displacement and every reaction it emits for a
  frame model comes out of `anastruct` (https://github.com/ritchie46/anaStruct, GPL-3.0, test-only,
  never reachable from production), a 2D structural analysis package written by someone who has never
  seen this repository. Member end forces and the two eigenproblems, which anastruct does not
  express, come from a from-the-textbook Euler-Bernoulli assembly handed to `scipy.linalg` — and
  that assembly is never trusted alone: the reference refuses to emit a single number until anastruct
  has reproduced every displacement and every reaction of every load case to 1e-6 relative, and until
  anastruct's own N/Q/M sampling has reproduced the end forces at both member ends.

  THE FIXTURES. `shared://🧮️solves-fem2d-1-benchmarks/🏗️timber-portal-frame.snapshot.json` is the committed real-world timber
  portal frame every fem2d mutation case shares, carried here verbatim.
  `shared://🧮️solves-fem2d-1-benchmarks/🪵️timber-frame-members.snapshot.json` is its FRAME SUBSTRUCTURE — identical nodes,
  members, materials, sections and supports, with the two slab regions and the two area loads
  dropped. The committed document itself is not solved, and the `derives-the-timber-frame-substructure`
  scenario states the reason as a checkable geometric fact rather than as prose: its `slab_spare`
  region touches the rest of the model at exactly ONE node, so the meshed slab can rotate rigidly
  about that node. That region exists to give the `delete-`/`replace-region` verbs a trailing target,
  not to be analysed. `shared://🧮️solves-fem2d-1-benchmarks/📏️steel-cantilever.snapshot.json` (6 m IPE 300, eight elements) and
  `shared://🧮️solves-fem2d-1-benchmarks/📐️steel-simple-beam.snapshot.json` (8 m IPE 400 under 12 kN/m) carry the two closed forms
  `PL³/(3EI)` and `5wL⁴/(384EI)`, exact at the nodes for consistent-load Euler-Bernoulli elements.
  `shared://🧮️solves-fem2d-1-benchmarks/🌉️concrete-two-span.snapshot.json` is a 2 × 6 m continuous 300×600 concrete beam whose
  `0.375wL : 1.25wL : 0.375wL` reaction split is a textbook value.
  `shared://🧮️solves-fem2d-1-benchmarks/🏢️steel-frame-base.snapshot.json` is a three-bay two-storey steel moment frame with dead,
  live, wind and spare cases, three combinations, a detached four-corner-pinned plant-room slab and
  one spare record for every `delete-`/`replace-` verb this vocabulary declares — it is the base every
  mutation is applied to. `shared://🧮️solves-fem2d-1-benchmarks/🏛️steel-columns.snapshot.json` carries four disjoint columns, one
  per standard effective-length factor, each with its own reference compression.

  THE COMMITTED REFERENCE VALUES are `shared://🧮️solves-fem2d-1-benchmarks/📊️expected.results.json`, in SI units, produced by the
  reference above and pinned by BOTH implementations on every run. It also carries the four
  normalisation decades the cross-language projection divides by, because the comparison profile
  carries one absolute tolerance and a projection mixing metres at 1e-5 with newtons at 1e5 could not
  be judged by it at all.

  THE MUTATION CORPUS is `shared://🧮️solves-fem2d-1-benchmarks/🧬️mutated.snapshots.json`: per kind, the payload and the committed
  post-mutation snapshot, plus whether that kind is expected to CHANGE the frame analysis or to leave
  it INVARIANT. Both are evidence. A kind declared `changes` that answers exactly as the base model
  did has not been exercised at all; a kind declared `invariant` that moves the frame has corrupted
  something it was never meant to touch — which is precisely the failure mode a document comparison
  cannot see. Region kinds are `invariant` here BY CONSTRUCTION: a meshed continuum is not something
  a 2D frame package can express, so the projection carries exactly the nodes and members a frame
  solver can see, and region geometry keeps its own two third-party oracles
  (`three-fem2d-mesh-reader`, `manifold-fem2d-mesh-measure`).

  UNITS AND SIGNS, stated once because two solvers disagree about them: metres, newtons,
  newton-metres, radians; `x` right and `y` UP; `rz` counter-clockwise positive; a reaction is the
  force the support applies TO the structure; a member's axial force is tension-positive and is
  reported at its START node, because that is what this artifact's own beam recovery reports for a
  member carrying a longitudinal distributed load.

  @id-solves
  @level-exhaustive
  @mode-differential
  Scenario Outline: Solve <id> and answer what a third-party structural solver answers
    Given the committed model shared://🧮️solves-fem2d-1-benchmarks/<fixture>
    And the committed reference values shared://🧮️solves-fem2d-1-benchmarks/📊️expected.results.json
    When every load case and every combination is solved for linear static equilibrium
    Then both implementations report the same displacements, reactions and member end forces, and every closed form the model carries holds
    Examples:
    | id                   | fixture                              |
    | steel-cantilever     | 📏️steel-cantilever.snapshot.json     |
    | steel-simple-beam    | 📐️steel-simple-beam.snapshot.json    |
    | concrete-two-span    | 🌉️concrete-two-span.snapshot.json    |
    | timber-frame-members | 🪵️timber-frame-members.snapshot.json |
    | steel-frame-base     | 🏢️steel-frame-base.snapshot.json     |
    | steel-columns        | 🏛️steel-columns.snapshot.json        |

  @id-solves-after
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the base frame, solve what it left behind, and answer what the solver answers
    Given the base model shared://🧮️solves-fem2d-1-benchmarks/🏢️steel-frame-base.snapshot.json
    And the committed post-mutation corpus shared://🧮️solves-fem2d-1-benchmarks/🧬️mutated.snapshots.json
    And the committed reference values shared://🧮️solves-fem2d-1-benchmarks/📊️expected.results.json
    When the <id> mutation is applied through production dispatch and the result is solved
    Then both implementations report the same analysis, and the model <effect> exactly as this kind declares
    Examples:
    | id                           | effect     |
    | create-node                  | invariant  |
    | delete-node                  | invariant  |
    | create-element               | changes    |
    | delete-element               | changes    |
    | replace-element              | changes    |
    | create-material              | invariant  |
    | delete-material              | invariant  |
    | replace-material             | changes    |
    | create-section               | invariant  |
    | delete-section               | invariant  |
    | replace-section              | changes    |
    | create-support               | changes    |
    | delete-support               | changes    |
    | replace-support              | changes    |
    | create-region                | invariant  |
    | delete-region                | invariant  |
    | replace-region               | invariant  |
    | create-load-case             | changes    |
    | delete-load-case             | changes    |
    | add-load                     | changes    |
    | remove-load                  | changes    |
    | change-load-case-self-weight | changes    |
    | create-combination           | changes    |
    | delete-combination           | changes    |
    | update-analysis-settings     | invariant  |

  @id-modal-cantilever-frequencies
  @level-exhaustive
  @mode-differential
  Scenario: Free-vibration frequencies of the cantilever match the closed form to two percent
    Given the committed model shared://🧮️solves-fem2d-1-benchmarks/📏️steel-cantilever.snapshot.json
    And the committed reference values shared://🧮️solves-fem2d-1-benchmarks/📊️expected.results.json
    When the lowest natural frequencies the analysis settings ask for are solved
    Then each one is within 2 percent of βₙ²/(2πL²)·√(EI/ρA) for βₙL of 1.8751, 4.6941 and 7.8548, and both implementations agree

  @id-buckling-column-factors
  @level-exhaustive
  @mode-differential
  Scenario: Buckling factors of the four columns match π²EI/(KL)² to two percent
    Given the committed model shared://🧮️solves-fem2d-1-benchmarks/🏛️steel-columns.snapshot.json
    And the committed reference values shared://🧮️solves-fem2d-1-benchmarks/📊️expected.results.json
    When each column's own load case is solved for its lowest linear-buckling factor
    Then each factor is within 2 percent of π²EI/(KL)² divided by the applied reference load, for K of 2.0, 1.0, 0.6992 and 0.5, and both implementations agree

  @id-refuses-a-mechanism
  @level-exhaustive
  @mode-differential
  Scenario: A cantilever with its only support deleted is refused rather than answered
    Given the committed model shared://🧮️solves-fem2d-1-benchmarks/📏️steel-cantilever.snapshot.json
    When every support is deleted and the model is solved again
    Then both implementations report the singular condition instead of a displacement, and both still solve the supported model

  @id-derives-the-timber-frame-substructure
  @level-exhaustive
  @mode-differential
  Scenario: The frame substructure is the committed real-world document, minus a slab hung off one node
    Given the committed real-world model shared://🧮️solves-fem2d-1-benchmarks/🏗️timber-portal-frame.snapshot.json
    And the derived frame substructure shared://🧮️solves-fem2d-1-benchmarks/🪵️timber-frame-members.snapshot.json
    When the two are compared member by member
    Then the substructure changes nothing but the regions and the area loads, and the dropped `slab_spare` region is shown to touch the frame at exactly one node
