"""Round 4 edits of the representations scene (verifier findings B1, M1-M3, m1-m11 and per-beat headings)."""
import re
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
target = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py"
t = target.read_text(encoding="utf-8")


def rep(a, b, count=1):
    global t
    assert t.count(a) == count, (a[:80], t.count(a))
    t = t.replace(a, b)


rep('_engine = importlib.import_module("manim")\n', '_engine = importlib.import_module("manim")\n_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")\n')
rep("CHIP_ROWS = (", '''HEADINGS = {
    "source": ("Villa Savoye · south wall", "Villa Savoye · Südwand"), "plan": ("Plan · cut at window height", "Grundriss · Schnitt auf Fensterhöhe"),
    "section": ("Section · sill and head", "Schnitt · Brüstung und Sturz"), "elevation": ("Elevation · south façade", "Ansicht · Südfassade"),
    "three": ("3D · body, void and fill", "3D · Körper, Aussparung, Füllung"), "stored": ("Stored · Axis, FootPrint, Body", "Gespeichert · Axis, FootPrint, Body"),
    "lod": ("Level of detail", "Detailstufe"), "scale200": ("Scale 1:200", "Maßstab 1:200"), "scale20": ("Scale 1:20", "Maßstab 1:20"),
    "sweep": ("Cut plane sweep", "Schnittebene wandert"), "derived": ("Take-off · areas", "Mengen · Flächen"),
    "update": ("Design change · sill lowered", "Planänderung · Brüstung gesenkt"), "takeoff": ("Take-off · live", "Mengenermittlung · live"),
    "volume": ("Take-off · volume", "Mengenermittlung · Volumen"), "copies": ("Copies · stale", "Kopien · veraltet"),
    "drafted": ("Drafted line", "Gezeichnete Linie"), "regenerate": ("Regeneration", "Neuerzeugung"), "identity": ("One object · one GlobalId", "Ein Objekt · eine GlobalId")}
CHIP_ROWS = (''')
rep("        self.trail = self.chip = self.card = None\n", "        self.trail = self.chip = self.card = self.sub = None\n")

t = re.sub(r'self\.step\("(\w+)", ', lambda mt: f'self.step("{mt.group(1)}", *self._head("{mt.group(1)}"), ', t)
t = t.replace("self.dimension(", "self._dimension(")
rep("    def _grow_dimension(", '''    def _head(self, key: str):
        """📑 Swap the section heading chip of the beat; returns the animations."""
        new = _fonts.section_heading(self.tr(*HEADINGS[key]))
        old, self.sub = self.sub, new
        return [_engine.FadeIn(new)] + ([_engine.FadeOut(old)] if old is not None else [])

    def _dimension(self, *args, **kwargs):
        """↔️ A toolkit dimension whose label is pushed a little further from the end ticks."""
        shape = self.dimension(*args, **kwargs)
        label = getattr(shape, "label", None)
        if label is not None:
            v = np.array(label.get_center()) - np.array(shape.line.get_center())
            v[2] = 0.0
            shape.label.shift(v / max(np.linalg.norm(v), 1e-9) * 0.06)
        return shape

    def _layer_name(self, layer: dict) -> str:
        """🏷️ Localized layer name with a bullet instead of brackets."""
        return re.sub(r"\\s*\\((.*?)\\)", r" · \\1", self._name(layer))

    def _grow_dimension(''')
rep("import importlib\nimport math\nimport sys", "import importlib\nimport math\nimport re\nimport sys")
rep("    def _grow_dimension(self, start, end, tracker, value: float, unit: str, *, offset: float, digits: int = 2, basis: str = \"\", color=DATA, size=S_SMALL, flip: bool = False, text_at: float = 0.5):", "    def _grow_dimension(self, start, end, tracker, value: float, unit: str, *, offset: float, digits: int = 2, basis: str = \"\", color=DATA, size=S_SMALL, flip: bool = False, text_at: float = 0.5):")

# M2 card
a = t.index("    def _card(self):")
b = t.index("    def _trail_group(self):")
t = t[:a] + '''    def _card(self):
        """🧾 Object card: the IFC properties of the wall with their localized meaning, the data that never change from view to view."""
        m = self.m
        language = self.language()
        words = {"sourced": (GOOD, "sourced", "Quelle"), "assumed": (WARN, "assumed", "angenommen")}
        rows = [("TotalThickness", self.tr("IfcMaterialLayerSet · sum of the layers", "IfcMaterialLayerSet · Summe der Schichten"), self.quantity(m.q0["thickness"], "m"), m.basis["thickness"])]
        for pid in ("Pset_WallCommon.IsExternal", "Pset_WallCommon.LoadBearing", "Pset_WallCommon.FireRating"):
            item = _model.information(TAG, pid, language, m.base)
            value = item["value"]
            rows.append((pid, item["meaning"], ("TRUE" if value else "FALSE") if isinstance(value, bool) else str(value), item["basis"]))
        frame = _engine.RoundedRectangle(corner_radius=0.06, width=4.9, height=2.3, stroke_color=GOOD, stroke_width=W_THIN, fill_color=BACKGROUND, fill_opacity=0.9).move_to((RIGHT, -1.35, 0))
        group = _engine.VGroup(frame, self.text(f"IfcWall · {self._name(_model.element(m.element['type'], m.base))}", (2.1, -0.42, 0), size=S_LABEL, color=GOOD, anchor="l", width=3.1),
                               self.label("stays the same", "bleibt gleich", (6.8, -0.42, 0), size=S_SMALL, color=MUTED, anchor="r"),
                               _drafting.segments([(P((2.0, -0.62)), P((6.9, -0.62)))], GOOD, W_THIN))
        for index, (name, meaning, value, basis) in enumerate(rows):
            y = -0.86 - 0.44 * index
            tone, en, de = words[basis]
            group.add(self.text(name, (2.1, y, 0), size=S_SMALL, color=INK, anchor="l", width=3.1), self.text(value, (5.6, y, 0), size=S_SMALL, color=DATA, anchor="r"),
                      self.label(en, de, (6.8, y, 0), size=S_SMALL, color=tone, anchor="r"), self.text(meaning, (2.1, y - 0.19, 0), size=S_SMALL, color=MUTED, anchor="l", width=4.6))
        group.table = frame
        return group

''' + t[b:]

# M1 source stars + footnote + m10 note
rep("""self.text(f"{self.quantity(op['end'] - op['start'], 'm')} × {self.quantity(op['head'] - op['sill'], 'm')}", window_at""", """self.text(f"{self.quantity(op['end'] - op['start'], 'm')} × {self.quantity(op['head'] - op['sill'], 'm')}{self._basis('sill')}", window_at""")
rep("""self.text(f"{self.quantity(m.q0['length'], 'm')} × {self.quantity(m.q0['height'], 'm')}", wall_at""", """self.text(f"{self.quantity(m.q0['length'], 'm')} × {self.quantity(m.q0['height'], 'm')}{self._basis('levels')}", wall_at""")
rep("""        self.chip, self.card, self.trail = self._chip(), self._card(), self._trail_group()
        self._stage(_engine.VGroup(axon, callouts, compass))""", """        self.chip, self.card, self.trail = self._chip(), self._card(), self._trail_group()
        self.legend = self._legend()
        simplification = self.label("other façades: ribbon windows are a teaching simplification", "andere Fassaden: Bandfenster sind eine Lehrvereinfachung", (-6.8, -2.2, 0), size=S_SMALL, color=MUTED, anchor="l", width=6.0)
        self._stage(_engine.VGroup(axon, callouts, compass, simplification))""")
rep("_engine.FadeIn(compass), _engine.FadeIn(self.chip)", "_engine.FadeIn(compass), _engine.FadeIn(simplification), _engine.FadeIn(self.legend), _engine.FadeIn(self.chip)")

# cut key stars, de words
rep("""group.add(self.text(f"{self.tr(name_en, name_de)} {self._sign(z)}", (x, sy(z), 0)""", """group.add(self.text(f"{self.tr(name_en, name_de)} {self._sign(z)}{self._basis('levels')}", (x, sy(z), 0)""")
rep('self.tr("plan cut ", "Grundrissschnitt ") + self._sign(cut)', 'self.tr("plan cut ", "Schnittebene ") + self._sign(cut) + self._basis("levels")')
rep('self.tr("plan cut ", "Grundrissschnitt ") + self.quantity(h.get_value(), "m", 2)', 'self.tr("plan cut ", "Schnittebene ") + self.quantity(h.get_value(), "m", 2)')

# board section marks stars
rep("""marks = _engine.VGroup(*[self.level_mark(P((x_c + 0.05, section.at(0, value)[1])), self._sign(value), size=S_SMALL, length=0.1) for value in (geo["storeys"]["ground"], geo["storeys"]["first"], geo["storeys"]["roof"], geo["box"]["height"])])""",
    """marks = _engine.VGroup(*[self.level_mark(P((x_c + 0.05, section.at(0, value)[1])), self._star(self._sign(value), basis), size=S_SMALL, length=0.1) for value, basis in ((geo["storeys"]["ground"], self.m.basis["levels"]), (geo["storeys"]["first"], self.m.basis["levels"]), (geo["storeys"]["roof"], self.m.basis["levels"]), (geo["box"]["height"], "sourced"))])""")

# sweep stars, detail scale, frame
rep("""self.text(f"{self.tr('window', 'Fenster')}  {self.tr('sill', 'Brüstung')} {self.quantity(op['sill'], 'm')} · {self.tr('head', 'Sturz')} {self.quantity(op['head'], 'm')}{self._basis('sill')}",""", """self.text(f"{self.tr('window', 'Fenster')}  {self.tr('sill', 'Brüstung')} {self.quantity(op['sill'], 'm')}{self._basis('sill')} · {self.tr('head', 'Sturz')} {self.quantity(op['head'], 'm')}{self._basis('head')}",""")
rep("self._cut_plan(h.get_value(), -6.5, -0.6, 1.6, wall[\"length\"] - 3.0)", "self._cut_plan(h.get_value(), -6.5, -0.6, 1.45, wall[\"length\"] - 3.0)")

# layer rows stars, layer names, lod scale label
rep("""text = f"{self._name(layer)}  {self.quantity(t, 'm', 2)} → {self.number(t * 1000 / den, 2 if den > 100 else 1)} mm\"""", """text = f"{self._layer_name(layer)}  {self.quantity(t, 'm', 2)}{self._star('', layer['thickness']['basis'])} → {self.number(t * 1000 / den, 2 if den > 100 else 1)} mm\"""")
rep("""self.text(f"{self._name(layer)}  {self.quantity(layer['thickness']['value'], 'm', 2)}{self._star('', layer['thickness']['basis'])}",""", """self.text(f"{self._layer_name(layer)}  {self.quantity(layer['thickness']['value'], 'm', 2)}{self._star('', layer['thickness']['basis'])}",""")
rep("        legend = self._layer_rows(-4.1, -1.35, 20)\n        pieces = (title,", """        legend = _engine.VGroup(self.label("layers on paper at 1:20", "Schichten auf Papier im Maßstab 1:20", (-4.1, -1.1, 0), size=S_SMALL, color=MUTED, anchor="l"), self._layer_rows(-4.1, -1.45, 20))
        pieces = (title,""")

# scale200 stars
rep("""self.text(f"{self.quantity(t, 'm')} → {mm(t)} mm", (x, 0.02, 0)""", """self.text(f"{self.quantity(t, 'm')}{self._basis('thickness')} → {mm(t)} mm", (x, 0.02, 0)""")
rep("""self.text(f"{self.quantity(layer, 'm')} → {mm(layer, 2)} mm", (x, -1.68, 0)""", """self.text(f"{self.quantity(layer, 'm')}{self._star('', m.layers[0]['thickness']['basis'])} → {mm(layer, 2)} mm", (x, -1.68, 0)""")

# M3 footprint outline
rep("""            ring = [P((sx(x_from), sy(0))), P((sx(x_to), sy(0))), P((sx(x_to), sy(t))), P((sx(x_from), sy(t)))]
            group.add(_drafting.polygon(ring, ARCH, 0.45, ARCH, W_EDGE), self.hatch([ring], ARCH, 0.07, 45))""", """            ring = [P((sx(x_from), sy(0))), P((sx(x_to), sy(0))), P((sx(x_to), sy(t))), P((sx(x_from), sy(t)))]
            group.add(_drafting.polyline(ring, ARCH, W_CUT, closed=True))""")
rep("""        shape = _drafting.polygon([view(*p) for p in ring], ARCH, 0.45, ARCH, W_EDGE)
        hatch = self.hatch([[view(*p) for p in ring]], ARCH, 0.07, 45)""", """        shape = _drafting.polyline([view(*p) for p in ring], ARCH, W_CUT, closed=True)
        hatch = _engine.VGroup()""")

# m7 gross/net hatch
rep("""                x0 = length * net.get_value()
                return _engine.VGroup(self.hatch([rect(x0, xe, wall["base"], wall["top"])], tone, 0.07, 45)) if xe > x0 else _engine.VGroup()""", """                return _engine.VGroup(self.hatch([rect(0, xe, wall["base"], wall["top"])], tone, 0.07, 45))""")
rep("            return _engine.VGroup(self.hatch(rings, tone, 0.07, 45))\n\n        return self.live(build", "            return _engine.VGroup(self.hatch(rings, tone, 0.07, -45))\n\n        return self.live(build")
rep("rows.add(self.hatch([ring], tone, 0.06, 45), _engine.Rectangle(width=0.4", "rows.add(self.hatch([ring], tone, 0.06, -45 if tone == GOOD else 45), _engine.Rectangle(width=0.4")

# derived stars
rep("""f"{n(wall['length'])} m × {n(wall['top'] - wall['base'])} m", "m²")""", """f"{n(wall['length'])} m × {n(wall['top'] - wall['base'])} m{self._basis('levels')}", "m²")""")
rep("""f"{n(width)} m × {n(height)} m", "m²")""", """f"{n(width)} m × {n(height)} m{self._basis('sill')}", "m²")""")
rep("""f"{n(q['gross_side_area'])} − {n(q['opening_area'])}", "m²")]""", """f"{n(q['gross_side_area'])} − {n(q['opening_area'])}", "m²")]""")
rep("lives.append(self.live_label(lambda t=tracker, v=value, u=unit: self.quantity(v * t.get_value(), u, 2), (1.5, y, 0)", "lives.append(self.live_label(lambda t=tracker, v=value, u=unit: self.quantity(v * t.get_value(), u, 2) + \"*\", (1.5, y, 0)")

# takeoff stars, de words
rep("""before = self._percent(q0[name], 1) if unit == "%" else self.quantity(q0[name], unit)""", """before = (self._percent(q0[name], 1) if unit == "%" else self.quantity(q0[name], unit)) + "*\"""")
rep("""lives.append(self.live_label((lambda nm=name, u=unit: self._percent(self._q(key())[nm], 1) if u == "%" else self.quantity(self._q(key())[nm], u)), (6.8""", """lives.append(self.live_label((lambda nm=name, u=unit: (self._percent(self._q(key())[nm], 1) if u == "%" else self.quantity(self._q(key())[nm], u)) + "*"), (6.8""")
rep("""= {n(self._q(key())['net_side_area'])} m²",""", """= {n(self._q(key())['net_side_area'])} m²*",""")
rep("""= {n(self._q(key())['opening_area'])} m²",
                    lambda""", """= {n(self._q(key())['opening_area'])} m²*",
                    lambda""")
rep("""= {self._percent(self._q(key())['glazing_share'], 1)}"]""", """= {self._percent(self._q(key())['glazing_share'], 1)}*"]""")
rep("""self.mono(f"OpeningResized  sill {n(m.op['sill'])} → {n(m.event['sill'])} m",""", """self.mono(f"{self.tr('OpeningResized', 'Öffnung geändert')}  {self.tr('sill', 'Brüstung')} {n(m.op['sill'])} → {n(m.event['sill'])} m",""")
rep("""mono("OpeningResized", 2.1, DATA), mono(f"object  {m.window}", 1.82), mono(f"sill    {self.number(m.op['sill'], 2)} → {self.number(event['sill'], 2)} m", 1.54), mono(f"head    {self.number(event['head'], 2)} m", 1.26))""", """mono(self.tr("OpeningResized", "Öffnung geändert"), 2.1, DATA), mono(f"{self.tr('object', 'Objekt')}  {m.window}", 1.82), mono(f"{self.tr('sill', 'Brüstung')}  {self.number(m.op['sill'], 2)} → {self.number(event['sill'], 2)} m", 1.54), mono(f"{self.tr('head', 'Sturz')}  {self.number(event['head'], 2)} m", 1.26))""")

# B1 volume: three decimals everywhere, rev 1 values
rep("""self.text(f"{self.tr('void', 'Aussparung')}  {self.quantity(void, 'm³')}", (0.0, 2.35, 0)""", """self.text(f"{self.tr('void', 'Aussparung')}  {self.quantity(void, 'm³', 3)}*", (0.0, 2.35, 0)""")
rep("""self.text(f"{self.tr('wall', 'Wand')}  {self.quantity(q['volume'], 'm³')}", (-6.2, 2.35, 0)""", """self.text(f"{self.tr('wall', 'Wand')}  {self.quantity(q['volume'], 'm³', 3)}*", (-6.2, 2.35, 0)""")
a = t.index('        specs = [("gross volume", "Bruttovolumen"')
b = t.index('        note = self.label("volume = net side area')
t = t[:a] + '''        void = lambda k: self._q(k)["gross_volume"] - self._q(k)["volume"]
        specs = [("gross volume", "Bruttovolumen", lambda: self.quantity(self._q(0)["gross_volume"], "m³", 3) + "*", INK, None), ("void", "Aussparung", lambda: self.quantity(void(key()), "m³", 3) + "*", WARN, void),
                 ("wall volume", "Wandvolumen", lambda: self.quantity(self._q(key())["volume"], "m³", 3) + "*", GOOD, lambda k: self._q(k)["volume"])]
        static, lives = _engine.VGroup(), []
        for index, (en, de, fn, tone, before) in enumerate(specs):
            y = 1.9 - 0.8 * index
            static.add(_engine.Rectangle(width=0.14, height=0.45, stroke_width=0, fill_color=tone, fill_opacity=0.9).move_to((2.15, y - 0.05, 0)), self.label(en, de, (2.4, y + 0.1, 0), size=S_SMALL, color=MUTED, anchor="l"))
            if before is not None:
                static.add(self.text(f"{self.tr('rev ', 'Rev. ')}{m.base['revision']}  {self.quantity(before(0), 'm³', 3)}*", (2.4, y - 0.17, 0), size=S_SMALL, color=MUTED, anchor="l"))
            lives.append(self.live_label(fn, (6.8, y - 0.05, 0), size=S_BODY, color=tone, anchor="r"))
        lives.append(self.live_label(lambda: f"{n(self._q(key())['net_side_area'])} m² × {n(q0['thickness'])} m = {n(self._q(key())['volume'], 3)} m³*", (2.15, -0.7, 0), size=S_SMALL, color=INK, anchor="l"))
        lives.append(self.live_label(lambda: f"{n(self._q(0)['gross_volume'], 3)} − {n(void(key()), 3)} = {n(self._q(key())['volume'], 3)} m³*", (2.15, -1.1, 0), size=S_SMALL, color=INK, anchor="l"))
''' + t[b:]
target.write_text(t, encoding="utf-8")
print("[DEBUG] round 4 edits applied")
