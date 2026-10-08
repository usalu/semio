"""🔎 Prints launch rows of both launch files whose command matches a regular expression (read-only audit helper of slice L-S5)."""
import json, re, sys

def strip_jsonc(text):
    out, i, n, in_string = [], 0, len(text), False
    while i < n:
        c = text[i]
        if in_string:
            out.append(c)
            if c == "\\":
                out.append(text[i + 1]); i += 2; continue
            if c == '"': in_string = False
            i += 1; continue
        if c == '"': in_string = True; out.append(c); i += 1; continue
        if text.startswith("//", i):
            while i < n and text[i] != "\n": i += 1
            continue
        if text.startswith("/*", i):
            i = text.index("*/", i) + 2; continue
        out.append(c); i += 1
    return re.sub(r",(\s*[}\]])", r"\1", "".join(out))

pattern = re.compile(sys.argv[1])
for path in (".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"):
    document = json.loads(strip_jsonc(open(path, encoding="utf8").read()))
    rows = [row for row in document["configurations"] if isinstance(row, dict) and pattern.search(row.get("command", "") + " " + row.get("name", ""))]
    print(f"== {path}: {len(rows)} rows")
    for row in rows:
        print(json.dumps({key: row[key] for key in ("name", "command", "cwd", "env", "presentation", "serverReadyAction") if key in row}, ensure_ascii=False))
