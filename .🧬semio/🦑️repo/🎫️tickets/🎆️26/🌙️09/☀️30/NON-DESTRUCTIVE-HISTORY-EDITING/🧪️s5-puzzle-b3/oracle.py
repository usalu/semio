"""🐍️ Independent oracle of the board-tools corpus (`🧫️fixtures/🧫️board-tools/🔣️.json`).

Validates the corpus against its JSON Schema with `jsonschema`, then re-derives every case's `expect` from the board and
its rows with a board model of its own (nodes own handles, a wire joins two handles, a region is a `shapely` box) —
written from design §22.32 (a) and the board-event payloads, not from the Rust: a painted area needs a positive extent, a resize needs a
region the board holds, a stamped brush adds a node and the wire back to its source handle, a delete takes a node with
every wire on its handles, a run of rows of one kind is one release, the first release that changes the board names the
row's tool, and a flush that changes nothing commits nothing. Exits non-zero on the first disagreement; an optional
argument names another corpus (negative controls).
"""

import copy
import json
import math
import pathlib
import sys

import jsonschema
from shapely.geometry import box

EDITOR = pathlib.Path(__file__).resolve().parents[2]
TOOLS = ("regionCreate", "regionResize", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete")


class Board:
    def __init__(self, fixture):
        self.nodes = set()
        self.wires = {}
        self.owner = {}
        self.regions = []
        for node in fixture["nodes"]:
            self.place(node["id"], [handle["id"] for handle in node["handles"]])
        for edge in fixture["edges"]:
            self.wire(edge["id"], edge["source"], edge["target"])

    def place(self, node, handles):
        self.nodes.add(node)
        for handle in handles:
            self.owner[handle] = node

    def wire(self, edge, source, target):
        self.wires[edge] = (self.owner[source], self.owner[target])

    def state(self):
        return (sorted(self.nodes), sorted(self.wires), copy.deepcopy(self.regions))

    def fold(self, name, payload):
        if name == "regionCreate":
            numbers = [payload.get(key) for key in ("x", "y", "width", "height")]
            if all(isinstance(number, (int, float)) and math.isfinite(number) for number in numbers) and box(payload["x"], payload["y"], payload["x"] + payload["width"], payload["y"] + payload["height"]).area > 0 and payload["width"] > 0 and payload["height"] > 0:
                self.regions.append({key: float(payload[key]) for key in ("x", "y", "width", "height")})
        elif name == "regionResize":
            index = 0 if payload["id"] == "$region" and self.regions else None
            if index is not None:
                self.regions[index] = {key: float(payload[key]) for key in ("x", "y", "width", "height")}
        elif name == "brushPlace":
            node = payload["nodeId"]
            self.place(node, [f"{node}:v{index}" for index, _ in enumerate(payload.get("handles", []))])
            if payload.get("sourceHandleId"):
                self.wire(payload["edgeId"], payload["sourceHandleId"], f"{node}:v{payload.get('targetHandleIndex', 0)}")
        elif name == "edgeCreate":
            self.wire(payload["id"], payload["source"], payload["target"])
        elif name == "edgeDelete":
            self.wires.pop(payload["id"], None)
        elif name == "nodeDelete":
            if payload["id"] in self.nodes:
                self.nodes.remove(payload["id"])
                self.wires = {edge: ends for edge, ends in self.wires.items() if payload["id"] not in ends}
                self.owner = {handle: node for handle, node in self.owner.items() if node != payload["id"]}

    def flush(self, rows):
        tool = None
        index = 0
        while index < len(rows):
            end = index
            while end < len(rows) and rows[end]["name"] == rows[index]["name"]:
                end += 1
            before = self.state()
            for row in rows[index:end]:
                self.fold(row["name"], row["payload"])
            if rows[index]["name"] in TOOLS and self.state() != before and tool is None:
                tool = rows[index]["name"]
            index = end
        return tool


def main():
    corpus_path = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else EDITOR / "🧫️fixtures" / "🧫️board-tools" / "🔣️.json"
    corpus = json.loads(corpus_path.read_text(encoding="utf-8"))
    schema = json.loads((EDITOR / "🧬️schema" / "🔣️board-tools" / "🔣️.json").read_text(encoding="utf-8"))
    jsonschema.Draft7Validator.check_schema(schema)
    jsonschema.validate(corpus, schema)
    for case in corpus["cases"]:
        board = Board(corpus["board"])
        for flush in case["setup"]:
            if board.flush(flush) is None:
                sys.exit(f"{case['name']}: a setup flush changes nothing")
        tool = board.flush(case["rows"])
        nodes, edges, regions = board.state()
        derived = {"edits": 0 if tool is None else 1, "tool": tool, "nodes": len(nodes), "edges": len(edges), "targetRegions": len(regions)}
        if "region" in case["expect"]:
            derived["region"] = regions[0] if regions else None
        if derived != case["expect"]:
            sys.exit(f"{case['name']}: the corpus states {case['expect']}, the oracle derives {derived}")
        if case.get("transient") and (tool is not None or any(row["name"] in TOOLS for row in case["rows"])):
            sys.exit(f"{case['name']}: a transient case holds a board-tool row")
    print(f"board-tools corpus: {len(corpus['cases'])} cases agree with the oracle")


if __name__ == "__main__":
    main()
