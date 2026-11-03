"""🐍 bp: _Motion measures only the figure (probes) when the cast index carries figureBoundsM."""
from pathlib import Path

path = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py"
text = path.read_bytes().decode("utf8")
pairs = [
    ('        bounds = index["boundsM"]\n        x0, y0, x1, y1 = window or', '        bounds = index.get("figureBoundsM", index["boundsM"])\n        self._figure_only = "figureBoundsM" in index\n        x0, y0, x1, y1 = window or'),
    ('    @staticmethod\n    def _frame_of(probes)', '    def get_points_defining_boundary(self):\n        """📏 The figure alone: the air layer of a transport cast reaches far beyond the person and must not move the top, centre or height of the group."""\n        if self._figure_only:\n            return np.array([probe.get_center() for probe in self.probes])\n        return super().get_points_defining_boundary()\n\n    @staticmethod\n    def _frame_of(probes)'),
]
for old, new in pairs:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
path.write_bytes(text.encode("utf8"))
