#!/usr/bin/env python3
"""💠️ An INDEPENDENT second implementation of the `s.lowpoly.lowpoly` document and its seventeen typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `lowpoly` document has exactly two
top-level members — `schema` and an id-keyed `objects` list — and every object carries an
INDEX-keyed, anonymous `paintLayers` stack whose pixels are a base64 byte buffer edited by OFFSET
RUNS. Two levels of addressing, one by id and one by index, in one vocabulary. No mesh or scene
library models a paint stack addressed by index inside an object addressed by id, and none of them
reads `.dsl.semio`. That this algebra IS adjudicable was settled in this same wave by
`mutate-fem3d-1` and `🏔️mutate-gisterrain-1`, which took Python second implementations over this same
carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the two members of
  `LowpolySnapshot` and the shape of an object.
* rules 2, 3 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  id-keyed object collection, the INDEX-keyed paint-layer collection with `insert`/`remove`, and
  absolute `move`/`rotate`/`scale`.
* the seventeen committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and their
  argument lists and for the three things only they state: that this subset tags its mutations
  EXTERNALLY — the payload is `{"MoveObject": {…}}`, a PascalCase variant name as the single key,
  where every sibling subset in this repository tags internally with a `"mutation"` member; that
  `edit-paint-layer` splices base64 RUNS into the layer's pixel buffer at byte offsets, overwriting in
  place and never resizing it; and that `create-mesh` carries a `meshWorkspace` argument which, when
  non-empty, becomes the object's persisted `meshContent` — the half-edge-mesh JSON the handle hashes —
  while `delete-mesh` clears both, so each undo carries the content back.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only. (The 2026-09-25 `meshContent` alignment followed the committed `create-mesh` after-snapshot;
that `delete-mesh` clears it was read off the subject's leaf, because no committed vector shows it.) All seventeen kinds are adjudicated and none is refused: the mesh child handle carries the
caller's own `childId`, so nothing here depends on a content-addressing function no specification
states.
"""

# region 🔖️Imports
import base64
import collections
import copy
import ctypes
import ctypes.util
import hashlib
import json
import math

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "objects")
"""🗂️ The two members `LowpolySnapshot` declares — and the cross-language projection."""

OBJECT_MEMBERS = {"id", "name", "transform", "smoothShading", "mesh", "paintLayers", "meshContent"}
"""🧊 The members each object carries, as the committed vectors spell them — `meshContent` being the
persisted half-edge-mesh JSON the `mesh` handle hashes."""

LAYER_MEMBERS = {"name", "visible", "opacity", "blendMode", "pixels"}
"""🎨 The members each paint layer carries. It has no id: the stack is addressed by INDEX."""

OBJECT_FIELDS = {"rename-object": ("name", "newName"), "change-object-smooth-shading": ("smoothShading", "newSmoothShading")}
"""✏️ The two single-field object setters."""

TRANSFORM_FIELDS = {"move-object": ("position", "newPosition"), "rotate-object": ("rotation", "newRotation"), "scale-object": ("scale", "newScale")}
"""📍 The three placement setters, each writing one member of the object's transform."""

LAYER_FIELDS = {"rename-paint-layer": ("name", "newName"), "change-paint-layer-visible": ("visible", "newVisible"), "change-paint-layer-opacity": ("opacity", "newOpacity"), "change-paint-layer-blend-mode": ("blendMode", "newBlendMode")}
"""🖌️ The four single-field paint-layer setters, all addressed by `(objectId, index)`."""

KINDS = (
    "create-object",
    "delete-object",
    "reorder-objects",
    "rename-object",
    "change-object-smooth-shading",
    "move-object",
    "rotate-object",
    "scale-object",
    "create-mesh",
    "delete-mesh",
    "insert-paint-layer",
    "remove-paint-layer",
    "rename-paint-layer",
    "change-paint-layer-visible",
    "change-paint-layer-opacity",
    "change-paint-layer-blend-mode",
    "edit-paint-layer",
    "apply-paint-stroke",
    "move-selection",
    "rotate-selection",
    "scale-selection",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def variant_of(kind):
    """🔤️ The EXTERNALLY tagged variant name of a kind — PascalCase of its words, and the single key
    the committed payload carries."""
    return "".join(word[:1].upper() + word[1:] for word in kind.split("-"))


VARIANTS = {kind: variant_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: two members, unique object
    ids, complete object and layer records, and every layer's pixels a decodable base64 buffer."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a lowpoly document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    identifiers = []
    for record in document["objects"]:
        if set(record) != OBJECT_MEMBERS:
            raise AssertionError("%s: an object must carry exactly %r, found %r" % (where, sorted(OBJECT_MEMBERS), sorted(record)))
        if set(record["transform"]) != {"position", "rotation", "scale"}:
            raise AssertionError("%s: object %r has no position/rotation/scale transform" % (where, record["id"]))
        if record["mesh"] is not None and set(record["mesh"]) != {"childId", "target"}:
            raise AssertionError("%s: object %r carries a malformed mesh child handle" % (where, record["id"]))
        identifiers.append(record["id"])
        for at, layer in enumerate(record["paintLayers"]):
            if set(layer) != LAYER_MEMBERS:
                raise AssertionError("%s: paint layer %d of object %r must carry exactly %r, found %r" % (where, at, record["id"], sorted(LAYER_MEMBERS), sorted(layer)))
            pixels(layer, "%s: paint layer %d of object %r" % (where, at, record["id"]))
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: objects carries a duplicate id in %r" % (where, identifiers))


def pixels(layer, where):
    """🎨 A layer's pixel buffer, decoded from its base64 spelling."""
    try:
        return base64.b64decode(layer["pixels"], validate=True)
    except Exception as error:
        raise AssertionError("%s: pixels is not base64 (%s)" % (where, error))


def stroked(buffer, payload):
    """🖌️ `apply-paint-stroke`'s pixels: every dab of the stroke stamped in order onto a copy of the square RGBA
    buffer, in float32 — the centre is the UV point (v up) rounded half away from zero onto the pixel grid, every
    pixel within the radius (at least half a pixel) gains `round((hardness + (1 - hardness)·(1 - dist/radius)) ·
    opacity · 255)` of alpha under the brush colour (its sRGB unit channels scaled to bytes, rounded half away from
    zero), or loses it under the eraser, saturating either way."""
    f32 = numpy.float32
    away = lambda value: math.floor(float(value) + 0.5) if float(value) >= 0 else -math.floor(-float(value) + 0.5)
    unit = lambda value: min(max(f32(value), f32(0.0)), f32(1.0))
    held = bytearray(buffer)
    side = math.isqrt(len(held) // 4)
    if side * side * 4 != len(held) or not payload["points"] or not payload["radius"] > 0 or not 0 <= payload["hardness"] <= 1 or not 0 <= payload["opacity"] <= 1 or any(not 0 <= value <= 1 for point in payload["points"] for value in point) or any(not 0 <= channel <= 1 for channel in payload["color"]):
        raise AssertionError("apply-paint-stroke: the layer is not a square texture, or the brush cannot dab, or a dab lies off the texture")
    size, radius = f32(side), max(f32(payload["radius"]), f32(0.5))
    hard, alpha = unit(payload["hardness"]), unit(payload["opacity"])
    color = bytes(int(min(max(away(unit(channel) * f32(255.0)), 0), 255)) for channel in payload["color"])
    for u, v in payload["points"]:
        cx, cy = away(unit(u) * (size - f32(1.0))), away((f32(1.0) - unit(v)) * (size - f32(1.0)))
        reach = math.ceil(float(radius))
        for y in range(cy - reach, cy + reach + 1):
            for x in range(cx - reach, cx + reach + 1):
                if not (0 <= x < side and 0 <= y < side):
                    continue
                dx, dy = f32(x - cx), f32(y - cy)
                dist = numpy.sqrt(dx * dx + dy * dy, dtype=f32)
                if dist > radius:
                    continue
                amount = int(min(max(away((hard + (f32(1.0) - hard) * (f32(1.0) - dist / radius)) * alpha * f32(255.0)), 0), 255))
                at = (y * side + x) * 4
                if payload["eraser"]:
                    held[at + 3] = max(held[at + 3] - amount, 0)
                else:
                    held[at:at + 3] = color
                    held[at + 3] = min(held[at + 3] + amount, 255)
    return bytes(held)


def selection_number(value):
    """✍️ One float32 as the platform's JSON writer prints it inside a mesh: shortest round-trip digits, fixed
    notation for a decimal exponent within -5..=15 (a whole number keeps `.0`), exponential otherwise."""
    v = numpy.float32(value)
    if v == 0:
        return "-0.0" if numpy.signbit(v) else "0.0"
    mantissa, exponent = numpy.format_float_scientific(numpy.abs(v), unique=True, trim="-").split("e")
    exponent, digits, sign = int(exponent), mantissa.replace(".", ""), "-" if v < 0 else ""
    if -5 <= exponent <= 15:
        if exponent >= len(digits) - 1:
            return sign + digits + "0" * (exponent - (len(digits) - 1)) + ".0"
        if exponent >= 0:
            return sign + digits[:exponent + 1] + "." + digits[exponent + 1:]
        return sign + "0." + "0" * (-exponent - 1) + digits
    return sign + digits[0] + ("." + digits[1:] if len(digits) > 1 else "") + "e" + ("+" if exponent >= 0 else "") + str(exponent)


def selection_json(value):
    """✍️ The platform's compact JSON of a half-edge mesh value tree, members in declaration order."""
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return selection_number(value)
    if isinstance(value, list):
        return "[" + ",".join(selection_json(item) for item in value) + "]"
    return "{" + ",".join(json.dumps(key) + ":" + selection_json(item) for key, item in value.items()) + "}"


def selection_moved(content, payload, kind):
    """🧲️ A selection leaf's mesh: every named vertex (every vertex when none is named) carried by the motion in float64
    off its float32 position and stored back as float32 — an offset, a right-handed turn about the normalised axis
    through the pivot, or per-axis factors about the pivot — then every vertex normal recomputed as the normalised
    Newell normal of the first face visiting it. Libm's `hypot`, `cos` and `sin`, the platform's own."""
    f32 = lambda value: float(numpy.float32(value))
    libm = ctypes.CDLL(ctypes.util.find_library("m"))
    for name, arity in (("hypot", 2), ("cos", 1), ("sin", 1)):
        getattr(libm, name).restype = ctypes.c_double
        getattr(libm, name).argtypes = [ctypes.c_double] * arity
    mesh = json.loads(content, object_pairs_hook=collections.OrderedDict)
    count = len(mesh["vertices"])
    present = sorted({vertex for vertex in (payload["vertexIds"] or range(count)) if vertex < count})
    if kind == "move-selection":
        transform = lambda point: [point[axis] + float(payload["offset"][axis]) for axis in range(3)]
    elif kind == "rotate-selection":
        pivot, axis = [float(value) for value in payload["pivot"]], [float(value) for value in payload["axis"]]
        length = libm.hypot(libm.hypot(axis[0], axis[1]), axis[2])
        x, y, z = (value / length for value in axis)
        c, s = libm.cos(float(payload["angle"])), libm.sin(float(payload["angle"]))
        t = 1.0 - c

        def transform(point):
            px, py, pz = (point[index] - pivot[index] for index in range(3))
            return [pivot[0] + ((t * x * x + c) * px + (t * x * y - s * z) * py + (t * x * z + s * y) * pz), pivot[1] + ((t * x * y + s * z) * px + (t * y * y + c) * py + (t * y * z - s * x) * pz), pivot[2] + ((t * x * z - s * y) * px + (t * y * z + s * x) * py + (t * z * z + c) * pz)]
    else:
        pivot, factor = [float(value) for value in payload["pivot"]], [float(value) for value in payload["factor"]]
        transform = lambda point: [pivot[axis] + (point[axis] - pivot[axis]) * factor[axis] for axis in range(3)]
    for vertex in present:
        mesh["vertices"][vertex]["position"] = [f32(value) for value in transform([float(value) for value in mesh["vertices"][vertex]["position"]])]
    normalised = lambda vector, length: [0.0, 0.0, 0.0] if length == 0.0 else [f32(float(value) / length) for value in vector]
    flat = [None] * count
    for face in mesh["faces"]:
        ids, edge = [], face["halfedge"]
        while True:
            ids.append(mesh["halfedges"][edge]["vertex"])
            edge = mesh["halfedges"][edge]["next"]
            if edge == face["halfedge"]:
                break
        points = [mesh["vertices"][vertex]["position"] for vertex in ids]
        origin = points[0]
        nx = ny = nz = 0.0
        for index, a in enumerate(points):
            b = points[(index + 1) % len(points)]
            ax, ay, az = (float(a[k]) - float(origin[k]) for k in range(3))
            bx, by, bz = (float(b[k]) - float(origin[k]) for k in range(3))
            nx += (ay - by) * (az + bz)
            ny += (az - bz) * (ax + bx)
            nz += (ax - bx) * (ay + by)
        newell = normalised([nx, ny, nz], libm.hypot(libm.hypot(nx, ny), nz))
        normal = normalised(newell, libm.hypot(libm.hypot(newell[0], newell[1]), newell[2]))
        for vertex in ids:
            if flat[vertex] is None:
                flat[vertex] = normal
    for vertex, normal in zip(mesh["vertices"], flat):
        vertex["normal"] = normal
    return selection_json(mesh)


def selection_handle(object_id, content):
    """🆔️ The content-addressed mesh child handle the persisted content hashes to: `mesh-` + the first 16 lowercase hex
    digits of SHA-256 over the content, aimed at the object's `<id>-mesh` child in the `s.stdio.semio` v1 mesh subset."""
    return {"childId": "mesh-" + hashlib.sha256(content.encode("utf-8")).hexdigest()[:16], "target": {"artifactId": object_id + "-mesh", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "mesh"}}}


def object_at(document, identity, kind, where):
    """🔎️ The index of the object this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(document["objects"]):
        if record["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses object %r, which the before-snapshot does not hold" % (where, kind, identity))


def layer_at(record, index, kind, where):
    """🔎️ One paint layer by INDEX, held to the stack's real bounds."""
    if not 0 <= index < len(record["paintLayers"]):
        raise AssertionError("%s-%s: the committed vector addresses paint layer %d of object %r, whose stack holds %d" % (where, kind, index, record["id"], len(record["paintLayers"])))
    return record["paintLayers"][index]
# endregion 🔖️Document


# region 🔖️Verbs
def spliced(buffer, runs, where):
    """✂️ `edit-paint-layer` overwrites base64 RUNS into the pixel buffer at byte offsets, in the order
    the payload lists them, and never resizes it — a run that would run past the end is an error, not
    a growth."""
    held = bytearray(buffer)
    for run in runs:
        chunk = base64.b64decode(run["bytes"], validate=True)
        offset = run["offset"]
        if offset < 0 or offset + len(chunk) > len(held):
            raise AssertionError("%s: a run of %d bytes at offset %d does not fit a %d-byte layer" % (where, len(chunk), offset, len(held)))
        held[offset:offset + len(chunk)] = chunk
    return bytes(held)


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    if kind == "create-object":
        index = payload.get("index")
        document["objects"].insert(len(document["objects"]) if index is None else index, copy.deepcopy(payload["object"]))
    elif kind == "delete-object":
        document["objects"].pop(object_at(document, payload["id"], kind, "mutate"))
    elif kind == "reorder-objects":
        at = object_at(document, payload["id"], kind, "mutate")
        record = document["objects"].pop(at)
        document["objects"].insert(payload["toIndex"], record)
    elif kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        document["objects"][object_at(document, payload["id"], kind, "mutate")][member] = payload[argument]
    elif kind in TRANSFORM_FIELDS:
        member, argument = TRANSFORM_FIELDS[kind]
        document["objects"][object_at(document, payload["id"], kind, "mutate")]["transform"][member] = copy.deepcopy(payload[argument])
    elif kind == "create-mesh":
        record = document["objects"][object_at(document, payload["id"], kind, "mutate")]
        record["mesh"] = {"childId": payload["childId"], "target": copy.deepcopy(payload["target"])}
        if payload["meshWorkspace"]:
            record["meshContent"] = payload["meshWorkspace"]
    elif kind == "delete-mesh":
        record = document["objects"][object_at(document, payload["id"], kind, "mutate")]
        record["mesh"], record["meshContent"] = None, ""
    elif kind == "insert-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        index = payload.get("index")
        record["paintLayers"].insert(len(record["paintLayers"]) if index is None else index, copy.deepcopy(payload["layer"]))
    elif kind == "remove-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer_at(record, payload["index"], kind, "mutate")
        record["paintLayers"].pop(payload["index"])
    elif kind in LAYER_FIELDS:
        member, argument = LAYER_FIELDS[kind]
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer_at(record, payload["index"], kind, "mutate")[member] = payload[argument]
    elif kind == "edit-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer = layer_at(record, payload["layerIndex"], kind, "mutate")
        where = "mutate-%s: layer %d of object %r" % (kind, payload["layerIndex"], payload["objectId"])
        layer["pixels"] = base64.b64encode(spliced(pixels(layer, where), payload["runs"], where)).decode("ascii")
    elif kind == "apply-paint-stroke":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer = layer_at(record, payload["layerIndex"], kind, "mutate")
        layer["pixels"] = base64.b64encode(stroked(pixels(layer, "mutate-%s" % kind), payload)).decode("ascii")
    elif kind in ("move-selection", "rotate-selection", "scale-selection"):
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        if record["mesh"] is None or not record["meshContent"]:
            raise AssertionError("mutate-%s: object %r carries no mesh" % (kind, payload["objectId"]))
        record["meshContent"] = selection_moved(record["meshContent"], payload, kind)
        record["mesh"] = selection_handle(payload["objectId"], record["meshContent"])
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `edit-paint-layer` inverts by capturing the SAME byte ranges out of the
    pre-mutation buffer, which is exact because the verb never resizes the layer."""
    if kind == "create-object":
        return [("delete-object", {"id": payload["object"]["id"]})]
    if kind == "delete-object":
        at = object_at(document, payload["id"], kind, "inverse")
        return [("create-object", {"object": copy.deepcopy(document["objects"][at]), "index": at})]
    if kind == "reorder-objects":
        return [(kind, {"id": payload["id"], "toIndex": object_at(document, payload["id"], kind, "inverse")})]
    if kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["objects"][object_at(document, payload["id"], kind, "inverse")][member]})]
    if kind in TRANSFORM_FIELDS:
        member, argument = TRANSFORM_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["objects"][object_at(document, payload["id"], kind, "inverse")]["transform"][member])})]
    if kind in ("create-mesh", "delete-mesh"):
        record = document["objects"][object_at(document, payload["id"], kind, "inverse")]
        held = record["mesh"]
        if held is None:
            return [("delete-mesh", {"id": payload["id"]})] if kind == "create-mesh" else []
        return [("create-mesh", {"id": payload["id"], "childId": held["childId"], "target": copy.deepcopy(held["target"]), "meshWorkspace": record["meshContent"]})]
    if kind == "insert-paint-layer":
        index = payload.get("index")
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("remove-paint-layer", {"objectId": payload["objectId"], "index": len(record["paintLayers"]) if index is None else index})]
    if kind == "remove-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("insert-paint-layer", {"objectId": payload["objectId"], "index": payload["index"], "layer": copy.deepcopy(layer_at(record, payload["index"], kind, "inverse"))})]
    if kind in LAYER_FIELDS:
        member, argument = LAYER_FIELDS[kind]
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [(kind, {"objectId": payload["objectId"], "index": payload["index"], argument: layer_at(record, payload["index"], kind, "inverse")[member]})]
    if kind == "edit-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        layer = layer_at(record, payload["layerIndex"], kind, "inverse")
        buffer = pixels(layer, "inverse-%s" % kind)
        runs = []
        for run in payload["runs"]:
            length = len(base64.b64decode(run["bytes"], validate=True))
            runs.append({"offset": run["offset"], "bytes": base64.b64encode(buffer[run["offset"]:run["offset"] + length]).decode("ascii")})
        return [(kind, {"objectId": payload["objectId"], "layerIndex": payload["layerIndex"], "runs": list(reversed(runs))})]
    if kind == "apply-paint-stroke":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        buffer = pixels(layer_at(record, payload["layerIndex"], kind, "inverse"), "inverse-%s" % kind)
        painted = stroked(buffer, payload)
        runs = []
        pixel = 0
        while pixel * 4 < len(buffer):
            if buffer[pixel * 4:pixel * 4 + 4] == painted[pixel * 4:pixel * 4 + 4]:
                pixel += 1
                continue
            start = pixel
            while pixel * 4 < len(buffer) and buffer[pixel * 4:pixel * 4 + 4] != painted[pixel * 4:pixel * 4 + 4]:
                pixel += 1
            runs.append({"offset": start * 4, "bytes": base64.b64encode(buffer[start * 4:pixel * 4]).decode("ascii")})
        return [("edit-paint-layer", {"objectId": payload["objectId"], "layerIndex": payload["layerIndex"], "runs": runs})]
    if kind in ("move-selection", "rotate-selection", "scale-selection"):
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("create-mesh", {"id": payload["objectId"], "childId": record["mesh"]["childId"], "target": copy.deepcopy(record["mesh"]["target"]), "meshWorkspace": record["meshContent"]})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after):
    """👁️ Every committed vector of this subset declares `status: applied`, so every one must move
    the compared projection."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member, index for index and byte for byte."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:400], json.dumps(original[member], sort_keys=True)[:400]))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def leaf(ctx, spec, name):
    """🧫️ One committed leaf of the vector the doc string addresses."""
    return json.loads(ctx.fixture_bytes(spec[name]).decode("utf-8"))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def payload_of(spec_mutation, kind):
    """🦠️ The committed payload. This subset tags EXTERNALLY: the whole record is
    `{"<Variant>": {arguments}}`, so the discriminator is the single key rather than a member."""
    if list(spec_mutation) != [VARIANTS[kind]]:
        raise AssertionError("mutate-%s: the committed vector carries %r, not a single %r arm" % (kind, sorted(spec_mutation), VARIANTS[kind]))
    return spec_mutation[VARIANTS[kind]]


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status and observability."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        outcome = leaf(ctx, spec, "outcome")
        if outcome.get("status") != "applied":
            raise AssertionError("mutate-%s: the committed outcome declares %r; this feature replays applied vectors only" % (kind, outcome.get("status")))
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(leaf(ctx, spec, "mutation"), kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back — the full inverse law, which the subject half of this case cannot assert because it never
    applies anything."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        payload = payload_of(leaf(ctx, spec, "mutation"), kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the committed document and answers with the whole of it. This implementation
    additionally requires, in role, that it really is the two-level document this case describes: two
    objects, one of them carrying a mesh child handle and a paint stack whose pixels decode to a real
    byte buffer, the other carrying neither."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    if len(document["objects"]) < 2:
        raise AssertionError("identity-round-trip: the committed document must carry two objects, found %d" % len(document["objects"]))
    if not any(record["mesh"] is not None and record["paintLayers"] for record in document["objects"]):
        raise AssertionError("identity-round-trip: no object carries both a mesh child handle and a paint stack")
    if not any(record["mesh"] is None and not record["paintLayers"] for record in document["objects"]):
        raise AssertionError("identity-round-trip: no object carries neither a mesh child handle nor a paint stack")
    reserialized = json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    if reserialized == committed:
        raise AssertionError("identity-round-trip: the committed file is pretty-printed and this writer is compact, so reproducing its bytes exactly would mean the handler returned the input unread")
    reparsed = json.loads(reserialized.decode("utf-8"))
    if reparsed != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return Outcome(reparsed, raw=reserialized)
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration
