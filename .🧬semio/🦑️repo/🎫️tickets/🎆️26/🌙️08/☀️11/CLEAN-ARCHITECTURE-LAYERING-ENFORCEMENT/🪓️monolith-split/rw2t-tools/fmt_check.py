"""🎨 rw2t: rustfmt (style edition 2024, repo config) of the files this slice touched, on temp copies; reports the line ranges that differ. usage: fmt_check.py [--apply-new]"""
import os, re, subprocess, sys, tempfile, difflib
REPO = "/Users/ueli/Documents/semio"
SCR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rw2t/fmt"
os.makedirs(SCR, exist_ok=True)
files = [l.strip() for l in open(os.path.dirname(os.path.abspath(__file__)) + "/touched.txt", encoding="utf8") if l.strip().endswith(".rs")]
NEW = ("📷️wgpu-canvas2d-camera", "🧶️scene-ink-tests/🦀️.rs", "🪞️scene-render/🧪️tests/🦀️.rs", "🧷️engine-attach-tests/🦀️.rs")
for rel in files:
    p = REPO + "/" + rel
    if not os.path.exists(p):
        continue
    t = open(p, encoding="utf8").read()
    tmp = SCR + "/x.rs"
    open(tmp, "w", encoding="utf8").write(t)
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--style-edition", "2024", "--config-path", REPO + "/rustfmt.toml", "--config", "skip_children=true", tmp], capture_output=True, text=True)
    if r.returncode:
        print("ERR", rel[-60:], r.stderr[:160]); continue
    out = open(tmp, encoding="utf8").read()
    if out == t:
        print("clean   ", rel[-70:]); continue
    a, b = t.split("\n"), out.split("\n")
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    hunks = [(i1 + 1, i2) for tag, i1, i2, j1, j2 in sm.get_opcodes() if tag != "equal"]
    isnew = any(n in rel for n in NEW)
    print("DIFF    ", rel[-70:], "hunks", len(hunks), hunks[:6], "(new/owned)" if isnew else "")
    if isnew and "--apply-new" in sys.argv:
        open(p, "w", encoding="utf8").write(out)
        print("   formatted", rel[-50:])
