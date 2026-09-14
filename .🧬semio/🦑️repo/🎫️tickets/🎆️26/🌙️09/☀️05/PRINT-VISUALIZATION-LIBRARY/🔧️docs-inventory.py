#!/usr/bin/env python3
"""🔍️ DOCS: dumps public \\SemioViz commands, environments and %region 🔖️Keys blocks per package."""
import os, re, sys, json

LATEX = r"C:\git\semio\🧰️framework\🛍️products\📓️print\🖋️latex"
MODE = sys.argv[1] if len(sys.argv) > 1 else "commands"
ONLY = sys.argv[2] if len(sys.argv) > 2 else None

cmd_re = re.compile(r"\\New(?:Expandable)?DocumentCommand\s*(\\SemioViz[A-Za-z]*)\s*\{([^}]*)\}")
env_re = re.compile(r"\\NewDocumentEnvironment\s*\{\s*(Viz[A-Za-z]*)\s*\}\s*\{([^}]*)\}")
key_re = re.compile(r"^\s*([A-Za-z0-9@_-]+)\s*\.(\w+)[^,]*(,|$)")

out = []
for name in sorted(os.listdir(LATEX)):
    if not name.startswith("semio-viz") or not name.endswith(".sty"):
        continue
    if ONLY and ONLY not in name:
        continue
    lines = open(os.path.join(LATEX, name), encoding="utf8").read().split("\n")
    if MODE == "commands":
        hits = []
        for i, line in enumerate(lines):
            m = cmd_re.search(line) or env_re.search(line)
            if not m:
                continue
            note = ""
            for j in range(i - 1, max(-1, i - 4), -1):
                if lines[j].lstrip().startswith("%") and not lines[j].lstrip().startswith("%region") and not lines[j].lstrip().startswith("%endregion"):
                    note = lines[j].strip().lstrip("%").strip()
                    break
            hits.append(f"  {m.group(1)} {{{m.group(2)}}}  -- {note}")
        if hits:
            out.append(name)
            out.extend(hits)
    else:
        inside = None
        for i, line in enumerate(lines):
            s = line.strip()
            if s.startswith("%region") and "Keys" in s:
                inside = s
                out.append(f"### {name} :: {s}")
                continue
            if s.startswith("%endregion") and inside:
                inside = None
                out.append("")
                continue
            if inside:
                out.append(line.rstrip())
print("\n".join(out))
