import sys

def leb(b, i):
    r = s = 0
    while True:
        c = b[i]; i += 1
        r |= (c & 0x7f) << s; s += 7
        if c < 0x80:
            return r, i

def counts(path):
    b = open(path, "rb").read()
    i, imports, functions = 8, 0, 0
    while i < len(b):
        sid = b[i]; size, j = leb(b, i + 1)
        if sid == 2:
            n, k = leb(b, j)
            for _ in range(n):
                for _ in range(2):
                    l, k = leb(b, k); k += l
                kind = b[k]; k += 1
                if kind == 0:
                    imports += 1; _, k = leb(b, k)
                elif kind == 1:
                    k += 1; f, k = leb(b, k); _, k = leb(b, k) if f & 1 else (0, k)
                elif kind == 2:
                    f, k = leb(b, k); _, k = leb(b, k)
                    if f & 1: _, k = leb(b, k)
                elif kind == 3:
                    k += 2
                else:
                    k += 2
        if sid == 3:
            functions, _ = leb(b, j)
        i = j + size
    return imports, functions

for p in sys.argv[1:]:
    print(p, counts(p))
