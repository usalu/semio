"""🔌 Print the imports, exports and custom sections of a core wasm module as JSON lines (kind, module, name).

https://webassembly.github.io/spec/core/binary/modules.html
"""
import json, sys

KINDS = ("func", "table", "memory", "global", "tag")

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7F) << s; s += 7
        if x < 0x80: return r, i

def name(b, i):
    l, i = leb(b, i); return b[i:i + l].decode("utf8", "replace"), i + l

def limits(b, i):
    f, i = leb(b, i); _, i = leb(b, i)
    return leb(b, i)[1] if f & 1 else i

def main(path):
    b = open(path, "rb").read(); i = 8
    while i < len(b):
        sid = b[i]; size, j = leb(b, i + 1); end = j + size
        if sid == 0:
            n, _ = name(b, j); print(json.dumps({"section": "custom", "name": n, "bytes": size}))
        elif sid == 2:
            n, k = leb(b, j)
            for _ in range(n):
                m, k = name(b, k); f, k = name(b, k); kind = b[k]; k += 1
                if kind == 0: _, k = leb(b, k)
                elif kind == 1: k = limits(b, k + 1)
                elif kind == 2: k = limits(b, k)
                elif kind == 3: k += 2
                elif kind == 4: k += 1; _, k = leb(b, k)
                print(json.dumps({"section": "import", "kind": KINDS[kind], "module": m, "name": f}, ensure_ascii=False))
        elif sid == 7:
            n, k = leb(b, j)
            for _ in range(n):
                f, k = name(b, k); kind = b[k]; k += 1; _, k = leb(b, k)
                print(json.dumps({"section": "export", "kind": KINDS[kind], "name": f}, ensure_ascii=False))
        i = end

if __name__ == "__main__":
    main(sys.argv[1])
