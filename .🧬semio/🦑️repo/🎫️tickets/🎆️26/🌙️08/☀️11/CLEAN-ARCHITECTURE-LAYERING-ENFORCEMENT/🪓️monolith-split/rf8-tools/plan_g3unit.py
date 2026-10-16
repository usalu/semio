"""g3unit: cut the test clusters of the editor unit laws into include fragments; the module identity of every test stays `editor_domain::editor_laws::<fn>`."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from fragsplit import *

EMOJIS = {l.split("\t")[0] for l in read(F + "/🗑️generated/rf8-folder-emojis.tsv").split("\n") if l}
C = lambda **k: k

SPEC = dict(
    target="g3unit",
    children=[
        C(folder="🕹️command-surface", ranges=[(362, 829)], doc="🕹️ Command surface: window transient isolation, preview sharing, command round trips, declared actions and chords, and the document io surface."),
        C(folder="🪟️window-and-menu-laws", ranges=[(830, 1183)], doc="🪟️ Window and menu laws: action bridging, viewport decoding, installs, history, labels, context menus and window measures."),
        C(folder="🧮️engine-compute", ranges=[(1184, 1715)], doc="🧮️ Engine compute: preview payloads, mesh bridges, preview marks and graph projections."),
        C(folder="📏️surface-budget-and-retirement", ranges=[(1716, 1829)], doc="📏️ Surface budget and retirement: example select options, the resident surface bound and flow host retirement."),
        C(folder="🔁️extension-round-trip", ranges=[(1830, 2313)], doc="🔁️ Extension round trip: hex column evaluation, contribution pages, the settled tick cycle and late installs."),
        C(folder="📈️hot-path-and-fault-laws", ranges=[(2314, 2697)], doc="📈️ Hot path and fault laws: the boot budget, window action declarations, the evaluate fault law and staged argument labels."),
        C(folder="🪄️mesh-component-laws", ranges=[(2698, 3140)], doc="🪄️ Mesh component laws: the mesh gumball, component edits and quick actions."),
        C(folder="🚪️archive-and-brep-laws", ranges=[(3141, 3372)], doc="🚪️ Archive and brep laws: the archive import door, hub genesis and the brep measurement and scoped selection laws."),
    ],
)

if __name__ == "__main__":
    for c in SPEC["children"]:
        assert c["folder"].split("️")[0] + "️" in EMOJIS, c["folder"]
    run(SPEC)
