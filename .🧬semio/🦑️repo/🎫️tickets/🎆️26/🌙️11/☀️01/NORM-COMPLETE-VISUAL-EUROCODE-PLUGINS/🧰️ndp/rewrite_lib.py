import re
def match_close(s, i):
    depth = 0; j = i; n = len(s)
    while j < n:
        c = s[j]
        if c == '"':
            j += 1
            while s[j] != '"':
                j += 2 if s[j] == '\\' else 1
        elif c == "'" and j + 2 < n and (s[j+2] == "'" or (s[j+1] == '\\')):
            j += 3 if s[j+1] != '\\' else 4
            continue
        elif c in '([{': depth += 1
        elif c in ')]}':
            depth -= 1
            if depth == 0: return j
        j += 1
    raise ValueError("unbalanced")
def split_top(s):
    parts = []; depth = 0; cur = 0; j = 0
    while j < len(s):
        c = s[j]
        if c == '"':
            j += 1
            while s[j] != '"': j += 2 if s[j] == '\\' else 1
        elif c in '([{': depth += 1
        elif c in ')]}': depth -= 1
        elif c == ',' and depth == 0:
            parts.append((cur, j)); cur = j + 1
        j += 1
    parts.append((cur, len(s)))
    return parts
def sub_literals(text, mapping):
    def r(m):
        v = m.group(1)
        return '"%s"' % mapping[v] if v in mapping else m.group(0)
    return re.sub(r'"([^"\\]*)"', r, text)
def rewrite_calls(src, call_re, mapping, arg_index=None):
    out = []; pos = 0
    for m in re.finditer(call_re, src):
        if m.start() < pos: continue
        open_i = m.end() - 1
        close = match_close(src, open_i)
        inner = src[open_i + 1:close]
        if arg_index is None:
            new = sub_literals(inner, mapping)
        else:
            parts = split_top(inner)
            if len(parts) <= arg_index: continue
            a, b = parts[arg_index]
            new = inner[:a] + sub_literals(inner[a:b], mapping) + inner[b:]
        out.append(src[pos:open_i + 1]); out.append(new); pos = close
    out.append(src[pos:])
    return "".join(out)
