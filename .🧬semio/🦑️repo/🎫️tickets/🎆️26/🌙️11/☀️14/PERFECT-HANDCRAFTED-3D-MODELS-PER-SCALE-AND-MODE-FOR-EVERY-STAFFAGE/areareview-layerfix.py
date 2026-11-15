"""Reorders thermal.regions[].layers of named regions to 'innermost first' by text surgery (JSON-compared): python3 areareview-layerfix.py"""
import glob, importlib.util, json, os, tempfile, time
T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("u", os.path.join(T, "areareview-unsection.py")); u = importlib.util.module_from_spec(spec); spec.loader.exec_module(u)
FIX = {  # slug -> region -> new order of layer ids
    "city-bus": {r: ["insulation", "skin"] for r in ("body-panels", "roof", "doors")},
    "tram": {r: ["insulation", "skin"] for r in ("body-panels", "roof", "doors")},
}
def fix(path, regions, orders=None):
    for _ in range(8):
        text = open(path, encoding="utf8").read()
        _, root = u.parse(text, 0)
        regs = root["asset"][3]["thermal"][3]["regions"][3]
        edits = []; exp = json.loads(text)
        for reg in regs:
            rid = json.loads(text[reg[3]["id"][1]:reg[3]["id"][2]])
            if rid not in regions: continue
            lay = reg[3]["layers"][3]
            items = [(l[1], l[2], json.loads(text[l[1]:l[2]])) for l in lay]
            want = regions[rid]
            cur = [i[2]["id"] for i in items]
            if cur == want: continue
            assert sorted(cur) == sorted(want), (rid, cur, want)
            byid = {i[2]["id"]: text[i[0]:i[1]] for i in items}
            for slot, wid in zip(items, want): edits.append((slot[0], slot[1], byid[wid]))
            for r in exp["asset"]["thermal"]["regions"]:
                if r["id"] == rid: r["layers"].sort(key=lambda l: want.index(l["id"]))
        if not edits: return 0
        new = text
        for a, b, t in sorted(edits, reverse=True): new = new[:a] + t + new[b:]
        assert json.loads(new) == exp, path
        if open(path, encoding="utf8").read() != text: time.sleep(0.2); continue
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
        with os.fdopen(fd, "w", encoding="utf8") as f: f.write(new)
        os.replace(tmp, path)
        return len(edits)
if __name__ == "__main__":
    FIX["tower-crane"] = {"cab": None}
    FIX["seated-office-worker"] = {"chair-fabric": None}
    for slug, regs in FIX.items():
        p = glob.glob(f"{u.CAT}/*/{slug}/🔣️.json")[0]
        a = json.load(open(p))["asset"]
        for r in a["thermal"]["regions"]:
            if r["id"] in regs and regs[r["id"]] is None:
                regs[r["id"]] = [l["id"] for l in reversed(r["layers"])]
        print(slug, fix(p, regs))
