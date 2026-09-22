"""🧮️ Attribute a core module's code+name bytes to defining crate and to the norm artifact crates that drive the instantiation."""
import sys, re, collections

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

CRATE = re.compile(r"Cs[0-9A-Za-z]{1,20}_(\d+)([A-Za-z_][A-Za-z0-9_]*)")
LEGACY = re.compile(r"^_ZN(\d+)([A-Za-z_][A-Za-z0-9_]*)")

def crates(fn):
    out = []
    for m in CRATE.finditer(fn):
        n = int(m.group(1)); ident = m.group(2)[:n]
        if len(m.group(2)) >= n: out.append(ident)
    if not out:
        m = LEGACY.match(fn)
        if m:
            n = int(m.group(1)); out.append(m.group(2)[:n])
    return out

def main(path):
    buf = open(path, "rb").read()
    mods = []
    for sid, off, sz in sections(buf, 8, len(buf)):
        if sid == 1: mods.append((off, sz))
    off, sz = max(mods, key=lambda m: m[1])
    names = {}; code = None; nimports = 0; namesz = 0
    for sid, o, s in sections(buf, off+8, off+sz):
        if sid == 10: code = (o, s)
        if sid == 0:
            nm, _ = name(buf, o)
            if nm == "name": names = func_names(buf, o, o+s); namesz = s
        if sid == 2:
            j = o; cnt, j = leb(buf, j)
            for _ in range(cnt):
                _m, j = name(buf, j); _f, j = name(buf, j)
                kind = buf[j]; j += 1
                if kind == 0: _t, j = leb(buf, j); nimports += 1
                elif kind == 1:
                    j += 1; lim = buf[j]; j += 1; _x, j = leb(buf, j)
                    if lim: _y, j = leb(buf, j)
                elif kind == 2:
                    lim = buf[j]; j += 1; _x, j = leb(buf, j)
                    if lim: _y, j = leb(buf, j)
                elif kind == 3: j += 2
    o, s = code
    j = o; cnt, j = leb(buf, j)
    defc = collections.Counter(); defn = collections.Counter()
    art = collections.Counter(); artn = collections.Counter(); artname = collections.Counter()
    namebytes = collections.Counter()
    shared = 0; sharedn = 0; sharedname = 0
    total_name_bytes = 0
    for k in range(cnt):
        bsz, j2 = leb(buf, j)
        tot = bsz + (j2 - j)
        fn = names.get(nimports + k, "")
        nb = len(fn) + 2
        total_name_bytes += nb
        cs = crates(fn)
        d = cs[0] if cs else "<unnamed>"
        defc[d] += tot; defn[d] += 1
        norms = sorted({c for c in cs if c.startswith("semio_s_artifact_norm_") or c == "semio_s_plugin_norm"})
        if norms:
            key = norms[0] if len(norms) == 1 else "+".join(norms[:3]) + ("…" if len(norms) > 3 else "")
            art[key] += tot; artn[key] += 1; artname[key] += nb
        else:
            shared += tot; sharedn += 1; sharedname += nb
        namebytes[d] += nb
        j = j2 + bsz
    print(f"functions {cnt}  code {s} bytes  name-section {namesz} bytes  name-bytes-of-func-names {total_name_bytes}")
    print(f"\n== code+name attributed to a norm artifact crate vs shared ==")
    A = sum(art.values()); AN = sum(artname.values())
    print(f"  norm-artifact-tagged : {A:>12} code ({A/1048576:.2f} MiB) {sum(artn.values()):>7} fns, names {AN} ({AN/1048576:.2f} MiB)")
    print(f"  shared/untagged      : {shared:>12} code ({shared/1048576:.2f} MiB) {sharedn:>7} fns, names {sharedname} ({sharedname/1048576:.2f} MiB)")
    print("\n== per norm artifact crate (code bytes, fns, name bytes) ==")
    for c, v in art.most_common(40):
        print(f"  {v:>11} {v/1048576:>7.2f} MiB {artn[c]:>7} fns  names {artname[c]/1048576:>7.2f} MiB  {c}")
    print("\n== per DEFINING crate (top 40) ==")
    for c, v in defc.most_common(40):
        print(f"  {v:>11} {v/1048576:>7.2f} MiB {defn[c]:>7} fns  names {namebytes[c]/1048576:>7.2f} MiB  {c}")

main(sys.argv[1])
