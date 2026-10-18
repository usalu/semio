"""🧱 OHF block reader: prints the non-stub rows of an `-it<N>-nonstub.txt` log as `code | crate | file:line | message | source line`, filtered by code. usage: blocks.py <file> <code>[,<code>] [--skip regex]"""
import re, sys
t = open(sys.argv[1], encoding="utf-8").read()
codes = sys.argv[2].split(",")
skip = re.compile(sys.argv[sys.argv.index("--skip") + 1]) if "--skip" in sys.argv else None
for blk in re.split(r"(?m)^#\d+ ", t)[1:]:
    head, _, body = blk.partition("\n")
    code = head.split(" ")[0]
    if code not in codes:
        continue
    m = re.search(r"-->\s+(\S+):(\d+)", body)
    f, ln = (m.group(1), m.group(2)) if m else ("", "")
    pm = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", f)
    pkg, rel = (pm.group(1), pm.group(2)) if pm else ("", f)
    msg = body.split("\n")[0][:150]
    src = ""
    for l in body.split("\n"):
        mm = re.match(r"^\s*\d+ \|\s?(.*)$", l)
        if mm:
            src = mm.group(1).strip()[:120]
            break
    line = f"{code} | {pkg.replace('semio_framework_', '')[:40]} | {rel.split('/')[-2] if '/' in rel else rel}:{ln} | {msg} | {src}"
    if skip and skip.search(line):
        continue
    print(line)
