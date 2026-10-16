"""🧰️ Shared helpers of the WP5 tools: atomic, snapshot-first writes (every file the work package touches is copied to the scratch snapshot before its first change, created files are recorded), path-literal rebasing and the region-aware item assembler. usage: imported by carve_tt.py and friends"""
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
ROOT = '/Users/ueli/Documents/semio'
PLUG = ROOT + '/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
SNAP = SCR + '/pwp5-snap'
CREATED = SNAP + '/_created.txt'
DELETED = SNAP + '/_deleted.txt'
DRY = os.environ.get('DRY') == '1'
rd = lambda p: open(p, encoding='utf8').read()

def _rel(path):
    real = os.path.realpath(path) if os.path.exists(path) else os.path.join(os.path.realpath(os.path.dirname(path)), os.path.basename(path))
    base = os.path.realpath(PLUG)
    assert real.startswith(base + '/'), ('write outside the plugin tree refused', path)
    return os.path.relpath(real, base)

def _note(file, rel):
    os.makedirs(SNAP, exist_ok=True)
    with open(file, 'a', encoding='utf8') as f: f.write(rel + '\n')

def snapshot(path):
    rel = _rel(path)
    dst = SNAP + '/' + rel
    if os.path.exists(dst) or (os.path.exists(CREATED) and rel in rd(CREATED).split('\n')): return rel
    if os.path.exists(path):
        os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copy2(path, dst)
    else:
        _note(CREATED, rel)
    return rel

def write(path, text):
    if DRY: return
    snapshot(path)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    tmp = path + '.pwp5tmp'
    open(tmp, 'w', encoding='utf8').write(text)
    os.replace(tmp, path)

def delete(path):
    if DRY: return
    rel = snapshot(path)
    _note(DELETED, rel)
    os.remove(path)

PATHLIT = re.compile(r'(#\[\s*path\s*=\s*")([^"]+)(")|(\binclude(?:_str|_bytes)?!\(\s*")([^"]+)("\s*\))')
def rebase_literals(text, old_dir, new_dir):
    def sub(m):
        if m.group(1): rel, pre, post = m.group(2), m.group(1), m.group(3)
        else: rel, pre, post = m.group(5), m.group(4), m.group(6)
        tgt = os.path.normpath(os.path.join(old_dir, rel))
        return pre + os.path.relpath(tgt, new_dir) + post
    return PATHLIT.sub(sub, text)

def assemble(items, assign, dests):
    """items: rsitems.Item list in file order; assign: {item index: dest}; returns {dest: text} with every region marker pair repeated (balanced) around the items a destination receives."""
    out = {d: [] for d in dests}; open_ = {d: [] for d in dests}; orig = []
    def sync(d):
        st = open_[d]
        k = 0
        while k < len(st) and k < len(orig) and st[k] == orig[k]: k += 1
        while len(st) > k:
            n = st.pop(); out[d].append(('end', n))
        for n in orig[k:]:
            st.append(n); out[d].append(('start', n))
    for i, it in enumerate(items):
        if it.kind == 'marker':
            m = re.match(r'//#(end)?region\s+(.*)$', it.text.strip())
            if m.group(1):
                n = m.group(2).strip(); assert orig and orig[-1] == n, (n, orig)
                orig.pop()
                for d in dests:
                    if open_[d] and open_[d][-1] == n and len(open_[d]) > len(orig):
                        open_[d].pop(); out[d].append(('end', n))
            else:
                orig.append(m.group(2).strip())
            continue
        d = assign[i]
        sync(d)
        out[d].append(('item', it))
    for d in dests:
        while open_[d]:
            n = open_[d].pop(); out[d].append(('end', n))
    return out

def render(parts):
    chunks = []; prev = None
    for kind, v in parts:
        if kind == 'start': text = '//#region ' + v
        elif kind == 'end': text = '//#endregion ' + v
        else: text = v.text
        if prev is not None:
            pk, pv = prev
            if pk == 'start' or kind == 'end': gap = '\n'
            elif pk == 'end': gap = '\n\n'
            elif pk == 'item' and kind == 'item': gap = '\n' if pv.end + 1 == v.start else '\n\n'
            else: gap = '\n\n'
            chunks.append(gap)
        chunks.append(text); prev = (kind, v)
    return ''.join(chunks) + '\n'
