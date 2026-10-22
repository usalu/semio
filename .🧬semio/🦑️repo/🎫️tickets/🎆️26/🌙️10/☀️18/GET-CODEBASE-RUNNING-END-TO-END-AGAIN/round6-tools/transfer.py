#!/usr/bin/env python3
"""Replays ONE simulated step onto the live checkout without running any tool there: the sim snapshots every file a step changed (/private/tmp/i6/snap/<tag>/<step>/), this script replaces exactly those files in the checkout, each by atomic rename, after proving the live file still equals the state the step started from (the latest earlier snapshot of that path, else the pristine base). Any difference = conflict: nothing is written.
usage: transfer.py <step> [--tag a] [--dry] [--force-conflicts p1,p2] | transfer.py --undo <step> [--force]
Backups and ledger: C/🗑️generated/integrate3-apply/ (numbered files + ledger.json); the checkout never sees a half-applied set longer than the rename loop (seconds)."""
import hashlib, json, os, shutil, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio"
CREL = os.path.relpath(C, LIVE)
BASE = "/private/tmp/i6/base"
GEN = C + "/🗑️generated/integrate3-apply"
LEDGER = GEN + "/ledger.json"
args = sys.argv[1:]
opt = lambda n, d=None: args[args.index(n) + 1] if n in args else d
TAG = opt("--tag", "a")
sys.path.insert(0, HERE)
import steps as STEPS  # noqa: E402


def read(path):
    if os.path.islink(path):
        return b"\0link:" + os.readlink(path).encode("utf8")
    return open(path, "rb").read() if os.path.isfile(path) else None


def ledger():
    return json.load(open(LEDGER, encoding="utf8")) if os.path.isfile(LEDGER) else {}


def save(l):
    os.makedirs(GEN, exist_ok=True)
    json.dump(l, open(LEDGER + ".tmp", "w", encoding="utf8"), ensure_ascii=False, indent=1)
    os.replace(LEDGER + ".tmp", LEDGER)


def three_way(live, base, new):
    """Merges the step's change (base -> new) into the live text when a peer edited the file meanwhile; None when it does not merge cleanly or the file is binary or absent."""
    import subprocess, tempfile
    if live is None or base is None or live.startswith(b"\0link:") or b"\0" in live or b"\0" in new:
        return None
    with tempfile.TemporaryDirectory() as folder:
        for name, data in (("live", live), ("base", base), ("new", new)):
            open(os.path.join(folder, name), "wb").write(data)
        run = subprocess.run(["git", "merge-file", "-p", os.path.join(folder, "live"), os.path.join(folder, "base"), os.path.join(folder, "new")], capture_output=True)
    return run.stdout if run.returncode == 0 else None


def undo(step):
    l = ledger()
    e = l.get(step)
    if not e:
        sys.exit("nothing recorded for " + step)
    folder = GEN + "/backup/" + step
    bad = []
    for rel, row in e["files"].items():
        cur = read(os.path.join(LIVE, rel))
        h = hashlib.sha256(cur).hexdigest() if cur is not None else None
        if h != row["after"] and "--force" not in args:
            bad.append(rel)
    if bad:
        sys.exit("refused: %d file(s) changed after the step (--force overrides): %s" % (len(bad), bad[:5]))
    for rel, row in e["files"].items():
        t = os.path.join(LIVE, rel)
        if row["before"] is None:
            if os.path.lexists(t):
                os.remove(t)
        else:
            tmp = t + ".i3undo"
            shutil.copyfile(os.path.join(folder, row["backup"]), tmp)
            os.chmod(tmp, row["mode"])
            os.replace(tmp, t)
    del l[step]
    save(l)
    print("restored", step, len(e["files"]), "files")


def main():
    if "--undo" in args:
        return undo(opt("--undo"))
    step = args[0]
    ids = [s["id"] for s in STEPS.steps("/x", "/y", LIVE)]
    earlier = ids[: ids.index(step)]
    snaps = f"/private/tmp/i6/snap/{TAG}"
    res = {r["step"]: r for r in json.load(open(f"{C}/🗑️generated/round6-apply/sim-{TAG}/result.json", encoding="utf8"))["result"]}
    row = res[step]
    mine = f"{snaps}/{step}"
    if not os.path.isdir(mine):
        sys.exit("no snapshot for %s (step failed or sim older than snapshots)" % step)
    rels = [r for r in row["changed"] if not r.startswith(CREL + "/")]
    gone = [r for r in row["removed"] if not r.startswith(CREL + "/")]

    def old_of(rel):
        for e in reversed(earlier):
            p = f"{snaps}/{e}/{rel}"
            if os.path.lexists(p):
                return read(p)
        return read(os.path.join(BASE, rel))

    plan, conflicts, same, nmerged = [], [], 0, []
    forced = {x for x in (opt("--force-conflicts") or "").split(",") if x}
    for rel in rels:
        src = f"{mine}/{rel}"
        new = read(src)
        if new is None:
            continue
        live = read(os.path.join(LIVE, rel))
        if live == new:
            same += 1
            continue
        if b"/private/tmp/i6" in new:
            conflicts.append((rel, "overlay path leaked into the content"))
            continue
        if live == old_of(rel) or rel in forced:
            plan.append((rel, src, live, new))
        else:
            merged = three_way(live, old_of(rel), new)
            if merged is None:
                conflicts.append((rel, "live differs from the step's start state and the change does not merge"))
            else:
                plan.append((rel, None, live, merged))
                nmerged.append(rel)
    for rel in gone:
        live = read(os.path.join(LIVE, rel))
        if live is None:
            continue
        if live == old_of(rel):
            plan.append((rel, None, live, None))
        else:
            conflicts.append((rel, "removed file changed on the live tree"))
    print("%s: %d to write (%d merged with peer edits), %d already equal, %d conflict(s)" % (step, len(plan), len(nmerged), same, len(conflicts)))
    for rel in nmerged[:30]:
        print("   MERGED", rel[-110:])
    for rel, why in conflicts[:40]:
        print("   CONFLICT", rel[-120:], "-", why)
    if conflicts and not forced:
        return 3
    if "--dry" in args:
        return 0
    folder = GEN + "/backup/" + step
    shutil.rmtree(folder, ignore_errors=True)
    os.makedirs(folder)
    files = {}
    for i, (rel, src, live, new) in enumerate(plan):
        target = os.path.join(LIVE, rel)
        name = None
        mode = None
        if live is not None:
            name = "%05d.orig" % i
            if os.path.islink(target):
                open(os.path.join(folder, name), "wb").write(live)
            else:
                shutil.copyfile(target, os.path.join(folder, name))
            mode = os.lstat(target).st_mode & 0o777
        files[rel] = {"before": hashlib.sha256(live).hexdigest() if live is not None else None, "backup": name, "mode": mode, "after": hashlib.sha256(new).hexdigest() if new is not None else None}
    l = ledger()
    l[step] = {"time": time.strftime("%Y-%m-%d %H:%M:%S"), "files": files}
    save(l)
    for rel, src, live, new in plan:
        target = os.path.join(LIVE, rel)
        if new is None:
            os.remove(target)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        tmp = target + ".i3tmp"
        if src is None:
            open(tmp, "wb").write(new)
            if live is not None and not os.path.islink(target):
                os.chmod(tmp, os.lstat(target).st_mode & 0o777)
        elif os.path.islink(src):
            if os.path.lexists(tmp):
                os.remove(tmp)
            os.symlink(os.readlink(src), tmp)
        else:
            shutil.copyfile(src, tmp)
            os.chmod(tmp, os.lstat(src).st_mode & 0o777)
        os.replace(tmp, target)
    print("%s: written %s" % (step, time.strftime("%H:%M:%S")))
    return 0


sys.exit(main())
