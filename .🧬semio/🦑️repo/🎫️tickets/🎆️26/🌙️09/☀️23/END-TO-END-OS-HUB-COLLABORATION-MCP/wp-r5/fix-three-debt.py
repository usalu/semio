import json, subprocess
ROOT = "/Users/ueli/Documents/semio"
paths = [l.strip() for l in open(f"{ROOT}/.tmp-ticket/wp-r5/generated/three-production-paths.txt", encoding="utf-8") if l.strip()]
files = subprocess.check_output(["git", "grep", "-l", '"reason": "three is already', "--", "*🔣️.json"], cwd=ROOT, text=True).split("\n")
for rel in [f for f in files if f]:
    p = f"{ROOT}/{rel}"; t = open(p, encoding="utf-8").read(); d = json.loads(t)
    for o in d.get("oracles", []):
        debt = o.get("productionDebt")
        if not debt or "reachableFrom" in debt: continue
        plan = " ".join(x for x in (debt.get("reason"), debt.get("consequence")) if x)
        o["productionDebt"] = {"reachableFrom": paths, "owner": debt.get("owner", "✏️s/🔌️plugins/📐️cad"), "plan": plan}
        print("debt", o["id"], rel.split("/🔮️oracles")[0][-60:])
    n = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
    if n != t: open(p, "w", encoding="utf-8").write(n)
