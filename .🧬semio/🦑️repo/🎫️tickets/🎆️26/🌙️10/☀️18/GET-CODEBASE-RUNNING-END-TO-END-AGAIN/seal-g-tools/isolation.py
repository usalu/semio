#!/usr/bin/env python3
"""Seal isolation demo (run INSIDE a mirror: mm run <n> -- python3 -I isolation.py <PB> live|sealed [early]).
Starts the door for `repo --help`, freezes the whole process group when the sealed loader child (live: immediately) exists, edits the help catalog,
thaws, and prints whether the running implementation shows the edit. Streams; no big buffers."""
import os, signal, subprocess, sys, time
PB, MODE = sys.argv[1], sys.argv[2]
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
DOOR = B + "/🚪️door/📜️script.ts"
CAT = B + "/⚙️operations/🔣️catalog.json"
MARK = "Tool-neutral build entry"
TOKEN = "EDIT%d" % os.getpid()
env = dict(os.environ, SEMIO_REPO_IMPLEMENTATION=MODE, SEMIO_LANGUAGE="en")


def tree(root):
    out = subprocess.run(["ps", "-A", "-o", "pid=,ppid=,command="], capture_output=True, text=True).stdout.splitlines()
    kids, cmd = {}, {}
    for row in out:
        parts = row.split(None, 2)
        if len(parts) < 3:
            continue
        p, q = int(parts[0]), int(parts[1])
        kids.setdefault(q, []).append(p)
        cmd[p] = parts[2]
    seen, todo = [], [root]
    while todo:
        x = todo.pop()
        for k in kids.get(x, []):
            seen.append((k, cmd[k]))
            todo.append(k)
    return seen


text = open(CAT, encoding="utf-8").read()
assert text.count(MARK) >= 1
p = subprocess.Popen([PB, DOOR, "repo", "--help"], env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True)
frozen = None
if MODE != "sealed" or "early" in sys.argv[3:]:
    os.killpg(p.pid, signal.SIGSTOP)
    frozen = ["immediately after spawn"]
deadline = time.time() + 20
while frozen is None and time.time() < deadline and p.poll() is None:
    kids = tree(p.pid)
    hit = [c for _, c in kids if "--eval" in c] if MODE == "sealed" else [1]
    if hit:
        os.killpg(p.pid, signal.SIGSTOP)
        frozen = [c[:90] for _, c in kids]
        break
    time.sleep(0.005)
print("frozen:", frozen is not None, "ended-before-freeze:", p.poll() is not None, "children:", frozen)
open(CAT, "w", encoding="utf-8").write(text.replace(MARK, MARK + " " + TOKEN, 1))
time.sleep(0.5)
if frozen is not None:
    os.killpg(p.pid, signal.SIGCONT)
out = p.communicate(timeout=300)[0]
open(CAT, "w", encoding="utf-8").write(text)
print(out[:1200]) if p.returncode else None
first = [line for line in out.splitlines() if MARK in line][:1]
print("mode", MODE, "exit", p.returncode, "first help line:", first)
print("RESULT", MODE, "running implementation shows the edit:", any(TOKEN in line for line in first))
