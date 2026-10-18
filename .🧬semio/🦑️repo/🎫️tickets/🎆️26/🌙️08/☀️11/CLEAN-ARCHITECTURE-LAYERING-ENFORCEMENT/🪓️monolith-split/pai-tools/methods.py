"""📐 pai method table: depth-1 methods of the single PluginApp impl with doc blocks, bounds and sizes. usage: methods.py [file]"""
import re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
from rslex import lex

HEAD = re.compile(r"^    (?:pub\(super\)\s+)?(async\s+)?fn\s+(\w+)")


def parse(text, header="PluginApp for"):
    ls = text.split("\n")
    d, lit = lex(text)
    impl = [i for i, l in enumerate(ls) if l.startswith("impl<") and header in l]
    assert len(impl) == 1, impl
    a = impl[0]
    b = next(i for i in range(a + 1, len(ls)) if ls[i] == "}")
    heads = []
    for i in range(a + 1, b):
        if d[i] == 1 and not lit[i] and HEAD.match(ls[i]):
            s = i
            while s - 1 > a and d[s - 1] == 1 and ls[s - 1].strip().startswith(("///", "#[", "//")):
                s -= 1
            heads.append((s, i))
    out = []
    for k, (s, i) in enumerate(heads):
        nxt = heads[k + 1][0] if k + 1 < len(heads) else b
        e = nxt - 1
        while ls[e].strip() == "":
            e -= 1
        m = HEAD.match(ls[i])
        out.append(dict(name=m.group(2), s=s, h=i, e=e, is_async=bool(m.group(1))))
    return ls, a, b, out


if __name__ == "__main__":
    p = sys.argv[1] if len(sys.argv) > 1 else HERE + "/pai.orig.rs"
    ls, a, b, ms = parse(open(p, encoding="utf-8").read())
    print("impl", a + 1, "end", b + 1, "methods", len(ms))
    for m in ms:
        body = "\n".join(ls[m["s"]:m["e"] + 1])
        print(f'{m["s"]+1:5}-{m["e"]+1:<5} {m["e"]-m["s"]+1:4}L {len(body.encode()):6}B {"async" if m["is_async"] else "     "} {m["name"]}')


def split_sig(ls, m):
    """Return (sig_text_without_body_brace, body_start_line, body_start_col) for method m."""
    text = "\n".join(ls[m["h"]: m["e"] + 1])
    k = text.index("(")
    depth = 0
    j = k
    while True:
        c = text[j]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                break
        j += 1
    b = text.index("{", j)
    return text[:b].rstrip(), text[b:], text[k + 1:j]


def params(plist):
    out, depth, cur = [], 0, ""
    for c in plist:
        if c in "<([":
            depth += 1
        elif c in ">)]":
            depth -= 1
        if c == "," and depth == 0:
            out.append(cur); cur = ""
        else:
            cur += c
    if cur.strip():
        out.append(cur)
    names = []
    for p in out:
        p = p.strip()
        if p in ("&self", "&mut self", "self"):
            continue
        n = p.split(":", 1)[0].strip()
        n = n.replace("mut ", "").strip()
        names.append(n)
    return names
