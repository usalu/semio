#!/usr/bin/env python3
"""Derivation script for ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/mutate-gltf-2-0/🧫️fixtures/🧊️base-with-nested-node.glb

Real source: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb
(284 KB real export: 1 scene, 271 flat nodes each `{mesh:i}` with no `children`, 2 materials).

The real export's whole node graph is flat -- no node has `children`, so `bind-node-child`/
`unbind-node-child` (2 of the 7 registered glTF mutation kinds this ticket's oracle case covers)
have no real, pre-existing edge to exercise. This script performs the SINGLE derivation: node 1 is
moved out of `scenes[0].nodes` (the 271-entry root list) into `nodes[0].children`. Every other byte,
including the whole BIN chunk (skinning/mesh geometry), is copied verbatim from the real source --
only the JSON chunk's node/scene pointers change.

Run once; output committed directly. Not part of any build.
"""
import json
import struct

SOURCE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb"
DEST = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧪️tests/mutate-gltf-2-0/🧫️fixtures/🧊️base-with-nested-node.glb"


def read_glb(path):
    with open(path, "rb") as f:
        data = f.read()
    magic, version, length = struct.unpack_from("<III", data, 0)
    assert magic == 0x46546C67 and version == 2
    offset = 12
    doc, bin_chunk = None, None
    while offset < length:
        chunk_len, chunk_type = struct.unpack_from("<II", data, offset)
        chunk_data = data[offset + 8 : offset + 8 + chunk_len]
        if chunk_type == 0x4E4F534A:
            doc = json.loads(chunk_data)
        elif chunk_type == 0x004E4942:
            bin_chunk = chunk_data
        offset += 8 + chunk_len
    return doc, bin_chunk


def write_glb(doc, bin_chunk, path):
    json_bytes = json.dumps(doc, separators=(",", ":")).encode("utf-8")
    json_bytes += b" " * ((4 - len(json_bytes) % 4) % 4)
    out = bytearray()
    total_len = 12 + 8 + len(json_bytes) + (8 + len(bin_chunk) if bin_chunk is not None else 0)
    out += struct.pack("<III", 0x46546C67, 2, total_len)
    out += struct.pack("<II", len(json_bytes), 0x4E4F534A)
    out += json_bytes
    if bin_chunk is not None:
        out += struct.pack("<II", len(bin_chunk), 0x004E4942)
        out += bin_chunk
    with open(path, "wb") as f:
        f.write(bytes(out))


def main():
    doc, bin_chunk = read_glb(SOURCE)
    assert doc["scenes"][0]["nodes"][:2] == [0, 1], "source shape assumption changed"
    doc["nodes"][0]["children"] = [1]
    doc["scenes"][0]["nodes"] = [n for n in doc["scenes"][0]["nodes"] if n != 1]
    assert doc["scenes"][0]["nodes"].index(5) == 4, "node5's derived root position changed"
    write_glb(doc, bin_chunk, DEST)
    print(f"wrote {DEST}: {len(doc['scenes'][0]['nodes'])} scene roots (was 271), node0.children={doc['nodes'][0]['children']}")


if __name__ == "__main__":
    main()
