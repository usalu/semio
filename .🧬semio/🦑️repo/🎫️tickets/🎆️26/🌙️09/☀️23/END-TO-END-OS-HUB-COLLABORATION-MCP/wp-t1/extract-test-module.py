"""🧪️ Moves one inline `#[cfg(test)] mod <name> { … }` out of a Rust source into a canonical test implementation and wires it by `#[path]`.

usage: extract-test-module.py <source.rs> <mod-line> <target.rs>   (target is created; never overwritten)
"""
import os, re, sys

def mask(source):
    out, i, n = list(source), 0, len(source)
    def blank(j):
        if out[j] not in "\n\r": out[j] = " "
    while i < n:
        if source.startswith("//", i):
            while i < n and source[i] != "\n": blank(i); i += 1
            continue
        if source.startswith("/*", i):
            depth = 0
            while i < n:
                if source.startswith("/*", i): depth += 1; blank(i); blank(i + 1); i += 2; continue
                if source.startswith("*/", i): depth -= 1; blank(i); blank(i + 1); i += 2
                else: blank(i); i += 1
                if depth == 0: break
            continue
        raw = re.match(r'(?:br|rb|r)(#*)"', source[i:i + 12])
        if raw and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == "_")):
            close = '"' + raw.group(1); j = source.find(close, i + len(raw.group(0)))
            for k in range(i + len(raw.group(0)), j): blank(k)
            i = j + len(close); continue
        if source[i] == '"':
            i += 1
            while i < n and source[i] != '"':
                if source[i] == "\\": blank(i); i += 1
                blank(i); i += 1
            i += 1; continue
        if source[i] == "'" and re.match(r"'(?:\\(?:x[0-9A-Fa-f]{2}|u\{[0-9A-Fa-f_]+\}|.)|[^'\\\r\n])'", source[i:i + 12]):
            m = re.match(r"'(?:\\(?:x[0-9A-Fa-f]{2}|u\{[0-9A-Fa-f_]+\}|.)|[^'\\\r\n])'", source[i:i + 12])
            for k in range(i + 1, i + len(m.group(0)) - 1): blank(k)
            i += len(m.group(0)); continue
        i += 1
    return "".join(out)

source_path, mod_line, target_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
assert not os.path.exists(target_path), f"{target_path} exists"
source = open(source_path, encoding="utf-8").read()
lines = source.split("\n")
head_pattern = re.compile(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+))\s*\{\s*$")
mod_line = next(number for number in range(mod_line, mod_line + 6) if head_pattern.match(lines[number - 1]))
head = lines[mod_line - 1]
m = head_pattern.match(head)
assert m, f"line {mod_line} is not an inline module head: {head!r}"
indent, decl = m.group(1), m.group(2)
start = sum(len(l) + 1 for l in lines[:mod_line - 1]) + head.index("{")
masked = mask(source)
depth, i = 0, start
while True:
    if masked[i] == "{": depth += 1
    elif masked[i] == "}":
        depth -= 1
        if depth == 0: break
    i += 1
body = source[start + 1:i]
end_line_text_after = source[i + 1:source.find("\n", i)] if source.find("\n", i) >= 0 else source[i + 1:]
assert end_line_text_after.strip() == "", f"trailing text after module close: {end_line_text_after!r}"
body_lines = body.strip("\n").split("\n")
step = indent + "    "
dedented = [l[len(step):] if l.startswith(step) else (l.lstrip() if l.strip() == "" else l) for l in body_lines]
for l in body_lines:
    assert l.strip() == "" or l.startswith(step), f"under-indented body line: {l!r}"
text = "\n".join(dedented).rstrip() + "\n"
for hit in re.finditer(r"include(?:_str|_bytes)?!\s*\(\s*\"([^\"]+)\"|#\s*\[\s*path\s*=\s*\"([^\"]+)\"", text):
    print(f"[warn] relative reference in moved body: {hit.group(0)}", file=sys.stderr)
os.makedirs(os.path.dirname(target_path), exist_ok=True)
open(target_path, "w", encoding="utf-8").write(text)
relative = os.path.relpath(target_path, os.path.dirname(source_path))
line_end = source.find("\n", i)
new = source[:sum(len(l) + 1 for l in lines[:mod_line - 1])] + f'{indent}#[path = "{relative}"]\n{indent}{decl};' + (source[line_end:] if line_end >= 0 else "")
open(source_path, "w", encoding="utf-8").write(new)
print(f"moved {len(body_lines)} line(s) of {decl} → {relative}")
