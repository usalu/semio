import sys; sys.path.insert(0, '.')
from common6 import *
def parse_fields():
    va, L, R = struct_fields()
    s, e = va['s'] - 1, va['e']
    # find body open brace
    body = R[s:e]
    # locate '{' line
    start = next(i for i, l in enumerate(body) if l.rstrip().endswith('{'))
    out = []; cur = []; depth = 0
    # iterate raw lines between start+1 and last '}' ; split top-level fields by ',' at depth 0 over stripped text
    Lb = L[s:e]
    i = start + 1
    pending_doc = []
    field_lines = []
    buf_raw = []; buf_str = ''
    d = 0
    while i < len(body) - 1:
        raw = body[i]; st = Lb[i]
        buf_raw.append(raw); buf_str += st + '\n'
        prev = ''
        for ch in st:
            if ch in '<([{': d += 1
            elif ch in '>)]}' and not (ch == '>' and prev in '-='): d -= 1
            prev = ch
        if d == 0 and st.rstrip().endswith(','):
            out.append((buf_raw, buf_str)); buf_raw = []; buf_str = ''
        i += 1
    if ''.join(buf_raw).strip(): out.append((buf_raw, buf_str))
    fields = []
    for raw, st in out:
        txt = re.sub(r'#\[[^\]]*\]', '', st).strip()
        m = re.search(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:', txt)
        # first top-level 'name:' occurrence
        first = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:', txt)
        if not first: continue
        name = first.group(1)
        rawtxt = '\n'.join(raw)
        cfg = [l.strip() for l in raw if l.strip().startswith('#[')]
        docs = [l for l in raw if l.strip().startswith('///')]
        decl = [l for l in raw if not l.strip().startswith(('///', '#[', '//'))]
        vis = re.match(r'\s*(pub(?:\([^)]*\))?)\s', decl[0]) if decl else None
        fields.append(dict(name=name, vis=vis.group(1) if vis else '', cfg=cfg, docs=docs, raw=raw, decl='\n'.join(decl)))
    return fields
def comp_of(name):
    for pat, c in pt.RULES:
        if re.search(pat, name): return c
    return 'UNASSIGNED'
if __name__ == '__main__':
    fs = parse_fields()
    print(len(fs), 'fields')
    c = collections.Counter(comp_of(f['name']) for f in fs)
    print(c)
    print(collections.Counter(f['vis'] for f in fs))
    for f in fs:
        if comp_of(f['name']) == 'UNASSIGNED': print('UNASSIGNED', f['name'])
    json.dump([dict(name=f['name'], vis=f['vis'], comp=comp_of(f['name']), cfg=f['cfg']) for f in fs], open(GEN + '/pwp6-fields.json', 'w'), ensure_ascii=False, indent=0)
