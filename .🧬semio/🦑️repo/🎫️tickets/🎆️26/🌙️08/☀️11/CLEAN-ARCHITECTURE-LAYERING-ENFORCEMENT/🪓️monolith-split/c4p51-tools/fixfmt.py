"""🧹️ Normalises the blank lines the splitter leaves around region markers and after the header docstring (c4p51).

usage: fixfmt.py <file>...  (TypeScript; no marker inside a template string is expected)
"""
import re, sys
RE_START = re.compile(r"^\s*//\s*#region\b")
RE_END = re.compile(r"^\s*//\s*#endregion\b")
for path in sys.argv[1:]:
    lines = open(path, encoding="utf-8").read().split("\n")
    out = []
    for i, line in enumerate(lines):
        if RE_END.match(line):
            while out and out[-1].strip() == "":
                out.pop()
        if RE_START.match(line):
            if out and out[-1].strip() != "" and not RE_START.match(out[-1]):
                out.append("")
        if out and RE_END.match(out[-1]) and line.strip() != "" and not RE_END.match(line):
            out.append("")
        out.append(line)
    text = "\n".join(out)
    m = re.match(r"^(/\*\*.*?\*/)\n(?=\S)", text, re.S)
    if m and not text.startswith("/**\n *") or (m and m.group(1).count("\n") == 0):
        text = m.group(1) + "\n\n" + text[m.end():]
    elif m:
        text = m.group(1) + "\n\n" + text[m.end():]
    open(path, "w", encoding="utf-8").write(text)
