#!/usr/bin/env python3
"""🧮 [audit12] G46 live-composition audit: per batch state on live, per multi-batch file the hash chain in landing order, and whether every batch's own base→new hunks are still present in live (reverse patch applies with `git apply --check -R`).
usage: python3 -I compose.py [--root DIR] [--out DIR]   (writes chain.json + per-file diffs under --out)"""
import hashlib, json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ORDER = ["agents16-tools/staged/allrun", "agents16-tools/staged/farmfree", "agents16-tools/staged/behind", "farmid-tools/staged/authority", "td-graph-tools/staged/demand", "optcache-tools/staged/store", "pinpub-tools/staged/nxsparse", "audit10-tools/staged/multiwriter", "audit9-tools/staged/jloss", "edit30-tools/staged/freshmtime", "playdemo-tools/staged/vitestext", "playdemo-tools/staged/wasmlock", "cbudget-tools/staged/clock", "rusttests-tools/staged/envelope", "audit10-tools/staged/split", "windows-tools/staged/winlaunch", "windows-tools/staged/fleetpy", "audit10-tools/staged/wininstall", "cisource-tools/staged/cisrc", "pinpub-tools/staged/pinned", "cmdid-tools/staged/identity"] + [f"progress-tools/staged/{b}" for b in "umlaut lang status rows sigint usage phases say".split()] + ["winps-tools/staged/native", "winps-tools/staged/pin"]
def digest(path):
    if not os.path.exists(path): return None
    with open(path, "rb") as f: return hashlib.sha256(f.read()).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def reverse_present(root, rel, base, new, scratch):
    os.makedirs(scratch, exist_ok=True)
    a, b, live = (os.path.join(scratch, n) for n in ("a", "b", "live"))
    open(a, "wb").write(open(base, "rb").read() if base and os.path.exists(base) else b"")
    open(b, "wb").write(open(new, "rb").read())
    open(live, "wb").write(open(os.path.join(root, rel), "rb").read() if os.path.exists(os.path.join(root, rel)) else b"")
    diff = subprocess.run(["git", "diff", "--no-index", "--no-color", "-U0", "a", "b"], cwd=scratch, capture_output=True).stdout
    if not diff: return "empty-diff", 0
    patch = diff.replace(b"a/a", b"a/live").replace(b"b/b", b"b/live")
    open(os.path.join(scratch, "p.diff"), "wb").write(patch)
    hunks = patch.count(b"\n@@ ")
    r = subprocess.run(["git", "apply", "--check", "-R", "--unidiff-zero", "p.diff"], cwd=scratch, capture_output=True)
    return ("present" if r.returncode == 0 else "MISSING:" + r.stderr.decode(errors="replace").strip().splitlines()[0][:160]), hunks
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(C)
    out = os.path.abspath(sys.argv[sys.argv.index("--out") + 1]) if "--out" in sys.argv else os.path.join(C, "🗑️generated", "audit12")
    files, report = {}, {"batches": {}, "multi": {}}
    for i, b in enumerate(ORDER):
        rows = json.load(open(os.path.join(C, b, "manifest.json"), encoding="utf8"))
        states = {}
        for rel, row in rows.items():
            live = digest(os.path.join(root, rel))
            kind = "applied" if live == row["new"] else "base" if live == row["base"] else "drifted"
            base = os.path.join(C, b, "base", row["file"]) if row["base"] else None
            pres, hunks = reverse_present(root, rel, base, os.path.join(C, b, "files", row["file"]), os.path.join(out, "scratch"))
            states[rel] = {"state": kind, "present": pres, "hunks": hunks}
            files.setdefault(rel, []).append({"batch": b, "order": i, "base": row["base"], "new": row["new"], "state": kind, "present": pres})
        report["batches"][b] = states
    for rel, chain in files.items():
        if len(chain) < 2: continue
        links = [{"from": p["batch"], "to": n["batch"], "linked": p["new"] == n["base"]} for p, n in zip(chain, chain[1:])]
        report["multi"][rel] = {"chain": chain, "links": links, "live": digest(os.path.join(root, rel))}
    os.makedirs(out, exist_ok=True)
    json.dump(report, open(os.path.join(out, "chain.json"), "w"), ensure_ascii=False, indent=1)
    for b, states in report["batches"].items():
        bad = {r: s for r, s in states.items() if s["state"] != "applied" or not s["present"].startswith(("present", "empty"))}
        print(f"{b}: {len(states)} files, {len(states) - len(bad)} applied+present", *(f"\n   {s['state']} {s['present']} {r[-100:]}" for r, s in bad.items()))
    print("MULTI-BATCH FILES:", len(report["multi"]))
    for rel, m in report["multi"].items():
        print(" ", rel[-110:]); [print(f"     {c['batch']}: {c['state']} {c['present']}") for c in m["chain"]]; [print(f"     link {l['from']} -> {l['to']}: {l['linked']}") for l in m["links"]]
main()
