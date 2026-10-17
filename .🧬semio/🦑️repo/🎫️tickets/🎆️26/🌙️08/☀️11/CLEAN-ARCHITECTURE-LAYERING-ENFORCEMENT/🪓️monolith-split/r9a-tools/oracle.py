#!/usr/bin/env python3
"""Scoped resolution differential for the bim model artifact (analysis only, output to /dev/null).
usage: oracle.py base [cfgs]            extract `git archive HEAD` of the artifact (or R9A_BASE_DIR copy) into the sandbox and record its errors
       oracle.py live <tag> [cfgs]      copy the live artifact into the sandbox and diff its errors against the recorded base
cfgs: comma list of lib,test,p2 (default lib,test)."""
import os, re, sys, shutil, subprocess, collections
REPO = '/Users/ueli/Documents/semio'
SBX = '/Users/ueli/.cache/r9a/sbx'
ART = '✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model'
ROOT = ART + '/🦀️.rs'
CRATE = 'semio_s_artifact_bim_model'
TC = '+nightly-2026-07-20'
CONFIGS = {'lib': [], 'test': ['--test'], 'p2': ['--target', 'wasm32-wasip2']}
IGNORE = shutil.ignore_patterns('target', 'node_modules', '.git', '🗑️generated')

def under(path):
    rp = os.path.realpath(path); rr = os.path.realpath(SBX)
    assert rp == rr or rp.startswith(rr + os.sep), ('write outside the sandbox', path)

def neutralize(tree):
    mods = set(); files = []
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith('.rs'): files.append(os.path.join(dp, f))
    texts = {}
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read(); texts[p] = t
        for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)', t, re.M): mods.add(m.group(1))
    keep = {'crate', 'super', 'self'} | mods
    rx_ext = re.compile(r'^\s*extern\s+crate\s+\w+\s+as\s+\w+;[ \t]*$', re.M)
    rx_glob = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    for p, t in texts.items():
        t2 = rx_ext.sub('', t)
        t2 = rx_glob.sub(lambda m: m.group(0) if m.group(3) in keep else m.group(1) + 'const _: () = ();', t2)
        if t2 != t:
            under(p); open(p, 'w', encoding='utf-8').write(t2)

def stage(dst, base):
    under(dst); shutil.rmtree(dst, ignore_errors=True); os.makedirs(dst)
    if base:
        src = os.environ.get('R9A_BASE_DIR')
        if src:
            shutil.copytree(os.path.join(src, ART), os.path.join(dst, ART), symlinks=True, ignore=IGNORE)
        else:
            tar = subprocess.run(['git', '-c', 'core.quotepath=false', 'archive', 'HEAD', '--', ART], cwd=REPO, capture_output=True).stdout
            subprocess.run(['tar', '-x', '-C', dst], input=tar, check=True)
    else:
        shutil.copytree(os.path.join(REPO, ART), os.path.join(dst, ART), symlinks=True, ignore=IGNORE)
    neutralize(dst)

def run(tree, cfg, out):
    root = os.path.join(tree, ROOT)
    cmd = ['rustc', TC, '--edition', '2021', '--crate-type', 'lib', '--crate-name', CRATE, '--emit=metadata', '-o', '/dev/null', '--error-format=short', '-W', 'unused-imports'] + CONFIGS[cfg] + [root]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(root))
    under(out); open(out, 'w', encoding='utf-8').write(p.stderr)
    return p.stderr

def parse(err, tree):
    errs = collections.Counter(); warns = collections.Counter()
    for line in err.split('\n'):
        line = line.replace(tree + '/', '')
        m = re.match(r'^(?:[^\s:][^:]*:\d+:\d+: )?(error(?:\[E\d+\])?|warning): (.*)$', line)
        if not m: continue
        kind, msg = m.group(1), m.group(2)
        if 'unlinked crate' in msg: continue
        if msg.startswith('aborting due to') or re.search(r'generated \d+ warning', msg): continue
        (errs if kind.startswith('error') else warns)[kind + ': ' + msg] += 1
    return errs, warns

def main():
    mode = sys.argv[1]
    os.makedirs(SBX, exist_ok=True)
    if mode == 'base':
        cfgs = (sys.argv[2] if len(sys.argv) > 2 else 'lib,test').split(',')
        tree = os.path.join(SBX, 'base'); stage(tree, True)
        for c in cfgs: run(tree, c, os.path.join(SBX, 'base-%s.err' % c)); print('base', c, 'recorded')
        return
    tag = sys.argv[2]; cfgs = (sys.argv[3] if len(sys.argv) > 3 else 'lib,test').split(',')
    tree = os.path.join(SBX, tag); stage(tree, False)
    bad = 0
    for c in cfgs:
        eb = run(tree, c, os.path.join(SBX, '%s-%s.err' % (tag, c)))
        ea = open(os.path.join(SBX, 'base-%s.err' % c), encoding='utf-8').read()
        a_e, a_w = parse(ea, os.path.join(SBX, 'base')); b_e, b_w = parse(eb, tree)
        oa, ob, wa, wb = a_e - b_e, b_e - a_e, a_w - b_w, b_w - a_w
        print('== %s %s: errors base=%d live=%d only-base=%d only-live=%d | warnings base=%d live=%d only-base=%d only-live=%d' % (tag, c, sum(a_e.values()), sum(b_e.values()), sum(oa.values()), sum(ob.values()), sum(a_w.values()), sum(b_w.values()), sum(wa.values()), sum(wb.values())))
        for k, v in sorted(oa.items())[:30]: print('  -base %dx %s' % (v, k[:230]))
        for k, v in sorted(ob.items())[:40]: print('  +live %dx %s' % (v, k[:230]))
        for k, v in sorted(wb.items())[:15]: print('  +warn %dx %s' % (v, k[:200]))
        if ob or wb: bad += 1
    print('ORACLE', tag, 'CLEAN' if not bad else 'DIFFERENCES')

main()
