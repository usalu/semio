"""🔎️ Section/module/function size breakdown of a wasm component without wasm-tools."""
import sys, os, re, collections

CORE_SECTIONS = {0:"custom",1:"type",2:"import",3:"func",4:"table",5:"memory",6:"global",7:"export",8:"start",9:"elem",10:"code",11:"data",12:"datacount",13:"tag"}
COMP_SECTIONS = {0:"custom",1:"core-module",2:"core-instance",3:"core-type",4:"component",5:"instance",6:"alias",7:"type",8:"canon",9:"start",10:"import",11:"export",12:"value"}

def leb(buf, i):
    r = 0; s = 0
    while True:
        b = buf[i]; i += 1
        r |= (b & 0x7f) << s
        if not (b & 0x80): return r, i
        s += 7

def name(buf, i):
    n, i = leb(buf, i)
    return buf[i:i+n].decode("utf-8", "replace"), i+n

def sections(buf, start, end):
    i = start
    while i < end:
        sid = buf[i]; i += 1
        sz, i = leb(buf, i)
        yield sid, i, sz
        i += sz

def func_names(buf, s, e):
    """name section subsection 1"""
    out = {}
    i = s
    nm, i = name(buf, i)
    if nm != "name": return out
    while i < e:
        sub = buf[i]; i += 1
        sz, i = leb(buf, i)
        if sub == 1:
            j = i
            cnt, j = leb(buf, j)
            for _ in range(cnt):
                idx, j = leb(buf, j)
                fn, j = name(buf, j)
                out[idx] = fn
        i += sz
    return out

def crate_of(fn):
    d = fn
    m = re.match(r"^_ZN(.*)$", fn)
    # rust legacy mangling _ZN<len>seg...  ; v0 mangling _R...
    if fn.startswith("_ZN"):
        i = 3; segs = []
        while i < len(d) and d[i].isdigit():
            j = i
            while j < len(d) and d[j].isdigit(): j += 1
            n = int(d[i:j]); segs.append(d[j:j+n]); i = j+n
        if segs: return segs[0]
    if fn.startswith("_R"):
        m = re.search(r"_R[A-Za-z]*?(\d+)([A-Za-z_][A-Za-z0-9_]*)", fn)
        if m: return m.group(2)
    m = re.match(r"^([A-Za-z0-9_]+)::", fn)
    if m: return m.group(1)
    return "<" + fn.split("$")[0][:40] + ">"

def main(path):
    buf = open(path, "rb").read()
    print(f"file {path}")
    print(f"total {len(buf)} bytes")
    assert buf[:4] == b"\0asm", "not wasm"
    ver = buf[4:8]
    is_component = ver[2:4] != b"\0\0"
    print(f"version {ver.hex()} component={is_component}")
    mods = []
    if is_component:
        tot = collections.Counter()
        for sid, off, sz in sections(buf, 8, len(buf)):
            nm = COMP_SECTIONS.get(sid, str(sid))
            tot[nm] += sz
            if sid == 1:
                mods.append((off, sz))
        print("\n== component sections ==")
        for k, v in tot.most_common():
            print(f"{v:>12}  {v/1048576:>9.2f} MiB  {k}")
    else:
        mods.append((8 - 8, len(buf)))
    print(f"\n== {len(mods)} core module(s) ==")
    for n, (off, sz) in enumerate(mods):
        print(f"  module[{n}] {sz} bytes ({sz/1048576:.2f} MiB)")
    for n, (off, sz) in enumerate(mods):
        if sz < 1_000_000: continue
        print(f"\n===== module[{n}] {sz} bytes =====")
        base = off if not is_component else off
        assert buf[base:base+4] == b"\0asm", "core module header"
        tot = collections.Counter()
        code = None; names = {}; nimports = 0
        for sid, o, s in sections(buf, base+8, off+sz):
            tot[CORE_SECTIONS.get(sid, str(sid))] += s
            if sid == 10: code = (o, s)
            if sid == 0:
                nm, _ = name(buf, o)
                tot[f"custom:{nm}"] = tot.pop("custom", 0) + s if False else tot.get(f"custom:{nm}", 0) + s
                if nm == "name": names = func_names(buf, o, o+s)
            if sid == 2:
                j = o; cnt, j = leb(buf, j)
                for _ in range(cnt):
                    _m, j = name(buf, j); _f, j = name(buf, j)
                    kind = buf[j]; j += 1
                    if kind == 0:
                        _t, j = leb(buf, j); nimports += 1
                    elif kind == 1:
                        j += 1; lim = buf[j]; j += 1; _x, j = leb(buf, j)
                        if lim: _y, j = leb(buf, j)
                    elif kind == 2:
                        lim = buf[j]; j += 1; _x, j = leb(buf, j)
                        if lim: _y, j = leb(buf, j)
                    elif kind == 3:
                        j += 2
        for k, v in tot.most_common():
            print(f"{v:>12}  {v/1048576:>9.2f} MiB  {k}")
        if code:
            o, s = code
            j = o; cnt, j = leb(buf, j)
            print(f"\n  code: {cnt} functions, {s} bytes, {nimports} imported funcs")
            per = collections.Counter(); cper = collections.Counter(); cn = collections.Counter()
            biggest = []
            for k in range(cnt):
                bsz, j2 = leb(buf, j)
                hdr = j2 - j
                fn = names.get(nimports + k, f"func[{nimports+k}]")
                per[fn] += bsz + hdr
                c = crate_of(fn)
                cper[c] += bsz + hdr; cn[c] += 1
                biggest.append((bsz + hdr, fn))
                j = j2 + bsz
            print("\n  -- per crate (top 40) --")
            for c, v in cper.most_common(40):
                print(f"  {v:>12}  {v/1048576:>8.2f} MiB  {cn[c]:>7} fns  {c}")
            biggest.sort(reverse=True)
            print("\n  -- biggest 30 functions --")
            for v, fn in biggest[:30]:
                print(f"  {v:>12}  {fn[:170]}")

main(sys.argv[1])
