import re, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lex import mask, segments, classify
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
MARK = re.compile(r'^(\s*)//\s?#(end)?region\s*(.*?)\s*$')
CONSTRE = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:const|static)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*[:=]')
def impl_target(head):
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
    rest = rest.split(' where ')[0].rstrip('{ ').strip()
    if ' for ' in rest: rest = rest.split(' for ', 1)[1]
    rest = rest.lstrip('&').strip()
    rest = re.sub(r'^(mut|dyn)\s+', '', rest)
    m = re.match(r'([A-Za-z_][A-Za-z0-9_:]*)', rest)
    return m.group(1).split('::')[-1] if m else None
class Unit:
    __slots__ = ('file','start','end','cl','kind','name','vis','test','attrs','target','tokens','regions','text','lines','bytes','trait')
    def __repr__(self): return f"<{self.kind} {self.name} {self.start}-{self.end}>"
class SourceFile:
    def __init__(self, key, path):
        self.key = key; self.path = path
        self.text = open(path, encoding='utf-8').read()
        self.lines = self.text.split('\n')
        self.units = []
        self._parse()
    def _regions(self):
        stack = []; per = [None] * (len(self.lines) + 2)
        for i, l in enumerate(self.lines):
            m = MARK.match(l)
            if m:
                if m.group(2): 
                    if stack: stack.pop()
                else:
                    stack.append((l.strip(), m.group(3)))
            per[i+1] = tuple(stack)
        return per
    def _parse(self):
        per = self._regions()
        mt = mask(self.text).split('\n')
        for a, b in segments(self.text):
            kind, name, vis, cl = classify(self.text, (a, b))
            u = Unit(); u.file = self.key; u.start = a; u.end = b; u.cl = cl; u.kind = kind; u.name = name; u.vis = vis or ''
            seg_lines = self.lines[a-1:b]
            attrs = ' '.join(l.strip() for l in self.lines[a-1:cl] if l.strip().startswith('#['))
            u.attrs = attrs
            u.test = 'cfg(test)' in attrs.replace(' ', '')
            u.target = None; u.trait = None
            body_masked = '\n'.join(l for l in mt[cl-1:b] if not l.strip().startswith('#['))
            if kind == 'other':
                cm = CONSTRE.match(body_masked.strip())
                if cm:
                    u.kind = 'const'; u.name = cm.group(1)
                    u.vis = 'pub' if re.match(r'^pub\b', body_masked.strip()) else ''
            if kind == 'impl':
                head = body_masked.split('{')[0]
                u.target = impl_target(head)
                mm = re.match(r'\s*(?:unsafe\s+)?impl\s*(?:<[^{]*?>)?\s*([A-Za-z_][A-Za-z0-9_:<>, ]*?)\s+for\s', re.sub(r'\s+', ' ', head))
                u.trait = mm.group(1) if mm else None
            if kind == 'macro-call':
                mm = re.search(r'\(\s*([A-Za-z_][A-Za-z0-9_]*)', body_masked)
                u.target = mm.group(1) if mm else None
                if re.match(r'\s*macro_rules\s*!', body_masked):
                    u.kind = 'macro_rules!'
                    mm = re.match(r'\s*macro_rules\s*!\s*([A-Za-z_][A-Za-z0-9_]*)', body_masked)
                    u.name = mm.group(1)
            u.tokens = frozenset(TOK.findall(mask('\n'.join(seg_lines))))
            u.regions = per[cl]
            u.text = '\n'.join(seg_lines)
            u.lines = b - a + 1
            u.bytes = len(u.text.encode())
            self.units.append(u)
if __name__ == '__main__':
    f = SourceFile('x', sys.argv[1])
    for u in f.units: print(u, u.vis, u.test, [r[1] for r in u.regions])
