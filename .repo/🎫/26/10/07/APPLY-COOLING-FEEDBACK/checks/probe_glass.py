"""🔬 [DEBUG] Probe: why the guard misses the ray through the glazing label in Teil 4 Beat 4."""
from pathlib import Path as _Path
import sys as _sys

_REPO_ROOT = next(p for p in _Path(__file__).resolve().parents if (p / "tutorial" / "manim_fonts.py").is_file())
_sys.path.insert(0, str(_REPO_ROOT / "tutorial"))
_sys.path.insert(0, str(_REPO_ROOT / "tutorial/energy/demand/Cooling/4_solar_radiation"))

import manim_visuals as mv
from manim import Scene
import scene_4


class ProbeGlass(scene_4.Beat4_GlassTransmittance):
    def play(self, *args, **kwargs):
        result = Scene.play(self, *args, **kwargs)
        texts = mv._visible_texts(self)
        label = [t for t in texts if getattr(t, "text", "").startswith("2-fach")]
        roots0, glyphs0 = mv._label_roots(self)
        ray_on = any(type(g).__name__ == "Line" and mv._box(g)[3] > 2.6
                     for g, *_ in mv._visible_graphics(self, roots0, glyphs0))
        if label and ray_on and not getattr(self, "_probed", False):
            self._probed = True
            lab = label[0]
            print(f"[DEBUG] label box {mv._box(lab)}")
            roots, glyphs = mv._label_roots(self)
            for g, stroke, fill, root in mv._visible_graphics(self, roots, glyphs):
                b = mv._box(g)
                if b[1] > -6 and b[3] > 1.5:
                    print(f"[DEBUG] graphic {type(g).__name__} box={tuple(round(v, 2) for v in b)} stroke={stroke:.2f} width={g.get_stroke_width():.1f}")
            for issue in mv.layout_conflicts(self):
                print(f"[DEBUG] issue {issue}")
        return result
