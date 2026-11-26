"""🍖️ Draws the body parts no view draws on their own bone: lbody.py <id>… writes `loft/<id>.lofts.json`, the ops of the second step of the chain (`../MX-2/loft.ts`: one loft shape per body solid in the three views), with one entry per authored scale and bone that carries plain body solids (loft, ellipsoid or capsule, no skin weights, no repeat) and has no shape in at least one view (the ham on the thigh, the shoulder on the scapula: the trunk outline drew them, and a trunk that became the body of its own solid no longer does). Where the paint shape stands in the side elevation only, the bone gets two entries, the views without it first (the tool reads the paint in the side elevation, which the second entry then replaces). Entries that borrow the paint of an ancestor stand before those that replace a shape. The entry replaces the filled body shapes the bone has in the other views and takes its paint from the body shape of the bone in the side elevation, else from a bone of the same chain (`tail-2` for `tail-1`), else from that of the nearest ancestor, else from the trunk. The loft ops of `loft/<id>.json` for those bones leave that list (the first step must not ask for a shape the second replaced). SCALES="50 100" limits the scales, BONES=<regular expression> the bones; FORCE=<regular expression> also takes bones that every view draws (a bone whose shape stands for several solids: the two barrels of a belly); FIT="<bone>:<stations> …" draws all body solids of a bone, of whatever kind, as ONE fitted loft with the id and the paint of its first drawn body shape (a trunk that is a polyhedron); FITS='{"<scale>:<bone>": [fit, …]}' states the fits of one scale and bone itself (two halves of an udder); EXTRA='{"<scale>:<bone>": {members}}' adds members to the entries of one scale and bone (`swell`)."""
import json, os, pathlib, re, sys
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
VIEWS = ("elevation", "front", "plan")
only = os.environ.get("BONES")
force = os.environ.get("FORCE")
fits = json.loads(os.environ.get("FITS", "{}"))
fit = {pair.split(":")[0]: int(pair.split(":")[1]) for pair in os.environ.get("FIT", "").split()}
for a in sys.argv[1:]:
    asset = json.load(open(GEN / "backup" / f"{a}.json"))["asset"]
    parent = {b["id"]: b.get("parent") for b in asset["rig"]["bones"]}
    entries = []
    for scale, rep in asset["representations"].items():
        volume = asset.get("volumes", {}).get(scale, {})
        if rep.get("status") != "authored" or volume.get("status") != "authored" or (os.environ.get("SCALES") and scale not in os.environ["SCALES"].split()):
            continue
        shapes = {v: rep["geometry"][v] for v in VIEWS}
        body = lambda bone, view: [s for s in shapes[view] if s["bone"] == bone and s.get("role") == "body" and s.get("filled") and s.get("closed", True) is not False]
        solids = {}
        for solid in volume["body"]:
            solids.setdefault(solid.get("bone"), []).append(solid)
        for bone, own in solids.items():
            if bone is None or (only and not re.fullmatch(only, bone)):
                continue
            if bone in fit:
                old = sorted({s["id"] for v in VIEWS for s in body(bone, v)})
                if old:
                    entries.append({"scale": scale, "bone": bone, "paint": old[0], "replace": old, "fit": fits.get(f"{scale}:{bone}", [{"id": old[0], "stations": fit[bone]}])})
                continue
            if any(s["kind"] not in ("loft", "ellipsoid", "capsule") or s.get("weights") or s.get("repeat") for s in own):
                continue
            if all(any(s["bone"] == bone for s in shapes[v]) for v in VIEWS) and not (force and re.fullmatch(force, bone)):
                continue
            old = [s["id"] for v in VIEWS for s in body(bone, v)]
            side = [s["id"] for s in body(bone, "elevation")]
            family = lambda name: "-".join(t for t in name.split("-") if not t.isdigit())
            kin = [s["id"] for s in shapes["elevation"] if s["bone"] != bone and family(s["bone"]) == family(bone) and s.get("role") == "body" and s.get("filled")]
            paint, cursor = (side[0] if side else kin[0] if kin else None), parent.get(bone)
            while paint is None and cursor is not None:
                found = body(cursor, "elevation")
                paint, cursor = (found[0]["id"] if found else None), parent.get(cursor)
            if paint is None:
                trunk = [s["id"] for s in shapes["elevation"] if s["bone"] in ("lumbar", "thorax", "trunk", "pelvis", "chest") and s.get("role") == "body" and s.get("filled")]
                paint = trunk[0] if trunk else None
            if paint is None:
                continue
            entry = {"scale": scale, "bone": bone, "paint": paint, **({"replace": sorted(set(old))} if old else {})}
            holds = [v for v in VIEWS if any(s["id"] == paint and s["bone"] == bone for s in shapes[v])]
            if holds and len(holds) < 3 and "elevation" in holds:
                entries.append({**entry, "views": [v for v in VIEWS if v not in holds]})
                entries.append({**entry, "views": holds})
            else:
                entries.append(entry)
    extra = json.loads(os.environ.get("EXTRA", "{}"))
    entries = [{**e, **extra.get(f"{e['scale']}:{e['bone']}", {})} for e in entries]
    entries.sort(key=lambda e: (int(e["scale"]), e["paint"] in e.get("replace", [])))
    (W / "loft" / f"{a}.lofts.json").write_text("[\n" + ",\n".join(" " + json.dumps(e, ensure_ascii=False) for e in entries) + "\n]\n")
    taken = {(e["scale"], e["bone"]) for e in entries}
    path = W / "loft" / f"{a}.json"
    ops = json.load(open(path))
    kept = [op for op in ops if (op.get("scale"), op.get("bone")) not in taken]
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in kept) + "\n]\n")
    print(a, len(entries), "body entries;", len(ops) - len(kept), "loft ops left the first step;", " ".join(sorted({e["bone"] for e in entries})))
