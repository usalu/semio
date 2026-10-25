#!/usr/bin/env python3
"""📏️ Recreates the lease image at its declared size without dropping a held lease: queues for the exclusive image lease in the control directory (FIFO: every holder finishes first, every later opener queues behind the ticket, nothing is killed), then detaches the image, moves its directory aside and releases; the next `bun repo` births the image at the declared size. The area is `derived` in the ledger: with the exclusive lease granted no holder exists and an idle lease entry is recreated on demand, so there is nothing to carry. Dry run by default (--yes performs it); the declared size must be changed in the budget policy BEFORE, otherwise nothing is done. Usage: recreate-leases.py [--yes] [--limit=<seconds>] [--birth-wait=<seconds>] [--directory=<image dir> --control=<control lease dir> --want=<bytes>] (the last three rehearse the swap on a scratch image)"""
import json, os, shutil, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
POLICY = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json")
HOLDER = os.path.join(HERE, "hold-control.ts")


def log(*parts):
    print(time.strftime("%H:%M:%S"), *parts, flush=True)


def declared():
    with open(POLICY, encoding="utf-8") as handle:
        row = json.load(handle)["partitions"]["leases"]
    return os.path.join(ROOT, row["directory"]), row["capacityBytes"]


def envelope(directory):
    try:
        with open(os.path.join(directory, "image.json"), encoding="utf-8") as handle:
            value = json.load(handle).get("maximumPhysicalBytes")
        return value if isinstance(value, int) else None
    except (OSError, ValueError):
        return None


def mounted(mount):
    try:
        return os.stat(mount).st_dev != os.stat(os.path.dirname(mount)).st_dev
    except OSError:
        return False


def main(argv):
    value = lambda name, default: int(next((a.split("=", 1)[1] for a in argv if a.startswith(name + "=")), default))
    limit, birth_wait, yes = value("--limit", 5400), value("--birth-wait", 90), "--yes" in argv
    option = lambda name: next((a.split("=", 1)[1] for a in argv if a.startswith(name + "=")), None)
    directory, want = declared()
    if option("--directory"):
        directory, want = option("--directory"), int(option("--want"))
    control = option("--control")
    have = envelope(directory)
    log("lease image %s envelope %s declared %s mounted %s" % (directory, have, want, mounted(os.path.join(directory, "m"))))
    if have == want:
        log("the image has its declared size: nothing to do")
        return 0
    if not yes:
        log("dry run: with --yes this queues for the exclusive lease, detaches, moves the directory aside and waits for the next command to birth the image")
        return 0
    runtime = shutil.which("bun") or sys.executable
    holder = subprocess.Popen([runtime, HOLDER, ROOT, directory, "--limit=%d" % limit] + (["--control=" + control] if control else []), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, cwd=ROOT)
    try:
        if holder.stdout.readline().strip() != "QUEUED":
            raise RuntimeError("queue holder failed")
        log("ticket queued; every holder finishes first")
        while True:
            line = holder.stdout.readline().strip()
            if not line or line.startswith("FAILED"):
                raise RuntimeError(line or "queue holder ended without the lease")
            if line.startswith("LOCKED"):
                log("exclusive lease granted:", line)
                break
        mount = os.path.join(directory, "m")
        if mounted(mount):
            for attempt in range(4):
                if subprocess.run(["hdiutil", "detach", mount], capture_output=True, text=True).returncode == 0:
                    break
                time.sleep(3)
            else:
                raise RuntimeError("hdiutil detach failed")
        if directory in subprocess.run(["hdiutil", "info"], capture_output=True, text=True).stdout:
            raise RuntimeError("an image still claims " + directory)
        aside = "%s.retired-%s" % (directory, time.strftime("%Y%m%d-%H%M%S"))
        os.rename(directory, aside)
        log("moved aside (not deleted):", aside)
    finally:
        try:
            holder.stdin.close()
        except OSError:
            pass
        holder.wait(timeout=60)
        log("lease released")
    started = time.time()
    while time.time() - started < birth_wait * 6:
        if envelope(directory) == want and mounted(os.path.join(directory, "m")):
            log("the image is born at the declared size", want)
            return 0
        time.sleep(3)
    raise RuntimeError("the image did not appear: run any `bun repo` command")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
