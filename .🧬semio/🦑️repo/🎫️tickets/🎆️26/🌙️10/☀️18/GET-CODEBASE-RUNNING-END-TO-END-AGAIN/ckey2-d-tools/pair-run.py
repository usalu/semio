"""Back-to-back pair: python3 pair-run.py <tag> <target> [extra repo args...] -> <tag>A, <tag>B (trace-run.py each), then ♨️warm verdicts and the changed inputs per action.
Env passes through (SEMIO_BUILD_IMPLEMENTATION_SCOPES for a candidate table)."""
import os, subprocess, sys
tag, target, extra = sys.argv[1], sys.argv[2], sys.argv[3:]
here = os.path.dirname(os.path.abspath(__file__))
root = "/Users/ueli/Documents/semio"
gen = os.path.join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/ckey2-d")
for suffix in ("A", "B"):
    subprocess.call([sys.executable, os.path.join(here, "trace-run.py"), tag + suffix, target, *extra])
    print(tag + suffix, open(os.path.join(gen, tag + suffix, "rc")).read().strip(), flush=True)
a, b = (os.path.join(gen, tag + s, "ev", "bazel.execlog.jsonl") for s in ("A", "B"))
with open(os.path.join(gen, tag + "-verdicts.txt"), "w") as out:
    if os.path.exists(a) and os.path.exists(b):
        for script in ("verdicts.ts", "diff-execlogs.py"):
            tool = os.path.join(os.path.dirname(here), "cwarm-d-tools", script)
            cmd = ["bun", tool, a, b] if script.endswith(".ts") else [sys.executable, "-I", tool, a, b]
            out.write("## " + script + "\n"); out.flush()
            subprocess.call(cmd, cwd=root, stdout=out, stderr=subprocess.STDOUT)
