# 🔍️ Shared Rust source scanner for the RENDERER-WP5 tools: comment/string stripping that keeps line structure, column-0 item outline, `impl ShellState` method extraction.
import re, os, collections

ROOT = "/Users/ueli/Documents/semio"
MONO = os.path.dirname(os.path.abspath(__file__)) + "/.."
SH = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu"
PLAN = MONO + "/PLAN-RENDERER-WGPU-CRATES.md"

def strip(t):
    o = []; i = 0; n = len(t)
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); i = n if j < 0 else j; continue
        if t.startswith('/*', i):
            d = 1; i += 2
            while i < n and d:
                if t.startswith('/*', i): d += 1; i += 2
                elif t.startswith('*/', i): d -= 1; i += 2
                else:
                    if t[i] == '\n': o.append('\n')
                    i += 1
            continue
        m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i + 12]) if c in 'rbc' and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')) else None
        if m:
            e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
            o.append('""' + '\n' * t[i:j].count('\n')); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            o.append('""' + '\n' * t[i:j].count('\n')); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", t[i:i + 12])
            if m: o.append("''"); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: o.append(' '); i += m.end(); continue
        o.append(c); i += 1
    return ''.join(o)

HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')

def outline(raw):
    L = strip(raw).split('\n'); R = raw.split('\n'); n = len(L)
    depth = 0; starts = []
    for l in L:
        starts.append(depth); depth += l.count('{') - l.count('}')
    items = []; ln = 0
    while ln < n:
        l = L[ln]
        m = HEAD.match(l) if starts[ln] == 0 and l and not l[0].isspace() and l[0] not in '}#' else None
        if m:
            d = 0; seen = False; j = ln
            while j < n:
                d += L[j].count('{') - L[j].count('}'); seen = seen or '{' in L[j]
                if (seen and d <= 0) or (not seen and L[j].rstrip().endswith(';')): break
                j += 1
            s = ln
            while s > 0 and R[s - 1].lstrip().startswith(('///', '#[', '//!')): s -= 1
            items.append(dict(start=s + 1, end=j + 1, kind=m.group(1), name=(m.group(2) or '').strip(), head=l.strip()))
            ln = j + 1
        else: ln += 1
    return items

IMPL = re.compile(r"(?m)^impl(?:<[^>]*>)?\s+(?:ShellState|\w+Ctx<'(?:a|ctx)>)\s*\{")
METH = re.compile(r"(?m)^    (?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)")

def match_brace(t, i):
    d = 0
    while i < len(t):
        if t[i] == '{': d += 1
        elif t[i] == '}':
            d -= 1
            if d == 0: return i
        i += 1
    return len(t) - 1

def methods(raw):
    from blank import blank
    t = blank(raw)
    out = []
    for im in IMPL.finditer(t):
        bo = im.end() - 1; bc = match_brace(t, bo)
        body = t[bo + 1:bc]
        for mm in METH.finditer(body):
            name = mm.group(1)
            k = bo + 1 + mm.start()
            tail = body[mm.end():]
            sig_end = None; depth_p = 0
            for idx, ch in enumerate(tail):
                if ch in '(<[': depth_p += 1
                elif ch in ')>]': depth_p -= 1
                elif ch == '{' and depth_p <= 0: sig_end = idx; break
                elif ch == ';' and depth_p <= 0: sig_end = -1; break
            if sig_end is None or sig_end < 0: continue
            j = bo + 1 + mm.end() + sig_end
            e = match_brace(t, j)
            sl = raw.count('\n', 0, bo + 1 + mm.start()) + 1
            rl = raw.split('\n')
            while sl > 1 and rl[sl - 2].lstrip().startswith(('///', '#[')): sl -= 1
            el = raw.count('\n', 0, e) + 1
            out.append(dict(name=name, start=sl, end=el, body=t[j:e + 1], sig=t[bo + 1 + mm.start():j], impl_at=raw.count('\n', 0, im.start()) + 1))
    return out
