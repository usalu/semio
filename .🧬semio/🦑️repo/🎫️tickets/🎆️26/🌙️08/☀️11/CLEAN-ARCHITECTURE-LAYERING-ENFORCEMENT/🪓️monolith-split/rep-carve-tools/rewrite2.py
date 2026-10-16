"""🧭 Second rewrite pass: relative paths that lost their target and names a file used to get from its former namespace."""
import os, re, collections
from rlib import *
import rcrates as C
import usetree
from rewrite import Model, sig_tokens

def old_node_of(model, rel, origin):
    r = origin.get(rel, rel)
    return model.old.nodes.get(r)

def new_node_of(model, key, rel):
    return model.worlds[key].nodes.get(rel)

def defined_or_imported(node):
    out = set(node.defs) | set(node.mods)
    for pub, leaves, it in node.uses:
        for lf in leaves:
            n = usetree.local_name(lf)
            if n: out.add(n)
    return out

def import_line(model, res, name, key):
    """`use` line that makes `name` (resolved as `res` in the old world) available in a file of crate `key`."""
    if res[0] == "def":
        p = model.def_ref(res[1], res[2], key)
    elif res[0] == "ext":
        p = "::".join(res[1])
    elif res[0] == "mod":
        p = model.mod_ref(res[1], key)
    else:
        raise KeyError(res)
    last = p.split("::")[-1]
    return "use " + p + ("" if last == name else " as " + name) + ";"

def insert_imports(text, lines):
    if not lines: return text
    its = items(text)
    last_use = None
    for it in its:
        if it["kind"] == "use" and not it["vis"] and "#[cfg" not in "".join(it["attrs"]):
            last_use = it
        if it["kind"] not in ("use", None) and it["kind"] != "mod":
            break
    block = "\n".join(sorted(set(lines)))
    if last_use is not None:
        return text[:last_use["end"]] + "\n" + block + text[last_use["end"]:]
    L = text.split("\n")
    i = 0
    while i < len(L) and (L[i].startswith("//!") or L[i].startswith("#![") or L[i].strip() == "" or (L[i].startswith("//") and not L[i].startswith("///") and not L[i].startswith("//#"))): i += 1
    return "\n".join(L[:i]) + ("\n" if i else "") + block + "\n" + ("\n" if L[i:i + 1] and L[i].strip() else "") + "\n".join(L[i:])

def fix_relative(text, model, key, new_node, old_node):
    """Rewrites `super::`/`self::` use leaves and inline paths that no longer resolve in the new world."""
    w = model.worlds[key]
    toks = tokenize(text)
    sig = sig_tokens(toks)
    n = len(sig)
    edits = []
    si = 0
    while si < n:
        kd, tx, a, b = toks[sig[si]]
        if kd == "id" and tx == "use" and not (si > 0 and toks[sig[si - 1]][1] in (".", "$")):
            depth = 0; sj = si + 1
            while sj < n:
                t2 = toks[sig[sj]]
                if t2[1] in "{(": depth += 1
                elif t2[1] in "})": depth -= 1
                elif t2[1] == ";" and depth == 0: break
                sj += 1
            tree = re.sub(r"//[^\n]*", "", text[toks[sig[si]][3]:toks[sig[sj]][2]])
            leaves = usetree.parse(tree)
            if any(lf[0][0] in ("super", "self") for lf in leaves):
                new_leaves = []; changed = False
                for path, alias, glob in leaves:
                    if path[0] in ("super", "self") and len(path) > (0 if glob else 1):
                        r, c = w.walk(new_node, list(path))
                        if r is None or (c < len(path) and r[0] == "mod"):
                            ro, co = model.old.walk(old_node, list(path))
                            if ro is not None and ro[0] in ("def", "ext", "mod"):
                                try:
                                    if ro[0] == "def": t = model.def_ref(ro[1], ro[2], key)
                                    elif ro[0] == "ext": t = "::".join(ro[1])
                                    else: t = model.mod_ref(ro[1], key)
                                except KeyError as e:
                                    raise KeyError(("relative leaf", new_node.key, path, e))
                                newp = t.split("::") + list(path[co:])
                                new_leaves.append((newp, alias, glob)); changed = True; continue
                    new_leaves.append((path, alias, glob))
                if changed:
                    k = si - 1
                    start_si = si
                    if k >= 0 and toks[sig[k]][1] == "pub": start_si = k
                    a0 = toks[sig[start_si]][2]; b0 = toks[sig[sj]][3]
                    prefix = text[a0:toks[sig[si]][2]]
                    ls = text.rfind("\n", 0, a0) + 1
                    indent = re.match(r"[ \t]*", text[ls:a0]).group(0) if text[ls:a0].strip() == "" else ""
                    edits.append((a0, b0, ("\n" + indent).join(prefix + l for l in usetree.emit(new_leaves))))
            si = sj + 1
            continue
        si += 1
    use_spans = []
    si = 0
    while si < n:
        kd, tx, a, b = toks[sig[si]]
        if kd == "id" and tx == "use" and not (si > 0 and toks[sig[si - 1]][1] in (".", "$")):
            sj = si + 1; depth = 0
            while sj < n:
                t2 = toks[sig[sj]]
                if t2[1] in "{(": depth += 1
                elif t2[1] in "})": depth -= 1
                elif t2[1] == ";" and depth == 0: break
                sj += 1
            use_spans.append((a, toks[sig[sj]][3])); si = sj + 1; continue
        si += 1
    si = 0
    while si < n:
        kd, tx, a, b = toks[sig[si]]
        if kd == "id" and tx in ("super", "self") and si + 2 < n and toks[sig[si + 1]][1] == ":" and toks[sig[si + 2]][1] == ":" and not any(x <= a < y for x, y in use_spans):
            prev = toks[sig[si - 1]][1] if si > 0 else ""
            prev2 = toks[sig[si - 2]][1] if si > 1 else ""
            if prev in ("$", "in") or (prev == ":" and prev2 == ":") or (prev == "(" and prev2 == "pub"):
                si += 1; continue
            segs = [tx]; ends = [sig[si]]
            k = si + 3
            while k < n and toks[sig[k]][0] == "id":
                segs.append(toks[sig[k]][1]); ends.append(sig[k])
                if k + 3 < n and toks[sig[k + 1]][1] == ":" and toks[sig[k + 2]][1] == ":" and toks[sig[k + 3]][0] == "id":
                    k += 3
                else:
                    break
            while segs and segs[-1] in ("super", "self") and False: pass
            r, c = w.walk(new_node, list(segs))
            if r is None:
                ro, co = model.old.walk(old_node, list(segs))
                if ro is not None and ro[0] in ("def", "ext"):
                    t = model.def_ref(ro[1], ro[2], key) if ro[0] == "def" else "::".join(ro[1])
                    edits.append((a, toks[ends[co - 1]][3], t))
                    si = k + 1
                    continue
        si += 1
    edits.sort()
    out = []; last = 0
    for a, b, r in edits:
        out.append(text[last:a]); out.append(r); last = b
    out.append(text[last:])
    return "".join(out), len(edits)

def augment(text, model, key, new_node, old_node, report, rel):
    used = bare_ids(text)
    own = defined_or_imported(new_node)
    lines = []
    for name in sorted(used):
        if name in own: continue
        ro = model.old.lookup(old_node, name)
        if ro is None: continue
        rn = model.worlds[key].lookup(new_node, name)
        if rn is not None:
            continue
        if ro[0] == "def" and ro[1] is old_node and name in old_node.defs and False: continue
        try:
            lines.append(import_line(model, ro, name, key))
        except (AssertionError, KeyError) as e:
            report.append((rel, name, str(e)[:100]))
    if lines:
        report.extend((rel, "+", l) for l in lines)
    return insert_imports(text, lines)

def has_super_glob(node):
    for pub, leaves, it in node.uses:
        for path, alias, glob in leaves:
            if glob and path and path[0] == "super": return True
    return False

def run(model, origin, cut_files, report):
    changed = 0
    for key in C.KEYS:
        w = model.worlds[key]
        for f in crawl(C.libroot(key)):
            nn = new_node_of(model, key, f)
            on = old_node_of(model, f, origin)
            if nn is None or on is None: continue
            full = read_stage_or_base(f)
            cut = full.find("\n\nmod owned_retirement {")
            t, tail = (full[:cut], full[cut:]) if cut >= 0 else (full, "")
            n, k = fix_relative(t, model, key, nn, on)
            if has_super_glob(nn) or f in cut_files or f in origin:
                n = augment(n, model, key, nn, on, report, f.replace(R, ""))
            if n != t:
                write(f, n + tail); changed += 1
    return changed
