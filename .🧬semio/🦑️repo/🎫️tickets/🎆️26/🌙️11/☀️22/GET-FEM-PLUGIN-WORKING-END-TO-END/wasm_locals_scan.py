import sys, struct

def leb(b, i):
    r = 0; s = 0
    while True:
        x = b[i]; i += 1
        r |= (x & 0x7f) << s; s += 7
        if not x & 0x80: return r, i

def sections(b, start, end):
    i = start + 8
    while i < end:
        sid = b[i]; sz, j = leb(b, i + 1)
        yield sid, j, j + sz
        i = j + sz

def code_funcs(b, s, e):
    n, i = leb(b, s)
    for k in range(n):
        sz, j = leb(b, i)
        bs = i
        nd, p = leb(b, j)
        tot = 0
        for _ in range(nd):
            c, p = leb(b, p); tot += c; p += 1
        yield k, i, j, j + sz, tot
        i = j + sz

def names(b, s, e):
    out = {}
    for sid, ss, se in sections(b, s - 0, e) if False else []:
        pass
    return out

def func_names(b, start, end):
    res = {}; nimp = 0
    for sid, s, e in sections(b, start, end):
        if sid == 2:
            n, i = leb(b, s)
            for _ in range(n):
                for _ in range(2):
                    l, i = leb(b, i); i += l
                k = b[i]; i += 1
                if k == 0: nimp += 1; _, i = leb(b, i)
                elif k == 1: i += 1; f = b[i]; i += 1; _, i = leb(b, i); 
                elif k == 2: f = b[i]; i += 1; _, i = leb(b, i); 
                elif k == 3: i += 2
        if sid == 0:
            l, i = leb(b, s)
            nm = b[i:i + l]; i += l
            if nm == b'linking':
                v, i = leb(b, i)
                while i < e:
                    t = b[i]; sz, j = leb(b, i + 1)
                    if t == 8:
                        n, p = leb(b, j)
                        for _ in range(n):
                            kind = b[p]; p += 1; fl, p = leb(b, p)
                            und = fl & 0x10; expl = fl & 0x40
                            if kind == 1:
                                l2, p = leb(b, p); nm2 = b[p:p+l2].decode('utf8', 'replace'); p += l2
                                if not und: _, p = leb(b, p); _, p = leb(b, p); _, p = leb(b, p)
                            elif kind == 3:
                                _, p = leb(b, p)
                            else:
                                idx, p = leb(b, p)
                                if (not und) or expl:
                                    l2, p = leb(b, p); nm2 = b[p:p+l2].decode('utf8', 'replace'); p += l2
                                    if kind == 0: res.setdefault(idx, nm2)
                    i = j + sz
            if nm == b'name':
                while i < e:
                    sub = b[i]; sz, j = leb(b, i + 1)
                    if sub == 1:
                        n, p = leb(b, j)
                        for _ in range(n):
                            idx, p = leb(b, p); l2, p = leb(b, p)
                            res[idx] = b[p:p + l2].decode('utf8', 'replace'); p += l2
                    i = j + sz
    return res, nimp

def scan_module(b, start, end, label, thr, off=None):
    names_, nimp = func_names(b, start, end)
    for sid, s, e in sections(b, start, end):
        if sid == 10:
            for k, fs, bs, be, tot in code_funcs(b, s, e):
                hit = off is not None and fs <= off < be
                if tot >= thr or hit:
                    print(f"{label}\tlocals={tot}\tbytes={be-bs}\tstart=0x{fs:x}\t{'HIT ' if hit else ''}{names_.get(nimp+k,'?')}")

if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'wasm':
        b = open(sys.argv[2], 'rb').read()
        off = int(sys.argv[3], 16)
        for sid, s, e in sections(b, 0, len(b)):
            if sid == 1:
                scan_module(b, s, e, 'core', 10**9, off)

def demangle(n):
    import re
    if n.startswith('_ZN') or n.startswith('_R'):
        parts = re.findall(r'(\d+)([A-Za-z_][A-Za-z0-9_$.]*)', n[3:])
        out = []
        for ln, rest in parts:
            seg = rest[:int(ln)] if len(rest) >= int(ln) else rest
            out.append(seg)
        return '::'.join(x for x in out if not (x.startswith('h') and len(x) == 17))
    return n

def ar_members(b):
    i = 8
    while i + 60 <= len(b):
        name = b[i:i+16].decode('latin1').strip(); sz = int(b[i+48:i+58].decode().strip())
        i += 60
        yield name, i, i + sz
        i += sz + (sz & 1)

def scan_rlib(path, thr):
    b = open(path, 'rb').read()
    for name, s, e in ar_members(b):
        if b[s:s+4] == b'\0asm':
            try:
                nm, nimp = func_names(b, s, e)
                for sid, ss, se in sections(b, s, e):
                    if sid == 10:
                        for k, fs, bs, be, tot in code_funcs(b, ss, se):
                            if tot >= thr:
                                print(f"{path}\tlocals={tot}\tbytes={be-bs}\t{nm.get(nimp+k,"?")}  [{demangle(nm.get(nimp+k,"?"))}]")
            except Exception as ex:
                print('ERR', path, name, ex, file=sys.stderr)

if __name__ == '__main__' and sys.argv[1] == 'rlib':
    import glob
    thr = int(sys.argv[2])
    for p in sys.argv[3:]:
        scan_rlib(p, thr)
