"""🧪️ W3-T-DRAW: independent second implementation of the draw selection-transform leaves.

Writes one committed fixture quintet per new leaf (`drag-layers`, `rotate-layers`, `scale-layers`, `drag-path-points`)
under `🔀️transform/🧫️fixtures/🧬️mutations/`. The after-document and the sparse diff are computed HERE, in Python, from the
leaf semantics (world-space motion mapped through each layer's parent matrix; path points translated in the path's own
axes, an anchor carrying its attached tangents) with the same IEEE operation order as the Rust leaves, so the Rust leaf
tests compare two independent implementations byte for byte. Run from the repository root:

    python3 '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-draw-selection-leaves.py'
"""

import copy
import json
import math
import os

ROOT = "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations"
IDENTITY = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
PATCH_FIELDS = ["visible", "locked", "name", "opacity", "blendMode", "transformJson", "fillJson", "strokeJson", "booleanOperation", "traceParamsJson", "layerJson", "pathSegments", "textContent", "textSize", "fillRule"]


def multiply(a, b):
    return [a[0] * b[0] + a[2] * b[1], a[1] * b[0] + a[3] * b[1], a[0] * b[2] + a[2] * b[3], a[1] * b[2] + a[3] * b[3], a[0] * b[4] + a[2] * b[5] + a[4], a[1] * b[4] + a[3] * b[5] + a[5]]


def inverse(m):
    a, b, c, d, e, f = m
    det = a * d - b * c
    return [d / det, -b / det, -c / det, a / det, (c * f - d * e) / det, (b * e - a * f) / det]


def to_matrix(t):
    s, c = math.sin(t["rotation"]), math.cos(t["rotation"])
    return [t["scaleX"] * c, t["scaleX"] * s, t["shear"] * c - t["scaleY"] * s, t["shear"] * s + t["scaleY"] * c, t["x"], t["y"]]


def to_transform(m):
    a, b, c, d, x, y = m
    scale_x = math.hypot(a, b)
    ux, uy = a / scale_x, b / scale_x
    return {"x": x, "y": y, "scaleX": scale_x, "scaleY": ux * d - uy * c, "rotation": math.atan2(b, a), "shear": ux * c + uy * d}


def dragged(t, parent, delta):
    inv = inverse(parent)
    out = dict(t)
    out["x"] = t["x"] + (inv[0] * delta[0] + inv[2] * delta[1])
    out["y"] = t["y"] + (inv[1] * delta[0] + inv[3] * delta[1])
    return out


def moved(t, parent, motion):
    return to_transform(multiply(inverse(parent), multiply(motion, multiply(parent, to_matrix(t)))))


def rotation(px, py, angle):
    s, c = math.sin(angle), math.cos(angle)
    return [c, s, -s, c, px - c * px + s * py, py - s * px - c * py]


def scaling(px, py, sx, sy):
    return [sx, 0.0, 0.0, sy, px * (1.0 - sx), py * (1.0 - sy)]


def transform_json(t):
    return "{" + ",".join(f'"{key}":{repr(float(t[key]))}' for key in ["x", "y", "scaleX", "scaleY", "shear", "rotation"]) + "}"


def base_layer(kind, layer_id, name, x=0.0, y=0.0, scale_x=1.0, scale_y=1.0):
    return {"kind": kind, "id": layer_id, "name": name, "visible": True, "locked": False, "opacity": 1.0, "blendMode": "normal", "transform": {"x": x, "y": y, "scaleX": scale_x, "scaleY": scale_y, "rotation": 0.0, "shear": 0.0}, "attributes": {"fillRule": "evenodd"}}


def rect(layer_id, name, x=0.0, y=0.0):
    layer = base_layer("shape", layer_id, name, x, y)
    layer["shapeKind"] = "rect"
    layer["rect"] = {"x": 0.0, "y": 0.0, "width": 120.0, "height": 60.0}
    return layer


def document(layers):
    return {"schema": "drawing.document", "id": "drawing-fixture", "title": "Fixture Base", "layers": layers, "artboard": {"width": 640.0, "height": 480.0}}


def walk(layers, parent=IDENTITY):
    for layer in layers:
        yield layer, parent
        if layer["kind"] == "group":
            yield from walk(layer["children"], multiply(parent, to_matrix(layer["transform"])))


def patch_entry(layer_id, **fields):
    patch = {field: None for field in PATCH_FIELDS}
    patch.update(fields)
    return {"id": layer_id, "patch": patch}


def diff_of(patched):
    return {"artifact": None, "schema": None, "id": None, "title": None, "layers": {"added": [], "removed": [], "patched": patched, "reordered": None}, "assets": None, "artboard": None}


def layer_case(before, mutation, place):
    after = copy.deepcopy(before)
    patched = []
    for layer, parent in walk(after["layers"]):
        if layer["id"] in mutation["targets"]:
            layer["transform"] = place(layer["transform"], parent)
            patched.append(patch_entry(layer["id"], transformJson=transform_json(layer["transform"])))
    return after, diff_of(patched)


def translate_points(segments, refs, delta):
    masks = [0] * len(segments)
    for index, point in refs:
        kind = segments[index]["kind"]
        masks[index] |= {"anchor": 1, "control1": 2, "control2": 4}[point]
        if point == "anchor":
            masks[index] |= {"quad": 2, "cubic": 4}.get(kind, 0)
            if index + 1 < len(segments) and segments[index + 1]["kind"] in ("quad", "cubic"):
                masks[index + 1] |= 2
    out = copy.deepcopy(segments)
    for segment, mask in zip(out, masks):
        def shift(key, bit):
            if mask & bit:
                segment[key] = [segment[key][0] + delta[0], segment[key][1] + delta[1]]
        shift("to", 1)
        if segment["kind"] == "quad":
            shift("ctrl", 2)
        if segment["kind"] == "cubic":
            shift("ctrl1", 2)
            shift("ctrl2", 4)
    return out


def write(leaf, case, before, mutation, after, diff):
    folder = os.path.join(ROOT, leaf, case)
    for relative, value in [("📸️snapshot/⬅️before", before), ("🦠️mutation", mutation), ("📸️snapshot/➡️after", after), ("🔺️diff", diff), ("🎯️outcome", {"status": "applied"})]:
        path = os.path.join(folder, relative)
        os.makedirs(path, exist_ok=True)
        with open(os.path.join(path, "🔣️.json"), "w") as out:
            out.write(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def main():
    group = base_layer("group", "group-a", "Scaled Group", 100.0, 50.0, 2.0, 2.0)
    group["children"] = [rect("shape-b", "Beta", 10.0, 10.0)]
    before = document([rect("shape-a", "Alpha"), group])
    mutation = {"mutation": "dragLayers", "targets": ["shape-a", "shape-b"], "dx": 20.0, "dy": -10.0}
    write("✋️drag-layers", "✋️drags-a-child", before, mutation, *layer_case(before, mutation, lambda t, parent: dragged(t, parent, [mutation["dx"], mutation["dy"]])))

    before = document([rect("shape-a", "Alpha")])
    mutation = {"mutation": "rotateLayers", "targets": ["shape-a"], "pivotX": 10.0, "pivotY": 0.0, "angle": math.pi / 2}
    write("🧭️rotate-layers", "🧭️quarter-turn", before, mutation, *layer_case(before, mutation, lambda t, parent: moved(t, parent, rotation(mutation["pivotX"], mutation["pivotY"], mutation["angle"]))))

    before = document([rect("shape-a", "Alpha", 10.0, 20.0)])
    mutation = {"mutation": "scaleLayers", "targets": ["shape-a"], "pivotX": 10.0, "pivotY": 20.0, "scaleX": 2.0, "scaleY": 0.5}
    write("📐️scale-layers", "📐️doubles-a-rect", before, mutation, *layer_case(before, mutation, lambda t, parent: moved(t, parent, scaling(mutation["pivotX"], mutation["pivotY"], mutation["scaleX"], mutation["scaleY"]))))

    path = base_layer("path", "path-a", "Curve", scale_x=2.0)
    path["segments"] = [{"kind": "move", "to": [0.0, 0.0]}, {"kind": "cubic", "ctrl1": [10.0, 20.0], "ctrl2": [30.0, 20.0], "to": [40.0, 0.0]}]
    before = document([path])
    mutation = {"mutation": "dragPathPoints", "targets": [{"layerId": "path-a", "index": 0, "point": "anchor"}, {"layerId": "path-a", "index": 1, "point": "anchor"}], "dx": 20.0, "dy": 10.0}
    after = copy.deepcopy(before)
    world = to_matrix(path["transform"])
    basis = inverse([world[0], world[1], world[2], world[3], 0.0, 0.0])
    local = [basis[0] * mutation["dx"] + basis[2] * mutation["dy"], basis[1] * mutation["dx"] + basis[3] * mutation["dy"]]
    after["layers"][0]["segments"] = translate_points(path["segments"], [(target["index"], target["point"]) for target in mutation["targets"]], local)
    write("📍️drag-path-points", "📍️drags-two-anchors", before, mutation, after, diff_of([patch_entry("path-a", pathSegments=after["layers"][0]["segments"])]))


if __name__ == "__main__":
    main()
