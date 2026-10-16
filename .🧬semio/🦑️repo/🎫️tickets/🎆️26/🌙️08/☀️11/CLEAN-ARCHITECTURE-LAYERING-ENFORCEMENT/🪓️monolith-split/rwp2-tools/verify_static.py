"""✅️ Static gate of the WP2 crates: manifests, path resolution, ownership, rank order, path lengths, rustfmt parse."""
import os, re, sys, json, subprocess, collections
from model import *
import deps_check as dc, oracle2_closure as oc
PFX = 'semio-framework-os-renderer-wgpu-'
bad = 0
def fail(msg):
    global bad; bad += 1; print('FAIL', msg)
owner = collections.defaultdict(list)
rank = {c: i for i, c in enumerate(RANK)}
for c in RANK:
    root = T + '/' + FOLDER[c] + '/🦀️.rs'
    man = T + '/' + FOLDER[c] + '/📦️packages/🦀️rust/Cargo.toml'
    try: d = dc.toml(man)
    except Exception as e: fail('%s manifest does not parse: %s' % (c, e)); continue
    if d['package']['name'] != PFX + c: fail('%s name %s' % (c, d['package']['name']))
    if d['lib']['path'] != '../../🦀️.rs': fail('%s lib path' % c)
    b = d['package']['metadata']['semio']['rust-source-boundary']
    if b != {'maxSourceBytes': 262144, 'maxSourceFiles': 32, 'maxActiveLines': 4000}: fail('%s boundary keys %s' % (c, b))
    tabs = [d.get('dependencies', {})] + [v.get('dependencies', {}) for v in d.get('target', {}).values()] + [d.get('dev-dependencies', {})]
    for tb in tabs:
        for k, v in tb.items():
            if isinstance(v, dict) and 'path' in v:
                p = os.path.normpath(os.path.join(os.path.dirname(man), v['path']))
                if not os.path.isfile(p + '/Cargo.toml'): fail('%s dep %s path %s does not resolve' % (c, k, v['path']))
            if k.startswith(PFX) and k[len(PFX):] in rank and rank[k[len(PFX):]] >= rank[c]: fail('%s depends upward on %s' % (c, k))
    # every #[path] and include literal resolves
    stack = [root]; seen = set()
    while stack:
        f = stack.pop()
        if f in seen: continue
        seen.add(f)
        if not os.path.isfile(f): fail('%s missing file %s' % (c, f)); continue
        txt = open(f, encoding='utf8').read()
        for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', txt):
            t = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if not os.path.isfile(t): fail('%s path literal %s from %s' % (c, m.group(1), f.split('wgpu/')[-1]))
            else: stack.append(t)
        for m in re.finditer(r'include_str!\("([^"]+)"\)', txt):
            t = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
            if not os.path.isfile(t) and 'CARGO' not in m.group(1): fail('%s include_str %s' % (c, m.group(1)))
        if len(f.encode()) > 240: fail('path too long %d %s' % (len(f.encode()), f))
    for f in seen:
        if '📥️lifted' in f: continue
        owner[f].append(c)
for f, cs in owner.items():
    if len(cs) > 1: fail('file owned by %s: %s' % (cs, f.split('wgpu/')[-1]))
# rustfmt parse of lib roots
for c in RANK:
    root = T + '/' + FOLDER[c] + '/🦀️.rs'
    r = subprocess.run(['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--check', '--config', 'skip_children=true,unstable_features=true', root], capture_output=True, text=True)
    if r.returncode not in (0, 1) or 'error' in r.stderr.lower() and 'parse' in r.stderr.lower(): fail('%s rustfmt: %s' % (c, r.stderr[:200]))
print('ALL STATIC CHECKS PASSED' if not bad else '%d failures' % bad)
