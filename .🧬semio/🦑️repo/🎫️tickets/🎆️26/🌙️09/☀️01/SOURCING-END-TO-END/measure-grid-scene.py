#!/usr/bin/env python3
"""📏️ Language-neutral size check for the grid window's World3d scene payload.

Replicates `box_mesh_spec`/`frame_mesh_spec`/`instance_json`/`grid_placement`/`grid_scale` from
`🧬️schema/🦀️component.rs` against the committed demo stock, and prints the byte size of the two
strings the scene carries. Written to settle whether `ui.fixed-capacity: fixed UI admission failed at
mesh-window.scene` is a payload OVERFLOW of `UI_FIXED_BYTES` (32 KiB) or a different encode failure —
`semio_framework_ui_scene::encode` maps every `SurfaceEncodeError` variant onto that one message.

Answer: 18,606 bytes for the ten-kind demo stock, ~57% of the cap. Not an overflow.
"""
import json, math, struct, os

f32 = lambda x: struct.unpack("f", struct.pack("f", x))[0]


def ryu_f32(x):
    """🔢️ Shortest decimal that round-trips through f32 — what serde_json emits for a `Vec<f32>`."""
    v = f32(x)
    for precision in range(1, 10):
        candidate = repr(round(v, precision))
        if f32(float(candidate)) == v:
            return candidate
    return repr(v)


def box_spec(width, height, depth):
    hw, hh, hd = f32(width * 0.5), f32(height * 0.5), f32(depth * 0.5)
    faces = [
        ((0, 0, 1), [(-hw, -hh, hd), (hw, -hh, hd), (hw, hh, hd), (-hw, hh, hd)]),
        ((0, 0, -1), [(hw, -hh, -hd), (-hw, -hh, -hd), (-hw, hh, -hd), (hw, hh, -hd)]),
        ((1, 0, 0), [(hw, -hh, hd), (hw, -hh, -hd), (hw, hh, -hd), (hw, hh, hd)]),
        ((-1, 0, 0), [(-hw, -hh, -hd), (-hw, -hh, hd), (-hw, hh, hd), (-hw, hh, -hd)]),
        ((0, 1, 0), [(-hw, hh, hd), (hw, hh, hd), (hw, hh, -hd), (-hw, hh, -hd)]),
        ((0, -1, 0), [(-hw, -hh, -hd), (hw, -hh, -hd), (hw, -hh, hd), (-hw, -hh, hd)]),
    ]
    positions, normals, indices = [], [], []
    for normal, corners in faces:
        base = len(positions) // 3
        for corner in corners:
            positions.extend(corner)
            normals.extend(normal)
        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3])
    return positions, normals, indices


def frame_spec(width, height, depth, profile):
    spec = ([], [], [])

    def add(piece_width, piece_height, cx, cy):
        positions, normals, indices = box_spec(piece_width, piece_height, depth)
        for i in range(0, len(positions), 3):
            positions[i] = f32(positions[i] + f32(cx))
            positions[i + 1] = f32(positions[i + 1] + f32(cy))
        offset = len(spec[0]) // 3
        spec[0].extend(positions)
        spec[1].extend(normals)
        spec[2].extend(index + offset for index in indices)

    half_h, half_w = height * 0.5, width * 0.5
    add(width, profile, 0.0, half_h - profile * 0.5)
    add(width, profile, 0.0, -half_h + profile * 0.5)
    stile_h = height - profile * 2.0
    add(profile, stile_h, -half_w + profile * 0.5, 0.0)
    add(profile, stile_h, half_w - profile * 0.5, 0.0)
    return spec


def spec_for(geometry):
    kind = geometry["kind"]
    if kind == "box":
        return box_spec(geometry["width"], geometry["height"], geometry["depth"])
    if kind == "slab":
        return box_spec(geometry["width"], geometry["thickness"], geometry["depth"])
    if kind == "frame":
        return frame_spec(geometry["width"], geometry["height"], geometry["depth"], geometry["profile"])
    raise SystemExit(f"unhandled geometry recipe {kind}")


def bounding_extent(geometry):
    kind = geometry["kind"]
    if kind == "slab":
        return max(geometry["width"], geometry["depth"], geometry["thickness"])
    return max(geometry["width"], geometry["height"], geometry["depth"])


CELL = 2.0
UI_FIXED_BYTES = 32 * 1024
STOCK = os.path.join(
    "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any",
    "📚️examples/🎬️demo/🧪️expected-stock.json",
)

stock = json.load(open(STOCK, encoding="utf8"))
meshes = []
for kind in stock:
    positions, normals, indices = spec_for(kind["geometry"])
    numbers = lambda values: "[" + ",".join(ryu_f32(value) for value in values) + "]"
    data = f'{{"positions":{numbers(positions)},"normals":{numbers(normals)},"colors":[],"indices":[{",".join(str(i) for i in indices)}]}}'
    meshes.append(f'{{"id":{json.dumps(kind["id"])},"data":{data}}}')
    print(f"{kind['id']:28s} verts={len(positions) // 3:4d} bytes={len(meshes[-1]):7d}")

columns = max(1, math.ceil(math.sqrt(len(stock))))
rows = math.ceil(len(stock) / columns)
instances = []
for index, kind in enumerate(stock):
    x = (index % columns - (columns - 1) * 0.5) * CELL
    z = (index // columns - (rows - 1) * 0.5) * CELL
    scale = (CELL * 0.8) / bounding_extent(kind["geometry"])
    instances.append(json.dumps(
        {"id": kind["id"], "meshId": kind["id"], "position": [x, 0.0, z], "rotation": [0.0, 0.0, 0.0, 1.0],
         "scale": [scale, scale, scale], "label": kind["name"], "selected": False, "hovered": False},
        separators=(",", ":"), ensure_ascii=False))

meshes_json = "[" + ",".join(meshes) + "]"
instances_json = "[" + ",".join(instances) + "]"
total = len(meshes_json) + len(instances_json)
print(f"\nmeshes_json    {len(meshes_json):7d} bytes")
print(f"instances_json {len(instances_json):7d} bytes")
print(f"TOTAL          {total:7d} bytes  ({total / UI_FIXED_BYTES:.0%} of UI_FIXED_BYTES = {UI_FIXED_BYTES})")
