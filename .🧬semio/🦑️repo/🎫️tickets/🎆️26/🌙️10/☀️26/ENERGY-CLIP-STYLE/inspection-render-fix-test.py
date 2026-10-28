"""🧪 Reproduce and independently check the ventilation house-to-scale transform."""
from __future__ import annotations

import ast
import copy
import importlib
import json
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/iv/fix"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "cache")
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True


def main():
    """🔬 Drive the real Manim interpolation boundary without rendering unrelated scenes."""
    import numpy as np
    from manim import Dot, FadeIn, Line, ReplacementTransform, VGroup, ValueTracker, tempconfig
    from manim.animation.animation import prepare_animation

    module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🐍️")
    origin = np.array([-6.3, -1.3, 0.0])

    def house():
        """🏠 Build the same section assembly and frozen air fill as the failing beat."""
        physical = module._pf_house(module.HOUSE, origin, 0.36)
        fill = module._poly(module.HOUSE["outer"], origin, 0.36, stroke_width=0,
                            fill_color=module.P_BLUE, fill_opacity=0.45)
        return VGroup(physical["group"], fill, Dot()).scale_to_fit_height(1.35).move_to([-3.0, 0.4, 0.0])

    def drive(animation):
        """🎞️ Exercise copies, updater ticks and all interpolated topology with Manim itself."""
        animation = prepare_animation(animation)
        animation.begin()
        for alpha in np.linspace(0, 1, 5):
            animation.update_mobjects(0.25)
            animation.interpolate(float(alpha))
        animation.finish()

    with tempconfig({"media_dir": str(OUTPUT / "media"), "text_dir": str(OUTPUT / "text")}):
        baseline_error = None
        try:
            drive(ReplacementTransform(house(), Dot([0.6, 0, 0])))
        except ValueError as error:
            baseline_error = str(error)
        assert baseline_error and "broadcast" in baseline_error, ("Expected original dynamic-topology failure", baseline_error)
        frozen = house()
        updater_count = sum(len(member.updaters) for member in frozen.get_family())
        assert updater_count > 0
        frozen.clear_updaters()
        assert not any(member.updaters for member in frozen.get_family())
        drive(ReplacementTransform(frozen, Dot([0.6, 0, 0])))

        source = Path(module.__file__).read_text(encoding="utf-8")
        tree = ast.parse(source)
        beat = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat2_Luftwechselrate")
        construct = next(node for node in beat.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
        show_use = next(node for node in construct.body if isinstance(node, ast.FunctionDef) and node.name == "show_use")
        extracted = ast.fix_missing_locations(ast.Module(body=[copy.deepcopy(show_use)], type_ignores=[]))

        class Driver:
            """🎭 Execute the actual production nested function through real Manim animations."""

            def play(self, *animations, **kwargs):
                for animation in animations:
                    drive(animation)

            def add(self, *objects):
                return self

        glyph = house()
        namespace = {**vars(module), "self": Driver(), "house": glyph, "count": ValueTracker(0),
                     "minutes": ValueTracker(0), "stage": np.array([-3.0, 0.4, 0.0]),
                     "stage_label": lambda *args: (Dot(), Dot()), "dots": [Dot([0.6, 0, 0])],
                     "labels": [VGroup(Dot([1.2, 0, 0]))], "leaders": [Line([0.6, 0, 0], [1.2, 0, 0])]}
        exec(compile(extracted, str(module.__file__), "exec"), namespace)
        production_error = None
        try:
            namespace["show_use"](glyph, "home", module.N_RATE, module.P_ORANGE, 0)
        except ValueError as error:
            production_error = str(error)
        payload = {"baseline_error": baseline_error, "dynamic_updaters": updater_count,
                   "frozen_transform_passed": True, "production_show_use_error": production_error}
        (OUTPUT / "result.json").write_text(json.dumps(payload, indent=2), encoding="utf-8")
        lines = ["# Inspection Render Fix Test", "", f"Status: {'FAILED production regression' if production_error else 'passed'}.", "", "The real source13 house geometry reproduces a Manim interpolation failure without full scene rendering. The driver invokes real ReplacementTransform.begin, update_mobjects and interpolate at five alpha values; NumPy performs the third-party interpolation. Freezing all descendant updaters before creating the transform passes the same boundary. The regression then extracts and executes the actual source nested show_use function with real Manim objects, independently checking the authored boundary rather than mirroring a proposed fix.", "", f"- Original dynamic geometry failure: {baseline_error}", f"- Descendant updater count before freeze: {updater_count}", "- Frozen geometry oracle: passed", f"- Production show_use result: {production_error or 'passed'}", "", "This test changes no production source. Generated fixtures/results belong to generated/iv/fix. Full source13 rendering and the final full-catalog verifier remain required after the narrow fix."]
        (TICKET / "inspection-render-fix-test.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
        print("[DEBUG]", json.dumps(payload), flush=True)
        assert production_error is None, production_error


if __name__ == "__main__":
    main()
