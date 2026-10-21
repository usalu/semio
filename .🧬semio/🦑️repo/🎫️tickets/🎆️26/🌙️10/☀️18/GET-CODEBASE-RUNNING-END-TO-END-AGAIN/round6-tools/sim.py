#!/usr/bin/env python3
"""Round-6 overlay simulation (never writes into the checkout: every step runs under a sandbox profile that denies writes below the live root).
usage: sim.py [--tag a] [--from step] [--to step] [--only a,b] [--continue] [--check]
  --check   run each step's dry-run command (no writes) instead of its write command
Result: C/🗑️generated/round6-apply/sim-<tag>/{result.json,<step>.log}; the overlay is built by overlay.py (clones of the checkout)."""
import json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
args = sys.argv[1:]
opt = lambda n, d=None: args[args.index(n) + 1] if n in args else d
TAG = opt("--tag", "a")
OV = "/private/tmp/i6/ov"
GEN = C + "/🗑️generated/round6-apply"
OUT = f"{GEN}/sim-{TAG}" + ("-check" if "--check" in args else "")
PROFILE = '(version 1)(allow default)(deny file-write* (subpath "%s"))(deny network-outbound (remote ip))' % LIVE
sys.path.insert(0, HERE)
from steps import steps  # noqa: E402


def inventory():
    rows = {}
    if "--check" in args and "--inv" not in args:
        return rows
    for base, dirs, files in os.walk(OV):
        dirs[:] = [d for d in dirs if d not in (".git", "node_modules")]
        for f in files:
            p = os.path.join(base, f)
            try:
                s = os.lstat(p)
            except OSError:
                continue
            rows[os.path.relpath(p, OV)] = (s.st_size, s.st_mtime_ns, s.st_ino, s.st_mode)
    return rows


def sync_tools():
    """Copies the live round-6 tool sources (scripts and makers, not their generated copies) into the overlay."""
    import glob, shutil
    dest = f"{OV}/{CREL}/round6-tools"
    os.makedirs(dest + "/staged", exist_ok=True)
    for pattern in ("*.py", "*.sh", "staged/*.py", "staged/*.sh", "staged/fixups/*"):
        for f in glob.glob(f"{HERE}/{pattern}"):
            if os.path.isfile(f):
                os.makedirs(os.path.dirname(f.replace(HERE, dest)), exist_ok=True)
                shutil.copyfile(f, f.replace(HERE, dest))


def clone(src, dst):
    """One clonefile(2) call clones a whole directory tree copy-on-write (about 30 s for the 194k-file checkout)."""
    import ctypes, ctypes.util
    libc = ctypes.CDLL(ctypes.util.find_library("c"), use_errno=True)
    if libc.clonefile(src.encode("utf8"), dst.encode("utf8"), 0) != 0:
        raise SystemExit("clonefile %s -> %s failed: errno %d" % (src, dst, ctypes.get_errno()))


def trash(path):
    """Moves a tree away and deletes it in a detached process (a 194k-file delete takes minutes)."""
    if os.path.lexists(path):
        gone = "%s.trash-%d" % (path, int(time.time() * 1000))
        os.rename(path, gone)
        subprocess.Popen(["rm", "-rf", gone], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)


def fresh(source="/private/tmp/i6/base"):
    trash(OV)
    clone(source, OV)


def main():
    if "--fresh" in args:
        fresh()
    if opt("--from-ckpt"):
        fresh("/private/tmp/i6/ckpt-" + opt("--from-ckpt"))
    sync_tools()
    table = steps(OV, C.replace(LIVE, OV), LIVE)
    names = [s["id"] for s in table]
    if opt("--only"):
        names = [n for n in names if n in opt("--only").split(",")]
    if opt("--from"):
        names = names[names.index(opt("--from")):]
    if opt("--to"):
        names = names[: names.index(opt("--to")) + 1]
    os.makedirs(OUT, exist_ok=True)
    result = []
    for s in table:
        if s["id"] not in names:
            continue
        cmds = s["check"] if "--check" in args else s["write"]
        if cmds is None:
            continue
        cmds = list(s.get("prepare", [])) + list(s.get("sim_prepare", [])) + list(cmds)
        before = inventory()
        t0 = time.time()
        log, rc = "", 0
        for cmd in cmds:
            if isinstance(cmd, dict):
                cmd, env_extra, cwd = cmd["cmd"], cmd.get("env", {}), cmd.get("cwd", OV)
            else:
                env_extra, cwd = s.get("env", {}), OV
            env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "SEMIO_ROOT": OV, "PATH": "/private/tmp/i6/bin:" + os.environ["PATH"], **env_extra}
            run = subprocess.run(["sandbox-exec", "-p", PROFILE, *cmd], capture_output=True, text=True, encoding="utf8", errors="replace", cwd=cwd, env=env, timeout=3600)
            log += "$ " + " ".join(cmd)[:300] + "\n" + run.stdout + "\n--- stderr ---\n" + run.stderr + "\n"
            rc = run.returncode
            if rc not in s.get("ok", (0,)):
                break
        after = inventory()
        changed = sorted(r for r in after if before.get(r) != after[r])
        gone = sorted(r for r in before if r not in after)
        row = {"step": s["id"], "rc": rc, "secs": round(time.time() - t0), "changed": changed, "new": [r for r in changed if r not in before], "removed": gone}
        open(f"{OUT}/{s['id']}.log", "w", encoding="utf8").write(log)
        result.append(row)
        ok = rc in s.get("ok", (0,))
        print(f"{'ok  ' if ok else 'FAIL'} rc={rc} {s['id']:26} changed {len(changed):5} (new {len(row['new'])}, removed {len(gone)}) {row['secs']}s" + ("" if ok else "\n    " + log.strip()[-700:].replace("\n", "\n    ")), flush=True)
        if ok and s["id"] in (opt("--ckpt-after") or "").split(","):
            trash("/private/tmp/i6/ckpt-" + s["id"])
            clone(OV, "/private/tmp/i6/ckpt-" + s["id"])
            print("     checkpoint ckpt-" + s["id"], flush=True)
        if not ok and "--continue" not in args:
            break
    prev = json.load(open(f"{OUT}/result.json", encoding="utf8"))["result"] if os.path.isfile(f"{OUT}/result.json") and ("--from" in args or "--only" in args) else []
    keep = [r for r in prev if r["step"] not in {x["step"] for x in result}]
    json.dump({"result": keep + result}, open(f"{OUT}/result.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)


main()
