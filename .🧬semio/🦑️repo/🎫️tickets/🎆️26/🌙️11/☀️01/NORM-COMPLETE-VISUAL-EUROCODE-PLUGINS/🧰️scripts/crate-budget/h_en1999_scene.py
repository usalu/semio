"""🪚 Crate-budget H: the EN 1999 scene core (kit, layout, members, handles, live scene) becomes the crate `-scene-core`; the scene keeps the diagrams."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from h_lib import *
ART = NORM + "/🗿️artifacts/🪶️en1999"
D = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene"
lower_core(ART, D, "semio-s-artifact-norm-en1999-scene", "core", {"📈️diagrams"}, "EN 1999 scene core: kit, layout, members, handles and the live scene", D + "/📦️packages/🦀️rust/Cargo.toml")
