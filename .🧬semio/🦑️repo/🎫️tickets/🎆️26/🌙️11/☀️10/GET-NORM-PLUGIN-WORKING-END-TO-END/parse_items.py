"""🧩️ Splits a Rust file into top-level items (leading doc comments, attributes and region markers stay with the item that follows)."""
import re
def split_items(text):
    lines = text.split("\n")
    items, cur, depth, started = [], [], 0, False
    def code_of(l):
        l = re.sub(r"//.*", "", l)
        l = re.sub(r'"(?:\\.|[^"\\])*"', '""', l)
        return re.sub(r"'(?:\\.|[^'\\])'", "''", l)
    for l in lines:
        cur.append(l)
        c = code_of(l)
        depth += c.count("{") + c.count("(") + c.count("[") - c.count("}") - c.count(")") - c.count("]")
        s = c.strip()
        if depth == 0 and s and not s.startswith(("#[", "//", "#!")) and (s.endswith("}") or s.endswith(";") or s.endswith(")") ):
            items.append("\n".join(cur)); cur = []
    if any(x.strip() for x in cur): items.append("\n".join(cur))
    return items
def head(item):
    for l in item.split("\n"):
        s = l.strip()
        if s and not s.startswith(("#[", "//", "#!")):
            return s
    return ""
