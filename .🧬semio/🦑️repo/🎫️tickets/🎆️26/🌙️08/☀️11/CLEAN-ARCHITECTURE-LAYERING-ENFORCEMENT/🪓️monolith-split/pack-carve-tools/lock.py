"""🔒 Computes the Cargo.lock entries the new pack packages add (hand-derived from the manifests; applied only when asked)."""
import re, sys
from lib import *
from crates import *

def dep_names(manifest_text, dev=True):
    names = set(); sect = None
    for l in manifest_text.split("\n"):
        s = l.strip()
        if s.startswith("["): sect = s; continue
        if sect in (("[dependencies]", "[dev-dependencies]") if dev else ("[dependencies]",)) and "=" in s and not s.startswith("#"):
            names.add(s.split("=")[0].strip())
    return sorted(names)

def block(name, deps):
    return '[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s]\n' % (name, "".join(' "%s",\n' % d for d in deps))

def new_blocks(dev=True):
    out = {}
    for k, *_ in CRATES:
        out[cname(k)] = block(cname(k), dep_names(read_stage_or_base(pkgdir(k) + "/Cargo.toml"), dev))
    out["semio-framework-pack"] = block("semio-framework-pack", dep_names(read_stage_or_base(P + "📦️packages/🦀️rust/Cargo.toml"), dev))
    return out

def apply_lock(text, blocks, check_deps=True):
    parts = text.split("\n[[package]]\n")
    head = parts[0]; pkgs = ["[[package]]\n" + p for p in parts[1:]]
    def pname(b): return re.search(r'name = "([^"]+)"', b).group(1)
    have = {pname(b): i for i, b in enumerate(pkgs)}
    if check_deps:
        for n, b in blocks.items():
            for d in re.findall(r'^ "([^"]+)",$', b, re.M):
                assert d in have or d in blocks, ("lock lacks dependency", n, d)
    for n, b in blocks.items():
        b = b.rstrip("\n") + "\n"
        if n in have:
            old = pkgs[have[n]]
            pkgs[have[n]] = b + ("\n" if old.endswith("\n\n") else "")
        else:
            pkgs.append(b + "\n")
    # keep packages sorted by name like cargo does (stable for equal names)
    def key(b): return pname(b).encode()
    pkgs.sort(key=key)
    pkgs = [p if p.endswith("\n\n") else p.rstrip("\n") + "\n\n" for p in pkgs]
    pkgs[-1] = pkgs[-1].rstrip("\n") + "\n"
    return head + "\n" + "".join(pkgs)
