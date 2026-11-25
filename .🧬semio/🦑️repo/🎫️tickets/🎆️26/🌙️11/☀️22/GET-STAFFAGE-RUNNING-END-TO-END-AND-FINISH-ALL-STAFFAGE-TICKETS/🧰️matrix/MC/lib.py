"""🧰️ Helpers of the ops writers of batch MC (`make/<id>.py`): read the catalog document of an asset and write `ops/<id>.json`, one op per line. A writer reads only what its own ops do not change, so writing and applying again gives the same file."""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def asset(slug):
    """📖️ The asset of the catalog document of an id, or of `<MC_SOURCE>/<id>.json` when that variable names a folder of copies (the proof of the chain reads the backups through it)."""
    source = os.environ.get("MC_SOURCE")
    return json.load(open(f"{source}/{slug}.json" if source else glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")[0]))["asset"]


def shape(a, scale, view, sid):
    """🔎️ A copy of a drawn shape by id."""
    return json.loads(json.dumps(next(s for s in a["representations"][str(scale)]["geometry"][view] if s["id"] == sid)))


def like(base, drop=(), **changes):
    """🧬️ A copy of a shape without the members named in `drop` and with the given members set."""
    out = {k: v for k, v in base.items() if k not in drop}
    out.update(changes)
    return out


GEOMETRY = ("kind", "x", "y", "w", "h", "r", "points", "closed", "d", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "followTip")


def write(slug, ops):
    """📤️ Writes the ops of an asset."""
    os.makedirs(f"{HERE}/ops", exist_ok=True)
    open(f"{HERE}/ops/{slug}.json", "w").write("[\n" + ",\n".join(json.dumps(o, ensure_ascii=False) for o in ops) + "\n]\n")
    print(f"{slug}: {len(ops)} ops written")


def has(a, scale, view, sid):
    """❓️ Whether a view of a band draws a shape of an id."""
    return any(s["id"] == sid for s in a["representations"][str(scale)]["geometry"][view])


def geometry(s):
    """📐️ The geometry members of a shape without its solid and its followTip."""
    return {k: v for k, v in s.items() if k in GEOMETRY and k not in ("solid", "followTip")}


def extruded(sweep, z0, z1, tip=None):
    """🧱️ An extruded solid hint with rounded positions."""
    out = {"kind": "extruded", "sweep": sweep, "z0": round(z0, 6), "z1": round(z1, 6)}
    if tip is not None:
        out["zTip"] = [round(tip[0], 6), round(tip[1], 6)]
    return out


def solid_of(target, profile, solid):
    """🧊️ The target shape with the geometry of a profile shape and a solid hint: paint, bone and order stay those of the target."""
    return like(target, drop=GEOMETRY, **geometry(profile), solid=solid)


def op(scale, view, sid, new):
    """🧷️ The op that replaces a drawn shape."""
    return {"op": "shape", "scale": str(scale), "view": view, "id": sid, "shape": new}


def scales(a):
    """🔢️ The authored bands of an asset."""
    return [k for k, r in a["representations"].items() if "geometry" in r]


def box(s):
    """📦️ The box [x0, x1, y0, y1] of the authored geometry of a poly, rect, capsule, ellipse or straight-line path shape."""
    if s["kind"] == "poly":
        pts = s["points"]
    elif s["kind"] == "rect":
        pts = [[s["x"], s["y"]], [s["x"] + s["w"], s["y"] + s["h"]]]
    elif s["kind"] == "capsule":
        r = max(s["r0"], s["r1"])
        pts = [[s["x0"] - r, s["y0"] - r], [s["x1"] + r, s["y1"] + r], [s["x0"] + r, s["y0"] + r], [s["x1"] - r, s["y1"] - r]]
    elif s["kind"] == "ellipse":
        pts = [[s["cx"] - s["rx"], s["cy"] - s["ry"]], [s["cx"] + s["rx"], s["cy"] + s["ry"]]]
    elif s["kind"] == "circle":
        pts = [[s["cx"] - s["r"], s["cy"] - s["r"]], [s["cx"] + s["r"], s["cy"] + s["r"]]]
    else:
        try:
            pts = [p for run in segments(s["d"]) for p in run]
        except ValueError:
            from mc import curved
            pts = curved(s["d"])
    return [min(p[0] for p in pts), max(p[0] for p in pts), min(p[1] for p in pts), max(p[1] for p in pts)]


def segments(d):
    """✂️ The runs of points of a path of straight lines (`M x y L x y … [Z]`)."""
    runs, tokens, i = [], d.split(), 0
    while i < len(tokens):
        if tokens[i] == "M":
            runs.append([[float(tokens[i + 1]), float(tokens[i + 2])]])
            i += 3
        elif tokens[i] == "L":
            runs[-1].append([float(tokens[i + 1]), float(tokens[i + 2])])
            i += 3
        elif tokens[i] in ("Z", "z"):
            i += 1
        else:
            raise ValueError(f"path command {tokens[i]} is not a straight line")
    return runs


def lateral(a, scale, view, sid):
    """↔️ The lateral span of a front or plan shape: the span of its solid when it is one already, else the lateral range of its flat outline."""
    s = shape(a, scale, view, sid)
    if "solid" in s:
        return s["solid"]["z0"], s["solid"]["z1"]
    b = box(s)
    return (b[0], b[1]) if view == "front" else (b[2], b[3])


HAIR = 0.003


def prism(a, scale, view, sid, twin=None, half=HAIR):
    """🧊️ The op that makes a front or plan shape the side elevation outline of its twin swept over the lateral span the shape has; a stroke without lateral width is swept over `half` each way, the true half width of the member it stands for, so that it stays a stroke as wide as the body solid."""
    z0, z1 = lateral(a, scale, view, sid)
    if z1 - z0 < 1e-6:
        z0, z1 = z0 - half, z1 + half
    return op(scale, view, sid, solid_of(shape(a, scale, view, sid), shape(a, scale, "elevation", twin or sid), extruded("w", z0, z1)))


def turned(a, scale, view, sid, bone, twin=None):
    """🏺️ The op that makes a front or plan shape the body of revolution of its side elevation twin about the axis of its bone."""
    return op(scale, view, sid, solid_of(shape(a, scale, view, sid), shape(a, scale, "elevation", twin or sid), {"kind": "revolved", "axisBone": bone, "axis": "u", "axisX": 0, "mirror": True}))


def slab(a, scale, sid, along, source=None):
    """🥞️ The ops that make the front and the plan shape of a part lying in the plan plane one solid: its plan outline (or that of the plan shape `source`, for a part the plan does not draw because another of the same outline covers it), re-expressed along the bone by `along` (plan x to bone u; the lateral position stays), swept along the bone normal over the thickness its side elevation twin draws. A wall drawn in the plan as a ring (a path of an outer and an inner run) keeps both runs in the plan and takes the outer run alone in the front, where the wall is seen from its end. A shape that is a solid already is kept as it is (a second run)."""
    plan = shape(a, scale, "plan", source or sid)
    b = box(shape(a, scale, "elevation", sid))
    solid = extruded("v", b[2], b[3])
    ops = []
    for view in ("front", "plan"):
        if not has(a, scale, view, sid):
            continue
        own = shape(a, scale, view, sid)
        if "solid" in own:
            ops.append(op(scale, view, sid, own))
            continue
        flat = geometry(plan)
        if "solid" in plan:
            outline = flat
        elif flat["kind"] == "poly":
            outline = {"kind": "poly", "points": [[round(along(x), 6), z] for x, z in flat["points"]], "closed": flat["closed"]}
        elif flat["kind"] == "path":
            runs = segments(flat["d"])
            if view == "front":
                runs = [max(runs, key=lambda run: (max(p[1] for p in run) - min(p[1] for p in run)) * (max(p[0] for p in run) - min(p[0] for p in run)))]
            outline = {"kind": "path", "d": " ".join("M " + " L ".join(f"{round(along(x), 6)} {z}" for x, z in run) + (" Z" if "Z" in flat["d"] else "") for run in runs)}
        else:
            raise ValueError(f"{sid}: plan kind {flat['kind']}")
        ops.append(op(scale, view, sid, like(own, drop=GEOMETRY, **outline, solid=solid)))
    return ops


def twins(a, scale, bones, filled=True):
    """👯️ The ids of the shapes of given bones that a band draws in all three views (filled ones only unless `filled` is False), in the order of the side elevation."""
    views = a["representations"][str(scale)]["geometry"]
    keep = lambda s: s["bone"] in bones and (s.get("filled") or not filled)
    front = {s["id"] for s in views["front"] if keep(s)}
    plan = {s["id"] for s in views["plan"] if keep(s)}
    return [s["id"] for s in views["elevation"] if keep(s) and s["id"] in front and s["id"] in plan]
