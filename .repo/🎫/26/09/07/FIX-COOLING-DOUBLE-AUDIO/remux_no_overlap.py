"""🔁 Remux Cooling full series VO with non-overlapping clause placement (no re-render)."""

from __future__ import annotations

import importlib.util
import re
import shutil
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = Path("/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio")
COOLING = ROOT / "tutorial" / "energy" / "demand" / "Cooling"
sys.path.insert(0, str(ROOT / "tutorial"))

# Import compose helpers by loading the previous orchestrator module.
orch = TICKET.parent / "MUX-COOLING-VO-SYNC" / "compose_cooling_with_vo.py"
spec = importlib.util.spec_from_file_location("compose_cooling_with_vo", orch)
mod = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(mod)

if __name__ == "__main__":
    final = mod.step_align_mux_concat()
    print("done", final)
