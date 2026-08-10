from pathlib import Path
p = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
puzzle = next(x for x in p.iterdir() if x.name.endswith("puzzle"))
apps = next(x for x in puzzle.iterdir() if "apps" in x.name)
# print full 3d test
threed = next(x for x in apps.iterdir() if "3d" in x.name)
comp = next(x for x in threed.iterdir() if "component" in x.name)
text = comp.read_text()
idx = text.find("two_instances_converge_disjoint_object_edits_via_backbone")
print(text[idx : idx + 1200])
# 2d addNode handler - search for AddNode / add_node
twod = next(x for x in apps.iterdir() if "2d" in x.name)
comp2 = next(x for x in twod.iterdir() if "component" in x.name)
t2 = comp2.read_text()
for key in ["AddNode", "add_node", '"addNode"', "fn add_node", "Command::AddNode"]:
    print(key, t2.count(key))
# find setNode / anchor related mutation apply
for m_key in ["anchor", "SetNode", "UpsertNode", "apply_mutation", "ingest_operations"]:
    print("count", m_key, t2.count(m_key))
