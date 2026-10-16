# 🔓️ WP4 definition widening: a private definition referenced by name from another module of the Shell tree becomes `pub(super)`.
import re, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

def vis_free(line): return not re.match(r'\s*pub\b', line)

def candidates(src, rel):
    """list of (line_index, name, what) of private definitions in this module text"""
    out = []
    for it in top_items(src):
        k = it['kind']; first = it['first']
        line = src.lines[first]
        if k in ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'const', 'static') and vis_free(line):
            nm = item_name(it['head'], k)
            if nm and nm != '_': out.append((first, nm, k))
        if k == 'struct' and '{' in src.bl[first]:
            body_start = first
            d = 0; started = False
            for i in range(first, it['last'] + 1):
                bl = src.bl[i]
                if started and d == 1:
                    m = re.match(r'^(\s+)([A-Za-z_]\w*)\s*:', bl)
                    if m and vis_free(src.lines[i]) and not re.match(r'\s*(pub|r#)', src.lines[i]): out.append((i, m.group(2), 'field'))
                d += bl.count('{') - bl.count('}')
                if '{' in bl: started = True
        if k == 'impl':
            h = it['head']
            if ' for ' in h.split('{')[0]: continue
            for m in top_items(src, first + 1, it['last'], 4):
                if m['kind'] in ('fn', 'const', 'type') and vis_free(src.lines[m['first']]):
                    nm = item_name(m['head'], m['kind'])
                    if nm: out.append((m['first'], nm, 'member'))
        if k == 'thread_local!':
            for i in range(first, it['last'] + 1):
                m = re.match(r'^(\s+)static\s+([A-Z_a-z]\w*)', src.bl[i])
                if m: out.append((i, m.group(2), 'tlstatic'))
    return out

def apply_widening(files, test_children):
    """files: {path: text}; test_children: {path: [text,...]} extra token sources owned by that module. returns (new files, report)"""
    toks = {}
    for p, t in files.items():
        s = Src(t)
        ids = set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', s.b))
        for x in test_children.get(p, ()): ids |= set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', Src(x).b))
        toks[p] = ids
    new = {}; report = collections.Counter(); detail = []
    for p, t in files.items():
        if p.endswith('/🧊️wgpu/🦀️.rs') and '/📥️staging/' not in p: continue
        if '/⏪️time-travel/🗣️' in p or '/⏪️time-travel/🚦️' in p: continue
        s = Src(t); lines = list(s.lines); changed = False
        for idx, nm, what in candidates(s, p):
            if any(nm in toks[q] for q in toks if q != p):
                lines[idx] = widen_line(lines[idx], what)
                report[what] += 1; detail.append((p, nm, what)); changed = True
        if changed: new[p] = '\n'.join(lines)
    return new, report, detail

def widen_line(line, what):
    ind = line[:len(line) - len(line.lstrip())]
    return ind + 'pub(super) ' + line.lstrip()
