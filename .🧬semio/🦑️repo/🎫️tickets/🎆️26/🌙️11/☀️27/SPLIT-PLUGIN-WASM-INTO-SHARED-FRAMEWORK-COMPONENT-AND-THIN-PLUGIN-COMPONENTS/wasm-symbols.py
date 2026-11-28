"""🔣 Print the undefined data symbols of a linked module's `linking` symbol table and the relocations that reference them (wasm-ld --emit-relocs output).

https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md
"""
import json, sys

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7F) << s; s += 7
        if x < 0x80: return r, i

def sleb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7F) << s; s += 7
        if x < 0x80:
            return (r - (1 << s) if x & 0x40 else r), i

def name(b, i):
    l, i = leb(b, i); return b[i:i + l].decode("utf8", "replace"), i + l

def main(path):
    b = open(path, "rb").read(); i = 8; symbols = []; relocs = {}
    while i < len(b):
        sid = b[i]; size, j = leb(b, i + 1); end = j + size
        if sid == 0:
            n, k = name(b, j)
            if n == "linking":
                _, k = leb(b, k)
                while k < end:
                    sub = b[k]; ssize, k = leb(b, k + 1); send = k + ssize
                    if sub == 8:
                        count, k = leb(b, k)
                        for _ in range(count):
                            kind = b[k]; flags, k = leb(b, k + 1)
                            undefined = bool(flags & 0x10); explicit = bool(flags & 0x40)
                            if kind in (0, 2, 4, 5):
                                index, k = leb(b, k)
                                nm = None
                                if not undefined or explicit: nm, k = name(b, k)
                                symbols.append({"kind": kind, "undefined": undefined, "name": nm})
                            elif kind == 1:
                                nm, k = name(b, k)
                                if not undefined:
                                    _, k = leb(b, k); _, k = leb(b, k); _, k = leb(b, k)
                                symbols.append({"kind": 1, "undefined": undefined, "name": nm})
                            elif kind == 3:
                                _, k = leb(b, k); symbols.append({"kind": 3, "undefined": False, "name": None})
                    k = send
            elif n.startswith("reloc."):
                _, k = leb(b, k); count, k = leb(b, k); rows = []
                for _ in range(count):
                    ty = b[k]; off, k = leb(b, k + 1); sym, k = leb(b, k)
                    add = 0
                    if ty in (3, 4, 5, 8, 9, 11, 14, 15, 16, 17, 21, 22, 23): add, k = sleb(b, k)
                    rows.append((ty, off, sym, add))
                relocs[n] = rows
        i = end
    undefined_data = {k for k, s in enumerate(symbols) if s["kind"] == 1 and s["undefined"]}
    for section, rows in relocs.items():
        hits = [r for r in rows if r[2] in undefined_data]
        print(json.dumps({"section": section, "relocations": len(rows), "againstUndefinedData": len(hits), "types": sorted({r[0] for r in hits}), "symbols": sorted({symbols[r[2]]["name"] for r in hits})[:20]}, ensure_ascii=False))
    print(json.dumps({"symbols": len(symbols), "undefinedData": len(undefined_data), "undefinedFunctions": sum(1 for s in symbols if s["kind"] == 0 and s["undefined"])}))

if __name__ == "__main__":
    main(sys.argv[1])
