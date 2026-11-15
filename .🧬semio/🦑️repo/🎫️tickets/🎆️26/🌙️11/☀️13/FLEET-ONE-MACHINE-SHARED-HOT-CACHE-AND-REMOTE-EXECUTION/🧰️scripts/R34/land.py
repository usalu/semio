#!/usr/bin/env python3
"""R34 landing: the fleet verdict stops flagging machine-local files the replica never carries (view without refused untracked paths and unsafe untracked links), the hot path notes unsafe links, and `status` shows the record of the 200 newest refused paths (.cache/f/refused-paths.json). Moves the replica code digest of every machine (11 files of the shared-worktree module).
usage: land.py --dry-run | --rehearse | --land [--authorized] [--no-tests] | --rollback <backup-dir>      (--root <dir> uses another tree, for rehearsals)
  --dry-run   writes nothing: per file `copy` (live equals the baseline, the result is exactly the tested one), `merge` (live changed elsewhere, the diff still applies by context), `same` (already landed), REFUSED (does not apply: rebuild the bundle with build-bundle.py).
  --rehearse  copies the live version of every touched file into a scratch root, lands there, checks every `copy` file against the tested hash, rolls back, checks that the scratch root equals the copy again and that a dry run is clean; touches nothing live.
  --land      needs N/LAND-OK-R34 (or --authorized), takes landing.lock of the build ticket AND 1113-landing.lock of the shared-worktree ticket, keeps the originals in T/🗑️generated/R34-landing-<time>/, replaces all files back to back, runs the parse proof and every test file of the module, appends to landings.md and 1113-landings.md.
"""
import os, sys, json, subprocess, hashlib, time, shutil, datetime, tempfile, ast

HERE = os.path.dirname(os.path.abspath(__file__))


def arg(name):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else None


def root():
    if arg("--root"):
        return os.path.abspath(arg("--root"))
    for candidate in ["/Users/Shared/semio", "/Users/ueli/Documents/semio"]:
        if os.path.exists(candidate + "/AGENTS.md") and os.path.exists(candidate + "/.cache/f/machine.json"):
            return candidate
    sys.exit("no worktree root found")


R = root()
TICKETS = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11"
T = TICKETS + "/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
N = TICKETS + "/☀️14/WORKERS-READY-WITH-LOW-RESOURCES"
SW_TICKET = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
B = MODULES + "/📚️library/🏗️build"
SW = MODULES + "/🤝️shared-worktree"
BUN = os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
BUILD_TESTS = []


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


def module_tests():
    found = []
    for directory, names, files in os.walk(R + "/" + SW):
        names[:] = [name for name in names if name != "node_modules"]
        if os.path.basename(directory) == "🧪️tests" or "/🧪️tests/" in directory + "/":
            found += [os.path.relpath(os.path.join(directory, f), R) for f in sorted(files) if f.endswith(".ts")]
    return sorted(found)


def apply_patch(live_path, diff):
    with tempfile.TemporaryDirectory() as directory:
        patch = directory + "/d.patch"
        with open(patch, "w", encoding="utf8") as handle:
            handle.write(diff)
        run = subprocess.run(["patch", "-s", "-F3", "--no-backup-if-mismatch", "-o", directory + "/out", live_path, patch], capture_output=True, text=True)
        if run.returncode != 0 or not os.path.exists(directory + "/out"):
            return None, (run.stdout + run.stderr).strip()[:200]
        return read(directory + "/out"), ""


def plan(bundle):
    out, refused = {}, []
    for rel, diff in sorted(bundle["patches"].items()):
        path = R + "/" + rel
        if not os.path.exists(path):
            refused.append((rel, "live file is gone"))
            continue
        raw = read(path)
        if sha(raw) == bundle["result"][rel]:
            out[rel] = ("same", raw)
            continue
        data, why = apply_patch(path, diff)
        if data is None:
            refused.append((rel, "does not apply (the file changed in the lines of the diff): " + why))
            continue
        out[rel] = ("copy" if sha(raw) == bundle["baseline"][rel] and sha(data) == bundle["result"][rel] else "merge", data)
    for rel, text in sorted(bundle["full"].items()):
        path = R + "/" + rel
        data = text.encode("utf8")
        if not os.path.exists(path):
            refused.append((rel, "live file is gone"))
        elif sha(read(path)) == bundle["result"][rel]:
            out[rel] = ("same", data)
        elif sha(read(path)) == bundle["baseline"][rel]:
            out[rel] = ("copy", data)
        else:
            refused.append((rel, "changed since the overlay was made and carried in full: rebuild the bundle"))
    for rel, text in sorted(bundle["new"].items()):
        path = R + "/" + rel
        data = text.encode("utf8")
        if os.path.exists(path):
            if read(path) == data:
                out[rel] = ("same", data)
            else:
                refused.append((rel, "new file exists with other content"))
        else:
            out[rel] = ("new", data)
    return out, refused


def parse_proof(files):
    bad = []
    for rel in files:
        path = R + "/" + rel
        if rel.endswith(".ts"):
            run = subprocess.run([BUN, "build", "--no-bundle", path], capture_output=True, text=True)
            if run.returncode != 0:
                bad.append((rel, run.stderr[:200]))
        elif rel.endswith(".json"):
            try:
                json.load(open(path, encoding="utf8"))
            except Exception as error:
                bad.append((rel, str(error)))
        elif rel.endswith(".py"):
            try:
                ast.parse(open(path, encoding="utf8").read())
            except Exception as error:
                bad.append((rel, str(error)))
        elif rel.endswith(".sh"):
            run = subprocess.run(["bash", "-n", path], capture_output=True, text=True)
            if run.returncode != 0:
                bad.append((rel, run.stderr[:200]))
    return bad


def restarts(touched):
    """The services that pick up a changed file by their code watch, from the paths that change."""
    lines = []
    replica = [p for p in touched if p.startswith(R + "/" + SW + "/")]
    cache = [p for p in touched if p.startswith(R + "/" + B + "/🌐️remote-cache/") or p.startswith(R + "/" + B + "/🌐️remote-execution/")]
    if replica:
        lines.append("replica daemon of every machine (%d files of the shared-worktree module move the code digest; the 1113 rollout applies: group server first as canary, then the members, expect agreement waits); their sidecars are adopted, not restarted, unless their own closure changed" % len(replica))
        lines.append("relay LaunchDaemon tech.semio.fleet.relay on the group server runs the replica code from its own start: it keeps the old code (and the old relay messages) until the owner restarts it")
    if cache:
        lines.append("build-cache (group server): the cache server restarts; durable state is the journal and the index, claims come from the claim journal")
        lines.append("build-scheduler (group server): restarts through its code-closure watch (it imports the gate and the identity)")
        lines.append("build-agent and its graph server child on every machine: restart through the code-closure watch (exit 75)")
    if any(p.startswith(R + "/" + B + "/🔣️policy.json") for p in touched):
        lines.append("policy.json: read by every `repo` build command at its next start; no service is restarted by it alone")
    return lines


def take(lock, owner):
    try:
        os.mkdir(lock)
    except FileExistsError:
        try:
            who = open(lock + "/owner").read().strip()
        except OSError:
            who = "unknown"
        return who
    open(lock + "/owner", "w").write(owner + "\n")
    return None


def give(lock, owner):
    try:
        if open(lock + "/owner").read().strip() == owner:
            os.remove(lock + "/owner")
            os.rmdir(lock)
    except OSError:
        pass


def rollback(backup):
    lock = T + "/landing.lock"
    owner = "R34-rollback-%d" % os.getpid()
    who = take(lock, owner)
    if who:
        sys.exit("landing.lock is held by " + who)
    try:
        for directory, _, files in os.walk(backup):
            for name in files:
                rel = os.path.relpath(os.path.join(directory, name), backup)
                if name.endswith(".absent"):
                    path = R + "/" + rel[:-7]
                    if os.path.exists(path):
                        os.remove(path)
                    continue
                path = R + "/" + rel
                os.makedirs(os.path.dirname(path), exist_ok=True)
                with open(path + ".r34tmp", "wb") as handle:
                    handle.write(read(os.path.join(directory, name)))
                os.replace(path + ".r34tmp", path)
        print("restored from", backup)
    finally:
        give(lock, owner)


def show(out, refused):
    for rel, (how, data) in out.items():
        print(how.ljust(5), rel[len(MODULES) + 1:] if rel.startswith(MODULES) else rel, sha(data)[:12])
    for rel, why in refused:
        print("REFUSED", rel[len(MODULES) + 1:], why)


def land(bundle):
    out, refused = plan(bundle)
    show(out, refused)
    if refused:
        sys.exit("refused: a live file changed in the lines of the diff: rebuild the bundle with build-bundle.py")
    if not os.path.exists(N + "/LAND-OK-R34") and "--authorized" not in sys.argv:
        sys.exit("LAND-OK-R34 does not exist: the owner lands this package (it moves the replica code digest of every machine and restarts every replica daemon)")
    owner = "R34-%d-%d" % (os.getpid(), int(time.time()))
    locks = [T + "/landing.lock", SW_TICKET + "/1113-landing.lock"]
    held = []
    try:
        for lock in locks:
            os.makedirs(os.path.dirname(lock), exist_ok=True)
            who = take(lock, owner)
            if who:
                sys.exit(lock + " is held by " + who + ": wait, never remove it")
            held.append(lock)
        out, refused = plan(bundle)
        if refused:
            sys.exit("refused after taking the locks: " + "; ".join(name for name, _ in refused))
        stamp = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
        backup = T + "/🗑️generated/R34-landing-" + datetime.datetime.now().strftime("%H%M%S")
        before = {}
        for rel, (how, data) in out.items():
            if how == "same":
                continue
            path = R + "/" + rel
            before[rel] = sha(read(path)) if os.path.exists(path) else "-"
            os.makedirs(os.path.dirname(backup + "/" + rel), exist_ok=True)
            if os.path.exists(path):
                shutil.copyfile(path, backup + "/" + rel)
            else:
                open(backup + "/" + rel + ".absent", "w").close()
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path + ".r34tmp", "wb") as handle:
                handle.write(data)
            if os.path.exists(path):
                os.chmod(path + ".r34tmp", os.stat(path).st_mode & 0o7777)
        for rel, (how, data) in out.items():
            if how != "same":
                os.replace(R + "/" + rel + ".r34tmp", R + "/" + rel)
        changed = [rel for rel, (how, _) in out.items() if how != "same"]
        print("landed", len(changed), "files; originals in", backup)
        bad = parse_proof(changed)
        for rel, why in bad:
            print("PARSE FAIL", rel, why)
        results = []
        tests = [] if "--no-tests" in sys.argv else BUILD_TESTS + [t for t in module_tests() if t not in BUILD_TESTS]
        for rel in tests:
            cwd = os.path.dirname(R + "/" + rel)
            env = {k: v for k, v in os.environ.items() if not k.startswith("SEMIO_")}
            env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"] = "0"
            env["SEMIO_GRAPH_SERVER"] = "off"
            run = subprocess.run([BUN, "test", "./" + os.path.basename(rel), "--timeout", "180000"], cwd=cwd, env=env, capture_output=True, text=True)
            failed = [l for l in (run.stdout + run.stderr).splitlines() if l.startswith("(fail)")]
            results.append((rel, run.returncode, failed))
            print(("ok  " if run.returncode == 0 else "RED ") + rel[len(MODULES) + 1:], *failed, sep="\n  " if failed else " ")
        for name, where in (("landings.md", T), ("1113-landings.md", SW_TICKET)):
            os.makedirs(where, exist_ok=True)
            with open(where + "/" + name, "a", encoding="utf8") as handle:
                handle.write("\n## Package R34 landing (%s): the fleet view leaves out files the replica never carries, refused paths are recorded and shown by status\n" % stamp)
                for rel, (how, data) in out.items():
                    handle.write("- %s `%s` before %s after %s\n" % (how, rel[len(MODULES) + 1:] if rel.startswith(MODULES) else rel, before.get(rel, "-")[:12], sha(data)[:12]))
                for rel, code, failed in results:
                    handle.write("- test `%s` exit %d, %d red\n" % (rel[len(MODULES) + 1:], code, len(failed)))
                for line in restarts([R + "/" + c for c in changed]):
                    handle.write("- restarts: %s\n" % line)
                handle.write("- originals for `land.py --rollback`: `%s`\n" % backup)
        if bad or any(code != 0 for _, code, _ in results):
            print("RED result: inspect, then --rollback " + backup)
    finally:
        for lock in reversed(held):
            give(lock, owner)


def rehearse(bundle):
    out, refused = plan(bundle)
    if refused:
        show(out, refused)
        sys.exit("rehearsal needs a clean dry run first")
    scratch = tempfile.mkdtemp(prefix="r34-rehearsal-", dir=os.environ.get("R34_SCRATCH"))
    try:
        original = {}
        for rel in bundle["patches"].keys() | bundle["full"].keys():
            target = scratch + "/" + rel
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(R + "/" + rel, target)
            original[rel] = sha(read(target))
        for sub in (T, SW_TICKET, N):
            os.makedirs(scratch + sub[len(R):], exist_ok=True)
        me = [sys.executable, os.path.abspath(__file__), "--root", scratch]
        landed = subprocess.run(me + ["--land", "--authorized", "--no-tests"], capture_output=True, text=True)
        print(landed.stdout[-1500:], landed.stderr[-500:])
        if landed.returncode != 0:
            return False
        backups = sorted(os.listdir(scratch + T[len(R):] + "/🗑️generated"))
        bad = 0
        for rel, (how, data) in out.items():
            if how == "copy" and sha(read(scratch + "/" + rel)) != bundle["result"][rel]:
                print("RESULT DIFFERS", rel)
                bad += 1
            if how == "new" and sha(read(scratch + "/" + rel)) != bundle["result"][rel]:
                print("NEW FILE DIFFERS", rel)
                bad += 1
        undone = subprocess.run(me + ["--rollback", scratch + T[len(R):] + "/🗑️generated/" + backups[-1]], capture_output=True, text=True)
        print(undone.stdout.strip())
        for rel, digest in original.items():
            if sha(read(scratch + "/" + rel)) != digest:
                print("NOT RESTORED", rel)
                bad += 1
        for rel in bundle["new"]:
            if os.path.exists(scratch + "/" + rel):
                print("NEW FILE STILL THERE", rel)
                bad += 1
        again = subprocess.run(me + ["--dry-run"], capture_output=True, text=True)
        print(again.stdout.strip().splitlines()[-1] if again.stdout.strip() else again.stderr[-300:])
        print("rehearsal:", "clean" if bad == 0 and landed.returncode == 0 and undone.returncode == 0 else "FAILED (%d)" % bad)
        return bad == 0 and landed.returncode == 0 and undone.returncode == 0
    finally:
        real = os.path.realpath(scratch)
        if real.startswith(tempfile.gettempdir()) or real.startswith("/private/var/") or real.startswith("/private/tmp/"):
            shutil.rmtree(scratch, ignore_errors=True)


def main():
    modes = [x for x in sys.argv[1:] if x in ("--dry-run", "--land", "--rehearse", "--rollback")]
    mode = modes[0] if modes else "--dry-run"
    if mode == "--rollback":
        rollback(sys.argv[sys.argv.index("--rollback") + 1])
        return
    bundle = json.load(open(HERE + "/R34-bundle.json", encoding="utf8"))
    if mode == "--land":
        land(bundle)
        return
    if mode == "--rehearse":
        sys.exit(0 if rehearse(bundle) else 1)
    out, refused = plan(bundle)
    show(out, refused)
    changed = [R + "/" + rel for rel, (how, _) in out.items() if how != "same"]
    for line in restarts(changed):
        print("restarts:", line)
    print("dry run: nothing written" + ("; %d refused" % len(refused) if refused else "; clean"))
    if refused:
        sys.exit(1)


main()
