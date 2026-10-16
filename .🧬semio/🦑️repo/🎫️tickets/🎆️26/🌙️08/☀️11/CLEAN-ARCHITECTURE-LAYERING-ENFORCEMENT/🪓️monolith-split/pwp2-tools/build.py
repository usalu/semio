#!/usr/bin/env python3
# 🏗️ PLUGIN-WP2 builder: from a consistent copy of the live plugin tree (WS/in, the same base the oracle uses) and the model, writes the carved tree into OUT (mirror of the plugin folder) plus edits.json.
# Never touches the repository. Idempotent.
import re, os, sys, json, collections, shutil, importlib.util
ARGS = list(sys.argv[1:])
from common import *
import plan_tools as pt
import model as M
import engine as E
from rs import blank, item_names, uses

spec = importlib.util.spec_from_file_location('pwp0oracle', SPLIT + '/pwp0-tools/oracle.py')
O = importlib.util.module_from_spec(spec)
sys.path.insert(0, SPLIT + '/pwp0-tools')
spec.loader.exec_module(O)

OUT = WS + '/out'
IN = WS + '/in'
LOG = []
VIOLATIONS = []
PART = {k: set(v['deps']) for k, v in M.CRATES_JSON.items()}
RANK = {k: v['rank'] for k, v in M.CRATES_JSON.items()}
LIBKEY = {M.libname(k): k for k in M.CRATES_JSON}

def log(*a):
    LOG.append(' '.join(str(x) for x in a))

def doc_head(text):
    lines = text.split('\n')
    i = 0
    while i < len(lines) and lines[i].startswith('//!'):
        i += 1
    return '\n'.join(lines[:i]), '\n'.join(lines[i:])

def stub_externs(key):
    c = M.CRATES_JSON[key]
    p = PLUG + '/' + c['pkg'].split('/🔌️plugin/')[1] + '/🦀️.rs'
    t = read(p) if os.path.exists(p) else ''
    return re.findall(r'extern crate semio_framework_os_kernel as (\w+);', t)

OWN_DIRS = ('📜️declaration/', '🏗️builder/🧩️app/', '🏗️builder/🔌️program/', '⚛️reactor/🪟️surfaces/', '🫧️transient/', '🎚️config/', '📝️draft/', '👥️presence/', '🧵️task/')

def owned_files(key):
    out = [M.ROOT[key]] + list(M.CHILDREN[key].values())
    import modtree
    host_tests = {h['test'] for h in M.TEST_HOSTS}
    seen = set(out)
    for rel in list(out):
        for f in modtree.tree(IN + '/' + O.PLUG + '/' + rel):
            r = os.path.relpath(f, IN + '/' + O.PLUG)
            if r in seen or r in host_tests or '🛬️lifted' in r or not r.startswith(OWN_DIRS):
                continue
            seen.add(r)
            out.append(r)
    return out

def check_deps(cur, rel, text):
    clean = blank(text)
    bad = collections.Counter()
    for m in re.finditer(r'(?<![A-Za-z0-9_])(semio_framework_plugin_[a-z0-9_]+)::', clean):
        k = LIBKEY.get(m.group(1))
        if k and k != cur:
            if RANK[k] >= RANK[cur]:
                bad[(k, 'UPWARD')] += 1
            elif k not in PART[cur]:
                bad[(k, 'added-edge')] += 1
    for (k, why), n in bad.items():
        VIOLATIONS.append((cur, rel, k + ' ' + why, n))

def prune_upward(text, cur):
    clean = blank(text)
    removed = []
    for m in re.finditer(r'(?m)^((?:#\[[^\]\n]*\][ \t]*\n?)*)(pub(?:\([^)]*\))?\s+)?use\s+(semio_framework_plugin_[a-z0-9_]+)::([^;]*);[ \t]*\n', clean):
        k = LIBKEY.get(m.group(3))
        if not k or k == cur:
            continue
        leaves = list(__import__('rs').expand_use(m.group(3) + '::' + m.group(4)))
        names = [(a or p[-1]) for p, a, g in leaves if not g]
        rest = clean[:m.start()] + clean[m.end():]
        used = [n for n in names if re.search(r'(?<![A-Za-z0-9_$])' + re.escape(n) + r'(?![A-Za-z0-9_])', rest)]
        if not used and (RANK[k] >= RANK[cur] or k not in PART[cur]):
            removed.append((m.start(), m.end(), names))
    for s, e, names in reversed(removed):
        text = text[:s] + text[e:]
    return text, [n for _, _, n in removed]

def known_locals(key, root_text):
    mods = set(M.CHILDREN[key]) | {m for m, _, _ in M.LIFTS_IN.get(key, [])}
    items = set(item_names(root_text))
    for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]*?(?:as\s+)?(\w+)\s*;', blank(root_text)):
        items.add(m.group(1))
    return mods | items | {'store', 'protocol', 'dsl', 'vcs'}

def prune_unknown_local(text, key, known):
    clean = blank(text)
    removed = []
    for m in re.finditer(r'(?m)^((?:#\[[^\]\n]*\][ \t]*\n?)*)(pub(?:\([^)]*\))?\s+)?use\s+crate::(\w+)([^;]*);[ \t]*\n', clean):
        if m.group(3) in known:
            continue
        leaves = list(__import__('rs').expand_use('crate::' + m.group(3) + m.group(4)))
        names = [(a or p[-1]) for p, a, g in leaves if not g]
        rest = clean[:m.start()] + clean[m.end():]
        used = [n for n in names if re.search(r'(?<![A-Za-z0-9_$])' + re.escape(n) + r'(?![A-Za-z0-9_])', rest)]
        if not used:
            removed.append((m.start(), m.end(), names))
    for s, e, names in reversed(removed):
        text = text[:s] + text[e:]
    return text, [n for _, _, n in removed]

def lift_children(rel):
    text = E.src_text(rel)
    base = os.path.dirname(rel)
    out = []
    for m in list(re.finditer(r'#\[path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;', text)) + list(re.finditer(r'include!\("([^"]+)"\)', text)):
        c = os.path.normpath(os.path.join(base, m.group(1)))
        if '🛬️lifted' in c and os.path.exists(IN + '/' + O.PLUG + '/' + c):
            out.append(c)
            out += lift_children(c)
    return out

def aliases_for(key, texts):
    base = set(stub_externs(key))
    blob = '\n'.join(blank(t) for t in texts)
    for a in ('dsl', 'protocol', 'store', 'vcs'):
        if re.search(r'(?<![A-Za-z0-9_$])' + a + r'::', blob) or re.search(r'use crate::\{[^}]*\b' + a + r'\b', blob):
            base.add(a)
    return sorted(base)

ATTRS = {'decl-app': ['#![feature(associated_type_defaults)]', '#![allow(async_fn_in_trait)]'], 'decl-surface': ['#![feature(associated_type_defaults)]', '#![allow(async_fn_in_trait)]'], 'decl-registry': ['#![allow(async_fn_in_trait)]'], 'plugin-program': ['#![allow(async_fn_in_trait)]']}

def mount_lines(key, real_lifts):
    root_dir = os.path.dirname(M.ROOT[key])
    out = []
    for mod, rel in M.CHILDREN[key].items():
        out += ['#[doc(hidden)]', f'#[path = "{os.path.relpath(rel, root_dir)}"]', f'pub mod {mod};']
    for mod, rel, st in M.LIFTS_IN.get(key, []):
        out += ['#[doc(hidden)]', f'#[path = "{os.path.relpath(rel, root_dir)}"]', f'pub mod {mod};']
        if st == 'incoming':
            out += ['#[allow(unused_imports)]', f'pub use {mod}::*;']
    return out

def main():
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(OUT)
    shutil.rmtree(IN, ignore_errors=True)
    O.chain(IN)
    E.set_input(IN + '/' + O.PLUG)
    new = {}
    # the unit table is frozen (plan time): the files cut from must still be byte-identical to the snapshot
    by_file = collections.defaultdict(list)
    for c in M.CUTS:
        by_file[c['file']].append(c)
    for f in by_file:
        assert E.src_text(f) == read(SNAP + '/' + f), ('source file drifted since the unit table', f)
    cut_out = collections.defaultdict(list)
    remaining = {}
    for f, cs in by_file.items():
        t = E.src_text(f)
        spans, per = [], []
        for c in cs:
            rows = E.unit_rows_for(f, set(c['names']))
            assert {r['name'] for r in rows} >= set(c['names']), (f, c['names'])
            sp = E.spans_of(rows, f)
            spans += sp
            per.append((c, sp))
        rem, _ = E.cut_text(t, sorted(set(spans)))
        for c in cs:
            for anchor, _, _, _ in c.get('tests', []):
                assert rem.count(anchor) == 1, (f, anchor)
                rem = rem.replace('\n' + anchor if ('\n' + anchor) in rem else anchor, '')
            if c.get('include'):
                line = f'include!("{c["include"][0]}");'
                assert rem.count(line) == 1, (f, line)
                rem = rem.replace(line + '\n', '')
        remaining[f] = rem
        lines = t.split('\n')
        for c, sp in per:
            cut_out[(c['to'], c['module'], c.get('lift'))].append((f, ['\n'.join(lines[s - 1:e]) for s, e in sp]))
        log('cut', f, [(c['to'], c['names']) for c in cs])
    for f, lines in M.DROP_INCLUDES.items():
        tx = remaining.get(f) or E.src_text(f)
        for l in lines:
            assert tx.count(l) == 1, ('drop include', f, l)
            tx = tx.replace(l, '')
        remaining[f] = tx
    hosts = []
    host_files = {}
    for h in M.TEST_HOSTS:
        if h.get('file'):
            tx = remaining.get(h['file']) or E.src_text(h['file'])
            assert tx.count(h['remove']) == 1, ('test host anchor', h['file'], h['host'])
            docm = re.search(r'((?:^///[^\n]*\n)+)' + re.escape(h['remove']), tx, re.M)
            h['doc'] = [l.replace('///', '//!', 1) for l in docm.group(1).rstrip('\n').split('\n')] if docm else []
            remaining[h['file']] = tx.replace((docm.group(1) if docm else '') + h['remove'], '')
            mod_src = E.src_text(h['file'])
        else:
            mod_src = E.src_text(h['lift'])
        if h.get('uses_from'):
            mod_src = E.src_text(h['uses_from'])
        use_lines = []
        for m in re.finditer(r'(?m)^((?:#\[[^\]\n]*\][ \t]*\n)*)(?!pub)(use[ \t]+[^;]*;)[ \t]*$', mod_src):
            use_lines.append((m.group(1) + m.group(2)).rstrip())
        body = ['//! ' + h['emoji'] + ' Test host of `' + h['module'] + '`: runs `' + os.path.basename(os.path.dirname(h['test'])) + '` in the facade, where the names it uses are in scope.'] + h.get('doc', []) + ['']
        body += use_lines
        body += ['#[allow(unused_imports)]', 'use crate::' + h['module'] + '::*;']
        up = '../../../../'
        if h['kind'] == 'include':
            body.append('include!("' + up + h['test'] + '");')
        else:
            body += ['#[path = "' + up + h['test'] + '"]', 'pub(crate) mod original_tests;']
            for fn in h.get('reexport', []):
                body.append('pub(crate) use original_tests::' + fn + ';')
        rel = M.HOSTS_DIR + '/' + h['emoji'] + h['slug'] + '/🦀️.rs'
        host_files[rel] = '\n'.join(body) + '\n'
        hosts.append(dict(h, uses=use_lines, rel=rel))
    json.dump(hosts, open(WS + '/hosts.json', 'w'), ensure_ascii=False, indent=1)
    macro_text = []
    for f in ('📜️declaration/📨️emission/🦀️.rs', '📜️declaration/👷️job/🦀️.rs'):
        t = remaining.get(f) or E.src_text(f)
        tmp = WS + '/tmp-outline.rs'
        write(tmp, t)
        its, L, R = pt.outline(tmp)
        spans = [(it['s'], it['e']) for it in its if it['kind'] == 'macro_rules!']
        rem, cuts = E.cut_text(t, spans)
        remaining[f] = rem
        macro_text += cuts
        log('macros', f, len(cuts))
    for (to, module, lift), parts in cut_out.items():
        rel = next(r for m, r, st in M.LIFTS_IN[to] if m == module) if to in M.WP2 else lift
        stub = E.src_text(rel) if os.path.exists(IN + '/' + O.PLUG + '/' + rel) else ''
        head, _ = doc_head(stub)
        items, hdr = [], []
        inc = next((c['include'] for c in M.CUTS if c['to'] == to and c['module'] == module and c.get('include')), None)
        tst = next((c['tests'] for c in M.CUTS if c['to'] == to and c['module'] == module and c.get('tests')), [])
        for f, texts in parts:
            aft = next((c.get('after', []) for c in M.CUTS if c['to'] == to and c['module'] == module and c.get('after')), [])
            extra = ([E.src_text(inc[1])] if inc else []) + [E.src_text(r) for _, r, _, _ in tst] + [E.src_text(r) for r, _ in aft] if f == next(c['file'] for c in M.CUTS if c['to'] == to and c['module'] == module) else []
            hdr += E.import_header(texts + extra, f, remaining[f], to)
            items += [E.widen_text(x) for x in texts]
        if inc:
            items.append(f'include!("{inc[2]}");')
        for c in M.CUTS:
            if c['to'] == to and c['module'] == module:
                for _, newpath in c.get('after', []):
                    items.append(f'include!("{newpath}");')
        for _, _, newpath, mod in tst:
            items.append(f'#[cfg(test)]\n#[path = "{newpath}"]\nmod {mod};')
        seen, hl = set(), []
        for h in hdr:
            if h not in seen:
                seen.add(h); hl.append(h)
        new[rel] = ('lift', to, (head + '\n' if head else '') + '\n'.join(hl) + '\n\n' + '\n\n'.join(items) + '\n')
    texts_by_crate = collections.defaultdict(list)
    for key in M.WP2:
        for rel in owned_files(key):
            t = remaining.get(rel) or E.src_text(rel)
            rep = []
            t = E.rewrite_paths(E.expand_group_uses(t), key, rep)
            t, dropped = prune_upward(t, key)
            if dropped:
                log('pruned-upward-imports', key, rel, dropped)
            new[rel] = ('file', key, t)
            texts_by_crate[key].append(t)
            log('rewrite', key, rel, len(rep))
    # lifts delivered by their source owners: made crate-correct in place (the destination owns them from the wiring on)
    real_lifts = set()
    for key, lst in M.LIFTS_IN.items():
        for mod, rel, st in lst:
            if st == 'incoming' and os.path.exists(IN + '/' + O.PLUG + '/' + rel) and E.has_body(E.src_text(rel)):
                real_lifts.add(rel)
                t = E.transform_lift(rel, key)
                for h in M.TEST_HOSTS:
                    if h.get('lift') == rel:
                        assert t.count(h['remove']) == 1, ('test host anchor in lift', rel)
                        t = t.replace(h['remove'], '')
                t, dropped = prune_upward(t, key)
                new[rel] = ('lift-in', key, t)
                texts_by_crate[key].append(t)
                log('lift-in', key, mod, rel)
    for rel in sorted(real_lifts):
        key = next(k for k, l in M.LIFTS_IN.items() for m, r, st in l if r == rel)
        for c in lift_children(rel):
            if c not in new:
                t = E.widen_text(E.rewrite_paths(E.expand_group_uses(E.src_text(c)), key))
                new[c] = ('lift-child', key, t)
                texts_by_crate[key].append(t)
                log('lift-child', key, c)
    for rel, (kind, to, text) in list(new.items()):
        if kind == 'lift':
            text = E.rewrite_paths(E.expand_group_uses(text), to)
            new[rel] = ('lift', to, text)
            if to in M.WP2:
                texts_by_crate[to].append(text)
    for key in M.WP2:
        rel = M.ROOT[key]
        kind, _, t = new[rel]
        head, rest = doc_head(t)
        aliases = aliases_for(key, texts_by_crate[key])
        known = known_locals(key, t)
        for r2, (k2, to2, tx2) in list(new.items()):
            if to2 == key and k2 in ('file', 'lift', 'lift-in', 'lift-child'):
                tx3, dropped = prune_unknown_local(tx2, key, known)
                if dropped:
                    log('pruned-unknown-local-imports', key, r2, dropped)
                    new[r2] = (k2, to2, tx3)
        kind, _, t = new[rel]
        head, rest = doc_head(t)
        hdr = '\n'.join(ATTRS.get(key, []) + [f'extern crate semio_framework_os_kernel as {a};' for a in aliases] + [''] + mount_lines(key, real_lifts))
        new[rel] = ('file', key, head + '\n' + hdr + '\n' + rest)
    for rel, (kind, to, text) in new.items():
        if to in M.WP2:
            check_deps(to, rel, text)
        write(OUT + '/' + rel, text)
    for rel, text in host_files.items():
        write(OUT + '/' + rel, text)
    mrel = '📣️macros/📜️wp2-declaration/🦀️.rs'
    mt = E.src_text(mrel)
    write(OUT + '/' + mrel, mt.rstrip('\n') + '\n\n' + '\n\n'.join(macro_text) + '\n')
    json.dump(dict(real_lifts=sorted(real_lifts), stub_lifts=[r for k, l in M.LIFTS_IN.items() for m, r, st in l if st == 'incoming' and r not in real_lifts]), open(WS + '/lifts.json', 'w'), ensure_ascii=False, indent=1)
    write(WS + '/build.log', '\n'.join(LOG))
    write(WS + '/violations.txt', '\n'.join(f'{a} {b} -> {c} x{d}' for a, b, c, d in VIOLATIONS))
    print(len(new), 'files written to', OUT, '; real incoming lifts:', len(real_lifts), '; upward/undeclared references:', len(VIOLATIONS))

if __name__ == '__main__':
    main()
