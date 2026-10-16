"""Minimal Rust lexer used by the frs split tools: brace depth per line, multi-line literal detection, top-level item outline."""
import re, sys

IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


def lex(text):
    """Return (depth_at_line_start, inside_literal_at_line_start, code_mask) lists indexed by 0-based line.
    depth counts { } ( ) [ ] over code tokens only."""
    n = len(text)
    i = 0
    depth = 0
    line = 0
    depths = [0]
    inlit = [False]
    # state per char: we only need line start states
    def newline():
        nonlocal line
        line += 1
        depths.append(depth)
        inlit.append(False)
    while i < n:
        c = text[i]
        if c == '\n':
            newline(); i += 1; continue
        if c == '/' and i + 1 < n and text[i+1] == '/':
            j = text.find('\n', i)
            i = n if j < 0 else j
            continue
        if c == '/' and i + 1 < n and text[i+1] == '*':
            d = 1; i += 2
            while i < n and d:
                if text.startswith('/*', i): d += 1; i += 2
                elif text.startswith('*/', i): d -= 1; i += 2
                else:
                    if text[i] == '\n':
                        newline(); inlit[-1] = True
                    i += 1
            continue
        m = re.match(r'(b?r)(#*)"', text[i:i+40]) if c in 'br' else None
        if m and (i == 0 or not (text[i-1].isalnum() or text[i-1] == '_')):
            hashes = m.group(2)
            close = '"' + hashes
            i += m.end()
            j = text.find(close, i)
            if j < 0: j = n
            seg = text[i:j]
            for _ in range(seg.count('\n')):
                pass
            # advance newline states
            k = i
            while k < j:
                if text[k] == '\n':
                    newline(); inlit[-1] = True
                k += 1
            i = j + len(close)
            continue
        if c == '"' or (c == 'b' and text.startswith('b"', i) and (i == 0 or not (text[i-1].isalnum() or text[i-1] == '_'))):
            if c == 'b': i += 1
            i += 1
            while i < n and text[i] != '"':
                if text[i] == '\\':
                    if text[i+1] == '\n':
                        i += 1
                        newline(); inlit[-1] = True
                    i += 2; continue
                if text[i] == '\n':
                    newline(); inlit[-1] = True
                i += 1
            i += 1
            continue
        if c == "'":
            m = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i+16])
            if m:
                i += m.end(); continue
            i += 1
            continue
        if c in '{([': depth += 1
        elif c in '})]': depth -= 1
        i += 1
    while len(depths) < text.count('\n') + 1:
        depths.append(depth); inlit.append(False)
    return depths, inlit


def multiline_literal_lines(text):
    d, lit = lex(text)
    return [i for i, f in enumerate(lit) if f]


ITEM_HEAD = re.compile(r'^(?P<vis>pub(?:\([^)]*\))?\s+)?(?:(?:default|const|async|unsafe|extern\s+"[^"]*"|extern)\s+)*(?P<kw>fn|struct|enum|trait|impl|mod|use|const|static|type|union|macro_rules!|extern\s+crate)\b\s*(?P<rest>.*)$')


def outline(text):
    """Top-level items: list of dict(start,end,head,kind,name,vis) with 0-based inclusive lines.
    start includes the preceding doc comments, attributes and region comments."""
    lines = text.split('\n')
    depths, inlit = lex(text)
    items = []
    i = 0
    N = len(lines)
    pend = None
    while i < N:
        s = lines[i]
        if depths[i] != 0 or inlit[i]:
            i += 1; continue
        st = s.strip()
        if st == '' :
            pend = None
            i += 1; continue
        if st.startswith('///') or st.startswith('#[') or st.startswith('#!') or st.startswith('//'):
            if pend is None: pend = i
            # multi-line attribute
            j = i
            if st.startswith('#[') or st.startswith('#!'):
                while j + 1 < N and depths[j + 1] != 0:
                    j += 1
            i = j + 1
            continue
        m = ITEM_HEAD.match(s)
        start = pend if pend is not None else i
        pend = None
        # find end: first line k>=i whose end-of-line depth is 0 and (line ends with ; or } )
        k = i
        while True:
            nd = depths[k + 1] if k + 1 < N else 0
            if nd == 0 and k + 1 <= N:
                last = lines[k].rstrip()
                if last.endswith(';') or last.endswith('}') or k + 1 >= N or last.endswith(')') or last.endswith(']'):
                    break
            k += 1
            if k >= N - 1:
                break
        kind = m.group('kw') if m else 'other'
        rest = m.group('rest') if m else s
        nm = None
        if kind in ('fn', 'struct', 'enum', 'trait', 'mod', 'const', 'static', 'type', 'union'):
            mm = IDENT.search(rest); nm = mm.group(0) if mm else None
        elif kind == 'macro_rules!':
            mm = IDENT.search(rest); nm = mm.group(0) if mm else None
        items.append(dict(start=start, end=k, head=s.strip()[:110], kind=kind, name=nm, vis=(m.group('vis') or '').strip() if m else ''))
        i = k + 1
    return items


if __name__ == '__main__':
    p = sys.argv[1]
    t = open(p, encoding='utf-8').read()
    ml = multiline_literal_lines(t)
    print('multiline-literal lines:', len(ml), ml[:10], file=sys.stderr)
    for it in outline(t):
        print(f"{it['start']+1:5}-{it['end']+1:5} {it['end']-it['start']+1:5} {it['kind']:7} {it['vis']:10} {it['name'] or ''} | {it['head'][:70]}")
