"""Parses the original HubDirectory port into per-method units (docs + signature + optional default body), keyed by method name."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *


def parse_members(lines, indent="    "):
    """lines: list of source lines inside a block. Returns list of dicts: kind in {member, marker, blank, other}."""
    items = []
    i = 0
    n = len(lines)
    pending = []
    while i < n:
        l = lines[i]
        s = l.strip()
        if s == "":
            if pending:
                items.append(dict(kind="other", lines=pending)); pending = []
            items.append(dict(kind="blank", lines=[l])); i += 1; continue
        if re.match(r"^" + indent + r"//#(end)?region", l):
            if pending:
                items.append(dict(kind="other", lines=pending)); pending = []
            items.append(dict(kind="marker", lines=[l])); i += 1; continue
        if s.startswith("///") or s.startswith("#[") or (s.startswith("//") and not s.startswith("//#")):
            pending.append(l); i += 1; continue
        m = re.match(r"^" + indent + r"(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)", l)
        if m:
            start = i
            depth = 0
            seen_open = False
            j = i
            masked = [mask("\n".join(lines[k] for k in range(i, min(n, i + 400)))).split("\n")][0]
            end = None
            nest = 0
            for k in range(0, len(masked)):
                for ch in masked[k]:
                    if ch == "{":
                        depth += 1; seen_open = True
                    elif ch == "}":
                        depth -= 1
                    elif ch in "([":
                        nest += 1
                    elif ch in ")]":
                        nest -= 1
                    elif ch == ";" and depth == 0 and nest == 0 and not seen_open:
                        end = k
                        break
                if end is not None:
                    break
                if seen_open and depth == 0:
                    end = k
                    break
            assert end is not None, (lines[i], i)
            body = lines[i:i + end + 1]
            items.append(dict(kind="member", name=m.group(1), doc=pending, lines=body, has_body=seen_open))
            pending = []
            i += end + 1
            continue
        pending.append(l); i += 1
    if pending:
        items.append(dict(kind="other", lines=pending))
    return items


def port_members():
    text = read(DIR + "/🔌️port/🦀️.rs") if not os.environ.get("HDC_PORT_ORIG") else open(os.environ["HDC_PORT_ORIG"]).read()
    lines = text.split("\n")
    a = next(i for i, l in enumerate(lines) if l.startswith("pub trait HubDirectory"))
    b = max(i for i, l in enumerate(lines) if l == "}")
    items = parse_members(lines[a + 1:b])
    return lines[:a], items, lines[b:]


if __name__ == "__main__":
    head, items, tail = port_members()
    ms = [it for it in items if it["kind"] == "member"]
    print(len(ms), sum(1 for m in ms if m["has_body"]), [m["name"] for m in ms if m["has_body"]])
    others = [it for it in items if it["kind"] == "other"]
    print("other chunks", len(others))
    for o in others: print(o["lines"][:2])
