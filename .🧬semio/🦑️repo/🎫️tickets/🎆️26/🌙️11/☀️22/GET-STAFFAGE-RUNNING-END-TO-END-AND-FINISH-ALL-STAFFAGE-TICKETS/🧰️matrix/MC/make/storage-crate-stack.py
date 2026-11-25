"""🧺️ Ops writer of `storage-crate-stack` (a leftover of batch MB-2, 28 `language-geometry` cells): the front section of the four bands was named by a phrase, "Longitudinal crate section" in English and "Kistenlängsschnitt" in German, where a section is named by its letters. The side elevation of 1:50 and 1:100 writes that name beside the two arrows of the cut line, so the scene was 4.18 mm wider on paper in English than in German and every shape lay 4.18 mm further right; the caption read "Section Longitudinal crate section". The label becomes A-A in both languages, as in the 19 other assets with a section: `python3 make/storage-crate-stack.py` writes `ops/storage-crate-stack.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("storage-crate-stack")
write("storage-crate-stack", [put(["representations", scale, "sections", "front", "label"], {"en": "A-A", "de": "A-A"}) for scale in scales(a) if "front" in a["representations"][scale].get("sections", {})])
