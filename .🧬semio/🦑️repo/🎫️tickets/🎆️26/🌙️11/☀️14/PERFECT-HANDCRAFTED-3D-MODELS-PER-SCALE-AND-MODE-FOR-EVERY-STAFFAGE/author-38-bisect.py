import json, subprocess, os, shutil, re, sys
T = os.path.dirname(os.path.abspath(__file__))
BUN = subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1", shell=True, text=True).strip()
p = T + "/volumes/objects/playground-swing.json"
bk = T + "/🗑️generated/author-38/swing-backup.json"
a0 = json.load(open(bk))
def run(label, edit):
    a = json.loads(json.dumps(a0)); edit(a)
    json.dump(a, open(p, "w"), ensure_ascii=False, indent=1)
    out = subprocess.run([BUN, "volumes-check.ts", "objects/playground-swing", "--product", "--rules", "volumes-cross-scale"], capture_output=True, text=True, cwd=T).stdout
    for l in out.splitlines():
        if "cross-scale" in l and "iou:" in l:
            m = re.search(r"(1:\d+).*?p95M:front=([0-9.]+)", l)
            print(label, m.group(1), "front p95", m.group(2), "ok" if l.startswith("ok") else "FAIL")
def nochain(a):
    for den in ("50", "100", "200"):
        for b in a["volumes"][den]["body"]:
            if b["id"].endswith("-rod"): b.pop("slender", None)
def drop(prefixes):
    def f(a):
        nochain(a)
        a["volumes"]["50"]["body"] = [b for b in a["volumes"]["50"]["body"] if not any(b["id"].startswith(x) for x in prefixes)]
    return f
run("base-nonslender", nochain)
run("-hanger", drop(["hanger"]))
run("-clamp", drop(["leg-left-clamp", "leg-right-clamp", "brace-clamp"]))
run("-shackle", drop(["shackle"]))
run("-weld", drop(["leg-left-weld", "leg-right-weld"]))
shutil.copy(bk, p)
