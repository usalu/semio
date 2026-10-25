"""One traced Bazel run: python3 trace-run.py <tag> <target> [extra repo args...]
Output under C/🗑️generated/ckey3-d/<tag>/: run.log, ev/ (Bazel execution log), trace/ (one .trace file per executor process), rc.
Every executor process of the run writes its source trace (SEMIO_BUILD_SOURCE_TRACE); one process per action (--workers=off, the default here; TRACE_WORKERS=on traces the persistent worker mode)."""
import os, subprocess, sys, time
tag, target, extra = sys.argv[1], sys.argv[2], sys.argv[3:]
root = "/Users/ueli/Documents/semio"
out = os.path.join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/ckey3-d", tag)
os.makedirs(os.path.join(out, "ev"), exist_ok=True); os.makedirs(os.path.join(out, "trace"), exist_ok=True)
env = {k: v for k, v in os.environ.items() if k != "SEMIO_SCRIPT_STALL_MS"}
env["SEMIO_BUILD_EVIDENCE_DIR"] = os.path.join(out, "ev")
if os.environ.get("TRACE") == "1": env["SEMIO_BUILD_SOURCE_TRACE"] = os.path.join(out, "trace")
env.setdefault("SEMIO_BUILD_GRAPH_POLICY", "current")
started = time.time()
with open(os.path.join(out, "run.log"), "wb") as log:
    code = subprocess.call(["nice", "-n", "10", "bun", "repo", "--tool=bazel", *([] if os.environ.get("TRACE_WORKERS") == "default" else ["--workers=" + os.environ.get("TRACE_WORKERS", "off")]), os.environ.get("TRACE_VERB", "run"), *([] if target == "-" else [target]), *extra], cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
open(os.path.join(out, "rc"), "w").write(f"{code} {round(time.time() - started)}s\n")
