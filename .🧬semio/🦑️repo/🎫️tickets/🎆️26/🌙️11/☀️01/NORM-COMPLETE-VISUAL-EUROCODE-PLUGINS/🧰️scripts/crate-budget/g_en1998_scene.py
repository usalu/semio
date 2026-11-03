"""🪚 Crate-budget G: EN 1998 scene `response`, `capacity` and `geotechnical` become the crate `-scene-analysis` (root `🔬️analysis`); the scene re-exports them."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
ART = NORM + "/🗿️artifacts/🫨️en1998"
SC = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
P = "semio-s-artifact-norm-en1998-scene"
N = P + "-analysis"
dev = open(SC + "/📦️packages/🦀️rust/Cargo.toml", encoding="utf-8").read().split("[dev-dependencies]\n")[1]
new_crate(ART, SC + "/🔬️analysis", N, "EN 1998 scene analysis: seismic response, capacity and geotechnical diagrams", ["semio-s-artifact-norm-en1998-model", "semio-s-artifact-norm-en1998-schema", "semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene", "semio-s-artifact-norm-en1998-scene-structures"], P, SC + "/📦️packages/🦀️rust/Cargo.toml", dev=dev)
add_dep(SC + "/📦️packages/🦀️rust/Cargo.toml", N)
