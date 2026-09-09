@capability-fem3d-1-mutate
@oracle-pynite-fem3d-solver
@comparison-semantic-fem3d-analysis-v1
Feature: Solve real fem3d structures, and every typed edit to one, against an independent frame solver

  Every other reference in this artifact adjudicates the model ALGEBRA — what a typed mutation writes
  into the document. This one adjudicates the ANSWER: given a structural model, what displacements,
  reactions and member forces does it have. That question has a real third-party arbiter, and this
  case uses one — `PyNiteFEA` (MIT), an independent 3D frame and truss finite-element solver that
  assembles its own stiffness, applies its own boundary conditions and solves its own system. Nothing
  of this repository's kernel reaches it. The reference is `🐍️.py` beside this file; `🦀️.rs` registers
  the SUBJECT half only, so this repository's answer is never on both sides.

  WHY THIS CASE EXISTS. `../../../🌐️any/🔮️oracle/🔣️.json` recorded, honestly and for a long time, that
  twenty-two of this artifact's twenty-five mutation kinds had no qualifying third-party reference at
  all: `fem3d-non-geometry-mutation-semantics`, capability `fem3d-1-mutate-uncarried`. Its survey was
  right that a finite-element solver cannot adjudicate document ALGEBRA — a solver reads a model, it
  does not edit one. What that survey did not do is ask the other question. A solver adjudicates what
  the edited document MEANS, and every one of those twenty-two kinds changes what the model means:
  a section that carries less, a support that has stopped holding, a load case that no longer exists.
  So this case mutates and then SOLVES, and the debt is discharged on the half a solver really owns.

  THE COORDINATE ARGUMENT, which is load-bearing. This artifact is Z-up and PyNite is Y-up, and the
  two solvers build a member's local axes from different reference directions. The reference maps
  between the frames with the proper rotation `(X, Y, Z)ₚ = (x, z, −y)`, computes this artifact's OWN
  local triad from its documented rule, and solves for the PyNite roll angle that makes the two
  coincide — then checks the result against PyNite's own transformation matrix to `1e-12` per member.
  If that check ever fails the scenario stops before it compares anything, because `Iy` and `Iz` would
  no longer name the same axes on both sides.

  WHAT IS COMPARED, AND WHAT IS ASSERTED IN ROLE. Two solvers' last bits never agree, so the
  cross-language projection is each case's answer reduced to four scale-carrying scalars at six
  significant figures — a millionfold margin over the `1e-12`-scale disagreement two direct
  double-precision factorisations of one linear system actually show. The FULL displacement, reaction
  and member-force field is asserted numerically by the subject against
  `shared://🧮️solves-fem3d-1-benchmarks/📊️expected.results.json`, which this case's Python reference produced with PyNite and which
  is committed beside these models. Each side additionally asserts, in role and before projecting:
  the reference holds itself to the closed forms this feature states and to global equilibrium on
  every case and every combination; the subject holds itself to the committed reference within the
  tolerances stated here.

  THE LINE-ELEMENT PROJECTION. `shared://🧮️solves-fem3d-1-benchmarks/🧊️steel-frame.snapshot.json` — the shared model every fem3d
  mutation subset case uses — also carries a meshed `FemSolid`, and a frame solver has no
  tetrahedron. Every scenario here is therefore declared over the model's LINE-ELEMENT sub-document:
  the same document with `solids` emptied and every `area` load dropped. Both implementations apply
  that projection, to the same committed bytes, before solving; it scopes the verdict, it does not
  soften it. The solid path is judged by `../🧱️solves-fem3d-1-solid` against scikit-fem, and the modal
  and buckling paths by `../🎵️solves-fem3d-1-eigen` against SciPy.

  WHY THE MUTATE-THEN-SOLVE SCENARIOS CARRY THEIR OWN PAIR. The committed specification vectors of
  the mutation subsets are algebra fixtures, and their line-element sub-model is a MECHANISM: the one
  frame they hold hangs off a node whose only other stiffness comes from the solid's tetrahedra, so
  no frame solver — this repository's included — can read them. Solving them would report "singular"
  on both sides and prove nothing. This case therefore commits `shared://🧮️solves-fem3d-1-benchmarks/🦠️mutation-base.snapshot.json`,
  a portal frame that actually stands up and carries the same spares the mutation corpus needs, plus
  one after-model per kind. The typed payload relating each pair is stated in the Examples table
  below, in the vocabulary's own grammar; the subject reaches its after-model through PRODUCTION
  dispatch and is held to the committed one; the reference simply solves the committed one. The
  algebra of those same payloads stays where it belongs — with the subset cases that own it.

  THE MODELS ARE REAL. `🧊️steel-frame.snapshot.json` is the sixteen-node two-storey steel frame every
  fem3d subset case shares. `🏢️space-frame-2x2-bay.snapshot.json` is a 2 × 2-bay, 2-storey frame on a
  6 m × 5 m grid — twenty-seven nodes, forty-two members, HEB 240 columns under an IPE 400 / IPE 330
  beam grid, carrying floor dead and live UDLs and a wind resultant as storey forces, combined at
  ULS and SLS. `🏛️timber-roof-truss.snapshot.json` is a 12 m C24 king-post truss of thirteen
  pin-jointed members braced out of plane, under panel-point snow and roof build-up. All values are
  SI and all sections are real commercial profiles.

  @id-static
  @level-exhaustive
  @mode-differential
  Scenario Outline: Solve <id> whole — every load case and every combination
    Given the committed model shared://🧮️solves-fem3d-1-benchmarks/<fixture>
    And the committed reference shared://🧮️solves-fem3d-1-benchmarks/📊️expected.results.json
    When both implementations solve its line-element sub-model for every load case and combination
    Then they agree on every reduced answer, and the subject is within 1e-6 relative of the reference's full displacement, reaction and member-force field
    Examples:
    | id          | fixture                                |
    | steel-frame | 🧊️steel-frame.snapshot.json             |
    | space-frame | 🏢️space-frame-2x2-bay.snapshot.json     |
    | roof-truss  | 🏛️timber-roof-truss.snapshot.json       |

  @id-closed-form
  @level-exhaustive
  @mode-differential
  Scenario Outline: Solve <id>, where the answer is also known in closed form
    Given the committed model shared://🧮️solves-fem3d-1-benchmarks/<fixture>
    And the committed reference shared://🧮️solves-fem3d-1-benchmarks/📊️expected.results.json
    When both implementations solve it
    Then they agree on every reduced answer, and each lands on <closed-form> in role
    Examples:
    | id                  | fixture                                     | closed-form                                                       |
    | cantilever-local-z  | 📏️cantilever-tip-load-local-z.snapshot.json  | tip deflection PL³/3EIy, tip rotation PL²/2EIy, base moment PL     |
    | cantilever-local-y  | 📐️cantilever-tip-load-local-y.snapshot.json  | tip deflection PL³/3EIz, tip rotation PL²/2EIz, base moment PL     |
    | cantilever-torsion  | 🌀️cantilever-tip-torsion.snapshot.json       | tip twist TL/GJ and a base torque reaction of exactly −T           |
    | simply-supported-udl| 🌉️simply-supported-udl.snapshot.json         | midspan deflection 5wL⁴/384EIy and shoe reactions of wL/2 each     |

  @id-mutate-solve
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a frame that stands up, then solve what it became
    Given the committed model shared://🧮️solves-fem3d-1-benchmarks/🦠️mutation-base.snapshot.json
    And the committed after-model shared://🧮️solves-fem3d-1-benchmarks/🦠️mutation-after-<id>.snapshot.json
    And the committed reference shared://🧮️solves-fem3d-1-benchmarks/📊️expected.results.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    And both implementations solve the model it produced
    Then the applied model IS the committed after-model, the two agree on every reduced answer, and both report the same mechanism — which combinations changed, which are untouched, which appeared and which are gone
    Examples:
    | id                           | mutation                                                                                                                                                                    |
    | create-node                  | {"mutation":"createNode","node":{"id":"added_node","x":12.0,"y":12.0,"z":0.0}}                                                                                               |
    | delete-node                  | {"mutation":"deleteNode","id":"spare_node"}                                                                                                                                  |
    | create-element               | {"mutation":"createElement","element":{"kind":"bar","id":"brace2","start":"c4","end":"h3","materialId":"steel","sectionId":"beam"}}                                           |
    | delete-element               | {"mutation":"deleteElement","id":"brace"}                                                                                                                                    |
    | replace-element              | {"mutation":"replaceElement","id":"roof_1","newElement":{"kind":"frame","id":"roof_1","start":"h1","end":"h2","materialId":"steel","sectionId":"col","roll":0.0}}              |
    | create-material              | {"mutation":"createMaterial","material":{"id":"alu","name":"Aluminium EN AW-6082","e":70000000000.0,"g":26000000000.0,"nu":0.25,"rho":2700.0}}                                |
    | delete-material              | {"mutation":"deleteMaterial","id":"spare_material"}                                                                                                                          |
    | replace-material             | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S235","e":190000000000.0,"g":73076923077.0,"nu":0.3,"rho":7850.0}}                       |
    | create-section               | {"mutation":"createSection","section":{"id":"shs120","name":"SHS 120x120x6","area":0.00266,"iy":5.56e-05,"iz":5.56e-05,"j":8.89e-05}}                                         |
    | delete-section               | {"mutation":"deleteSection","id":"spare_section"}                                                                                                                            |
    | replace-section              | {"mutation":"replaceSection","id":"beam","newSection":{"id":"beam","name":"IPE 450 Roof Beam","area":0.00988,"iy":0.00033743,"iz":1.676e-05,"j":6.69e-07}}                     |
    | create-support               | {"mutation":"createSupport","support":{"id":"eave_stay","nodeId":"h4","fixed":["Tz"]}}                                                                                        |
    | delete-support               | {"mutation":"deleteSupport","id":"roller"}                                                                                                                                   |
    | replace-support              | {"mutation":"replaceSupport","id":"roller","newSupport":{"id":"roller","nodeId":"h1","fixed":["Tx","Ty"]}}                                                                    |
    | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"snow","name":"Snow","loads":[{"kind":"nodal","id":"s_h3","nodeId":"h3","dof":"Tz","value":-14000.0}],"selfWeight":false}}       |
    | delete-load-case             | {"mutation":"deleteLoadCase","id":"wind"}                                                                                                                                    |
    | add-load                     | {"mutation":"addLoad","caseId":"live","load":{"kind":"nodal","id":"q_h4","nodeId":"h4","dof":"Tz","value":-15000.0}}                                                          |
    | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"q_h3"}                                                                                                                    |
    | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                                 |
    | create-combination           | {"mutation":"createCombination","combination":{"id":"als","name":"Accidental","terms":{"dead":1.0,"wind":1.0}}}                                                               |
    | delete-combination           | {"mutation":"deleteCombination","id":"sls"}                                                                                                                                  |
    | update-analysis-settings     | {"mutation":"updateAnalysisSettings","settings":{"modalCount":8,"bucklingCount":5,"deformationScale":250.0}}                                                                  |
