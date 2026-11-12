import json, subprocess, glob, os, re
bun = glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun"))[0]
T = os.path.dirname(os.path.abspath(__file__)) + "/.."
slugs = "pregnant-person runner-with-blade-prosthesis running-child seated-guitarist seated-office-worker seated-reader skateboarder sleeping-person standing-adult standing-phone-user tour-guide walking-commuter wheelchair-user worker-carrying-box".split()
rows = []
for s in slugs:
    j = json.loads(subprocess.run([bun, T + "/volume-check.ts", s, "--json"], capture_output=True, text=True).stdout)
    v = subprocess.run([bun, "validate.ts", s], capture_output=True, text=True).stdout
    vi = int(re.search(r"volume issues (\d+)", v).group(1))
    pc = subprocess.run([bun, "posecheck.ts", s, "--all"], capture_output=True, text=True).stdout.splitlines()
    poses = [l for l in pc if " pose " in l]
    dflt = [l for l in poses if "(DEFAULT)" in l][0]
    bbd = re.search(r"bbox ([\d.]+) x ([\d.]+) x ([\d.]+) \(dimensions ([\d.]+) x ([\d.]+) x ([\d.]+)\)", dflt)
    g = [float(x) for x in bbd.groups()]
    dflt_err = [(g[i] - g[i + 3]) / g[i + 3] * 100 for i in range(3)]
    issues = sum(int(re.search(r", (\d+) issues?", l).group(1)) for l in poses)
    rows.append({"slug": s, "solids": j["solids"], "bbox": [round(b["relativeError"] * 100, 1) for b in j["bbox"]], "iou": [round(x["iou"], 3) for x in j["views"]], "vi": vi, "poses": len(poses), "pose_issues": issues, "dflt_err": [round(x, 1) for x in dflt_err], "pass": j["ok"], "detail": [l for l in pc if l.startswith("   ")]})
json.dump(rows, open(T + "/🗑️generated/authoring-humans-b/collect.json", "w"), ensure_ascii=False, indent=1)
for r in rows: print(r["slug"], r["solids"], r["bbox"], r["iou"], r["vi"], r["poses"], r["pose_issues"], r["dflt_err"], r["pass"])
