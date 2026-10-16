# 🚚 Item mover for orphan-rule repairs on the live tree: cuts trait impls / macro invocations out of a kernel or crate file and appends them to a staging module of the crate that owns the trait or the type.
# Names are resolved in the source module (current tree); paths into carved crates come out as their `semio_framework_*` idents, paths into the destination become `crate::`.
# usage: mover.py dry|write <moves-name>
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns, registry, widen
import okcrates as C
from okrs import tokenize, items

KW = {'fn', 'let', 'mut', 'ref', 'pub', 'use', 'mod', 'struct', 'enum', 'trait', 'impl', 'type', 'const', 'static', 'where', 'for', 'in', 'as', 'dyn', 'if', 'else', 'match', 'return', 'loop', 'while', 'move', 'unsafe', 'async', 'await', 'break', 'continue', 'true', 'false', 'self', 'Self', 'extern', 'crate', 'super'}

def ident_of(key): return 'semio_framework_' + key.replace('-', '_')

def module_of(path):
    w = ns.world()
    if path in w.files: return w.files[path]
    for ident in list(ns._ident_roots):
        if not ident.startswith('semio_framework_os'): continue
        ew = ns.ext_world(ident)
        if ew and path in ew.files: return ew.files[path]
    raise KeyError(path)

def extern_def(segs):
    """Res of an extern path inside the registered crate worlds (def / mod), following re-exports; None when unknown"""
    w = ns.ext_world(segs[0])
    if w is None: return None
    cur = ns.Res('mod', mod=w.root)
    for i, s in enumerate(segs[1:]):
        if cur.kind != 'mod': return cur
        nxt = ns.lookup(cur.mod, s)
        if nxt is None: return None
        if nxt.kind == ns.EXTERN_GLOB:
            for c in nxt.extern:
                r = extern_def(c + list(segs[i + 2:]))
                if r is not None: return r
            return None
        if nxt.kind == 'extern': return extern_def(nxt.extern + list(segs[i + 2:]))
        cur = nxt
    return cur

def to_text(r, dest_ident, need_pub):
    if r.kind == 'extern': segs = list(r.extern)
    elif r.kind == ns.EXTERN_GLOB:
        cand = ns.disambiguate(r)
        if not cand: return None
        segs = list(cand[0])
    else: return None
    d = extern_def(segs[:6])
    if d is not None and d.kind == 'def': need_pub.add((d.mod.file, d.name))
    if segs[0] == dest_ident: segs = ['crate'] + segs[1:]
    return '::'.join(segs)

def build(ctx, text, dest_ident, label):
    """-> (rewritten text, import lines, violations, need_pub) for item text that moves out of module ctx"""
    toks = tokenize(text)
    sig = [i for i, t in enumerate(toks) if t[0] not in ('ws', 'comment')]
    n = len(sig); edits = []; violations = []; need_pub = set(); imports = {}
    defined = set()
    for it in items(text):
        if it['name'] and it['kind'] not in ('impl', 'use'): defined.add(it['name'])
    k = 0
    while k < n:
        i = sig[k]; kd, tx, a, b = toks[i]
        if kd != 'id': k += 1; continue
        prev = toks[sig[k - 1]][1] if k else ''
        lead = k >= 2 and toks[sig[k - 1]][1] == ':' and toks[sig[k - 2]][1] == ':'
        if prev == '.' or prev == "'" or (lead and not (tx in ('crate', 'super', 'self') or tx.startswith('semio_framework'))):
            k += 1; continue
        if lead:
            pre = toks[sig[k - 3]] if k >= 3 else None
            if pre is not None and (pre[0] == 'id' or pre[1] in ('>', ')', ']')): k += 1; continue
        segs = [tx]; j = k + 1; last = k
        while j + 2 <= n - 1 and toks[sig[j]][1] == ':' and toks[sig[j + 1]][1] == ':' and toks[sig[j + 2]][0] == 'id':
            segs.append(toks[sig[j + 2]][1]); last = j + 2; j += 3
        nxt = toks[sig[last + 1]][1] if last + 1 < n else ''
        if nxt == ':' and last + 3 < n and toks[sig[last + 2]][1] == ':' and toks[sig[last + 3]][1] == '<': nxt = '('
        if tx in KW and tx not in ('crate', 'super'): k = last + 1; continue
        if tx in ns._ident_roots or tx in ('std', 'core', 'alloc'): k = last + 1; continue
        if tx in ('crate', 'super', 'self') or tx in ('semio_framework_os_kernel',) or (tx[0].isupper() or nxt in ('(', '!') or len(segs) > 1):
            if tx in defined and tx not in ('crate', 'super', 'self'): k = last + 1; continue
            r = ns.resolve(ctx, segs) if (tx in ('crate', 'super', 'self') or len(segs) > 1 or True) else None
            if tx in ('crate', 'super', 'self'):
                text2 = to_text(r, dest_ident, need_pub)
                if text2 is None:
                    violations.append((label, '::'.join(segs), str(r)[:120]))
                else:
                    start = toks[sig[k - 2]][2] if lead else toks[sig[k]][2]
                    edits.append((start, toks[sig[last]][3], text2))
            else:
                head = ns.lookup(ctx, tx)
                if head is not None and head.kind == ns.EXTERN_GLOB and not ns.disambiguate(head): head = None
                if head is not None and head.kind in ('extern', ns.EXTERN_GLOB, 'def', 'mod'):
                    if head.kind == 'def' or head.kind == 'mod':
                        # a name defined in a kernel module: only legal when it moves too (checked by the caller) -> report
                        violations.append((label, tx, 'defined in ' + str(head)[:100]))
                    else:
                        t3 = to_text(head, dest_ident, need_pub)
                        if t3 is None: violations.append((label, tx, str(head)[:100]))
                        else: imports[tx] = t3
            k = last + 1; continue
        k += 1
    out = text
    for s, e, r in sorted(edits, reverse=True): out = out[:s] + r + out[e:]
    lines = []
    for nm, p in sorted(imports.items()):
        segs = p.split('::')
        lines.append('use ' + p + ('' if segs[-1] == nm else ' as ' + nm) + ';')
    return out, lines, violations, need_pub

def pick(src_text, pattern):
    rx = re.compile(pattern, re.S)
    out = []
    for it in items(src_text):
        code = re.sub(r'(?m)^\s*//.*$', '', it['text'])
        if rx.search(code): out.append(it)
    return out

def run(moves, mode):
    registry.load(refresh=True)
    ns.forget_worlds(); registry.load()
    groups = collections.OrderedDict()
    for mv in moves: groups.setdefault((mv['src'], mv['dest']), []).append(mv)
    plan = []; viol = []; pubs = set()
    srcs = {}
    for (src, dest), mvs in groups.items():
        ctx = module_of(src)
        text = srcs.get(src) or open(src, encoding='utf8').read()
        picked = []
        for mv in mvs:
            p = pick(text, mv['match'])
            assert len(p) >= 1, ('no item matches', mv['match'], src[-60:])
            if 'count' in mv: assert len(p) == mv['count'], (mv['match'], len(p), mv['count'])
            picked += p
        picked = sorted({it['start']: it for it in picked}.values(), key=lambda i: i['start'])
        MARKRX = re.compile(r'^\s*//#(end)?region\b.*$')
        def strip_lead(txt):
            ls = txt.split('\n'); out = []; lead = True
            for l in ls:
                if lead and MARKRX.match(l): continue
                if l.strip() and not l.strip().startswith('//'): lead = False
                out.append(l)
            return '\n'.join(out)
        def lead_markers(txt):
            out = []
            for l in txt.split('\n'):
                if MARKRX.match(l): out.append(l)
                elif l.strip() and not l.strip().startswith('//'): break
            return out
        body = '\n\n'.join(strip_lead(it['text']) for it in picked)
        dest_ident = ident_of(dest)
        new_body, lines, v, need = build(ctx, body, dest_ident, os.path.relpath(src, ns.R)[-60:])
        if any(mv.get('reexport') for mv in mvs):
            pubnames = {it['name'] for it in picked if it['kind'] in ('struct', 'enum', 'trait', 'type', 'fn', 'const') and it['name']}
            new_body, _done = widen.widen_items(new_body, pubnames)
        for mv in mvs:
            for a, b in mv.get('fixups', {}).items():
                new_body = new_body.replace(a, b); lines = [l.replace(a, b) for l in lines]
        need = {x for x in need if x[0] != '<none>'}
        viol += v; pubs |= need
        plan.append((src, dest, picked, new_body, lines))
        # cut from the source text, keeping region markers
        first = picked[0]['start']
        for it in sorted(picked, key=lambda i: -i['start']):
            keep = '\n'.join(lead_markers(it['text']))
            text = text[:it['start']] + (keep + '\n' if keep else '') + text[it['end']:]
        if any(mv.get('reexport') for mv in mvs):
            names = sorted({it['name'] for it in picked if it['kind'] in ('struct', 'enum', 'trait', 'type', 'fn', 'const')})
            sname0 = os.path.basename(os.path.dirname(src))
            mod0 = 'lifted_' + re.sub(r'[^a-z0-9]+', '_', sname0.lower().strip('_')).strip('_')
            text = text[:first] + 'pub use ' + dest_ident + '::' + mod0 + '::{' + ', '.join(names) + '};\n' + text[first:]
        srcs[src] = text
    for x in viol: print('VIOLATION', x)
    print('moves:', [(os.path.relpath(s, ns.R)[-40:], d, len(p)) for s, d, p, nb, l in plan])
    pubs = {(f, n) for f, n in pubs if f.startswith(ns.M + '/')}
    print('widen:', sorted((os.path.relpath(f, ns.R)[-50:], n) for f, n in pubs))
    if mode != 'write' or viol: return
    import apply as A
    model_ = C.load()[0]
    texts = {}
    LIT = re.compile(r'(#\[\s*path\s*=\s*"|include(?:_str|_bytes)?!\(\s*")([^"]+)"')
    for src, dest, picked, new_body, lines in plan:
        node = C.node_abs(dest, model_)
        sname = ctx_name = os.path.basename(os.path.dirname(src))
        path = node + '/📥️lifted/' + sname + '/🦀️.rs'
        def rebase(m):
            target = os.path.normpath(os.path.join(os.path.dirname(src), m.group(2)))
            return m.group(1) + os.path.relpath(target, os.path.dirname(path)) + '"'
        new_body = LIT.sub(rebase, new_body)
        modname = 'lifted_' + re.sub(r'[^a-z0-9]+', '_', sname.lower().strip('_')).strip('_')
        existing = texts.get(path) or (open(path, encoding='utf8').read() if os.path.exists(path) else None)
        header = f'//! 📥️ Trait impls of `{sname}` whose trait or type belongs to `{dest}` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `{os.path.relpath(src, ns.M)}`.\n'
        if existing is None:
            texts[path] = header + '\n'.join(lines) + '\n\n' + new_body.rstrip('\n') + '\n'
            root = node + '/🦀️.rs'
            rt = texts.get(root) or open(root, encoding='utf8').read()
            decl = f'#[path = "📥️lifted/{sname}/🦀️.rs"]\npub mod {modname};'
            if decl not in rt:
                i = 0; ls = rt.split('\n')
                while i < len(ls) and (ls[i].startswith('//!') or not ls[i].strip()): i += 1
                texts[root] = '\n'.join(ls[:i]) + '\n' + decl + '\n' + '\n'.join(ls[i:])
        else:
            have = set(re.findall(r'(?m)^use .*;$', existing))
            add = [l for l in lines if l not in have]
            body_ex = existing
            if new_body.strip() in existing: continue
            idx = max([m.end() for m in re.finditer(r'(?m)^use .*;$', existing)] or [existing.index('\n') + 1])
            texts[path] = existing[:idx] + ('\n' + '\n'.join(add) if add else '') + existing[idx:].rstrip('\n') + '\n\n' + new_body.rstrip('\n') + '\n'
    for src, text in srcs.items(): texts[src] = text
    for f, n in sorted(pubs):
        if not f.startswith(ns.M + '/'): continue
        base = texts.get(f) or open(f, encoding='utf8').read()
        base, done = widen.widen_items(base, {n})
        texts[f] = base
    for p, t in texts.items(): A.write(p, t)
    print('written', len(texts), 'files')

if __name__ == '__main__':
    import importlib
    mode, name = sys.argv[1], sys.argv[2]
    mod = importlib.import_module('moves_' + name)
    run(mod.MOVES, mode)
