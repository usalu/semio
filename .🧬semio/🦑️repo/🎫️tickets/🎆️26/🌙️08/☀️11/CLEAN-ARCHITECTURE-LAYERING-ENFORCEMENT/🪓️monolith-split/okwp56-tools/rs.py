# 🦀 Rust source scanner for the OS-KERNEL-WP5-6 carve: length-preserving comment/string stripper, column-0 item outline and impl-method outline (analysis only, no repo writes).
import re

def strip(src):
    out = list(src); n = len(src); i = 0
    def blank(a, b):
        for k in range(a, b):
            if out[k] != '\n': out[k] = ' '
    while i < n:
        c = src[i]
        if c == '/' and i + 1 < n and src[i + 1] == '/':
            j = src.find('\n', i); j = n if j < 0 else j
            blank(i, j); i = j
        elif c == '/' and i + 1 < n and src[i + 1] == '*':
            d = 1; j = i + 2
            while j < n and d:
                if src.startswith('/*', j): d += 1; j += 2
                elif src.startswith('*/', j): d -= 1; j += 2
                else: j += 1
            blank(i, j); i = j
        elif c == 'r' and re.match(r'r#*"', src[i:i + 10]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == '_')):
            m = re.match(r'r(#*)"', src[i:]); h = m.group(1); end = '"' + h
            j = src.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            blank(i, j); i = j
        elif c == 'b' and i + 1 < n and src[i + 1] == '"' and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == '_')):
            j = i + 2
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            blank(i, j + 1); i = j + 1
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            blank(i, j + 1); i = j + 1
        elif c == "'":
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)|[^\\'\n])'", src[i:i + 14])
            if m: blank(i, i + len(m.group(0))); i += len(m.group(0))
            else: i += 1
        else:
            i += 1
    return ''.join(out)

HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:default|async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use|extern\s+crate)\b\s*(.*)')

def line_starts(text):
    s = [0]
    for m in re.finditer('\n', text): s.append(m.end())
    return s

def outline(text, base=0):
    """Column-0 items of `text` (a whole file): list of dicts with line span (1-based, inclusive) including leading attrs, docs and region markers."""
    st = strip(text); L = st.split('\n'); R = text.split('\n'); n = len(L)
    depth = 0; starts = []
    for l in L:
        starts.append(depth)
        depth += l.count('{') - l.count('}')
    items = []; ln = 0
    while ln < n:
        l = L[ln]
        if starts[ln] == 0 and l and not l[0].isspace() and not l.startswith(('}', '#', ')')):
            m = HEAD.match(l)
            if m is None and re.match(r'^(?:pub(?:\([^)]*\))?\s+)?(?:[a-z_]+!|macro_rules!)', l) is not None:
                m = re.match(r'^(?:pub(?:\([^)]*\))?\s+)?([a-z_]+!)\s*(.*)', l)
            if m:
                kind = m.group(1); rest = m.group(2)
                j = ln; d = 0; seen = False
                while j < n:
                    for ch in L[j]:
                        if ch in '{([': d += 1; seen = True if ch == '{' else seen
                        elif ch in '})]': d -= 1
                    if j > ln or True:
                        if (seen and d <= 0 and '{' in ''.join(L[ln:j + 1])) or (d <= 0 and L[j].rstrip().endswith(';')):
                            break
                    j += 1
                s = ln
                while s > 0 and (R[s - 1].lstrip().startswith(('///', '#[', '//!', '//#region', '//#endregion', '// ')) or R[s - 1].strip() == '' and False): s -= 1
                name = ''
                if kind == 'impl': name = re.sub(r'\s+', ' ', ' '.join(x.strip() for x in L[ln:j + 1]).split('{')[0].strip())[:200]
                else:
                    mm = re.match(r'([A-Za-z_][A-Za-z_0-9]*)', rest)
                    name = mm.group(1) if mm else rest[:40]
                items.append(dict(s=s + 1, h=ln + 1, e=j + 1, kind=kind, name=name, vis=l.startswith('pub')))
                ln = j + 1; continue
        ln += 1
    return items

FNHEAD = re.compile(r'^(\s*)(?:pub(?:\([^)]*\))?\s+)?(?:(?:default|async|const|unsafe|extern\s+"[^"]*")\s+)*fn\s+([A-Za-z_][A-Za-z_0-9]*)')

def methods(text, impl_h, impl_e):
    """Depth-1 items (fns, consts, types) of the impl whose header line is impl_h and last line impl_e (1-based): list of dicts."""
    st = strip(text); L = st.split('\n'); R = text.split('\n')
    depth = 0; out = []; i = impl_h - 1
    starts = {}
    d = 0
    for k in range(impl_h - 1, impl_e):
        starts[k] = d
        d += L[k].count('{') - L[k].count('}')
    k = impl_h
    while k < impl_e - 1:
        l = L[k]
        if starts.get(k) == 1 and l.strip() and not l.lstrip().startswith(('}', '#')):
            m = FNHEAD.match(l)
            kind = 'fn' if m else None
            name = m.group(2) if m else None
            if not m:
                m2 = re.match(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(const|type|static)\s+([A-Za-z_][A-Za-z_0-9]*)', l)
                if m2: kind, name = m2.group(1), m2.group(2)
            if kind:
                j = k; dd = 0; seen = False
                while j < impl_e:
                    for ch in L[j]:
                        if ch == '{': dd += 1; seen = True
                        elif ch == '}': dd -= 1
                    if (seen and dd <= 0) or (not seen and L[j].rstrip().endswith(';') and dd <= 0): break
                    j += 1
                s = k
                prev_end = out[-1]['e'] if out else impl_h
                while True:
                    while s > impl_h and s > prev_end and R[s - 1].lstrip().startswith(('///', '#[', '//!', '//#region', '//#endregion', '//')): s -= 1
                    q = s
                    while q > prev_end and R[q - 1].strip() == '': q -= 1
                    if q != s and q > prev_end and R[q - 1].lstrip().startswith(('///', '#[')): s = q; continue
                    break
                out.append(dict(s=s + 1, h=k + 1, e=j + 1, kind=kind, name=name, vis=bool(re.match(r'^\s*pub', l))))
                k = j + 1; continue
        k += 1
    return out
