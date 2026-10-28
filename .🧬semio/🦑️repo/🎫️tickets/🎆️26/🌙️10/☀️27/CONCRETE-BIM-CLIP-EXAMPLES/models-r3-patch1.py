import pathlib
import re

p = next(pathlib.Path("C:/git/semio/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models").glob("*.py"))
s = p.read_text(encoding="utf-8")


def sub(old, new):
    global s
    assert s.count(old) == 1, old[:90]
    s = s.replace(old, new)


def replace_method(name, source):
    global s
    start = s.index(f"    def {name}(")
    end = s.index("\n    def ", start + 10)
    s = s[:start] + source.rstrip("\n") + "\n" + s[end:]


# narration
sub('''         "The services model holds boiler, radiators, roof drains and downpipes: the same house, other objects.",
         "Das Haustechnikmodell enthält Kessel, Heizkörper, Dachabläufe und Fallrohre: dasselbe Haus, andere Objekte."],''',
    '''         "The services model drains the 110 m² roof garden: boiler, radiators, drains, downpipes.",
         "Das Haustechnikmodell entwässert den 110-m²-Dachgarten: Kessel, Heizkörper, Abläufe, Fallrohre."],''')
sub('''         "Coordination puts the models on one origin. One GlobalId marks the shared slab; each model keeps its author.",
         "Koordination legt Modelle auf einen Ursprung. Eine GlobalId markiert die gemeinsame Decke; Urheber bleiben."],''',
    '''         "Coordination puts the models on one origin. In this teaching model one GlobalId marks the shared slab.",
         "Koordination legt Modelle auf einen Ursprung. Im Lehrmodell markiert eine GlobalId die gemeinsame Decke."],''')

# helper methods: text without brackets, analysis on the villa
sub('''    def _timeline(self, *stages):''', '''    def text(self, string, point=(0, 0, 0), **kwargs):
        """🔤 Text without bracket glyphs: a bracketed remark becomes a comma clause."""
        return super().text(re.sub(r"\\s*\\(([^)]*)\\)", r", \\1", string), point, **kwargs)

    def _villa_analysis(self, point, scale, *, ids=(), nodes=False, highlight=()):
        """🏙️ The analysis model drawn on the villa: structure axon with grid bubbles and slab outline underneath, members and shells on top."""
        base = self.villa_axon(point, scale, disciplines=("structure",), grid=True, shell=0.25, ground=False)
        view = self.analysis_view(self.analysis, point, scale, projection="axon", ids=ids, highlight=highlight)
        if not nodes:
            view.remove(view.nodes, view.supports)
        view.submobjects.insert(0, base)
        view.base = base
        return view

    def _timeline(self, *stages):''')
sub("import importlib\nimport sys\n", "import importlib\nimport re\nimport sys\n")

# architecture: ramp section inset
sub('''        choice = self.label("Ribbon windows on all four sides: a teaching choice", "Fensterbänder an allen vier Seiten: eine Lehrentscheidung", (0.95, -1.8, 0), size=16.2, color=MUTED, anchor="l", width=5.7)
        self._show("architecture", (0.8, [self._leave(), FadeIn(arch), FadeIn(card)]), *steps, (0.7, [FadeIn(total), FadeIn(choice)]))
        self.stage = VGroup(arch, card, rows, total, choice)''',
    '''        choice = self.label("Ribbon windows on all four sides: a teaching choice", "Fensterbänder an allen vier Seiten: eine Lehrentscheidung", (0.95, -1.8, 0), size=16.2, color=MUTED, anchor="l", width=5.7)
        flights = [flight["tag"] for flight in self.geometry["ramp"]["flights"]]
        ramp = VGroup(self.villa_section((-5.75, 2.4, 0), 0.095, cut="ramp", layers=("structure", "architecture"), highlight=flights, thumbnail=True),
                      self.label("ramp section", "Rampenschnitt", (-5.75, 1.78, 0), size=15.5, color=MUTED, width=1.5))
        self._show("architecture", (0.8, [self._leave(), FadeIn(arch), FadeIn(card)]), *steps, (0.7, [FadeIn(total), FadeIn(choice), FadeIn(ramp)]))
        self.stage = VGroup(arch, card, rows, total, choice, ramp)''')

# rooms: references, levels note
sub('''                y, area = -1.2 - row * 0.34, ValueTracker(0)
                trackers.append((area, space["area"]))''',
    '''                y, area = -1.2 - row * 0.34, ValueTracker(0)
                reference = next(item for item in self.model["spaces"] if item["tag"] == space["tag"]).get("reference_area")
                trackers.append((area, reference["value"] if reference else space["area"]))''')
sub('''self._counter(area, (x + 1.1, y, 0), size=17.0, color=ARCH, digits=1),''', '''self._counter(area, (x + 1.1, y, 0), size=17.0, color=ARCH, digits=0 if reference else 1),''')
sub('''        self._show("rooms", (0.8, [self._leave(), FadeIn(plans), FadeIn(header)]), *steps)
        self.stage = VGroup(plans, texts, header)''',
    '''        levels = self.label("teaching levels, not sourced · sourced areas in whole m²", "Lehrhöhen, nicht belegt · belegte Flächen in ganzen m²", (0, -2.35, 0), size=15.5, color=MUTED, width=8.0)
        self._show("rooms", (0.8, [self._leave(), FadeIn(plans), FadeIn(header), FadeIn(levels)]), *steps)
        self.stage = VGroup(plans, texts, header, levels)''')

# buildup: teaching values
sub('''        sides = VGroup(self.label("outside", "außen", (x0, y0 + height / 2 + 0.2, 0), size=16.2, color=MUTED, anchor="l"),''',
    '''        names.add(self.label("teaching values", "Lehrwerte", (x0 + total * zoom + 0.35, y0 - 0.95, 0), size=15.5, color=MUTED, anchor="l"))
        sides = VGroup(self.label("outside", "außen", (x0, y0 + height / 2 + 0.2, 0), size=16.2, color=MUTED, anchor="l"),''')

# structure: chip, bright C3 line
sub('''        chip = VGroup(frame_box(5.9, 0.46, GOOD).move_to((RIGHT, -0.85, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'} · C3", (RIGHT, -0.85, 0), size=17.0, color=GOOD, width=5.6))
        ring = Circle(radius=0.14, stroke_color=GOOD, stroke_width=2.4).move_to(struct.anchors["C3"])
        leader = self.leader(chip[0], struct.anchors["C3"], color=GOOD, dot=False)''',
    '''        chip = VGroup(frame_box(5.9, 0.46, GOOD).move_to((RIGHT, -0.85, 0)),
                      self.text(f"{pset}.LoadBearing = {'TRUE' if flag['value'] else 'FALSE'}", (RIGHT, -0.85, 0), size=16.2, color=GOOD, width=5.0))
        c3 = next(item for item in self.geometry["pilotis"] if item["tag"] == "C3")
        top = struct.at(c3["x"], c3["y"], c3["top"])
        ring = segments([(struct.at(c3["x"], c3["y"], c3["base"]), top)], GOOD, 4.5)
        leader = self.leader(chip[0], top, color=GOOD, dot=True)''')

# services: larger labelled boiler and radiator, reference area
sub('''        steps[1][1].append(self._flash(self.ghost.parts["OP-S"], ARCH))''', '''        steps[1][1].append(self._flash(self.ghost.parts["OP-S"], ARCH))
        names = VGroup(self.leader((-6.55, -1.35, 0), services.at("RAD-1"), self.tr("radiator RAD-1", "Heizkörper RAD-1"), color=SERV, size=16.2, anchor="l"),
                       self.leader((-6.55, -1.9, 0), services.at("BL-1"), self.tr("boiler BL-1", "Kessel BL-1"), color=SERV, size=16.2, anchor="l"))''')
sub('''area.animate(rate_func=LINEAR).set_value(garden["area"]), flow''', '''area.animate(rate_func=LINEAR).set_value(next(item for item in self.model["spaces"] if item["tag"] == "SP-TERRACE")["reference_area"]["value"]), flow''')
sub('''(1.1, [FadeIn(hatched), FadeIn(row),''', '''(1.1, [FadeIn(hatched), FadeIn(row), FadeIn(names),''')
sub('''self.stage = VGroup(services, self.ghost, card, rows, total, hatched, row, scenario)''', '''self.stage = VGroup(services, self.ghost, card, rows, total, hatched, row, names, scenario)''')

# schema header width
sub('''(x + 0.2, 1.95, 0), size=17.0, color=tone, anchor="l", width=1.7)''', '''(x + 0.2, 1.95, 0), size=17.0, color=tone, anchor="l", width=1.5)''')

# federation note
sub('''        note = self._lines([self.tr("Teaching model: both reference one slab.", "Lehrmodell: beide verweisen auf eine Decke."),
                            self.tr("Real models often carry their own copies.", "Reale Modelle führen oft eigene Kopien.")], (RIGHT - 2.7, -0.7, 0), width=5.4)''',
    '''        note = self._lines([self.tr("Teaching model: both reference one slab.", "Lehrmodell: beide verweisen auf eine Decke.")], (RIGHT - 2.7, -0.7, 0), width=5.4)''')

# slab: legend without brackets, conflict note, spacing dimension, scale bar
sub('''(self.tr("net (hatched)", "netto (schraffiert)"), -1.3, ARCH))])''', '''(self.tr("net, hatched", "netto, schraffiert"), -1.3, ARCH))])''')
sub('''        both = self.label("one object, one GlobalId", "ein Objekt, eine GlobalId", (RIGHT, -1.3, 0), size=16.2, color=DATA)''',
    '''        both = VGroup(self.label("one object, one GlobalId", "ein Objekt, eine GlobalId", (RIGHT, -1.3, 0), size=16.2, color=DATA),
                      self.label("teaching box 21.5 × 19 m; sources report other footprints", "Lehrbox 21,5 × 19 m; Quellen nennen andere Grundflächen", (RIGHT, -1.75, 0), size=15.5, color=MUTED, width=5.8))
        spacing = _model.clearance()["grid_spacing"]
        first_bay = self.dimension(plan.to_screen(1.25, width), plan.to_screen(1.25 + spacing, width), self.quantity(spacing, "m"), offset=0.2, color=STRUCT, size=15.5)
        dims.add(first_bay, self.scale_bar((-1.0, 2.5, 0), 0.17, 5.0, steps=2))''')

p.write_text(s, encoding="utf-8")
print("part 1 ok")
