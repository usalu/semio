import re

CHAR_RE = re.compile(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^\\'\n])'", re.S)
RAW_RE = re.compile(r'b?r(#*)"')

def blank(seg):
    return re.sub(r"[^\n]", " ", seg)

def strip_code(src):
    out = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        if c == "/" and src.startswith("//", i):
            j = src.find("\n", i)
            if j < 0: j = n
            out.append(" " * (j - i)); i = j; continue
        if c == "/" and src.startswith("/*", i):
            depth = 1; j = i + 2
            while j < n and depth:
                if src.startswith("/*", j): depth += 1; j += 2
                elif src.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            out.append(blank(src[i:j])); i = j; continue
        if c in "rb" and (i == 0 or not (src[i-1].isalnum() or src[i-1] == "_")):
            m = RAW_RE.match(src, i)
            if m:
                end = '"' + m.group(1)
                j = src.find(end, m.end())
                j = n if j < 0 else j + len(end)
                seg = src[i:j]
                out.append('"' + blank(seg[1:-1]) + '"' if False else blank(seg)); i = j; continue
        if c == '"' or (c == "b" and src.startswith('b"', i) and (i == 0 or not (src[i-1].isalnum() or src[i-1] == "_"))):
            j = i + (2 if c == "b" else 1)
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j = min(j + 1, n)
            out.append('""' + blank(src[i+2:j]) if False else '"' + blank(src[i+1:j-1]) + '"'); i = j; continue
        if c == "'":
            m = CHAR_RE.match(src, i)
            if m:
                out.append("' '" if False else "'" + blank(m.group(0)[1:-1]) + "'"); i = m.end(); continue
        out.append(c); i += 1
    return "".join(out)

if __name__ == "__main__":
    import sys
    s = open(sys.argv[1], encoding="utf-8").read()
    t = strip_code(s)
    assert len(t) == len(s), (len(t), len(s))
    print("ok", len(s))
