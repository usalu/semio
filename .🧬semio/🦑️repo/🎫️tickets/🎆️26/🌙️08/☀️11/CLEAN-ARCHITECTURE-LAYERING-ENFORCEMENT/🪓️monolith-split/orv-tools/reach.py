"""Module-tree walker of the orv tools: resolves #[path] mounts (inline-module aware), include!/include_str!/include_bytes! literals.
usage: reach.py <target|all>   prints the reachable .rs files and side files of the crate closure."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *

INC = ("include", "include_str", "include_bytes")


def scan(path, text=None):
    """Returns (mounts, includes, missing_plain): mounts = [(abs_path, cfg_test_only)], includes = [abs_path]."""
    text = read(path) if text is None else text
    toks = [t for t in lex(text) if t[0] != "comment"]
    here = os.path.dirname(path)
    mounts, includes, plain = [], [], []
    stack = []
    depth = 0
    pending_path = None
    pending_attr_path_inline = None
    i = 0
    n = len(toks)
    attrs_path = None
    while i < n:
        k, x = toks[i][0], toks[i][1]
        if x == "#" and k == "p":
            j = i + 1
            if toks[j][1] == "!":
                j += 1
            d = 0
            s = j
            while True:
                if toks[j][1] == "[":
                    d += 1
                elif toks[j][1] == "]":
                    d -= 1
                    if d == 0:
                        break
                j += 1
            seg = toks[s:j + 1]
            if len(seg) >= 4 and seg[1][1] == "path" and seg[2][1] == "=" and seg[3][0] == "str":
                attrs_path = seg[3][1][1:-1]
            i = j + 1
            continue
        if k == "id" and x == "mod" and i + 1 < n and toks[i + 1][0] == "id":
            name = toks[i + 1][1]
            nxt = toks[i + 2][1] if i + 2 < n else ""
            if nxt == ";":
                comp = [c for c in (s_[1] for s_ in stack)]
                base = os.path.join(here, *comp) if comp else here
                if attrs_path is not None:
                    mounts.append(os.path.normpath(os.path.join(base, attrs_path)))
                else:
                    a, b = os.path.join(base, name + ".rs"), os.path.join(base, name, "mod.rs")
                    if os.path.isfile(a): mounts.append(a)
                    elif os.path.isfile(b): mounts.append(b)
                    else: plain.append((name, a))
                attrs_path = None
                i += 3
                continue
            if nxt == "{":
                stack.append((depth + 1, attrs_path if attrs_path is not None else name))
                attrs_path = None
                depth += 1
                i += 3
                continue
        if k == "p" and x == "{":
            depth += 1
        elif k == "p" and x == "}":
            if stack and stack[-1][0] == depth:
                stack.pop()
            depth -= 1
        elif k == "id" and x in INC and i + 3 < n and toks[i + 1][1] == "!" and toks[i + 2][1] == "(" and toks[i + 3][0] == "str":
            lit = toks[i + 3][1]
            if lit.startswith('"') and not lit.startswith('"/'):
                includes.append(os.path.normpath(os.path.join(here, lit[1:-1])))
        if k not in ("p",) and x not in ("pub", "unsafe", "async"):
            if not (k == "id" and x in ("pub",)):
                pass
        i += 1
    return mounts, includes, plain


def reach(root_abs, root_text=None):
    """BFS over mounts. Returns (rs_files in order, side_files, children {file: [mounted files]}, plain_missing)."""
    seen, order, side = set(), [], set()
    children = {}
    plain_all = []
    queue = [os.path.normpath(root_abs)]
    while queue:
        p = queue.pop(0)
        if p in seen:
            continue
        seen.add(p)
        if not os.path.isfile(p):
            continue
        order.append(p)
        m, inc, plain = scan(p, root_text if (root_text is not None and p == os.path.normpath(root_abs)) else None)
        plain_all += plain
        children[p] = []
        for q in m:
            children[p].append(q)
            queue.append(q)
        for q in inc:
            if q.endswith(".rs"):
                children[p].append(q)
                queue.append(q)
            else:
                side.add(q)
    return order, side, children, plain_all


def descendants(children, p):
    out, st = [], list(children.get(p, []))
    while st:
        q = st.pop()
        if q in out:
            continue
        out.append(q)
        st.extend(children.get(q, []))
    return out


if __name__ == "__main__":
    names = list(TARGETS) if sys.argv[1] == "all" else sys.argv[1].split(",")
    for nm in names:
        t = TARGETS[nm]
        order, side, children, plain = reach(f"{ROOT}/{t['root']}")
        print(f"== {nm}: {len(order)} rs files, {len(side)} side files, plain-mod misses {len(plain)}")
        for p in order:
            print("  ", os.path.relpath(p, ROOT).replace(M + "/", "M/"))
        for p in sorted(side):
            print("   side", os.path.relpath(p, ROOT).replace(M + "/", "M/"), "EXISTS" if os.path.isfile(p) else "MISSING")
        for pl in plain:
            print("   PLAIN-MISS", pl)
