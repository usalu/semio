#!/usr/bin/env python3
"""Computes the lineage revision of a node in an XLSX worksheet part, a port of `address_revision` in the base subset's cell-address module, to author fixture rows."""
import sys, zipfile
from xml.parsers import expat

M = (1 << 64) - 1

def hb(h, data):
    for b in data:
        h = ((h ^ b) * 0x100000001b3) & M
    return h

def hf(h, s):
    b = s.encode()
    h = hb(h, len(b).to_bytes(8, "little"))
    return hb(h, b)

def parse(data):
    root = {"c": []}
    stack = [root]
    p = expat.ParserCreate()
    p.ordered_attributes = True
    def start(name, attrs):
        node = {"t": "e", "n": name, "a": [(attrs[i], attrs[i + 1]) for i in range(0, len(attrs), 2)], "c": []}
        stack[-1]["c"].append(node)
        stack.append(node)
    def end(name):
        stack.pop()
    def text(d):
        ch = stack[-1]["c"]
        if ch and ch[-1]["t"] == "x":
            ch[-1]["x"] += d
        else:
            ch.append({"t": "x", "x": d})
    p.StartElementHandler, p.EndElementHandler, p.CharacterDataHandler = start, end, text
    p.Parse(data, True)
    return root["c"][0]

def hash_node(h, n, rec):
    if n["t"] == "x":
        return hf(h, n["x"])
    h = hb(h, b"\x00")
    h = hf(h, n["n"])
    h = hb(h, len(n["a"]).to_bytes(8, "little"))
    for k, v in n["a"]:
        h = hf(h, k)
        h = hf(h, v)
    h = hb(h, len(n["c"]).to_bytes(8, "little"))
    if rec:
        for c in n["c"]:
            h = hash_node(h, c, True)
    return h

def revision(root, path, sst=None):
    h = 0xcbf29ce484222325
    node = root
    for i in path:
        h = hash_node(h, node, False)
        h = hb(h, i.to_bytes(8, "little"))
        for c in node["c"]:
            h = hash_node(h, c, False)
        node = node["c"][i]
    h = hash_node(h, node, True)
    if sst is not None and node["t"] == "e" and dict(node["a"]).get("t") == "s":
        idx = int("".join(x["x"] for c in node["c"] if c["t"] == "e" and c["n"] == "v" for x in c["c"] if x["t"] == "x").strip())
        entries = [c for c in sst["c"] if c["t"] == "e" and c["n"] == "si"]
        h = hash_node(h, entries[idx], True)
    return f"{h:016x}"

if __name__ == "__main__":
    xlsx, part, *path = sys.argv[1:]
    z = zipfile.ZipFile(xlsx)
    root = parse(z.read(part))
    sst = parse(z.read("xl/sharedStrings.xml")) if "xl/sharedStrings.xml" in z.namelist() else None
    print(revision(root, [int(i) for i in path], sst))
