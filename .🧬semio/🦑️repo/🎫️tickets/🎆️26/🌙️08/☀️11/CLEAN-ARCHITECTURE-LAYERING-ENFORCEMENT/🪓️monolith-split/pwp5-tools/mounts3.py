"""🌳️ Mount equivalence of the WP3 lifts: the file set reachable from the 21 WP3 hub units through every `#[path]` and `include!` (leaving-the-directory mounts included), in the pre-edit tree (sandbox B of the differential) against the live tree. The symmetric difference must be exactly the files the lifts created, relocated or removed. usage: python3 mounts3.py <B-plugin-dir> """
import os, re, sys
LIVE = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin'
HUB = LIVE + '/🪝️mounts/👥️wp3-owners/🦀️.rs'
LIT = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude!\(\s*"([^"]+\.rs)"\s*\)')
def inline_mods(text, pos):
    stack = []
    for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{|[{}]', text[:pos]):
        s = m.group(0)
        if s == '{': stack.append(None)
        elif s == '}':
            if stack: stack.pop()
        else: stack.append(m.group(1))
    return [x for x in stack if x]
def walk(root_file, base, rel_root):
    seen = []
    def visit(p):
        if p in seen: return
        if not os.path.exists(p): seen.append(p); return
        seen.append(p)
        t = open(p, encoding='utf8').read()
        for m in LIT.finditer(t):
            rel = m.group(1) or m.group(2)
            if not rel.endswith('.rs'): continue
            mods = inline_mods(t, m.start()) if m.group(1) else []
            visit(os.path.normpath(os.path.join(os.path.dirname(p), *mods, rel)))
    visit(root_file)
    return [os.path.relpath(x, base) for x in seen]
def roots(hub_file, plug):
    t = open(hub_file, encoding='utf8').read()
    out = []
    for m in re.finditer(r'#\[path\s*=\s*"\.\./\.\./([^"]+)"\]', t):
        out.append(os.path.normpath(os.path.join(plug, m.group(1))))
    return out
def tree(plug, hub_file):
    files = []
    for r in roots(hub_file, plug):
        for f in walk(r, plug, plug):
            if f not in files: files.append(f)
    return files
before_plug = sys.argv[1]
b = tree(before_plug, before_plug + '/🪝️mounts/👥️wp3-owners/🦀️.rs'); a = tree(LIVE, HUB)
sb, sa = set(b), set(a)
print('hub units', len(roots(HUB, LIVE)), '| files reachable before', len(sb), 'after', len(sa))
print('only before (relocated or removed):'); [print('  -', x) for x in sorted(sb - sa)]
print('only after (lift destinations and moved files):'); [print('  +', x) for x in sorted(sa - sb)]
missing = [x for x in a if not os.path.exists(os.path.join(LIVE, x))]
print('missing targets after:', missing)
