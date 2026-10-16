# ✂️ Text surgery on Rust module files: remove / replace the mount of a child file (path mod, include!, inline mod wrapping an include), prune dead imports.
import os, re
from rs import blank, expand_use

MOUNT = re.compile(r"(?P<attrs>(?:#\[[^\]]*\][ \t]*\n?[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?mod[ \t]+(?P<name>[A-Za-z_][A-Za-z0-9_]*)[ \t]*;[ \t]*\n?")
INLINE = re.compile(r"(?P<attrs>(?:#\[[^\]]*\][ \t]*\n?[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?mod[ \t]+(?P<name>[A-Za-z_][A-Za-z0-9_]*)[ \t]*\{")
INCL = re.compile(r'(?P<attrs>(?:#\[[^\]]*\][ \t]*\n?[ \t]*)*)include!\(\s*"(?P<path>[^"]+)"\s*\)[ \t]*;[ \t]*\n?')
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')


def depth_at(clean):
    d, out = 0, []
    for ch in clean:
        out.append(d)
        if ch == "{": d += 1
        elif ch == "}": d -= 1
    return out


def match_brace(clean, i):
    depth, j = 1, i
    while j < len(clean) and depth:
        depth += (clean[j] == "{") - (clean[j] == "}")
        j += 1
    return j


def find_mounts(text, parent_rel, child_rel):
    """-> list of dict(kind, start, end, name, vis, attrs, inline_span) of statements in `text` that mount child_rel"""
    clean = blank(text)
    base = os.path.dirname(parent_rel)
    dm = depth_at(clean)
    found = []
    for m in MOUNT.finditer(clean):
        attrs = text[m.start("attrs"):m.end("attrs")]
        pm = PATHATTR.search(attrs)
        if not pm:
            continue
        if os.path.normpath(os.path.join(base, pm.group(1))) == child_rel and dm[m.start("name")] == 0:
            found.append(dict(kind="mod", start=m.start(), end=m.end(), name=m.group("name"), vis=(m.group("vis") or "").strip(), attrs=attrs))
    for m in INCL.finditer(text):
        if clean[m.start("path") - 9:m.start("path") - 1].strip() == "" and not clean[m.start() + len(m.group("attrs")):m.start() + len(m.group("attrs")) + 8] == "include!":
            continue
        p = text[m.start("path"):m.end("path")]
        if os.path.normpath(os.path.join(base, p)) != child_rel:
            continue
        attrs = text[m.start("attrs"):m.end("attrs")]
        enclosing = None
        if dm[m.start()] > 0:
            for im in INLINE.finditer(clean):
                e = match_brace(clean, im.end())
                if im.start() < m.start() < e and dm[im.start("name")] == 0:
                    enclosing = (im.start(), e, im.group("name"), (im.group("vis") or "").strip(), text[im.start("attrs"):im.end("attrs")], im.end())
                    break
        found.append(dict(kind="include", start=m.start(), end=m.end(), name=None, vis="", attrs=attrs, inline=enclosing))
    return found


def replace_span(text, s, e, new):
    return text[:s] + new + text[e:]


def remove_mount(text, parent_rel, child_rel, replacement=None, names=None):
    """remove (or replace) every statement mounting child_rel; inline `mod X { use ..; include!(..); }` wrappers go whole when nothing else is inside"""
    for _ in range(20):
        ms = find_mounts(text, parent_rel, child_rel)
        if not ms:
            return text
        m = ms[0]
        if names is not None:
            names.append(m["name"] if m["kind"] == "mod" else (m["inline"][2] if m.get("inline") else None))
        if m["kind"] == "include" and m["inline"]:
            s, e, name, vis, attrs, body_start = m["inline"]
            clean = blank(text)
            body = text[body_start:e - 1]
            cbody = clean[body_start:e - 1]
            body_wo = re.sub(r"(?<![A-Za-z0-9_])use[ \t][^;]*;", "", cbody)
            n_inc = len(INCL.findall(body))
            rest = re.sub(r"include![ \t]*\([ \t]*\)[ \t]*;|include![ \t]*\(\s*\)\s*;", "", re.sub(r"include!\s*\([^)]*\)\s*;", "", body_wo)).strip()
            if rest == "" and n_inc <= 1:
                text = replace_span(text, s, min(e + 1, len(text)) if text[e:e + 1] == "\n" else e, replacement or "")
                continue
        text = replace_span(text, m["start"], m["end"], replacement or "")
    return text


def prune_dead_uses(text, keep_if=None, extra=""):
    """drop non-pub single-leaf `use` statements whose local name is unused in the rest of the file; returns (text, dropped list)"""
    clean = blank(text)
    dm = depth_at(clean)
    spans = [(m.start(), m.end()) for m in re.finditer(r"(?m)^[ \t]*(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*use[ \t]+([^;]*);[ \t]*\n?", clean)]
    stmts = []
    for m in re.finditer(r"(?m)^(?P<ind>[ \t]*)(?P<attrs>(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?use[ \t]+(?P<body>[^;]*);[ \t]*\n?", clean):
        if dm[m.start("ind")] != 0:
            continue
        stmts.append(m)
    rest = list(clean)
    for m in stmts:
        for i in range(m.start(), m.end()):
            if rest[i] != "\n": rest[i] = " "
    rest = "".join(rest) + "\n" + blank(extra)
    dropped, out, last = [], [], 0
    for m in stmts:
        if m.group("vis"):
            continue
        leaves = list(expand_use(m.group("body")))
        if any(glob for _, _, glob in leaves) or len(leaves) != 1:
            continue
        path, alias, glob = leaves[0]
        local = alias or path[-1]
        if local == "_" or local == "self":
            continue
        if re.search(r"(?<![A-Za-z0-9_$])" + re.escape(local) + r"(?![A-Za-z0-9_])", rest):
            continue
        if keep_if and keep_if(path):
            continue
        dropped.append("::".join(path))
        out.append(text[last:m.start()])
        last = m.end()
    out.append(text[last:])
    return "".join(out), dropped


def drop_uses_of(text, names):
    """remove depth-0 `use` statements (any visibility) whose path starts with one of the module names; returns (text, removed statements)"""
    names = {n for n in names if n}
    if not names:
        return text, []
    clean = blank(text)
    dm = depth_at(clean)
    out, last, removed = [], 0, []
    for m in re.finditer(r"(?m)^(?P<ind>[ \t]*)(?P<attrs>(?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(?P<vis>pub(?:[ \t]*\([^)]*\))?[ \t]+)?use[ \t]+(?P<body>[^;]*);[ \t]*\n?", clean):
        if dm[m.start("ind")] != 0:
            continue
        leaves = list(expand_use(m.group("body")))
        if not leaves:
            continue
        heads = [(p[1] if p and p[0] in ("self",) and len(p) > 1 else p[0]) for p, a, g in leaves if p]
        if all(h in names for h in heads):
            removed.append(text[m.start("vis") if m.group("vis") else m.start("body") - 4:m.end()].strip())
            out.append(text[last:m.start()])
            last = m.end()
    out.append(text[last:])
    return "".join(out), removed
