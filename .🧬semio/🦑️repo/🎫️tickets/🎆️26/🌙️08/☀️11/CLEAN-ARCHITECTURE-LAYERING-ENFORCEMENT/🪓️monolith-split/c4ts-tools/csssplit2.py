"""🎨️ Splits a stylesheet into contiguous region-aware section children under a styles folder; the root keeps its leading lines and the ordered @import list.

usage: csssplit2.py <spec.json> [--apply]
spec: {"snapshot": path, "root": path, "base": dir, "keepRootLines": n, "children": [[start, end, folder, doc], ...]}
Region markers `/* #region X */` / `/* #endregion X */` are repeated in every child a region crosses; the ordered non-blank line sequence (minus the repeats) must equal the original.
"""
import os, sys, json, re
spec = json.load(open(sys.argv[1], encoding="utf-8"))
text = open(spec["snapshot"], encoding="utf-8").read()
lines = text.split("\n")
assert lines[-1] == ""
lines = lines[:-1]
keep = spec["keepRootLines"]
rs = re.compile(r"^\s*/\*\s*#region\s+(.*?)\s*\*/\s*$")
re_ = re.compile(r"^\s*/\*\s*#endregion\s+(.*?)\s*\*/\s*$")
stack_at = []
stack = []
marks = {}
for i, l in enumerate(lines):
    m = rs.match(l)
    if m:
        stack.append(m.group(1)); marks[m.group(1)] = (l.strip(), None)
    m = re_.match(l)
    if m:
        assert stack and stack[-1] == m.group(1), "unbalanced region at %d" % (i + 1)
        stack.pop(); marks[m.group(1)] = (marks[m.group(1)][0], l.strip())
    stack_at.append(list(stack))
assert not stack
children = spec["children"]
assert children[0][0] == keep + 1 or lines[keep:children[0][0]-1] == [] or all(not x.strip() for x in lines[keep:children[0][0]-1]), "gap before first child"
out = {}
extras = {}
for idx, (start, end, folder, doc) in enumerate(children):
    if idx + 1 < len(children):
        assert children[idx + 1][0] == end + 1, "children not contiguous at %s" % folder
    body = lines[start - 1:end]
    while body and body[-1].strip() == "":
        body.pop()
    while body and body[0].strip() == "":
        body.pop(0)
    before = stack_at[start - 2] if start >= 2 else []
    reopen = [marks[r][0] for r in before]
    after = stack_at[end - 1]
    if body and re_.match(lines[end - 1] if lines[end - 1].strip() else ""):
        pass
    close = [marks[r][1] for r in reversed(after)]
    emoji = folder[: folder.index("️") + 1]
    parts = ["/* %s %s */" % (emoji, doc), ""]
    parts += reopen
    parts += body
    parts += close
    out[folder] = "\n".join(parts) + "\n"
    extras[folder] = len(reopen) + len(close)
assert children[-1][1] == len(lines), "last child must end at EOF (%d vs %d)" % (children[-1][1], len(lines))
rootlines = lines[:keep]
imports = "\n".join('@import "./%s/%s/🎨️.css";' % (os.path.basename(spec["base"]), c[2]) for c in children)
root = "\n".join(rootlines).rstrip("\n") + "\n\n" + imports + "\n"
seq = []
for folder, t in out.items():
    seq += t.split("\n")[2:]
mk = lambda l: bool(rs.match(l) or re_.match(l))
a = [l for l in seq if l.strip()]
b = [l for l in lines[keep:] if l.strip()]
a_nomark = [l for l in a if not mk(l)]
b_nomark = [l for l in b if not mk(l)]
assert a_nomark == b_nomark, "non-blank non-marker line sequence differs"
for folder, t in out.items():
    print("%-44s %4d lines (+%d markers)" % (folder, t.count("\n"), extras[folder]))
print("root", root.count("\n"))
if "--apply" in sys.argv:
    assert open(spec["root"], encoding="utf-8").read() == text, "root changed since the snapshot"
    for folder, t in out.items():
        target = spec["base"] + "/" + folder + "/🎨️.css"
        assert not os.path.exists(target), target
    for folder, t in out.items():
        target = spec["base"] + "/" + folder + "/🎨️.css"
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(t)
    open(spec["root"], "w", encoding="utf-8").write(root)
    print("APPLIED")
