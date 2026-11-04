import os
import re

root = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990"
struct_re = re.compile(r"pub struct (\w+)\s*(?:<[^>]+>)?\s*\{")
field_re = re.compile(r"^\s*pub(?:\([^)]+\))? (\w+)\s*:", re.M)
literal_re = re.compile(r"(?<![\w:$])([A-Z]\w*)\s*\{")

def close(text, start):
    depth = 0
    for i in range(start, len(text)):
        char = text[i]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return i
    return -1

structs = {}
files = []
for dirpath, _, names in os.walk(root):
    if "node_modules" in dirpath or "/🗑️" in dirpath:
        continue
    for name in names:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8", errors="replace").read()
        files.append((path, text))
        for match in struct_re.finditer(text):
            end = close(text, match.end() - 1)
            if end < 0:
                continue
            body = text[match.end():end]
            fields = set(field_re.findall(body))
            structs.setdefault(match.group(1), set()).update(fields)

hits = []
for path, text in files:
    for match in literal_re.finditer(text):
        name = match.group(1)
        if name not in structs:
            continue
        pre = text[max(0, match.start() - 16):match.start()]
        if re.search(r"(struct|enum|impl|trait|fn)\s*$", pre):
            continue
        end = close(text, match.end() - 1)
        if end < 0:
            continue
        body = text[match.end():end]
        if ".." in body or "$" in body:
            continue
        used = set(re.findall(r"(?:^|,)\s*(\w+)\s*:", body))
        extra = sorted(used - structs[name] - {"r#type"})
        if extra:
            line = text.count("\n", 0, match.start()) + 1
            hits.append((path, line, name, extra))

print(f"structs {len(structs)} hits {len(hits)}")
for path, line, name, extra in hits[:40]:
    print(f"{line} {name} {extra} {path[len(root)+1:]}")
