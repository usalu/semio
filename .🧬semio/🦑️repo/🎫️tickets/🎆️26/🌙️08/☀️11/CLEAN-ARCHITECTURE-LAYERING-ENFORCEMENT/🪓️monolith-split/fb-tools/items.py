"""🧩 Top-level Rust item splitter for the FB carve: splits a source file into items (leading docs, attributes and plain comments attached) and region markers, keeping exact text."""
import re, sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "pos"))
from posstub import mask

REGION = re.compile(r"^\s*//\s*#(end)?region\b")
NAME = re.compile(r"\b(?:fn|struct|enum|const|static|type|trait|mod|union)\s+([A-Za-z_][A-Za-z0-9_]*)")


def split_items(text):
    """[(kind, name, text)] in source order; kind in {'item','marker','gap'}; concatenating every text reproduces the input exactly."""
    lines = text.split("\n")
    masked = mask(text).split("\n")
    out, pending, cur, depth = [], [], None, 0
    i = 0
    while i < len(lines):
        raw, m = lines[i], masked[i]
        stripped = m.strip()
        if cur is None:
            if not stripped or stripped.startswith("#[") and not stripped.endswith("]") is False and False:
                pass
            if REGION.match(raw) and depth == 0:
                if pending:
                    out.append(("gap", "", "\n".join(pending) + "\n"))
                    pending = []
                out.append(("marker", raw.strip(), raw + "\n"))
                i += 1
                continue
            if not stripped:
                pending.append(raw)
                i += 1
                continue
            is_attr = raw.lstrip().startswith("#[") or raw.lstrip().startswith("#![")
            is_comment = raw.lstrip().startswith("//") or not stripped
            if is_attr or is_comment:
                pending.append(raw)
                if is_attr:
                    d = m.count("[") - m.count("]")
                    j = i
                    while d > 0:
                        j += 1
                        pending.append(lines[j])
                        d += masked[j].count("[") - masked[j].count("]")
                    i = j
                i += 1
                continue
            cur = list(pending) + [raw]
            pending = []
            depth = m.count("{") + m.count("(") + m.count("[") - m.count("}") - m.count(")") - m.count("]")
            if depth <= 0 and (stripped.endswith(";") or stripped.endswith("}")):
                out.append(("item", item_name(cur, masked_cur(cur)), "\n".join(cur) + "\n"))
                cur, depth = None, 0
            i += 1
            continue
        cur.append(raw)
        depth += m.count("{") + m.count("(") + m.count("[") - m.count("}") - m.count(")") - m.count("]")
        if depth <= 0 and (stripped.endswith(";") or stripped.endswith("}") or stripped.endswith("),")):
            out.append(("item", item_name(cur, masked_cur(cur)), "\n".join(cur) + "\n"))
            cur, depth = None, 0
        i += 1
    if cur is not None:
        raise ValueError("unterminated item: " + cur[0][:80])
    if pending:
        out.append(("gap", "", "\n".join(pending) + "\n"))
    return out


def masked_cur(cur):
    return mask("\n".join(cur))


def item_name(cur, mtext):
    for line in mtext.split("\n"):
        s = line.strip()
        if not s or s.startswith("#[") or s.startswith("//"):
            continue
        m = NAME.search(line)
        if m:
            return m.group(1)
        m = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?(?:impl|use)\b(.*)", line)
        if m:
            return line.strip()[:60]
        return line.strip()[:60]
    return ""


if __name__ == "__main__":
    t = open(sys.argv[1], encoding="utf-8").read()
    parts = split_items(t)
    assert "".join(p[2] for p in parts).rstrip("\n") == t.rstrip("\n"), "split is not lossless"
    for k, n, x in parts:
        print(k, len(x.encode()), n[:70])
