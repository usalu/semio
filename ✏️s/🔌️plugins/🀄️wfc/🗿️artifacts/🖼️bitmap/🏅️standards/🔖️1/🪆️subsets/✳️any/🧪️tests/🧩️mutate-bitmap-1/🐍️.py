"""🐍️ s.wfc.bitmap — an INDEPENDENT second implementation of the document and all ten typed
mutations, written from `../../🧬️schema/📸️snapshot/🔣️.json`, the ten per-kind payload schemas, and
RFC 4648 §4 for the pixel carrier. It imports nothing from this repository and transliterates none
of its Rust: the base64 codec below is written from the RFC, not ported.

Run it standalone from the repository root:

    python3 "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-bitmap-1/🐍️.py"

It replays every committed quintet: applies the mutation to `before`, requires the committed `after`
member by member, computes its OWN inverse and requires `before` back, and checks that the committed
diff declares exactly the lanes that moved.
"""

import json
import os
import sys

ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
VALUE = {character: index for index, character in enumerate(ALPHABET)}

HERE = os.path.dirname(os.path.abspath(__file__))
SUBSET = os.path.abspath(os.path.join(HERE, "..", ".."))
FIXTURES = os.path.join(SUBSET, "🧫️fixtures", "🧬️mutations")


def decode_base64(text):
    """🔤️ RFC 4648 §4 standard base64 with padding — written from the RFC."""
    if len(text) % 4:
        raise ValueError("base64 length is not a multiple of four")
    out = bytearray()
    for start in range(0, len(text), 4):
        quad = text[start : start + 4]
        pad = quad.count("=")
        if pad > 2 or (pad and not quad.endswith("=" * pad)):
            raise ValueError("misplaced base64 padding")
        triple = 0
        for offset, character in enumerate(quad):
            triple |= (0 if character == "=" else VALUE[character]) << (18 - 6 * offset)
        out.append((triple >> 16) & 0xFF)
        if pad < 2:
            out.append((triple >> 8) & 0xFF)
        if pad < 1:
            out.append(triple & 0xFF)
    return bytes(out)


def encode_base64(data):
    out = []
    for start in range(0, len(data), 3):
        chunk = data[start : start + 3]
        triple = (chunk[0] << 16) | ((chunk[1] if len(chunk) > 1 else 0) << 8) | (chunk[2] if len(chunk) > 2 else 0)
        out.append(ALPHABET[(triple >> 18) & 63])
        out.append(ALPHABET[(triple >> 12) & 63])
        out.append(ALPHABET[(triple >> 6) & 63] if len(chunk) > 1 else "=")
        out.append(ALPHABET[triple & 63] if len(chunk) > 2 else "=")
    return "".join(out)


def indices(document):
    buffer = decode_base64(document["input"]["pixels"])
    if len(buffer) != document["input"]["width"] * document["input"]["height"]:
        raise ValueError("the pixel buffer is not width * height bytes")
    return bytearray(buffer)


def with_indices(document, buffer):
    document["input"]["pixels"] = encode_base64(bytes(buffer))
    return document


def pin_order(pin):
    return (pin["y"], pin["x"])


def apply_mutation(document, mutation):
    """🧬️ The whole vocabulary. Raises `Fatal` for every refusal the specification declares."""
    document = json.loads(json.dumps(document))
    (kind, payload), = mutation.items()
    if kind == "ChangeSeed":
        if document["seed"] == payload["seed"]:
            return document, ["mutation.no-op"]
        document["seed"] = payload["seed"]
        return document, []
    if kind == "ResizeInput":
        width, height = payload["width"], payload["height"]
        if width == 0 or height == 0 or width > 512 or height > 512:
            raise Fatal("mutation.invariant")
        if (width, height) == (document["input"]["width"], document["input"]["height"]):
            return document, ["mutation.no-op"]
        old = indices(document)
        grown = bytearray(width * height)
        for y in range(min(height, document["input"]["height"])):
            for x in range(min(width, document["input"]["width"])):
                grown[y * width + x] = old[y * document["input"]["width"] + x]
        document["input"]["width"], document["input"]["height"] = width, height
        return with_indices(document, grown), []
    if kind == "SetInputPixels":
        region = decode_base64(payload["pixels"])
        width, height = payload["width"], payload["height"]
        if width == 0 or height == 0 or len(region) != width * height:
            raise Fatal("mutation.invariant")
        if payload["x"] + width > document["input"]["width"] or payload["y"] + height > document["input"]["height"]:
            raise Fatal("mutation.invariant")
        if any(index >= len(document["input"]["palette"]) for index in region):
            raise Fatal("mutation.unknown-palette-color")
        buffer = indices(document)
        prior = bytearray()
        for row in range(height):
            start = (payload["y"] + row) * document["input"]["width"] + payload["x"]
            prior += buffer[start : start + width]
        if bytes(prior) == region:
            return document, ["mutation.no-op"]
        for row in range(height):
            start = (payload["y"] + row) * document["input"]["width"] + payload["x"]
            buffer[start : start + width] = region[row * width : (row + 1) * width]
        return with_indices(document, buffer), []
    if kind == "AddPaletteColor":
        index = payload["index"]
        palette = document["input"]["palette"]
        if index > len(palette) or len(palette) >= 256:
            raise Fatal("mutation.invariant")
        palette.insert(index, payload["color"])
        buffer = indices(document)
        for position, value in enumerate(buffer):
            if value >= index:
                buffer[position] = value + 1
        for pin in document.get("pinned", []):
            if pin["color"] >= index:
                pin["color"] += 1
        return with_indices(document, buffer), []
    if kind == "ChangePaletteColor":
        index = payload["index"]
        palette = document["input"]["palette"]
        if index >= len(palette):
            raise Fatal("mutation.missing-target")
        if palette[index] == payload["color"]:
            return document, ["mutation.no-op"]
        palette[index] = payload["color"]
        return document, []
    if kind == "RemovePaletteColor":
        index = payload["index"]
        palette = document["input"]["palette"]
        if index >= len(palette):
            raise Fatal("mutation.missing-target")
        if len(palette) == 1:
            raise Fatal("mutation.invariant")
        buffer = indices(document)
        used = set(buffer) | {pin["color"] for pin in document.get("pinned", [])}
        if index in used:
            raise Fatal("mutation.colour-in-use")
        palette.pop(index)
        for position, value in enumerate(buffer):
            if value > index:
                buffer[position] = value - 1
        for pin in document.get("pinned", []):
            if pin["color"] > index:
                pin["color"] -= 1
        return with_indices(document, buffer), []
    if kind == "ResizeOutput":
        width, height = payload["width"], payload["height"]
        if width == 0 or height == 0 or width > 512 or height > 512:
            raise Fatal("mutation.invariant")
        output = {"width": width, "height": height, "periodic": payload["periodic"]}
        if document["output"] == output:
            return document, ["mutation.no-op"]
        kept = [pin for pin in document.get("pinned", []) if pin["x"] < width and pin["y"] < height]
        cascaded = len(document.get("pinned", [])) - len(kept)
        document["output"] = output
        document["pinned"] = kept
        return document, (["mutation.cascade"] if cascaded else [])
    if kind == "ChangeModel":
        if not 2 <= payload["patternSize"] <= 5 or not 1 <= payload["symmetry"] <= 8:
            raise Fatal("mutation.invariant")
        ground = payload.get("ground")
        if ground is not None and ground >= len(document["input"]["palette"]):
            raise Fatal("mutation.unknown-palette-color")
        model = {"patternSize": payload["patternSize"], "symmetry": payload["symmetry"], "periodicInput": payload["periodicInput"]}
        if ground is not None:
            model["ground"] = ground
        if document["model"] == model:
            return document, ["mutation.no-op"]
        document["model"] = model
        return document, []
    if kind == "PinPixel":
        if payload["x"] >= document["output"]["width"] or payload["y"] >= document["output"]["height"]:
            raise Fatal("mutation.invariant")
        if payload["color"] >= len(document["input"]["palette"]):
            raise Fatal("mutation.unknown-palette-color")
        pins = document.setdefault("pinned", [])
        for pin in pins:
            if (pin["x"], pin["y"]) == (payload["x"], payload["y"]):
                if pin["color"] == payload["color"]:
                    return document, ["mutation.no-op"]
                pin["color"] = payload["color"]
                return document, []
        pins.append({"x": payload["x"], "y": payload["y"], "color": payload["color"]})
        pins.sort(key=pin_order)
        return document, []
    if kind == "UnpinPixel":
        pins = document.setdefault("pinned", [])
        remaining = [pin for pin in pins if (pin["x"], pin["y"]) != (payload["x"], payload["y"])]
        if len(remaining) == len(pins):
            raise Fatal("mutation.missing-target")
        document["pinned"] = remaining
        return document, []
    raise ValueError("unknown mutation kind " + kind)


def inverse(document, mutation):
    """↩️ The reference's OWN inverse, computed against `before` — never read from the fixture."""
    (kind, payload), = mutation.items()
    if kind == "ChangeSeed":
        return [{"ChangeSeed": {"seed": document["seed"]}}]
    if kind == "ResizeInput":
        if (payload["width"], payload["height"]) == (document["input"]["width"], document["input"]["height"]):
            return []
        return [
            {"ResizeInput": {"width": document["input"]["width"], "height": document["input"]["height"]}},
            {"SetInputPixels": {"x": 0, "y": 0, "width": document["input"]["width"], "height": document["input"]["height"], "pixels": document["input"]["pixels"]}},
        ]
    if kind == "SetInputPixels":
        buffer = indices(document)
        width, height = payload["width"], payload["height"]
        prior = bytearray()
        for row in range(height):
            start = (payload["y"] + row) * document["input"]["width"] + payload["x"]
            prior += buffer[start : start + width]
        return [{"SetInputPixels": {"x": payload["x"], "y": payload["y"], "width": width, "height": height, "pixels": encode_base64(bytes(prior))}}]
    if kind == "AddPaletteColor":
        return [{"RemovePaletteColor": {"index": payload["index"]}}]
    if kind == "ChangePaletteColor":
        return [{"ChangePaletteColor": {"index": payload["index"], "color": document["input"]["palette"][payload["index"]]}}]
    if kind == "RemovePaletteColor":
        return [{"AddPaletteColor": {"index": payload["index"], "color": document["input"]["palette"][payload["index"]]}}]
    if kind == "ResizeOutput":
        steps = [{"ResizeOutput": dict(document["output"])}]
        for pin in document.get("pinned", []):
            if pin["x"] >= payload["width"] or pin["y"] >= payload["height"]:
                steps.append({"PinPixel": dict(pin)})
        return steps
    if kind == "ChangeModel":
        model = document["model"]
        restored = {"patternSize": model["patternSize"], "symmetry": model["symmetry"], "periodicInput": model["periodicInput"]}
        if model.get("ground") is not None:
            restored["ground"] = model["ground"]
        return [{"ChangeModel": restored}]
    if kind == "PinPixel":
        for pin in document.get("pinned", []):
            if (pin["x"], pin["y"]) == (payload["x"], payload["y"]):
                return [] if pin["color"] == payload["color"] else [{"PinPixel": dict(pin)}]
        return [{"UnpinPixel": {"x": payload["x"], "y": payload["y"]}}]
    if kind == "UnpinPixel":
        for pin in document.get("pinned", []):
            if (pin["x"], pin["y"]) == (payload["x"], payload["y"]):
                return [{"PinPixel": dict(pin)}]
        return []
    raise ValueError("unknown mutation kind " + kind)


class Fatal(Exception):
    pass


def moved_lanes(before, after):
    lanes = set()
    for field in ("schema", "seed", "output", "model"):
        if before.get(field) != after.get(field):
            lanes.add(field)
    if (before["input"]["width"], before["input"]["height"]) != (after["input"]["width"], after["input"]["height"]):
        lanes.add("inputExtent")
    if before["input"]["pixels"] != after["input"]["pixels"]:
        lanes.add("inputPixels")
    if before["input"]["palette"] != after["input"]["palette"]:
        lanes.add("palette")
    if before.get("pinned", []) != after.get("pinned", []):
        lanes.add("pinned")
    return lanes


def declared_lanes(diff):
    lanes = set()
    if diff.get("schema") is not None:
        lanes.add("schema")
    if diff.get("seed") is not None:
        lanes.add("seed")
    if diff.get("inputWidth") is not None or diff.get("inputHeight") is not None:
        lanes.add("inputExtent")
    if diff.get("inputPixels") is not None or diff.get("inputRegions"):
        lanes.add("inputPixels")
    if diff.get("palette") is not None:
        lanes.add("palette")
    if diff.get("output") is not None:
        lanes.add("output")
    if diff.get("model") is not None:
        lanes.add("model")
    if diff.get("pinnedRemoved") or diff.get("pinnedUpserted"):
        lanes.add("pinned")
    return lanes


def read(directory, *leaf):
    with open(os.path.join(directory, *leaf), encoding="utf-8") as handle:
        return json.load(handle)


def vectors():
    for kind_dir in sorted(os.listdir(FIXTURES)):
        kind_path = os.path.join(FIXTURES, kind_dir)
        if not os.path.isdir(kind_path):
            continue
        for case_dir in sorted(os.listdir(kind_path)):
            yield kind_dir, case_dir, os.path.join(kind_path, case_dir)


def main():
    failures = []
    checked = 0
    for kind_dir, case_dir, path in vectors():
        before = read(path, "📸️snapshot", "⬅️before", "🔣️.json")
        after = read(path, "📸️snapshot", "➡️after", "🔣️.json")
        mutation = read(path, "🦠️mutation", "🔣️.json")
        diff = read(path, "🔺️diff", "🔣️.json")
        outcome = read(path, "🎯️outcome", "🔣️.json")
        label = kind_dir + "/" + case_dir
        try:
            produced, messages = apply_mutation(before, mutation)
        except Fatal as fatal:
            failures.append(label + ": the reference refuses this vector (" + str(fatal) + ")")
            continue
        if produced != after:
            failures.append(label + ": the reference's after-snapshot differs from the committed one")
        declared = [row["code"] for row in outcome.get("messages", [])]
        if sorted(messages) != sorted(declared):
            failures.append(label + ": diagnostics " + repr(messages) + " differ from the committed " + repr(declared))
        restored = produced
        for step in inverse(before, mutation):
            restored, _ = apply_mutation(restored, step)
        if restored != before:
            failures.append(label + ": the reference's own inverse did not restore the before-snapshot")
        moved, declared_set = moved_lanes(before, after), declared_lanes(diff)
        if moved - declared_set:
            failures.append(label + ": lanes " + repr(sorted(moved - declared_set)) + " moved but the committed diff does not declare them")
        checked += 1
    for failure in failures:
        print("FAIL " + failure)
    print("checked " + str(checked) + " committed vectors, " + str(len(failures)) + " failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
