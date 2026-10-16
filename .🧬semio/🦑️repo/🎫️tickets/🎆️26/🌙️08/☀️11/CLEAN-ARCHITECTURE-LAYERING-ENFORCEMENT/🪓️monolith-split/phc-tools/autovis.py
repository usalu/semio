#!/usr/bin/env python3
"""🔓️ Privacy repair from compiler-shaped evidence: widens the item, method, field or import named by the "defined here" note of an E0603/E0616/E0624/E0451/E0423 diagnostic to the narrowest visibility that reaches the use site."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, scan
from carve import REPO, inline_descendants

CODES = {'E0603', 'E0616', 'E0624', 'E0451', 'E0423'}


def norm(sandbox_name):
    p = os.path.normpath(os.path.join(REPO, sandbox_name)) if not sandbox_name.startswith('/') else sandbox_name
    return p


def defined_here(d):
    out = []
    for ch in d.get('children', []):
        if 'defined here' in ch['message'] or 'private field' in ' '.join((s.get('label') or '') for s in ch['spans']) or 'consider making the field' in ch['message'] or 'consider marking' in ch['message']:
            for s in ch['spans']:
                out.append((norm(s['file_name']), s['line_start'], ch['message'], s.get('label')))
    return out


def vis_text(same_crate):
    return 'pub(crate)' if same_crate else 'pub'


def replace_vis(src, header_first_tok, toks_after, new):
    """Replace the visibility in front of the item header (or insert one)."""
    t0 = header_first_tok
    if t0.x == 'pub' and t0.k == 'id':
        end = t0.b
        k = toks_after
        if k and k[0].x == '(':
            e = 0
            while k[e].x != ')':
                e += 1
            end = k[e].b
        return (t0.a, end, new)
    return (t0.a, t0.a, new + ' ')


def innermost(fm, line):
    best = None
    scopes = [fm] + list(inline_descendants(fm))
    for S in scopes:
        for it in S.all_items:
            a = it.toks_all[0].sl + 1
            b = it.toks_all[-1].el + 1
            if a <= line <= b:
                if best is None or (b - a) <= (best[1].toks_all[-1].el - best[1].toks_all[0].sl):
                    best = (S, it)
    return best


def plan(C, diags, roots):
    """Returns {file: {(a, b): replacement}} and the list of reasons."""
    patches = collections.defaultdict(dict)
    why = []
    for d in diags:
        code = (d.get('code') or {}).get('code')
        if code not in CODES:
            continue
        prim = next((s for s in d['spans'] if s.get('is_primary')), None)
        use_file = norm(prim['file_name']) if prim else None
        use_crate = C.new.crate_of_file.get(use_file)
        for dfile, line, msg, label in defined_here(d):
            fm = C.new.by_file.get(dfile)
            if fm is None:
                continue
            def_crate = C.new.crate_of_file.get(dfile)
            vis = vis_text(use_crate == def_crate)
            hit = innermost(fm, line)
            if hit is None:
                continue
            S, it = hit
            src = fm.src
            if code == 'E0603':
                if it.kind == 'mod' and it.item_is_decl if False else False:
                    continue
                toks = it.toks
                span = replace_vis(src, toks[0], toks[1:], vis)
                if it.vis == 'pub' and vis == 'pub(crate)':
                    continue
                patches[dfile][(span[0], span[1])] = span[2]
                why.append(dict(code=code, file=os.path.relpath(dfile, REPO), kind=it.kind, name=it.name if it.kind != 'use' else ','.join(sorted(u.binding for u in S.uses if u.item is it)), owner='', vis=vis))
            elif code in ('E0624',) and it.kind == 'impl':
                for nm, v, vs, ve, off in scan.impl_methods(it):
                    ln = fm.src.count('\n', 0, off) + 1
                    if ln == line and v != 'pub':
                        patches[dfile][(vs, ve)] = vis if vs != ve else vis + ' '
                        why.append(dict(code=code, file=os.path.relpath(dfile, REPO), kind='method', name=nm, owner=carve.impl_self_ident(it) or '', vis=vis))
            elif code in ('E0616', 'E0451', 'E0423') and it.kind == 'struct':
                for nm, v, vs, ve, no in scan.struct_members(it):
                    ln = fm.src.count('\n', 0, no) + 1
                    if ln == line and v != 'pub':
                        patches[dfile][(vs, ve)] = vis if vs != ve else vis + ' '
                        why.append(dict(code=code, file=os.path.relpath(dfile, REPO), kind='field', name=nm, owner=it.name, vis=vis))
    return patches, why


def apply(ws, patches):
    n = 0
    for f, ps in patches.items():
        src = ws.read(f)
        for (a, b), r in sorted(ps.items(), key=lambda x: -x[0][0]):
            src = src[:a] + r + src[b:]
            n += 1
        ws.write(f, src)
    return n


DECISIONS = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'vis-decisions.json')


def load_decisions():
    import json
    return json.load(open(DECISIONS, encoding='utf-8')) if os.path.exists(DECISIONS) else []


def save_decisions(decisions):
    import json
    seen = set()
    out = []
    for d in decisions:
        k = (d['file'], d['kind'], d['name'], d['owner'])
        if k not in seen:
            seen.add(k)
            out.append({x: d[x] for x in ('file', 'kind', 'name', 'owner', 'vis')})
    json.dump(sorted(out, key=lambda d: (d['file'], d['kind'], d['owner'], d['name'])), open(DECISIONS, 'w', encoding='utf-8'), ensure_ascii=False, indent=1)


def apply_decisions(ws, C, decisions):
    """Re-applies recorded widenings by (file, kind, owner, name); returns the number applied and the unmatched ones."""
    patches = collections.defaultdict(dict)
    miss = []
    for d in decisions:
        fm = C.new.by_file.get(os.path.join(REPO, d['file']))
        if fm is None:
            miss.append(d)
            continue
        found = False
        for S in [fm] + list(inline_descendants(fm)):
            for it in S.all_items:
                if d['kind'] == 'method' and it.kind == 'impl' and (carve.impl_self_ident(it) or '') == d['owner']:
                    for nm, v, vs, ve, off in scan.impl_methods(it):
                        if nm == d['name'] and v != 'pub' and v != d['vis']:
                            patches[fm.file][(vs, ve)] = d['vis'] if vs != ve else d['vis'] + ' '
                            found = True
                elif d['kind'] == 'field' and it.kind == 'struct' and it.name == d['owner']:
                    for nm, v, vs, ve, no in scan.struct_members(it):
                        if nm == d['name'] and v != 'pub' and v != d['vis']:
                            patches[fm.file][(vs, ve)] = d['vis'] if vs != ve else d['vis'] + ' '
                            found = True
                elif d['kind'] not in ('method', 'field') and it.kind == d['kind'] and (it.name == d['name'] or (it.kind == 'use' and ','.join(sorted(u.binding for u in S.uses if u.item is it)) == d['name'])):
                    if it.vis == 'pub' or it.vis == d['vis']:
                        found = True
                        continue
                    toks = it.toks
                    span = replace_vis(fm.src, toks[0], toks[1:], d['vis'])
                    patches[fm.file][(span[0], span[1])] = span[2]
                    found = True
        if not found:
            miss.append(d)
    return apply(ws, patches), miss
