"""📦 rw2t: external crate heads a law names that the destination manifest does not declare (with the dependency line to copy)."""
import sys, re, os, collections
from gen import *

def manifest_keys(suffix):
    mf = os.path.join(T, CRATES[suffix], "📦️packages/🦀️rust/Cargo.toml")
    keys = {}
    sec = None
    for line in read(mf).split("\n"):
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1); continue
        if sec and "dependencies" in sec and "=" in line and not line.lstrip().startswith("#"):
            k = line.split("=", 1)[0].strip()
            if re.match(r"^[A-Za-z0-9_\-]+$", k):
                keys.setdefault(k.replace("-", "_"), (sec, line))
    return keys

def all_lines():
    pool = {}
    for s in CRATES:
        for k, v in manifest_keys(s).items():
            pool.setdefault(k, v)
    return pool

def heads(text):
    c = blank(text)
    out = set()
    for m in re.finditer(r"(?<![\w:$])([a-z_][a-z0-9_]*)::", c):
        out.add(m.group(1))
    for cfg, vis, p, local, glob in uses(text):
        if p:
            out.add(p[0])
    return out

def missing(law, dest):
    text = read(LAWFILES[law])
    hs = heads(text)
    mk = manifest_keys(dest)
    ext_aliases = extern_aliases(dest) if dest in CRATES else {}
    loc = local_names(text)
    out = []
    for h in sorted(hs):
        if h in ("crate", "self", "super", "std", "core", "alloc") or h in loc:
            continue
        if h.startswith(PFX.rstrip("_")) :
            sfx = h[len(PFX):].replace("_", "-")
            if sfx in CRATES and sfx != dest and (PKG + sfx).replace("-", "_") not in mk and sfx not in closure(dest):
                out.append(("first-party", h, sfx))
            continue
        if h in mk or h in ext_aliases:
            continue
        out.append(("external", h, None))
    return out

if __name__ == "__main__":
    pool = all_lines()
    dest = sys.argv[1]
    register_crate(dest) if False else None
    for law in sys.argv[2:]:
        print("##", law)
        for kind, h, sfx in missing(law, dest):
            print("   ", kind, h, sfx or "", "|", pool.get(h, ("", "?"))[1][:140] if kind == "external" else "")
