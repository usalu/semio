import re, sys

def mask(text, keep_strings=False):
    out = list(text)
    n = len(text)
    i = 0
    def blank(a, b):
        for k in range(a, b):
            if out[k] != '\n':
                out[k] = ' '
    while i < n:
        c = text[i]
        if c == '/' and i + 1 < n and text[i+1] == '/':
            j = text.find('\n', i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif c == '/' and i + 1 < n and text[i+1] == '*':
            depth = 1
            j = i + 2
            while j < n and depth:
                if text.startswith('/*', j):
                    depth += 1; j += 2
                elif text.startswith('*/', j):
                    depth -= 1; j += 2
                else:
                    j += 1
            blank(i, j)
            i = j
        elif c == 'r' and re.match(r'r#*"', text[i:i+12]) and (i == 0 or not (text[i-1].isalnum() or text[i-1] == '_')):
            m = re.match(r'r(#*)"', text[i:])
            hashes = m.group(1)
            end = '"' + hashes
            j = text.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            if not keep_strings: blank(i + len(m.group(0)), j - len(end))
            i = j
        elif c == 'b' and i + 1 < n and text[i+1] == '"' and (i == 0 or not (text[i-1].isalnum() or text[i-1] == '_')):
            j = i + 2
            while j < n and text[j] != '"':
                j += 2 if text[j] == '\\' else 1
            if not keep_strings: blank(i + 2, j)
            i = j + 1
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == '\\' else 1
            if not keep_strings: blank(i + 1, j)
            i = j + 1
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", text[i:i+12])
            if m:
                if not keep_strings: blank(i + 1, i + len(m.group(0)) - 1)
                i += len(m.group(0))
            else:
                i += 1
        else:
            i += 1
    return ''.join(out)

ITEM = re.compile(r'^(pub(\([^)]*\))?\s+)?(unsafe\s+)?(async\s+)?(const\s+)?(extern\s+"[^"]*"\s+)?(fn|struct|enum|type|trait|mod|static|use|impl|union|macro_rules!)\b\s*(<[^>]*>)?\s*([A-Za-z_][A-Za-z0-9_]*)?')

def segments(text):
    """Return top-level segments: list of dict(start,end,lines) with 1-based inclusive line numbers."""
    masked = mask(text)
    lines = text.split('\n')
    mlines = masked.split('\n')
    depth = 0
    segs = []
    cur_start = None
    item_started = False
    pre_start = None
    for idx, ml in enumerate(mlines):
        ln = idx + 1
        raw = lines[idx]
        if depth == 0:
            s = raw.strip()
            if s == '':
                if not item_started and pre_start is not None:
                    pass
                continue
            if not item_started:
                if pre_start is None:
                    pre_start = ln
                code = ml.strip()
                is_pre = raw.lstrip().startswith('//') or (code.startswith('#[') or code.startswith('#!['))
                if code == '' and not is_pre:
                    continue
                if not is_pre and code != '':
                    item_started = True
                    cur_start = pre_start
                elif is_pre and code.startswith('#[') :
                    pass
        opens = ml.count('{') + ml.count('(') + ml.count('[')
        closes = ml.count('}') + ml.count(')') + ml.count(']')
        if item_started:
            depth += opens - closes
            if depth == 0:
                code = ml.rstrip()
                nxt = ''
                for jj in range(idx + 1, len(mlines)):
                    if mlines[jj].strip():
                        nxt = mlines[jj].strip(); break
                cont = nxt.startswith(('+', '-', '*', '/', '&&', '||', '.', '?', '==', '!=', '<', '>', '|', 'as ', '%', '^')) and not nxt.startswith(('//',))
                if (code.endswith('}') or code.endswith(';') or code.endswith(')') or code.endswith(']')) and not (cont and not code.endswith(';')):
                    segs.append((cur_start, ln))
                    item_started = False
                    pre_start = None
                    cur_start = None
        else:
            depth += opens - closes
    return segs

def classify(text, seg):
    lines = text.split('\n')
    a, b = seg
    mtxt = mask('\n'.join(lines[a-1:b]))
    ml = mtxt.split('\n')
    head = None
    for k, l in enumerate(ml):
        s = l.strip()
        if s == '' or s.startswith('#['):
            continue
        # skip multi-line attributes
        head = (k, s)
        break
    if head is None:
        return ('comment', None, None, a)
    k, s = head
    code_ln = a + k
    m = ITEM.match(s)
    if m:
        vis = m.group(1).strip() if m.group(1) else ''
        kind = m.group(7)
        name = m.group(9)
        return (kind, name, vis, code_ln)
    if re.match(r'^[A-Za-z_:]+!', s):
        return ('macro-call', s.split('!')[0], '', code_ln)
    return ('other', s[:40], '', code_ln)

if __name__ == '__main__':
    p = sys.argv[1]
    text = open(p, encoding='utf-8').read()
    segs = segments(text)
    for a, b in segs:
        kind, name, vis, cl = classify(text, (a, b))
        print(f"{a}\t{b}\t{b-a+1}\t{kind}\t{vis}\t{name}")
