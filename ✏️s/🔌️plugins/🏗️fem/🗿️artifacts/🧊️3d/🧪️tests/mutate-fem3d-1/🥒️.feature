@capability-fem3d-1-mutate
@oracle-fem3d-python-independent
@comparison-ordered-json-v1
@mutations-fem3d-1-any
Feature: Apply every typed fem3d model mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.fem.fem3d` structural model and all twenty-five typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` (the
  nine members, `additionalProperties: false`), from `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  (the twenty-five verbs) and from the twenty-five committed specification vectors. It imports
  nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. What this vocabulary edits is the
  MODEL, not the analysis. `code_aster`, `OpenSees`, `anastruct` and `PyNite` compute displacements
  and forces FROM a model; none reads `.dsl.semio`, none defines this document, and not one of the
  twenty-five kinds asks a solver anything — `replace-section` swaps a profile record,
  `replace-element` rolls a frame about its own axis, `add-load` appends to a case's load list.

  A DEFECT IN THE SPECIFICATION, found while writing the reference and reported rather than worked
  around, and identical to the one its `◻2d` sibling carries.
  `…/🧬️schema/🧬️mutations/🔣️.json` is a verbatim copy of the SNAPSHOT schema with `title`
  changed to `Fem3dMutation`, and in the snapshot schema itself every record `$def` is an EMPTY
  `{"title": …, "type": "object"}`. The record shapes were read off the twenty-five committed
  vectors instead — including the one that only they state: an `element` is a `frame` carrying a
  `roll` about its own axis OR a `bar` carrying none.

  The artifact is real. `local://🧊️steel-frame.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem3d-frame.py`
  from the artifact's own committed demo model
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`): a sixteen-node,
  two-storey steel frame on an 8 × 10 m grid, four fully clamped column bases, sixteen HEA 200
  members, two real materials with their real moduli, a first-floor concrete slab solid, four pinned
  slab-corner supports, a dead case with an area pressure, a live case with a nodal load and an area
  pressure, and an ULS combination at 1.35/1.5 — all carried across unchanged.

  What the derivation ADDS, and why. The committed model REFERENCES every entity it holds, so six
  unreferenced spares are appended, each taken from a committed specification vector of this same
  subset and repointed only onto ids the model already holds. They are appended LAST, so the
  committed entities keep their indices and every spare is the TRAILING member of its collection —
  which matters because no `create-` verb in this vocabulary carries an index, so the inverse of a
  delete is exact only for a trailing record. That limit is a property of the closed schema, not of
  an implementation, and both implementations share it; it is caught here only because both sides
  assert the restoring law IN ROLE, index for index.

  One deliberate exception: `delete-node` addresses `n3`, which the spare support `s_spare` points
  at. The committed vector for this kind is named `removes-the-column-head-node-under-a-live-frame`,
  so the non-cascade IS the specified behaviour, and the row exercises it against a real frame.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the nine
  members — the check an after-snapshot comparison cannot make on its own.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived steel frame
    Given the real derived model local://🧊️steel-frame.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same model, and only the member this verb writes moved
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                            |
      | create-node                  | {"mutation":"createNode","node":{"id":"n_roof","x":4.0,"y":5.0,"z":8.4}}                                                                                                                                                                            |
      | delete-node                  | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                 |
      | create-element               | {"mutation":"createElement","element":{"kind":"bar","id":"brace_00","start":"n00_g","end":"n00_l2","materialId":"steel","sectionId":"shs120"}}                                                                                                      |
      | delete-element               | {"mutation":"deleteElement","id":"fb2_1"}                                                                                                                                                                                                           |
      | replace-element              | {"mutation":"replaceElement","id":"e1","newElement":{"kind":"frame","id":"e1","start":"n00_g","end":"n00_l1","materialId":"steel","sectionId":"hea200","roll":1.5}}                                                                                 |
      | create-material              | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"g":80770000000.0,"nu":0.3,"rho":7850.0}}                                                                                                               |
      | delete-material              | {"mutation":"deleteMaterial","id":"alu"}                                                                                                                                                                                                            |
      | replace-material             | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"g":78000000000.0,"nu":0.3,"rho":7850.0}}                                                                                             |
      | create-section               | {"mutation":"createSection","section":{"id":"hea240","name":"HEA 240","area":0.00768,"iy":7.763e-05,"iz":2.769e-05,"j":4.16e-07}}                                                                                                                   |
      | delete-section               | {"mutation":"deleteSection","id":"shs120"}                                                                                                                                                                                                          |
      | replace-section              | {"mutation":"replaceSection","id":"hea200","newSection":{"id":"hea200","name":"HEA 200 with warping restraint","area":0.00538,"iy":3.69e-05,"iz":1.33e-05,"j":1.8e-06}}                                                                             |
      | create-support               | {"mutation":"createSupport","support":{"id":"s_roof","nodeId":"n00_l2","fixed":["Tz"]}}                                                                                                                                                             |
      | delete-support               | {"mutation":"deleteSupport","id":"s_spare"}                                                                                                                                                                                                         |
      | replace-support              | {"mutation":"replaceSupport","id":"s_00","newSupport":{"id":"s_00","nodeId":"n00_g","fixed":["Tx","Ty","Tz"]}}                                                                                                                                      |
      | create-solid                 | {"mutation":"createSolid","solid":{"id":"sol_roof","name":"Roof Slab","outline":[[0.0,0.0],[8.0,0.0],[8.0,10.0],[0.0,10.0]],"holes":[],"baseZ":5.6,"height":0.22,"layers":2,"meshSize":0.75,"materialId":"concrete"}}                               |
      | delete-solid                 | {"mutation":"deleteSolid","id":"sol_spare"}                                                                                                                                                                                                         |
      | replace-solid                | {"mutation":"replaceSolid","id":"sol1","newSolid":{"id":"sol1","name":"First Floor Slab thickened","outline":[[10.0,0.0],[12.0,0.0],[12.0,2.0],[10.0,2.0]],"holes":[],"baseZ":0.0,"height":0.75,"layers":2,"meshSize":1.0,"materialId":"concrete"}} |
      | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"snow","name":"Snow","loads":[{"kind":"area","id":"sn1","solidId":"sol1","pressure":900.0}],"selfWeight":false}}                                                                                      |
      | delete-load-case             | {"mutation":"deleteLoadCase","id":"wind"}                                                                                                                                                                                                           |
      | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l9","elementId":"fb1_0","wx":0.0,"wy":0.0,"wz":-3200.0}}                                                                                                                     |
      | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l3"}                                                                                                                                                                                             |
      | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                                                                                                        |
      | create-combination           | {"mutation":"createCombination","combination":{"id":"acc","name":"Accidental","terms":{"dead":1.0,"live":0.3}}}                                                                                                                                     |
      | delete-combination           | {"mutation":"deleteCombination","id":"sls_spare"}                                                                                                                                                                                                   |
      | update-analysis-settings     | {"mutation":"updateAnalysisSettings","settings":{"modalCount":8,"bucklingCount":5,"deformationScale":120.0}}                                                                                                                                        |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived frame and land back on it
    Given the real derived model local://🧊️steel-frame.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated model AND on the restored one, member for member and index for index
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                            |
      | create-node                  | {"mutation":"createNode","node":{"id":"n_roof","x":4.0,"y":5.0,"z":8.4}}                                                                                                                                                                            |
      | delete-node                  | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                 |
      | create-element               | {"mutation":"createElement","element":{"kind":"bar","id":"brace_00","start":"n00_g","end":"n00_l2","materialId":"steel","sectionId":"shs120"}}                                                                                                      |
      | delete-element               | {"mutation":"deleteElement","id":"fb2_1"}                                                                                                                                                                                                           |
      | replace-element              | {"mutation":"replaceElement","id":"e1","newElement":{"kind":"frame","id":"e1","start":"n00_g","end":"n00_l1","materialId":"steel","sectionId":"hea200","roll":1.5}}                                                                                 |
      | create-material              | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"g":80770000000.0,"nu":0.3,"rho":7850.0}}                                                                                                               |
      | delete-material              | {"mutation":"deleteMaterial","id":"alu"}                                                                                                                                                                                                            |
      | replace-material             | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"g":78000000000.0,"nu":0.3,"rho":7850.0}}                                                                                             |
      | create-section               | {"mutation":"createSection","section":{"id":"hea240","name":"HEA 240","area":0.00768,"iy":7.763e-05,"iz":2.769e-05,"j":4.16e-07}}                                                                                                                   |
      | delete-section               | {"mutation":"deleteSection","id":"shs120"}                                                                                                                                                                                                          |
      | replace-section              | {"mutation":"replaceSection","id":"hea200","newSection":{"id":"hea200","name":"HEA 200 with warping restraint","area":0.00538,"iy":3.69e-05,"iz":1.33e-05,"j":1.8e-06}}                                                                             |
      | create-support               | {"mutation":"createSupport","support":{"id":"s_roof","nodeId":"n00_l2","fixed":["Tz"]}}                                                                                                                                                             |
      | delete-support               | {"mutation":"deleteSupport","id":"s_spare"}                                                                                                                                                                                                         |
      | replace-support              | {"mutation":"replaceSupport","id":"s_00","newSupport":{"id":"s_00","nodeId":"n00_g","fixed":["Tx","Ty","Tz"]}}                                                                                                                                      |
      | create-solid                 | {"mutation":"createSolid","solid":{"id":"sol_roof","name":"Roof Slab","outline":[[0.0,0.0],[8.0,0.0],[8.0,10.0],[0.0,10.0]],"holes":[],"baseZ":5.6,"height":0.22,"layers":2,"meshSize":0.75,"materialId":"concrete"}}                               |
      | delete-solid                 | {"mutation":"deleteSolid","id":"sol_spare"}                                                                                                                                                                                                         |
      | replace-solid                | {"mutation":"replaceSolid","id":"sol1","newSolid":{"id":"sol1","name":"First Floor Slab thickened","outline":[[10.0,0.0],[12.0,0.0],[12.0,2.0],[10.0,2.0]],"holes":[],"baseZ":0.0,"height":0.75,"layers":2,"meshSize":1.0,"materialId":"concrete"}} |
      | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"snow","name":"Snow","loads":[{"kind":"area","id":"sn1","solidId":"sol1","pressure":900.0}],"selfWeight":false}}                                                                                      |
      | delete-load-case             | {"mutation":"deleteLoadCase","id":"wind"}                                                                                                                                                                                                           |
      | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l9","elementId":"fb1_0","wx":0.0,"wy":0.0,"wz":-3200.0}}                                                                                                                     |
      | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l3"}                                                                                                                                                                                             |
      | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                                                                                                        |
      | create-combination           | {"mutation":"createCombination","combination":{"id":"acc","name":"Accidental","terms":{"dead":1.0,"live":0.3}}}                                                                                                                                     |
      | delete-combination           | {"mutation":"deleteCombination","id":"sls_spare"}                                                                                                                                                                                                   |
      | update-analysis-settings     | {"mutation":"updateAnalysisSettings","settings":{"modalCount":8,"bucklingCount":5,"deformationScale":120.0}}                                                                                                                                        |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-model asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
      | id                           | dir                           | fixture                                                     |
      | create-node                  | 🌱⚪️create-node                | appends-the-column-head-node-n3                             |
      | delete-node                  | 🗑⚪️delete-node                | removes-the-column-head-node-under-a-live-frame             |
      | create-element               | 🌱🧩️create-element             | appends-a-diagonal-bracing-bar                              |
      | delete-element               | 🗑🧩️delete-element             | removes-the-bracing-bar-and-leaves-the-frame                |
      | replace-element              | 🔁🧩️replace-element            | rolls-the-column-about-its-own-axis                         |
      | create-material              | 🌱🧱️create-material            | appends-an-aluminium-alloy                                  |
      | delete-material              | 🗑🧱️delete-material            | removes-the-unreferenced-aluminium-alloy                    |
      | replace-material             | 🔁🧱️replace-material           | softens-the-steel-shear-modulus-in-place                    |
      | create-section               | 🌱create-section               | appends-a-square-hollow-profile                             |
      | delete-section               | 🗑📐️delete-section             | removes-the-spare-square-hollow-profile                     |
      | replace-section              | 🔁📐️replace-section            | raises-the-torsion-constant-of-hea200                       |
      | create-support               | 🌱🛡️create-support             | clamps-the-column-base-in-all-six-dofs                      |
      | delete-support               | 🗑delete-support               | releases-the-pinned-node-n2                                 |
      | replace-support              | 🔁🛡️replace-support            | frees-the-three-rotations-at-the-column-base                |
      | create-solid                 | 🌱🧊️create-solid               | appends-an-extruded-roof-slab                               |
      | delete-solid                 | 🗑🧊️delete-solid               | removes-the-roof-slab-and-keeps-its-material                |
      | replace-solid                | 🔁replace-solid                | thickens-the-slab-and-adds-a-mesh-layer                     |
      | create-load-case             | 🌱📋️create-load-case           | appends-a-wind-case-pushing-on-the-column-head              |
      | delete-load-case             | 🗑📋️delete-load-case           | removes-the-wind-case-together-with-its-load                |
      | add-load                     | ➕add-load                     | lays-an-area-pressure-over-the-roof-slab                    |
      | remove-load                  | ➖remove-load                  | drops-the-trailing-member-udl-from-the-dead-case            |
      | change-load-case-self-weight | ⚖change-load-case-self-weight | switches-self-weight-off-for-the-dead-case                  |
      | create-combination           | 🌱🔗️create-combination         | appends-a-serviceability-combination-keyed-by-case-id       |
      | delete-combination           | 🗑🔗️delete-combination         | removes-the-serviceability-combination-and-keeps-both-cases |
      | update-analysis-settings     | 🎛update-analysis-settings     | doubles-the-buckling-mode-count                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived model local://🧊️steel-frame.snapshot.json
    And the artifact's own committed carrier asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived model, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same nine members, and the Rust reproduces the committed carrier byte for byte
