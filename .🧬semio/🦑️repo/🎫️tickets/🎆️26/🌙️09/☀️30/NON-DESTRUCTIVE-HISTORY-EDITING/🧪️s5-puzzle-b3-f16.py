"""🗺️ S5-PUZZLE wave B3, F16 — the `vortex` domain declares `HierarchyProvider::Topology` and puzzle 2d supplies it (nodes with
their handles, edges, target regions), with its law. `python3 <this file> --check | --apply | --restore`, all or nothing.
Withdrawn once (fix-forward 1) while the unit harness left a selection's UI progress undrained; re-landed after the harness
drains it.
"""

import importlib.util
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[6]
spec = importlib.util.spec_from_file_location("wave", HERE / "🧪️s5-puzzle-b3-guest.py")
wave = importlib.util.module_from_spec(spec)
spec.loader.exec_module(wave)
LAW = f"{wave.ED}/🧪️tests/🧪️board-tools/🦀️.rs"
staged = (wave.STAGED / "board-tools-law.rs").read_text(encoding="utf-8")
TOPOLOGY_LAW = "\n" + staged[staged.index("/// 🗺️ LAW (F16)") :]
HUNKS = [
    (wave.E, "/// `Flat` hierarchy (no parent/child structure was ever modeled for it).\n", "/// whose topology the app supplies (`interaction_topology`): nodes, their handles, edges and target regions.\n"),
    (wave.E, "hierarchy: HierarchyProvider::Flat,", "hierarchy: HierarchyProvider::Topology,"),
    (wave.E, "    /// 🪧️ A history-edit reference chip names its board entity as the outliner does", wave.TOPOLOGY + "    /// 🪧️ A history-edit reference chip names its board entity as the outliner does"),
    (wave.E, "/// tool, and a transient flush commits nothing.\n", "/// tool, a transient flush commits nothing, and the `vortex` topology names every board entity.\n"),
]
mode = sys.argv[1] if len(sys.argv) > 1 else ""
if mode not in ("--check", "--apply", "--restore"):
    sys.exit(__doc__)
texts = {}
for path, old, new in HUNKS:
    text = texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))
    old, new = (new, old) if mode == "--restore" else (old, new)
    if text.count(old) != 1:
        sys.exit(f"{path}: anchor resolves {text.count(old)} times, expected 1:\n{old[:120]}")
    texts[path] = text.replace(old, new)
law = (ROOT / LAW).read_text(encoding="utf-8")
if mode == "--restore":
    if law.count(TOPOLOGY_LAW) != 1:
        sys.exit(f"{LAW}: the topology law does not resolve")
    texts[LAW] = law.replace(TOPOLOGY_LAW, "")
else:
    if "LAW (F16)" in law:
        sys.exit(f"{LAW}: the topology law is already there")
    texts[LAW] = law + TOPOLOGY_LAW
if mode != "--check":
    for path, text in texts.items():
        (ROOT / path).write_text(text, encoding="utf-8")
print(f"{mode[2:]}: {len(texts)} files")
