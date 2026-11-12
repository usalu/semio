import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from edit_lib import apply

CLIP = Path(sys.argv[1])
P = "ventilation.natural."


def tr(scene, name, source, target):
    return f'stage.transition("{P}{scene}.{name}", source={source}, target={target})'


EDITS = [
    ("staffage_height = _manim_module.staffage_height\n",
     'staffage_height = _manim_module.staffage_height\n_morph = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🐍️")\nTRANSITIONS = _Path(__file__).resolve().parent / "🔀️transitions" / "🔣️.json"\n'),
    ('        apply_scene_style(self)\n\n        subtitle = section_heading(tr("The window is a control"',
     '        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n\n        subtitle = section_heading(tr("The window is a control"'),
    ("        apply_scene_style(self)\n        N = self.NARRATION\n        vol = ROOM_A * ROOM_H",
     "        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n        N = self.NARRATION\n        vol = ROOM_A * ROOM_H"),
    ('        apply_scene_style(self)\n\n        subtitle = section_heading(tr("Cross ventilation: both openings count"',
     '        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n\n        subtitle = section_heading(tr("Cross ventilation: both openings count"'),
    ('        apply_scene_style(self)\n\n        subtitle = section_heading(tr("Stack effect: height creates pressure"',
     '        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n\n        subtitle = section_heading(tr("Stack effect: height creates pressure"'),
    ('        apply_scene_style(self)\n\n        subtitle = section_heading(tr("Night ventilation: discharging the thermal mass"',
     '        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n\n        subtitle = section_heading(tr("Night ventilation: discharging the thermal mass"'),
    ('        apply_scene_style(self)\n\n        subtitle = section_heading(tr("Limits of natural ventilation"',
     '        apply_scene_style(self)\n        stage = _morph.stage(self, TRANSITIONS)\n\n        subtitle = section_heading(tr("Limits of natural ventilation"'),
]

W = "window-rules"
for old, new, sc, name in [
    ("ReplacementTransform(open_pct, open_mid)", ("open_pct", "open_mid"), W, "opening-partial"),
    ("ReplacementTransform(n_low, n_mid)", ("n_low", "n_mid"), W, "rate-partial"),
    ("ReplacementTransform(open_mid, open_wide)", ("open_mid", "open_wide"), W, "opening-wide"),
    ("ReplacementTransform(n_mid, n_high)", ("n_mid", "n_high"), W, "rate-wide"),
    ("ReplacementTransform(outdoor_cool, outdoor_hot)", ("outdoor_cool", "outdoor_hot"), W, "outdoor-hot"),
    ("ReplacementTransform(open_wide, open_thin)", ("open_wide", "open_thin"), W, "opening-thin"),
    ("ReplacementTransform(n_high, n_thin)", ("n_high", "n_thin"), W, "rate-thin"),
    ("ReplacementTransform(tag_flush, tag_throttle)", ("tag_flush", "tag_throttle"), W, "tag-throttle"),
    ("ReplacementTransform(outdoor_hot, outdoor_winter)", ("outdoor_hot", "outdoor_winter"), W, "outdoor-winter"),
    ("ReplacementTransform(inside_summer, inside_winter)", ("inside_summer", "inside_winter"), W, "inside-winter"),
    ("ReplacementTransform(open_thin, open_short)", ("open_thin", "open_short"), W, "opening-short"),
    ("ReplacementTransform(n_thin, n_short)", ("n_thin", "n_short"), W, "rate-short"),
    ("ReplacementTransform(tag_throttle, tag_short)", ("tag_throttle", "tag_short"), W, "tag-short"),
    ("ReplacementTransform(tot_k, tot_k_final)", ("tot_k", "tot_k_final"), "purge-or-tilt", "tilted-total-final"),
    ("TransformFromCopy(row_s, ratio)", ("row_s", "ratio"), "purge-or-tilt", "ratio-from-purge-row"),
    ("ReplacementTransform(a1_lbl.copy(), items[\"a1\"])", ("a1_lbl", 'items["a1"]'), "cross-ventilation", "a1-into-formula"),
    ("ReplacementTransform(a2_lbl.copy(), items[\"a2\"])", ("a2_lbl", 'items["a2"]'), "cross-ventilation", "a2-into-formula"),
    ("ReplacementTransform(aeff_wide, aeff_thin)", ("aeff_wide", "aeff_thin"), "cross-ventilation", "aeff-thin"),
    ("ReplacementTransform(guides, guides_thin)", ("guides", "guides_thin"), "cross-ventilation", "guides-thin"),
    ("ReplacementTransform(aeff_thin, aeff_wide)", ("aeff_thin", "aeff_wide"), "cross-ventilation", "aeff-wide-again"),
    ("ReplacementTransform(guides_thin, guides)", ("guides_thin", "guides"), "cross-ventilation", "guides-wide-again"),
    ("ReplacementTransform(aeff_wide, tip)", ("aeff_wide", "tip"), "cross-ventilation", "tip-summer"),
    ("ReplacementTransform(tip, tip_winter)", ("tip", "tip_winter"), "cross-ventilation", "tip-winter"),
    ("Transform(h_dim, tall_dim)", ("h_dim", "tall_dim"), "stack-effect", "height-dimension-taller"),
    ("ReplacementTransform(h_tok, h_tok_tall)", ("h_tok", "h_tok_tall"), "stack-effect", "height-token-taller"),
    ("ReplacementTransform(legend, legend_winter)", ("legend", "legend_winter"), "stack-effect", "legend-winter"),
    ("ReplacementTransform(mass_hot, mass_cool)", ("mass_hot", "mass_cool"), "night-ventilation", "mass-cooled"),
    ("ReplacementTransform(schedule, schedule_winter)", ("schedule", "schedule_winter"), "night-ventilation", "schedule-winter"),
    ("ReplacementTransform(season_lbl, season_lbl_winter)", ("season_lbl", "season_lbl_winter"), "night-ventilation", "season-winter"),
    ("ReplacementTransform(mass_cool, mass_warm)", ("mass_cool", "mass_warm"), "night-ventilation", "mass-warm"),
    ("ReplacementTransform(outdoor, outdoor_cold)", ("outdoor", "outdoor_cold"), "limits", "outdoor-cold"),
]:
    EDITS.append((old, tr(sc, name, *new)))

apply(CLIP, EDITS)
