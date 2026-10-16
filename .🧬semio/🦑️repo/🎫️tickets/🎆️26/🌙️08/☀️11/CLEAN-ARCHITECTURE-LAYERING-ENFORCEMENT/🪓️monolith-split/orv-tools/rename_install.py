"""Re-installs renamed children: for each (target, old folder, old mod) -> (new folder, new mod). Refuses unless the live root equals the new stage root after the textual mount rename."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC

RENAMES = {
    "neural-model": [("🏷️schema", "schema", "🏷️schema-model", "schema_model"), ("🌳️tree", "tree", "🌳️neuron-tree", "neuron_tree")],
    "io": [("🌉️dispatch", "dispatch", "🌉️compose-dispatch", "compose_dispatch"), ("🔌️wire", "wire", "🔌️wire-framing", "wire_framing")],
}
for name, rows in RENAMES.items():
    t = TARGETS[name]
    pk = f"{ROOT}/{t['pkgdir']}"
    stage = f"{W}/{name}/stage/{t['pkgdir']}"
    live = read(f"{pk}/🦀️.rs")
    exp = live
    for of, om, nf, nm in rows:
        old = f'#[path = "{of}/🦀️.rs"]\nmod {om};\npub use {om}::*;'
        new = f'#[path = "{nf}/🦀️.rs"]\nmod {nm};\npub use {nm}::*;'
        assert exp.count(old) == 1, (name, of)
        exp = exp.replace(old, new)
    new_root = read(f"{stage}/🦀️.rs")
    assert exp == new_root, f"{name}: live root differs from the new stage root beyond the rename"
    for c in SPEC[name]["children"]:
        f = c["folder"]
        if any(f == nf for _, _, nf, _ in rows):
            continue
        assert read(f"{pk}/{f}/🦀️.rs") == read(f"{stage}/{f}/🦀️.rs"), (name, f)
    for of, om, nf, nm in rows:
        write(f"{pk}/{nf}/🦀️.rs", read(f"{stage}/{nf}/🦀️.rs"))
    tmp = f"{pk}/🦀️.rs.orv-tmp"
    write(tmp, new_root)
    os.replace(tmp, f"{pk}/🦀️.rs")
    for of, om, nf, nm in rows:
        os.remove(f"{pk}/{of}/🦀️.rs")
        os.rmdir(f"{pk}/{of}")
    print(f"{name}: renamed {[(r[0], r[2]) for r in rows]}; root replaced")
