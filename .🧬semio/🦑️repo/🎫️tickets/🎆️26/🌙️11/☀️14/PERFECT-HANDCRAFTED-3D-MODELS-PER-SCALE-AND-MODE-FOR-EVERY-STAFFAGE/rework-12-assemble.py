"""🧩 Author 12: carries the hand statements of a tree (`rework-12-kit.py`) into a candidate asset document: rig, anchors, dimensions, poses, clips, thermal areas, the four volumes and the drawing plan for `rework-12-draw2d.ts`.

Usage inside `rework-12-<slug>-model.py`: `assemble(tree, cfg)` writes `🗑️generated/rework-12/<slug>/pre.json` and `plan.json`.
"""
import copy
import json
import math
import os
from importlib.machinery import SourceFileLoader

K = SourceFileLoader("rework12kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-12-kit.py")).load_module()

LOAD_WORDS = {"wind": ("Drag on the foliage of the", "Windwiderstand am Laub:"), "snow": ("Snow on the foliage of the", "Schnee auf dem Laub:"), "carried": ("Fruit carried by the", "Fruchtlast:")}
LEVEL_LIMIT = {0: 3.0, 1: 6.0, 2: 9.0, 3: 14.0}
JOINT_LEVEL = {50: 9, 100: 2, 200: 1, 500: 0}


def low(text):
    return text[0].lower() + text[1:]


def de_num(value, digits=2):
    return f"{value:.{digits}f}".replace(".", ",")


def assemble(tree, cfg):
    slug = tree.slug
    path = os.path.join(K.CATALOG, slug, "🔣️.json")
    doc = json.load(open(path))
    old = doc["asset"]
    asset = copy.deepcopy(old)
    rig = tree.solve()
    old_bones = {b["id"]: b for b in old["rig"]["bones"]}
    like = {b["id"]: cfg.get("like", {}).get(b["id"], b["id"] if b["id"] in old_bones else None) for b in rig}
    built = {50: tree.build(50)}
    wood50, leaves50 = built[50]
    fine = [p for s in wood50 + leaves50 for p in s["mesh"].v]
    box = (tuple(min(p[k] for p in fine) for k in range(3)), tuple(max(p[k] for p in fine) for k in range(3)))
    for scale in K.SCALES[1:]:
        built[scale] = tree.build(scale, box=box)
    for group in (wood50, leaves50):
        for s in group:
            if not K.closed(s["mesh"]):
                raise SystemExit(f"open shell {s['id']}")
    # masses and centres per bone
    template_mass = next(b["mass"] for b in old["rig"]["bones"] if "mass" in b)
    template_section = next(b["section"] for b in old["rig"]["bones"] if "section" in b)
    leaf_density = cfg.get("leaf_density", 0.5)
    weight = {b["id"]: [0.0, (0.0, 0.0, 0.0)] for b in rig}
    leaf_area = {b["id"]: 0.0 for b in rig}
    leaf_centre = {b["id"]: (0.0, 0.0, 0.0) for b in rig}

    def put(bone, kg, at):
        total, c = weight[bone]
        weight[bone] = [total + kg, K.add(c, K.mul(at, kg))]

    for s in wood50:
        put(s["bone"], abs(s["mesh"].volume()) * tree.density, s["mesh"].centroid())
    for s in leaves50:
        area = s["mesh"].area()
        put(s["bone"], area * leaf_density, s["mesh"].centroid())
        leaf_centre[s["bone"]] = K.add(leaf_centre[s["bone"]], K.mul(s["mesh"].centroid(), area))
        leaf_area[s["bone"]] += area
    # poses and clips: offsets from the old sagittal statements
    droop = set(cfg.get("droop_poses", ()))

    def offsets_of(values, mode):
        out = {}
        for b in rig:
            source = like[b["id"]]
            if b["parent"] is None or b.get("fixed") or source is None or source not in values:
                continue
            a = values[source].get("angleDeg", 0)
            if abs(a) < 1e-9:
                continue
            offset = tree.offsets(b["id"], mode, a)
            if abs(offset["angleDeg"]) >= 0.01:
                out[b["id"]] = offset
        return out

    def peaked(id, frames):
        """🎚️ The statements of one pose or clip scaled so that their largest turn is `cfg["peaks"][id]` degrees (stated by the author of the tree; the same on every rebuild)."""
        peak = cfg.get("peaks", {}).get(id)
        top = max((abs(v.get("angleDeg", 0)) for frame in frames for v in frame.values()), default=0.0)
        if peak is None or top < 1e-9:
            return frames
        return [{bone: dict(v, angleDeg=v.get("angleDeg", 0) * peak / top) for bone, v in frame.items()} for frame in frames]

    reach = {b["id"]: 0.0 for b in rig}

    def note(offsets):
        for bone, o in offsets.items():
            reach[bone] = max(reach[bone], abs(o["angleDeg"]))

    total_area = sum(leaf_area.values()) or 1.0
    bone_by_id = {b["id"]: b for b in rig}
    remap = cfg.get("old_to_new", {})
    for pose in asset["poses"]:
        mode = "droop" if pose["id"] in droop else "lee"
        pose["bones"] = {} if pose["id"] == asset["presentation"]["defaultPose"] else offsets_of(peaked(pose["id"], [pose.get("bones", {})])[0], mode)
        note(pose["bones"])
        loads, kinds = [], {}
        for load in pose.get("loads", []):
            if "distributed" in load:
                target = remap.get(load["distributed"]["bone"], load["distributed"]["bone"])
                if target in bone_by_id:
                    load = copy.deepcopy(load)
                    load["distributed"]["bone"] = target
                    load["distributed"]["toM"] = min(load["distributed"]["toM"], bone_by_id[target]["lengthM"])
                    load["distributed"]["fromM"] = min(load["distributed"]["fromM"], load["distributed"]["toM"])
                    loads.append(load)
                continue
            entry = kinds.setdefault(load["kind"], {"fx": 0.0, "fy": 0.0, "template": load})
            entry["fx"] += load.get("fxN", 0)
            entry["fy"] += load.get("fyN", 0)
        for kind, entry in kinds.items():
            words = LOAD_WORDS.get(kind, ("Load on the", "Last:"))
            for b in rig:
                if leaf_area[b["id"]] <= 0:
                    continue
                share = leaf_area[b["id"]] / total_area
                at = tree.local(b["id"], K.mul(leaf_centre[b["id"]], 1 / leaf_area[b["id"]]))
                new = {k: v for k, v in entry["template"].items() if k not in ("id", "title", "bone", "x", "y", "z", "fxN", "fyN")}
                item = {"id": f"{kind}-{b['id']}", "kind": kind, "title": {"en": f"{words[0]} {low(b['title']['en'])}", "de": f"{words[1]} {b['title']['de']}"}, "bone": b["id"], "x": round(max(0.0, min(b["lengthM"], at[0])), 3), "y": 0, "fxN": round(entry["fx"] * share, 1), "fyN": round(entry["fy"] * share, 1)}
                item.update({k: v for k, v in new.items() if k != "kind"})
                loads.insert(len([l for l in loads if "distributed" not in l]), item)
        if "loads" in pose:
            pose["loads"] = loads
    load_ids = {}
    for pose in asset["poses"]:
        for load in pose.get("loads", []):
            load_ids.setdefault(load["kind"], [])
            if load["id"] not in load_ids[load["kind"]]:
                load_ids[load["kind"]].append(load["id"])
    old_kind = {load["id"]: load["kind"] for pose in old["poses"] for load in pose.get("loads", [])}
    for clip in asset["clips"]:
        tracks, done = [], set()
        for track in clip.get("tracks", []):
            if not track["target"].startswith("load:"):
                tracks.append(track)
                continue
            kind = old_kind.get(track["target"][5:])
            if kind is None or kind in done:
                continue
            done.add(kind)
            tracks.extend(dict(track, target=f"load:{lid}") for lid in load_ids.get(kind, []))
        if "tracks" in clip:
            clip["tracks"] = tracks
        mode = "droop" if clip["id"] in cfg.get("droop_clips", ()) else "lee"
        moving = [key for key in clip["keys"] if "bones" in key]
        for key, frame in zip(moving, peaked(clip["id"], [key["bones"] for key in moving])):
            key["bones"] = offsets_of(frame, mode)
            note(key["bones"])
    # rig
    bones = []
    for b in rig:
        entry = {"id": b["id"], "parent": b["parent"], "title": b["title"], "x": b["x"], "y": b["y"]}
        if abs(b["z"]) > 1e-9:
            entry["z"] = b["z"]
        entry["angleDeg"] = b["angleDeg"]
        if abs(b["abductionDeg"]) > 1e-9:
            entry["abductionDeg"] = b["abductionDeg"]
        entry["lengthM"] = b["lengthM"]
        if b["parent"] is None or b.get("fixed"):
            entry["joint"] = {"type": "fixed"}
        else:
            limit = max(LEVEL_LIMIT[min(b["level"], 3)], math.ceil(reach[b["id"]] * 1.2 * 2) / 2)
            entry["joint"] = {"type": "revolute", "minDeg": -limit, "maxDeg": limit}
        kg, c = weight[b["id"]]
        com = tree.local(b["id"], K.mul(c, 1 / kg)) if kg > 0 else (b["lengthM"] / 2, 0, 0)
        mass = dict(template_mass)
        mass.update(kg=round(kg, 1 if kg < 100 else 0), comX=round(com[0], 3), comY=round(com[1], 3), distribution="uniform")
        entry["mass"] = mass
        r = b["r"]
        section = dict(template_section)
        section.update(areaM2=float(f"{math.pi * r * r:.4g}"), secondMomentM4=float(f"{math.pi * r ** 4 / 4:.4g}"), extremeFibreM=round(r, 4), material=tree.wood, title={"en": f"{b['noun'][0]}, d = {2 * r:.2f} m", "de": f"{b['noun'][1]}, d = {de_num(2 * r)} m"})
        entry["section"] = section
        bones.append(entry)
    asset["rig"] = dict(old["rig"], bones=bones, root=rig[0]["id"])
    # dimensions
    spans = {}
    for scale in K.SCALES:
        w, l = built[scale]
        every = [p for s in w + l for p in s["mesh"].v]
        spans[scale] = (max(p[0] for p in every) - min(p[0] for p in every), max(p[1] for p in every), max(p[2] for p in every) - min(p[2] for p in every))
    mid = [(min(spans[s][k] for s in K.SCALES) + max(spans[s][k] for s in K.SCALES)) / 2 for k in range(3)]
    for scale in K.SCALES:
        off = [abs(spans[scale][k] / mid[k] - 1) for k in range(3)]
        print(f"  1:{scale} extent {[round(v, 2) for v in spans[scale]]} off the stated dimensions by {[round(v * 100, 1) for v in off]} %" + ("  <-- over 1.8 %" if max(off) > 0.018 else ""))
    asset["dimensions"] = dict(old["dimensions"], widthM=round(mid[0], 3), heightM=round(mid[1], 3), depthM=round(mid[2], 3), massKg=round(sum(w[0] for w in weight.values())))
    # anchors and emitters
    anchors = []
    root_r = tree.axes[tree.order[0]]["stations"][0][3]
    foot = round(cfg.get("footprint", 2.4 * root_r), 2)
    for a in old["anchors"]:
        if a["kind"] == "support":
            anchors.append(dict(a, bone=rig[0]["id"], x=0, y=0, footprintLengthM=foot, footprintWidthM=foot))
        elif a["kind"] != "emitter":
            raise SystemExit(f"anchor {a['id']} of kind {a['kind']} needs a hand decision")
    emit_points = {}
    for emitter in asset["thermal"]["emitters"]:
        spec = cfg["emitters"][emitter["id"]]
        chosen = [s for s in leaves50 if s["id"] in spec["on"]]
        if len(chosen) != len(spec["on"]):
            raise SystemExit(f"emitter {emitter['id']} names unknown masses")
        ids = []
        emit_points[emitter["id"]] = []
        for n, s in enumerate(chosen, 1):
            top = max(s["mesh"].v, key=lambda p: p[1])
            c = s["mesh"].centroid()
            point = (c[0] * 0.6 + top[0] * 0.4, top[1] - spec["diameter"] * 0.06, c[2] * 0.6 + top[2] * 0.4)
            at = tree.local(s["bone"], point)
            aid = f"{spec['prefix']}-{n}"
            anchors.append({"id": aid, "bone": s["bone"], "x": round(at[0], 4), "y": round(at[1], 4), "z": round(at[2], 4), "kind": "emitter", "title": {"en": f"{spec['title'][0]}, anchor {n}", "de": f"{spec['title'][1]}, Anker {n}"}})
            ids.append(aid)
            emit_points[emitter["id"]].append((s["bone"], point))
        emitter["anchor"], emitter["anchors"], emitter["diameterM"] = ids[0], ids, spec["diameter"]
    asset["anchors"] = anchors
    # volumes
    areas = {}
    volumes = {}
    plan = {"shapes": {}, "symbols": []}
    name_en, name_de = old["title"]["en"], old["title"]["de"]
    for scale in K.SCALES:
        wood, leaves = built[scale]
        digits = K.ROUND[scale]
        body, thermal, structure = [], [], []
        for s in wood:
            material = tree.bark[s["kind"]] if isinstance(tree.bark, dict) else tree.bark
            body.append(tree.solid(s["id"], s["bone"], s["mesh"], material, tree.tones.get(s["kind"], 0.3), extra={"slender": True} if s["slender"] else None, digits=4 if s["slender"] else digits))
            thermal.append({"id": f"th-{s['id']}", "of": s["id"], "region": tree.regions[s["kind"]]})
            if scale == 50:
                areas[tree.regions[s["kind"]]] = areas.get(tree.regions[s["kind"]], 0.0) + s["mesh"].area()
        for s in leaves:
            cls = s["spec"].get("cls") or ("shade" if s["shade"] else "sun")
            body.append(tree.solid(s["id"], s["bone"], s["mesh"], tree.leaf_materials[cls], tree.tones[cls], digits=digits))
            thermal.append({"id": f"th-{s['id']}", "of": s["id"], "region": tree.regions[cls]})
        for extra in cfg.get("extras", lambda scale, tree: [])(scale, tree):
            body.append(extra["solid"])
            thermal.append({"id": f"th-{extra['solid']['id']}", "of": extra["solid"]["id"], "region": extra["region"]})
        for emitter in asset["thermal"]["emitters"]:
            spec = cfg["emitters"][emitter["id"]]
            sides = {50: 14, 100: 12, 200: 10, 500: 8}[scale]
            for n, (bone, point) in enumerate(emit_points[emitter["id"]], 1):
                mesh = K.lens(point, spec["diameter"] / 2, max(spec["diameter"] * 0.12, 0.0002 * scale * 1.2), sides)
                thermal.append(tree.solid(f"th-{emitter['id']}-{n}", bone, mesh, tree.leaf_materials[spec["cls"]], 0.5, role="detail", extra={"region": emitter["region"], "tags": ["emitter", emitter["id"]]}, digits=digits))
        seen = set()
        for s in wood:
            first = s["bone"] not in seen and s["axis"] in [bone_by_id[s["bone"]]["axis"]]
            structure.append(tree.solid(f"member-{s['id']}", s["bone"], s["core"], tree.wood, 0.4, extra=dict({"member": s["bone"], "part": "member"}, **({"slender": True} if s["slender"] else {})), digits=4 if s["slender"] else digits))
            if first:
                seen.add(s["bone"])
                b = bone_by_id[s["bone"]]
                if b["parent"] is not None and b["level"] <= JOINT_LEVEL[scale]:
                    r = max(s["samples"][0][1][0] * 0.68, 0.0002 * scale * 0.6)
                    structure.append(tree.solid(f"joint-{b['id']}", b["id"], K.sphere(b["a"], r, 10 if scale == 50 else 8 if scale == 100 else 6), tree.wood, 0.3, role="detail", extra={"member": b["id"], "part": "joint"}, digits=digits))
        missing = [b["id"] for b in rig if b["id"] not in seen]
        if missing:
            raise SystemExit(f"1:{scale} bones without member: {missing}")
        structure.append(tree.solid("support-base", rig[0]["id"], K.disc((0.0, 0.0, 0.0), foot / 2, max(0.075, 0.0003 * scale), {50: 32, 100: 24, 200: 16, 500: 12}[scale]), tree.wood, 0.3, role="ground", extra={"member": rig[0]["id"], "part": "support", "anchor": anchors[0]["id"]}, digits=digits))
        heavy = sorted(rig, key=lambda b: -weight[b["id"]][0])[: {50: 3, 100: 3, 200: 2, 500: 1}[scale]]
        for b in heavy:
            mid = K.mul(K.add(b["a"], b["b"]), 0.5)
            host = min((s for s in wood if s["bone"] == b["id"]), key=lambda s: min(K.norm(K.sub(q[0], mid)) for q in s["samples"]))
            near = min(host["samples"], key=lambda q: K.norm(K.sub(q[0], mid)))
            structure.append(tree.solid(f"mass-{b['id']}", b["id"], K.sphere(near[0], max(near[1][0] * 0.5, 0.0002 * scale * 0.6), 10 if scale == 50 else 8 if scale < 500 else 6), tree.wood, 0.3, role="detail", extra={"member": b["id"], "part": "mass"}, digits=digits))
        n_wood, n_leaf = len(wood), len(leaves)
        habit = cfg["habit"][scale]
        mm = 0.15 * scale
        decisions = [
            {"en": f"Body at 1:{scale} ({len(body)} solids, smallest feature {mm:g} mm): {habit[0]} Every limb is one tapered loft through hand-stated stations that starts inside its parent, so sway never opens a joint; the wood is {n_wood} solids, none thinner than {K.MIN_DIA[scale] * 1000:g} mm, the foliage {n_leaf} masses that each follow the branch that carries them.",
             "de": f"Körper bei 1:{scale} ({len(body)} Körper, kleinstes Merkmal {de_num(mm, 1)} mm): {habit[1]} Jeder Ast ist ein verjüngter Loft durch von Hand gesetzte Stationen, der im Mutterast beginnt, sodass kein Gelenk beim Schwingen aufgeht; das Holz besteht aus {n_wood} Körpern, keiner dünner als {K.MIN_DIA[scale] * 1000:g} mm, das Laub aus {n_leaf} Massen, die je dem tragenden Ast folgen."},
            {"en": f"Thermal at 1:{scale} ({len(thermal)} solids): every bark and foliage surface is its region by reference to the body solid, so the thermal view is the same tree; no region has layers. The transpiration emitters are low outlet domes seated on the tops of the foliage masses that carry their anchors.",
             "de": f"Thermik bei 1:{scale} ({len(thermal)} Körper): Jede Rinden- und Laubfläche ist ihre Region als Verweis auf den Körper, die thermische Ansicht zeigt also denselben Baum; keine Region hat Schichten. Die Transpirationsemitter sind flache Austrittskuppen auf den Oberseiten der Laubmassen, die ihre Anker tragen."},
            {"en": f"Structure at 1:{scale} ({len(structure)} solids): the load path is the wood itself. Every stem, limb and branch of the body is a round member along its true axis at 0.55 of the bark radius, connected inside its parent down to the root collar; joint spheres sit at the rig joints, the fixed support is the root plate disc of {foot:g} m on the ground, and the heaviest segments carry a mass sphere on their axis.",
             "de": f"Tragwerk bei 1:{scale} ({len(structure)} Körper): Der Lastpfad ist das Holz selbst. Jeder Stamm, Ast und Zweig des Körpers ist ein Rundstab entlang seiner wahren Achse mit dem 0,55-fachen Rindenradius, im Mutterast angeschlossen bis hinab zum Wurzelhals; Gelenkkugeln sitzen an den Gelenken des Skeletts, das eingespannte Auflager ist die Wurzeltellerscheibe von {de_num(foot)} m auf dem Boden, und die schwersten Abschnitte tragen eine Massenkugel auf ihrer Achse."},
        ]
        volumes[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}
        # drawing plan
        rep = asset["representations"][str(scale)]
        hatch = next((h["id"] for h in rep.get("hatches", [])), None)
        shapes = []
        groups = {}
        for s in wood:
            if not s["slender"]:
                groups.setdefault((s["bone"], s["kind"]), []).append(s["id"])
        for (bone, kind), ids in groups.items():
            material = tree.bark[kind] if isinstance(tree.bark, dict) else tree.bark
            shapes.append({"id": f"{bone}-{kind}" if sum(1 for g in groups if g[0] == bone) > 1 else bone, "bone": bone, "role": "body", "filled": True, "tone": tree.tones.get(kind, 0.3), "region": tree.regions[kind], "material": material, "solids": ids, "cellF": 0.25, "tolF": 0.5, "layer": 0 if scale < 200 else -1000})
        leaf_groups = {}
        for s in leaves:
            cls = s["spec"].get("cls") or ("shade" if s["shade"] else "sun")
            group = s["spec"].get("group")
            key = (group or s["bone"], cls)
            leaf_groups.setdefault(key, {"bone": cfg.get("group_bone", {}).get(group, s["bone"]) if group else s["bone"], "cls": cls, "ids": []})["ids"].append(s["id"])
        for key, g in leaf_groups.items():
            shape = {"id": f"fol-{key[0]}-{g['cls']}".replace("fol-fol-", "fol-"), "bone": g["bone"], "role": "body", "filled": True, "tone": tree.tones[g["cls"]], "region": tree.regions[g["cls"]], "material": tree.leaf_materials[g["cls"]], "solids": g["ids"], "cellF": 0.5, "tolF": {50: 3.0, 100: 1.6, 200: 1.0, 500: 0.5}[scale], "minAreaF2": 16, "closeCells": 3}
            if g["cls"] == "shade" and hatch:
                shape["hatch"] = hatch
            shapes.append(shape)
        for extra in cfg.get("extra_shapes", lambda scale, tree: [])(scale, tree):
            shapes.append(extra)
        plan["shapes"][str(scale)] = {"elevation": shapes, "front": shapes, "plan": shapes}
        rep["geometry"] = {"elevation": [], "front": [], "plan": []}
        drawing = cfg["drawing"][scale]
        rep["decisions"] = dict(rep["decisions"])
        rep["decisions"]["silhouette"] = {"en": f"At 1:{scale} (1 m is {1000 // scale} mm) the {name_en} is drawn as the projection of its model of the same scale: {drawing[0]} Side elevation, front elevation and plan are three views of the same solids.", "de": f"Bei 1:{scale} (1 m sind {1000 // scale} mm) ist die Zeichnung ({name_de}) die Projektion des Modells desselben Maßstabs: {drawing[1]} Seitenansicht, Vorderansicht und Draufsicht sind drei Ansichten derselben Körper."}
        rep["decisions"]["retained"] = {"en": f"Retained: {len(groups)} wood outlines, one per rig bone and bark region, and {len(leaf_groups)} foliage outlines, painted in the depth order of the view.", "de": f"Beibehalten: {len(groups)} Holzumrisse, je Skelettknochen und Rindenregion einer, und {len(leaf_groups)} Laubumrisse, in der Tiefenfolge der Ansicht gemalt."}
        rep["decisions"]["merged"] = {"en": "The wood solids of one bone are merged into one outline; the foliage masses of one limb system are merged into one lobed outline with its sky openings, bound to the bone that carries the system.", "de": "Die Holzkörper eines Knochens sind zu einem Umriss verschmolzen; die Laubmassen eines Astsystems sind zu einem gelappten Umriss mit seinen Himmelslücken verschmolzen, gebunden an den Knochen, der das System trägt."}
        rep["decisions"]["omitted"] = {"en": f"Omitted: wood thinner than {K.MIN_DIA[scale] * 1000:g} mm, single leaves and bark texture strokes; the relief of bark and foliage is carried by the outlines themselves.", "de": f"Weggelassen: Holz dünner als {K.MIN_DIA[scale] * 1000:g} mm, einzelne Blätter und Texturstriche der Rinde; das Relief von Rinde und Laub tragen die Umrisse selbst."}
        rep["decisions"]["thinElements"] = {"en": f"Branches taper to {K.MIN_DIA[scale] * 1000:g} mm, the smallest width that still prints at the coarse end of this drawing's range, and are never drawn thinner.", "de": f"Äste verjüngen sich bis {K.MIN_DIA[scale] * 1000:g} mm, der kleinsten Breite, die am groben Ende des Bereichs dieser Zeichnung noch druckt, und werden nie dünner gezeichnet."}
        rep["decisions"]["hatching"] = {"en": "The shaded inner foliage carries the shade line hatch; no other hatching." if hatch and any(s.get("hatch") for s in shapes) else "No hatching.", "de": "Das beschattete innere Laub trägt die Schattenschraffur; sonst keine Schraffur." if hatch and any(s.get("hatch") for s in shapes) else "Keine Schraffur."}
        rep["decisions"]["overlays"] = {"en": "Every thermal region keeps its own shapes bound to the bones that carry it, so sway, loads and temperatures act on the drawn parts.", "de": "Jede thermische Region behält ihre eigenen, an die tragenden Knochen gebundenen Formen, sodass Schwingung, Lasten und Temperaturen an den gezeichneten Teilen wirken."}
    asset["volumes"] = volumes
    # thermal areas of the bark regions from the 1:50 model
    trim = cfg.get("area_trim", {})
    for region in asset["thermal"]["regions"]:
        if region.get("areaBasis") == "exchange":
            continue
        if region["id"] in cfg.get("areas", {}):
            region["areaM2"] = cfg["areas"][region["id"]]
        elif region["id"] in areas:
            region["areaM2"] = round(areas[region["id"]] * trim.get(region["id"], 0.95), 2 if areas[region["id"]] < 10 else 1)
    # symbols
    wood100, leaves100 = built[100]
    for symbol in asset["symbols"]:
        if symbol["semantics"] != "true-extent":
            continue
        entry = {"id": symbol["id"], "scale": 100, "cellM": 0.03, "tolM": 0.05, "geometry": {}}
        stems = cfg.get("stems") or [[s["id"] for s in wood100 if s["kind"] == cfg.get("stem_kind", "trunk")]]
        for view in ("elevation", "front", "plan"):
            out = []
            n_scaffold = 0
            for shape in symbol["geometry"][view]:
                meta = {k: v for k, v in shape.items() if k not in ("points", "closed", "kind", "d")}
                if shape["id"] == "crown":
                    out.append(dict(meta, solids=[s["id"] for s in leaves100]))
                elif shape["id"].startswith("stem-"):
                    k = int(shape["id"].split("-")[1]) - 1
                    if k >= len(stems):
                        continue
                    if view == "plan":
                        host = next(s for s in wood100 if s["id"] == stems[k][0])
                        c, v, _ = min(host["samples"], key=lambda q: abs(q[0][1] - 1.3))
                        out.append(dict(meta, kind="poly", closed=True, points=[[round(c[0] + v[0] * math.cos(math.radians(a)), 4), round(c[2] + v[0] * math.sin(math.radians(a)), 4)] for a in range(0, 360, 15)]))
                    else:
                        out.append(dict(meta, solids=stems[k]))
                elif shape["id"].startswith("scaffold-"):
                    axes = cfg.get("scaffolds", [])
                    if n_scaffold < len(axes):
                        p = tree.axes[axes[n_scaffold]]["path"]
                        out.append(dict(meta, kind="poly", closed=False, points=[[round(p.at(f)[0][0], 4), round(p.at(f)[0][2], 4)] for f in (0, 0.25, 0.5, 0.75, 1)]))
                        n_scaffold += 1
                else:
                    made = cfg.get("symbol_extra", lambda view, shape, tree: None)(view, shape, tree)
                    if made is not None:
                        out.append(made)
            entry["geometry"][view] = out
        plan["symbols"].append(entry)
    doc["asset"] = asset
    out = os.path.join(K.OUT, slug)
    K.write(os.path.join(out, "pre.json"), doc)
    K.write(os.path.join(out, "plan.json"), plan)
    sizes = {s: len(json.dumps(volumes[str(s)], separators=(",", ":"))) for s in K.SCALES}
    print(f"{slug}: bones {len(rig)}, dims {asset['dimensions']}, areas { {k: round(v, 1) for k, v in areas.items()} }, volume json bytes {sizes}")
    for s in K.SCALES:
        w, l = built[s]
        print(f"  1:{s}: wood {len(w)}, leaves {len(l)}, vertices {sum(len(x['mesh'].v) for x in w + l)}")
    for b in rig:
        f = tree.frames[b["id"]]
        if abs(f["w"][2]) < 0.3:
            print(f"  [DEBUG] bone {b['id']} is nearly edge-on in the elevation (w.z {f['w'][2]:.2f})")
    return doc
