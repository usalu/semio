import json, os, re, subprocess, glob, copy
T = os.path.dirname(os.path.abspath(__file__))
BUN = glob.glob(os.path.expanduser('~/Library/Caches/semio/runtimes/bun/*/bun'))[0]
G = os.path.join(T, '🗑️generated', 'author-32')
os.makedirs(G, exist_ok=True)

def src(asset):
    return os.path.join(T, 'volumes', asset + '.json')

def load(asset):
    return json.load(open(src(asset)))

def scratch(asset, d, name='scratch'):
    p = os.path.join(G, name + '-' + asset.replace('/', '-') + '.json')
    json.dump(d, open(p, 'w'))
    return p

def run(args, timeout=900):
    r = subprocess.run([BUN] + args, cwd=T, capture_output=True, text=True, timeout=timeout)
    return r.stdout + r.stderr

def areas(asset, scale, path=None):
    a = ['author-32-probe.ts', asset, '--scale', str(scale), '--owned']
    if path: a += ['--source', path]
    out = run(a)
    res = {}
    for l in out.splitlines():
        m = re.match(r'\[owned\] areaDeviation:(\S+) = \S+\s+(?:WARNING.*?; )?exposed ([\d.]+) m2 of ([\d.]+) m2 mesh, areaM2 ([\d.]+)', l)
        if m: res[m.group(1)] = (float(m.group(2)), float(m.group(4)), float(m.group(3)))
    return res, out

def save(asset, d):
    p = src(asset)
    cur = json.load(open(p))
    for s, v in d['volumes'].items():
        cur['volumes'][s] = v
    json.dump(cur, open(p + '.a32.tmp', 'w'))
    os.replace(p + '.a32.tmp', p)

def per_solid(asset, scale, thermal, d):
    """exposure per thermal solid id (m2) with everything else of the scale in place"""
    dd = copy.deepcopy(d)
    dd['volumes'][str(scale)]['thermal'] = thermal
    p = scratch(asset, dd, 'ps')
    out = run(['author-32-probe.ts', asset, '--scale', str(scale), '--owned', '--per-solid', '--source', p])
    res = {}
    for l in out.splitlines():
        m = re.match(r'\[owned\] areaDeviation:(\S+) = \S+\s+(?:WARNING.*?; )?exposed ([\d.]+) m2 of ([\d.]+) m2 mesh', l)
        if m: res[m.group(1)] = (float(m.group(2)), float(m.group(3)))
    return res

def check(asset, scale=None, rules=None, full=True):
    a = ['volumes-check.ts', asset, '--product']
    if scale: a += ['--scale', str(scale)]
    if rules: a += ['--rules', rules]
    out = run(a)
    lines = [l for l in out.splitlines() if l.startswith(('FAIL', 'ok  ')) or 'error' in l.lower()]
    return out, lines

def fails(asset, scale=None, rules=None):
    out, lines = check(asset, scale, rules)
    import re
    for l in lines:
        if l.startswith('FAIL'):
            l = re.sub(r' info:\S+', '', l)
            print(l[:700])
    print([l for l in out.splitlines() if l.startswith('FAIL:')][-1:] or ['ALL OK'])

ROOT = os.path.abspath(os.path.join(T, *(['..'] * 7)))
def asset_path(cat, slug):
    base = os.path.join(ROOT, '🎓️teaching', '🏛️architecture', '🧍️staffage', '🗂️catalog')
    c = [x for x in os.listdir(base) if x.endswith(cat)][0]
    return os.path.join(base, c, slug, '🔣️.json')

def region_edit(path, region, area=None, hidden=None):
    """text-level edit of one thermal region's areaM2 / hidden (re-reads the file right before)"""
    lines = open(path, encoding='utf8').read().split('\n')
    start = next(i for i, l in enumerate(lines) if l == '  "thermal": {' or l == '    "thermal": {')
    at = next(i for i in range(start, len(lines)) if lines[i].strip() == f'"id": "{region}",' and lines[i].startswith('     "id"'))
    end = next(i for i in range(at + 1, len(lines)) if lines[i].startswith('    }'))
    if area is not None:
        ai = next(i for i in range(at, end) if re.match(r'\s+"areaM2": ', lines[i]))
        lines[ai] = re.sub(r'"areaM2": [\d.eE+-]+', f'"areaM2": {area}', lines[ai])
    if hidden is not None:
        if any(lines[i].startswith('     "hidden"') for i in range(at, end)):
            return
        lines[end - 1] = lines[end - 1] + ','
        lines[end:end] = ['     "hidden": {', f'      "en": {json.dumps(hidden[0], ensure_ascii=False)},', f'      "de": {json.dumps(hidden[1], ensure_ascii=False)}', '     }']
    open(path, 'w', encoding='utf8').write('\n'.join(lines))

def outside(asset, scale, d, structure):
    """outsideBodyM per view for a structure list"""
    dd = copy.deepcopy(d); dd['volumes'][str(scale)]['structure'] = structure
    p = scratch(asset, dd, 'out')
    out = run(['author-32-probe.ts', asset, '--scale', str(scale), '--source', p, '--rules', 'volumes-structure-coverage', '--all'])
    r = {}
    for l in out.splitlines():
        m = re.search(r'outsideBodyM:(\w+) = ([\d.]+)', l)
        if m: r[m.group(1)] = float(m.group(2))
    return r

def find_outside(asset, scale, d, structure, groups=None, limit=0.0075):
    res = {}
    for s in structure:
        if s.get('part') == 'support': continue
        r = outside(asset, scale, d, [s])
        if max(r.values() or [0]) > limit: res[s['id']] = r
    return res
