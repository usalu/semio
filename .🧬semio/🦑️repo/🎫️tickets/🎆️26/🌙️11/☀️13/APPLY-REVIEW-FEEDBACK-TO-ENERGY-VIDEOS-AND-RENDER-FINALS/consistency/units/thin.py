import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP_PARTS = {".venv", "site-packages", "__pycache__", ".ruff_cache", "🔊️speech", "🧪️tests"}
PAT = re.compile(r"(?P<pre>[0-9]|\)\}|[A-Za-z_]\}|\)\}\})\\;(?=\\frac\{\{?\\mathrm|\\mathrm\{\{?(?!in\b)|\^\{\\circ\}|\\text\{\{?(?:Kd)\b)")


def fix(line):
    out, pos, hits = [], 0, []
    for m in PAT.finditer(line):
        before = line[: m.start() + len(m.group("pre"))]
        if before.count("$") % 2 == 1:
            continue
        if re.search(r"in\}\}?$", before) or re.search(r"\\(?:mathrm|text)\{\{?[A-Za-z]+\}\}?$", before) and not re.search(r"\)\}$", before):
            continue
        out.append(line[pos : m.start() + len(m.group("pre"))] + "\\,")
        pos = m.end()
        hits.append(line[max(0, m.start() - 12) : m.end() + 24])
    out.append(line[pos:])
    return "".join(out), hits


def main():
    apply = "--apply" in sys.argv
    total = 0
    for p in ROOT.rglob("🐍️.py"):
        if set(p.parts) & SKIP_PARTS:
            continue
        text = p.read_bytes().decode("utf-8")
        lines = text.split("\n")
        n = 0
        for i, line in enumerate(lines):
            cr = line.endswith("\r")
            body = line[:-1] if cr else line
            new, hits = fix(body)
            if hits:
                lines[i] = new + ("\r" if cr else "")
                n += len(hits)
                for h in hits:
                    print(f"{p.relative_to(ROOT)}:{i + 1}: {h}")
        if n:
            total += n
            if apply:
                p.write_bytes("\n".join(lines).encode("utf-8"))
    print("total", total)


main()
