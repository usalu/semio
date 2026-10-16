"""g3geom: cut the fixture, calibration, harness, examples, scene-bridge and mesh-delivery regions of the example-geometry test target into include fragments."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from fragsplit import *

EMOJIS = {l.split("\t")[0] for l in read(F + "/🗑️generated/rf8-folder-emojis.tsv").split("\n") if l}
C = lambda **k: k

SPEC = dict(
    target="g3geom",
    children=[
        C(folder="📇️fixture", ranges=[(25, 175)], doc="📇️ Fixture: the committed expected-geometry statement types and the budget contract."),
        C(folder="⏱️calibration", ranges=[(176, 326)], doc="⏱️ Calibration: the deterministic machine-load calibration that scales the budget ceilings."),
        C(folder="🧰️harness", ranges=[(327, 646)], doc="🧰️ Harness: the serial lock, the evaluation run, the parry oracle and the timing helpers."),
        C(folder="🔬️examples", ranges=[(647, 688)], doc="🔬️ Examples: one evaluation law per bundled example."),
        C(folder="🌉️scene-bridge", ranges=[(689, 767)], doc="🌉️ Scene bridge: the provenance law of the framework-side scene-bridge fixture."),
        C(folder="🚚️mesh-delivery", ranges=[(768, 1230)], doc="🚚️ Mesh delivery: the preview mesh delivery through the browser wire, per example."),
    ],
)

if __name__ == "__main__":
    for c in SPEC["children"]:
        assert c["folder"].split("️")[0] + "️" in EMOJIS, c["folder"]
    run(SPEC)
