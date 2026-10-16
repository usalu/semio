#!/usr/bin/env python3
# 🔬️ Union differential of PLUGIN-WP2: sandbox A = live plugin tree + the carved WP2 files (OUT) with the seven crates mounted as modules of ONE crate
# (`crate::` -> `crate::pl_<crate>::`, `pub(crate)` -> `pub(in crate::pl_<crate>)`, `semio_framework_plugin_<k>::` -> `crate::pl_<k>::`, hub units `extern crate` -> `use`);
# sandbox B = the live tree as it was before the carve (the repository files are not changed until `apply.py`). Compares the error multisets (patched oracle of 📓️RESOLUTION-ORACLE.md).
# usage: uo.py <tag> [--configs lib,test,wasip2,lib-features,test-features] [--seed name] [--identity]
import collections, json, os, re, shutil, subprocess, sys, importlib.util, time
ARGS = list(sys.argv[1:])
from common import *
import model as M
import engine as E
import hubgen
import modtree
from rs import blank

spec = importlib.util.spec_from_file_location('pwp0oracle', SPLIT + '/pwp0-tools/oracle.py')
O = importlib.util.module_from_spec(spec)
sys.path.insert(0, SPLIT + '/pwp0-tools')
spec.loader.exec_module(O)
O.GEN = WS
OUT = WS + '/out'
LIBKEY = {M.libname(k): k for k in M.CRATES_JSON}
PL = lambda k: 'pl_' + k.replace('-', '_')

def guard(sb, path):
    real = os.path.realpath(path)
    assert real.startswith(os.path.realpath(sb) + '/'), (sb, path)

def sandbox_plug(sb):
    return sb + '/' + O.PLUG

def mapping(text, k):
    """resolver output (same-crate `crate::`, cross-crate `semio_framework_plugin_X::`) -> union module paths"""
    clean = blank(text)
    out, last = [], 0
    for m in re.finditer(r'(?<![A-Za-z0-9_$:])crate::', clean):
        out.append(text[last:m.start()]); out.append('crate::' + PL(k) + '::'); last = m.end()
    out.append(text[last:])
    t = ''.join(out)
    t = re.sub(r'\bpub\(crate\)', 'pub(in crate::component::' + PL(k) + ')', t)
    return t

def crossmap(text):
    def sub(m):
        k = LIBKEY.get(m.group(1))
        return 'crate::' + PL(k) + '::' if k else m.group(0)
    return re.sub(r'(?<![A-Za-z0-9_$:])(semio_framework_plugin_[a-z0-9_]+)::', sub, text)

def externs_to_use(text):
    return re.sub(r'(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(semio_framework_plugin_[a-z0-9_]+)\s+as\s+(\w+)\s*;', lambda m: f"{m.group(1) or ''}use crate::{PL(LIBKEY[m.group(2)])} as {m.group(3)};" if m.group(2) in LIBKEY else m.group(0), text)

def synth_module(x, lifts):
    """pl_<x> for a crate outside WP2: its still-monolithic modules, the root glob, the items the plan lifts into it and the lift files this tool wrote for it"""
    lines = [f'pub mod {PL(x)} {{']
    root_mod = next((n for n, f in M.MOD_FILE.items() if M.MOD_CRATE.get(n) == x and M.ROOT.get(x) == f), None)
    for n, c in sorted(M.MOD_CRATE.items()):
        if c == x and M.MOD_FILE.get(n) and not M.MOD_FILE[n].startswith(('🧪️', '📣️')) and c not in M.WP2:
            lines.append(f'    #[allow(unused_imports)] pub(crate) use crate::{n};')
            if n == root_mod:
                lines.append(f'    #[allow(unused_imports)] pub(crate) use crate::{n}::*;')
    seen = set()
    for name, rows in sorted(E.HOMES.by_name.items()):
        for r in rows:
            if r['dst'] == x and r['src'] != x and r['kind'] != 'impl' and r['src_mod'] and not r['cut'] and r['src'] not in M.WP2 and (r['src_mod'], name) not in seen:
                seen.add((r['src_mod'], name))
                lines.append(f'    #[allow(unused_imports)] pub(crate) use crate::{r["src_mod"]}::{name};')
    for rel, mod in lifts:
        lines.append(f'    #[allow(unused_imports)] pub use crate::pl_{mod}::*;')
    lines.append('}')
    return '\n'.join(lines)

INCOMING_OLD = {'owned_context_fields': 'owned_context'}

def simulate_delivery(text, k, plug):
    """incoming lift modules whose stub is empty are mounted as re-exports of the old monolith module (what the owner's lift will contain)"""
    for mod, rel, st in M.LIFTS_IN.get(k, []):
        if st != 'incoming' or (os.path.exists(plug + '/' + rel) and E.has_body(read(plug + '/' + rel))):
            continue
        root_dir = os.path.dirname(M.ROOT[k])
        p = os.path.relpath(rel, root_dir)
        pat = re.compile(r'#\[path = "' + re.escape(p) + r'"\]\npub mod ' + mod + ';')
        old = INCOMING_OLD.get(mod, mod)
        text = pat.sub('pub mod ' + mod + ' { #[allow(unused_imports)] pub(crate) use crate::' + old + '::*; }', text)
    return text

def hub_root_files(plug):
    out = [plug + '/🦀️.rs']
    d = plug + '/🪝️mounts'
    if os.path.isdir(d):
        out += [d + '/' + h + '/🦀️.rs' for h in sorted(os.listdir(d))]
    return out

def flipped(k):
    """a crate is a real crate root once its manifest `[lib] path` points at `<root>/🦀️.rs` and the root is more than a stub"""
    pkg = PLUG + '/' + M.CRATES_JSON[k]['pkg'].split('/🔌️plugin/')[1] + '/Cargo.toml'
    if not os.path.exists(pkg):
        return False
    m = re.search(r'(?m)^\[lib\]\s*\npath\s*=\s*"([^"]+)"', read(pkg))
    if not m or m.group(1) != '../../🦀️.rs':
        return False
    root = PLUG + '/' + M.ROOT[k]
    return os.path.exists(root) and len([l for l in read(root).split('\n') if l.strip() and not l.startswith('//!') and not l.startswith('extern crate')]) > 0

def carved_in(plug, mine, wp2_live=False):
    found = {k for k in M.CRATES_JSON if k not in M.WP2 and flipped(k)}
    if mine:
        found |= set(M.WP2)
    elif wp2_live:
        found |= {k for k in M.WP2 if flipped(k)}
    return found

def apply_union(sb, mine, wp2_live=False):
    plug = sandbox_plug(sb)
    carved = carved_in(plug, mine, wp2_live)
    owners = {}
    roots = {}
    for k in sorted(carved):
        root = plug + '/' + M.ROOT[k]
        if not os.path.exists(root):
            continue
        txt = read(root)
        if not mine and k not in M.WP2 and len(txt.strip().split('\n')) <= 4 and '🧭️ WP' in txt:
            continue
        roots[k] = root
        for f in modtree.tree(root):
            owners.setdefault(f, k)
    for f, k in owners.items():
        guard(sb, f)
        t = read(f)
        rel = os.path.relpath(f, plug)
        if k in M.WP2 and not os.path.exists(OUT + '/' + rel):
            t = E.rewrite_paths(E.expand_group_uses(t), k)
        t = mapping(t, k)
        if f == roots[k]:
            t = re.sub(r'(?m)^#!\[(?:feature|allow)\([^\]]*\)\]\s*$', '', t)
            if k in M.WP2:
                t = simulate_delivery(t, k, plug)
        write(f, t)
    return carved, roots, owners

def prepare(sb, mine, wp2_live=False):
    plug = sandbox_plug(sb)
    root_text = None
    if mine:
        for dp, dn, fn in os.walk(OUT):
            for f in fn:
                src = os.path.join(dp, f)
                rel = os.path.relpath(src, OUT)
                dst = plug + '/' + rel
                guard(sb, dst)
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copy(src, dst)
        hub = hubgen.build_hub()
        hub = hub.replace('//#endregion 📜️DeclarationFacet', hubgen.extra_units() + '\n//#endregion 📜️DeclarationFacet', 1)
        write(plug + '/' + hubgen.HUB_REL, hub)
        write(plug + '/🦀️.rs', hubgen.root_edit())
        rr = read(plug + '/⚛️reactor/🦀️.rs')
        old = '#[path = "🪟️surfaces/🦀️.rs"]\npub(crate) mod surface_context;\n'
        assert rr.count(old) == 1
        write(plug + '/⚛️reactor/🦀️.rs', rr.replace(old, 'pub(crate) use semio_framework_plugin_plugin_program::surface_context;\n'))
    if mine:
        import flips
        for dp, dn, fn in os.walk(plug):
            dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
            for fl in fn:
                if fl.endswith('.rs'):
                    p = os.path.join(dp, fl)
                    rel = os.path.relpath(p, plug)
                    t = read(p)
                    if '🛬️lifted' in t:
                        n, hits = flips.flip(t, rel, True)
                        if hits:
                            guard(sb, p)
                            write(p, n)
    carved, roots, owners = apply_union(sb, mine, wp2_live)
    lifts = collections.defaultdict(list)
    if mine or wp2_live:
        for c in M.CUTS:
            if c['to'] not in M.WP2:
                f = plug + '/' + c['lift']
                if f not in [x for x, _ in lifts[c['to']]]:
                    lifts[c['to']].append((f, 'lift_' + c['to'].replace('-', '_') + '_' + str(len(lifts[c['to']]))))
        for x, lst in lifts.items():
            for f, m in lst:
                guard(sb, f)
    # cross-crate path prefixes and extern crate items, everywhere
    for dp, dn, fn in os.walk(plug):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for fl in fn:
            if fl.endswith('.rs'):
                p = os.path.join(dp, fl)
                t = read(p)
                if 'semio_framework_plugin_' in t:
                    n = crossmap(externs_to_use(t))
                    if n != t:
                        guard(sb, p)
                        write(p, n)
    mounts = ''
    for x, lst in lifts.items():
        for f, m in lst:
            mounts += f'#[path = "{os.path.relpath(f, plug)}"]\npub mod pl_{m};\n'
    for k, root in sorted(roots.items()):
        mounts += f'#[path = "{M.ROOT[k]}"]\npub mod {PL(k)};\n'
    ref = set()
    for dp, dn, fn in os.walk(plug):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target')]
        for fl in fn:
            if fl.endswith('.rs'):
                for m in re.finditer(r'crate::pl_([a-z0-9_]+)::', read(os.path.join(dp, fl))):
                    ref.add(m.group(1))
    for k in sorted(M.CRATES_JSON):
        if PL(k)[3:] in ref and k not in roots:
            mounts += synth_module(k, [(os.path.relpath(f, plug), m) for f, m in lifts.get(k, [])]) + '\n'
    cur = read(plug + '/🦀️.rs')
    write(plug + '/🦀️.rs', cur.rstrip('\n') + '\n' + mounts)
    O.neutralize(plug)
    os.makedirs(sb + '/outdir', exist_ok=True)

def main():
    args = ARGS
    tag = args[0]
    configs = ['lib', 'test', 'wasip2', 'lib-features', 'test-features']
    identity = '--identity' in args
    for i, a in enumerate(args):
        if a == '--configs':
            configs = args[i + 1].split(',')
    base = WS + '/sb-' + tag
    A, B = base + '/A', base + '/B'
    os.makedirs(base, exist_ok=True)
    post = '--post' in args
    O.neutralize_after = None
    E.set_input(WS + '/in/' + O.PLUG)
    for sb in (A, B):
        shutil.rmtree(sb, ignore_errors=True)
    subprocess.check_call(['cp', '-R', WS + '/in', B])
    if post:
        O.chain(A)
        prepare(B, False)
        prepare(A, False, True)
    else:
        subprocess.check_call(['cp', '-R', WS + '/in', A])
        prepare(B, False)
        prepare(A, not identity)
    seeds = [a.split(':', 1)[1] for a in args if a.startswith('seed:')]
    SEEDS = {
        'pub': ('📜️declaration/👁️view/🦀️.rs', 'pub fn cancellation_action_definition', 'fn cancellation_action_definition'),
        'import': ('📜️declaration/👁️view/🦀️.rs', 'use semio_framework_ui_locale::LocalizedLabel;', ''),
        'path': ('📜️declaration/🧩️app/🦀️.rs', '#[path = "🛬️lifted/⚠️fault/🦀️.rs"]', '#[path = "🛬️lifted/📛️absent/🦀️.rs"]'),
    }
    for name in seeds:
        rel, old, new_ = SEEDS[name]
        pth = A + '/' + O.PLUG + '/' + rel
        s = read(pth)
        assert s.count(old) == 1, (name, s.count(old))
        guard(A, pth)
        write(pth, s.replace(old, new_))
        print('seeded', name, flush=True)
    summary = {}
    for name in configs:
        ea, ta = O.run(A, 'A-' + name, O.CFGS[name], 'w2' + tag)
        eb, tb = O.run(B, 'B-' + name, O.CFGS[name], 'w2' + tag)
        a, b = O.parse(ea, A), O.parse(eb, B)
        fa, fb = collections.Counter(), collections.Counter()
        for k, v in a.items():
            fa[k[:4]] += v
        for k, v in b.items():
            fb[k[:4]] += v
        new, old = fa - fb, fb - fa
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()))
        print(f'== {name}: A(carved) errors={sum(a.values())} B(live) errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}', flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:60]:
            print('  +new', v, k)
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:25]:
            print('  -old', v, k)
    json.dump(summary, open(f'{GEN}/pwp2-oracle-{tag}-summary.json', 'w'), indent=1)

if __name__ == '__main__':
    main()
