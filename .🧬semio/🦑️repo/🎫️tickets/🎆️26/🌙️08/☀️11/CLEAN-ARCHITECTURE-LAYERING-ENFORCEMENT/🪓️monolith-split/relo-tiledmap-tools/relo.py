"""🧭️ Patched full-resolution differential for the tiled-map GIS relocation (slice B): reuses harden2-tools (neutral sandbox, ho.rustc) with its own crate specs.

Follow-up specs (tmh, tmm, scn, sty, rw; or RELO_FOLLOW_UP=1 for tm/gm) restore every file of relo-tiledmap-baseline-fu for the base tree.

Usage: HO_WORK=<scratch> [RELO_FOLLOW_UP=1] python3 relo.py snap <t> <base|split>   |   python3 relo.py cmp <t> <cfg,cfg>
"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "harden2-tools"))
import neutral, ho, mksbx, shutil

_mk = mksbx.mk


def _mk_with_absent(sbx, overrides):
    """🩹️ `mksbx.mk` only places overrides whose path still exists in the repository; place the rest (files this slice deleted) too."""
    _mk(sbx, overrides)
    for rel, src in overrides.items():
        dst = os.path.join(sbx, rel)
        if not os.path.lexists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy(src, dst)


mksbx.mk = _mk_with_absent

REPO = "/Users/ueli/Documents/semio"
WORK = os.environ["HO_WORK"]
SPEC = {
    "tm": {"crate": "semio_framework_surface", "ct": "lib", "root": "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🦀️.rs", "fam": None},
    "gm": {"crate": "semio_s_artifact_gis_gismap", "ct": "lib", "root": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs", "fam": None},
    "tmh": {"crate": "semio_framework_surface_tiled_map", "ct": "lib", "root": "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs", "fam": None},
    "tmm": {"crate": "semio_framework_surface_tiled_map_model", "ct": "lib", "root": "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧮️model/🦀️.rs", "fam": None},
    "scn": {"crate": "semio_framework_ui_scene", "ct": "lib", "root": "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️.rs", "fam": None},
    "sty": {"crate": "semio_framework_ui_styling", "ct": "lib", "root": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🦀️.rs", "fam": None},
    "rw": {"crate": "semio_framework_os_renderer_wgpu", "ct": "lib", "root": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/🦀️.rs", "fam": None},
}
FOLLOW_UP = {"tmh", "tmm", "scn", "sty", "rw"}


BASELINE = os.path.join(HERE, "..", "🗑️generated", "relo-tiledmap-baseline")
BASELINE_FU = os.path.join(HERE, "..", "🗑️generated", "relo-tiledmap-baseline-fu")


def pristine_tree(t):
    """🧱️ The current repository with every file this slice edited restored from the pristine baseline copy (peer edits elsewhere stay live)."""
    follow_up = t in FOLLOW_UP or os.environ.get("RELO_FOLLOW_UP") == "1"
    prefix = "" if follow_up else {"tm": "🧰️framework/🔨️modules/🗺️surface/", "gm": "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/"}[t]
    base = os.path.realpath(BASELINE_FU if follow_up else BASELINE)
    files = [rel for rel in (os.path.relpath(os.path.join(d, f), base) for d, _, fs in os.walk(base) for f in fs) if os.sep in rel]
    overrides = {rel: os.path.join(base, rel) for rel in files if rel.startswith(prefix) and os.path.exists(os.path.join(REPO, rel))}
    raw = os.path.join(WORK, "raw-" + t)
    mksbx.mk(raw, overrides)
    for rel in files:
        if rel.startswith(prefix) and not os.path.exists(os.path.join(REPO, rel)):
            dst = os.path.join(raw, rel)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copy(os.path.join(base, rel), dst)
    return raw


def snap(t, which):
    src = pristine_tree(t) if which == "base" else REPO
    neutral.run(t + "-" + which, src, SPEC[t]["root"], os.path.join(WORK, "nsbx-" + t + "-" + which))


def cmp(t, cfgs):
    s = SPEC[t]
    for cfg in cfgs:
        extra = ho.CFG[cfg]
        res = {}
        for w in ("base", "split"):
            root = os.path.join(WORK, "nsbx-" + t + "-" + w, s["root"])
            res[w] = ho.rustc(root, s["crate"], s["ct"], extra, os.path.join(WORK, "out-" + t + "-" + cfg + "-" + w), s["fam"])
        a, b = res["base"][0], res["split"][0]
        more, less = b - a, a - b
        print(f"[{t} {cfg}] base={ho.total(a)} split={ho.total(b)} nonext base={ho.total(ho.nonext(a))} split={ho.total(ho.nonext(b))} | split-only={ho.total(more)} base-only={ho.total(less)}")
        for k, n in sorted(more.items()):
            print("  +SPLIT", n, tuple(str(x)[:160] for x in k))
        for k, n in sorted(less.items()):
            print("  -BASE ", n, tuple(str(x)[:160] for x in k))


if __name__ == "__main__":
    if sys.argv[1] == "snap":
        snap(sys.argv[2], sys.argv[3])
    else:
        cmp(sys.argv[2], sys.argv[3].split(","))
