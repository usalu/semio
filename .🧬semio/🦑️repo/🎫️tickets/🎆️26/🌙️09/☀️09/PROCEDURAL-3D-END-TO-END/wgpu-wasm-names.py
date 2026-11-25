import json
import gzip
from pathlib import Path

root = Path(__file__).parent / "🗑️generated"
path = root / "bridge-current/named/semio-renderer-wgpu-component_bg.wasm"
data = path.read_bytes() if path.exists() else gzip.decompress(path.with_suffix(".wasm.gz").read_bytes())
profile = json.loads((root / "wgpu-paint/worker.cpuprofile").read_text())

def integer(pos):
    value = shift = 0
    while True:
        b = data[pos]
        pos += 1
        value |= (b & 127) << shift
        if b < 128:
            return value, pos
        shift += 7

def string(pos):
    length, pos = integer(pos)
    return data[pos:pos + length].decode("utf8", "replace"), pos + length

names = {}
pos = 8
while pos < len(data):
    section = data[pos]
    size, body = integer(pos + 1)
    end = body + size
    if section == 0:
        name, cursor = string(body)
        if name == "name":
            while cursor < end:
                kind = data[cursor]
                size, start = integer(cursor + 1)
                cursor = start + size
                if kind != 1:
                    continue
                count, start = integer(start)
                for _ in range(count):
                    index, start = integer(start)
                    names[index], start = string(start)
    pos = end

nodes = {node["id"]: node for node in profile["nodes"]}
totals = {}
for index, node_id in enumerate(profile.get("samples", [])):
    name = nodes[node_id]["callFrame"]["functionName"]
    if name.startswith("wasm-function["):
        number = int(name[14:-1])
        name = names.get(number, name)
    totals[name] = totals.get(name, 0) + profile["timeDeltas"][index]
for name, elapsed in sorted(totals.items(), key=lambda row: -row[1])[:15]:
    print(f"{elapsed / 1000:.1f} ms {name}")
parents = {child: node["id"] for node in profile["nodes"] for child in node.get("children", [])}
inclusive = {}
for index, node_id in enumerate(profile.get("samples", [])):
    seen = set()
    while node_id in nodes:
        name = nodes[node_id]["callFrame"]["functionName"]
        if name.startswith("wasm-function["):
            name = names.get(int(name[14:-1]), name)
        if name not in seen:
            inclusive[name] = inclusive.get(name, 0) + profile["timeDeltas"][index]
            seen.add(name)
        node_id = parents.get(node_id)
print("INCLUSIVE")
for name, elapsed in sorted(inclusive.items(), key=lambda row: -row[1])[:30]:
    print(f"{elapsed / 1000:.1f} ms {name}")
