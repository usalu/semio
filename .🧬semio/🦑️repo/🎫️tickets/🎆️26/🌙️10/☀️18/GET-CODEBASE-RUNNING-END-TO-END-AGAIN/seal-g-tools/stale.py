#!/usr/bin/env python3
"""🏛️ Light commands run the last snapshot until a promotion (run INSIDE a mirror): edits the help catalog, runs `repo --help` sealed (stale expected), promotes, runs it again (fresh expected), restores."""
import os, subprocess, sys, time
CAT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"
MARK, TOKEN = "Tool-neutral build entry", "STALE%d" % os.getpid()
original = open(CAT, encoding="utf-8").read()
def help_line(mode):
    done = subprocess.run(["bun", "repo", "--help"], env=dict(os.environ, SEMIO_REPO_IMPLEMENTATION=mode, SEMIO_LANGUAGE="en"), capture_output=True, text=True)
    return TOKEN in done.stdout
try:
    open(CAT, "w", encoding="utf-8").write(original.replace(MARK, MARK + " " + TOKEN, 1))
    print("live   help shows the edit:", help_line("live"))
    print("sealed help shows the edit (before any promotion):", help_line("sealed"))
    started = time.perf_counter()
    done = subprocess.run(["bun", "repo", "--help"], env=dict(os.environ, SEMIO_REPO_IMPLEMENTATION="promote"), capture_output=True, text=True)
    print("promote exit %d in %.1f s: %s" % (done.returncode, time.perf_counter() - started, [l[:120] for l in done.stderr.splitlines() if "Sealed snapshot" in l][:1]))
    print("sealed help shows the edit (after promote):", help_line("sealed"))
finally:
    open(CAT, "w", encoding="utf-8").write(original)
