"""📋 Summarize a modowners tsv: file, owners (package names), skip class."""
import sys, re, os
ROOT = "/Users/ueli/Documents/semio"
def pkgname(d):
    try:
        t=open(f"{ROOT}/{d}/Cargo.toml").read()
        return re.search(r'\[package\][^\[]*?name\s*=\s*"([^"]+)"',t,re.S).group(1)
    except Exception:
        return "?"+d[-30:]
def klass(f, pkgs):
    allp = " ".join([f]+pkgs)
    if "🔌️plugin/🪝️mounts/" in allp: return "pi1"
    if "🗄️stdio/🗿️artifacts/" in allp: return "pch"
    if any(x in allp for x in ["/🀄️wfc/","/🌀️procedural/","/🧩️puzzle/","/🪐️space/🗿️"]) and "✏️s/" in allp: return "pch"
    if "🧪️testing" in f and "🏪️store" in f: return "okwp8"
    return "mine"
out=[]
for line in open(sys.argv[1]):
    f,n,pk,sites = line.rstrip("\n").split("\t")
    pkgs = pk.split(" ; ")
    names=[pkgname(p) for p in pkgs]
    out.append((klass(f,pkgs), f, names, sites))
for c,f,names,sites in sorted(out):
    print(c, "|", f.replace("📦️packages/🦀️rust","PKG"), "|", " ; ".join(names))
