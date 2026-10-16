"""Masking and use-tree helpers of the svm (SCHEMA-VALUE-MATH-SPLITS) tools: blank comments, strings and char literals; flatten and render `use` trees."""
import re

IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


def mask(text):
    out = list(text)
    n = len(text)
    i = 0

    def blank(a, b):
        for k in range(a, b):
            if out[k] != '\n':
                out[k] = ' '

    while i < n:
        c = text[i]
        if c == '/' and i + 1 < n and text[i + 1] == '/':
            j = text.find('\n', i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
            continue
        if c == '/' and i + 1 < n and text[i + 1] == '*':
            d = 1
            k = i + 2
            while k < n and d:
                if text.startswith('/*', k):
                    d += 1
                    k += 2
                elif text.startswith('*/', k):
                    d -= 1
                    k += 2
                else:
                    k += 1
            blank(i, k)
            i = k
            continue
        if c in 'br' and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == '_')):
            m = re.match(r'b?r(#*)"', text[i:i + 40])
            if m:
                close = '"' + m.group(1)
                s = i + m.end()
                j = text.find(close, s)
                j = n if j < 0 else j + len(close)
                blank(i, j)
                i = j
                continue
        if c == '"' or (c == 'b' and text.startswith('b"', i) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == '_'))):
            k = i + (2 if c == 'b' else 1)
            while k < n and text[k] != '"':
                k += 2 if text[k] == '\\' else 1
            k += 1
            blank(i, k)
            i = k
            continue
        if c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i + 16])
            if m:
                blank(i, i + m.end())
                i += m.end()
                continue
            i += 1
            continue
        i += 1
    return ''.join(out)


def idents(masked, with_dot=False):
    """Identifiers of masked code. with_dot=False: those not preceded by '.' (paths, types, calls). with_dot=True: those preceded by '.' only."""
    res = set()
    for m in IDENT.finditer(masked):
        s = m.start()
        k = s - 1
        while k >= 0 and masked[k] in ' \t\n':
            k -= 1
        dot = k >= 0 and masked[k] == '.' and not (k >= 1 and masked[k - 1] == '.')
        if dot == with_dot:
            res.add(m.group(0))
    return res


class UseTree:
    def __init__(self, path, alias, glob):
        self.path = path
        self.alias = alias
        self.glob = glob

    def binding(self):
        if self.glob:
            return None
        if self.alias:
            return self.alias
        return self.path[-1]

    def key(self):
        return (tuple(self.path), self.alias, self.glob)


def _tokens(s):
    return re.findall(r'::|\{|\}|,|\*|[A-Za-z_][A-Za-z0-9_]*|r#[A-Za-z_]\w*|;', s)


def parse_use(text):
    """Return (attrs_and_vis_prefix, [UseTree]) for one `use` statement text (masked of comments)."""
    m = re.search(r'\buse\b', text)
    head = text[:m.start()]
    body = text[m.end():]
    body = body.strip()
    if body.endswith(';'):
        body = body[:-1]
    toks = _tokens(body)
    pos = 0
    out = []

    def parse_tree(prefix):
        nonlocal pos
        path = list(prefix)
        if pos < len(toks) and toks[pos] == '::':
            pos += 1
        while True:
            t = toks[pos]
            if t == '{':
                pos += 1
                while True:
                    if toks[pos] == '}':
                        pos += 1
                        return
                    parse_tree(path)
                    if toks[pos] == ',':
                        pos += 1
                    elif toks[pos] == '}':
                        pos += 1
                        return
                return
            if t == '*':
                pos += 1
                out.append(UseTree(path, None, True))
                return
            path.append(t)
            pos += 1
            if pos < len(toks) and toks[pos] == '::':
                pos += 1
                continue
            alias = None
            if pos < len(toks) and toks[pos] == 'as':
                alias = toks[pos + 1]
                pos += 2
            out.append(UseTree(path, alias, False))
            return

    parse_tree([])
    return head, out


def render_use(head, trees):
    """Group trees by parent path; emit one `use` statement per group (head repeated)."""
    groups = []
    index = {}
    for t in trees:
        if t.glob:
            key = tuple(t.path)
            leaf = '*'
        else:
            key = tuple(t.path[:-1])
            leaf = t.path[-1] + (f' as {t.alias}' if t.alias else '')
        if key not in index:
            index[key] = len(groups)
            groups.append((key, []))
        groups[index[key]][1].append(leaf)
    lines = []
    for key, leaves in groups:
        prefix = '::'.join(key)
        if len(leaves) == 1 and key:
            lines.append(f'{head}use {prefix}::{leaves[0]};')
        elif not key and len(leaves) == 1:
            lines.append(f'{head}use {leaves[0]};')
        else:
            lines.append(f'{head}use {prefix}::{{{", ".join(leaves)}}};')
    return lines
