@capability-fem2d-1-mutate
@oracle-fem2d-python-independent
@comparison-ordered-json-v1
@mutations-fem2d-1-any
Feature: Apply every typed fem2d model mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.fem.fem2d` structural model and all twenty-five typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` (the
  nine members, `additionalProperties: false`), from `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`
  (the twenty-five verbs) and from the twenty-five committed specification vectors. It imports
  nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. What this vocabulary edits is the
  MODEL, not the analysis. `code_aster`, `OpenSees`, `anastruct` and `PyNite` compute displacements
  and forces FROM a model; none of them reads `.dsl.semio`, none defines this document, and not one
  of the twenty-five kinds asks a solver anything — `replace-section` swaps a profile record,
  `add-load` appends to a case's load list, `change-load-case-self-weight` flips a boolean. What a
  reference can genuinely adjudicate is the model algebra over nine id-keyed collections, and that is
  what this one does.

  A DEFECT IN THE SPECIFICATION, found while writing the reference and reported rather than worked
  around. Two of the three schema files here do not say what they claim.
  `…/🧬️schema/🧬️mutations/🔣️.json` is a verbatim copy of the SNAPSHOT schema with `title`
  changed to `Fem2dMutation`. And in the snapshot schema itself, every one of `FemNode`,
  `FemElement`, `FemRegion`, `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase` and
  `FemCombination` is an EMPTY `{"title": …, "type": "object"}` with no properties at all. The record
  shapes the reference implements were read off the twenty-five committed vectors instead, which
  agree with one another on every field.

  The artifact is real. `local://🏗️timber-portal-frame.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem2d-frame.py`
  from the artifact's own committed demo model
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`): a twelve-node
  timber-and-steel portal frame with a ridge at 7.6 m, nine beam elements, four supports, three real
  materials with their real moduli and densities, four sections with their real areas and second
  moments, a first-floor slab, a dead case carrying an area pressure, a live case carrying a nodal
  load and an area pressure, and an ULS combination at 1.35/1.5 — all carried across unchanged.

  What the derivation ADDS, and why. The committed model REFERENCES every entity it holds: every
  material by an element or the slab, every section by an element, every node by an element or a
  support, the slab by two area loads, and both cases by the ULS combination's terms. Deleting a
  referenced entity asks a question no committed document answers, so seven unreferenced spares are
  appended, each taken from a committed specification vector of this same subset and repointed only
  onto ids the model already holds. They are appended LAST, so the committed entities keep their
  indices and every spare is the TRAILING member of its collection — which matters, because no
  `create-` verb in this vocabulary carries an index, so the inverse of a delete is exact only for a
  trailing record. That limit is a property of the closed schema, not of an implementation, and both
  implementations share it; it is caught here only because both sides assert the restoring law IN
  ROLE, index for index.

  One deliberate exception: `delete-node` addresses `n3`, which the spare support `s_spare` points
  at. That is not an oversight — the committed vector for this very kind is named
  `removes-node-n3-without-cascading-to-its-support`, so the non-cascade IS the specified behaviour
  and the row exercises it against a real model rather than a two-node sketch.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the nine
  members. That is the check an after-snapshot comparison cannot make on its own: an implementation
  that re-derived a sibling collection on every edit — renumbering ids, re-sorting sections — would
  still land on the right value for the member it meant to write.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived timber portal frame
    Given the real derived model local://🏗️timber-portal-frame.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same model, and only the member this verb writes moved
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                                                                   |
      | create-node                  | {"mutation":"createNode","node":{"id":"eave_mid","x":4.0,"y":5.6}}                                                                                                                                                                                                                         |
      | delete-node                  | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                                                        |
      | create-element               | {"mutation":"createElement","element":{"kind":"bar","id":"e12","start":"rc0","end":"rc1","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                                       |
      | delete-element               | {"mutation":"deleteElement","id":"e11"}                                                                                                                                                                                                                                                    |
      | replace-element              | {"mutation":"replaceElement","id":"e3","newElement":{"kind":"bar","id":"e3","start":"n1","end":"n2","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                            |
      | create-material              | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7850.0}}                                                                                                                                                                        |
      | delete-material              | {"mutation":"deleteMaterial","id":"c30"}                                                                                                                                                                                                                                                   |
      | replace-material             | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7900.0}}                                                                                                                                                      |
      | create-section               | {"mutation":"createSection","section":{"id":"chs114","name":"CHS 114 Column","area":0.0016,"iy":2.4e-06}}                                                                                                                                                                                  |
      | delete-section               | {"mutation":"deleteSection","id":"ipe300"}                                                                                                                                                                                                                                                 |
      | replace-section              | {"mutation":"replaceSection","id":"chs76","newSection":{"id":"chs76","name":"CHS 76 Foundation Column reinforced","area":0.0014,"iy":1.8e-06}}                                                                                                                                             |
      | create-support               | {"mutation":"createSupport","support":{"id":"s5","nodeId":"rc2","fixed":["Ty"]}}                                                                                                                                                                                                           |
      | delete-support               | {"mutation":"deleteSupport","id":"s_spare"}                                                                                                                                                                                                                                                |
      | replace-support              | {"mutation":"replaceSupport","id":"s1","newSupport":{"id":"s1","nodeId":"n1","fixed":["Tx","Ty","Rz"]}}                                                                                                                                                                                    |
      | create-region                | {"mutation":"createRegion","region":{"id":"roof_slab","name":"Roof Slab","outline":[[0.0,5.6],[8.0,5.6],[8.0,5.7],[0.0,5.7]],"holes":[],"thickness":0.14,"materialId":"concrete","meshSize":0.5}}                                                                                          |
      | delete-region                | {"mutation":"deleteRegion","id":"slab_spare"}                                                                                                                                                                                                                                              |
      | replace-region               | {"mutation":"replaceRegion","id":"r1","newRegion":{"id":"r1","name":"First Floor Slab with stair opening","outline":[[10.0,2.75],[12.0,2.75],[12.0,2.85],[10.0,2.85]],"holes":[[[10.6,2.78],[11.4,2.78],[11.4,2.82],[10.6,2.82]]],"thickness":0.2,"materialId":"concrete","meshSize":1.0}} |
      | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"wind","name":"Wind","loads":[{"kind":"nodal","id":"lw1","nodeId":"p8_l2","dof":"Tx","value":6000.0}],"selfWeight":false}}                                                                                                                   |
      | delete-load-case             | {"mutation":"deleteLoadCase","id":"snow"}                                                                                                                                                                                                                                                  |
      | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l8","elementId":"e8","wx":0.0,"wy":-2400.0}}                                                                                                                                                                        |
      | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l7"}                                                                                                                                                                                                                                    |
      | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                                                                                                                                               |
      | create-combination           | {"mutation":"createCombination","combination":{"id":"sls","name":"SLS characteristic","terms":[{"caseId":"dead","factor":1.0},{"caseId":"live","factor":1.0}]}}                                                                                                                            |
      | delete-combination           | {"mutation":"deleteCombination","id":"uls_spare"}                                                                                                                                                                                                                                          |
      | update-analysis-settings     | {"mutation":"updateAnalysisSettings","settings":{"modalCount":6,"bucklingCount":4,"deformationScale":150.0}}                                                                                                                                                                               |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived frame and land back on it
    Given the real derived model local://🏗️timber-portal-frame.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated model AND on the restored one, member for member and index for index
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                                                                   |
      | create-node                  | {"mutation":"createNode","node":{"id":"eave_mid","x":4.0,"y":5.6}}                                                                                                                                                                                                                         |
      | delete-node                  | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                                                        |
      | create-element               | {"mutation":"createElement","element":{"kind":"bar","id":"e12","start":"rc0","end":"rc1","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                                       |
      | delete-element               | {"mutation":"deleteElement","id":"e11"}                                                                                                                                                                                                                                                    |
      | replace-element              | {"mutation":"replaceElement","id":"e3","newElement":{"kind":"bar","id":"e3","start":"n1","end":"n2","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                            |
      | create-material              | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7850.0}}                                                                                                                                                                        |
      | delete-material              | {"mutation":"deleteMaterial","id":"c30"}                                                                                                                                                                                                                                                   |
      | replace-material             | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7900.0}}                                                                                                                                                      |
      | create-section               | {"mutation":"createSection","section":{"id":"chs114","name":"CHS 114 Column","area":0.0016,"iy":2.4e-06}}                                                                                                                                                                                  |
      | delete-section               | {"mutation":"deleteSection","id":"ipe300"}                                                                                                                                                                                                                                                 |
      | replace-section              | {"mutation":"replaceSection","id":"chs76","newSection":{"id":"chs76","name":"CHS 76 Foundation Column reinforced","area":0.0014,"iy":1.8e-06}}                                                                                                                                             |
      | create-support               | {"mutation":"createSupport","support":{"id":"s5","nodeId":"rc2","fixed":["Ty"]}}                                                                                                                                                                                                           |
      | delete-support               | {"mutation":"deleteSupport","id":"s_spare"}                                                                                                                                                                                                                                                |
      | replace-support              | {"mutation":"replaceSupport","id":"s1","newSupport":{"id":"s1","nodeId":"n1","fixed":["Tx","Ty","Rz"]}}                                                                                                                                                                                    |
      | create-region                | {"mutation":"createRegion","region":{"id":"roof_slab","name":"Roof Slab","outline":[[0.0,5.6],[8.0,5.6],[8.0,5.7],[0.0,5.7]],"holes":[],"thickness":0.14,"materialId":"concrete","meshSize":0.5}}                                                                                          |
      | delete-region                | {"mutation":"deleteRegion","id":"slab_spare"}                                                                                                                                                                                                                                              |
      | replace-region               | {"mutation":"replaceRegion","id":"r1","newRegion":{"id":"r1","name":"First Floor Slab with stair opening","outline":[[10.0,2.75],[12.0,2.75],[12.0,2.85],[10.0,2.85]],"holes":[[[10.6,2.78],[11.4,2.78],[11.4,2.82],[10.6,2.82]]],"thickness":0.2,"materialId":"concrete","meshSize":1.0}} |
      | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"wind","name":"Wind","loads":[{"kind":"nodal","id":"lw1","nodeId":"p8_l2","dof":"Tx","value":6000.0}],"selfWeight":false}}                                                                                                                   |
      | delete-load-case             | {"mutation":"deleteLoadCase","id":"snow"}                                                                                                                                                                                                                                                  |
      | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l8","elementId":"e8","wx":0.0,"wy":-2400.0}}                                                                                                                                                                        |
      | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l7"}                                                                                                                                                                                                                                    |
      | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                                                                                                                                               |
      | create-combination           | {"mutation":"createCombination","combination":{"id":"sls","name":"SLS characteristic","terms":[{"caseId":"dead","factor":1.0},{"caseId":"live","factor":1.0}]}}                                                                                                                            |
      | delete-combination           | {"mutation":"deleteCombination","id":"uls_spare"}                                                                                                                                                                                                                                          |
      | update-analysis-settings     | {"mutation":"updateAnalysisSettings","settings":{"modalCount":6,"bucklingCount":4,"deformationScale":150.0}}                                                                                                                                                                               |

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
      | id                           | dir                           | fixture                                                  |
      | create-node                  | 🌱⚪️create-node                | appends-node-n3                                          |
      | delete-node                  | 🗑⚪️delete-node                | removes-node-n3-without-cascading-to-its-support         |
      | create-element               | 🌱🧩️create-element             | appends-bar-e2-between-n2-and-n3                         |
      | delete-element               | 🗑🧩️delete-element             | removes-bar-e2-and-keeps-its-end-nodes                   |
      | replace-element              | 🔁🧩️replace-element            | converts-beam-e1-into-a-bar-in-place                     |
      | create-material              | 🌱🧱️create-material            | appends-concrete-c30                                     |
      | delete-material              | 🗑🧱️delete-material            | removes-the-unreferenced-timber-material                 |
      | replace-material             | 🔁🧱️replace-material           | restates-steel-as-s355-in-its-original-slot              |
      | create-section               | 🌱create-section               | appends-the-ipe300-profile                               |
      | delete-section               | 🗑📐️delete-section             | removes-the-spare-hollow-section                         |
      | replace-section              | 🔁📐️replace-section            | stiffens-ipe200-with-a-reinforced-profile                |
      | create-support               | 🌱🛡️create-support             | adds-a-vertical-roller-at-node-n2                        |
      | delete-support               | 🗑delete-support               | releases-the-roller-at-node-n2                           |
      | replace-support              | 🔁replace-support              | upgrades-the-roller-at-n2-to-a-full-fixity               |
      | create-region                | 🌱🗺️create-region              | appends-a-solid-rectangular-slab                         |
      | delete-region                | 🗑🗺️delete-region              | removes-the-slab-and-keeps-its-material                  |
      | replace-region               | 🔁🗺️replace-region             | punches-a-stair-opening-through-the-slab                 |
      | create-load-case             | 🌱📋️create-load-case           | appends-a-live-case-carrying-one-nodal-load              |
      | delete-load-case             | 🗑📋️delete-load-case           | removes-the-live-case-together-with-its-loads            |
      | add-load                     | ➕add-load                     | appends-a-member-udl-to-the-dead-case                    |
      | remove-load                  | ➖remove-load                  | strips-the-trailing-member-udl-from-the-dead-case        |
      | change-load-case-self-weight | ⚖change-load-case-self-weight | switches-self-weight-on-for-the-dead-case                |
      | create-combination           | 🌱🔗️create-combination         | appends-an-uls-combination-over-both-cases               |
      | delete-combination           | 🗑🔗️delete-combination         | removes-the-uls-combination-and-keeps-both-cases         |
      | update-analysis-settings     | 🎛update-analysis-settings     | doubles-the-modal-count-and-halves-the-deformation-scale |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived frame in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived model local://🏗️timber-portal-frame.snapshot.json
    And the artifact's own committed carrier asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived model, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same nine members, and the Rust reproduces the committed carrier byte for byte
