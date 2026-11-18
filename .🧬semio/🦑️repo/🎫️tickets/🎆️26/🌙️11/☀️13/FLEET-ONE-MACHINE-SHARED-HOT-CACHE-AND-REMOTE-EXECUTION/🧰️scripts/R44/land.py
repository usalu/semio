#!/usr/bin/env python3
"""Landing of package R44 (order-independent JSON extraction in the provisioning scripts) onto the shared-worktree module, under the binding 1113 landing protocol.

  land.py --dry-run            print, per file, the live sha256, whether the live file still equals the base the overlay started from (fast-forward), needs a three-way merge (git merge-file, base = the base copy, ours = live, theirs = overlay) or conflicts; nothing is written, no lock is taken
  land.py --land              lock (mkdir 1113-landing.lock), re-read, merge, write by rename, bash -n, load-check, run every test file under every tests directory of the module, record the landing in 1113-landings.md, unlock; any failure restores the originals
  land.py --rollback [STAMP]  restore the originals of a landing (a file that changed after the landing is merged backwards, never overwritten)

Options: --root (default: the live worktree), --overlay/--base (default: the folders next to this file), --only SUBSTRING (run only test files whose path contains it; repeatable), --skip-tests, --keep-on-fail, --label, --bun.
Python 3.9 compatible; it changes nothing outside --root, its backup folder and the two ticket files."""
import argparse
import datetime
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_ROOT = "/Users/ueli/Documents/semio"
MODULE = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
TOPOLOGY = MODULE + "/🗺️topology"
PROVISION = TOPOLOGY + "/🥾️provision"
FILES = [
    ("provision.sh", PROVISION + "/🐚️.sh", True),
    ("provision.ps1", PROVISION + "/🔵️.ps1", False),
    ("provision-tests.ts", PROVISION + "/🧪️tests/🟦️.ts", False),
    ("provision-oracle.py", PROVISION + "/🧪️tests/🐍️.py", False),
    ("topology-tests.ts", TOPOLOGY + "/🧪️tests/🟦️.ts", False),
    ("extraction-tests.ts", PROVISION + "/🧪️tests/🟦️extraction.ts", False),
]
NEW_FILES = {"extraction-tests.ts"}
PROTOCOL_TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
FLEET_TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
BUN = "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
LOCK_STALE_SECONDS = 30 * 60


def sha(path):
    if not os.path.isfile(path):
        return None
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def say(text):
    print("[land R44] " + text, flush=True)


def merge(live, base, theirs, out):
    """Three-way merge into `out`: returns the number of conflicts (git merge-file; it never touches a repository)."""
    shutil.copyfile(live, out)
    run = subprocess.run(["git", "merge-file", "-L", "live", "-L", "base", "-L", "overlay", out, base, theirs], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if run.returncode < 0 or run.returncode > 127:
        raise RuntimeError("git merge-file failed: " + run.stderr.decode("utf8", "replace"))
    return run.returncode


def plan(args):
    rows = []
    for name, relative, executable in FILES:
        live = os.path.join(args.root, relative)
        base = os.path.join(args.base, name)
        theirs = os.path.join(args.overlay, name)
        row = {"name": name, "relative": relative, "live": live, "base": base, "theirs": theirs, "executable": executable}
        row["liveSha"], row["baseSha"], row["theirsSha"] = sha(live), sha(base), sha(theirs)
        if row["theirsSha"] is None:
            raise SystemExit("missing overlay file " + theirs)
        if name in NEW_FILES:
            row["mode"] = "new" if row["liveSha"] is None else ("already-landed" if row["liveSha"] == row["theirsSha"] else "refused-exists")
        elif row["liveSha"] is None:
            row["mode"] = "refused-missing"
        elif row["liveSha"] == row["theirsSha"]:
            row["mode"] = "already-landed"
        elif row["liveSha"] == row["baseSha"]:
            row["mode"] = "fast-forward"
        else:
            row["mode"] = "merge"
        rows.append(row)
    return rows


def resolve(row, workdir):
    """The content that the live file gets: the overlay itself, or the merged text; returns (path, conflicts)."""
    if row["mode"] in ("new", "fast-forward"):
        return row["theirs"], 0
    out = os.path.join(workdir, row["name"] + ".merged")
    conflicts = merge(row["live"], row["base"], row["theirs"], out)
    return out, conflicts


def check_syntax(row, path):
    if row["name"].endswith(".sh"):
        run = subprocess.run(["bash", "-n", path], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return run.returncode == 0, run.stderr.decode("utf8", "replace")
    if row["name"].endswith(".py"):
        run = subprocess.run([sys.executable, "-I", "-c", "import ast,sys;ast.parse(open(sys.argv[1],encoding='utf8').read())", path], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return run.returncode == 0, run.stderr.decode("utf8", "replace")
    if row["name"].endswith(".ts") and os.path.exists(BUN):
        run = subprocess.run([BUN, "build", "--no-bundle", path], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return run.returncode == 0, run.stderr.decode("utf8", "replace")
    return True, ""


def dry_run(args):
    rows = plan(args)
    with tempfile.TemporaryDirectory(prefix="land-r44-") as workdir:
        bad = False
        for row in rows:
            path, conflicts = (None, 0)
            if row["mode"].startswith("refused"):
                bad = True
            elif row["mode"] != "already-landed":
                path, conflicts = resolve(row, workdir)
                ok, message = check_syntax(row, path)
                if conflicts or not ok:
                    bad = True
                row["after"] = sha(path)
                row["syntax"] = "ok" if ok else "FAILED " + message.strip()[:200]
            say("%-20s %-15s live %s base %s overlay %s%s%s" % (row["name"], row["mode"], (row["liveSha"] or "-")[:12], (row["baseSha"] or "-")[:12], row["theirsSha"][:12], " conflicts=%d" % conflicts if conflicts else "", " " + row.get("syntax", "") if "syntax" in row else ""))
    say("dry run %s: nothing was written" % ("FOUND PROBLEMS" if bad else "clean"))
    return 1 if bad else 0


def lock_directory(args):
    return os.path.join(args.root, PROTOCOL_TICKET, "1113-landing.lock")


def acquire(args):
    lock = lock_directory(args)
    for _ in range(2):
        try:
            os.mkdir(lock)
            with open(os.path.join(lock, "owner"), "w", encoding="utf8") as handle:
                handle.write("%s R44 %s\n" % (args.label, datetime.datetime.now().isoformat(timespec="seconds")))
            return
        except FileExistsError:
            owner = os.path.join(lock, "owner")
            age = datetime.datetime.now().timestamp() - os.path.getmtime(owner) if os.path.exists(owner) else 0
            if age > LOCK_STALE_SECONDS and args.take_stale:
                say("taking over a lock older than 30 minutes: " + open(owner, encoding="utf8").read().strip())
                shutil.rmtree(lock)
                continue
            raise SystemExit("the landing lock is held (%s, %d s old): wait and retry; a lock older than 30 minutes may be taken with --take-stale" % (open(owner, encoding="utf8").read().strip() if os.path.exists(owner) else "no owner file", age))
    raise SystemExit("could not take the landing lock")


def release(args):
    shutil.rmtree(lock_directory(args), ignore_errors=True)


def write_by_rename(target, source, executable, like):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    temporary = target + ".land-r44.%d" % os.getpid()
    shutil.copyfile(source, temporary)
    mode = os.stat(like).st_mode & 0o777 if like and os.path.exists(like) else (0o755 if executable else 0o644)
    os.chmod(temporary, mode)
    os.replace(temporary, target)


def test_files(args):
    found = []
    for directory, names, files in os.walk(os.path.join(args.root, MODULE)):
        names[:] = [name for name in names if name not in ("node_modules", "🧫️fixtures", ".git")]
        if os.path.basename(directory) == "🧪️tests" or "🧪️tests" in directory.split(os.sep):
            for name in files:
                if name.endswith(".ts") and not name.endswith(".d.ts"):
                    found.append(os.path.join(directory, name))
    found.sort()
    if args.only:
        found = [path for path in found if any(part in path for part in args.only)]
    return found


def run_tests(args):
    env = {key: value for key, value in os.environ.items() if not key.startswith("SEMIO_")}
    failures = []
    results = []
    for path in test_files(args):
        run = subprocess.run([args.bun, "test", "./" + os.path.basename(path), "--timeout", "180000"], cwd=os.path.dirname(path), env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        text = run.stdout.decode("utf8", "replace")
        red = [line for line in text.splitlines() if line.startswith("(fail)")]
        label = os.path.relpath(path, os.path.join(args.root, MODULE))
        say("test %s exit %d, %d red" % (label, run.returncode, len(red)))
        results.append((label, run.returncode, len(red)))
        if run.returncode != 0:
            failures.append((label, red[:5], text[-600:]))
    return results, failures


def backup_directory(args, stamp):
    return os.path.join(args.root, FLEET_TICKET, "🗑️generated", "R44-landing-" + stamp)


def land(args):
    rows = plan(args)
    if any(row["mode"].startswith("refused") for row in rows):
        raise SystemExit("refused: " + ", ".join("%s (%s)" % (row["name"], row["mode"]) for row in rows if row["mode"].startswith("refused")))
    stamp = datetime.datetime.now().strftime("%m%d%H%M%S")
    backup = backup_directory(args, stamp)
    acquire(args)
    landed = []
    try:
        rows = plan(args)
        os.makedirs(backup, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix="land-r44-") as workdir:
            staged = []
            for row in rows:
                if row["mode"] == "already-landed":
                    say("%s is already landed" % row["name"])
                    continue
                path, conflicts = resolve(row, workdir)
                if conflicts:
                    raise RuntimeError("%s: %d merge conflicts with the live file; resolve them by hand (land.py --dry-run shows the state)" % (row["name"], conflicts))
                ok, message = check_syntax(row, path)
                if not ok:
                    raise RuntimeError("%s fails its syntax check: %s" % (row["name"], message.strip()[:300]))
                staged.append((row, path))
            manifest = []
            for row, path in staged:
                before = row["liveSha"]
                if before is not None:
                    destination = os.path.join(backup, row["relative"])
                    os.makedirs(os.path.dirname(destination), exist_ok=True)
                    shutil.copy2(row["live"], destination)
                write_by_rename(row["live"], path, row["executable"], row["live"])
                after = sha(row["live"])
                manifest.append({"name": row["name"], "relative": row["relative"], "before": before, "after": after, "base": row["baseSha"], "overlay": row["theirsSha"], "mode": row["mode"]})
                landed.append(row)
                os.makedirs(os.path.join(backup, "landed"), exist_ok=True)
                shutil.copyfile(row["live"], os.path.join(backup, "landed", row["name"]))
                say("landed %-20s %s -> %s (%s)" % (row["name"], (before or "new")[:12], after[:12], row["mode"]))
                with open(os.path.join(backup, "manifest.json"), "w", encoding="utf8") as handle:
                    json.dump(manifest, handle, indent=1)
        failures = []
        results = []
        if not args.skip_tests:
            results, failures = run_tests(args)
        if failures:
            raise RuntimeError("test failures: " + json.dumps([[label, red] for label, red, _ in failures], ensure_ascii=False))
        record(args, stamp, manifest, results, backup)
        say("landed; originals for land.py --rollback %s: %s" % (stamp, backup))
    except Exception as error:
        say("FAILED: %s" % error)
        if landed and not args.keep_on_fail:
            say("restoring the originals")
            rollback_from(args, backup)
        elif landed:
            say("--keep-on-fail: the landed files stay; originals are in " + backup)
        release(args)
        return 1
    release(args)
    return 0


def record(args, stamp, manifest, results, backup):
    path = os.path.join(args.root, PROTOCOL_TICKET, "1113-landings.md")
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    lines = ["", "## Package R44 landing (%s): order-independent JSON extraction in the provisioning scripts, with its law tests" % now]
    for item in manifest:
        lines.append("- copy `%s` before %s after %s" % (item["relative"].replace(MODULE + "/", ""), (item["before"] or "new")[:12], item["after"][:12]))
    for label, code, red in results:
        lines.append("- test `%s` exit %d, %d red" % (label, code, red))
    lines += [
        "- restarts: none. The code digest of the replica does not change (the provisioning scripts are sealed companions, the tests are outside the import closure): no replica daemon, relay, cache, scheduler or agent restarts on any machine.",
        "- kit: the group server's kit maintenance (relay/attest loop) re-seals the changed companions under the same digest and republishes `v1/bootstrap/kit` (SHA256SUMS and `published` change); a machine that provisions after that gets the new scripts, joined machines get the files through the replica like any file.",
        "- originals for `land.py --rollback %s`: `%s`" % (stamp, backup),
    ]
    with open(path, "a", encoding="utf8") as handle:
        handle.write("\n".join(lines) + "\n")


def rollback_from(args, backup):
    with open(os.path.join(backup, "manifest.json"), encoding="utf8") as handle:
        manifest = json.load(handle)
    with tempfile.TemporaryDirectory(prefix="land-r44-back-") as workdir:
        for item in manifest:
            live = os.path.join(args.root, item["relative"])
            original = os.path.join(backup, item["relative"])
            current = sha(live)
            if item["before"] is None:
                if current == item["after"]:
                    os.remove(live)
                    say("removed %s (new file of the landing)" % item["name"])
                else:
                    say("KEPT %s: it changed after the landing" % item["name"])
                continue
            if current == item["after"]:
                write_by_rename(live, original, False, live)
                say("restored %s" % item["name"])
            else:
                landed = os.path.join(backup, "landed", item["name"])
                out = os.path.join(workdir, item["name"] + ".back")
                conflicts = merge(live, landed, original, out)
                if conflicts:
                    say("KEPT %s: backwards merge has %d conflicts; restore by hand from %s" % (item["name"], conflicts, original))
                else:
                    write_by_rename(live, out, False, live)
                    say("restored %s by a backwards merge (later edits kept)" % item["name"])


def rollback(args):
    root = os.path.join(args.root, FLEET_TICKET, "🗑️generated")
    stamps = sorted(name[len("R44-landing-"):] for name in os.listdir(root) if name.startswith("R44-landing-"))
    if not stamps:
        raise SystemExit("no R44 landing backup in " + root)
    stamp = args.stamp or stamps[-1]
    acquire(args)
    try:
        rollback_from(args, backup_directory(args, stamp))
        path = os.path.join(args.root, PROTOCOL_TICKET, "1113-landings.md")
        with open(path, "a", encoding="utf8") as handle:
            handle.write("\n## Package R44 rollback (%s) of landing %s\n- originals restored from `%s`\n" % (datetime.datetime.now().strftime("%Y-%m-%d %H:%M"), stamp, backup_directory(args, stamp)))
    finally:
        release(args)
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--root", default=DEFAULT_ROOT)
    parser.add_argument("--overlay", default=os.path.join(HERE, "overlay"))
    parser.add_argument("--base", default=os.path.join(HERE, "base"))
    parser.add_argument("--bun", default=BUN)
    parser.add_argument("--only", action="append", default=[])
    parser.add_argument("--label", default="coordinator")
    parser.add_argument("--skip-tests", action="store_true")
    parser.add_argument("--keep-on-fail", action="store_true")
    parser.add_argument("--take-stale", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--land", action="store_true")
    parser.add_argument("--rollback", action="store_true")
    parser.add_argument("stamp", nargs="?")
    args = parser.parse_args()
    args.root = os.path.realpath(args.root)
    if sum([args.dry_run, args.land, args.rollback]) != 1:
        parser.error("choose exactly one of --dry-run, --land, --rollback")
    return dry_run(args) if args.dry_run else land(args) if args.land else rollback(args)


if __name__ == "__main__":
    sys.exit(main())
