import json, subprocess, sys, itertools, shutil, os
T = os.path.dirname(os.path.abspath(__file__))
BUN = subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1", shell=True, text=True).strip()
p = T + "/volumes/objects/playground-swing.json"
bk = T + "/🗑️generated/author-38/swing-backup.json"
den = sys.argv[1]
shutil.copy(p, bk)
a0 = json.load(open(bk))
legs = [float(x) for x in sys.argv[2].split(",")]
brs = [float(x) for x in sys.argv[3].split(",")]
chs = [float(x) for x in sys.argv[4].split(",")]
rcw = float(sys.argv[5])
for rl, rb, rc in itertools.product(legs, brs, chs):
    a = json.loads(json.dumps(a0))
    for b in a["volumes"][den]["body"]:
        def setr(r):
            if b["kind"] == "revolve": b["profile"] = [[x, r if y > 0 else 0] for x, y in b["profile"]]
            elif b["kind"] == "capsule": b["r0"] = b["r1"] = r
            elif b["kind"] == "loft":
                for s in b["sections"]:
                    if b["id"].endswith("-rod"): continue
                    if s["rx"] == 0.03: s["rx"] = r
                    if abs(s["ry"] - 0.03) < 1e-9: s["ry"] = r
                    elif abs(s["ry"] - 0.03 / 0.8706) < 1e-3: s["ry"] = r / 0.8706
        if b["id"].startswith("brace-"): setr(rb)
        if b["id"].endswith("-rod") and b["kind"] == "loft":
            for s_ in b["sections"]: s_["rx"], s_["ry"] = rcw, rc
        elif b["id"].endswith("-rod"):
            z = b["p0"][2] if b["kind"] == "capsule" else b["origin"][2]
            u0 = b["p0"][0] if b["kind"] == "capsule" else b["origin"][0]
            u1 = b["p1"][0] if b["kind"] == "capsule" else b["origin"][0] + b["profile"][2][0]
            for k in ("p0", "p1", "r0", "r1", "profile", "origin", "axis"): b.pop(k, None)
            b["kind"] = "loft"; b["capStart"] = True; b["capEnd"] = True
            b["sections"] = [{"at": [u0, 0, z], "rx": rcw, "ry": rc}, {"at": [u1, 0, z], "rx": rcw, "ry": rc}]
        if b["id"].startswith("leg-"): setr(rl)
    json.dump(a, open(p, "w"), ensure_ascii=False, indent=1)
    out = subprocess.run([BUN, "volumes-check.ts", "objects/playground-swing", "--scale", den, "--product", "--rules", "volumes-silhouette"], capture_output=True, text=True, cwd=T).stdout
    line = [l for l in out.splitlines() if "silhouette" in l and "iou:" in l][0]
    import re
    print(rl, rb, rc, re.findall(r"iou:(\w+)=([0-9.]+)", line))
shutil.copy(bk, p)
