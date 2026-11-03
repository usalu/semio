import json, copy, os, sys
here = os.path.dirname(os.path.abspath(__file__))
gen = os.path.join(os.path.dirname(here), "🗑️generated", "C2", "var")
os.makedirs(gen, exist_ok=True)
src = json.load(open("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/stair-flight/🔣️.json", encoding="utf8"))
def make(name, fn):
    d = copy.deepcopy(src)
    fn(d["asset"]["representations"]["100"]["geometry"]["plan"])
    p = os.path.join(gen, name + ".json")
    json.dump(d, open(p, "w", encoding="utf8"), ensure_ascii=False)
    return p
def v2(pl):
    for s in pl:
        if s["id"].startswith("walking"): s["region"] = "stringer-steel"
def v3(pl):
    for s in pl:
        if s["id"].startswith("tread-"): s["filled"] = False
def v4(pl):
    for s in pl:
        if s["id"].startswith("walking"): s["z"] = 50
def vA(pl):
    for s in pl:
        if s["id"].startswith("walking"):
            s.pop("region", None)
def vB(pl):
    for s in pl:
        if s["id"].startswith("walking"): s["bone"] = "walker-carrier"
def vC(pl):
    for s in pl:
        if s["id"].startswith("walking"): s["role"] = "body"
def vD(pl):
    for s in pl:
        if s["id"].startswith("walking"): s["penMm"] = 0.3
def vE(pl):
    for s in pl:
        if s["id"].startswith("tread-"): s["tone"] = 0.9; s.pop("material", None)
jobs = []
for n, f in (("vA", vA), ("vB", vB), ("vC", vC), ("vD", vD), ("vE", vE)):
    p = make(n, f).replace("\\", "/")
    jobs.append(["--file", p, "--scale", "100", "--viewpoint", "plan", "--out", gen.replace("\\", "/") + "/" + n])
json.dump(jobs, open(os.path.join(here, "jobs-var.json"), "w", encoding="utf8"), ensure_ascii=False)
print(len(jobs))
