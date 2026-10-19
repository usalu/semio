"""Move trait-impl method bodies that touch one component field onto that component's inherent impl; the trait method keeps a one-line delegation."""
import sys, re
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from lib import *
import nsmap

def import_map(texts):
    """name -> full path from every top-level `use` of the given texts"""
    out = {}
    for t in texts:
        for u in nsmap.use_statements(t):
            s = u.strip()
            if s.startswith("pub") or "#[cfg" in s:
                pass
            body = re.sub(r'^(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+', '', s).rstrip(";").strip()
            def walk(prefix, b):
                b = b.strip()
                if b.startswith("{") and b.endswith("}"):
                    inner = b[1:-1]
                    depth = 0; part = ""; parts = []
                    for ch in inner:
                        if ch == "{": depth += 1
                        if ch == "}": depth -= 1
                        if ch == "," and depth == 0:
                            parts.append(part); part = ""
                        else:
                            part += ch
                    if part.strip(): parts.append(part)
                    for p in parts: walk(prefix, p)
                    return
                m = re.match(r'^(.*?)::(\{.*\})$', b, re.S)
                if m:
                    walk(prefix + m.group(1) + "::", m.group(2)); return
                if b == "*" or b.endswith("::*"):
                    return
                m2 = re.match(r'^(.*?)\s+as\s+([A-Za-z0-9_]+)$', b)
                if m2:
                    out[m2.group(2)] = prefix + m2.group(1).strip()
                else:
                    leaf = b.split("::")[-1]
                    if leaf != "self":
                        out[leaf] = prefix + b
            walk("", body)
    return out

def find_method(body, name):
    for it in top_items(body):
        m = re.search(r'(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+' + re.escape(name) + r'\b', it)
        if m:
            return it
    raise KeyError(name)

def split_sig(it, name):
    m = re.search(r'fn\s+' + re.escape(name) + r'\b', it)
    i = it.index("(", m.end())
    depth = 0; j = i
    while True:
        c = it[j]
        if c == "(": depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0: break
        j += 1
    k = it.index("{", j)
    return it[:k], it[k:], it[i + 1:j]

def params(arglist):
    out = []; depth = 0; part = ""
    for ch in arglist:
        if ch in "<([": depth += 1
        if ch in ">)]": depth -= 1
        if ch == "," and depth == 0:
            out.append(part); part = ""
        else:
            part += ch
    if part.strip(): out.append(part)
    names = []
    for p in out:
        p = p.strip()
        if "self" in p.split(":")[0] and ":" not in p: continue
        if p.endswith("self"): continue
        names.append(re.sub(r"^mut\s+", "", p.split(":")[0].strip()))
    return names

def qualify(text, imap, allow=()):
    """qualify imported type/const names (and allow-listed functions) with their full path; strings and comments untouched"""
    pieces = re.split(r'("(?:\\.|[^"\\])*"|//[^\n]*)', text)
    out = []
    for k, pc in enumerate(pieces):
        if k % 2 == 1:
            out.append(pc); continue
        def rep(m):
            n = m.group(0)
            if n in imap and (n[0].isupper() or n in allow):
                return imap[n]
            return n
        out.append(re.sub(r"(?<![A-Za-z0-9_:.'])[A-Za-z_][A-Za-z0-9_]*(?![A-Za-z0-9_])", rep, pc))
    return "".join(out)

def move_method(src, impl_marker, name, field, tgt, tgt_impl_re, imap, allow=(), dry=False, selfcrate=None):
    t = rd(src)
    i = t.index(impl_marker)
    o = t.index("{", i)
    c = block_bounds(t, o)
    body = t[o + 1:c - 1]
    it = find_method(body, name)
    sig, fn_body, arglist = split_sig(it, name)
    pnames = params(arglist)
    is_async = re.search(r'\basync\s+fn\b', sig) is not None
    deleg = sig.rstrip() + " { self.%s.%s(%s)%s }\n" % (field, name, ", ".join(pnames), ".await" if is_async else "")
    comp_sig = re.sub(r'(?m)^(\s*)(async\s+)?fn\b', lambda m: m.group(1) + "pub " + (m.group(2) or "") + "fn", sig, count=1)
    comp_body = fn_body.replace("self.%s." % field, "self.")
    assert not re.search(r'self\.%s(?![A-Za-z0-9_])' % field, comp_body), name
    comp = qualify(comp_sig, imap, allow) + qualify(comp_body, imap, allow)
    if selfcrate:
        comp = comp.replace(selfcrate[0], selfcrate[1])
    if not comp.endswith("\n"):
        comp += "\n"
    new_t = t[:o + 1] + body.replace(it, deleg, 1) + t[c - 1:]
    tt = rd(tgt)
    m = re.search(tgt_impl_re, tt)
    assert m, tgt_impl_re
    ob = tt.index("{", m.end() - 1) if tt[m.end() - 1] != "{" else m.end() - 1
    cb = block_bounds(tt, ob)
    new_tt = tt[:cb - 1].rstrip("\n") + "\n\n" + comp.rstrip("\n").lstrip("\n") + "\n" + tt[cb - 1:]
    if dry:
        return deleg, comp, len(it.encode()), len(deleg.encode())
    wr(src, new_t)
    wr(tgt, new_tt)
    return len(it.encode()), len(deleg.encode())
