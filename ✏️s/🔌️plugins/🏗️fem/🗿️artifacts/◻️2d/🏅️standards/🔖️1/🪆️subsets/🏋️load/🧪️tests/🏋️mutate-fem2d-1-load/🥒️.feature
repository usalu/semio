@capability-fem2d-1-mutate
@oracle-fem2d-python-independent
@comparison-ordered-json-v1
@mutations-fem2d-1-load
Feature: Apply every typed fem2d load mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem2d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds (`create-load-case`, `delete-load-case`, `add-load`, `remove-load`, `change-load-case-self-weight`, `create-combination`, `delete-combination`) have a subset-owned test. The reference is
  `🐍️.py` in this directory: a second implementation of the `s.fem.fem2d` structural model and
  this subset's typed mutations, written in Python from
  `../../../🌐️any/🧬️schema/📸️snapshot/🔣️.json` (the nine members, `additionalProperties: false`),
  from `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and from the committed specification
  vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. What this vocabulary edits is the
  MODEL, not the analysis. `code_aster`, `OpenSees`, `anastruct` and `PyNite` compute displacements
  and forces FROM a model; none of them reads `.dsl.semio`, none defines this document. What a
  reference can genuinely adjudicate is the model algebra, and that is what this one does.

  A DEFECT IN THE SPECIFICATION, found while writing the reference and reported rather than worked
  around. Two of the three schema files here do not say what they claim.
  `…/🧬️schema/🧬️mutations/🔣️.json` is a verbatim copy of the SNAPSHOT schema with `title`
  changed to `Fem2dMutation`. And in the snapshot schema itself, every one of the nine record types
  is an EMPTY `{"title": …, "type": "object"}` with no properties at all. The record shapes the
  reference implements were read off the committed vectors instead, which agree with one another on
  every field.

  The artifact is real. `shared://🏋️mutate-fem2d-1-load/🏗️timber-portal-frame.snapshot.json` is the SAME derived timber-portal-frame model every
  fem2d mutation subset case shares — a twelve-node timber-and-steel portal frame with a ridge at
  7.6 m, derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem2d-frame.py`
  from the artifact's own committed demo model, with seven unreferenced spares appended so every
  `delete-` and `replace-` verb this vocabulary declares has an unambiguous trailing target — see
  `../../../🌐️any/🔄️round-trips-the-committed-document/🥒️.feature` for the full derivation
  provenance. No `create-` verb in this vocabulary carries an index, so the inverse of a delete is
  exact only for a trailing record; that limit is a property of the closed schema, not of an
  implementation, and both implementations share it.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the nine
  members. That is the check an after-snapshot comparison cannot make on its own: an implementation
  that re-derived a sibling collection on every edit — renumbering ids, re-sorting sections — would
  still land on the right value for the member it meant to write.


  THE WHOLE CORPUS, NOT ONE ROW PER KIND (ticket `26/09/06/FEM-PLUGIN-END-TO-END`). Case discovery
  is explicit in all three languages, so until this wave the two `Scenario Outline`s below did not
  exist and every committed vector but one per kind was Rust-only evidence. `frame-vector-<kind>`
  replays the second happy path — the two-storey braced steel frame — and `reject-<kind>-<n>`
  replays every refusal and no-op the kind declares, holding BOTH implementations to the same
  diagnostic code, level and address rather than merely to an unchanged document.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived timber portal frame
    Given the real derived model shared://🏋️mutate-fem2d-1-load/🏗️timber-portal-frame.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same model, and only the member this verb writes moved
    Examples:
    | id                           | mutation                                                                                                                                                                 |
    | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"wind","name":"Wind","loads":[{"kind":"nodal","id":"lw1","nodeId":"p8_l2","dof":"Tx","value":6000.0}],"selfWeight":false}} |
    | delete-load-case             | {"mutation":"deleteLoadCase","id":"snow"}                                                                                                                                |
    | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l8","elementId":"e8","wx":0.0,"wy":-2400.0}}                                                      |
    | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l7"}                                                                                                                  |
    | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                             |
    | create-combination           | {"mutation":"createCombination","combination":{"id":"sls","name":"SLS characteristic","terms":[{"caseId":"dead","factor":1.0},{"caseId":"live","factor":1.0}]}}          |
    | delete-combination           | {"mutation":"deleteCombination","id":"uls_spare"}                                                                                                                        |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived frame and land back on it
    Given the real derived model shared://🏋️mutate-fem2d-1-load/🏗️timber-portal-frame.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated model AND on the restored one, member for member and index for index
    Examples:
    | id                           | mutation                                                                                                                                                                 |
    | create-load-case             | {"mutation":"createLoadCase","loadCase":{"id":"wind","name":"Wind","loads":[{"kind":"nodal","id":"lw1","nodeId":"p8_l2","dof":"Tx","value":6000.0}],"selfWeight":false}} |
    | delete-load-case             | {"mutation":"deleteLoadCase","id":"snow"}                                                                                                                                |
    | add-load                     | {"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l8","elementId":"e8","wx":0.0,"wy":-2400.0}}                                                      |
    | remove-load                  | {"mutation":"removeLoad","caseId":"live","loadId":"l7"}                                                                                                                  |
    | change-load-case-self-weight | {"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}                                                                                             |
    | create-combination           | {"mutation":"createCombination","combination":{"id":"sls","name":"SLS characteristic","terms":[{"caseId":"dead","factor":1.0},{"caseId":"live","factor":1.0}]}}          |
    | delete-combination           | {"mutation":"deleteCombination","id":"uls_spare"}                                                                                                                        |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
    | id                           | dir                            | fixture                                 |
    | create-load-case             | 📋️create-load-case             | 📍️appends-a-live-case-59118a            |
    | delete-load-case             | 🗑️delete-load-case             | 🚫️removes-the-live-06415d               |
    | add-load                     | ➕️add-load                     | 📏️appends-a-member-udl-to-the-dead-case |
    | remove-load                  | ➖️remove-load                  | ➖️strips-the-trailing-member-133914     |
    | change-load-case-self-weight | ⚖️change-load-case-self-weight | ⚖️switches-self-abbff2                  |
    | create-combination           | 🔗️create-combination           | 🔗️appends-an-uls-0c18bb                 |
    | delete-combination           | ✂️delete-combination           | ✂️removes-the-uls-438c0c                |

  @id-frame-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> steel-frame specification vector through both implementations
    Given the committed before-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
    | id                           | dir                            | fixture                      |
    | add-load                     | ➕️add-load                     | 💨️pushes-a-wind-load-5c3f1e  |
    | change-load-case-self-weight | ⚖️change-load-case-self-weight | 🏋️switches-self-5977a5       |
    | create-combination           | 🔗️create-combination           | ➕️appends-the-6-10a-eefe01   |
    | create-load-case             | 📋️create-load-case             | ❄️appends-the-snow-4c007c    |
    | delete-combination           | ✂️delete-combination           | 🗑️drops-the-spare-60fda7     |
    | delete-load-case             | 🗑️delete-load-case             | 🗑️drops-the-spare-49435f     |
    | remove-load                  | ➖️remove-load                  | ✂️strips-the-roof-udl-0c1b3c |

  @id-reject
  @level-exhaustive
  @mode-differential
  Scenario Outline: Refuse the committed <id> vector in both implementations, for the same reason
    Given the committed before-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then both implementations leave the document exactly where it was and report the same diagnostic code, level and address
    Examples:
    | id                             | dir                            | fixture                     |
    | add-load-1                     | ➕️add-load                     | 🚫️rejects-a-missing-4271bc  |
    | add-load-2                     | ➕️add-load                     | 👻️dangling-node-8d113b      |
    | change-load-case-self-weight-1 | ⚖️change-load-case-self-weight | 🔁️keeps-self-weight-ff696b  |
    | create-combination-1           | 🔗️create-combination           | 🚫️rejects-a-dangling-2aa3ea |
    | create-load-case-1             | 📋️create-load-case             | 🚫️rejects-a-dangling-4904f4 |
    | delete-combination-1           | ✂️delete-combination           | ⛔️rejects-a-missing-d4bc03  |
    | delete-combination-2           | ✂️delete-combination           | 🔗️blocks-in-use-0b898b      |
    | delete-load-case-1             | 🗑️delete-load-case             | ⛔️rejects-a-missing-79ed15  |
    | delete-load-case-2             | 🗑️delete-load-case             | 🔗️blocks-in-use-7cdc5f      |
    | remove-load-1                  | ➖️remove-load                  | ⛔️rejects-a-missing-1a8a80  |
