import re
def scan_end(t, i):
    """index after the matching '}' of the first '{' at/after i, skipping strings, chars and comments"""
    n = len(t); depth = 0; started = False
    while i < n:
        c = t[i]
        if c == '/' and t[i:i+2] == '//':
            i = t.find('\n', i); i = n if i < 0 else i; continue
        if c == '/' and t[i:i+2] == '/*':
            i = t.find('*/', i) + 2; continue
        if c == '"':
            i += 1
            while i < n and t[i] != '"':
                i += 2 if t[i] == '\\' else 1
            i += 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", t[i:])
            if m: i += m.end(); continue
            i += 1; continue
        if c == '{': depth += 1; started = True
        elif c == '}':
            depth -= 1
            if started and depth == 0: return i + 1
        i += 1
    raise ValueError("unbalanced")
def top_fns(t):
    """column-0 fn items: name -> (start incl. leading comment/attr lines, end)"""
    out = {}
    for m in re.finditer(r"(?m)^(?:pub(?:\([a-z]+\))? )?fn (\w+)", t):
        s = m.start()
        # leading comment/attribute lines
        lines_start = s
        while True:
            prev_end = lines_start - 1
            if prev_end <= 0: break
            prev_start = t.rfind('\n', 0, prev_end) + 1
            ln = t[prev_start:prev_end]
            if ln.startswith('//') or ln.startswith('#['):
                lines_start = prev_start
            else:
                break
        e = scan_end(t, m.end())
        if t[e:e+1] == '\n': e += 1
        out[m.group(1)] = (lines_start, e)
    return out
