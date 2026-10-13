"""🧫️ Independent oracle for the stage cache fixture: canonical key bytes (recursively sorted keys, no whitespace, UTF-8) and their SHA-256,
plus RFC 6962 Merkle roots over sorted leaves. Prints the vectors as JSON. usage: python3 🧫️stage-cache-oracle.py"""
import hashlib, json

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")

def merkle(leaves):
    nodes = [bytes.fromhex(leaf) for leaf in sorted(leaves)]
    def tree(items):
        if len(items) == 1:
            return hashlib.sha256(b"\x00" + items[0]).digest()
        split = 1
        while split * 2 < len(items):
            split *= 2
        return hashlib.sha256(b"\x01" + tree(items[:split]) + tree(items[split:])).digest()
    return (hashlib.sha256(b"").digest() if not nodes else tree(nodes)).hex()

keys = [
    {"stage": "describe", "schemaVersion": 1, "inputs": {"component": "a" * 64}, "tools": {"emitter": "b" * 64}},
    {"stage": "transpile", "schemaVersion": 2, "inputs": {"component": "c" * 64}, "tools": {"jco": "1.11.2", "materialization": "d" * 64, "node": "v24.3.0"}},
    {"stage": "optimize", "schemaVersion": 1, "inputs": {"core": "e" * 64, "core2": "f" * 64}, "tools": {"wasm-opt": "version 123", "args": "-Oz --strip-debug"}},
    {"stage": "probe", "schemaVersion": 1, "inputs": {"🌉️bridge.js": "0" * 64}, "tools": {"node": "v24.3.0 ünïcode"}},
]
leaves = [hashlib.sha256(str(index).encode()).hexdigest() for index in range(7)]
vectors = {
    "keys": [{"key": key, "canonical": canonical(key).decode("utf-8"), "sha256": hashlib.sha256(canonical(key)).hexdigest()} for key in keys],
    "merkle": [{"leaves": leaves[:count], "root": merkle(leaves[:count])} for count in [0, 1, 2, 3, 5, 7]],
}
print(json.dumps(vectors, ensure_ascii=False, indent=1))
