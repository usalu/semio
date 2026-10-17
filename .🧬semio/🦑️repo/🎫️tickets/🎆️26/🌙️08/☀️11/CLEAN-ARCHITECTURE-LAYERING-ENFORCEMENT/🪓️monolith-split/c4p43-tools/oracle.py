#!/usr/bin/env python3
import os, re, sys, shutil, subprocess, collections, importlib

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p43 as pfa

REPO = pfa.REPO
SCR = pfa.SCR
SBX = os.path.join(SCR, 'sbx')
TOOLCHAIN = '+nightly-2026-07-20'

import json
O = '🧰️framework/🛍️products/💻️os/🔨️modules'
SPR = O + '/📡️spr'
CHN = SPR + '/🧵️channel'
GROUPS = {
    'inf': dict(crate='semio_framework_os_inference', copy=[O + '/💡️inference'], root=O + '/💡️inference/🦀️.rs', specs=['inf'], cfgs='lib,test,p2,p2test'),
    'dir': dict(crate='semio_framework_os_directory', copy=[O + '/📇️directory'], root=O + '/📇️directory/🦀️.rs', specs=['dir'], cfgs='lib,test,p2,p2test'),
    'hcd': dict(crate='semio_framework_os_history_codec', copy=[SPR + '/📜️history'], root=SPR + '/📜️history/⚙️codec/🦀️.rs', specs=['hcd'], cfgs='lib,test,p2,p2test'),
    'plw': dict(crate='semio_framework_os_spr', copy=[SPR], root=SPR + '/🦀️.rs', specs=['plw'], cfgs='lib,laws,test,p2,p2laws,p2test'),
    'chn': dict(crate='semio_framework_os_channel', copy=[CHN], root=CHN + '/🦀️.rs', specs=['ccd', 'cdc'], cfgs='lib,testkit,test,p2,p2testkit,p2test'),
    'cut': dict(crate='semio_framework_os_channel_tests', copy=[CHN], root=CHN + '/🧪️tests/🪤️companion/🦀️.rs', specs=['cut'], cfgs='test,p2test'),
}
CONFIGS = {
    'lib': [],
    'laws': ['--cfg', 'feature="protocol-laws"'],
    'testkit': ['--cfg', 'feature="testkit"'],
    'test': ['--test'],
    'p2': ['--target', 'wasm32-wasip2'],
    'p2laws': ['--target', 'wasm32-wasip2', '--cfg', 'feature="protocol-laws"'],
    'p2testkit': ['--target', 'wasm32-wasip2', '--cfg', 'feature="testkit"'],
    'p2test': ['--test', '--target', 'wasm32-wasip2'],
}
FRAG = ('cut',)


def spec_of(t):
    if t in FRAG:
        d = json.load(open(os.path.join(T, 'specs-frag', t + '.json'), encoding='utf-8'))
        return dict(orig=t + '.rs', file=d['file'], children=[dict(folder=c['folder']) for c in d['chunks']], stage_frag=d['stage'])
    return importlib.import_module('specs.' + t).SPEC


def folders_of(sp):
    return [c['folder'] for c in sp['children']] if 'stage_frag' in sp else pfa.rel_folders(sp)


def stage_dir_of(t, sp, stage_root):
    return sp['stage_frag'] + '/' + os.path.dirname(sp['file']) if 'stage_frag' in sp else os.path.join(stage_root, t)


def stage_root_file(t, sp, stage_root):
    return sp['stage_frag'] + '/' + sp['file'] if 'stage_frag' in sp else os.path.join(stage_root, t, '🦀️.rs')

IGNORE = shutil.ignore_patterns('target', 'node_modules', '.git', '🗑️generated', 'dist', 'pkg')


def under(path, root):
    rp = os.path.realpath(path)
    rr = os.path.realpath(root)
    assert rp == rr or rp.startswith(rr + os.sep), ('write outside the sandbox', path)


def write(path, text):
    under(os.path.dirname(path), SBX)
    with open(path, 'w', encoding='utf-8') as fh:
        fh.write(text)


IGNORE = shutil.ignore_patterns('target', 'node_modules', '.git', '🗑️generated', 'dist', 'pkg')


def under(path, root):
    rp = os.path.realpath(path)
    rr = os.path.realpath(root)
    assert rp == rr or rp.startswith(rr + os.sep), ('write outside the sandbox', path)


def write(path, text):
    under(os.path.dirname(path), SBX)
    with open(path, 'w', encoding='utf-8') as fh:
        fh.write(text)


def copy_tree(g, dst):
    under(dst, SBX)
    shutil.rmtree(dst, ignore_errors=True)
    for c in g['copy']:
        src = os.path.join(REPO, c)
        out = os.path.join(dst, c)
        if os.path.isdir(src):
            shutil.copytree(src, out, symlinks=True, ignore=IGNORE)
        else:
            os.makedirs(os.path.dirname(out), exist_ok=True)
            shutil.copyfile(src, out)


def overlay(g, dst, stage_root):
    for tg in g['specs']:
        sp = spec_of(tg)
        target_dir = os.path.join(dst, os.path.dirname(sp['file']))
        under(target_dir, SBX)
        shutil.copyfile(stage_root_file(tg, sp, stage_root), os.path.join(target_dir, '🦀️.rs'))
        sd = stage_dir_of(tg, sp, stage_root)
        for f in folders_of(sp):
            d = os.path.join(target_dir, f)
            shutil.rmtree(d, ignore_errors=True)
            os.makedirs(d)
            shutil.copyfile(os.path.join(sd, f, '🦀️.rs'), os.path.join(d, '🦀️.rs'))


def baseline_into(g, dst):
    for tg in g['specs']:
        sp = spec_of(tg)
        p = os.path.join(dst, sp['file'])
        under(os.path.dirname(p), SBX)
        shutil.copyfile(os.path.join(T, 'orig', sp['orig']), p)
        for f in folders_of(sp):
            d = os.path.join(os.path.dirname(p), f)
            under(d, SBX)
            shutil.rmtree(d, ignore_errors=True)


_MODS = {}


def neutralize_text(t, mods):
    keep = {'crate', 'super', 'self'} | mods
    rx_ext = re.compile(r'^\s*extern\s+crate\s+\w+\s+as\s+\w+;[ \t]*$', re.M)
    rx_glob = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$', re.M)
    n = [0, 0]
    t2 = rx_ext.sub(lambda m: (n.__setitem__(0, n[0] + 1), 'const _: () = ();')[1], t)
    def sub(m):
        if m.group(3) in keep:
            return m.group(0)
        n[1] += 1
        return m.group(1) + 'const _: () = ();'
    return rx_glob.sub(sub, t2), n


def neutralize(tree):
    files = []
    mods = set()
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith('.rs'):
                files.append(os.path.join(dp, f))
    texts = {}
    for p in files:
        t = open(p, encoding='utf-8', errors='ignore').read()
        texts[p] = t
        for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)', t, re.M):
            mods.add(m.group(1))
    _MODS[tree] = mods
    tot = [0, 0]
    for p, t in texts.items():
        t2, n = neutralize_text(t, mods)
        tot[0] += n[0]
        tot[1] += n[1]
        if t2 != t:
            write(p, t2)
    return tot


def pull_missing(tree, err):
    pulled = 0
    for m in list(re.finditer(r"couldn't find file `([^`]+)`", err)) + list(re.finditer(r"couldn't read `([^`]+)`", err)):
        path = os.path.normpath(m.group(1))
        prefix = os.path.realpath(tree)
        if not (path.startswith(tree + os.sep) or path.startswith(prefix + os.sep)):
            continue
        rel = path[len(tree) + 1:] if path.startswith(tree + os.sep) else path[len(prefix) + 1:]
        src = os.path.join(REPO, rel)
        if os.path.isfile(src) and not os.path.exists(path):
            under(os.path.dirname(path), SBX)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            shutil.copyfile(src, path)
            if path.endswith('.rs'):
                t = open(path, encoding='utf-8', errors='ignore').read()
                t2, _ = neutralize_text(t, _MODS.get(tree, set()))
                if t2 != t:
                    write(path, t2)
            pulled += 1
    return pulled


def fix_inner_attrs(path):
    t = open(path, encoding='utf-8').read()
    lines = t.split('\n')
    inner = [l for l in lines if re.match(r'^\s*#!\[', l)]
    rest = [l for l in lines if not re.match(r'^\s*#!\[', l)]
    write(path, '\n'.join(inner + rest))


def run(g, tree, cfg, label, tag):
    root = os.path.join(tree, g['root'])
    cmd = ['rustc', TOOLCHAIN, '--edition', '2021', '--crate-type', 'lib', '--crate-name', g['crate'], '--emit=metadata', '-o', '/dev/null', '--error-format=short', '-W', 'unused-imports'] + CONFIGS[cfg] + [root]
    out = os.path.join(SCR, 'orc')
    os.makedirs(out, exist_ok=True)
    for attempt in range(60):
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(root))
        if ("couldn't find file" in p.stderr or "couldn't read" in p.stderr) and pull_missing(tree, p.stderr):
            continue
        break
    open(os.path.join(out, '%s-%s-%s.err' % (tag, label, cfg)), 'w', encoding='utf-8').write(p.stderr)
    return p.stderr


def parse(err):
    errs = collections.Counter()
    warns = collections.Counter()
    for line in err.split('\n'):
        m = re.match(r'^(?:[^\s:][^:]*:\d+:\d+: )?(error(?:\[E\d+\])?|warning): (.*)$', line)
        if not m:
            continue
        kind, msg = m.group(1), re.sub(r'/dev/rmeta\w+', '/dev/rmeta', m.group(2))
        msg = re.sub(r"couldn't read `([^`]+)`", lambda m: "couldn't read `" + re.sub(r'/sbx/[a-z]+/(orig|new)/', '/sbx/X/', os.path.normpath(m.group(1))) + '`', msg)
        if os.environ.get('C4P43_NORM'):
            msg = re.sub(r'\b(?:super::)+super\b', 'super', msg)
            msg = re.sub(r'\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])', '', msg)
        if msg.startswith('aborting due to') or re.search(r'generated \d+ warning', msg):
            continue
        (errs if kind.startswith('error') else warns)[kind + ': ' + msg] += 1
    return errs, warns


def main():
    tag = sys.argv[1]
    g = GROUPS[tag]
    cfgs = sys.argv[2].split(',') if len(sys.argv) > 2 else g['cfgs'].split(',')
    stage_root = os.path.join(SCR, 'stage')
    A = os.path.join(SBX, tag, 'orig')
    B = os.path.join(SBX, tag, 'new')
    if not os.environ.get('C4P43_REUSE'):
        os.makedirs(os.path.join(SBX, tag), exist_ok=True)
        copy_tree(g, A)
        baseline_into(g, A)
        copy_tree(g, B)
        if not os.environ.get('C4P43_LIVE'):
            overlay(g, B, stage_root)
        for tree in (A, B):
            n = neutralize(tree)
        print('neutralized extern/glob', n)
    bad = 0
    for cfg in cfgs:
        ea = run(g, A, cfg, 'orig', tag)
        eb = run(g, B, cfg, 'new', tag)
        a_e, a_w = parse(ea)
        b_e, b_w = parse(eb)
        oa = a_e - b_e
        ob = b_e - a_e
        wa = a_w - b_w
        wb = b_w - a_w
        print('== %s %s: errors orig=%d new=%d  only-orig=%d only-new=%d | warnings orig=%d new=%d only-orig=%d only-new=%d' % (tag, cfg, sum(a_e.values()), sum(b_e.values()), sum(oa.values()), sum(ob.values()), sum(a_w.values()), sum(b_w.values()), sum(wa.values()), sum(wb.values())))
        for k, v in sorted(oa.items())[:40]:
            print('  -orig %dx %s' % (v, k[:230]))
        for k, v in sorted(ob.items())[:40]:
            print('  +new  %dx %s' % (v, k[:230]))
        for k, v in sorted(wb.items())[:25]:
            print('  +warn %dx %s' % (v, k[:200]))
        for k, v in sorted(wa.items())[:25]:
            print('  -warn %dx %s' % (v, k[:200]))
        if ob or wb:
            bad += 1
    print('ORACLE', tag, 'CLEAN' if not bad else 'DIFFERENCES')


if __name__ == '__main__':
    main()
