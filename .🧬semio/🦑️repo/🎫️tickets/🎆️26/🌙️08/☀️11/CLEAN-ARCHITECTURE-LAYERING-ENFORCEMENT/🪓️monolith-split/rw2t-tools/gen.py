"""🧬 rw2t: generator of the explicit import block of a WP2 law file (replaces `use super::*;`), with the crate:: head rewrites.
usage: gen.py <law> [--new-host <suffix>]  (prints the proposed block and the rewrites; --apply writes the law file)"""
import sys
from plan import *

PKG = PKG_PREFIX
WIDEN = []
def lib(suffix):
    return PFX + suffix.replace("-", "_")

def extern_aliases(suffix):
    root = ROOTS[suffix] if suffix in ROOTS else os.path.join(T, CRATES[suffix], "🦀️.rs")
    t = read(root)
    out = {}
    for m in re.finditer(r"extern crate (\w+)(?: as (\w+))?;", t):
        out[m.group(2) or m.group(1)] = m.group(1)
    return out

HEAD_CRATE = {  # old monolith root module -> crate suffix that owns it (plan 6.4)
    "element_contracts": "contracts", "host_base": "host-base", "media_slots": "media-slots", "asset_authority": "asset-authority",
    "canvas2d_gumball": "canvas2d-host", "canvas2d_paint": "canvas2d-paint", "native_platform": "native-platform",
}

def path_of(name, ix, host_suffix, allowed):
    cands = ix.get(name, [])
    cands = [c for c in cands if c[0] in allowed or c[0] == host_suffix]
    if not cands:
        return None, []
    cands.sort(key=lambda c: (RANK.get(c[0], 999), len(c[1])))
    suffix, mp, cf = cands[0]
    p = lib(suffix) + "::" + "::".join(mp + (name,))
    return p, cands

def build(name, ix, alias, new_host=None):
    path = LAWFILES[name]
    host = HOST[name]
    dest = new_host or host
    r = classify(name, ix, alias)
    text = read(path)
    allowed = closure(dest) if dest in CRATES else set()
    lines, notes = {}, []
    aliases_old = extern_aliases(host)
    for w, origin in r["own"].items():
        kind, v = origin
        if kind == "import":
            p = v
            head = p.split("::")[0]
            if head == "crate":
                rest = p.split("::")[1:]
                if rest[0] in aliases_old:
                    p = aliases_old[rest[0]] + "::" + "::".join(rest[1:]) if len(rest) > 1 else aliases_old[rest[0]]
                else:
                    p = (lib(host) + "::" + "::".join(rest)) if new_host and new_host != host else "crate::" + "::".join(rest)
            lines[w] = p
        else:
            if not new_host or new_host == host:
                lines[w] = "super::" + w
            else:
                pp, _ = path_of(w, ix, host, allowed)
                lines[w] = pp or (lib(host) + "::" + w)
                if not pp:
                    notes.append(f"WIDEN {w} (defined {v.split('/')[-2]})")
                    WIDEN.append((w, v))
    for w, lst in r["cross"].items():
        pp, c = path_of(w, ix, dest, allowed | {dest})
        if pp is None:
            notes.append(f"NO-OWNER-IN-DEPS {w} {[(a, b) for a, b, _ in lst]}")
            continue
        if len(c) > 1:
            notes.append(f"AMBIG {w} -> {[(a, b) for a, b, _ in c]} chose {pp}")
        lines[w] = pp
    for w, ps in r["ext"].items():
        lines[w] = ps[-1] if len(ps) > 1 and "ui_wgpu" in ps[-1] else ps[0]
        if len(ps) > 1:
            lines[w] = [p for p in ps if p.startswith("ui_wgpu")][0] if any(p.startswith("ui_wgpu") for p in ps) else ps[0]
    # heads
    rewrites = []
    body = text
    for m in sorted(set(re.findall(r"(?<![\w:])crate::((?:\w+::)*\w+)", blank(text)))):
        rewrites.append(m)
    return lines, notes, r, rewrites

def block(lines):
    by = {}
    for w, p in lines.items():
        if p.endswith("::" + w):
            by.setdefault(p[: -len(w) - 2], []).append(w)
        else:
            by.setdefault(p, []).append(None)
    out = []
    for pre in sorted(by):
        names = by[pre]
        if None in names:
            out.append(f"use {pre};")
        elif len(names) == 1:
            out.append(f"use {pre}::{names[0]};")
        else:
            out.append(f"use {pre}::{{{', '.join(sorted(names))}}};")
    return "\n".join(out)

if __name__ == "__main__":
    nm = sys.argv[1]
    nh = sys.argv[sys.argv.index("--new-host") + 1] if "--new-host" in sys.argv else None
    setup()
    if nh:
        register_crate(nh)
    ix = build_index()
    alias = alias_table(list(CRATES))
    lines, notes, r, rewrites = build(nm, ix, alias, nh)
    print(block(lines))
    print("--- notes"); [print(n) for n in notes]
    print("--- crate:: heads"); [print("  ", h) for h in rewrites]
