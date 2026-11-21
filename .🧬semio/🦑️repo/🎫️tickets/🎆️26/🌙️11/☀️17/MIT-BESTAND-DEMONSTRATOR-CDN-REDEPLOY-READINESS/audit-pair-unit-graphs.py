"""🪢️ Compare transitive Cargo unit identities from the retained release plans."""
from collections import Counter
from functools import lru_cache
from hashlib import sha256
from itertools import combinations
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
GENERATED = TICKET / "🗑️generated"
CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"


def root_graph(path, workspace, roster):
    graph = json.loads(path.read_bytes())
    units = graph["units"]

    @lru_cache(None)
    def signature(index):
        raw = units[index]
        fields = {key: value for key, value in raw.items() if key != "dependencies"}
        dependencies = [{**edge, "index": signature(edge["index"])} for edge in raw["dependencies"]]
        fields["dependencies"] = sorted(dependencies, key=lambda edge: json.dumps(edge, sort_keys=True))
        return sha256(json.dumps(fields, sort_keys=True, ensure_ascii=False).encode()).hexdigest()

    for index in graph["roots"]:
        unit = units[index]
        owner = unit["pkg_id"].rsplit("#", 1)[-1].split("@")[0]
        plugin = workspace["pluginIds"].get(owner)
        if plugin not in roster:
            continue
        closure, pending = set(), [index]
        while pending:
            next_unit = pending.pop()
            if next_unit in closure:
                continue
            closure.add(next_unit)
            pending.extend(edge["index"] for edge in units[next_unit]["dependencies"])
        yield plugin, {
            "manifest": workspace["manifest"],
            "workspaceGraph": path.name,
            "root": index,
            "units": len(closure),
            "signatures": Counter(signature(i) for i in closure if i != index),
        }


def graph_matches(all_roots):
    for left, right in combinations(sorted(all_roots), 2):
        a, b = all_roots[left], all_roots[right]
        shared = sum((a["signatures"] & b["signatures"]).values())
        union = sum((a["signatures"] | b["signatures"]).values())
        yield {
            "left": left, "right": right, "units": [a["units"], b["units"]],
            "shared": shared, "union": union, "sameWorkspace": a["manifest"] == b["manifest"],
            "ratio": round(shared / union, 4),
        }


cases = json.loads(CASES.read_bytes())
roster = set(cases["pluginIds"] + cases["extensionIds"])
workspaces = json.loads((GENERATED / "pair-workspaces.json").read_bytes())
roots = {}
for number, workspace in enumerate(workspaces):
    path = GENERATED / f"roster-graph-{number}.json"
    for plugin, graph in root_graph(path, workspace, roster):
        assert plugin not in roots, plugin
        roots[plugin] = graph
assert roots.keys() == roster, (sorted(roster - roots.keys()))
pairs = sorted(graph_matches(roots), key=lambda pair: (-pair["ratio"], -pair["shared"], pair["left"], pair["right"]))
print(json.dumps({"roots": {plugin: {k:v for k,v in data.items() if k != "signatures"} for plugin,data in roots.items()}, "pairs": pairs}, ensure_ascii=False, indent=2))
