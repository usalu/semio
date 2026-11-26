"""🧮 Independently verify the funded cache fixture with Python's graphlib DAG walker."""
from graphlib import TopologicalSorter
from hashlib import sha256
import json
from pathlib import Path

fixture_path = Path("🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🧪️tests/🧫️fixtures/🪜️stepped-driver/🔣️.json")
fixture = json.loads(fixture_path.read_text())
recipe = fixture["cache"]
weights = {node["key"]: node["weight"] for node in fixture["nodes"]}
parents = {node["key"]: node["parents"] for node in fixture["nodes"]}
order = tuple(TopologicalSorter(parents).static_order())
memo = {}
source_ordinals = {node["key"]: ordinal for ordinal, node in enumerate(fixture["nodes"])}
domain = b"semio.original-inference.v1\0test.original-inference-cache\0" + (1).to_bytes(4, "little")

for index, expected in enumerate(recipe["runs"]):
    if index == 2:
        weights[recipe["edit"]["key"]] = recipe["edit"]["weight"]
    values, ancestry, digests = {}, {}, {}
    hits = 0
    for node in order:
        fingerprint = (node, weights[node], tuple(ancestry[parent] for parent in parents[node]))
        ancestry[node] = fingerprint
        own_hash = sha256(domain + source_ordinals[node].to_bytes(8, "little") + weights[node].to_bytes(8, "little", signed=True)).digest()
        for parent in parents[node]:
            own_hash = sha256(b"parent\0" + own_hash + digests[parent]).digest()
        digests[node] = own_hash
        if fingerprint in memo:
            hits += 1
        else:
            memo[fingerprint] = weights[node] + sum(values[parent] for parent in parents[node])
        values[node] = memo[fingerprint]
    actual = {"computed": len(order) - hits, "hits": hits}
    assert actual == expected, (actual, expected)
    assert values == (fixture["expected"] if index < 2 else recipe["edited"]), values
    assert {key: digest.hex() for key, digest in digests.items()} == recipe["initialFingerprints" if index < 2 else "editedFingerprints"]
    print(json.dumps({"run": index + 1, **actual, "values": values}, sort_keys=True))
