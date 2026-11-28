"""📏 Attribute code bytes of a core wasm module (or the largest core module inside a component) to crates.

Reads the `name` custom section and charges each defined function's body size to the crate that defines the function
(first path segment of the demangled symbol) and to the crate of the type it is instantiated for (first crate inside
`<...>`), then writes JSON plus a Markdown table of both views.
https://webassembly.github.io/spec/core/appendix/custom.html#name-section
"""
import json, re, sys, collections

def leb(b, i):
    r = s = 0
    while True:
        x = b[i]; i += 1; r |= (x & 0x7F) << s; s += 7
        if x < 0x80: return r, i

def sections(b, start, end):
    i = start
    while i < end:
        sid = b[i]; size, j = leb(b, i + 1); yield sid, j, j + size; i = j + size

def core_modules(b, start=8, end=None):
    end = len(b) if end is None else end
    if b[4:8] == b"\x01\x00\x00\x00" and start == 8:
        yield (start, end); return
    for sid, s, e in sections(b, start, end):
        if sid == 1: yield (s + 8, e)
        elif sid == 4: yield from core_modules(b, s + 8, e)

def analyse(b, start, end):
    imported = 0; bodies = []; names = {}
    for sid, s, e in sections(b, start, end):
        if sid == 2:
            n, i = leb(b, s)
            for _ in range(n):
                for _ in range(2):
                    l, i = leb(b, i); i += l
                k = b[i]; i += 1
                if k == 0: _, i = leb(b, i); imported += 1
                elif k == 1: i += 1; f, i = leb(b, i); _, i = leb(b, i); i = leb(b, i)[1] if f & 1 else i
                elif k == 2: f, i = leb(b, i); _, i = leb(b, i); i = leb(b, i)[1] if f & 1 else i
                elif k == 3: i += 2
                elif k == 4: i += 1; _, i = leb(b, i)
        elif sid == 10:
            n, i = leb(b, s)
            for _ in range(n):
                l, j = leb(b, i); bodies.append(j + l - i); i = j + l
        elif sid == 0:
            l, i = leb(b, s); nm = b[i:i + l].decode(); i += l
            if nm != "name": continue
            while i < e:
                sub = b[i]; size, j = leb(b, i + 1)
                if sub == 1:
                    n, k = leb(b, j)
                    for _ in range(n):
                        idx, k = leb(b, k); l2, k = leb(b, k); names[idx] = b[k:k + l2].decode("utf8", "replace"); k += l2
                i = j + size
    return imported, bodies, names

LEGACY = re.compile(r"(\d+)")

def demangle(sym):
    if not sym.startswith("_ZN"): return sym
    out = []; i = 3
    while i < len(sym) and sym[i].isdigit():
        m = LEGACY.match(sym, i); n = int(m.group(1)); i = m.end(); out.append(sym[i:i + n]); i += n
    t = "::".join(out)
    for a, z in (("$LT$", "<"), ("$GT$", ">"), ("$u20$", " "), ("$RF$", "&"), ("$C$", ","), ("..", "::"), ("$u7b$", "{"), ("$u7d$", "}")):
        t = t.replace(a, z)
    return t

def crate_of(name):
    n = name.lstrip("_<&* ")
    return re.split(r"::|<| |>", n, 1)[0] or "?"

def type_crate(name):
    m = re.search(r"<([^<>]*?)(?: as |>)", name)
    return crate_of(m.group(1)) if m else crate_of(name)

def main(path, out):
    b = open(path, "rb").read()
    mods = sorted(core_modules(b), key=lambda r: r[0] - r[1])
    s, e = mods[0]
    imported, bodies, names = analyse(b, s, e)
    by_owner = collections.Counter(); by_type = collections.Counter(); count = collections.Counter()
    for k, size in enumerate(bodies):
        d = demangle(names.get(imported + k, "?"))
        by_owner[crate_of(d)] += size; by_type[type_crate(d)] += size; count[crate_of(d)] += 1
    total = sum(bodies)
    report = {"file": path, "coreModules": len(mods), "coreBytes": e - s, "codeBytes": total, "functions": len(bodies),
              "byOwnerCrate": by_owner.most_common(), "byTypeCrate": by_type.most_common(), "functionsByOwnerCrate": count.most_common()}
    json.dump(report, open(out + ".json", "w"), indent=1)
    with open(out + ".md", "w") as f:
        f.write(f"# 📏 Code attribution of `{path.rsplit('/', 1)[-1]}`\n\ncore {e - s:,} B, code {total:,} B, {len(bodies):,} functions\n\n| defining crate | bytes | share | functions |\n|---|---:|---:|---:|\n")
        for c, v in by_owner.most_common(40): f.write(f"| `{c}` | {v:,} | {100 * v / total:.1f} % | {count[c]:,} |\n")
        f.write("\n| crate of the first type argument | bytes | share |\n|---|---:|---:|\n")
        for c, v in by_type.most_common(40): f.write(f"| `{c}` | {v:,} | {100 * v / total:.1f} % |\n")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
