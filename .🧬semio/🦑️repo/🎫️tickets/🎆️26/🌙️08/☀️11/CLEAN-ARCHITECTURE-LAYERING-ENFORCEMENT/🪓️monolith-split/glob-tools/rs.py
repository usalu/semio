import os, re
R = "/Users/ueli/Documents/semio"

def strip(src):
    out = []; i = 0; n = len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            d = 1; j = i + 2
            while j < n and d:
                if src.startswith("/*", j): d += 1; j += 2
                elif src.startswith("*/", j): d -= 1; j += 2
                else: j += 1
            out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif c in "rb" and re.match(r'b?r#*"', src[i:i + 12]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'b?r(#*)"', src[i:])
            end = '"' + m.group(1)
            j = src.find(end, i + m.end())
            j = n if j < 0 else j + len(end)
            out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif c == '"' or (c == "b" and src.startswith('b"', i)):
            j = i + (2 if c == "b" else 1)
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j += 1
            out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", src[i:i + 12])
            if m:
                out.append(" " * m.end()); i += m.end()
            else:
                out.append(c); i += 1
        else:
            out.append(c); i += 1
    return "".join(out)

MODRX = re.compile(r'((?:^[ \t]*#\[[^\n]*\]\s*\n)*)^[ \t]*(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', re.M)
PATHRX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
INCRX = re.compile(r'include!\(\s*"([^"]+)"\s*\)')

def read(p):
    return open(p, encoding="utf-8").read()

def with_includes(path):
    t = read(path)
    for m in INCRX.finditer(strip_keep_str(t)):
        pass
    return t

def strip_keep_str(t):
    return t

def mod_children(path, text):
    d = os.path.dirname(path)
    res = []
    for m in MODRX.finditer(text):
        attrs, vis, name = m.group(1), m.group(2), m.group(3)
        pm = PATHRX.search(attrs)
        if pm:
            f = os.path.normpath(os.path.join(d, pm.group(1)))
        else:
            base = os.path.join(d, name)
            f = base + ".rs" if os.path.exists(base + ".rs") else os.path.join(base, "mod.rs")
        res.append((name, (vis or "").strip(), f, attrs))
    return res

def tree(path, mod=(), acc=None):
    acc = {} if acc is None else acc
    text = read(path)
    inc = ""
    for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', text):
        ip = os.path.normpath(os.path.join(os.path.dirname(path), m.group(1)))
        if os.path.exists(ip): inc += "\n" + read(ip)
    acc[mod] = (path, text + inc)
    for name, vis, f, attrs in mod_children(path, text):
        if os.path.exists(f): tree(f, mod + (name,), acc)
    return acc

HEAD = re.compile(r'^(pub(?:\([^)]*\))?\s+)?(?:(?:default|async|unsafe|const|extern(?:\s+"[^"]*")?)\s+)*(fn|struct|enum|trait|type|const|static|union|mod|macro_rules!)\s*(\w+)')
USE = re.compile(r'^(pub(?:\([^)]*\))?\s+)?use\s+')

def use_names(stmt):
    stmt = re.sub(r"\s+", " ", stmt)
    body = stmt.rstrip(";").strip()
    names = []
    def walk(s):
        s = s.strip()
        if "{" in s:
            pre, rest = s.split("{", 1)
            inner = rest[: rest.rindex("}")]
            depth = 0; cur = ""; parts = []
            for ch in inner:
                if ch == "{": depth += 1
                if ch == "}": depth -= 1
                if ch == "," and depth == 0: parts.append(cur); cur = ""
                else: cur += ch
            if cur.strip(): parts.append(cur)
            for p in parts: walk(p)
        else:
            if s.endswith("*"): names.append("*")
            else:
                m = re.search(r"\bas\s+(\w+)$", s)
                if m: names.append(m.group(1))
                else:
                    seg = s.split("::")[-1].strip()
                    if seg == "self":
                        seg = s.split("::")[-2].strip() if "::" in s else seg
                    names.append(seg)
    walk(body[body.index("use ") + 4:] if body.startswith("use ") else re.sub(r"^pub(\([^)]*\))?\s+use\s+", "", body))
    return names

def split_attrs(l):
    attrs = []
    while l.startswith("#["):
        d = 0
        for k, ch in enumerate(l):
            if ch == "[": d += 1
            elif ch == "]":
                d -= 1
                if d == 0:
                    attrs.append(l[:k + 1]); l = l[k + 1:].lstrip(); break
        else:
            return attrs + [l], ""
    return attrs, l

def strip_attrs(l):
    return split_attrs(l)[1]

def top_items(text):
    s = strip(text)
    lines = s.split("\n")
    olines = text.split("\n")
    items = []
    i = 0; n = len(lines)
    pend = []
    while i < n:
        raw = lines[i]
        if not raw.strip() or raw[0] in " \t}":
            i += 1; continue
        attrs, l = split_attrs(raw)
        if attrs:
            pos = 0; oa = []
            for a in attrs:
                st = raw.index(a, pos); oa.append(olines[i][st:st + len(a)]); pos = st + len(a)
            attrs = oa
        pend = pend + attrs
        if not l:
            i += 1; continue
        m = HEAD.match(l)
        u = USE.match(l)
        cfg = [a for a in pend if a.startswith("#[cfg(")]
        if m:
            items.append(dict(kind=m.group(2), name=m.group(3), vis=(m.group(1) or "").strip(), line=i + 1, cfg=cfg))
        elif u:
            j = i; st = l
            while not st.rstrip().endswith(";"):
                j += 1; st += "\n" + lines[j]
            for nm in use_names(st):
                items.append(dict(kind="use", name=nm, vis=(u.group(1) or "").strip(), line=i + 1, cfg=cfg))
            i = j
        pend = []
        i += 1
    return items

IDENT = re.compile(r"[A-Za-z_]\w*")
def idents(text):
    return set(IDENT.findall(strip(text)))
