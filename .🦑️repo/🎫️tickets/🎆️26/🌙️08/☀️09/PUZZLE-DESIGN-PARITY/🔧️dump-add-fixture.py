from pathlib import Path
comp = Path("/tmp/puzzle2d_component_path.txt").read_text().strip()
t = Path(comp).read_text()
i = t.find("pub fn add_node_to_fixture")
print(t[i:i+1500])
i = t.find("fn new_node_id")
print('\n==== new_node_id ====\n', t[i:i+400])
# how ops are produced - look for diff / emit / operations in Puzzle2dPlayApp
for key in ["fn apply_command", "emit_ops", "operations_from", "DocumentMutation", "setNode", "SetNode", "fn handle_typed", "dispatch_typed"]:
    print(key, t.find(key))
