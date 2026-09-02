import os

ART = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"

MUTATE_EXAMPLES = [
    ("create-node", '{"mutation":"createNode","node":{"id":"n_roof","x":4.0,"y":5.0,"z":8.4}}'),
    ("delete-node", '{"mutation":"deleteNode","id":"n3"}'),
    ("create-element", '{"mutation":"createElement","element":{"kind":"bar","id":"brace_00","start":"n00_g","end":"n00_l2","materialId":"steel","sectionId":"shs120"}}'),
    ("delete-element", '{"mutation":"deleteElement","id":"fb2_1"}'),
    ("replace-element", '{"mutation":"replaceElement","id":"e1","newElement":{"kind":"frame","id":"e1","start":"n00_g","end":"n00_l1","materialId":"steel","sectionId":"hea200","roll":1.5}}'),
    ("create-material", '{"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"g":80770000000.0,"nu":0.3,"rho":7850.0}}'),
    ("delete-material", '{"mutation":"deleteMaterial","id":"alu"}'),
    ("replace-material", '{"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"g":78000000000.0,"nu":0.3,"rho":7850.0}}'),
    ("create-section", '{"mutation":"createSection","section":{"id":"hea240","name":"HEA 240","area":0.00768,"iy":7.763e-05,"iz":2.769e-05,"j":4.16e-07}}'),
    ("delete-section", '{"mutation":"deleteSection","id":"shs120"}'),
    ("replace-section", '{"mutation":"replaceSection","id":"hea200","newSection":{"id":"hea200","name":"HEA 200 with warping restraint","area":0.00538,"iy":3.69e-05,"iz":1.33e-05,"j":1.8e-06}}'),
    ("create-support", '{"mutation":"createSupport","support":{"id":"s_roof","nodeId":"n00_l2","fixed":["Tz"]}}'),
    ("delete-support", '{"mutation":"deleteSupport","id":"s_spare"}'),
    ("replace-support", '{"mutation":"replaceSupport","id":"s_00","newSupport":{"id":"s_00","nodeId":"n00_g","fixed":["Tx","Ty","Tz"]}}'),
    ("create-solid", '{"mutation":"createSolid","solid":{"id":"sol_roof","name":"Roof Slab","outline":[[0.0,0.0],[8.0,0.0],[8.0,10.0],[0.0,10.0]],"holes":[],"baseZ":5.6,"height":0.22,"layers":2,"meshSize":0.75,"materialId":"concrete"}}'),
    ("delete-solid", '{"mutation":"deleteSolid","id":"sol_spare"}'),
    ("replace-solid", '{"mutation":"replaceSolid","id":"sol1","newSolid":{"id":"sol1","name":"First Floor Slab thickened","outline":[[10.0,0.0],[12.0,0.0],[12.0,2.0],[10.0,2.0]],"holes":[],"baseZ":0.0,"height":0.75,"layers":2,"meshSize":1.0,"materialId":"concrete"}}'),
    ("create-load-case", '{"mutation":"createLoadCase","loadCase":{"id":"snow","name":"Snow","loads":[{"kind":"area","id":"sn1","solidId":"sol1","pressure":900.0}],"selfWeight":false}}'),
    ("delete-load-case", '{"mutation":"deleteLoadCase","id":"wind"}'),
    ("add-load", '{"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l9","elementId":"fb1_0","wx":0.0,"wy":0.0,"wz":-3200.0}}'),
    ("remove-load", '{"mutation":"removeLoad","caseId":"live","loadId":"l3"}'),
    ("change-load-case-self-weight", '{"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}'),
    ("create-combination", '{"mutation":"createCombination","combination":{"id":"acc","name":"Accidental","terms":{"dead":1.0,"live":0.3}}}'),
    ("delete-combination", '{"mutation":"deleteCombination","id":"sls_spare"}'),
    ("update-analysis-settings", '{"mutation":"updateAnalysisSettings","settings":{"modalCount":8,"bucklingCount":5,"deformationScale":120.0}}'),
]

SPEC_VECTORS = [
    ("create-node", "mesh", "🌱⚪️create-node", "appends-the-column-head-node-n3"),
    ("delete-node", "mesh", "🗑⚪️delete-node", "removes-the-column-head-node-under-a-live-frame"),
    ("create-element", "mesh", "🌱🧩️create-element", "appends-a-diagonal-bracing-bar"),
    ("delete-element", "mesh", "🗑🧩️delete-element", "removes-the-bracing-bar-and-leaves-the-frame"),
    ("replace-element", "mesh", "🔁🧩️replace-element", "rolls-the-column-about-its-own-axis"),
    ("create-material", "material", "🌱🧱️create-material", "appends-an-aluminium-alloy"),
    ("delete-material", "material", "🗑🧱️delete-material", "removes-the-unreferenced-aluminium-alloy"),
    ("replace-material", "material", "🔁🧱️replace-material", "softens-the-steel-shear-modulus-in-place"),
    ("create-section", "mesh", "🌱create-section", "appends-a-square-hollow-profile"),
    ("delete-section", "mesh", "🗑📐️delete-section", "removes-the-spare-square-hollow-profile"),
    ("replace-section", "mesh", "🔁📐️replace-section", "raises-the-torsion-constant-of-hea200"),
    ("create-support", "boundary", "🌱🛡️create-support", "clamps-the-column-base-in-all-six-dofs"),
    ("delete-support", "boundary", "🗑delete-support", "releases-the-pinned-node-n2"),
    ("replace-support", "boundary", "🔁🛡️replace-support", "frees-the-three-rotations-at-the-column-base"),
    ("create-solid", "mesh", "🌱🧊️create-solid", "appends-an-extruded-roof-slab"),
    ("delete-solid", "mesh", "🗑🧊️delete-solid", "removes-the-roof-slab-and-keeps-its-material"),
    ("replace-solid", "mesh", "🔁replace-solid", "thickens-the-slab-and-adds-a-mesh-layer"),
    ("create-load-case", "load", "🌱📋️create-load-case", "appends-a-wind-case-pushing-on-the-column-head"),
    ("delete-load-case", "load", "🗑📋️delete-load-case", "removes-the-wind-case-together-with-its-load"),
    ("add-load", "load", "➕add-load", "lays-an-area-pressure-over-the-roof-slab"),
    ("remove-load", "load", "➖remove-load", "drops-the-trailing-member-udl-from-the-dead-case"),
    ("change-load-case-self-weight", "load", "⚖change-load-case-self-weight", "switches-self-weight-off-for-the-dead-case"),
    ("create-combination", "load", "🌱🔗️create-combination", "appends-a-serviceability-combination-keyed-by-case-id"),
    ("delete-combination", "load", "🗑🔗️delete-combination", "removes-the-serviceability-combination-and-keeps-both-cases"),
    ("update-analysis-settings", "analysis", "🎛update-analysis-settings", "doubles-the-buckling-mode-count"),
]

SUBSET_KINDS = {
    "mesh": ["create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-solid", "delete-solid", "replace-solid"],
    "material": ["create-material", "delete-material", "replace-material"],
    "boundary": ["create-support", "delete-support", "replace-support"],
    "load": ["create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight", "create-combination", "delete-combination"],
    "analysis": ["update-analysis-settings"],
}

mutate_map = dict(MUTATE_EXAMPLES)
spec_map = {k: (s, d, f) for k, s, d, f in SPEC_VECTORS}

CASE_NAME = {s: f"mutate-fem3d-1-{s}" for s in SUBSET_KINDS}
DERIVED = "🧊️steel-frame.snapshot.json"

def pad_table(headers, rows):
    all_rows = [headers] + rows
    widths = [max(len(r[i]) for r in all_rows) for i in range(len(headers))]
    lines = []
    for r in all_rows:
        cells = " | ".join(c.ljust(w) for c, w in zip(r, widths))
        lines.append(f"    | {cells} |")
    return "\n".join(lines)

for subset, kinds in SUBSET_KINDS.items():
    case = CASE_NAME[subset]
    case_dir = f"{ART}/✳️{subset}/🧪️tests/{case}"
    os.makedirs(f"{case_dir}/🧫️fixtures", exist_ok=True)

    mutate_rows = [[k, mutate_map[k]] for k in kinds]
    spec_rows = [[k, spec_map[k][1], spec_map[k][2]] for k in kinds]
    mutate_table = pad_table(["id", "mutation"], mutate_rows)
    spec_table = pad_table(["id", "dir", "fixture"], spec_rows)
    kind_list_human = ", ".join(f"`{k}`" for k in kinds)

    feature = f"""@capability-fem3d-1-mutate
@oracle-fem3d-python-independent
@comparison-ordered-json-v1
@mutations-fem3d-1-{subset}
Feature: Apply every typed fem3d {subset} mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem3d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds ({kind_list_human}) have a subset-owned test. The reference is
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
  around, identical to the one its `◻2d` sibling carries. `…/🧬️schema/🧬️mutations/🔣️.json` is a
  verbatim copy of the SNAPSHOT schema with `title` changed to `Fem3dMutation`, and in the snapshot
  schema itself every record `$def` is an EMPTY `{{"title": …, "type": "object"}}`. The record
  shapes were read off the committed vectors instead — including the one that only they state: an
  `element` is a `frame` carrying a `roll` about its own axis OR a `bar` carrying none.

  The artifact is real. `local://{DERIVED}` is the SAME derived steel frame model every fem3d
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
    Given the real derived model local://{DERIVED}
    When the <id> mutation is applied with the parameters the feature states
      \"\"\"
      <mutation>
      \"\"\"
    Then both implementations produce the same model, and only the member this verb writes moved
    Examples:
{mutate_table}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived frame and land back on it
    Given the real derived model local://{DERIVED}
    When the <id> mutation is applied and then its own computed inverse is applied
      \"\"\"
      <mutation>
      \"\"\"
    Then both implementations agree on the mutated model AND on the restored one, member for member and index for index
    Examples:
{mutate_table}

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
{spec_table}
"""
    with open(f"{case_dir}/🥒️.feature", "w", encoding="utf-8") as f:
        f.write(feature)
    print("wrote", f"{case_dir}/🥒️.feature")
