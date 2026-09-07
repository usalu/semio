@capability-fem2d-1-mutate
@oracle-fem2d-python-independent
@comparison-ordered-json-v1
@mutations-fem2d-1-mesh
Feature: Apply every typed fem2d mesh mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem2d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds (`create-node`, `delete-node`, `create-element`, `delete-element`, `replace-element`, `create-section`, `delete-section`, `replace-section`, `create-region`, `delete-region`, `replace-region`) have a subset-owned test. The reference is
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

  The artifact is real. `local://🏗️timber-portal-frame.snapshot.json` is the SAME derived timber-portal-frame model every
  fem2d mutation subset case shares — a twelve-node timber-and-steel portal frame with a ridge at
  7.6 m, derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem2d-frame.py`
  from the artifact's own committed demo model, with seven unreferenced spares appended so every
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
    Given the real derived model local://🏗️timber-portal-frame.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same model, and only the member this verb writes moved
    Examples:
    | id              | mutation                                                                                                                                                                                                                                                                                   |
    | create-node     | {"mutation":"createNode","node":{"id":"eave_mid","x":4.0,"y":5.6}}                                                                                                                                                                                                                         |
    | delete-node     | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                                                        |
    | create-element  | {"mutation":"createElement","element":{"kind":"bar","id":"e12","start":"rc0","end":"rc1","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                                       |
    | delete-element  | {"mutation":"deleteElement","id":"e11"}                                                                                                                                                                                                                                                    |
    | replace-element | {"mutation":"replaceElement","id":"e3","newElement":{"kind":"bar","id":"e3","start":"n1","end":"n2","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                            |
    | create-section  | {"mutation":"createSection","section":{"id":"chs114","name":"CHS 114 Column","area":0.0016,"iy":2.4e-06}}                                                                                                                                                                                  |
    | delete-section  | {"mutation":"deleteSection","id":"ipe300"}                                                                                                                                                                                                                                                 |
    | replace-section | {"mutation":"replaceSection","id":"chs76","newSection":{"id":"chs76","name":"CHS 76 Foundation Column reinforced","area":0.0014,"iy":1.8e-06}}                                                                                                                                             |
    | create-region   | {"mutation":"createRegion","region":{"id":"roof_slab","name":"Roof Slab","outline":[[0.0,5.6],[8.0,5.6],[8.0,5.7],[0.0,5.7]],"holes":[],"thickness":0.14,"materialId":"concrete","meshSize":0.5}}                                                                                          |
    | delete-region   | {"mutation":"deleteRegion","id":"slab_spare"}                                                                                                                                                                                                                                              |
    | replace-region  | {"mutation":"replaceRegion","id":"r1","newRegion":{"id":"r1","name":"First Floor Slab with stair opening","outline":[[10.0,2.75],[12.0,2.75],[12.0,2.85],[10.0,2.85]],"holes":[[[10.6,2.78],[11.4,2.78],[11.4,2.82],[10.6,2.82]]],"thickness":0.2,"materialId":"concrete","meshSize":1.0}} |

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
    | id              | mutation                                                                                                                                                                                                                                                                                   |
    | create-node     | {"mutation":"createNode","node":{"id":"eave_mid","x":4.0,"y":5.6}}                                                                                                                                                                                                                         |
    | delete-node     | {"mutation":"deleteNode","id":"n3"}                                                                                                                                                                                                                                                        |
    | create-element  | {"mutation":"createElement","element":{"kind":"bar","id":"e12","start":"rc0","end":"rc1","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                                       |
    | delete-element  | {"mutation":"deleteElement","id":"e11"}                                                                                                                                                                                                                                                    |
    | replace-element | {"mutation":"replaceElement","id":"e3","newElement":{"kind":"bar","id":"e3","start":"n1","end":"n2","materialId":"steel","sectionId":"ipe300"}}                                                                                                                                            |
    | create-section  | {"mutation":"createSection","section":{"id":"chs114","name":"CHS 114 Column","area":0.0016,"iy":2.4e-06}}                                                                                                                                                                                  |
    | delete-section  | {"mutation":"deleteSection","id":"ipe300"}                                                                                                                                                                                                                                                 |
    | replace-section | {"mutation":"replaceSection","id":"chs76","newSection":{"id":"chs76","name":"CHS 76 Foundation Column reinforced","area":0.0014,"iy":1.8e-06}}                                                                                                                                             |
    | create-region   | {"mutation":"createRegion","region":{"id":"roof_slab","name":"Roof Slab","outline":[[0.0,5.6],[8.0,5.6],[8.0,5.7],[0.0,5.7]],"holes":[],"thickness":0.14,"materialId":"concrete","meshSize":0.5}}                                                                                          |
    | delete-region   | {"mutation":"deleteRegion","id":"slab_spare"}                                                                                                                                                                                                                                              |
    | replace-region  | {"mutation":"replaceRegion","id":"r1","newRegion":{"id":"r1","name":"First Floor Slab with stair opening","outline":[[10.0,2.75],[12.0,2.75],[12.0,2.85],[10.0,2.85]],"holes":[[[10.6,2.78],[11.4,2.78],[11.4,2.82],[10.6,2.82]]],"thickness":0.2,"materialId":"concrete","meshSize":1.0}} |

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
    | id              | dir               | fixture                          |
    | create-node     | ⚪️create-node     | 📍️appends-node-n3                |
    | delete-node     | 🕳️delete-node     | 🚫️removes-node-n3-without-6eab3f |
    | create-element  | 🧩️create-element  | ➖️appends-bar-e2-between-fc1c09  |
    | delete-element  | 🗑️delete-element  | 🚫️removes-bar-e2-and-3c0260      |
    | replace-element | ♻️replace-element | ♻️converts-beam-e1-into-a-5d21f5 |
    | create-section  | 📐️create-section  | 📐️appends-the-ipe300-profile     |
    | delete-section  | ✂️delete-section  | 🚫️removes-the-spare-1c235a       |
    | replace-section | 📏️replace-section | 💪️stiffens-ipe200-with-5e9c08    |
    | create-region   | 🗺️create-region   | 🧱️appends-a-solid-d78275         |
    | delete-region   | 🚫️delete-region   | 🚫️removes-the-slab-and-5b301a    |
    | replace-region  | 🔄️replace-region  | 🪜️punches-a-stair-f7b3b1         |

  @id-frame-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> steel-frame specification vector through both implementations
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree
    Examples:
    | id              | dir               | fixture                     |
    | create-element  | 🧩️create-element  | 📐️braces-the-upper-d96634   |
    | create-node     | ⚪️create-node     | 🏢️appends-the-canopy-226a20 |
    | create-region   | 🗺️create-region   | 🏢️infills-the-upper-6cc520  |
    | create-section  | 📐️create-section  | ➕️adds-the-hea220-dfdf34    |
    | delete-element  | 🗑️delete-element  | ✂️cuts-the-lower-e4a250     |
    | delete-node     | 🕳️delete-node     | 🗑️drops-the-spare-6c285d    |
    | delete-region   | 🚫️delete-region   | 🧹️drops-the-spare-460714    |
    | delete-section  | ✂️delete-section  | ✂️drops-the-spare-dcf609    |
    | replace-element | ♻️replace-element | 🔧️regrades-the-roof-fb20eb  |
    | replace-region  | 🔄️replace-region  | 🪟️widens-the-window-09a8ec  |
    | replace-section | 📏️replace-section | 🛠️thickens-the-chs-e235a5   |

  @id-reject
  @level-exhaustive
  @mode-differential
  Scenario Outline: Refuse the committed <id> vector in both implementations, for the same reason
    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When the committed mutation is applied to the committed before-model
    Then both implementations leave the document exactly where it was and report the same diagnostic code, level and address
    Examples:
    | id                | dir               | fixture                      |
    | create-element-1  | 🧩️create-element  | 🚫️rejects-a-dangling-b9e64c  |
    | create-node-1     | ⚪️create-node     | 🚫️rejects-a-duplicate-eb0df0 |
    | create-region-1   | 🗺️create-region   | 🚫️rejects-a-duplicate-11ca0d |
    | create-region-2   | 🗺️create-region   | 📐️denies-two-point-99954a    |
    | create-region-3   | 🗺️create-region   | 🕳️denies-loose-hole-d9efa1   |
    | create-section-1  | 📐️create-section  | 🚫️rejects-a-duplicate-e91bc7 |
    | create-section-2  | 📐️create-section  | ⚗️denies-zero-area-58b5ca    |
    | delete-element-1  | 🗑️delete-element  | ⛔️rejects-a-missing-611215   |
    | delete-element-2  | 🗑️delete-element  | 🔗️blocks-udl-a1df8e          |
    | delete-node-1     | 🕳️delete-node     | ⛔️rejects-a-missing-429801   |
    | delete-region-1   | 🚫️delete-region   | ⛔️rejects-a-missing-a83a6d   |
    | delete-region-2   | 🚫️delete-region   | 🔗️blocks-in-use-7c7862       |
    | delete-section-1  | ✂️delete-section  | ⛔️rejects-a-missing-dbd0a4   |
    | delete-section-2  | ✂️delete-section  | 🔗️blocks-in-use-0a6a3c       |
    | replace-element-1 | ♻️replace-element | ⛔️rejects-a-missing-bd448c   |
    | replace-element-2 | ♻️replace-element | 🪪️denies-rename-0d46d8       |
    | replace-element-3 | ♻️replace-element | 🚫️dangling-start-cda887      |
    | replace-region-1  | 🔄️replace-region  | ⛔️rejects-a-missing-6e0d70   |
    | replace-region-2  | 🔄️replace-region  | 🪪️denies-rename-574c91       |
    | replace-region-3  | 🔄️replace-region  | 👻️dangling-mat-7ef81b        |
    | replace-region-4  | 🔄️replace-region  | 📐️denies-zero-thick-7d805e   |
    | replace-section-1 | 📏️replace-section | ⛔️rejects-a-missing-b468f4   |
    | replace-section-2 | 📏️replace-section | 🪪️denies-rename-1d02dd       |
    | replace-section-3 | 📏️replace-section | ⚗️denies-zero-iy-404e31      |
