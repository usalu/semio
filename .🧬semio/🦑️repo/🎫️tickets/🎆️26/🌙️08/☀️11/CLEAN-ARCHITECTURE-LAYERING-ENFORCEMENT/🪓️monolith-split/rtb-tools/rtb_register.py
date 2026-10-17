"""📋️ Registers the new rtb crates and syncs the lock and the external-dependency users for every manifest the slice created or edited.
usage: rtb_register.py plan|apply"""
import json, os, sys
from lib import *
import register as RG
import manifest

APPLY = "apply" in sys.argv
NEW = [("fixtures", "🧰️fixtures"), ("frame-tests-1", "🍇️frame-tests-1")]
NEW_CHILDREN = ["🧰️fixtures", "🐚️shell", "📜️repo-text"]


def pkg_dir(folder):
    return rel(T + "/" + folder + "/📦️packages/🦀️rust")


def touched_manifests():
    """Manifests under the renderer tree that differ from the pre-edit snapshot or are new."""
    out = []
    for suffix, folder in crate_table().items():
        mp = T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml"
        snap = SNAP + "/E/🎯️targets/🧊️wgpu/" + folder + "/📦️packages/🦀️rust/Cargo.toml"
        if not os.path.exists(snap) or read(snap) != read(mp):
            out.append((suffix, mp))
    mp = T + "/📦️packages/🦀️rust/Cargo.toml"
    snap = SNAP + "/E/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml"
    if read(snap) != read(mp):
        out.append(("top", mp))
    return out


def main():
    crates = crate_table()
    for suffix, folder in NEW:
        pd = pkg_dir(folder)
        print("member", suffix, RG.add_member(pd) if APPLY else "(dry)")
        name = f"@semio-tech/os-renderer-wgpu-{suffix}-rs"
        print("owner", name, RG.add_owner(name, pd, pd + "/📋️project.json") if APPLY else "(dry)")
    if APPLY:
        RG.add_routes([(PKG_PREFIX + s, f"@semio-tech/os-renderer-wgpu-{s}-rs") for s, f in NEW])
        print("taxonomy members", RG.add_taxonomy_members([f for s, f in NEW]))
        print("taxonomy children", RG.add_taxonomy_children(NEW_CHILDREN))
    multi_all = {}
    for suffix, mp in touched_manifests():
        name = re.search(r'^name = "([^"]+)"', read(mp), re.M).group(1)
        deps = RG.manifest_dep_packages(read(mp))
        if APPLY:
            multi = RG.lock_sync("Cargo.lock", name, deps, create=suffix in dict(NEW))
            if multi:
                multi_all[name] = multi
        else:
            print("lock", name, len(deps))
    if multi_all:
        print("multi-version dependencies to review:", multi_all)


if __name__ == "__main__":
    main()
