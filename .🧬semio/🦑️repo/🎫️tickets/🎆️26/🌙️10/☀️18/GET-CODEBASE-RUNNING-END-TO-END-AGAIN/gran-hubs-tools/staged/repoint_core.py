"""🔀 Facade repoint core: rewrites the paths of one hub crate in a Rust source text to the producer crates that own the names (`use` trees, inline paths, module aliases, `extern crate .. as` aliases). Pure functions on text, fail-closed, no I/O.

`rewrite_file(text, idents, resolve) -> Result` where `idents` is the set of crate identifiers that name the hub (`semio_framework_os_kernel`, aliases) and `resolve(segs)` maps a path below the hub to one of
  ("rewrite", owner_ident, new_segs)    the name lives in another crate
  ("own",)                              the hub itself defines it (macro, type, const): the hub edge must stay
  ("modalias", hub_segs)                the path is a pure re-export module: `use hub::m;` becomes an alias for its members
  ("fail", reason)
Docs: tools/staged/README in the ticket result (`result-gran-hubs.md`, section 5)."""
import re
from dataclasses import dataclass, field

IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def skeleton(t):
    """Blanks comments, strings and char literals with spaces (positions and newlines kept)."""
    o = list(t)
    i = 0
    n = len(t)

    def blank(a, b):
        for k in range(a, b):
            if o[k] != "\n":
                o[k] = " "

    while i < n:
        c = t[i]
        if c == "/" and t.startswith("//", i):
            j = t.find("\n", i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif c == "/" and t.startswith("/*", i):
            d = 1
            j = i + 2
            while j < n and d:
                if t.startswith("/*", j):
                    d += 1
                    j += 2
                elif t.startswith("*/", j):
                    d -= 1
                    j += 2
                else:
                    j += 1
            blank(i, j)
            i = j
        elif c in "rb" and re.match(r'(?:br|r)#*"', t[i:i + 12]) and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == "_")):
            m = re.match(r'(?:br|r)(#*)"', t[i:])
            end = '"' + m.group(1)
            j = t.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            blank(i + len(m.group(0)), j - len(end)) if j - len(end) > i + len(m.group(0)) else None
            i = j
        elif (c == "b" and t.startswith('b"', i)) or c == '"':
            j = i + (2 if c == "b" else 1)
            while j < n and t[j] != '"':
                j += 2 if t[j] == "\\" else 1
            blank(i + (2 if c == "b" else 1), j)
            i = j + 1
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", t[i:i + 12])
            if m:
                blank(i + 1, i + len(m.group(0)) - 1)
                i += len(m.group(0))
            else:
                i += 1
        else:
            i += 1
    return "".join(o)


def split_commas(s):
    parts, d, cur = [], 0, ""
    for ch in s:
        if ch == "{":
            d += 1
        if ch == "}":
            d -= 1
        if ch == "," and d == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return parts


def leaves(tree):
    """Flattens a use tree into [(segs, alias)]; whitespace-insensitive except around `as`."""
    u = re.sub(r"\s+", " ", tree.strip())
    u = re.sub(r"\s*(::|\{|\}|,)\s*", r"\1", u).strip()
    out = []

    def rec(s, pre):
        s = s.strip()
        if not s:
            return
        if s.startswith("{") and s.endswith("}"):
            for p in split_commas(s[1:-1]):
                rec(p, pre)
            return
        m = re.match(r"^(::)?(.*?)::(\{.*\})$", s)
        if m:
            segs = tuple(x for x in m.group(2).split("::") if x)
            for p in split_commas(m.group(3)[1:-1]):
                rec(p, pre + segs)
            return
        am = re.match(r"^(.*?) as (%s)$" % IDENT, s)
        alias = None
        if am:
            s, alias = am.group(1), am.group(2)
        out.append((pre + tuple(x for x in s.split("::") if x), alias))

    rec(u, ())
    return out


def emit_tree(segs, alias):
    return "::".join(segs) + (f" as {alias}" if alias else "")


def group_text(owner, items):
    """items: [(rest_segs, alias)] -> `owner::a::b` or `owner::{a::b, c as d}`."""
    if len(items) == 1:
        segs, al = items[0]
        return owner + "::" + emit_tree(segs, al) if segs else owner + (f" as {al}" if al else "")
    inner = ", ".join(emit_tree(s, a) for s, a in items)
    return f"{owner}::{{{inner}}}"


@dataclass
class Result:
    text: str
    owners: set = field(default_factory=set)
    own_refs: int = 0
    errors: list = field(default_factory=list)
    edits: int = 0
    aliases: dict = field(default_factory=dict)


def _statements(sk):
    """[(attr_start, vis_text, use_pos, end)] for every `use ...;` and `extern crate ...;` statement."""
    out = []
    for m in re.finditer(r"(?<![A-Za-z0-9_])(use|extern\s+crate)(?![A-Za-z0-9_])", sk):
        p = m.start()
        d = 0
        j = m.end()
        n = len(sk)
        while j < n:
            ch = sk[j]
            if ch in "{(":
                d += 1
            elif ch in "})":
                d -= 1
            elif ch == ";" and d <= 0:
                break
            j += 1
        if j >= n:
            continue
        k = p
        vis = ""
        while True:
            q = k
            while q > 0 and sk[q - 1].isspace():
                q -= 1
            if q >= 1 and sk[q - 1] == ")":
                depth = 0
                r = q - 1
                while r >= 0:
                    if sk[r] == ")":
                        depth += 1
                    elif sk[r] == "(":
                        depth -= 1
                        if depth == 0:
                            break
                    r -= 1
                w = sk[:r].rstrip()
                if w.endswith("pub"):
                    vis = sk[w.rfind("pub"):q]
                    k = w.rfind("pub")
                    continue
                break
            if sk[max(0, q - 3):q] == "pub" and (q < 4 or not (sk[q - 4].isalnum() or sk[q - 4] == "_")):
                vis = "pub"
                k = q - 3
                continue
            break
        out.append((k, vis.strip(), p, j + 1, m.group(1).split()[0]))
    return out


def _attrs_before(t, sk, k):
    """Raw text of the attributes (`#[..]`) directly before position k."""
    attrs = []
    q = k
    while True:
        while q > 0 and sk[q - 1].isspace():
            q -= 1
        if q > 0 and sk[q - 1] == "]":
            depth = 0
            r = q - 1
            while r >= 0:
                if sk[r] == "]":
                    depth += 1
                elif sk[r] == "[":
                    depth -= 1
                    if depth == 0:
                        break
                r -= 1
            h = r - 1
            inner = False
            while h >= 0 and sk[h] in " !":
                inner = inner or sk[h] == "!"
                h -= 1
            if h >= 0 and sk[h] == "#" and not inner:
                attrs.insert(0, t[h:q])
                q = h
                continue
        break
    return attrs


def _resolve_prefix(resolve, allsegs, minlen):
    cut = len(allsegs)
    last = ("fail", "unresolved")
    while cut >= minlen:
        r = resolve(allsegs[:cut])
        if r[0] != "fail":
            return r, cut
        last = r
        cut -= 1
    return last, 0


def rewrite_file(text, idents, resolve, alias_cfg_test=False, skip_spans=(), bare_ok=(), real_idents=None):
    sk = skeleton(text)
    res = Result(text)
    idents = set(idents)
    edits = []
    stmts = _statements(sk)
    spans = [(s[0], s[3]) for s in stmts]
    mod_alias = {}

    def in_stmt(pos):
        return any(a <= pos < b for a, b in spans)

    for k, vis, p, end, kind in stmts:
        raw = text[p:end]
        body = sk[p:end]
        if not any(re.search(r"(?<![A-Za-z0-9_])" + re.escape(i) + r"(?![A-Za-z0-9_])", body) for i in idents):
            continue
        if re.sub(r"\s+", "", raw) != re.sub(r"\s+", "", body):
            res.errors.append("comment or string inside a statement naming the hub: " + raw[:60].replace("\n", " "))
            continue
        if kind == "extern":
            m = re.match(r"extern\s+crate\s+(%s)\s+as\s+(%s)\s*;" % (IDENT, IDENT), body.strip())
            if m and m.group(2) in idents and vis == "":
                edits.append((k, end, ("#[cfg(test)]\n" + text[k:end]) if alias_cfg_test else ""))
                res.edits += 1
                continue
            res.errors.append("extern crate form not supported: " + body.strip()[:60])
            continue
        tree = body[body.index("use") + 3:body.rindex(";")]
        lv = leaves(tree)
        by_owner, order, own_items, other = {}, [], [], []
        crate_alias = []
        own_root = None
        failed = False
        for segs, alias in lv:
            if not segs or segs[0] not in idents:
                other.append((segs, alias))
                continue
            rest = list(segs[1:])
            if not rest or rest[-1] in ("self", "*"):
                res.errors.append("hub imported as a whole or by glob: " + emit_tree(segs, alias))
                failed = True
                continue
            r = resolve(rest)
            if r[0] == "rewrite" and not r[2]:
                crate_alias.append(f"use {r[1]} as {alias or rest[-1]};")
                res.owners.add(r[1])
                try:
                    r2 = resolve(rest, macro_ns=True)
                except TypeError:
                    r2 = None
                if r2 and r2[0] == "rewrite" and r2[2]:
                    if r2[1] not in by_owner:
                        by_owner[r2[1]] = []
                        order.append(r2[1])
                    by_owner[r2[1]].append((tuple(r2[2]), alias))
            elif r[0] == "rewrite":
                _, owner, newsegs = r
                if owner not in by_owner:
                    by_owner[owner] = []
                    order.append(owner)
                by_owner[owner].append((tuple(newsegs), alias))
                res.owners.add(owner)
            elif r[0] == "own":
                own_items.append((tuple(rest), alias))
                own_root = own_root or segs[0]
                res.own_refs += 1
            elif r[0] == "modalias":
                mod_alias[alias or rest[-1]] = list(r[1])
            else:
                res.errors.append(f"{emit_tree(segs, alias)}: {r[1]}")
                failed = True
        if failed:
            continue
        stmts_out = crate_alias + [f"use {group_text(o, by_owner[o])};" for o in order]
        if own_items:
            stmts_out.append(f"use {group_text(own_root, own_items)};")
        stmts_out += [f"use {emit_tree(s, a)};" for s, a in other]
        ls = text.rfind("\n", 0, k) + 1
        indent = text[ls:k] if text[ls:k].strip() == "" else ""
        pre = "".join(a + "\n" + indent for a in _attrs_before(text, sk, k))
        vis_txt = (vis + " ") if vis else ""
        if not stmts_out:
            edits.append((k, end, ""))
        else:
            new = stmts_out[0]
            for s in stmts_out[1:]:
                new += "\n" + indent + pre + vis_txt + s
            edits.append((p, end, new))
        res.edits += 1

    for nm in sorted(set(idents) | set(mod_alias)):
        base = list(mod_alias.get(nm, [])) if nm in mod_alias and nm not in idents else []
        for m in re.finditer(r"(?<![A-Za-z0-9_.])" + re.escape(nm) + r"(?![A-Za-z0-9_])", sk):
            pos = m.start()
            if in_stmt(pos) or any(a <= pos < b for a, b in skip_spans):
                continue
            if sk[max(0, pos - 2):pos] == "::":
                q = pos - 2
                while q > 0 and sk[q - 1].isspace():
                    q -= 1
                if q > 0 and (sk[q - 1].isalnum() or sk[q - 1] in "_>)"):
                    continue
            j = m.end()
            segs = []
            while True:
                q = j
                while q < len(sk) and sk[q].isspace():
                    q += 1
                if sk.startswith("::", q):
                    q += 2
                    while q < len(sk) and sk[q].isspace():
                        q += 1
                    mm = re.compile(IDENT).match(sk, q)
                    if mm:
                        segs.append((mm.group(0), mm.end()))
                        j = mm.end()
                        continue
                break
            if not segs:
                if nm in bare_ok:
                    res.owners.add(nm)
                elif nm in idents:
                    after = sk[m.end():m.end() + 3].lstrip()
                    before = sk[max(0, pos - 3):pos]
                    is_alias = real_idents is not None and nm not in real_idents
                    if is_alias and before.rstrip()[-2:] != "::" and not (before.rstrip().endswith("=") and "crate" in sk[max(0, pos - 12):pos]):
                        pass
                    else:
                        res.errors.append("bare hub identifier at offset %d" % pos)
                continue
            allsegs = base + [s for s, _ in segs]
            after = sk[segs[-1][1]:segs[-1][1] + 3].lstrip()
            if after.startswith("!") and not after.startswith("!="):
                try:
                    r, cut = resolve(allsegs, macro_ns=True), len(allsegs)
                except TypeError:
                    r, cut = _resolve_prefix(resolve, allsegs, len(base) + 1)
                if r[0] == "fail":
                    r, cut = _resolve_prefix(resolve, allsegs, len(base) + 1)
            else:
                r, cut = _resolve_prefix(resolve, allsegs, len(base) + 1)
            if r[0] == "fail":
                res.errors.append(f"{nm}::{'::'.join(s for s, _ in segs)}: {r[1]}")
                continue
            if r[0] == "own":
                res.own_refs += 1
                continue
            if r[0] == "modalias":
                res.errors.append("module path used as a value: " + nm + "::" + "::".join(s for s, _ in segs))
                continue
            _, owner, newsegs = r
            consumed = cut - len(base)
            endp = segs[consumed - 1][1]
            lead = "::" if text[max(0, pos - 2):pos] == "::" else ""
            edits.append((pos - len(lead), endp, lead + owner + ("::" + "::".join(newsegs) if newsegs else "")))
            res.owners.add(owner)
            res.edits += 1
    edits.sort()
    for a, b in zip(edits, edits[1:]):
        if a[1] > b[0]:
            res.errors.append("overlapping edits")
            break
    if res.errors:
        res.text = text
        res.edit_list = []
        return res
    out = text
    for a, b, new in reversed(edits):
        out = out[:a] + new + out[b:]
    res.text = out
    res.edit_list = edits
    return res
