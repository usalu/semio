"""🔒 OHR Cargo.lock sync (rtb lock_sync): rewrites the dependency lists of the manifests this slice edited. usage: locksync.py [--apply]"""
import os, re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
os.environ["DRY"] = "0" if "--apply" in sys.argv else "1"
import register as RG
from lib import read, REPO
E_ = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T_ = E_ + "/🎯️targets/🧊️wgpu"
touched = [
    T_ + "/🛎️frame-actions/📦️packages/🦀️rust/Cargo.toml",
    T_ + "/🧾️frame-transaction/📦️packages/🦀️rust/Cargo.toml",
    T_ + "/📨️runtime-mailbox/📦️packages/🦀️rust/Cargo.toml",
    T_ + "/🍇️frame-tests-1/📦️packages/🦀️rust/Cargo.toml",
    T_ + "/🎪️host-winit/📦️packages/🦀️rust/Cargo.toml",
    E_ + "/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/Cargo.toml",
]
lock = RG.rd("Cargo.lock")
for rel in touched:
    text = read(REPO + "/" + rel)
    name = re.search(r'^name = "([^"]+)"', text, re.M).group(1)
    deps = RG.manifest_dep_packages(text)
    key = f'[[package]]\nname = "{name}"\n'
    if key not in lock:
        print(name, "NEW", sorted(deps))
        continue
    i = lock.index(key)
    j = lock.index("\n\n", i) + 2
    old = lock[i:j]
    olddeps = set(re.findall(r'^ "([^" ]+)(?: [^"]*)?",$', old, re.M))
    print(name.replace("semio-framework-os-", ""), "+", sorted(deps - olddeps), "-", sorted(olddeps - deps))
    if "--apply" in sys.argv:
        multi = RG.lock_sync("Cargo.lock", name, deps)
        if multi: print("   multi-version:", multi)
