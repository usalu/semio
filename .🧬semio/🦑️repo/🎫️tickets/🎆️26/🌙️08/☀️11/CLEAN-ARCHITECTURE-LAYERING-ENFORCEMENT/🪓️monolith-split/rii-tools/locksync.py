"""🔒 rii: Cargo.lock dependency lists of the manifests this slice edited (rtb lock_sync). usage: locksync.py [--apply]"""
import os, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
os.environ["DRY"] = "0" if "--apply" in sys.argv else "1"
import register as RG
from lib import read, REPO
touched = [l.strip() for l in open(os.path.dirname(os.path.abspath(__file__)) + "/touched.txt", encoding="utf8") if l.strip().endswith("Cargo.toml")]
lock = RG.rd("Cargo.lock")
for rel in touched:
    text = read(REPO + "/" + rel)
    name = re.search(r'^name = "([^"]+)"', text, re.M).group(1)
    deps = RG.manifest_dep_packages(text)
    key = f'[[package]]\nname = "{name}"\n'
    if key not in lock:
        print(name.replace("semio-framework-os-renderer-wgpu-", ""), "NEW", sorted(deps))
        if "--apply" in sys.argv:
            multi = RG.lock_sync("Cargo.lock", name, deps, create=True)
            if multi: print("   multi-version:", multi)
        continue
    i = lock.index(key)
    j = lock.index("\n\n", i) + 2
    old = lock[i:j]
    olddeps = set(re.findall(r'^ "([^" ]+)(?: [^"]*)?",$', old, re.M))
    print(name.replace("semio-framework-os-renderer-wgpu-", ""), "+", sorted(deps - olddeps), "-", sorted(olddeps - deps))
    if "--apply" in sys.argv:
        multi = RG.lock_sync("Cargo.lock", name, deps)
        if multi: print("   multi-version:", multi)
