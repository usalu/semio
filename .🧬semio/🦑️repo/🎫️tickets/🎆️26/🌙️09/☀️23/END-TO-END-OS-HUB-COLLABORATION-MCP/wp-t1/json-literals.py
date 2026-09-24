"""🔁️ Prints a Rust source with every literal `json!(...)` macro call rewritten to `literal(r#"..."#)` for review."""
import sys
source = open(sys.argv[1], encoding="utf-8").read()
out, index = [], 0
while True:
    start = source.find("json!(", index)
    if start < 0: out.append(source[index:]); break
    out.append(source[index:start])
    depth, cursor = 0, start + len("json!")
    while True:
        char = source[cursor]
        if char == "(": depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0: break
        cursor += 1
    body = source[start + len("json!("):cursor]
    out.append(f'literal(r#"{body}"#)')
    index = cursor + 1
print("".join(out), end="")
