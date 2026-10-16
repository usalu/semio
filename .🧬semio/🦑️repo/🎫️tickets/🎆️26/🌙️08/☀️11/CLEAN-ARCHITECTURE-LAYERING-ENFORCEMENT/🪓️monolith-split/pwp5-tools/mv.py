"""🚚️ Item mover for the WP3 staging: cuts named top-level items (with docs, attributes and cfg) out of one Rust file into a new child file, keeps losslessness and widens the private names that now cross the file boundary."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems import split_items

HEAD = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|unsafe|async|extern\s+"[^"]*")\s+)*(fn|struct|enum|trait|type|const|static|union|impl|mod|macro_rules!|thread_local!)\b')
VIS = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)')

def first_code_line(item_text):
    for l in item_text.split('\n'):
        s = l.strip()
        if not s or s.startswith('//') or s.startswith('#[') or s.startswith('/*') or s.startswith('*'):
            continue
        return l
    return ''

def key(item):
    l = first_code_line(item.text).strip()
    return l

def matches(item, spec):
    """spec: 'struct Name' | 'enum Name' | 'fn name' | 'impl Name' (inherent or trait impl FOR name via 'impl Trait for Name') | 'const NAME' | 'static NAME' | 'type Name' | 'trait Name'"""
    line = key(item)
    kind, _, rest = spec.partition(' ')
    if kind == 'thread_local':
        return line.startswith('thread_local!') and rest in item.text
    if kind == 'impl':
        body = re.sub(r'^(?:unsafe\s+)?impl\b', '', line).strip()
        return body.startswith(rest) or re.sub(r'<[^>]*>\s*', '', body, count=1).startswith(rest)
    m = HEAD.match(line)
    if not m or m.group(1) != kind: return False
    after = line[m.end():].strip()
    after = after.lstrip('<') if kind in ('fn',) and False else after
    name = re.match(r'[A-Za-z_][\w]*', after)
    return bool(name) and name.group(0) == rest

def cut(text, specs, strict=True):
    items = split_items(text)
    taken, used = [], set()
    for it in items:
        if it.kind == 'marker': continue
        for s in specs:
            if matches(it, s):
                taken.append(it); used.add(s); break
    missing = [s for s in specs if s not in used]
    if strict and missing: raise SystemExit(f'items not found: {missing}')
    lines = text.split('\n')
    drop = set()
    for it in taken:
        for i in range(it.start, it.end + 1): drop.add(i)
        j = it.end + 1
        if j < len(lines) and not lines[j].strip() and j not in drop: drop.add(j)
    rest = '\n'.join(l for i, l in enumerate(lines) if i not in drop)
    return rest, [it.text for it in taken]

def names_private(text):
    """private top-level names plus private fields and inherent methods: name -> kind"""
    out = {}
    for it in split_items(text):
        if it.kind in ('marker', 'innerdoc', 'use', 'other'): continue
        line = key(it)
        if it.kind == 'impl':
            if re.match(r'^(?:unsafe\s+)?impl\b[^{]*\sfor\s', line): continue
            for m in re.finditer(r'^    (?:async\s+|const\s+|unsafe\s+)*(?:fn|const|type)\s+([A-Za-z_]\w*)', it.text, re.M):
                pre = m.group(0)
                if not re.match(r'^    pub', pre): out[m.group(1)] = 'impl'
            continue
        if not VIS.match(line) and it.kind in ('fn', 'struct', 'enum', 'const', 'static', 'type', 'trait', 'union'):
            n = re.search(r'\b(?:fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)', line)
            if n: out[n.group(1)] = 'item'
        if it.kind == 'struct':
            for m in re.finditer(r'^    (?!pub)([A-Za-z_]\w*)\s*:', it.text, re.M): out[m.group(1)] = 'field'
    return out

def widen(text, names, vis='pub(crate)'):
    """give the named private items/fields/methods visibility `vis` (never inside trait impls)"""
    items = split_items(text)
    lines = text.split('\n')
    for it in items:
        if it.kind in ('marker', 'innerdoc', 'use', 'other'): continue
        line = key(it)
        trait_impl = it.kind == 'impl' and re.match(r'^(?:unsafe\s+)?impl\b[^{]*\sfor\s', line)
        for i in range(it.start, it.end + 1):
            l = lines[i]
            if i == it.start or True:
                top = re.match(r'^((?:(?:const|unsafe|async)\s+)*)(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)', l)
                if top and l == l.lstrip() and top.group(3) in names and not l.startswith('pub'):
                    lines[i] = vis + ' ' + l
                    continue
                if it.kind == 'impl' and not trait_impl:
                    m = re.match(r'^    ((?:(?:async|const|unsafe)\s+)*(?:fn|const|type)\s+([A-Za-z_]\w*))', l)
                    if m and m.group(2) in names: lines[i] = '    ' + vis + ' ' + l[4:]
                if it.kind == 'struct' and l.startswith('    ') and not l.startswith('     '):
                    m = re.match(r'^    ([A-Za-z_]\w*)\s*:', l)
                    if m and m.group(1) in names: lines[i] = '    ' + vis + ' ' + l[4:]
    return '\n'.join(lines)

def idents(text):
    t = re.sub(r'//[^\n]*', '', text)
    return set(re.findall(r'[A-Za-z_]\w*', t))


def cut_method(text, name):
    """cut one four-space-indented `fn name` (with its docs and attributes) out of an impl; returns (rest, method_text)"""
    lines = text.split('\n')
    import re as _re
    start = None
    for i, l in enumerate(lines):
        if _re.match(r'^    (?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe)\s+)*fn ' + _re.escape(name) + r'\b', l):
            start = i; break
    assert start is not None, name
    s = start
    while s > 0 and lines[s - 1].lstrip().startswith(('///', '#[', '//')) and lines[s - 1].startswith('    '):
        s -= 1
    depth = 0; seen = False; j = start
    while j < len(lines):
        depth += lines[j].count('{') - lines[j].count('}')
        seen = seen or '{' in lines[j]
        if seen and depth <= 0: break
        j += 1
    body = '\n'.join(lines[s:j + 1])
    end = j + 1
    if end < len(lines) and not lines[end].strip(): end += 1
    return '\n'.join(lines[:s] + lines[end:]), body
