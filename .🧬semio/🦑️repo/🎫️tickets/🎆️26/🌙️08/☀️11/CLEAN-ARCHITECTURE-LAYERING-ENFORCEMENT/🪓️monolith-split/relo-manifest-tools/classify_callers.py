"""🔎️ Classifies every reference to a moved name in the Rust files given on stdin: qualified path prefix, `use` tree root, or bare."""
import re, sys, pathlib, json
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import moved_names as m

NAME_RE = re.compile(r"\b(" + "|".join(sorted(m.ALL, key=len, reverse=True)) + r")\b")

def strip_comments(text):
    out = []
    for line in text.split("\n"):
        stripped = line.lstrip()
        if stripped.startswith("//"):
            out.append("")
            continue
        idx = line.find("//")
        while idx >= 0:
            before = line[:idx]
            if before.count('"') % 2 == 0:
                line = before
                break
            idx = line.find("//", idx + 2)
        out.append(line)
    return "\n".join(out)

def use_statements(text):
    for match in re.finditer(r"\buse\s+([^;]+);", text, re.S):
        yield match.start(), match.end(), match.group(1)

def classify(path):
    raw = pathlib.Path(path).read_text()
    text = strip_comments(raw)
    uses = list(use_statements(text))
    hits = []
    for match in NAME_RE.finditer(text):
        name = match.group(1)
        pos = match.start()
        line_no = text.count("\n", 0, pos) + 1
        in_use = next((u for u in uses if u[0] <= pos < u[1]), None)
        if in_use:
            body = in_use[2]
            root = re.match(r"\s*(?:::)?([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)", body)
            local = body[: pos - in_use[0] - len("use ")]
            hits.append((line_no, name, "use", root.group(1) if root else "?", re.sub(r"\s+", " ", body)[:140]))
            continue
        prefix = re.search(r"((?:[A-Za-z_][A-Za-z0-9_]*::)+)$", text[max(0, pos - 200):pos])
        if prefix:
            hits.append((line_no, name, "path", prefix.group(1)[:-2], ""))
        else:
            hits.append((line_no, name, "bare", "", ""))
    return hits

files = [line.strip() for line in sys.stdin if line.strip()]
report = {f: classify(f) for f in files}
json.dump(report, open(sys.argv[1], "w"), ensure_ascii=False, indent=0)
for f, hits in report.items():
    kinds = {}
    for h in hits:
        kinds.setdefault((h[2], h[3]), set()).add(h[1])
    print(f)
    for (kind, root), names in sorted(kinds.items()):
        print(f"   {kind:4} {root[:70]:70} {','.join(sorted(names))[:200]}")
