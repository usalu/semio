"""rx: still of the Nakagin podium and capsule exterior, to verify no furnishing or figure is drawn ad hoc in tower views."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
_mod = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📋️program.🐍️")


class PodiumProbe(_mod.Beat3_Program):
    def construct(self):
        self._later, self._seconds = {"pre": [], "post": [], "trash": []}, None
        self.add(self.podium_axon(at=(0, 0, 0), scale=0.34))
        self.wait(0.1)
