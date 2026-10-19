"""🔧 LFIX call-site rewriting library: parses Rust call expressions (balanced, string aware) and rewrites their argument lists.
usage: from portlib import *; src = rewrite(src, 'dispatch_typed', old_arity=3, kind='method', fn=lambda c: c.append('app.1.cx()'))"""
import re, sys

def _skip_string(s, i):
    q = s[i]
    i += 1
    while i < len(s) and s[i] != q:
        if s[i] == '\\':
            i += 1
        i += 1
    return i

def _skip_raw(s, i):
    m = re.compile(r'r(#*)"').match(s, i)
    if not m:
        return None
    end = s.find('"' + m.group(1), m.end())
    return end + 1 + len(m.group(1)) if end >= 0 else None

def match_paren(s, open_idx):
    d = 0
    i = open_idx
    while i < len(s):
        c = s[i]
        if c == '"':
            i = _skip_string(s, i)
        elif c == "'" and i + 2 < len(s) and (s[i + 2] == "'" or (s[i + 1] == '\\' and s[i + 3:i + 4] == "'")):
            i += 3 if s[i + 1] != '\\' else 4
            continue
        elif c in '([{':
            d += 1
        elif c in ')]}':
            d -= 1
            if d == 0:
                return i
        elif c == '/' and s[i:i + 2] == '//':
            i = s.find('\n', i)
            if i < 0:
                return -1
            continue
        i += 1
    return -1

def split_top(s):
    """split `s` (args text) at top-level commas -> list of (start,end) spans (stripped text positions)."""
    spans = []
    d = 0
    start = 0
    i = 0
    while i < len(s):
        c = s[i]
        if c == '"':
            i = _skip_string(s, i)
        elif c == "'" and i + 2 < len(s) and (s[i + 2] == "'" or (s[i + 1] == '\\' and s[i + 3:i + 4] == "'")):
            i += 3 if s[i + 1] != '\\' else 4
            continue
        elif c in '([{':
            d += 1
        elif c in ')]}':
            d -= 1
        elif c == ',' and d == 0:
            spans.append((start, i))
            start = i + 1
        i += 1
    tail = s[start:]
    if tail.strip():
        spans.append((start, len(s)))
    return spans

class Call:
    def __init__(self, name, pre, name_start, open_idx, close_idx, args_text):
        self.name = name; self.pre = pre; self.name_start = name_start
        self.open = open_idx; self.close = close_idx
        self.raw = args_text
        self.args = [args_text[a:b].strip() for a, b in split_top(args_text)]
        self.trailing_comma = args_text.rstrip().endswith(',')
    def recv(self, src):
        """receiver expression text before `.name(`: an identifier/field chain; call/index suffixes (`f().x`) are followed only when the chain already began with `.`."""
        if self.pre != '.':
            return None
        j = self.name_start - 1
        while j > 0 and src[j - 1].isspace():
            j -= 1
        k = j - 1
        started_dot = False
        while k >= 0:
            c = src[k]
            if c.isalnum() or c in '_.&*:':
                if c == '.':
                    started_dot = True
                k -= 1
                continue
            if c in ')]' and (started_dot or k == j - 1):
                depth = 0
                while k >= 0:
                    if src[k] in ')]}':
                        depth += 1
                    elif src[k] in '([{':
                        depth -= 1
                        if depth == 0:
                            break
                    k -= 1
                k -= 1
                started_dot = True
                continue
            break
        return src[k + 1:j].strip()

def find_calls(src, name, kind='any', old_arity=None):
    pat = re.compile(r'(?P<pre>\.|::|(?<![A-Za-z0-9_]))' + re.escape(name) + r'\s*(?:::<[^()]*>)?\s*\(')
    out = []
    for m in pat.finditer(src):
        pre = m.group('pre')
        if kind == 'method' and pre != '.':
            continue
        if kind == 'func' and pre == '.':
            continue
        before = src[max(0, m.start() - 16):m.start()]
        if re.search(r'fn\s+$', before):
            continue
        ls = src.rfind('\n', 0, m.start()) + 1
        if src[ls:m.start()].lstrip().startswith('//'):
            continue
        open_idx = m.end() - 1
        close = match_paren(src, open_idx)
        if close < 0:
            continue
        call = Call(name, pre, m.start(name) if False else m.start() + len(pre), open_idx, close, src[open_idx + 1:close])
        if old_arity is not None and len(call.args) != old_arity:
            continue
        out.append(call)
    return out

def rewrite(src, name, kind='any', old_arity=None, fn=None, only_if=None):
    """apply fn(call_args_list, call, src) -> new args list (or None to skip) for every matching call, right to left. returns (new_src, count)."""
    calls = find_calls(src, name, kind, old_arity)
    n = 0
    for c in reversed(calls):
        if only_if and not only_if(c, src):
            continue
        new = fn(list(c.args), c, src)
        if new is None:
            continue
        if new[:len(c.args)] == c.args and len(new) > len(c.args):
            body = c.raw.rstrip()
            tail = ''.join(', ' + a for a in new[len(c.args):])
            if body.endswith(','):
                body = body[:-1]
            src = src[:c.open + 1] + body + tail + src[c.close:]
        else:
            src = src[:c.open + 1] + ', '.join(new) + src[c.close:]
        n += 1
    return src, n

def root_of(expr):
    e = expr.strip()
    while True:
        e2 = re.sub(r'^(&mut\s+|&\s*|\*\s*)', '', e)
        if e2 == e:
            break
        e = e2
    m = re.match(r'[A-Za-z_][A-Za-z0-9_]*', e)
    return m.group(0) if m else None

def read(p):
    return open(p, encoding='utf-8').read()

def write(p, s):
    open(p, 'w', encoding='utf-8').write(s)
