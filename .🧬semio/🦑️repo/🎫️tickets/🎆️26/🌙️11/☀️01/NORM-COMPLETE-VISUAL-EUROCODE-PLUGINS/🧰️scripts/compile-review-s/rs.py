"""🔎 Shared Rust reading helpers of the compile-review S scanners (comment/string stripping, struct/enum index, brace matching)."""
import os, re

NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = NORM + "/🗿️artifacts"
CODE = {"1990": "⚖️en1990", "1991": "🏋️en1991", "1992": "🏛️en1992", "1993": "🔩️en1993", "1994": "🧩️en1994", "1995": "🪵️en1995", "1996": "🪨️en1996", "1997": "🌍️en1997", "1998": "🫨️en1998", "1999": "🪶️en1999"}
SKIP = {"target", "node_modules", ".git", "dist"}

def rs_files(root):
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            if f.endswith(".rs"):
                yield os.path.join(dp, f)

def strip(s, keep_strings=False):
    out, i, n = [], 0, len(s)
    while i < n:
        c = s[i]
        if s.startswith("//", i):
            j = s.find("\n", i)
            i = n if j < 0 else j
        elif s.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if s.startswith("/*", i): depth += 1; i += 2
                elif s.startswith("*/", i): depth -= 1; i += 2
                else: i += 1
            out.append(" ")
        elif c == "r" and re.match(r'r#*"', s[i:i+8]) and (i == 0 or not (s[i-1].isalnum() or s[i-1] == "_")):
            m = re.match(r'r(#*)"', s[i:])
            end = '"' + m.group(1)
            j = s.find(end, i + len(m.group(0)))
            j = n if j < 0 else j + len(end)
            out.append('"S"'); i = j
        elif c == '"':
            j = i + 1
            while j < n and s[j] != '"':
                j += 2 if s[j] == "\\" else 1
            out.append('"S"' if not keep_strings else s[i:j+1]); i = j + 1
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", s[i:i+12])
            if m: out.append("'c'"); i += len(m.group(0))
            else: out.append(c); i += 1
        else:
            out.append(c); i += 1
    return "".join(out)

def match_close(s, i, open_="{", close="}"):
    depth = 0
    n = len(s)
    while i < n:
        c = s[i]
        if c in "([{": depth += 1
        elif c in ")]}": depth -= 1
        if depth == 0: return i
        i += 1
    return -1

def split_top(s, sep=",", angles=True):
    parts, depth, cur, ang, instr, k = [], 0, [], 0, False, 0
    n = len(s)
    while k < n:
        c = s[k]
        if instr:
            cur.append(c)
            if c == "\\" and k + 1 < n: cur.append(s[k + 1]); k += 1
            elif c == '"': instr = False
            k += 1
            continue
        if c == '"': instr = True; cur.append(c); k += 1; continue
        if c in "([{": depth += 1
        elif c in ")]}": depth -= 1
        if not angles: pass
        elif c == "<" and depth == 0 and k + 1 < n: ang += 1
        elif c == ">" and depth == 0 and ang and s[k-1] != "-" and s[k-1] != "=": ang -= 1
        if c == sep and depth == 0 and ang == 0:
            parts.append("".join(cur)); cur = []
        else: cur.append(c)
        k += 1
    if "".join(cur).strip(): parts.append("".join(cur))
    return parts

STRUCT = re.compile(r'\bstruct\s+(\w+)\s*(?:<[^>{;(]*>)?\s*(\{)')
ENUM = re.compile(r'\benum\s+(\w+)\s*(?:<[^>{]*>)?\s*\{')

def fields_of(body):
    out = {}
    for part in split_top(body):
        part = re.sub(r'#\[[^\]]*\]', '', part).strip()
        m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:\s*(.+)$', part, re.S)
        if m: out[m.group(1)] = re.sub(r'\s+', '', m.group(2))
    return out

def variants_of(body):
    out = []
    for part in split_top(body):
        part = re.sub(r'#\[[^\]]*\]', '', part).strip()
        m = re.match(r'(\w+)', part)
        if m: out.append(m.group(1))
    return out

def index_types(root):
    structs, enums, files = {}, {}, {}
    for f in rs_files(root):
        try: src = strip(open(f, encoding="utf-8").read())
        except Exception: continue
        files[f] = src
        for m in STRUCT.finditer(src):
            e = match_close(src, m.end() - 1)
            if e > 0: structs.setdefault(m.group(1), []).append((f, fields_of(src[m.end():e])))
        for m in ENUM.finditer(src):
            e = match_close(src, m.end() - 1)
            if e > 0: enums.setdefault(m.group(1), []).append((f, variants_of(src[m.end():e])))
    return structs, enums, files

def rel(f):
    return f.replace(NORM + "/", "")
