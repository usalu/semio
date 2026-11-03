import json, sys, re

CHASSIS = {
    "frame": "body-panels", "bumper-front": "body-panels", "bumper-rear": "body-panels",
    "plan-bumper-front": "body-panels", "plan-bumper-rear": "body-panels", "tail-lamp": "body-panels",
    "spare-cover": "body-panels", "spare-boss": "body-panels", "bracket-front": "body-panels",
    "bracket-rear": "body-panels", "well-front": "body-panels", "well-rear": "body-panels",
    "mirror": "body-panels", "mirror-l-plan": "body-panels", "mirror-r-plan": "body-panels",
    "radiator-neck": "radiator", "radiator-cap": "radiator", "plan-radiator-cap": "radiator",
    "headlamp-bowl": "fenders", "plan-lamp-l": "fenders", "plan-lamp-r": "fenders",
    "plan-lamp-bar-l": "fenders", "plan-lamp-bar-r": "fenders", "lamp-stalk": "fenders",
    "headlamp-lens": "glazing", "headlamp": "glazing", "headlamp-l-plan": "glazing", "headlamp-r-plan": "glazing",
    "tail-lamp-l-plan": "tailgate", "tail-lamp-r-plan": "tailgate",
}
WHEEL = {"wheel-f": "tyres", "wheel-r": "tyres"}

path, mode = sys.argv[1], sys.argv[2]
text = open(path, encoding="utf-8").read()
doc = json.loads(text)
reps = doc["asset"]["representations"]
todo, unmapped = {}, []
for key, rep in reps.items():
    if rep.get("status") != "authored":
        continue
    for lst in ("elevation", "plan"):
        for sh in rep["geometry"].get(lst) or []:
            if not sh.get("filled") or sh.get("region") or sh.get("regions"):
                continue
            sid, bone = sh["id"], sh.get("bone")
            reg = WHEEL.get(bone) or CHASSIS.get(sid)
            if reg is None:
                unmapped.append((key, lst, sid, bone))
            else:
                todo.setdefault(sid, set()).add(reg)
print("unmapped", unmapped)
print("mapping", {k: sorted(v) for k, v in todo.items()})
withreg = set()
for rep in reps.values():
    for lst in ("elevation", "plan"):
        for sh in (rep.get("geometry", {}).get(lst) or []):
            if sh.get("region") or sh.get("regions"):
                withreg.add(sh["id"])
clash = sorted(set(todo) & withreg)
print("clash", clash)
if mode == "apply":
    if any(len(v) != 1 for v in todo.values()):
        sys.exit("refusing: clash or ambiguous")
    todo = {k: v for k, v in todo.items()}
    start = text.index('"representations"')
    end = text.index('"poses"', start)
    seg = text[start:end].split("\n")
    out, n = [], 0
    for i, line in enumerate(seg):
        m = re.match(r'^(\s*)"id": "([^"]+)",\s*$', line)
        if m and m.group(2) in todo:
            ind = len(m.group(1))
            s = i
            while not (seg[s].strip() == "{" and len(seg[s]) - len(seg[s].lstrip()) < ind):
                s -= 1
            e = i
            while not (seg[e].strip().startswith("}") and len(seg[e]) - len(seg[e].lstrip()) < ind):
                e += 1
            has = any(re.match(r'^\s*"regions?":', seg[k]) and len(seg[k]) - len(seg[k].lstrip()) == ind for k in range(s, e))
            if not has:
                reg = next(iter(todo[m.group(2)]))
                out.append(f'{m.group(1)}"region": "{reg}",')
                n += 1
        out.append(line)
    new = text[:start] + "\n".join(out) + text[end:]
    json.loads(new)
    open(path, "w", encoding="utf-8", newline="").write(new)
    print("inserted", n)
