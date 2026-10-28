import pathlib

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:100]
    s = s.replace(old, new)


# imports and constants
sub('''_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")''',
    '''_sheet = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📏️sheet.🐍️")
_plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")
_fonts = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")''')
sub('''KIND_LABELS = {''', '''HEADINGS = {
    "whole": ("Villa Savoye · one building", "Villa Savoye · ein Gebäude"), "architecture": ("Architecture model · 28 objects", "Architekturmodell · 28 Objekte"),
    "rooms": ("Rooms · IfcSpace per storey", "Räume · IfcSpace je Geschoss"), "buildup": ("Wall build-up · plan cut", "Wandaufbau · Grundrissschnitt"),
    "structure": ("Structure model · 33 objects", "Tragwerksmodell · 33 Objekte"), "services": ("Services model · 13 objects", "Haustechnikmodell · 13 Objekte"),
    "schema": ("One schema · 13 classes", "Ein Schema · 13 Klassen"), "federation": ("Federation · one origin, one GlobalId", "Koordination · ein Ursprung, eine GlobalId"),
    "slab": ("One slab · one GlobalId", "Eine Decke · eine GlobalId"), "owners": ("Ground floor · who draws what", "Erdgeschoss · wer zeichnet was"),
    "check": ("Clash check · clearance", "Kollisionsprüfung · Abstand"), "translation": ("Translation · structure to analysis", "Übersetzung · Tragwerk zur Analyse"),
    "members": ("Analysis mesh on the slab", "Analysenetz auf der Decke"), "midsurface": ("Mid-surface · section at C3", "Mittelfläche · Schnitt bei C3"),
    "supports": ("Supports · piloti feet", "Lagerung · Pilotifüße"), "loads": ("Loads · q builds up", "Lasten · q baut sich auf"),
    "loss": ("Losses · what is not kept", "Verluste · was nicht erhalten bleibt"), "properties": ("Piloti C3 · kept and dropped", "Piloti C3 · erhalten und verworfen"),
    "trace": ("Trace · source GlobalId", "Rückverfolgung · Quell-GlobalId")}
KIND_NAMES = {
    "IfcWindow:element": ("Window", "Fenster"), "IfcOpeningElement:element": ("Opening", "Aussparung"), "IfcWall.STANDARD:element": ("Wall", "Wand"),
    "IfcWall.STANDARD:material.layer.render": ("Render", "Putz"), "IfcCurtainWall:element": ("Glass wall", "Glaswand"), "IfcWall.SOLIDWALL:element": ("Screen", "Schirm"),
    "IfcRamp:element": ("Ramp", "Rampe"), "IfcRampFlight:element": ("Flight", "Lauf"), "IfcSpace:element": ("Space", "Raum"), "IfcSlab.ROOF:element": ("Roof slab", "Dachdecke"),
    "IfcSlab.FLOOR:voids": ("Ramp void", "Rampenloch"), "IfcColumn:Pset_ColumnCommon.FireRating": ("Fire rating", "Feuerwid."), "IfcColumn:Pset_ColumnCommon.IsExternal": ("External", "Außen")}
STAR = "*"
KIND_LABELS = {''')

# heading in _show, legend helpers
sub('''        animation, total = self._timeline(*stages)
        self.step(key, animation, seconds=total)''', '''        heading = _fonts.section_heading(self.tr(*HEADINGS[key]))
        old, self.sub = self.sub, heading
        first = stages[0]
        stages = ((first[0], [*first[1], FadeIn(heading), *([FadeOut(old)] if old is not None else [])]), *stages[1:])
        animation, total = self._timeline(*stages)
        self.step(key, animation, seconds=total)''')
sub('''        self.stage = VGroup()
        self.ghost = None''', '''        self.stage = VGroup()
        self.sub = None
        self.ghost = None''')
sub('''    def _timeline(self, *stages):''', '''    def _legend(self, point, *, assumed=False, piloti=False, width=5.8):
        """⭐ Star legend lines: assumed teaching-model values and the disputed piloti count."""
        x, y, _ = P(point)
        lines = ([self.tr("* assumed in the teaching model", "* im Lehrmodell angenommen")] if assumed else []) + ([self.tr("* sources differ: 15–25", "* Quellen uneins: 15–25")] if piloti else [])
        return self._lines(lines, (x, y, 0), size=15.5, color=MUTED, pitch=0.24, width=width)

    def _piloti_label(self):
        """🏛️ The piloti count wording: the teaching-model number with its star."""
        return (f"pilotis in this teaching model{STAR}", f"Pilotis in diesem Lehrmodell{STAR}")

    def _timeline(self, *stages):''')

# sum line star for pilotis
sub('''        terms = " + ".join(self.number(counts[c], 0) for c in classes)''', '''        terms = " + ".join(self.number(counts[c], 0) + (STAR if c == "IfcColumn" else "") for c in classes)''')

# narration
sub('''"The services model drains the 110 m² hanging garden: boiler, radiators, drains, downpipes.",
         "Das Haustechnikmodell entwässert den 110-m²-Garten im 1. OG: Kessel, Heizkörper, Abläufe, Fallrohre."],''',
    '''"The services model holds boiler, radiator, roof drains and downpipes: the same house, other objects.",
         "Das Haustechnikmodell enthält Kessel, Heizkörper, Dachabläufe und Fallrohre: dasselbe Haus, andere Objekte."],''')

# rooms
sub('''            title = self.text(f"{storeys[level]['name'][language]} {self._level_text(storeys[level]['elevation']['value'])} m", (x, 2.5, 0), size=18.0, color=INK)''',
    '''            elevation = storeys[level]["elevation"]["value"]
            title = self.text(f"{storeys[level]['name'][language]} {self._level_text(elevation)} m" + (STAR if elevation else ""), (x, 2.5, 0), size=18.0, color=INK)''')
sub('''self.text("m²", (x + 1.15, y, 0), size=16.2, color=MUTED, anchor="l"))''', '''self.text("m²" + ("" if reference else STAR), (x + 1.15, y, 0), size=16.2, color=MUTED, anchor="l"))''')
sub('''        levels = self.label("teaching levels, not sourced · sourced areas in whole m²", "Lehrhöhen, nicht belegt · belegte Flächen in ganzen m²", (0, -2.35, 0), size=15.5, color=MUTED, width=8.0)''',
    '''        areas = [next(item for item in self.model["spaces"] if item["tag"] == tag)["reference_area"]["value"] for tag in ("SP-TERRACE", "SP-SOLARIUM")]
        levels = self.label(f"* assumed in the teaching model · source: {self.number(areas[0], 0)} and {self.number(areas[1], 0)} m²",
                            f"* im Lehrmodell angenommen · Quelle: {self.number(areas[0], 0)} und {self.number(areas[1], 0)} m²", (0, -2.35, 0), size=15.5, color=MUTED, width=8.0)''')

# buildup
sub('''names.add(Rectangle(width=0.2, height=0.2, stroke_color=line, stroke_width=1.2, fill_color=fill, fill_opacity=1.0).move_to((x0 + total * zoom + 0.35, y, 0)),
                      self.text(f"{layer['name'][language]} · {self.quantity(layer['thickness']['value'], 'm')}",''', '''names.add(Rectangle(width=0.2, height=0.2, stroke_color=line, stroke_width=1.2, fill_color=fill, fill_opacity=1.0).move_to((x0 + total * zoom + 0.35, y, 0)),
                      self.text(f"{layer['name'][language]} · {self.quantity(layer['thickness']['value'], 'm')}{STAR}",''')
sub('''        names.add(self.label("teaching values", "Lehrwerte", (x0 + total * zoom + 0.35, y0 - 0.95, 0), size=15.5, color=MUTED, anchor="l"))''',
    '''        names.add(self._legend((x0 + total * zoom + 0.35, y0 - 0.95, 0), assumed=True, width=2.9))''')
sub('''self.quantity(total * grow.get_value(), "m"), offset=-0.22, color=DATA, size=17.0))''', '''self.quantity(total * grow.get_value(), "m") + STAR, offset=-0.22, color=DATA, size=17.0))''')

# structure
sub('''        spec = [("IfcColumn", ("pilotis (teaching choice)", "Pilotis (Lehrentscheidung)")),''', '''        spec = [("IfcColumn", self._piloti_label()),''')
sub('''        piloti_note = self.label("The sources disagree on the piloti count.", "Die Quellen sind uneins über die Pilotizahl.", (0.95, -0.38, 0), size=16.2, color=MUTED, anchor="l", width=5.6)''',
    '''        piloti_note = self._legend((0.95, -0.38, 0), piloti=True)''')

# services
sub('''        row = VGroup(self._counter(area, (0.95, -0.55, 0), size=19.0, color=GOOD),
                     self.label(f"m² hanging garden (first floor), drained by {drains} roof drains", f"m² hängender Garten (1. OG), entwässert über {drains} Dachabläufe", (1.1, -0.55, 0), size=16.2, color=GOOD, anchor="l", width=5.4))''',
    '''        row = VGroup(self._counter(area, (0.95, -0.5, 0), size=19.0, color=GOOD),
                     self.label("m² hanging garden, first floor, open air", "m² hängender Garten, 1. OG, offen", (1.1, -0.5, 0), size=16.2, color=GOOD, anchor="l", width=5.4),
                     self.label(f"its floor drains are not modelled; the {drains} roof drains belong to the roof level", f"seine Bodenabläufe sind nicht modelliert; die {drains} Dachabläufe gehören zur Dachebene",
                                (1.1, -0.78, 0), size=15.5, color=MUTED, anchor="l", width=5.4))''')
sub('''(0.95, -1.1, 0), width=5.6)
        pipes''', '''(0.95, -1.3, 0), width=5.6)
        pipes''')

p.write_text(s, encoding="utf-8")
print("part 1 ok")
