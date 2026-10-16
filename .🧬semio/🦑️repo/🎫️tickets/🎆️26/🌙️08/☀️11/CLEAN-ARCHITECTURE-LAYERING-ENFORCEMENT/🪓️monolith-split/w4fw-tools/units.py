import sys, re, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lex import mask, segments, classify
from graph import FILES
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
CONSTRE = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:const|static)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*[:=]')
IMPLRE = re.compile(r'^(?:unsafe\s+)?impl\s*(?:<[^{]*?>)?\s*(.*?)\s*(?:where\b.*)?\{?\s*$', re.S)
def impl_target(head):
    # head: text of impl header up to '{'
    h = re.sub(r'\s+', ' ', head)
    m = re.match(r'(?:unsafe\s+)?impl\s*', h)
    rest = h[m.end():]
    if rest.startswith('<'):
        depth = 0
        for i, c in enumerate(rest):
            if c == '<': depth += 1
            elif c == '>':
                depth -= 1
                if depth == 0:
                    rest = rest[i+1:].strip(); break
    rest = rest.split(' where ')[0]
    rest = rest.rstrip('{ ').strip()
    if ' for ' in rest:
        rest = rest.split(' for ', 1)[1]
    rest = rest.lstrip('&').strip()
    rest = re.sub(r'^(mut|dyn)\s+', '', rest)
    m = re.match(r'([A-Za-z_][A-Za-z0-9_:]*)', rest)
    if not m: return None
    return m.group(1).split('::')[-1]
def parse_file(key, path):
    text = open(path, encoding='utf-8').read()
    lines = text.split('\n')
    segs = segments(text)
    units = []
    for a, b in segs:
        kind, name, vis, cl = classify(text, (a, b))
        seg_text = '\n'.join(lines[a-1:b])
        m = mask(seg_text)
        attrs = ' '.join(l.strip() for l in lines[a-1:cl] if l.strip().startswith('#['))
        is_test = 'cfg(test)' in attrs.replace(' ', '')
        u = dict(file=key, start=a, end=b, kind=kind, name=name, vis=vis, test=is_test, lines=b-a+1, bytes=len(seg_text.encode()), attrs=attrs[:200])
        if kind in ('other',):
            cm = CONSTRE.match(m.strip().split('\n')[-1] if False else '\n'.join(l for l in m.split('\n') if not l.strip().startswith('#[')).strip())
            if cm:
                u['kind'] = 'const'; u['name'] = cm.group(1)
                u['vis'] = 'pub' if re.match(r'^pub\b', '\n'.join(l for l in m.split('\n') if not l.strip().startswith('#[')).strip()) else ''
        if kind == 'impl':
            head = '\n'.join(l for l in m.split('\n') if not l.strip().startswith('#[')).strip()
            head = head.split('{')[0]
            u['target'] = impl_target(head)
        if kind == 'macro-call':
            mt = re.search(r'\(\s*([A-Za-z_][A-Za-z0-9_]*)', m)
            u['target'] = mt.group(1) if mt else None
        u['tokens'] = sorted(set(TOK.findall(m)))
        units.append(u)
    return units
if __name__ == '__main__':
    out = {}
    for k, p in FILES.items():
        out[k] = parse_file(k, p)
    json.dump(out, open('units.json', 'w'), ensure_ascii=False)
    for k, us in out.items():
        print(k, len(us))
