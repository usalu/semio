@capability-fem3d-1-mutate
@oracle-fem3d-python-independent
@comparison-ordered-json-v1
@mutations-fem3d-1-any-material
Feature: Apply every typed fem3d material mutation twice — once in Rust, once in Python — and require the same answer
  🧩️ Duplicated (relative paths adjusted, the extra spec-vector-replay Outline dropped — its committed-fixture references only resolve from the real owning subset, which the escape guard blocks a ✳️any-owned case from reaching sideways into) from `../../../✳️material/🧪️tests/mutate-fem3d-1-material/` by shard F4 (this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` owner — same mechanism E3 already proved on `sequence`: reuse the already-manifested `fem3d-1-mutate` capability, no new v2 manifest entry or runtime-inventory coordinate. The dropped Outline's own replay evidence stays intact, undiminished, at the original subset-owned case above — this duplicate only needs to satisfy the coverage gate's mutate-<kind>/inverse-<kind> requirement.


  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem3d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds (`create-material`, `delete-material`, `replace-material`) have a subset-owned test. The reference is
  `🐍️.py` in this directory: a second implementation of the `s.fem.fem3d` structural model and
  this subset's typed mutations, written in Python from
  `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json` (the nine members, `additionalProperties: false`),
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
  `../../../✳️any/🧪️tests/round-trips-the-committed-document/🥒️.feature` for the full derivation
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
    | id               | mutation                                                                                                                                                |
    | create-material  | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"g":80770000000.0,"nu":0.3,"rho":7850.0}}                   |
    | delete-material  | {"mutation":"deleteMaterial","id":"alu"}                                                                                                                |
    | replace-material | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"g":78000000000.0,"nu":0.3,"rho":7850.0}} |

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
    | id               | mutation                                                                                                                                                |
    | create-material  | {"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"g":80770000000.0,"nu":0.3,"rho":7850.0}}                   |
    | delete-material  | {"mutation":"deleteMaterial","id":"alu"}                                                                                                                |
    | replace-material | {"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"g":78000000000.0,"nu":0.3,"rho":7850.0}} |
