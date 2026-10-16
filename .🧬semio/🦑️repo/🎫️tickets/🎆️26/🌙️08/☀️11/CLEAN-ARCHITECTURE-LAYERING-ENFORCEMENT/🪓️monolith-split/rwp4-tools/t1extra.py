# 🧩️ WP4 T1 extras: free fns with ShellState parameters, the composer constructor, the Drop impl and `ShellState::<assoc>` paths.
import re, sys, os, json, collections, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from t1lib import *
from units import *
from t1rewrite import *

def split_top_commas(b, raw):
    parts = []; d = 0; start = 0
    for i, ch in enumerate(b):
        if ch in '([{': d += 1
        elif ch in ')]}': d -= 1
        elif ch == ',' and d == 0:
            parts.append(raw[start:i]); start = i + 1
    tail = raw[start:]
    if tail.strip(): parts.append(tail)
    return parts

def regroup_new(raw, report):
    """raw: text of fn new. returns text with `let mut state = Self { ... };` regrouped by component and the rest rewritten."""
    b = blank(raw)
    m = re.search(r'let mut state = Self \{', b)
    assert m, 'no struct literal in new'
    op = m.end() - 1
    cl = find_matching(b, op)
    inner_raw = raw[op + 1:cl]; inner_b = b[op + 1:cl]
    entries = split_top_commas(inner_b, inner_raw)
    groups = collections.OrderedDict((c['struct'], []) for c in COMPS['components'])
    for e in entries:
        t = e.strip('\n')
        eb = blank(t)
        # leading attributes
        k = 0; attrs = []
        lines = t.split('\n'); blines = eb.split('\n')
        i = 0
        while i < len(lines) and (blines[i].strip() == '' or blines[i].lstrip().startswith('#[')):
            if blines[i].strip(): attrs.append(lines[i].strip())
            i += 1
        rest = '\n'.join(lines[i:]).strip()
        nm = re.match(r'([A-Za-z_]\w*)', rest)
        assert nm, rest[:60]
        f = nm.group(1)
        S = F2S[f]
        groups[S].append((attrs, rest))
    out = ['let mut state = Self {']
    for S, items in groups.items():
        out.append(f'            {FIELD_OF[S]}: {S} {{')
        for attrs, rest in items:
            for a in attrs: out.append('                ' + a)
            r = rest.split('\n')
            out.append('                ' + r[0] + (',' if len(r) == 1 else ''))
            for l in r[1:-1]: out.append('                ' + l.lstrip() if False else '                ' + l)
            if len(r) > 1: out.append('                ' + r[-1].lstrip() + ',')
        out.append('            },')
    out.append('        };')
    new_raw = raw[:m.start()] + '\n'.join(out) + raw[cl + 2:]
    # rest of new: state.<tier1 method>(...) calls -> component receivers
    head_len = m.start() + len('\n'.join(out))
    tail = new_raw[head_len:]
    tail2 = rewrite_fn('fn x() {' + tail, 'new', 'usecase', None, report, 'state', False)[len('fn x() {'):]
    return new_raw[:head_len] + tail2

def rewrite_free_defs_in_file(path, report):
    """rewrite the tier-1 / WP5 free fns of one leftover chunk file in place"""
    text = open(path, encoding='utf-8').read()
    src = Src(text)
    lines = list(src.lines)
    its = top_items(src)
    changed = 0
    spans = []
    for it in its:
        if it['kind'] == 'fn':
            nm = item_name(it['head'], 'fn')
            hdr = '\n'.join(src.bl[it['first']:it['first'] + 3])
            if nm and re.search(r'\bstate\s*:\s*&\s*(?:mut\s+)?ShellState', hdr):
                s = trivia_start(src, it['first'], 0)
                spans.append((s, it['first'], it['last'], nm))
        if it['kind'] == 'impl' and re.match(r'impl\s+UiPrefsSnapshot\b', it['head']):
            for m in top_items(src, it['first'] + 1, it['last'], 4):
                if item_name(m['head'], m['kind']) == 'capture':
                    spans.append((m['first'], m['first'], m['last'], 'UiPrefsSnapshot::capture'))
    for s, code, last, nm in sorted(spans, reverse=True):
        raw = '\n'.join(lines[s:last + 1])
        info = PLAN1.get(nm)
        if info and info.get('tier1'):
            new = rewrite_fn(raw, nm, 'tier1', None, report, 'state', True)
        else:
            new = rewrite_fn(raw, nm, 'usecase', None, report, 'state', True)
        lines[s:last + 1] = new.split('\n'); changed += 1
    if changed: open(path, 'w', encoding='utf-8').write('\n'.join(lines) + '\n')
    return changed
