# 🚀️ WP5 apply: writes the verified working copy of the Shell element and the consumer overlay into the repo; refuses when a live file differs from the snapshot or from what the last apply wrote.
# usage: apply5.py [--dry]
import os, sys, shutil, subprocess, json, filecmp
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg
ST = cfg.GEN + "/apply-state"
LIVE = f"{cfg.ROOT}/{cfg.SHELL_REL}"

def same(a, b):
    r = subprocess.run(["diff", "-rq", "--exclude=node_modules", a, b], capture_output=True, text=True)
    return r.stdout.strip() == "", r.stdout[:2000]

def main(dry):
    first = not os.path.exists(ST + "/applied-shell")
    ok, why = same(cfg.ORIG if first else ST + "/applied-shell", LIVE)
    assert ok, "live Shell element differs from " + ("the snapshot" if first else "the last applied tree") + " (peer edit?):\n" + why
    ov = cfg.GEN + "/new-ext"; files = []
    for dp, dn, fn in os.walk(ov):
        for f in fn: files.append(os.path.relpath(os.path.join(dp, f), ov))
    for rel in files:
        live = f"{cfg.ROOT}/{rel}"
        base = f"{ST}/ext-applied/{rel}" if not first and os.path.exists(f"{ST}/ext-applied/{rel}") else None
        if base: assert open(live, "rb").read() == open(base, "rb").read(), "external file differs from the last apply: " + rel
        else:
            if os.path.exists(f"{ST}/ext-orig/{rel}"): assert open(live, "rb").read() == open(f"{ST}/ext-orig/{rel}", "rb").read(), "external file changed: " + rel
    print("guards passed;", len(files), "external files")
    if dry: return
    if first:
        shutil.copytree(cfg.ORIG, ST + "/orig-shell", ignore=shutil.ignore_patterns("node_modules"))
        for rel in files:
            os.makedirs(os.path.dirname(f"{ST}/ext-orig/{rel}"), exist_ok=True); shutil.copyfile(f"{cfg.ROOT}/{rel}", f"{ST}/ext-orig/{rel}")
    subprocess.check_call(["rsync", "-a", "--delete", "--exclude", "node_modules", cfg.NEW + "/", LIVE + "/"])
    for rel in files:
        os.makedirs(os.path.dirname(f"{cfg.ROOT}/{rel}"), exist_ok=True); shutil.copyfile(f"{ov}/{rel}", f"{cfg.ROOT}/{rel}")
        os.makedirs(os.path.dirname(f"{ST}/ext-applied/{rel}"), exist_ok=True); shutil.copyfile(f"{ov}/{rel}", f"{ST}/ext-applied/{rel}")
    shutil.rmtree(ST + "/applied-shell", ignore_errors=True)
    shutil.copytree(cfg.NEW, ST + "/applied-shell", ignore=shutil.ignore_patterns("node_modules"))
    json.dump(files, open(ST + "/ext-files.json", "w"), ensure_ascii=False)
    print("applied")

if __name__ == "__main__": main("--dry" in sys.argv)
