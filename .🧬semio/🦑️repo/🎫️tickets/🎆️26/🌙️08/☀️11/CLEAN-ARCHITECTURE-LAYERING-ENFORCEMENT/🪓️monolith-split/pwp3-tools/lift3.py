"""🛬️ Lift tool of the plugin WP3: cuts closure-complete item sets (or a whole module) out of one source file into the stub lift file of the destination crate, mounts the lift file from the source as a child module and re-exports it with `pub use lifted::*`, so every module path of the monolith stays valid. All writes are all-or-nothing (backup under the scratchpad). usage: imported by lifts.py"""
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import cutplan as cp, mv
from rsitems import split_items
ROOT = '/Users/ueli/Documents/semio'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
BACKUP = SCR + '/pwp3-atomic'
rd = lambda p: open(p, encoding='utf8').read()
_written = {}
DRY = os.environ.get('DRY') == '1'

def write(path, text):
    if DRY: return
    if path not in _written:
        os.makedirs(BACKUP, exist_ok=True)
        if os.path.exists(path):
            shutil.copy2(path, BACKUP + '/' + str(len(_written)) + '.bak'); _written[path] = len(_written)
        else:
            _written[path] = None
    open(path, 'w', encoding='utf8').write(text)

_deleted = {}

def delete(path):
    if DRY: return
    os.makedirs(BACKUP, exist_ok=True)
    shutil.copy2(path, BACKUP + '/del' + str(len(_deleted)) + '.bak'); _deleted[path] = len(_deleted)
    os.remove(path)

def rollback():
    for path, n in _deleted.items():
        shutil.copy2(BACKUP + '/del' + str(n) + '.bak', path)
    for path, n in _written.items():
        if n is None:
            if os.path.exists(path): os.remove(path)
        else:
            shutil.copy2(BACKUP + '/' + str(n) + '.bak', path)
    print('ROLLED BACK', len(_written), 'files')

PATHLIT = re.compile(r'(#\[\s*path\s*=\s*")([^"]+)(")|(\binclude(?:_str|_bytes)?!\(\s*")([^"]+)("\s*\))')

def rebase_literals(text, old_dir, new_dir):
    def sub(m):
        if m.group(1):
            rel = m.group(2); pre, post = m.group(1), m.group(3)
        else:
            rel = m.group(5); pre, post = m.group(4), m.group(6)
        tgt = os.path.normpath(os.path.join(old_dir, rel))
        return pre + os.path.relpath(tgt, new_dir) + post
    return PATHLIT.sub(sub, text)

def children_text(path):
    seen, out = set(), []
    def visit(p, root):
        if p in seen or not os.path.exists(p): return
        seen.add(p); t = rd(p)
        if not root: out.append(t)
        for m in PATHLIT.finditer(t):
            rel = m.group(2) or m.group(5)
            if rel.endswith('.rs'): visit(os.path.normpath(os.path.join(os.path.dirname(p), rel)), False)
    visit(path, True)
    return '\n'.join(out)

def lift(src, seeds, dest, modname, stay=(), extra=(), whole=False, widen_all=False, doc=None, files=()):
    text = rd(src)
    items = cp.itemize(text)
    if whole:
        moved = {id(d) for d in items}
        reason = {}
        defs = {}
    else:
        items, defs, moved, reason = cp.plan(src, list(seeds), stay)
        for e in extra:
            if e not in defs: raise SystemExit('extra not found: ' + e)
            moved.add(id(defs[e]))
    lines = text.split('\n'); drop = set(); taken = []
    for d in items:
        if id(d) not in moved: continue
        it = d['it']
        if d['kind'] == 'macro_rules!': raise SystemExit('macro_rules! cannot be lifted textually: ' + d['first'])
        taken.append(it)
        for i in range(it.start, it.end + 1): drop.add(i)
        j = it.end + 1
        if j < len(lines) and not lines[j].strip() and j not in drop: drop.add(j)
    rest = '\n'.join(l for i, l in enumerate(lines) if i not in drop)
    srt = sorted(taken, key=lambda x: x.start); parts = []
    for k, it in enumerate(srt):
        if k: parts.append('\n' if it.start == srt[k - 1].end + 1 else '\n\n')
        parts.append(it.text)
    body = ''.join(parts)
    body = re.sub(r'\bpub\(super\)', 'pub(crate)', body)
    sup = len(re.findall(r'\bsuper::', body)); sel = len(re.findall(r'(?<![\w:])self::', body))
    if sup or sel:
        print(f'  NOTE {os.path.basename(os.path.dirname(src))}: moved text has {sup} super:: and {sel} self:: paths -> review')
    others_text = re.sub(r'//[^\n]*', '', rest + '\n' + children_text(src))
    others = set(re.findall(r'[A-Za-z_]\w*', others_text))
    dots = set(re.findall(r'\.\s*([A-Za-z_]\w*)', others_text)); inits = set(re.findall(r'\b([A-Za-z_]\w*)\s*:(?!:)', others_text))
    priv = mv.names_private(body)
    used = lambda n, k: n in others if k == 'item' else (n in dots if k == 'impl' else (n in dots or n in inits))
    names = set() if whole else ({n for n in priv} if widen_all else {n for n, k in priv.items() if used(n, k)})
    body = mv.widen(body, names)
    if os.path.exists(dest):
        head = '\n'.join(l for l in rd(dest).split('\n') if l.startswith('//!'))
        head = re.sub(r'\d+ item\(s\), about \d+ lines', f'{len(taken)} item(s), about {sum(it.end - it.start + 1 for it in taken)} lines', head, count=1)
    else:
        if doc is None: raise SystemExit('new lift file needs doc=: ' + dest)
        head = doc
        os.makedirs(os.path.dirname(dest), exist_ok=True)
    new_dir = os.path.dirname(dest); old_dir = os.path.dirname(src)
    body = rebase_literals(body, old_dir, new_dir)
    for rel_file in files:
        rebased = os.path.relpath(os.path.join(old_dir, rel_file), new_dir)
        assert body.count('"' + rebased + '"') == 1, (rel_file, rebased)
        body = body.replace('"' + rebased + '"', '"' + rel_file + '"')
        moved_to = os.path.join(new_dir, rel_file)
        if not DRY: os.makedirs(os.path.dirname(moved_to), exist_ok=True)
        write(moved_to, rd(os.path.join(old_dir, rel_file)))
        delete(os.path.join(old_dir, rel_file))
    out = (head + '\n' if head else '') + ('' if whole else 'use super::*;\n\n') + body.rstrip('\n') + '\n'
    rel = os.path.relpath(dest, os.path.dirname(src))
    if len(os.path.relpath(dest, ROOT).encode()) > 240: raise SystemExit('path too long: ' + dest)
    mount = f'\n#[path = "{rel}"]\nmod {modname};\npub use {modname}::*;\n'
    new_src = rest.rstrip('\n') + '\n' + mount
    write(dest, out); write(src, new_src)
    n = sum(it.end - it.start + 1 for it in taken)
    print(f'  lifted {len(taken)} items / {n} lines  {os.path.relpath(src, ROOT)[-70:]} -> {os.path.relpath(dest, ROOT)[-70:]}; widened {sorted(names)[:12]}{"..." if len(names) > 12 else ""}')
    return taken


def relocate(src, dest):
    """moves a whole module file into its stub lift file (no thin mount stays behind); the caller re-points the one parent that mounted it"""
    text = rd(src)
    head = '\n'.join(l for l in rd(dest).split('\n') if l.startswith('//!'))
    body = rebase_literals(text, os.path.dirname(src), os.path.dirname(dest))
    body = re.sub(r'\bpub\(super\)', 'pub(crate)', body)
    lines = body.split('\n'); k = 0
    while k < len(lines) and lines[k].startswith('//!'): k += 1
    body = '\n'.join(lines[k:]).lstrip('\n')
    write(dest, head + '\n' + ('//!\n' if False else '') + body.rstrip('\n') + '\n')
    delete(src)
    print('  relocated', os.path.relpath(src, ROOT)[-60:], '->', os.path.relpath(dest, ROOT)[-60:])
