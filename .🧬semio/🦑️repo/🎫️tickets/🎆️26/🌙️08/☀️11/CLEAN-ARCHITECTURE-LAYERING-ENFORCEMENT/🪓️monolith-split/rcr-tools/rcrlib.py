"""Shared helpers of the rcr (REPO-CRATE-ROOT-SPLITS) tools: Rust tokenizer, top-level item outline, paths, snapshots."""
import collections, hashlib, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/rcr-work"
MODS = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
PKG = "📦️packages/🦀️rust"

TARGETS = {
    "tree": dict(mod="🌳️tree", crate="semio_framework_repo_tree", pkg="semio-framework-repo-tree"),
    "test-runner": dict(mod="🏃️test-runner", crate="semio_framework_repo_test_runner", pkg="semio-framework-repo-test-runner"),
    "hooks": dict(mod="🪝️hooks", crate="semio_framework_repo_hooks", pkg="semio-framework-repo-hooks"),
    "mcp": dict(mod="🔌️mcp", crate="semio_framework_repo_mcp", pkg="semio-framework-repo-mcp"),
    "providers": dict(mod="🧩️providers", crate="semio_framework_repo_providers", pkg="semio-framework-repo-providers"),
    "languages": dict(mod="🗣️languages", crate="semio_framework_repo_languages", pkg="semio-framework-repo-languages"),
}
for k, t in TARGETS.items():
    t["tree"] = f"{MODS}/{t['mod']}"
    t["pkgdir"] = f"{MODS}/{t['mod']}/{PKG}"
    t["lib"] = f"{MODS}/{t['mod']}/{PKG}/🦀️.rs"


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def lines_of(text):
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


RAW = re.compile(r'b?r(#*)"')
IDRE = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')


def lex(src):
    """Tokens (kind, text, start_line, end_line, start_idx); lines 0-based."""
    n = len(src)
    i = 0
    toks = []
    line = 0
    while i < n:
        c = src[i]
        if c == '\n':
            line += 1; i += 1; continue
        if c in ' \t\r':
            i += 1; continue
        if src.startswith('//', i):
            j = src.find('\n', i)
            if j < 0: j = n
            toks.append(('comment', src[i:j], line, line, i)); i = j; continue
        if src.startswith('/*', i):
            depth = 1; j = i + 2; sl = line
            while j < n and depth:
                if src.startswith('/*', j): depth += 1; j += 2
                elif src.startswith('*/', j): depth -= 1; j += 2
                else:
                    if src[j] == '\n': line += 1
                    j += 1
            toks.append(('comment', src[i:j], sl, line, i)); i = j; continue
        m = RAW.match(src, i) if c in 'br' and (i == 0 or not (src[i-1].isalnum() or src[i-1] == '_')) else None
        if m:
            end = '"' + m.group(1)
            j = src.find(end, m.end())
            j = n if j < 0 else j + len(end)
            sl = line; line += src[i:j].count('\n')
            toks.append(('str', src[i:j], sl, line, i)); i = j; continue
        if c == '"' or (c == 'b' and src.startswith('b"', i)):
            j = i + (2 if c == 'b' else 1); sl = line
            while True:
                if src[j] == '\\':
                    if src[j+1] == '\n': line += 1
                    j += 2; continue
                if src[j] == '"': j += 1; break
                if src[j] == '\n': line += 1
                j += 1
            toks.append(('str', src[i:j], sl, line, i)); i = j; continue
        if c == "'" or src.startswith("b'", i):
            k = i + (1 if c == "'" else 2)
            m = re.match(r"\\(x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)'", src[k:k+14])
            if m:
                toks.append(('char', src[i:k+m.end()], line, line, i)); i = k + m.end(); continue
            if src[k+1:k+2] == "'" and src[k] != '\\':
                toks.append(('char', src[i:k+2], line, line, i)); i = k + 2; continue
            m = IDRE.match(src, k)
            e = k + (m.end() - k if m else 0)
            toks.append(('life', src[i:e], line, line, i)); i = e; continue
        m = IDRE.match(src, i)
        if m:
            toks.append(('id', m.group(0), line, line, i)); i = m.end(); continue
        if c.isdigit():
            m = re.match(r'[0-9][0-9A-Za-z_\.]*', src[i:])
            t = m.group(0)
            if t.endswith('.'): t = t[:-1]
            if '..' in t: t = t.split('..')[0]
            toks.append(('num', t, line, line, i)); i += len(t); continue
        toks.append(('p', c, line, line, i)); i += 1
    return toks


def top_items(src):
    """Top-level items. Each: dict(start, end (0-based inclusive lines, including leading doc/attr/comment lines), kind, name, vis, ids(set of identifier tokens within), attrs)."""
    toks = lex(src)
    depth = 0
    ends = []
    t0 = 0
    for idx, t in enumerate(toks):
        k, x = t[0], t[1]
        if k == 'p':
            if x in '{([': depth += 1
            elif x in '})]':
                depth -= 1
                if depth == 0 and x == '}':
                    nxt = toks[idx+1][1] if idx + 1 < len(toks) else ''
                    if nxt != ';':
                        ends.append((t0, idx)); t0 = idx + 1
            elif x == ';' and depth == 0:
                ends.append((t0, idx)); t0 = idx + 1
    res = []
    for a, b in ends:
        sub = toks[a:b+1]
        j = 0
        attrs = []
        while j < len(sub):
            k, x = sub[j][0], sub[j][1]
            if k == 'comment': j += 1; continue
            if x == '#' and k == 'p':
                d = 0; s = j; j += 1
                if sub[j][1] == '!': j += 1
                while True:
                    if sub[j][1] == '[': d += 1
                    elif sub[j][1] == ']':
                        d -= 1
                        if d == 0: break
                    j += 1
                attrs.append(''.join(l[1] for l in sub[s:j+1]))
                j += 1; continue
            break
        hdr = sub[j:]
        vis, kind, name = name_kind(hdr) if hdr else ('', 'other', '')
        ctx = set()
        for q in range(len(sub)):
            if sub[q][0] == 'id':
                prev = sub[q-1][1] if q else ''
                ctx.add((sub[q][1], prev in ('.', ':')))
        res.append(dict(first_line=sub[0][2], hdr_line=hdr[0][2] if hdr else sub[0][2], end=sub[-1][3], attrs=attrs, kind=kind, name=name, vis=vis,
                        ids={t[1] for t in sub if t[0] == 'id'}, ctx=ctx, hdr=hdr))
    return res


def name_kind(toks):
    i = 0
    vis = ''
    if toks[i][1] == 'pub':
        vis = 'pub'; i += 1
        if toks[i][1] == '(':
            j = i
            while toks[j][1] != ')': j += 1
            vis = 'pub' + ''.join(t[1] for t in toks[i:j+1]); i = j + 1
    while i < len(toks) and (toks[i][1] in ('async', 'unsafe', 'extern', 'default') or toks[i][0] == 'str' or (toks[i][1] == 'const' and i + 1 < len(toks) and toks[i+1][1] in ('fn', 'unsafe', 'async', 'extern'))):
        i += 1
    kw = toks[i][1]
    if kw in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union'):
        nm = toks[i+1][1]
        if nm == 'mut': nm = toks[i+2][1]
        return vis, kw, nm
    if kw == 'use': return vis, 'use', ''
    if kw == 'impl':
        j = i + 1; s = []
        while j < len(toks) and toks[j][1] not in ('{',) and toks[j][1] != 'where':
            s.append(toks[j][1]); j += 1
        return vis, 'impl', ' '.join(s)
    if kw == 'macro_rules': return vis, 'macro', toks[i+2][1]
    return vis, kw, ''


if __name__ == "__main__":
    t = TARGETS[sys.argv[1]]
    src = read(f"{ROOT}/{t['lib']}")
    for it in top_items(src):
        print(f"{it['first_line']+1:5}-{it['end']+1:5} {it['end']-it['first_line']+1:4} {it['kind']:7} {it['vis']:10} {it['name'][:50]}")


def impl_members(item):
    """Names of fns/consts/types declared directly inside an inherent or trait impl / trait body, with their visibility keyword."""
    toks = item["hdr"]
    out = []
    depth = 0
    started = False
    i = 0
    n = len(toks)
    while i < n:
        k, x = toks[i][0], toks[i][1]
        if k == "p" and x in "{([":
            depth += 1
            if x == "{" and depth == 1:
                started = True
        elif k == "p" and x in "})]":
            depth -= 1
        elif started and depth == 1 and k == "id" and x in ("fn", "const", "type"):
            nm = toks[i + 1][1] if i + 1 < n else ""
            if x == "const" and nm == "fn":
                nm = toks[i + 2][1]
            vis = ""
            j = i - 1
            while j >= 0 and toks[j][1] in ("async", "unsafe", "const", "extern", "default") or (j >= 0 and toks[j][0] == "str"):
                j -= 1
            if j >= 0 and toks[j][1] == ")":
                m = j
                while toks[m][1] != "(":
                    m -= 1
                if toks[m - 1][1] == "pub":
                    vis = "pub" + "".join(t[1] for t in toks[m:j + 1])
                    j = m - 2
                else:
                    j = j
            elif j >= 0 and toks[j][1] == "pub":
                vis = "pub"
            out.append((nm, vis))
        i += 1
    return out
