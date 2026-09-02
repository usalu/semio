import os

ART = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets"

MUTATE_EXAMPLES = [
    ("create-node", '{"mutation":"createNode","node":{"id":"eave_mid","x":4.0,"y":5.6}}'),
    ("delete-node", '{"mutation":"deleteNode","id":"n3"}'),
    ("create-element", '{"mutation":"createElement","element":{"kind":"bar","id":"e12","start":"rc0","end":"rc1","materialId":"steel","sectionId":"ipe300"}}'),
    ("delete-element", '{"mutation":"deleteElement","id":"e11"}'),
    ("replace-element", '{"mutation":"replaceElement","id":"e3","newElement":{"kind":"bar","id":"e3","start":"n1","end":"n2","materialId":"steel","sectionId":"ipe300"}}'),
    ("create-material", '{"mutation":"createMaterial","material":{"id":"s355","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7850.0}}'),
    ("delete-material", '{"mutation":"deleteMaterial","id":"c30"}'),
    ("replace-material", '{"mutation":"replaceMaterial","id":"steel","newMaterial":{"id":"steel","name":"Steel S355","e":210000000000.0,"nu":0.3,"rho":7900.0}}'),
    ("create-section", '{"mutation":"createSection","section":{"id":"chs114","name":"CHS 114 Column","area":0.0016,"iy":2.4e-06}}'),
    ("delete-section", '{"mutation":"deleteSection","id":"ipe300"}'),
    ("replace-section", '{"mutation":"replaceSection","id":"chs76","newSection":{"id":"chs76","name":"CHS 76 Foundation Column reinforced","area":0.0014,"iy":1.8e-06}}'),
    ("create-support", '{"mutation":"createSupport","support":{"id":"s5","nodeId":"rc2","fixed":["Ty"]}}'),
    ("delete-support", '{"mutation":"deleteSupport","id":"s_spare"}'),
    ("replace-support", '{"mutation":"replaceSupport","id":"s1","newSupport":{"id":"s1","nodeId":"n1","fixed":["Tx","Ty","Rz"]}}'),
    ("create-region", '{"mutation":"createRegion","region":{"id":"roof_slab","name":"Roof Slab","outline":[[0.0,5.6],[8.0,5.6],[8.0,5.7],[0.0,5.7]],"holes":[],"thickness":0.14,"materialId":"concrete","meshSize":0.5}}'),
    ("delete-region", '{"mutation":"deleteRegion","id":"slab_spare"}'),
    ("replace-region", '{"mutation":"replaceRegion","id":"r1","newRegion":{"id":"r1","name":"First Floor Slab with stair opening","outline":[[10.0,2.75],[12.0,2.75],[12.0,2.85],[10.0,2.85]],"holes":[[[10.6,2.78],[11.4,2.78],[11.4,2.82],[10.6,2.82]]],"thickness":0.2,"materialId":"concrete","meshSize":1.0}}'),
    ("create-load-case", '{"mutation":"createLoadCase","loadCase":{"id":"wind","name":"Wind","loads":[{"kind":"nodal","id":"lw1","nodeId":"p8_l2","dof":"Tx","value":6000.0}],"selfWeight":false}}'),
    ("delete-load-case", '{"mutation":"deleteLoadCase","id":"snow"}'),
    ("add-load", '{"mutation":"addLoad","caseId":"dead","load":{"kind":"memberUdl","id":"l8","elementId":"e8","wx":0.0,"wy":-2400.0}}'),
    ("remove-load", '{"mutation":"removeLoad","caseId":"live","loadId":"l7"}'),
    ("change-load-case-self-weight", '{"mutation":"changeLoadCaseSelfWeight","caseId":"live","newSelfWeight":true}'),
    ("create-combination", '{"mutation":"createCombination","combination":{"id":"sls","name":"SLS characteristic","terms":[{"caseId":"dead","factor":1.0},{"caseId":"live","factor":1.0}]}}'),
    ("delete-combination", '{"mutation":"deleteCombination","id":"uls_spare"}'),
    ("update-analysis-settings", '{"mutation":"updateAnalysisSettings","settings":{"modalCount":6,"bucklingCount":4,"deformationScale":150.0}}'),
]

SPEC_VECTORS = [
    ("create-node", "mesh", "🌱⚪️create-node", "appends-node-n3"),
    ("delete-node", "mesh", "🗑⚪️delete-node", "removes-node-n3-without-cascading-to-its-support"),
    ("create-element", "mesh", "🌱🧩️create-element", "appends-bar-e2-between-n2-and-n3"),
    ("delete-element", "mesh", "🗑🧩️delete-element", "removes-bar-e2-and-keeps-its-end-nodes"),
    ("replace-element", "mesh", "🔁🧩️replace-element", "converts-beam-e1-into-a-bar-in-place"),
    ("create-material", "material", "🌱🧱️create-material", "appends-concrete-c30"),
    ("delete-material", "material", "🗑🧱️delete-material", "removes-the-unreferenced-timber-material"),
    ("replace-material", "material", "🔁🧱️replace-material", "restates-steel-as-s355-in-its-original-slot"),
    ("create-section", "mesh", "🌱create-section", "appends-the-ipe300-profile"),
    ("delete-section", "mesh", "🗑📐️delete-section", "removes-the-spare-hollow-section"),
    ("replace-section", "mesh", "🔁📐️replace-section", "stiffens-ipe200-with-a-reinforced-profile"),
    ("create-support", "boundary", "🌱🛡️create-support", "adds-a-vertical-roller-at-node-n2"),
    ("delete-support", "boundary", "🗑delete-support", "releases-the-roller-at-node-n2"),
    ("replace-support", "boundary", "🔁replace-support", "upgrades-the-roller-at-n2-to-a-full-fixity"),
    ("create-region", "mesh", "🌱🗺️create-region", "appends-a-solid-rectangular-slab"),
    ("delete-region", "mesh", "🗑🗺️delete-region", "removes-the-slab-and-keeps-its-material"),
    ("replace-region", "mesh", "🔁🗺️replace-region", "punches-a-stair-opening-through-the-slab"),
    ("create-load-case", "load", "🌱📋️create-load-case", "appends-a-live-case-carrying-one-nodal-load"),
    ("delete-load-case", "load", "🗑📋️delete-load-case", "removes-the-live-case-together-with-its-loads"),
    ("add-load", "load", "➕add-load", "appends-a-member-udl-to-the-dead-case"),
    ("remove-load", "load", "➖remove-load", "strips-the-trailing-member-udl-from-the-dead-case"),
    ("change-load-case-self-weight", "load", "⚖change-load-case-self-weight", "switches-self-weight-on-for-the-dead-case"),
    ("create-combination", "load", "🌱🔗️create-combination", "appends-an-uls-combination-over-both-cases"),
    ("delete-combination", "load", "🗑🔗️delete-combination", "removes-the-uls-combination-and-keeps-both-cases"),
    ("update-analysis-settings", "analysis", "🎛update-analysis-settings", "doubles-the-modal-count-and-halves-the-deformation-scale"),
]

SUBSET_KINDS = {
    "mesh": ["create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-region", "delete-region", "replace-region"],
    "material": ["create-material", "delete-material", "replace-material"],
    "boundary": ["create-support", "delete-support", "replace-support"],
    "load": ["create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight", "create-combination", "delete-combination"],
    "analysis": ["update-analysis-settings"],
}

mutate_map = dict(MUTATE_EXAMPLES)
spec_map = {k: (s, d, f) for k, s, d, f in SPEC_VECTORS}

CASE_NAME = {s: f"mutate-fem2d-1-{s}" for s in SUBSET_KINDS}
DERIVED = "🏗️timber-portal-frame.snapshot.json"
DSL_LEAF = "📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"

def pad(rows):
    widths = [max(len(r[i]) for r in rows) for i in range(len(rows[0]))]
    return widths

def table(headers, rows):
    all_rows = [headers] + rows
    widths = pad(all_rows)
    lines = []
    for r in all_rows:
        cells = " | ".join(c.ljust(w) for c, w in zip(r, widths))
        lines.append(f"    | {cells} |")
    return "\n".join(lines)

os.makedirs(f"{ART}/✳️mesh/🧪️tests", exist_ok=True)

for subset, kinds in SUBSET_KINDS.items():
    case = CASE_NAME[subset]
    case_dir = f"{ART}/✳️{subset}/🧪️tests/{case}"
    os.makedirs(f"{case_dir}/🧫️fixtures", exist_ok=True)

    # ---- feature ----
    mutate_rows = [[k, mutate_map[k]] for k in kinds]
    spec_rows = [[k, spec_map[k][1], spec_map[k][2]] for k in kinds]
    mutate_table = table(["id", "mutation"], mutate_rows)
    spec_table = table(["id", "dir", "fixture"], spec_rows)
    kind_list_human = ", ".join(f"`{k}`" for k in kinds)

    feature = f"""@capability-fem2d-1-mutate
@oracle-fem2d-python-independent
@comparison-ordered-json-v1
@mutations-fem2d-1-{subset}
Feature: Apply every typed fem2d {subset} mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-fem2d-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds ({kind_list_human}) have a subset-owned test. The reference is
  `🐍️.py` in this directory: a second implementation of the `s.fem.fem2d` structural model and
  this subset's typed mutations, written in Python from
  `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json` (the nine members, `additionalProperties: false`),
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
  is an EMPTY `{{"title": …, "type": "object"}}` with no properties at all. The record shapes the
  reference implements were read off the committed vectors instead, which agree with one another on
  every field.

  The artifact is real. `local://{DERIVED}` is the SAME derived timber-portal-frame model every
  fem2d mutation subset case shares — a twelve-node timber-and-steel portal frame with a ridge at
  7.6 m, derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-fem2d-frame.py`
  from the artifact's own committed demo model, with seven unreferenced spares appended so every
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
  Scenario Outline: Apply <id> to the real derived timber portal frame
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

print("all features written")
