import re
from uses import *

CFGTEST = re.compile(r'#\[\s*cfg\s*\(\s*test\s*\)\s*\]')

def item_end(code, i):
    """Given index i at start of an item/field/statement (after attributes), return index just past it."""
    n = len(code)
    depth = 0
    j = i
    while j < n:
        c = code[j]
        if c in '{([': depth += 1
        elif c in '})]':
            depth -= 1
            if depth < 0: return j
            if depth == 0 and c == '}':
                k = j + 1
                while k < n and code[k] in ' \t': k += 1
                if k < n and code[k] in ';,': return k + 1
                return j + 1
        elif c == ';' and depth == 0: return j + 1
        elif c == ',' and depth == 0: return j + 1
        j += 1
    return n

def blank_cfg_test(code):
    out = list(code)
    for m in CFGTEST.finditer(code):
        # skip further attributes
        j = m.end()
        while True:
            k = j
            while k < len(code) and code[k].isspace(): k += 1
            if code.startswith('#[', k):
                d = 0
                while k < len(code):
                    if code[k] == '[': d += 1
                    elif code[k] == ']':
                        d -= 1
                        if d == 0: k += 1; break
                    k += 1
                j = k
            else:
                j = k; break
        e = item_end(code, j)
        for x in range(m.start(), e):
            if out[x] != '\n': out[x] = ' '
    return "".join(out)

def is_test_file(f, files):
    cur = f
    while cur is not None:
        if 'cfg(test)' in cur.attrs.replace(' ', '') : return True
        cur = cur.parent
    return False

def prod_code(f, files):
    if is_test_file(f, files): return ""
    return blank_cfg_test(f.code)
