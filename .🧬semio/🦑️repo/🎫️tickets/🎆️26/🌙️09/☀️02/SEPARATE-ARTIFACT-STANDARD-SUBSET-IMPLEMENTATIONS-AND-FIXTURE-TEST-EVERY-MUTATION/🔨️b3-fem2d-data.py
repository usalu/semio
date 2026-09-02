# Shared data for fem2d split, imported by other generator scripts.

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

assert sum(len(v) for v in SUBSET_KINDS.values()) == 25
assert set(k for k, _ in MUTATE_EXAMPLES) == set(k for k, _, _, _ in SPEC_VECTORS)
mutate_map = dict(MUTATE_EXAMPLES)
spec_map = {k: (s, d, f) for k, s, d, f in SPEC_VECTORS}
for subset, kinds in SUBSET_KINDS.items():
    for k in kinds:
        assert spec_map[k][0] == subset, (k, spec_map[k], subset)
print("data OK")
