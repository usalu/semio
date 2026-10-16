# 🫥️ Length-preserving blanking of comments, string and char literals so token scans can address raw text by offset.
import re

def blank(t):
    o = list(t); i = 0; n = len(t)
    def fill(a, b):
        for k in range(a, b):
            if o[k] != '\n': o[k] = ' '
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); j = n if j < 0 else j
            fill(i, j); i = j; continue
        if t.startswith('/*', i):
            d = 1; j = i + 2
            while j < n and d:
                if t.startswith('/*', j): d += 1; j += 2
                elif t.startswith('*/', j): d -= 1; j += 2
                else: j += 1
            fill(i, j); i = j; continue
        if c in 'rbc' and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')):
            m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i + 12])
            if m:
                e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
                fill(i + m.end(), j - len(e)); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            fill(i + 1, j); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*?|[^\\'])'", t[i:i + 14])
            if m:
                fill(i + 1, i + m.end() - 1); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: i += m.end(); continue
        i += 1
    return ''.join(o)
