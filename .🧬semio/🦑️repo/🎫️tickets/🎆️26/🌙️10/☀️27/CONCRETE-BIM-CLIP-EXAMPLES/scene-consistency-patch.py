import sys
from pathlib import Path
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
pairs = [
    ('LINK, K_KN = "LK-ROOF", _analysis.ARROW_KN', 'LINK, K_KN = "LK-ROOF", 2.2 * _analysis.ARROW_KN'),
    ('DETAIL_AT, DETAIL_SCALE, DETAIL_WIDTH = (3.0, -2.3, 0), 2.8, 2.2', 'DETAIL_AT, DETAIL_SCALE, DETAIL_WIDTH = (2.75, -2.3, 0), 2.8, 1.8'),
    ('CARD_AT = (3.75, 2.75, 0), (3.65, 1.85, 0), (3.75, 2.28, 0), (3.95, 0.7, 0)', 'CARD_AT = (3.75, 2.75, 0), (3.65, 1.85, 0), (3.75, 2.28, 0), (4.05, 0.62, 0)'),
    ('HEAD_Y, PAIR_Y, SMALL = 2.2, 0.9, 0.09', 'HEAD_Y, PAIR_Y, SMALL = 2.28, 0.9, 0.09'),
    ('card = self.panel(5.8, 1.45, (3.95, CARD_AT[1] - 0.05, 0))', 'card = self.panel(5.6, 1.45, (4.05, CARD_AT[1] - 0.05, 0))'),
    ('(1.2, 1.15, 0), size=S_LABEL, color=INK, anchor="l", width=5.4)', '(1.4, 1.15, 0), size=S_LABEL, color=INK, anchor="l", width=5.2)'),
    ('(1.2, 0.75, 0), size=S_LABEL, color=GLASS, anchor="l", width=5.4)', '(1.4, 0.75, 0), size=S_LABEL, color=GLASS, anchor="l", width=5.2)'),
    ('(1.2, 0.35, 0), size=S_SMALL, color=MUTED, anchor="l", width=5.4)', '(1.4, 0.35, 0), size=S_SMALL, color=MUTED, anchor="l", width=5.2)'),
    ('frame = _engine.Rectangle(width=5.8, height=1.95, stroke_color=MUTED, stroke_width=_d.W_HAIR, fill_opacity=0).move_to((3.95, -1.5, 0))',
     'frame = _engine.Rectangle(width=5.6, height=1.95, stroke_color=MUTED, stroke_width=_d.W_HAIR, fill_opacity=0).move_to((4.05, -1.5, 0))'),
    ('"Dachaufbau · IfcMaterialLayerSet", (1.2, -0.72, 0)', '"Dachaufbau · IfcMaterialLayerSet", (1.4, -0.72, 0)'),
    ('                  _engine.FadeIn(section), _engine.FadeIn(card), _engine.FadeIn(facts),\n                  _engine.FadeIn(detail), _engine.FadeIn(caption), _engine.Create(zoom), _engine.Create(frame), _engine.FadeIn(rain),',
     '                  _engine.FadeIn(section), _engine.Create(zoom), _engine.Create(frame), _engine.FadeIn(card), _engine.FadeIn(facts),\n                  _engine.FadeIn(detail), _engine.FadeIn(caption), _engine.FadeIn(rain),'),
    ('        rows = [(self.tr("event", "Ereignis"), self.event["type"]), (self.tr("object GlobalId", "Objekt-GlobalId"), self.gid),',
     '        rows = [(self.tr("object GlobalId", "Objekt-GlobalId"), self.gid),'),
    ('self.quantity(self.soil_new, "m", 2))]\n        event = self.card(rows, CARD_AT, 5.8, self.tr("design event · teaching scenario", "Entwurfsereignis · Lehrszenario"), GOOD, value_width=3.1, mono=(rows[1][0],))\n        copy = _engine.TransformFromCopy(self.ident, event.values[1])',
     'self.quantity(self.soil_new, "m", 2))]\n        event = self.card(rows, CARD_AT, 5.6, f"{self.event[\'type\']} · " + self.tr("teaching scenario", "Lehrszenario"), GOOD, value_width=3.1, mono=(rows[0][0],))\n        copy = _engine.TransformFromCopy(self.ident, event.values[0])'),
    ('q_label = self.live_label(lambda: "q = " + self.quantity(mix(self.zone_old["planted"]["area_load"], self.zone_new["planted"]["area_load"]), "kN/m²", 2),\n                                  section.at(self.cut_x, (span[0] + span[1]) / 2, storeys["roof"]) + (0.0, 0.3 + self.zone_new["planted"]["area_load"] * self.k_q, 0), size=S_LABEL, color=DATA)',
     'q_label = self.live_label(lambda: "q = " + self.quantity(mix(self.zone_old["planted"]["area_load"], self.zone_new["planted"]["area_load"]), "kN/m²", 2),\n                                  (SECTION_AT[0], section.at(self.cut_x, 0.0, storeys["roof"])[1] + 0.32 + self.zone_new["planted"]["area_load"] * self.k_q, 0), size=S_LABEL, color=DATA)'),
    ('        span = max(self.planted_spans(), key=lambda s: s[1] - s[0])\n        q_label', '        q_label'),
    ('plan = self.plan_cells((2.2, -1.0, 0), 0.105)', 'plan = self.plan_cells((2.35, -1.0, 0), 0.13)'),
    ('derivation = self.derivation((5.2, -1.05, 0))', 'derivation = self.derivation((5.4, -1.05, 0))'),
    ('frame = self.panel(3.2, 2.45, point, DATA)\n        x0, y0 = point[0] - 1.45, point[1] + 0.98', 'frame = self.panel(3.0, 2.45, point, DATA)\n        x0, y0 = point[0] - 1.35, point[1] + 0.98'),
    ('"Einflussflächen-Schätzung, keine FEM", (x0, y0, 0), size=S_SMALL, color=MUTED, anchor="l", width=2.95)', '"Einflussflächen-Schätzung, keine FEM", (x0, y0, 0), size=S_SMALL, color=MUTED, anchor="l", width=2.75)'),
    ('for zid, colour in (("planted", _core.layer_color("soil")), ("solarium", GLASS)):', 'for zid, colour in (("planted", GOOD), ("solarium", GLASS)):'),
    ('color=_core.layer_color("soil"), anchor="l", width=2.3)', 'color=GOOD, anchor="l", width=2.6)'),
    ('review = self.panel(8.1, 2.1, (2.5, -1.62, 0), GOOD)', 'review = self.panel(8.1, 1.95, (2.5, -1.58, 0), GOOD)'),
    ('(-1.45, -2.38, 0), size=S_LABEL, color=GOOD, anchor="l", width=7.6)', '(-1.45, -2.32, 0), size=S_LABEL, color=GOOD, anchor="l", width=7.6)'),
    ('((-3.9, self.new, self.soil_new, self.tr("edit A", "Änderung A"), ARCH), (3.9, self.alt,', '((-3.5, self.new, self.soil_new, self.tr("edit A", "Änderung A"), ARCH), (3.5, self.alt,'),
    ('same_id = self.label("same GlobalId, different content", "gleiche GlobalId, anderer Inhalt", (0.0, -0.1, 0), size=S_SMALL, color=WARN, width=2.0)',
     'same_id = _engine.VGroup(self.label("same GlobalId", "gleiche GlobalId", (0.0, -0.05, 0), size=S_LABEL, color=WARN, width=2.4), self.label("different content", "anderer Inhalt", (0.0, -0.4, 0), size=S_LABEL, color=WARN, width=2.4))'),
    ('(0.0, -1.0, 0), size=S_LABEL, color=MUTED, width=2.6)', '(0.0, -1.0, 0), size=S_SMALL, color=MUTED, width=2.9)'),
    ('details.add(self.build_up(model, (x - 1.0, -2.2, 0), 2.2, 1.1,', 'details.add(self.build_up(model, (x - 0.9, -2.2, 0), 2.2, 1.1,'),
]
for old, new in pairs:
    assert old in text, old[:80]
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
