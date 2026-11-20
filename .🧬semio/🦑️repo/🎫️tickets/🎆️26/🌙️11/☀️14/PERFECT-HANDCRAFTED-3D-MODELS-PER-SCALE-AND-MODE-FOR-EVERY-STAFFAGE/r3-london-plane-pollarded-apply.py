"""🌳 Round 3 london plane: writes the root fixes into an asset document text, member by member, so that every untouched member stays byte-identical.

Usage: python3 r3-london-plane-pollarded-apply.py <asset 🔣️.json> <out.json> [--geometry <2d.json>] [--anchors <anchors.json>] [--areas <areas.json>]
Applies `r3-london-plane-pollarded-roots.json` (bone z offsets, text corrections; idempotent), optionally replaces the `geometry` member of every representation, anchor coordinates and region `areaM2` values. The input is read right before writing; `<out.json>` may be the input itself.
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    return isinstance(value, list) and (all(scalar(i) for i in value) or all(isinstance(i, list) and all(scalar(leaf) for leaf in i) for i in value))


def dump(value, depth):
    pad, inner = " " * depth, " " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(k, ensure_ascii=False) + ": " + dump(v, depth + 1) for k, v in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(i, depth + 1) for i in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def member(text, indent, key, lo=0, hi=None):
    found = re.compile(r'\n' + " " * indent + '"' + re.escape(key) + r'":\s*').search(text, lo, len(text) if hi is None else hi)
    if not found:
        raise SystemExit("member %s not found" % key)
    return found.end(), member_end(text, found.end())


def option(name):
    return json.load(open(sys.argv[sys.argv.index(name) + 1], encoding="utf-8")) if name in sys.argv else None


def main():
    source, target = sys.argv[1], sys.argv[2]
    roots = json.load(open(os.path.join(HERE, "r3-london-plane-pollarded-roots.json"), encoding="utf-8"))
    geometry, anchors, areas = option("--geometry"), option("--anchors"), option("--areas")
    text = open(source, encoding="utf-8").read()
    before = json.loads(text)
    for old, new in roots["text"]:
        if old in text:
            text = text.replace(old, new)
        elif new not in text:
            raise SystemExit("text not found: " + old[:60])
    lo, hi = member(text, 2, "rig")
    rig = text[lo:hi]
    for bone, z in roots["boneZ"].items():
        found = re.search(r'(\n     "id": "' + re.escape(bone) + r'",.*?\n     "y": [-0-9.e]+,\n)(     "z": [-0-9.e]+,\n)?', rig, re.S)
        if not found:
            raise SystemExit("bone not found: " + bone)
        rig = rig[: found.end(1)] + '     "z": %s,\n' % json.dumps(z) + rig[found.end():]
    text = text[:lo] + rig + text[hi:]
    if anchors is not None:
        lo, hi = member(text, 2, "anchors")
        items = json.loads(text[lo:hi])
        for item in items:
            if item["id"] in anchors:
                item.update(anchors[item["id"]])
        text = text[:lo] + dump(items, 2) + text[hi:]
    if areas is not None:
        lo, hi = member(text, 2, "thermal")
        block = text[lo:hi]
        for region, value in areas.items():
            block, n = re.subn(r'("id": "' + re.escape(region) + r'",.*?"areaM2": )[-0-9.e]+', lambda m: m.group(1) + json.dumps(value), block, count=1, flags=re.S)
            if n != 1:
                raise SystemExit("region not found: " + region)
        text = text[:lo] + block + text[hi:]
    if geometry is not None:
        rlo, rhi = member(text, 2, "representations")
        for scale in ("500", "200", "100", "50"):
            slo, shi = member(text, 3, scale, rlo, rhi)
            glo, ghi = member(text, 4, "geometry", slo, shi)
            text = text[:glo] + dump(geometry[scale], 4) + text[ghi:]
    after = json.loads(text)
    allowed = {"rig", "summary", "provenance"} | ({"anchors"} if anchors is not None else set()) | ({"thermal"} if areas is not None else set()) | ({"representations"} if geometry is not None else set())
    changed = {k for k in after["asset"] if after["asset"][k] != before["asset"].get(k)}
    if not changed <= allowed or list(after["asset"]) != list(before["asset"]):
        raise SystemExit("unexpected change: %s" % sorted(changed - allowed))
    current = open(source, encoding="utf-8").read()
    if json.loads(current) != before:
        raise SystemExit("the asset changed while editing; run again")
    open(target, "w", encoding="utf-8").write(text)
    print("written", target, "changed members:", sorted(changed))


main()
