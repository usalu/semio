"""✂️ split-s include!-splitter: incsplit.py <stage|lossless|install> <tag>. Cuts a Rust test root into `include!`-ed sibling children (shared scope, no visibility changes). Spec: specs/<tag>.py -> SPEC = dict(file, orig, children=[dict(emoji, kebab, doc, ranges=[(a,b)])], subs=[(old, new)], insert_at=None)."""
import os, re, sys, importlib, collections, shutil

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
REPO = '/Users/ueli/Documents/semio'
SCR = '/Users/ueli/.cache/split-s'
V = '️'
DEEP = re.compile(r'(include_(?:str|bytes)!\(\s*")(\.\./)')
RGN = re.compile(r'^\s*//\s?#(end)?region\b')


def regions_open(lines):
    stack = []
    at = []
    for l in lines:
        at.append(tuple(stack))
        m = RGN.match(l)
        if m:
            if m.group(1):
                assert stack, ('orphan endregion', l)
                stack.pop()
            else:
                stack.append(l.strip())
    at.append(tuple(stack))
    return at


def build(sp):
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    lines = orig.split('\n')
    if lines and lines[-1] == '':
        trailing = True
        lines = lines[:-1]
    else:
        trailing = False
    at = regions_open(lines)
    owner = {}
    files = {}
    for c in sp['children']:
        assert c['emoji'] and c['kebab']
        folder = c['emoji'] + V + c['kebab']
        out = ['/// %s%s %s' % (c['emoji'], V, c['doc']), '']
        for a, b in c['ranges']:
            for L in range(a, b + 1):
                assert L not in owner, ('line in two children', L)
                owner[L] = folder
            pre = ['%s' % r for r in at[a - 1]]
            body = lines[a - 1:b]
            post = at[b]
            out.extend(pre)
            out.extend(body)
            out.extend(['//#endregion ' + r.split('#region', 1)[1].strip() for r in reversed(post)])
            out.append('')
        while out and out[-1] == '':
            out.pop()
        files[folder] = out
    subs = sp.get('subs', [])
    for folder, out in files.items():
        for i, l in enumerate(out):
            if sp.get('deepen'):
                out[i] = l = DEEP.sub(lambda m: m.group(1) + '../' + m.group(2), l)
            for old, new in subs:
                if old in l:
                    out[i] = l = l.replace(old, new)
    kept = [i for i in range(1, len(lines) + 1) if i not in owner]
    runs = []
    for i in kept:
        if runs and runs[-1][1] == i - 1:
            runs[-1][1] = i
        else:
            runs.append([i, i])
    print('root keeps original lines', runs)
    root = []
    placed = set()
    ins = sp.get('insert_at')
    for i, l in enumerate(lines, 1):
        if i in owner:
            f = owner[i]
            if f not in placed and (ins is None):
                pass
            continue
        root.append(l)
    first = min(owner)
    k = 0
    for i in range(1, first):
        if i not in owner:
            k += 1
    incl = []
    for c in sp['children']:
        folder = c['emoji'] + V + c['kebab']
        incl.extend(['include!("%s/🦀️.rs");' % folder, ''])
    at_root = (sp['insert_at'] if sp.get('insert_at') else k)
    root[at_root:at_root] = incl[:-1]
    rootsrc = '\n'.join(root) + '\n'
    return rootsrc, {f: '\n'.join(o) + '\n' for f, o in files.items()}


def norm(l):
    return l.strip()


def lossless(tag, sp):
    root, files = build(sp)
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    O = collections.Counter(norm(l) for l in orig.split('\n') if l.strip())
    N = collections.Counter(norm(l) for l in root.split('\n') if l.strip())
    for t in files.values():
        N.update(norm(l) for l in t.split('\n') if l.strip())
    D = collections.Counter()
    if sp.get('deepen'):
        for l, c in list(O.items()):
            if DEEP.search(l):
                D[DEEP.sub(lambda m: m.group(1) + '../' + m.group(2), l)] += c
                del O[l]
    for l, c in list(O.items()):
        for old, new in sp.get('subs', []):
            if old in l:
                D[l.replace(old, new)] += c
                del O[l]
    O.update(D)
    miss = O - N
    extra = N - O
    print('orig', sum(O.values()), 'new', sum(N.values()))
    print('missing', sum(miss.values()))
    for l, c in miss.items():
        print('  -', c, l[:160])
    ok = not miss
    subs = collections.Counter()
    for old, new in sp.get('subs', []):
        pass
    for l, c in sorted(extra.items()):
        why = ''
        if l.startswith('///'):
            why = 'doc'
        elif l.startswith('include!('):
            why = 'include'
        elif RGN.match(l):
            why = 'marker'
        else:
            for old, new in sp.get('subs', []):
                if old in l or new in l:
                    why = 'declared-sub'
            if DEEP.search(l) and sp.get('deepen'):
                why = 'declared-deepen'
        print('  +', why or 'UNEXPLAINED', c, l[:140])
        if not why:
            ok = False
    for tname, rx in (('test', r'#\[(test|[a-z_:]*async_test|[a-z_:]*test)\]'),):
        r = re.compile(rx)
        co = sum(1 for l in orig.split('\n') if r.match(l.strip()))
        cn = sum(1 for l in root.split('\n') if r.match(l.strip())) + sum(1 for t in files.values() for l in t.split('\n') if r.match(l.strip()))
        print('test attrs orig/new', co, cn)
        ok = ok and co == cn
    seq_ok = splice_check(sp, root, files, orig)
    ok = ok and seq_ok
    mo = len(re.findall(r'^\s*//\s?#region\b', orig, re.M))
    eo = len(re.findall(r'^\s*//\s?#endregion\b', orig, re.M))
    for name, t in [('root', root)] + list(files.items()):
        a = len(re.findall(r'^\s*//\s?#region\b', t, re.M))
        b = len(re.findall(r'^\s*//\s?#endregion\b', t, re.M))
        if a != b:
            print('  region imbalance', name[-40:], a, b)
            ok = False
    for f, t in files.items():
        print('  child', len(t.split('\n')) - 1, f)
    print('root lines', len(root.split('\n')) - 1)
    print('LOSSLESS', 'OK' if ok else 'FAIL')
    return ok


def splice_check(sp, root, files, orig):
    def tx(l):
        if sp.get('deepen'):
            l = DEEP.sub(lambda m: m.group(1) + '../' + m.group(2), l)
        for old, new in sp.get('subs', []):
            l = l.replace(old, new)
        return l
    mount = re.compile(r'^include!\("([^"/]+)/🦀️\.rs"\);$')
    out = []
    used = collections.Counter()
    for line in root.split('\n'):
        m = mount.match(line.strip())
        if m and m.group(1) in files:
            used[m.group(1)] += 1
            body = files[m.group(1)].split('\n')[1:]
            out.extend(body)
        else:
            out.append(line)
    keep = lambda ls: [l.strip() for l in ls if l.strip() and not RGN.match(l) and not l.strip().startswith('/// ') or False]
    def seq(ls):
        return [l.strip() for l in ls if l.strip() and not RGN.match(l)]
    so = seq([tx(l) for l in orig.split('\n') if not mount.match(l.strip())])
    sn = seq([l for l in out if not mount.match(l.strip())])
    bad = [f for f in files if used[f] != 1]
    if so != sn:
        k = next((i for i, (a, b) in enumerate(zip(so, sn)) if a != b), min(len(so), len(sn)))
        print('  SEQUENCE DIFFERS at', k, so[k:k + 1], sn[k:k + 1])
        return False
    if bad:
        print('  CHILD MOUNTED != 1', bad)
        return False
    print('  spliced sequence equals the original (%d lines, markers ignored)' % len(so))
    return True


def install(tag, sp):
    target = os.path.join(REPO, sp['file'])
    cur = open(target, 'rb').read()
    orig = open(os.path.join(T, 'orig', sp['orig']), 'rb').read()
    assert cur == orig, 'repo file differs from the recorded original; refusing to overwrite'
    root, files = build(sp)
    base = os.path.dirname(target)
    for f in files:
        assert not os.path.exists(os.path.join(base, f)), ('child exists', f)
    for f, t in files.items():
        os.makedirs(os.path.join(base, f))
        open(os.path.join(base, f, '🦀️.rs'), 'w', encoding='utf-8').write(t)
    tmp = target + '.pfa-tmp'
    open(tmp, 'w', encoding='utf-8').write(root)
    os.replace(tmp, target)
    print('installed', tag, len(files))


if __name__ == '__main__':
    cmd, tag = sys.argv[1], sys.argv[2]
    sp = importlib.import_module('specs.' + tag).SPEC
    if cmd == 'lossless':
        sys.exit(0 if lossless(tag, sp) else 1)
    elif cmd == 'install':
        install(tag, sp)
    elif cmd == 'show':
        r, f = build(sp)
        print(r)
