# 🧩️ WP4 Rust item scanner: blanks comments and literals (length-preserving), matches braces and lists top-level items with their members.
import re, os, sys

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
        if c in 'rbc' and (i == 0 or not (t[i-1].isalnum() or t[i-1] == '_')):
            m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i+12])
            if m:
                e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
                fill(i + m.end(), j - len(e)); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            fill(i + 1, j); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*?|[^\\'])'", t[i:i+14])
            if m:
                fill(i + 1, i + m.end() - 1); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: i += m.end(); continue
        i += 1
    return ''.join(o)

def line_starts(t):
    s = [0]
    for m in re.finditer('\n', t): s.append(m.end())
    return s

class Src:
    def __init__(self, text):
        self.raw = text; self.b = blank(text); self.ls = line_starts(text)
        self.lines = text.split('\n'); self.bl = self.b.split('\n')
    def line_of(self, idx):
        import bisect
        return bisect.bisect_right(self.ls, idx) - 1

REGION = re.compile(r'^\s*//#(region|endregion)\b(.*)$')
HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:default|async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use|extern)(?:\b|(?<=!))')

def match_close(b, open_idx):
    d = 0; i = open_idx; n = len(b)
    while i < n:
        c = b[i]
        if c == '{': d += 1
        elif c == '}':
            d -= 1
            if d == 0: return i
        i += 1
    raise ValueError('unbalanced')

def item_end(src, ln, limit=None):
    """end line index (inclusive) of the item whose first code line is ln"""
    b = src.bl
    n = len(b) if limit is None else limit
    d = 0; seen = False; j = ln
    paren = 0
    while j < n:
        line = b[j]
        for ch in line:
            if ch == '{': d += 1; seen = True
            elif ch == '}': d -= 1
            elif ch == '(' or ch == '[': paren += 1
            elif ch == ')' or ch == ']': paren -= 1
        if seen and d <= 0 and paren <= 0: return j
        if not seen and paren <= 0 and line.rstrip().endswith(';'): return j
        j += 1
    return n - 1

def top_items(src, lo=0, hi=None, indent=0):
    """items whose first code line has exactly `indent` leading spaces inside [lo, hi). returns list of dicts with first/last line (code only, no trivia)"""
    hi = len(src.bl) if hi is None else hi
    out = []; ln = lo
    while ln < hi:
        l = src.bl[ln]
        if l.strip() == '' or REGION.match(src.lines[ln]):
            ln += 1; continue
        ind = len(l) - len(l.lstrip(' '))
        s = l.strip()
        if ind == indent and not s.startswith('#') and not s.startswith('}'):
            m = HEAD.match(s)
            if m or True:
                end = item_end(src, ln, hi)
                out.append(dict(first=ln, last=end, head=s, kind=(m.group(1) if m else 'other')))
                ln = end + 1; continue
        if ind == indent and s.startswith('#'):
            # attribute lines: part of next item; skip (trivia)
            ln += 1; continue
        ln += 1
    return out

NAME_FN = re.compile(r'\bfn\s+([A-Za-z_]\w*)')
def item_name(head, kind):
    if kind == 'fn':
        m = NAME_FN.search(head); return m.group(1) if m else None
    m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(?:(?:default|async|const|unsafe)\s+)*(?:struct|enum|union|trait|type|static|const|mod|macro_rules!|thread_local!)\s*(?:mut\s+)?([A-Za-z_]\w*)?', head)
    return m.group(1) if m else None
