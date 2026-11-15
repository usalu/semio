"""Author 69 helpers: edit one list of one scale of a human source in place (re-reads right before writing)."""
import json
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"

def edit(slug, scale, lst, fn):
    p = f"{T}/volumes/humans/{slug}.json"
    d = json.load(open(p))
    fn(d["volumes"][str(scale)][lst], d["volumes"][str(scale)])
    json.dump(d, open(p, "w"), ensure_ascii=False, separators=(",", ":"))

def by_id(lst, i):
    return next(s for s in lst if s["id"] == i)

import re, sys
def member_end(text, start):
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def member_range(text, indent, key):
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"

def member_text(slug, key):
    t = open(f"{CAT}/{slug}/🔣️.json").read()
    r = member_range(t, "    ", key)
    return t, r

def edit_member(slug, key, fn, dry=False):
    """Edit one top-level member of `asset` (4-space indent) in place; json.dumps(indent=2) formatting."""
    t, r = member_text(slug, key)
    seg = t[r[0]:r[1]]
    val = json.loads(seg[seg.index(":") + 1:])
    orig = json.dumps(val, indent=2, ensure_ascii=False).replace("\n", "\n    ")
    same = seg[seg.index(":") + 1:].strip() == orig.strip()
    out = fn(val)
    new = '    "%s": ' % key + json.dumps(val, indent=2, ensure_ascii=False).replace("\n", "\n    ")
    if dry: return same
    t2, r2 = member_text(slug, key)
    open(f"{CAT}/{slug}/🔣️.json", "w").write(t2[:r2[0]] + new + t2[r2[1]:])
    return same
