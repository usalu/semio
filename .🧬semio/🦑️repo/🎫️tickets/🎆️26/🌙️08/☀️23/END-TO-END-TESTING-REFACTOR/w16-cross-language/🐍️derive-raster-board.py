#!/usr/bin/env python3
"""🖨️ Derives the `mutate-raster-1` case fixture ONCE from committed real content.

Provenance, in full — every node in the output is copied from a committed file, none is invented:

* ``✏️s/🔌️plugins/🖨️raster/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`` — the artifact's own
  committed demo document. Its carrier is a compact positional encoding of hex-of-UTF-8 members:
  `p[…]` a pixel layer, `a[…]` an adjustment layer, `[0]` an absent optional and `[1,x]` a present
  one. From it come the document's `schema`, `id` and `title`, both of its real layers — the
  1024×1024 `backdrop` pixel layer bound to the `semio-emblem` image key, and the `brighten`
  `brightnessContrast` adjustment layer with its committed 0.12/0.08 parameters — and the real
  `semio-emblem` asset handle.
* ``…/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/…/📸️snapshot/⬅️before/🔣️component.json`` — the
  committed `artwork` GROUP node with its `sketch` child, taken verbatim.

The derivation is the composition and nothing else: the committed group is placed first among the
document's root layers, followed by the two committed real layers. No field is edited.

Why it exists: the committed example is FLAT, and three of the twelve kinds address a group —
`create-layer` takes a `parentId`, `delete-layer`'s committed vector removes a group with nested
children, and `reorder-layers` can lift a node OUT of a group. Running them against a document with
no group would exercise only the root case. It also lets `add-layer-asset` and `remove-layer-asset`
be exercised in their ACCEPTING direction for the first time: the two committed vectors for those
kinds are a declared no-op and a declared rejection, a gap the case's own feature already names.
"""

# region 🔖️Imports
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
def split_top(text):
    """✂️ Splits a bracketed carrier list into its top-level members."""
    members, depth, start = [], 0, 0
    for at, character in enumerate(text):
        if character == "[":
            depth += 1
        elif character == "]":
            depth -= 1
        elif character == "," and depth == 0:
            members.append(text[start:at])
            start = at + 1
    if text[start:]:
        members.append(text[start:])
    return members


def unhex(token):
    """🔤️ A hex-of-UTF-8 carrier scalar."""
    return bytes.fromhex(token).decode("utf-8")


def optional(token):
    """❓️ `[0]` is an absent optional, `[1,x]` a present one."""
    members = split_top(token[1:-1])
    return None if members[0] == "0" else members[1]


def transform(token):
    """📐️ The carrier's five-number transform tuple."""
    x, y, scale_x, scale_y, rotation = (float(member) for member in split_top(token[1:-1]))
    return {"x": x, "y": y, "scaleX": scale_x, "scaleY": scale_y, "rotation": rotation}


def base_layer(members):
    """🧱️ The six members every layer node shares, in the carrier's own order. `mask` is NOT one of
    them — the committed vectors show it on `pixel` and `group` nodes only, never on `adjustment`."""
    return {"id": unhex(members[0]), "name": unhex(members[1]), "visible": members[2] == "true", "opacity": float(members[3]), "blendMode": unhex(members[4]), "transform": transform(members[5])}


def layer(token):
    """🧱️ One carrier layer member: `p[…]` pixel or `a[…]` adjustment."""
    tag, body = token[0], split_top(token[2:-1])
    shared = base_layer(body)
    if tag == "p":
        width, height, image = optional(body[7]), optional(body[8]), optional(body[9])
        return {"kind": "pixel", **shared, "mask": optional(body[6]), "width": int(width), "height": int(height), "imageKey": unhex(image) if image else None}
    if tag == "a":
        params = {unhex(split_top(pair[1:-1])[0]): float(unhex(split_top(pair[1:-1])[1])) for pair in split_top(body[7][1:-1])}
        return {"kind": "adjustment", **shared, "adjustmentKind": unhex(body[6]), "params": params}
    raise SystemExit("unknown carrier layer tag %r" % tag)


def asset(token):
    """🔗️ One `[hex(assetId),[hex(childId),hex(target)]]` member of the root asset pool."""
    key, handle = split_top(token[1:-1])
    child, target = (unhex(part) for part in split_top(handle[1:-1]))
    artifact, _, dialect = target.partition("!")
    kind, _, rest = dialect.partition("@")
    standard, _, subset = rest.partition("/")
    return unhex(key), {"childId": child, "target": {"artifactId": artifact, "dialect": {"artifactKind": kind, "standard": standard, "subset": subset}}}


def carrier(path):
    """📖️ The committed demo document, as the members its carrier writes."""
    table = {}
    for line in open(path, encoding="utf-8").read().splitlines()[1:]:
        key, separator, value = line.partition("=")
        if separator == "=":
            table[key] = value
    return {
        "schema": unhex(table["schema"]),
        "id": unhex(table["id"]),
        "title": unhex(optional(table["title"])),
        "layers": [layer(token) for token in split_top(table["layers"][1:-1])],
        "assets": dict(asset(token) for token in split_top(table["assets"][1:-1])),
    }


# endregion 🔖️Carrier


# region 🔖️Derivation
def derive(example, group_vector, target):
    """🧬️ Writes the derived document: the committed group first, then the two committed real layers."""
    document = carrier(example)
    if len(document["layers"]) != 2 or [node["kind"] for node in document["layers"]] != ["pixel", "adjustment"]:
        raise SystemExit("the committed example is expected to carry one pixel and one adjustment layer")
    group = json.load(open(group_vector, encoding="utf-8"))["layers"][0]
    if group["kind"] != "group":
        raise SystemExit("the committed create-layer vector is expected to open with a group node")
    document["layers"] = [group, *document["layers"]]
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d root layer(s), %d asset(s))" % (target, len(document["layers"]), len(document["assets"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2], sys.argv[3])
