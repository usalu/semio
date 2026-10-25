"""Determinism probe of the rustc unit cache: build a crate in an isolated store (A), remember the cache entry of the crate, delete that entry (like the GC), build again in another isolated store (B: the crate is compiled again) and compare the content digests of the two entries' outputs. Usage: determinism.py <crate> <package> [extra cargo args]"""
import json, os, subprocess, sys, shutil, time
ROOT = "/Users/ueli/Documents/semio"
U = ROOT + "/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u"
crate, package = sys.argv[1], sys.argv[2]
extra = sys.argv[3:]
def entries():
    rows = {}
    for p1 in os.listdir(U + "/e"):
        for k in os.listdir(f"{U}/e/{p1}"):
            for f in os.listdir(f"{U}/e/{p1}/{k}"):
                try: d = json.load(open(f"{U}/e/{p1}/{k}/{f}"))
                except Exception: continue
                if d["crate"] == crate: rows[(k, f)] = d
    return rows
def build(tag):
    env = dict(os.environ, SEMIO_AGENT_ID="cap-c-det-" + tag, SEMIO_SCRIPT_STALL_MS="14400000")
    r = subprocess.run(["nice", "-n", "10", "bun", "repo", "cargo", "--store=isolated", "--configuration=prototype", "--", "build", "--manifest-path", "Cargo.toml", "-p", package, *extra], cwd=ROOT, env=env, capture_output=True, text=True)
    print(tag, "rc", r.returncode, r.stderr[-300:].replace("\n", " | "), flush=True)
    return r.returncode
before = set(entries())
if build("a"): sys.exit(1)
new = {k: v for k, v in entries().items() if k not in before} or entries()
print("entries after A:", len(new))
mine = sorted(new.items(), key=lambda kv: kv[1]["createdMs"])[-1]
(key, name), a = mine
print("key", key[:12], [(o["name"][-30:], o["sha256"][:10]) for o in a["outputs"]], flush=True)
shutil.rmtree(f"{U}/e/{key[:2]}/{key}")
if build("b"): sys.exit(1)
second = [v for (k, f), v in entries().items() if k == key]
print("entries for the key after B:", len(second))
if second:
    b = second[-1]
    for x, y in zip(a["outputs"], b["outputs"]):
        print(x["name"][-40:], "same" if x["sha256"] == y["sha256"] else "DIFFERENT", x["sha256"][:10], y["sha256"][:10])
