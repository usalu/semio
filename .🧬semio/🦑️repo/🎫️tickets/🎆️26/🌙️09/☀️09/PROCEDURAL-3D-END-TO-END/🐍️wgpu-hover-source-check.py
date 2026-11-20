from pathlib import Path
from tempfile import TemporaryDirectory
import json
import subprocess

renderer = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer")
source = next(renderer.rglob("📎️registry-and-node-graph")) / "🦀️.rs"
fixture = next(renderer.rglob("🔣️hover.json"))
case = next(case for case in json.loads(fixture.read_text()) if case["id"].startswith("retained graph"))
owner = source.read_text()
begin = owner.index("type GraphHover")
end = owner.index("fn node_graph_current_hover", begin)
start = owner.index("fn node_graph_scene_hover_update")
finish = owner.index("fn node_graph_apply_hover", start)

def mark(record):
    if record is None:
        return "None"
    return "Some((" + ", ".join(json.dumps(record.get(key, "")) for key in ("nodeId", "portId", "edgeId")) + "))"

rows = []
for step in case["steps"]:
    node = step.get("localNode")
    update = step["write"]
    if update is None:
        expected = "None"
    elif not update:
        expected = "Some(None)"
    else:
        expected = "Some(" + mark({"nodeId": update["hoveredId"]}) + ")"
    rows.append(f"({mark(step['scene'])}, {mark({'nodeId': node} if node else None)}, {expected}, {mark({'nodeId': step['live']} if step['live'] else None)})")

check = owner[begin:end] + owner[start:finish] + '''
fn main() {
    let mut previous = None;
    let mut live = None;
    let mut owned = false;
    for (incoming, local, expected, expected_live) in [
        ''' + ",\n        ".join(rows) + '''
    ] {
        if local.is_some() { live = local; owned = false; }
        let update = node_graph_scene_hover_update(previous, incoming, live, owned);
        assert_eq!(update, expected);
        if previous != incoming {
            owned = update.flatten().is_some() || (update.is_none() && incoming.is_some() && owned);
            previous = incoming;
        }
        if let Some(update) = update { live = update; }
        assert_eq!(live, expected_live);
    }
}
'''
generated = Path(__file__).parent / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as work:
    path = Path(work) / "hover.rs"
    binary = path.with_suffix("")
    path.write_text(check)
    subprocess.run(["rustc", "--edition", "2024", str(path), "-o", str(binary)], check=True)
    subprocess.run([str(binary)], check=True)
print(f"{len(rows)} native hover reconciliation fixture steps passed")
