"""🧵 Restores true size of slender parts (author 65): usage `python3 author-74-fix.py <category/slug> [--dry] [--skip id,id] [--no-thick] [--no-flag] [--only-scales 200,500]`.
Thick rows (same-axis 1:50 slender rod found, coarse cross extent > 1.25x) take the 1:50 solid's geometry at the coarse scale; qualifying unflagged solids get `slender: true`. Writes a change log JSON to the generated folder."""
import copy, json, os, subprocess, sys, tempfile, time

T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
META = ("id", "material", "tone", "role", "region", "regions", "layer", "tags", "member", "part", "anchor", "maxDenominator", "texture", "envelope", "sectionFaces")


def scan(spec):
    out = subprocess.run([BUN, "author-65-scan.ts", spec], cwd=T, capture_output=True, text=True).stdout
    return json.loads(out).get(spec, [])


def main():
    a = sys.argv[1]
    dry = "--dry" in sys.argv
    skip = set((sys.argv[sys.argv.index("--skip") + 1] if "--skip" in sys.argv else "").split(","))
    only = set((sys.argv[sys.argv.index("--only-scales") + 1] if "--only-scales" in sys.argv else "").split(",")) - {""}
    path = os.path.join(T, "volumes", a + ".json")
    plan = dict(x.split(":") for x in sys.argv[sys.argv.index("--plan") + 1].split(",")) if "--plan" in sys.argv else None
    outp = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else path
    rows = scan(a)
    doc = json.load(open(path))
    vols = doc["volumes"]
    log = []
    used = {}
    for r in rows:
        if "error" in r:
            print("scan error", r); return
    for r in rows:
        sc = str(r["scale"]); lst = r["list"]
        if only and sc not in only: continue
        mode = "all" if plan is None else plan.get(sc)
        if mode is None: continue
        keep = mode.endswith("+k")
        mode = mode[:-2] if keep else mode
        if r["id"] in skip: continue
        lst_n = vols[sc][lst]
        n = next((s for s in lst_n if s["id"] == r["id"]), None)
        if n is None or "of" in n: continue
        if r["thick"] and "--no-thick" not in sys.argv and mode in ("all", "thick") and r.get("match"):
            m = next((s for s in vols["50"][lst] if s["id"] == r["match"]["id"]), None)
            key = (sc, lst, r["match"]["id"])
            if m is None or "of" in m or m.get("bone") != n.get("bone") or (m.get("repeat") or None) != (n.get("repeat") or None) or key in used:
                log.append({"scale": sc, "list": lst, "id": r["id"], "action": "MANUAL", "why": "bone/repeat/ref/shared match", "match": r["match"]["id"]}); continue
            used[key] = r["id"]
            new = copy.deepcopy(m)
            if keep and n["kind"] == "loft" and m["kind"] in ("capsule", "loft"):
                secs = copy.deepcopy(m["sections"]) if m["kind"] == "loft" else [{"at": m["p0"], "rx": m["r0"], "ry": m["r0"]}, {"at": m["p1"], "rx": m["r1"], "ry": m["r1"]}]
                new = copy.deepcopy(n)
                new["sections"] = secs
                for k in ("capStart", "capEnd"):
                    if k in m: new[k] = m[k]
            for k in META:
                new.pop(k, None)
                if k in n: new[k] = n[k]
            new["id"] = n["id"]
            new["slender"] = True
            ids = list(n.keys())
            new = {**{k: new[k] for k in ids if k in new}, **{k: v for k, v in new.items() if k not in ids}}
            idx = lst_n.index(n)
            log.append({"scale": sc, "list": lst, "id": r["id"], "action": "restore", "from": r["crossMm"], "match": r["match"]["id"], "matchCrossMm": r["match"]["crossMm"], "kind": [n["kind"], m["kind"]]})
            if not dry: lst_n[idx] = new
        elif r["unflagged"] and "--no-flag" not in sys.argv and mode in ("all", "flag") and not n.get("slender"):
            if n.get("texture") is True: continue
            log.append({"scale": sc, "list": lst, "id": r["id"], "action": "flag", "crossMm": r["crossMm"], "ratio": r["ratio"]})
            if not dry: n["slender"] = True
        elif r.get("falseFlag"):
            log.append({"scale": sc, "list": lst, "id": r["id"], "action": "FALSEFLAG", "ratio": r["ratio"]})
    if not dry:
        fresh = json.load(open(path))
        for sc in vols:
            for lst in ("body", "thermal", "structure"):
                if vols[sc].get(lst) is not None and fresh["volumes"][sc].get(lst) != vols[sc][lst]: fresh["volumes"][sc][lst] = vols[sc][lst]
        tmp = outp + ".65.tmp"
        open(tmp, "w").write(json.dumps(fresh, indent=1) + "\n")
        os.replace(tmp, outp)
    os.makedirs(os.path.join(T, "🗑️generated/author-74"), exist_ok=True)
    json.dump(log, open(os.path.join(T, "🗑️generated/author-74", "changes-" + a.replace("/", "_") + ".json"), "w"), indent=1)
    for l in log: print(json.dumps(l, ensure_ascii=False))
    print(len(log), "entries", "(dry)" if dry else "")


main()
