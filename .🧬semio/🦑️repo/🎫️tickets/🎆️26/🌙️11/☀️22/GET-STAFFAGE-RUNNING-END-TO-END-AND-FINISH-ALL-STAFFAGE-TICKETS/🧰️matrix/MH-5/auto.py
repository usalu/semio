#!/usr/bin/env python3
"""🤖️ The search for the repair of one person of batch MH-5: `python3 auto.py <id> [--scales 50,100] [--margin 0.93] [--fresh]`.

It works on a copy (`$GEN/auto/<id>.json`, the catalog file with the ops of `ops/<id>.json` applied; the catalog is never written here). Round by round the model gate (`mgate.ts`) names the shapes whose three views disagree in the hard tier; for each of them a fixed list of candidate repairs is applied to copies in one `fit.ts --try` process, from the least to the most invasive: fit what the views do not show (`sync`), cut hidden length (`clip`), turn flat shapes into prisms (`convert`, the ops of `../MR/fit.ts`), make two or three views the same body (`copy` from a view that is a real solid, never from a plate thinner than 30 mm; a body of revolution also 6 to 20 % slimmer), then the same with a few millimetres of visible slack, for all views or for one view alone (`lock`); when none passes, the geometry the shape has in a neighbouring band (`copy-scale`). A candidate passes when the largest disagreement of the shape is below `margin` × tolerance in the model AND the silhouette guard of that scale (`sil.ts`, all three views) is still `OK`; of the passing candidates the one with the smallest disagreement (in steps of 10 mm, the earlier in the list among equals) is kept (the guard measures them four at a time in that order and stops at the first group with a passing one) and appended to `ops/<id>.json` (as `set` ops with the fitted geometry when applying the op twice would move it again). A shape without a passing candidate is reported as open and left as it was. Heavy processes run under the lock of `q.sh`. The log is `$GEN/auto/<id>.log`.
"""
import json, os, re, shutil, subprocess, sys, glob

HOME = os.path.expanduser("~")
B = sorted(glob.glob(HOME + "/Library/Caches/semio/runtimes/bun/*/bun"))[0]
R = "/Users/ueli/Documents/semio"
S = R + "/🎓️teaching/🏛️architecture/🧍️staffage"
T = S + "/🧪️tests/🔭️multiview-coverage"
U = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
W = U + "/🧰️matrix/MH-5"
GEN = U + "/🗑️generated/matrix/MH-5"
GEOMETRY_KEYS = ["kind", "points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"]
VIEWS = ["elevation", "front", "plan"]

args = sys.argv[1:]
asset_id = args[0]
flag = lambda name, default=None: args[args.index("--" + name) + 1] if "--" + name in args else default
only_scales = flag("scales")
margin = float(flag("margin", "0.93"))
auto = GEN + "/auto"
os.makedirs(auto, exist_ok=True)
work = f"{auto}/{asset_id}.json"
ops_file = f"{W}/ops/{asset_id}.json"
log_file = open(f"{auto}/{asset_id}.log", "a")
catalog = glob.glob(f"{S}/🗂️catalog/*/{asset_id}/🔣️.json")[0]


def say(text):
    print(text, flush=True)
    log_file.write(text + "\n")
    log_file.flush()


def run(cmd, cwd=T):
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True).stdout


def fit_try(source, candidates, prefix):
    path = prefix + ".candidates.json"
    json.dump(candidates, open(path, "w"))
    out = run([B, W + "/fit.ts", source, "--try", path, "--prefix", prefix])
    result = {}
    for line in out.splitlines():
        parts = line.split("\t")
        if len(parts) == 3:
            result.setdefault(parts[0], []).append((parts[1], parts[2]))
    return result


accepted = [] if "--fresh" in args or not os.path.exists(ops_file) else json.load(open(ops_file))
if accepted:
    run([B, W + "/fit.ts", asset_id, "--ops", ops_file, "--out", work])
else:
    shutil.copy(catalog, work)


def shapes_of(scale, shape_id, bone):
    doc = json.load(open(work))["asset"]
    g = doc["representations"][scale]["geometry"]
    return {view: next((s for s in g[view] if s["id"] == shape_id and s["bone"] == bone), None) for view in VIEWS}


def candidates_for(scale, shape_id, bone, phase=1):
    shapes = shapes_of(scale, shape_id, bone)
    base = {"scale": scale, "id": shape_id, "bone": bone}
    if phase == 2:
        doc = json.load(open(work))["asset"]
        others = [key for key, rep in doc["representations"].items() if key != scale and rep.get("status") == "authored"]
        others.sort(key=lambda key: abs(int(key) - int(scale)))
        return [(f"scale{key}", [{"op": "copy-scale", **base, "from": key}, {"op": "measure", **base}]) for key in others]
    sync = lambda **more: {"op": "sync", **base, **more}
    measure = {"op": "measure", **base}
    out = []
    add = lambda name, *ops: out.append((name, list(ops) + [measure]))
    kind = {view: (None if shapes[view] is None else "flat" if "solid" not in shapes[view] else shapes[view]["solid"]["kind"] + ("-" + shapes[view]["solid"].get("sweep", "") if shapes[view]["solid"]["kind"] == "extruded" else "")) for view in VIEWS}
    flats = [view for view in VIEWS if kind[view] == "flat"]
    side = [view for view in ("front", "plan") if kind[view] in ("extruded-v", "extruded-w")]
    clip = lambda view: {"op": "clip", "scale": scale, "view": view, "id": shape_id, "bone": bone}
    copy = lambda source, to: {"op": "copy", **base, "from": source, "to": to}
    variants = []
    variants.append(("sync", []))
    for view in side:
        variants.append(("clip-" + view, [clip(view)]))
    if len(side) == 2:
        variants.append(("clip-both", [clip("front"), clip("plan")]))
    for slack in (None, 0.002, 0.004):
        tag = "" if slack is None else f"~{int(slack * 1000)}"
        more = {} if slack is None else {"slack": slack}
        for name, pre in variants:
            add(name + tag, *pre, sync(**more))
        if flats:
            side_flats = [view for view in flats if view != "elevation"]
            if side_flats:
                add("convert-sides" + tag, sync(convert=side_flats, **more))
                for view in side_flats:
                    if len(side_flats) > 1:
                        add("convert-" + view + tag, sync(convert=[view], **more))
            if "elevation" in flats:
                add("convert-all" + tag, sync(convert=flats, **more))
                add("convert-elevation" + tag, sync(convert=["elevation"], **more))
        if slack is None and kind["elevation"] not in (None, "revolved"):
            mr = {"front": {"op": "front-sweep", **base}, "plan": {"op": "plan-revolve", **base}}
            ex = {view: {"op": "extrude", **base, "view": view} for view in ("front", "plan")}
            present = [view for view in ("front", "plan") if kind[view] is not None]
            for view in present:
                add("mr-" + view, mr[view], sync())
                add("extrude-" + view, ex[view], sync())
            if len(present) == 2:
                add("mr-both", mr["front"], mr["plan"], sync())
                add("extrude-both", ex["front"], ex["plan"], sync())
                add("mr-front-extrude-plan", mr["front"], ex["plan"], sync())
        if slack is not None:
            solid_views = [view for view in VIEWS if kind[view] is not None and kind[view].startswith("extruded")]
            wide = slack * 1.5
            for view in solid_views:
                add(f"solo-{view}~{int(wide * 1000)}", sync(lock=[other for other in VIEWS if other != view], slack=wide))
            if len(solid_views) > 1:
                add(f"solo-chain~{int(wide * 1000)}", *[sync(lock=[other for other in VIEWS if other != view], slack=wide) for view in ("front", "plan", "elevation") if view in solid_views])
        if slack is None:
            for source in VIEWS:
                if kind[source] in (None, "flat"):
                    continue
                solid = shapes[source]["solid"]
                if solid["kind"] == "extruded" and solid["z1"] - solid["z0"] < 0.03:
                    continue
                others = [view for view in VIEWS if view != source and kind[view] is not None]
                if len(others) == 2:
                    add(f"{source}>both", copy(source, others))
                for view in others:
                    add(f"{source}>{view}", copy(source, [view]), sync())
                if solid["kind"] == "revolved":
                    shrink = lambda view, factor: {"op": "scale-profile", "scale": scale, "view": view, "id": shape_id, "bone": bone, "coordinate": 1, "factor": factor}
                    for factor in (0.94, 0.88, 0.8):
                        for view in others:
                            add(f"{source}>{view}*{factor}", copy(source, [view]), shrink(view, factor), sync())
                        if len(others) == 2:
                            add(f"{source}>both*{factor}", copy(source, others), shrink(others[0], factor), shrink(others[1], factor))
    return out


def model_hard():
    out = run([B, W + "/mgate.ts", work] + (["--scales", only_scales] if only_scales else []))
    found = []
    for line in out.splitlines():
        m = re.match(r"\S+ 1:(\d+) HARD (\w+) ([^/\s]+)/(\S+) (\d+) mm", line)
        if m and (m.group(1), m.group(4), m.group(3)) not in found:
            found.append((m.group(1), m.group(4), m.group(3)))
    return found, out.splitlines()[-1] if out else "no output"


def guard(scale, files):
    out = run([W + "/q.sh", "timeout", "1500", B, W + "/sil.ts", "--scales", scale] + files, cwd=S + "/📦️packages/🟦️typescript")
    result = {}
    for line in out.splitlines():
        m = re.match(r"1:\d+ (.*) (OK|FAIL)  (\S+)$", line)
        if m:
            result[m.group(3)] = (m.group(2), m.group(1))
    return result


opened = []
while True:
    hard, summary = model_hard()
    todo = [triple for triple in hard if triple not in opened]
    say(f"## {asset_id}: {summary}; {len(todo)} to do, {len(opened)} open")
    if not todo:
        break
    scale, shape_id, bone = todo[0]
    kept = None
    notes = []
    for phase in (1, 2):
        if kept is not None:
            break
        candidates = candidates_for(scale, shape_id, bone, phase)
        prefix = f"{auto}/{asset_id}.c"
        for old in glob.glob(prefix + ".*.json"):
            os.remove(old)
        tried = fit_try(work, {name: ops for name, ops in candidates}, prefix)
        passing = []
        for name, ops in candidates:
            lines = tried.get(name, [])
            if not lines or lines[-1][1] != "written":
                continue
            m = next((re.search(r"MEASURE (\[.*?\]) tolerance ([\d.]+)", text) for op, text in lines if op == "measure"), None)
            if m is None:
                continue
            values, tolerance = json.loads(m.group(1)), float(m.group(2))
            notes.append(f"{name} {values}")
            if min(values) >= 0 and max(values) <= margin * tolerance:
                passing.append((name, ops, values))
        kept = None
        passing.sort(key=lambda item: int(max(item[2]) // 10))
        for start in range(0, len(passing), 4):
            chunk = passing[start:start + 4]
            result = guard(scale, [f"{prefix}.{name}.json" for name, _, _ in chunk])
            good = []
            for name, ops, values in chunk:
                got = result.get(f"{asset_id}.c.{name}.json")
                say(f"   1:{scale} {bone}/{shape_id} candidate {name} {values}: {got}")
                if got is not None and got[0] == "OK":
                    good.append((name, ops, values))
            if good:
                kept = good[0]
                break
    if kept is None:
        opened.append((scale, shape_id, bone))
        say(f"OPEN 1:{scale} {bone}/{shape_id}: no candidate passes model and guard; model results: {'; '.join(notes)}")
        continue
    name, ops, values = kept
    ops = ops[:-1]
    result_file = f"{prefix}.{name}.json"
    again = fit_try(result_file, {"again": ops}, f"{auto}/{asset_id}.again")
    stable = all(text.startswith("unchanged") or re.fullmatch(r"(\w+ unchanged(, )?)+", text) for op, text in again.get("again", []) if op != "done")
    if not stable:
        before, after = json.load(open(work))["asset"], json.load(open(result_file))["asset"]
        ops = []
        for view in VIEWS:
            old = next((s for s in before["representations"][scale]["geometry"][view] if s["id"] == shape_id and s["bone"] == bone), None)
            new = next((s for s in after["representations"][scale]["geometry"][view] if s["id"] == shape_id and s["bone"] == bone), None)
            if new is not None and old != new:
                ops.append({"op": "set", "scale": scale, "view": view, "id": shape_id, "bone": bone, "why": f"result of the candidate {name} of auto.py", "set": {key: new.get(key) for key in GEOMETRY_KEYS}})
    accepted.extend(ops)
    shutil.copy(result_file, work)
    os.makedirs(os.path.dirname(ops_file), exist_ok=True)
    json.dump(accepted, open(ops_file, "w"), indent=1)
    say(f"KEPT 1:{scale} {bone}/{shape_id}: {name} {values} ({'ops' if stable else 'as set ops'})")
say(f"## {asset_id} done: {len(accepted)} ops in {ops_file}; open: {opened}")
