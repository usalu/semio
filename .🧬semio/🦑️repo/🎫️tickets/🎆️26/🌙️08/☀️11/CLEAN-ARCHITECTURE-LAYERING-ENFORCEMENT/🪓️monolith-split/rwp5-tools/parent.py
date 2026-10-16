# 🔗️ Parent mount updater: regenerates the `mod` mounts and the SHELL_SOURCE include list of the WP5 staging modules from the files present in a working tree.
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, rsedit
from rsedit import Doc

def snake(s):
    s = s.split("️", 1)[-1] if "️" in s else s
    return re.sub(r"[^a-z0-9]+", "_", s.lower()).strip("_")

def mods_of(root):
    out = []
    for d, c, _ in cfg.CRATES:
        base = cfg.stg(root, d)
        if not os.path.isdir(base): continue
        for slug in sorted(os.listdir(base)):
            p = f"{base}/{slug}/🦀️.rs"
            if os.path.exists(p): out.append((d, c, slug, p))
    return out

def exports_items(path):
    doc = Doc(open(path, encoding="utf-8").read())
    for it in doc.items():
        if it["kind"] in ("impl", "use"): continue
        if it["kind"] == "mod": continue
        return True
    return False

MOUNT = re.compile(r'^#\[path = "📥️staging/(?P<d>[^/]+)/(?P<s>[^/]+)/🦀️\.rs"\]$')

def update(root):
    pp = f"{root}/{cfg.PARENT}"
    lines = open(pp, encoding="utf-8").read().split("\n")
    mine = set(cfg.DIR2CRATE)
    out = []; first = None; removed = 0
    i = 0
    while i < len(lines):
        m = MOUNT.match(lines[i])
        if m and m.group("d") in mine:
            if first is None: first = len(out)
            i += 2; removed += 1
            if i < len(lines) and re.match(r"^pub(?:\(crate\))? use staged_[a-z0-9_]+::\*;$", lines[i]): i += 1
            continue
        mi = re.match(r'^\s*include_str!\("📥️staging/([^/]+)/[^/]+/🦀️\.rs"\),$', lines[i])
        if mi and mi.group(1) in mine:
            if "inc_first" not in locals(): inc_first = len(out)
            i += 1; continue
        out.append(lines[i]); i += 1
    new_mounts = []; new_inc = []
    for d, c, slug, p in mods_of(root):
        name = f"staged_{snake(c.replace('shell-', ''))}__{snake(slug)}"
        new_mounts.append(f'#[path = "📥️staging/{d}/{slug}/🦀️.rs"]')
        new_mounts.append(f"mod {name};")
        if exports_items(p): new_mounts.append(f"pub use {name}::*;")
        new_inc.append(f'    include_str!("📥️staging/{d}/{slug}/🦀️.rs"),')
    out[first:first] = new_mounts
    k = inc_first + len(new_mounts)
    out[k:k] = new_inc
    open(pp, "w", encoding="utf-8").write("\n".join(out))
    return removed, len(new_mounts), len(new_inc)

if __name__ == "__main__":
    print(update(sys.argv[1] if len(sys.argv) > 1 else cfg.NEW))
