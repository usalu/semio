"""🌳️ Mount equivalence of the WP5 cuts: the file set reachable through every `#[path]` and `include!` from `👷️job/🦀️.rs` (the parent that mounts tool_run, tool_machine and time_travel) and from the three WP5 module roots, before (pre-edit snapshot for the touched files, live tree for the rest) and after. The difference must be exactly the files the cuts created. Also checks that every module name mounted before is still mounted after (same `mod` names under the same parents) and that each mount keeps its cfg attributes. usage: python3 mounts5.py"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L
LIT = re.compile(r'((?:[ \t]*#\[[^\n]*\]\s*\n)*)[ \t]*#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*\n((?:[ \t]*#\[[^\n]*\]\s*\n)*)[ \t]*((?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+))|\binclude!\(\s*"([^"]+\.rs)"\s*\)')
def inline_mods(text, pos):
    stack = []
    for m in re.finditer(r'(?m)^[ \t]*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{|[{}]', text[:pos]):
        s = m.group(0)
        if s == '{': stack.append(None)
        elif s == '}':
            if stack: stack.pop()
        else: stack.append(m.group(1))
    return [x for x in stack if x]
def reach(root_rel, before):
    seen = {}; mods = []
    def load(rel):
        if before:
            q = L.SNAP + '/' + rel
            if os.path.exists(q): return L.rd(q)
            if rel in created: return None
        p = L.PLUG + '/' + rel
        return L.rd(p) if os.path.exists(p) else None
    def visit(rel):
        if rel in seen: return
        t = load(rel); seen[rel] = t is not None
        if t is None: return
        for m in LIT.finditer(t):
            path = m.group(2) or m.group(6)
            if not path.endswith('.rs'): continue
            inl = inline_mods(t, m.start()) if m.group(2) else []
            child = os.path.normpath(os.path.join(os.path.dirname(rel), *inl, path))
            if m.group(2):
                attrs = re.sub(r'\s+', ' ', (m.group(1) or '') + (m.group(3) or '')).strip()
                mods.append((rel, m.group(5), child, attrs))
            visit(child)
    visit(root_rel)
    return seen, mods
created = [l for l in L.rd(L.CREATED).split('\n') if l] if os.path.exists(L.CREATED) else []
ROOTS = ['👷️job/🦀️.rs', '⏪️time-travel/🦀️.rs', '⏯️tool-run/🦀️.rs', '🛠️tool-machine/🦀️.rs']
fb = {}; fa = {}; mb = []; ma = []
for r in ROOTS:
    s, m = reach(r, True); fb.update(s); mb += m
    s, m = reach(r, False); fa.update(s); ma += m
sb, sa = {k for k, v in fb.items() if v}, {k for k, v in fa.items() if v}
print('roots', len(ROOTS), '| files reachable before', len(sb), 'after', len(sa))
print('only before:', sorted(sb - sa)); print('only after:', sorted(sa - sb))
print('created files:', created)
print('only-after == created .rs files in the reach:', sorted(sa - sb) == sorted(c for c in created if c.endswith('.rs') and c in sa))
print('missing targets after:', [k for k, v in fa.items() if not v])
before = {(p, n): (c, a) for p, n, c, a in mb}; after = {(p, n): (c, a) for p, n, c, a in ma}
lost = {k: v for k, v in before.items() if k not in after}
changed = {k: (v, after[k]) for k, v in before.items() if k in after and v != after[k]}
print('mounts before', len(before), 'after', len(after), '| lost', lost, '| retargeted or attribute-changed', changed)
print('new mounts:', sorted((p, n) for (p, n) in after if (p, n) not in before))
