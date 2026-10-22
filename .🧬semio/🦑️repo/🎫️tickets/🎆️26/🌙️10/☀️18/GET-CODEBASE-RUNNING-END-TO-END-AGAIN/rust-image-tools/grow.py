#!/usr/bin/env python3
"""📏️ Grows the rust cache image to its declared size without killing a holder and without losing the warm cache.

Sequence (every step is resumable; nothing is ever killed, nothing is ever deleted):
  1. queue for the exclusive image lease through the repository's own queued leases API (queue-hold.ts): every holder finishes first, every later opener queues behind this ticket;
  2. apply the declaration patches (policy-patches.json: budget policy, schema, strict storage, ledger need and areas) right after the ticket stands, so a command that starts during the drain already carries the new size;
  3. with the lease granted: verify no file is open on the mount, `hdiutil detach` it, verify no image still claims it, move the directory aside (`fixed.retired-<stamp>`);
  4. release the lease; the next `bun repo` births the larger image (a trigger command is run when none does within --birth-wait seconds);
  5. mount the old image read-only on a scratch mountpoint and carry the directories the ledger declares for the rust area (`carry`: cargo home, unit objects, unit entries, admission memo) additively (no-clobber, temporary name then rename; objects before entries), then detach the old image.
Dry run is the default; --yes performs it. --carry-only resumes at step 5. A failure before the move rolls the patches back."""
import contextlib, json, os, platform, shutil, signal, subprocess, sys, tempfile, time

HERE = os.path.dirname(os.path.abspath(__file__))
TICKET = os.path.dirname(HERE)
sys.path.insert(0, os.path.join(TICKET, "cache-gc-tools"))
from paths import J, ROOT, TARGET  # noqa: E402

BUDGET = J(ROOT, TARGET["budget-policy"])
LEDGER = J(ROOT, TARGET["caching-index"].replace(os.path.basename(TARGET["caching-index"]), ""), "💾️storage", "🧾️ledger", "🔣️policy.json")
PATCHES = os.path.join(HERE, "policy-patches.json")
STATE = os.path.join(HERE, "grow-state.json")
GENERATED = os.path.join(TICKET, "🗑️generated", "rust-image")
HOLDER = os.path.join(HERE, "queue-hold.ts")
STOP = {"flag": False}


def log(*parts):
    print(time.strftime("%H:%M:%S"), *parts, flush=True)


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write_atomic(path, text):
    temporary = path + ".grow-tmp"
    with open(temporary, "w", encoding="utf-8") as handle:
        handle.write(text)
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def policy():
    return json.loads(read(BUDGET))["partitions"]["rust"]


def rust_dir():
    return J(ROOT, policy()["directory"])


def envelope(directory):
    try:
        value = json.loads(read(J(directory, "image.json"))).get("maximumPhysicalBytes")
        return value if isinstance(value, int) else None
    except (OSError, ValueError):
        return None


def mounted(mount):
    try:
        return os.stat(mount).st_dev != os.stat(os.path.dirname(mount)).st_dev
    except OSError:
        return False


def open_files(mount):
    if platform.system() == "Windows" or shutil.which("lsof") is None:
        return []
    result = subprocess.run(["lsof", "+f", "--", mount], capture_output=True, text=True)
    return [line[:160] for line in result.stdout.splitlines()[1:]] if result.returncode == 0 else []


def claims(directory):
    return directory in subprocess.run(["hdiutil", "info"], capture_output=True, text=True).stdout


def patches():
    return json.loads(read(PATCHES))


def row_state(text, row):
    return "applied" if row["new"] in text else "pending" if text.count(row["old"]) == 1 else "broken"


def patch_state():
    return {row_state(read(J(ROOT, row["path"])), row) for row in patches()}


def target_capacity():
    import re
    row = next(row for row in patches() if row["path"].endswith("budget/🔣️policy.json") and "capacityBytes" in row["new"])
    return int(re.search(r'"capacityBytes": (\d+)', row["new"]).group(1))


def apply_patches(direction="forward"):
    stamp = time.strftime("%Y%m%d-%H%M%S")
    backup = J(HERE, "backup-" + stamp)
    by_file = {}
    for row in patches():
        by_file.setdefault(row["path"], []).append(row)
    staged = {}
    for path, rows in by_file.items():
        text = read(J(ROOT, path))
        for row in rows:
            source, target = (row["old"], row["new"]) if direction == "forward" else (row["new"], row["old"])
            if direction == "forward" and row_state(text, row) == "applied":
                continue
            if text.count(source) != 1:
                raise RuntimeError("patch anchor not found exactly once in %s: %r" % (path, source[:70]))
            text = text.replace(source, target)
        staged[path] = text
    os.makedirs(backup, exist_ok=True)
    for path, text in staged.items():
        original = J(ROOT, path)
        shutil.copy2(original, J(backup, path.replace(os.sep, "_")[-120:]))
        write_atomic(original, text)
    log("patches %s: %d files (backup %s)" % (direction, len(staged), backup))


class Holder:
    """🎟️ The queued exclusive ticket, held by a bun child through the repository leases API."""

    def __init__(self, resource, limit):
        runtime = shutil.which("bun") or sys.executable
        self.process = subprocess.Popen([runtime, HOLDER, ROOT, resource, "--limit=%d" % limit], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, cwd=ROOT)
        self.lines = []

    def line(self):
        return self.process.stdout.readline().strip()

    def release(self):
        try:
            self.process.stdin.close()
        except OSError:
            pass
        try:
            self.process.wait(timeout=60)
        except subprocess.TimeoutExpired:
            self.process.terminate()


def live_cargo():
    out = subprocess.run(["ps", "-A", "-o", "pid=,etime=,command="], capture_output=True, text=True).stdout
    rows = []
    for line in out.splitlines():
        if "repo cargo" in line and "--store=" in line and "grow.py" not in line:
            pid, etime, command = (line.split(None, 2) + ["", ""])[:3]
            rows.append("%s %s %s" % (pid, etime, command[:100]))
    return rows


def swap(limit, birth_wait):
    directory = rust_dir()
    mount = J(directory, "m")
    resource = "native-fixed-image:" + directory
    have, want = envelope(directory), None
    log("rust image %s envelope %s" % (directory, have))
    holder = Holder(resource, limit)
    first = holder.line()
    if first != "QUEUED":
        raise RuntimeError("queue holder failed: " + first)
    log("ticket queued for", resource)
    patched = False
    try:
        apply_patches()
        patched = True
        want = policy()["capacityBytes"]
        while True:
            line = holder.line()
            if not line:
                raise RuntimeError("queue holder ended without the lease")
            if line.startswith("LOCKED"):
                log("exclusive lease granted:", line)
                break
            if line.startswith("FAILED"):
                raise RuntimeError(line)
            log("waiting for the holders of the image to finish;", len(live_cargo()), "repo cargo command(s) alive:", "; ".join(live_cargo()[:4]))
        files = []
        for attempt in range(12):
            files = open_files(mount) if mounted(mount) else []
            if not files:
                break
            log("files still open on the mount (%d): %s" % (len(files), files[0]))
            time.sleep(5)
        if files:
            raise RuntimeError("files stay open on %s: %s" % (mount, "; ".join(files[:3])))
        if mounted(mount):
            for attempt in range(4):
                result = subprocess.run(["hdiutil", "detach", mount], capture_output=True, text=True)
                if result.returncode == 0:
                    break
                log("detach failed:", result.stderr.strip()[:160])
                time.sleep(3)
            else:
                raise RuntimeError("hdiutil detach failed")
        if claims(directory):
            raise RuntimeError("an image still claims " + directory)
        aside = "%s.retired-%s" % (directory, time.strftime("%Y%m%d-%H%M%S"))
        os.rename(directory, aside)
        record = json.loads(read(STATE)) if os.path.exists(STATE) else {"asides": []}
        record["asides"].append({"path": aside, "envelope": have, "movedAt": time.strftime("%Y-%m-%dT%H:%M:%S")})
        write_atomic(STATE, json.dumps(record, indent=1))
        log("moved aside:", aside)
        patched = False
    except BaseException as error:
        if patched:
            log("failure before the move, rolling the patches back:", error)
            try:
                apply_patches("backward")
            except Exception as rollback:
                log("ROLLBACK FAILED, restore from backup-*:", rollback)
        raise
    finally:
        holder.release()
        log("lease released")
    wait_for_birth(directory, want, birth_wait)


def trigger_birth():
    log("no command births the image; triggering one cheap cargo command")
    runtime = shutil.which("bun") or sys.executable
    manifest = J(ROOT, "🧰️framework", "Cargo.toml")
    subprocess.Popen(["nice", "-n", "10", runtime, "repo", "cargo", "--store=shared", "--configuration=prototype", "--", "metadata", "--format-version=1", "--no-deps", "--offline", "--manifest-path", manifest], cwd=ROOT, stdout=open(J(GENERATED, "birth-trigger.log"), "w"), stderr=subprocess.STDOUT, start_new_session=True)


def wait_for_birth(directory, want, birth_wait):
    started, triggered = time.time(), False
    while time.time() - started < birth_wait * 6:
        have = envelope(directory)
        if have == want and mounted(J(directory, "m")):
            log("the larger image is born and mounted: envelope", have)
            return
        if have is not None and have != want and mounted(J(directory, "m")):
            raise RuntimeError("a stale command birthed an image of %s bytes, declared %s: rerun the swap" % (have, want))
        if not triggered and time.time() - started > birth_wait:
            triggered = True
            trigger_birth()
        time.sleep(3)
    raise RuntimeError("the larger image did not appear")


def carried():
    ledger = json.loads(read(LEDGER))
    area = next(row for row in ledger["areas"] if row["id"] == "rust")
    return area.get("carry", [])


def carry(aside, directory):
    scratch = tempfile.mkdtemp(prefix="rust-old-")
    old_mount = J(scratch, "m")
    os.mkdir(old_mount)
    result = subprocess.run(["hdiutil", "attach", "-readonly", "-nobrowse", "-noautoopen", "-mountpoint", old_mount, J(aside, "fixed.dmg")], capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError("attach of the old image failed: " + result.stderr.strip()[:200])
    log("old image mounted read-only at", old_mount)
    new_mount = J(directory, "m")
    stats = {"files": 0, "bytes": 0, "skipped": 0, "dirs": 0}
    last = [time.time()]

    def tick(path):
        if time.time() - last[0] > 15:
            last[0] = time.time()
            log("carry: %d files, %.2f GiB, %d existing skipped (%s)" % (stats["files"], stats["bytes"] / 2**30, stats["skipped"], path[-70:]))

    try:
        for relative in carried():
            source, target = J(old_mount, relative), J(new_mount, relative)
            if not os.path.isdir(source):
                log("carry: nothing at", relative)
                continue
            log("carry:", relative)
            for current, directories, names in os.walk(source):
                if STOP["flag"]:
                    log("carry: interrupted, the part carried so far stays")
                    return stats
                destination = J(target, os.path.relpath(current, source)) if current != source else target
                if not os.path.isdir(destination):
                    os.makedirs(destination, exist_ok=True)
                    stats["dirs"] += 1
                for name in names:
                    origin, copy = J(current, name), J(destination, name)
                    if os.path.lexists(copy):
                        stats["skipped"] += 1
                        continue
                    temporary = copy + ".carry-tmp"
                    try:
                        if os.path.islink(origin):
                            os.symlink(os.readlink(origin), temporary)
                        else:
                            shutil.copy2(origin, temporary)
                            stats["bytes"] += os.path.getsize(temporary)
                        os.replace(temporary, copy)
                        stats["files"] += 1
                    except OSError as error:
                        log("carry: skipped %s: %s" % (origin[-80:], error))
                        with contextlib.suppress(OSError):
                            os.remove(temporary)
                    tick(origin)
            log("carry: %s done" % relative)
        return stats
    finally:
        subprocess.run(["hdiutil", "detach", old_mount], capture_output=True, text=True)
        log("old image detached; still claimed:", claims(aside))
        with contextlib.suppress(OSError):
            os.rmdir(old_mount)
            os.rmdir(scratch)


def main(argv):
    yes = "--yes" in argv
    value = lambda name, default: int(next((a.split("=", 1)[1] for a in argv if a.startswith(name + "=")), default))
    limit, birth_wait = value("--limit", 5400), value("--birth-wait", 90)
    os.makedirs(GENERATED, exist_ok=True)
    signal.signal(signal.SIGINT, lambda *_: STOP.update(flag=True))
    signal.signal(signal.SIGTERM, lambda *_: STOP.update(flag=True))
    directory = rust_dir()
    free = shutil.disk_usage(ROOT).free
    want = target_capacity()
    log("host free %.1f GiB; patches %s; image envelope %s -> declared %s; mounted %s" % (free / 2**30, sorted(patch_state()), envelope(directory), want, mounted(J(directory, "m"))))
    if not yes:
        log("dry run: pass --yes to perform")
        return 0
    if envelope(directory) != want and "--carry-only" not in argv:
        if free < 60 * 2**30:
            log("refused: host free space below the 60 GiB the larger image and the old one need together")
            return 2
        swap(limit, birth_wait)
    record = json.loads(read(STATE)) if os.path.exists(STATE) else {"asides": []}
    sources = [row for row in record["asides"] if row.get("envelope") and row["envelope"] != want]
    if not sources:
        log("no aside image recorded: nothing to carry")
        return 0
    stats = carry(sources[0]["path"], directory)
    usage = shutil.disk_usage(J(directory, "m"))
    summary = {"aside": sources[0]["path"], "carry": stats, "newEnvelope": envelope(directory), "volumeBytes": usage.total, "freeBytes": usage.free}
    write_atomic(J(GENERATED, "grow-summary.json"), json.dumps(summary, indent=1))
    log("done", json.dumps(summary))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
