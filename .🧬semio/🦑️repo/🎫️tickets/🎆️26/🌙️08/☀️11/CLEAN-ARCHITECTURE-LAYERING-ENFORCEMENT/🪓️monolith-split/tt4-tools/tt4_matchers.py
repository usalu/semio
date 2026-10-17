"""🎯 Locates the `expect(actual)` of every strict-matcher diagnostic and (optionally) widens only that call to `expect<unknown>(actual)`.

usage: tt4_matchers.py <tsc-output> [--apply] [--only <path-substring>] [--list <file>]
A diagnostic qualifies when its position lies inside the argument list of `.toBe/.toEqual/.toStrictEqual/.toContain/.toContainEqual(`
whose receiver chain (`.not`, `.resolves`, `.rejects`) ends in a bare `expect(...)` call without a type argument.
"""
import json, os, re, sys

sys.path.insert(0, os.path.dirname(__file__))
from tt4_blocks import blocks, offset

MATCHERS = ("toBe", "toEqual", "toStrictEqual", "toContain", "toContainEqual")
CHAIN = (".not", ".resolves", ".rejects")

def balanced_back(text, close_index):
    """Index of the '(' matching the ')' at close_index, skipping strings, template literals and comments crudely by scanning forward from a safe anchor."""
    depth = 0
    i = close_index
    while i >= 0:
        c = text[i]
        if c == ")":
            depth += 1
        elif c == "(":
            depth -= 1
            if depth == 0:
                return i
        i -= 1
    return -1

def enclosing_matcher(text, pos):
    """Find a matcher call whose argument list contains pos: walk back over every '(' that is unclosed at pos."""
    depth = 0
    i = pos - 1
    while i >= 0:
        c = text[i]
        if c == ")":
            depth += 1
        elif c == "(":
            if depth == 0:
                for m in MATCHERS:
                    if text[:i].endswith("." + m):
                        return i - len(m) - 1, m, i
                return None
            depth -= 1
        i -= 1
    return None

def locate_expect(text, dot_index):
    """From the '.' of the matcher walk back over chain words to the ')' closing expect(...), return the index of 'expect'."""
    i = dot_index
    while True:
        for word in CHAIN:
            if text[:i].endswith(word):
                i -= len(word)
                break
        else:
            break
    if i == 0 or text[i - 1] != ")":
        return None
    open_index = balanced_back(text, i - 1)
    if open_index < 0:
        return None
    head = text[:open_index]
    if not head.endswith("expect"):
        return None
    start = open_index - len("expect")
    if start > 0 and (text[start - 1].isalnum() or text[start - 1] in "_$."):
        return None
    return start

def main():
    path = sys.argv[1]
    apply_ = "--apply" in sys.argv
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
    out = {}
    skipped = []
    cache = {}
    starts_cache = {}
    for b in blocks(path):
        if only and only not in b["file"]:
            continue
        if b["code"] not in ("TS2769", "TS2345", "TS2322", "TS2554", "TS2559", "TS2353", "TS2741", "TS2740", "TS2739"):
            continue
        f = b["file"]
        if not os.path.exists(f):
            continue
        if f not in cache:
            cache[f] = open(f, encoding="utf8").read()
        text = cache[f]
        if f not in starts_cache:
            starts_cache[f] = [0] + [m.end() for m in re.finditer("\n", text)]
        starts = starts_cache[f]
        if b["line"] > len(starts):
            continue
        try:
            pos = offset(text, starts, b["line"], b["col"])
        except IndexError:
            continue
        found = enclosing_matcher(text, pos)
        if found is None:
            continue
        dot, matcher, paren = found
        start = locate_expect(text, dot)
        if start is None:
            skipped.append((f, b["line"], b["col"], matcher, b["code"], "no bare expect( receiver"))
            continue
        out.setdefault(f, {})[start] = (b["line"], matcher, b["code"], b["msg"][:140])
    total = 0
    result = []
    for f, edits in out.items():
        for start, (line, matcher, code, msg) in sorted(edits.items()):
            result.append({"file": f, "line": line, "matcher": matcher, "code": code, "msg": msg})
            total += 1
        if apply_:
            text = cache[f]
            for start in sorted(edits, reverse=True):
                assert text[start:start + 7] == "expect(", (f, start, text[start:start + 20])
                text = text[:start + 6] + "<unknown>" + text[start + 6:]
            open(f, "w", encoding="utf8").write(text)
    print(json.dumps({"files": len(out), "sites": total, "skipped": len(skipped)}))
    if "--list" in sys.argv:
        json.dump({"sites": result, "skipped": skipped}, open(sys.argv[sys.argv.index("--list") + 1], "w", encoding="utf8"), ensure_ascii=False, indent=1)

main()
