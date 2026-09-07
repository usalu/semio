@capability-fem3d-1-mutate
@oracle-fem3d-python-independent
@comparison-ordered-json-v1
@mutations-fem3d-1-mesh
Feature: Apply every typed fem3d mesh mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem3d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds (`create-node`, `delete-node`, `create-element`, `delete-element`, `replace-element`, `create-section`, `delete-section`, `replace-section`, `create-solid`, `delete-solid`, `replace-solid`) have a subset-owned test. The reference is
  `🐍️.py` in this directory: a second implementation of the `s.fem.fem3d` structural model and
  this subset's typed mutations, written in Python from
  `../../../🌐️any/🧬️schema/📸️snapshot/🔣️.json` (the nine members, `additionalProperties: false`),
  from `…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio` and from the committed specification
  vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. What this vocabulary edits is the
  MODEL, not the analysis. `code_aster`, `OpenSees`, `anastruct` and `PyNite` compute displacements
  and forces FROM a model; none reads `.dsl.semio`, none defines this document. What a reference can
  genuinely adjudicate is the model algebra, and that is what this one does.

  A DEFECT IN THE SPECIFICATION, found while writing the reference and reported rather than worked
  around, identical to the one its `◻️2d` sibling carries. `…/🧬️schema/🧬️mutations/🔣️.json` is a
  verbatim copy of the SNAPSHOT schema with `title` changed to `Fem3dMutation`, and in the snapshot
  schema itself every record `$def` is an EMPTY `{"title": …, "type": "object"}`. The record
  shapes were read off the committed vectors instead — including the one that only they state: an
  `element` is a `frame` carrying a `roll` about its own axis OR a `bar` carrying none.

  The artifact is real. `local://🧊️steel-frame.snapshot.json` is the SAME derived steel frame model every fem3d
  mutation subset case shares — a sixteen-node, two-storey steel frame on an 8 × 10 m grid, derived
  ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem3d-frame.py`
  from the artifact's own committed demo model, with six unreferenced spares appended so every
  `delete-` and `replace-` verb this vocabulary declares has an unambiguous trailing target — see
  `../../../🌐️any/🧪️tests/🔄️round-trips-the-committed-document/🥒️.feature` for the full derivation
  provenance. No `create-` verb in this vocabulary carries an index, so the inverse of a delete is
  exact only for a trailing record; that limit is a property of the closed schema, not of an
  implementation, and both implementations share it.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the nine
  members. That is the check an after-snapshot comparison cannot make on its own: an implementation
  that re-derived a sibling collection on every edit — renumbering ids, re-sorting sections — would
  still land on the right value for the member it meant to write.

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
    | id              | mutation                                                                                                                                                                                                                                            |
    | create-node     | {"mutation":"createNode","node":{"id":"n_roof","x":4.0,"y":5.0,"z":8.4}}                                                                                                                                                                            |
    | delete-node     | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                 |
    | create-element  | {"mutation":"createElement","element":{"kind":"bar","id":"brace_00","start":"n00_g","end":"n00_l2","materialId":"steel","sectionId":"shs120"}}                                                                                                      |
    | delete-element  | {"mutation":"deleteElement","id":"fb2_1"}                                                                                                                                                                                                           |
    | replace-element | {"mutation":"replaceElement","id":"e1","newElement":{"kind":"frame","id":"e1","start":"n00_g","end":"n00_l1","materialId":"steel","sectionId":"hea200","roll":1.5}}                                                                                 |
    | create-section  | {"mutation":"createSection","section":{"id":"hea240","name":"HEA 240","area":0.00768,"iy":7.763e-05,"iz":2.769e-05,"j":4.16e-07}}                                                                                                                   |
    | delete-section  | {"mutation":"deleteSection","id":"shs120"}                                                                                                                                                                                                          |
    | replace-section | {"mutation":"replaceSection","id":"hea200","newSection":{"id":"hea200","name":"HEA 200 with warping restraint","area":0.00538,"iy":3.69e-05,"iz":1.33e-05,"j":1.8e-06}}                                                                             |
    | create-solid    | {"mutation":"createSolid","solid":{"id":"sol_roof","name":"Roof Slab","outline":[[0.0,0.0],[8.0,0.0],[8.0,10.0],[0.0,10.0]],"holes":[],"baseZ":5.6,"height":0.22,"layers":2,"meshSize":0.75,"materialId":"concrete"}}                               |
    | delete-solid    | {"mutation":"deleteSolid","id":"sol_spare"}                                                                                                                                                                                                         |
    | replace-solid   | {"mutation":"replaceSolid","id":"sol1","newSolid":{"id":"sol1","name":"First Floor Slab thickened","outline":[[10.0,0.0],[12.0,0.0],[12.0,2.0],[10.0,2.0]],"holes":[],"baseZ":0.0,"height":0.75,"layers":2,"meshSize":1.0,"materialId":"concrete"}} |

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
    | id              | mutation                                                                                                                                                                                                                                            |
    | create-node     | {"mutation":"createNode","node":{"id":"n_roof","x":4.0,"y":5.0,"z":8.4}}                                                                                                                                                                            |
    | delete-node     | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                 |
    | create-element  | {"mutation":"createElement","element":{"kind":"bar","id":"brace_00","start":"n00_g","end":"n00_l2","materialId":"steel","sectionId":"shs120"}}                                                                                                      |
    | delete-element  | {"mutation":"deleteElement","id":"fb2_1"}                                                                                                                                                                                                           |
    | replace-element | {"mutation":"replaceElement","id":"e1","newElement":{"kind":"frame","id":"e1","start":"n00_g","end":"n00_l1","materialId":"steel","sectionId":"hea200","roll":1.5}}                                                                                 |
    | create-section  | {"mutation":"createSection","section":{"id":"hea240","name":"HEA 240","area":0.00768,"iy":7.763e-05,"iz":2.769e-05,"j":4.16e-07}}                                                                                                                   |
    | delete-section  | {"mutation":"deleteSection","id":"shs120"}                                                                                                                                                                                                          |
    | replace-section | {"mutation":"replaceSection","id":"hea200","newSection":{"id":"hea200","name":"HEA 200 with warping restraint","area":0.00538,"iy":3.69e-05,"iz":1.33e-05,"j":1.8e-06}}                                                                             |
    | create-solid    | {"mutation":"createSolid","solid":{"id":"sol_roof","name":"Roof Slab","outline":[[0.0,0.0],[8.0,0.0],[8.0,10.0],[0.0,10.0]],"holes":[],"baseZ":5.6,"height":0.22,"layers":2,"meshSize":0.75,"materialId":"concrete"}}                               |
    | delete-solid    | {"mutation":"deleteSolid","id":"sol_spare"}                                                                                                                                                                                                         |
    | replace-solid   | {"mutation":"replaceSolid","id":"sol1","newSolid":{"id":"sol1","name":"First Floor Slab thickened","outline":[[10.0,0.0],[12.0,0.0],[12.0,2.0],[10.0,2.0]],"holes":[],"baseZ":0.0,"height":0.75,"layers":2,"meshSize":1.0,"materialId":"concrete"}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
    | id              | dir               | fixture                           |
    | create-node     | ⚪️create-node     | 📍️appends-the-column-head-node-n3 |
    | delete-node     | 🕳️delete-node     | 🚫️removes-the-column-head-056295  |
    | create-element  | 🧩️create-element  | ➖️appends-a-diagonal-bracing-bar  |
    | delete-element  | 🗑️delete-element  | 🚫️removes-the-bracing-be89d2      |
    | replace-element | ♻️replace-element | 🔄️rolls-the-column-50f732         |
    | create-section  | 📐️create-section  | 🔳️appends-a-square-bd0e4e         |
    | delete-section  | ✂️delete-section  | 🚫️removes-the-spare-30ecfb        |
    | replace-section | 📏️replace-section | 🌀️raises-the-torsion-296ef0       |
    | create-solid    | 🧊️create-solid    | 🏠️appends-an-extruded-roof-slab   |
    | delete-solid    | 🚫️delete-solid    | 🚫️removes-the-roof-slab-f0fb64    |
    | replace-solid   | 🔄️replace-solid   | 📚️thickens-the-slab-and-b51ef0    |

  @id-hall-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> glulam-hall vector through both implementations
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
    | id              | dir               | fixture                   |
    | replace-element | ♻️replace-element | 🏗️hall-strut-d0e4b7       |
    | create-node     | ⚪️create-node     | 🏗️hall-new-node-b26700    |
    | delete-section  | ✂️delete-section  | 🏗️hall-cut-shs-d44b9e     |
    | replace-section | 📏️replace-section | 🏗️hall-deep-purlin-176fd0 |
    | create-section  | 📐️create-section  | 🏗️hall-new-beam-251a92    |
    | replace-solid   | 🔄️replace-solid   | 🏗️hall-thick-raft-cddc0f  |
    | delete-node     | 🕳️delete-node     | 🏗️hall-cut-node-8350fd    |
    | delete-element  | 🗑️delete-element  | 🏗️hall-cut-tie-c268d4     |
    | delete-solid    | 🚫️delete-solid    | 🏗️hall-cut-apron-6c79d3   |
    | create-solid    | 🧊️create-solid    | 🏗️hall-new-slab-d79da4    |
    | create-element  | 🧩️create-element  | 🏗️hall-new-tie-074a69     |

  @id-reject
  @level-exhaustive
  @mode-differential
  Scenario Outline: Refuse the committed <id> vector in both implementations and leave the model where it was
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then both implementations refuse it, or declare it a no-op, and leave the committed before-model exactly as it was
    Examples:
    | id                      | dir               | fixture                   |
    | same-element-61adb2     | ♻️replace-element | ⏸️same-element-61adb2     |
    | dangling-sec-70b168     | ♻️replace-element | 🚨️dangling-sec-70b168     |
    | renames-brace-219be2    | ♻️replace-element | 🪪️renames-brace-219be2    |
    | dup-node-id-86f2e1      | ⚪️create-node     | 🚨️dup-node-id-86f2e1      |
    | purlin-in-use-99eb01    | ✂️delete-section  | ⛓️purlin-in-use-99eb01    |
    | no-such-section-50d29b  | ✂️delete-section  | 🚨️no-such-section-50d29b  |
    | same-section-d1d013     | 📏️replace-section | ⏸️same-section-d1d013     |
    | negative-iy-d4e0a8      | 📏️replace-section | 🧨️negative-iy-d4e0a8      |
    | renames-purlin-dfe160   | 📏️replace-section | 🪪️renames-purlin-dfe160   |
    | dup-section-id-a76686   | 📐️create-section  | 🚨️dup-section-id-a76686   |
    | zero-area-475a19        | 📐️create-section  | 🧨️zero-area-475a19        |
    | same-solid-8ad12c       | 🔄️replace-solid   | ⏸️same-solid-8ad12c       |
    | zero-height-2b131a      | 🔄️replace-solid   | 📐️zero-height-2b131a      |
    | dangling-mat-9c89da     | 🔄️replace-solid   | 🚨️dangling-mat-9c89da     |
    | renames-apron-7bfadd    | 🔄️replace-solid   | 🪪️renames-apron-7bfadd    |
    | no-such-node-4027a8     | 🕳️delete-node     | 🚨️no-such-node-4027a8     |
    | rafter-under-udl-e0342d | 🗑️delete-element  | ⛓️rafter-under-udl-e0342d |
    | no-such-element-eb788c  | 🗑️delete-element  | 🚨️no-such-element-eb788c  |
    | raft-under-load-e4ea39  | 🚫️delete-solid    | ⛓️raft-under-load-e4ea39  |
    | no-such-solid-f08d23    | 🚫️delete-solid    | 🚨️no-such-solid-f08d23    |
    | sliver-outline-316a7c   | 🧊️create-solid    | 📐️sliver-outline-316a7c   |
    | dangling-mat-1ebd78     | 🧊️create-solid    | 🚨️dangling-mat-1ebd78     |
    | dangling-start-ab4132   | 🧩️create-element  | 🚨️dangling-start-ab4132   |
