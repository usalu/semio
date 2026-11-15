import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP_PARTS = {".venv", "site-packages", "__pycache__", ".ruff_cache", "🔊️speech", "🧪️tests"}
SKIP_UNITS = {"t20/15"}
IN_BEFORE = re.compile(r"(?:\\text\{\{?in\}?\}|\\mathrm\{\{?in\}?\})(?:\\;|\\,|\s)*$")


def balanced(s, start, doubled):
    i, depth = start, 0
    step = 2 if doubled else 1
    while i < len(s):
        tok = s[i : i + step]
        if tok == "{" * step:
            depth += 1
            i += step
        elif tok == "}" * step:
            depth -= 1
            i += step
            if depth == 0:
                return i
        else:
            i += 1
    return -1


def split_unit(inner):
    depth = 0
    for i, ch in enumerate(inner):
        if ch in "({":
            depth += 1
        elif ch in ")}":
            depth -= 1
        elif ch == "/" and depth == 0:
            return inner[:i], inner[i + 1 :]
    return None


def strip_parens(s):
    if s.startswith("(") and s.endswith(")"):
        depth = 0
        for i, ch in enumerate(s):
            depth += ch == "("
            depth -= ch == ")"
            if depth == 0 and i < len(s) - 1:
                return s
        return s[1:-1]
    return s


def convert_line(line):
    out, pos, changed = [], 0, []
    for m in re.finditer(r"\\mathrm\{", line):
        if m.start() < pos:
            continue
        doubled = line[m.end() : m.end() + 1] == "{"
        brace0 = m.end() - 1
        end = balanced(line, brace0, doubled)
        if end < 0:
            continue
        step = 2 if doubled else 1
        inner = line[brace0 + step : end - step]
        norm = inner.replace("{{", "{").replace("}}", "}") if doubled else inner
        parts = split_unit(norm)
        if not parts or norm in SKIP_UNITS or not parts[0].strip() or not parts[1].strip():
            continue
        if line[: m.start()].count("$") % 2 == 1 or IN_BEFORE.search(line[: m.start()]):
            continue
        num, den = parts[0], strip_parens(parts[1])
        if doubled:
            num, den = num.replace("{", "{{").replace("}", "}}"), den.replace("{", "{{").replace("}", "}}")
            new = "\\frac{{\\mathrm{{" + num + "}}}}{{\\mathrm{{" + den + "}}}}"
        else:
            new = "\\frac{\\mathrm{" + num + "}}{\\mathrm{" + den + "}}"
        out.append(line[pos : m.start()] + new)
        pos = end
        changed.append((line[m.start() : end], new))
    out.append(line[pos:])
    return "".join(out), changed


def main():
    apply = "--apply" in sys.argv
    paths = [Path(a) for a in sys.argv[1:] if not a.startswith("--")]
    total = 0
    for p in paths or [q for q in ROOT.rglob("🐍️.py") if not set(q.parts) & SKIP_PARTS]:
        if set(p.parts) & SKIP_PARTS:
            continue
        raw = p.read_bytes()
        text = raw.decode("utf-8")
        crlf = "\r\n" in text
        lines = text.split("\n")
        edits = []
        for i, line in enumerate(lines):
            cr = line.endswith("\r")
            body = line[:-1] if cr else line
            new, ch = convert_line(body)
            if ch:
                lines[i] = new + ("\r" if cr else "")
                edits.append((i + 1, ch))
        if edits:
            total += sum(len(c) for _, c in edits)
            print(f"{p.relative_to(ROOT)}: {sum(len(c) for _, c in edits)}")
            for n, ch in edits:
                for a, b in ch:
                    print(f"  {n}: {a}  ->  {b}")
            if apply:
                p.write_bytes("\n".join(lines).encode("utf-8"))
    print("total", total)


main()
