"""rx: still of the excel program plan with occupants (type A big plan, then the four small plans)."""
import importlib
import sys
from pathlib import Path

import numpy as np

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
_mod = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📋️program.🐍️")
_nk = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
_engine = _mod._engine


class PlanProbe(_mod.Beat3_Program):
    def construct(self):
        self.lang = self.language()
        self._later, self._seconds = {"pre": [], "post": [], "trash": []}, None
        self.types = {t.key: t for t in _nk.capsule_types()}
        big = self.plan(self.types["A"], _mod.PLAN_AT, _mod.PLAN_SCALE, people=True)
        parts = [big, self.outlines(big), big.people]
        for index, key in enumerate(self.types):
            at = np.array([-5.5 + index * 1.4, 0.0, 0.0])
            small = self.plan(self.types[key], at, 0.42, label=False, people=True)
            parts += [small, self.outlines(small), small.people]
        self.add(*parts)
        self.wait(0.1)
