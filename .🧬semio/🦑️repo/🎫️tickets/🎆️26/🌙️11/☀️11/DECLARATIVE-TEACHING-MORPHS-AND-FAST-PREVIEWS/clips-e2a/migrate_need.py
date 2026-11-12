import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply

CLIP = Path(sys.argv[1])
STAGE = "        stage = _morph.stage(self, TRANSITIONS)\n"

apply(CLIP, [
    ("fitted_staffage = _manim_module.fitted_staffage\n",
     'fitted_staffage = _manim_module.fitted_staffage\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ("def _fill_panel(scene, panel, sources, *, run_time: float = 2.0):", "def _fill_panel(scene, stage, panel, sources, *, run_time: float = 2.0):"),
    ("    scene.play(*[TransformFromCopy(_snap(src), items[key]) for key, src in sources.items()], run_time=run_time * 0.75)",
     '    scene.play(*[stage.transition(f"ventilation.need.demand.formula-{key}", source=_snap(src), target=items[key]) for key, src in sources.items()], run_time=run_time * 0.75)'),
    ("_fill_panel(self, panel, {", "_fill_panel(self, stage, panel, {"),
    ("class Beat1_Lueftungsbedarf", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("class Beat2_Lueftungsweg", "        apply_scene_style(self)\n", "        apply_scene_style(self)\n" + STAGE),
    ("ReplacementTransform(badge_winter, badge_summer)",
     'stage.transition("ventilation.need.path.badge-summer", source=badge_winter, target=badge_summer)'),
])
