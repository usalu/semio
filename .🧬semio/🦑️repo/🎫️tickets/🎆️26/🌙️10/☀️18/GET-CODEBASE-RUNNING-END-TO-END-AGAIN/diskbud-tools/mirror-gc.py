#!/usr/bin/env python3
"""🧹 Lock-aware retention for the shared mirrors below `$TMPDIR/sm` (ticket tooling, stdlib only). Default: report every mirror (age, apparent size, attached images, processes inside) and which ones the rule would drop. `--apply` needs `--prefix` (a lane never drops another lane's mirror), detaches every image attached below the mirror FIRST (a dropped mirror whose images stay attached keeps their disk space until the detach) and skips a mirror whose image cannot be detached or in which a process runs; the drop itself is `mirror.py drop`.
usage: mirror-gc.py [--older-than-hours N=6] [--prefix P]... [--apply] [--home DIR]"""
import argparse, json, os, subprocess, sys, tempfile, time
HERE = os.path.dirname(os.path.abspath(__file__))
MIRROR = os.path.join(os.path.dirname(HERE), "mirror-tools", "mirror.py")

def plan(mirrors, now, older_than_hours, prefixes):
    """Pure rule: a mirror is droppable when it is at least `older_than_hours` old, carries one of the prefixes (all when none are given: report only), has no attached image it cannot detach and no process inside."""
    rows = []
    for row in mirrors:
        age = (now - row["created"]) / 3600
        reasons = []
        if age < older_than_hours: reasons.append("young")
        if prefixes and not any(row["name"].startswith(p) for p in prefixes): reasons.append("other lane")
        if row["busy"]: reasons.append("process inside")
        rows.append({"name": row["name"], "ageHours": round(age, 1), "droppable": not reasons, "reasons": reasons})
    return rows

def attached(home):
    out = subprocess.run(["hdiutil", "info", "-plist"], capture_output=True, text=True).stdout if sys.platform == "darwin" else ""
    if not out: return []
    import plistlib
    found = []
    for image in plistlib.loads(out.encode()).get("images", []):
        for entity in image.get("system-entities", []):
            mount = entity.get("mount-point")
            if mount and (mount + os.sep).startswith(home + os.sep): found.append(mount)
    return found

def busy(root):
    if sys.platform == "win32": return False
    out = subprocess.run(["lsof", "-w", "-d", "cwd", "-Fn"], capture_output=True, text=True).stdout
    return any(line[1:] == root or line[1:].startswith(root + os.sep) for line in out.splitlines() if line.startswith("n"))

def size_kib(root):
    r = subprocess.run(["du", "-sk", "-x", root], capture_output=True, text=True)
    return int(r.stdout.split()[0]) if r.stdout else 0

def main():
    parser = argparse.ArgumentParser(); parser.add_argument("--older-than-hours", type=float, default=6); parser.add_argument("--prefix", action="append", default=[]); parser.add_argument("--apply", action="store_true"); parser.add_argument("--home")
    args = parser.parse_args()
    home = os.path.realpath(args.home or os.environ.get("SEMIO_MIRROR_HOME") or os.path.join(tempfile.gettempdir(), "sm")); meta = os.path.join(home, ".meta")
    if args.apply and not args.prefix: sys.exit("--apply needs --prefix <lane>- (mirrors are shared between lanes)")
    mirrors = []
    for entry in sorted(os.listdir(meta)) if os.path.isdir(meta) else []:
        if entry.endswith(".json"):
            data = json.load(open(os.path.join(meta, entry), encoding="utf8")); root = os.path.join(home, data["name"])
            mirrors.append({"name": data["name"], "created": data["created"], "busy": busy(root), "root": root})
    rows = plan(mirrors, time.time(), args.older_than_hours, args.prefix)
    mounts = attached(home)
    for row, mirror in zip(rows, mirrors):
        row["attached"] = [m for m in mounts if m.startswith(mirror["root"] + os.sep)]; row["sizeMiB"] = size_kib(mirror["root"]) // 1024
        print(json.dumps(row, ensure_ascii=False))
    if not args.apply: return
    for row, mirror in zip(rows, mirrors):
        if not row["droppable"]: continue
        failed = [m for m in row["attached"] if subprocess.run(["hdiutil", "detach", m], capture_output=True).returncode != 0]
        if failed: print(json.dumps({"name": row["name"], "skipped": "image busy", "mounts": failed}, ensure_ascii=False)); continue
        subprocess.run([sys.executable, "-I", MIRROR, "drop", row["name"]], check=False)
main() if __name__ == "__main__" else None
