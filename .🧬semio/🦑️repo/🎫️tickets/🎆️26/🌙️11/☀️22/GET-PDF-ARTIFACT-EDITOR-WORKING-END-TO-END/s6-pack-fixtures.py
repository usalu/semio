"""🧮 Convert PdfImageBody samples fixtures from logical sample words to the native packed rows of ISO 32000-1 §8.9.3."""
import json
import math
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf")
SINGLE = {"deviceGray", "calGray", "indexed", "separation"}
TRIPLE = {"deviceRgb", "calRgb", "lab"}


def components(space):
    """🎨 Number of colour components of an image colour space, one when absent."""
    if space is None:
        return 1
    kind = space["kind"]
    if kind in SINGLE:
        return 1
    if kind in TRIPLE:
        return 3
    if kind == "deviceCmyk":
        return 4
    if kind == "iccBased":
        return space.get("components", space.get("n"))
    if kind == "deviceN":
        return len(space["names"])
    raise ValueError(f"no image components for colour space {kind}")


def pack(image, values):
    """📦 Pack logical samples into MSB-first rows padded to a byte boundary."""
    mask = bool(image.get("imageMask"))
    depth = 1 if mask else image["bitsPerComponent"]
    count = 1 if mask else components(image.get("colorSpace"))
    width, height = image["width"], image["height"]
    per_row = width * count
    if depth not in (1, 2, 4, 8, 16) or len(values) != per_row * height:
        raise ValueError(f"geometry {width}x{height}x{count}@{depth} does not hold {len(values)} samples")
    stride = math.ceil(per_row * depth / 8)
    packed = []
    for row in range(height):
        bits = 0
        for sample in values[row * per_row:(row + 1) * per_row]:
            if not 0 <= sample < 1 << depth:
                raise ValueError(f"sample {sample} exceeds {depth} bits")
            bits = bits << depth | sample
        bits <<= stride * 8 - per_row * depth
        packed.extend(bits.to_bytes(stride, "big") if stride else b"")
    return packed, (width, height, count, depth, mask)


def octets(values, reason):
    """🧷 Keep geometry-free codec bodies as their identical octets and flag them for review."""
    if any(not 0 <= value <= 255 for value in values):
        raise ValueError(f"{reason}: samples do not fit octets")
    return list(values), reason


def convert(owner, body):
    """🔁 Pack against a consistent owner, otherwise keep the identical octets of a codec stress body."""
    if owner is None:
        return octets(body["values"], "UNOWNED")
    try:
        return pack(owner, body["values"])
    except ValueError as error:
        return octets(body["values"], f"INCONSISTENT {owner['width']}x{owner['height']} bpc={owner.get('bitsPerComponent')} mask={owner.get('imageMask')}: {error}")


def walk(node, owner, report):
    """🚶 Rewrite every samples body against its owning image object."""
    if isinstance(node, dict):
        for key, value in list(node.items()):
            if isinstance(value, dict) and value.get("kind") == "samples" and "values" in value and set(value) == {"kind", "values"}:
                image = node if key == "body" and "width" in node and "height" in node else None
                packed, geometry = convert(image, value)
                report.append((geometry, value["values"], packed))
                node[key] = {"kind": "samples", "packed": packed}
            else:
                walk(value, owner, report)
    elif isinstance(node, list):
        for index, value in enumerate(node):
            if isinstance(value, dict) and value.get("kind") == "samples" and set(value) == {"kind", "values"}:
                packed, geometry = convert(None, value)
                report.append((geometry, value["values"], packed))
                node[index] = {"kind": "samples", "packed": packed}
            else:
                walk(value, owner, report)


def main(paths):
    for path in paths:
        text = path.read_text(encoding="utf-8")
        data = json.loads(text)
        report = []
        walk(data, None, report)
        if not report:
            continue
        trailing = "\n" if text.endswith("\n") else ""
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + trailing, encoding="utf-8")
        for geometry, values, packed in report:
            print(path.relative_to(ROOT), "w,h,n,bpc,mask=", geometry, "values=", values, "packed=", packed)


if __name__ == "__main__":
    main([pathlib.Path(arg) for arg in sys.argv[1:]] or sorted(ROOT.rglob("*.json")))
