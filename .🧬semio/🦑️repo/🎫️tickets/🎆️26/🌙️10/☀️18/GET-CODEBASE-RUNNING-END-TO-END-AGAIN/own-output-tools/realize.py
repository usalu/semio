#!/usr/bin/env python3
"""usage: realize.py MIRROR -- command...   Runs the command in MIRROR and, while it fails with a real-authority-ancestry/file refusal, turns the named symlink into a real directory (children symlinked) or a real file copy, then retries."""
import os, re, shutil, subprocess, sys

mirror = os.path.abspath(sys.argv[1])
command = sys.argv[sys.argv.index("--") + 1:]
pattern = re.compile(r"requires real authority (ancestry|file): (/[^\n]+)")
base = os.path.realpath(mirror)


def inside(path):
    return os.path.realpath(os.path.dirname(path)).startswith(base) or os.path.dirname(path).startswith(mirror)


for attempt in range(600):
    result = subprocess.run(command, cwd=mirror, capture_output=True, text=True)
    text = result.stdout + result.stderr
    match = pattern.search(text)
    if not match:
        sys.stdout.write(text)
        print("realize: attempts=%d exit=%d" % (attempt, result.returncode))
        sys.exit(result.returncode)
    kind, path = match.group(1), match.group(2).strip()
    if not path.startswith(mirror + os.sep):
        sys.stdout.write(text)
        raise SystemExit("realize: refusal outside the mirror: " + path)
    if not os.path.islink(path):
        sys.stdout.write(text)
        raise SystemExit("realize: not a symlink, cannot fix: " + path)
    target = os.path.realpath(path)
    os.unlink(path)
    if kind == "file":
        shutil.copyfile(target, path)
    else:
        os.makedirs(path)
        for entry in os.listdir(target):
            os.symlink(os.path.join(target, entry), os.path.join(path, entry))
raise SystemExit("realize: gave up")
