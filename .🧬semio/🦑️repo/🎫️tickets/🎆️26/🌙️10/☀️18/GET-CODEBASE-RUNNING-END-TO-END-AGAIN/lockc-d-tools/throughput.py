"""Wall time of two concurrent small cargo builds of two independent workspaces under the custody lease: one shared build dir (serialised) vs one private lane dir each.
usage: throughput.py <scratch-root> <trials> ; cargo = repo-pinned nightly run directly in a scratch workspace outside the repository."""
import json, os, shutil, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
here = os.path.dirname(os.path.abspath(__file__)); repo = os.path.abspath(os.path.join(here, *[".."] * 8))
scratch, trials = os.path.abspath(sys.argv[1]), int(sys.argv[2])
tool = os.path.join(repo, ".cache/b/t/m/r/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin")
bun = subprocess.run("ls " + os.path.join(repo, ".cache/b/t/m/bun/*/bun"), shell=True, capture_output=True, text=True).stdout.split()[0]
runner = os.path.join(here, "custody-run.ts"); workspaces = [os.path.join(scratch, f"ws{i}") for i in range(2)]
for i, ws in enumerate(workspaces): subprocess.run([sys.executable, os.path.join(here, "..", "lockc-c-tools", "gen-ws.py"), ws, str(7 + i)], check=True, capture_output=True)
env = dict(os.environ, PATH=tool + ":" + os.environ["PATH"])
def build(leases, builddir, targetdir, top, ws):
    e = dict(env, CARGO_TARGET_DIR=targetdir, CARGO_BUILD_BUILD_DIR=builddir, LOCKC_VARIANT="0")
    cmd = [bun, runner, leases, builddir, "debug", "cargo", "build", "-j1", "--manifest-path", os.path.join(ws, "Cargo.toml"), "-p", top]
    return subprocess.Popen(cmd, env=e, cwd=ws, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
def wall(mode, trial):
    base = os.path.join(scratch, f"{mode}-{trial}"); shutil.rmtree(base, ignore_errors=True); leases = os.path.join(base, "leases"); os.makedirs(leases)
    if mode == "shared":
        dirs = [(os.path.join(base, "b"), os.path.join(base, "t"))] * 2
    else:
        dirs = [(os.path.join(base, f"lane{i}", "b"), os.path.join(base, f"lane{i}", "t")) for i in range(2)]
    start = time.time(); procs = [build(leases, d[0], d[1], f"t{i}", workspaces[i]) for i, d in enumerate(dirs)]
    waits = []
    for p in procs:
        out = p.communicate()[0]; assert p.returncode == 0, out
        waits.append(next(json.loads(l)["waitedMs"] for l in out.splitlines() if l.startswith('{"event":"custody"')))
    return {"mode": mode, "trial": trial, "wallS": round(time.time() - start, 2), "custodyWaitsMs": waits}
rows = []
for trial in range(trials):
    for mode in ("shared", "lanes"):
        row = wall(mode, trial); rows.append(row); print(json.dumps(row), flush=True)
for mode in ("shared", "lanes"):
    values = [r["wallS"] for r in rows if r["mode"] == mode]; print(json.dumps({"mode": mode, "medianWallS": sorted(values)[len(values) // 2], "values": values}))
