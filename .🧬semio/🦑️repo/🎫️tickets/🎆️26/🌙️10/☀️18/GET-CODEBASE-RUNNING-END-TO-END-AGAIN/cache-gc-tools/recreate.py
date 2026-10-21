#!/usr/bin/env python3
"""Recreates a derived cache partition image at its newly declared size: checks nobody uses it, detaches it, removes its directory. The next `bun repo` births the new image and rebuilds the derived data once. Dry run is the default; --yes performs it."""
import json, os, platform, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from paths import ROOT, J  # noqa: E402

DERIVED = {"graph": ".cache/b/g"}


def envelope(directory):
    try:
        with open(J(directory, "image.json"), encoding="utf-8") as handle:
            value = json.load(handle).get("maximumPhysicalBytes")
        return value if isinstance(value, int) else None
    except (OSError, ValueError):
        return None


def repo_processes():
    out = subprocess.run(["ps", "-A", "-o", "pid=,command="], capture_output=True, text=True).stdout
    mine = str(os.getpid())
    rows = []
    for line in out.splitlines():
        pid, _, command = line.strip().partition(" ")
        if pid == mine or "recreate.py" in command or "apply3.py" in command or "ps -A" in command:
            continue
        if "script.ts repo" in command or "script.ts\" repo" in command or ("📜️script.ts" in command and " repo " in command + " "):
            rows.append(line.strip()[:160])
    return rows


def open_files(mount):
    if platform.system() == "Windows" or shutil.which("lsof") is None:
        return []
    result = subprocess.run(["lsof", "+f", "--", mount], capture_output=True, text=True)
    return [line[:160] for line in result.stdout.splitlines()[1:]] if result.returncode == 0 else []


def mounted(mount):
    try:
        return os.stat(mount).st_dev != os.stat(os.path.dirname(mount)).st_dev
    except OSError:
        return False


def preflight(name):
    directory = J(ROOT, DERIVED[name])
    problems = []
    busy = repo_processes()
    if busy:
        problems.append("repo commands are running: " + "; ".join(busy[:3]))
    mount = J(directory, "m")
    if mounted(mount):
        files = open_files(mount)
        if files:
            problems.append("files are open on %s: %s" % (mount, "; ".join(files[:3])))
    return directory, mount, problems


def run(name, yes):
    directory, mount, problems = preflight(name)
    print("partition %s: %s (envelope %s)" % (name, directory, envelope(directory)))
    if not os.path.isdir(directory):
        print("nothing to recreate: the directory does not exist")
        return 0
    if problems:
        for problem in problems:
            print("refused:", problem)
        return 2
    print("plan: detach %s, verify no image still claims %s, remove %s; the next `bun repo` births the new image" % (mount, directory, directory))
    if not yes:
        print("dry-run: pass --yes to perform")
        return 0
    if mounted(mount):
        if platform.system() == "Darwin":
            result = subprocess.run(["hdiutil", "detach", mount], capture_output=True, text=True)
            if result.returncode != 0:
                print("refused: hdiutil detach failed:", result.stderr.strip()[:200])
                return 3
        else:
            print("refused: mounted but not an hdiutil image")
            return 3
    if platform.system() == "Darwin":
        info = subprocess.run(["hdiutil", "info"], capture_output=True, text=True).stdout
        if directory in info:
            print("refused: an image still claims", directory)
            return 3
    shutil.rmtree(directory)
    print("removed", directory)
    return 0


if __name__ == "__main__":
    arguments = [value for value in sys.argv[1:] if not value.startswith("--")]
    name = arguments[0] if arguments else "graph"
    if name not in DERIVED:
        raise SystemExit("derived partitions: " + ", ".join(DERIVED))
    raise SystemExit(run(name, "--yes" in sys.argv))
