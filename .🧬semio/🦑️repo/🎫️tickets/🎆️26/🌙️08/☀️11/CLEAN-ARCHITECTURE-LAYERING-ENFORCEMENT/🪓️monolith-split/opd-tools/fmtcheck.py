"""rustfmt parse + hunk count of every .rs file the opd slice edited, before (scratchpad backup) versus after. Parse failures must be 0; hunks must not grow beyond the edited lines."""
import os, subprocess, sys, re
ROOT = "/Users/ueli/Documents/semio"
SP = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/opdpre/"
files = []
for dp, dn, fn in os.walk(SP):
    for f in fn:
        p = os.path.join(dp, f)
        rel = os.path.relpath(p, SP)
        if rel.endswith(".rs"):
            files.append(rel)
files.sort()
cfg = ["--edition", "2021", "--check", "--config", "skip_children=true"]
if os.path.exists(ROOT + "/rustfmt.toml"):
    cfg += ["--config-path", ROOT + "/rustfmt.toml"]


def run(path):
    p = subprocess.run(["rustfmt", "+nightly-2026-07-20", *cfg, path], capture_output=True, text=True)
    err = p.stderr
    parse = len(re.findall(r"^error", err, re.M))
    hunks = len(re.findall(r"^Diff in ", p.stdout + p.stderr, re.M))
    return parse, hunks


bad = 0
for rel in files:
    live = os.path.join(ROOT, rel)
    before = run(SP + rel)
    after = run(live) if os.path.exists(live) else ("deleted", "-")
    flag = "" if after[0] in (0, "deleted") else "  <-- PARSE FAILURE"
    if flag:
        bad += 1
    print(f"before parse={before[0]} hunks={before[1]} | after parse={after[0]} hunks={after[1]} | {rel[-95:]}{flag}")
print("files", len(files), "parse failures after:", bad)
