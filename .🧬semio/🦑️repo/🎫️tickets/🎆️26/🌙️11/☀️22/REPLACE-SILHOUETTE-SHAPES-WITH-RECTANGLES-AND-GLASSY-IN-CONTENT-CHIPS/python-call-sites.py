"""🧪 Call-site driver: each scene subclasses the clip's own scene class and calls its own chrome helpers with sample content, static, over a coloured backdrop so glass shows."""
import importlib

import numpy as np
from manim import DOWN, LEFT, RIGHT, UP, Line, Rectangle, VGroup, Circle, Scene

ARCH = "🎓️teaching.🏛️architecture."
BIM = ARCH + "🏗️bim.🎬️clip."
ENERGY = ARCH + "⚡️energy."
FONTS = importlib.import_module(ENERGY + "🏗️authoring.🔤️fonts.🐍️")


def load(path):
    return importlib.import_module(path)


def backdrop(scene):
    scene.camera.background_color = "#0a1a20"
    scene.add(*[Line([-7.2, y, 0], [7.2, y, 0], color="#2e5a66", stroke_width=1) for y in (-3.0, -1.5, 0.0, 1.5, 3.0)],
              *[Line([x, -4, 0], [x, 4, 0], color="#2e5a66", stroke_width=1) for x in (-6, -3, 0, 3, 6)])


def show(scene, *mobs, strips=()):
    """Bright strips under the left half and the lower edge of every listed mobject so glass over content is visible, then the mobjects on top."""
    for mob in strips:
        top = mob.get_top()[1]
        scene.add(Rectangle(width=mob.width * 0.5, height=0.14, stroke_width=0, fill_color="#fccf05", fill_opacity=0.95).move_to([mob.get_left()[0] + mob.width * 0.25, top - 0.16, 0]))
    scene.add(*mobs)


class BimObjects(load(BIM + "🧬️objects.🐍️").Beat2_ObjectOrientation):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        obj = load(BIM + "🧬️objects.🐍️")
        chips = [self._chip("IfcColumn", (-5.2, 2.6, 0), obj.STRUCT), self._chip("IfcWall · long label that must squeeze", (-1.4, 2.6, 0), obj.ARCH, width=3.2),
                 self._storey_chip("Ground floor", (3.2, 2.5, 0), obj.SERV), self._storey_chip("Roof terrace", (5.6, 2.5, 0), obj.GOOD)]
        box = self.class_box("IfcWall", ["GlobalId", "Name", "Description", "PredefinedType"], (-4.6, -0.5, 0), color=obj.ARCH, width=3.2, grey=("GlobalId",))
        tree = self.class_tree(load(BIM + "🧱️model.🐍️").class_chain("IfcWall"), (2.6, -0.8, 0), width=5.4)
        show(self, *chips, box, tree, strips=(*chips, box))


class BimRepresentations(load(BIM + "👁️representations.🐍️").Beat3_SameObject):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        from types import SimpleNamespace
        model = load(BIM + "🧱️model.🐍️")
        base = model.load_model()
        self.m = SimpleNamespace(base=base, element=model.element("W-S", base), q0={"thickness": 0.32}, basis={"thickness": "assumed"})
        card = self._card()
        card.scale(0.85).move_to([-2.6, -0.7, 0])
        chip = self.guid_chip("W-S", (4.6, 2.5, 0))
        show(self, card, chip, strips=(card, chip))


class BimExchange(load(BIM + "🔌️exchange.🐍️").Beat5_Interfaces):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        ex = load(BIM + "🔌️exchange.🐍️")
        text = self.text("GlobalId 2O2Fr$t4X7Zf8NOew3FNr2", (-3.6, 2.3, 0), size=ex._d.S_LABEL, color=ex.INK)
        backed = ex._backed(text)
        ring = ex._ring(text, color=ex.DATA)
        step = self.step_panel(["#12=IFCWALL('2O2Fr$t4X7Zf8NOew3FNr2',#2,'W-S',$,$,#20,#30,'Tag',.STANDARD.);", "#20=IFCLOCALPLACEMENT($,#21);"], (3.0, 1.3, 0), width=6.0)
        model = self.model_card("architecture", (-4.2, -1.3, 0), thumbnail=False)
        table = self.property_table([("IsExternal", "TRUE", "", "sourced"), ("FireRating", "REI 60", "", "assumed")], (0.5, -1.0, 0), title="Pset_WallCommon", width=4.4)
        show(self, backed, ring, step, model, table, strips=(step, model, table))


class BimConsistency(load(BIM + "🔗️consistency.🐍️").Beat6_Consistency):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        card = self.card([("Area", "24.5 m²"), ("Volume", "61.0 m³", "assumed")], (-3.5, 1.2, 0), 3.8, title="Quantities")
        plain = self.panel(3.4, 1.6, (3.0, 1.2, 0))
        show(self, card, plain, strips=(card, plain))


class CapsuleCards(load(ARCH + "🦗️grasshopper.🎬️clip.💊️capsule.🐍️").Beat2_Capsule):
    def construct(self):
        from types import SimpleNamespace
        from manim import ValueTracker
        FONTS.apply_scene_style(self)
        backdrop(self)
        cap = load(ARCH + "🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
        self.st = SimpleNamespace(vals={key: ValueTracker(2.5) for key in cap.PARAM_COLOR})
        light = cap._light_card(3.0, 1.4, 0.5).move_to([-5.0, 2.2, 0])
        params = self._params_card([("Casing width", "w"), ("Casing length", "l"), ("Casing height", "h")], (-5.0, 0.3))
        card = self._card([("Casing width", "w", "2.0 m – 3.0 m"), ("Casing length", "l", "2.0 m – 3.0 m")], (0.2, 2.2))
        table = self._table((3.4, -0.6))
        show(self, light, params, card, table, strips=(light, params, card, table))


class NakaginChip(load(ARCH + "🏢️nakagin.🎨️visuals.🐍️").NakaginDrawing, load(BIM + "🎨️visuals.🐍️").ClipScene):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        chips = [self._chip("Ø 0.80 m", (-4.0, 1.5, 0)), self._chip("Ø 1.20 m", (-1.0, 0.0, 0)), self._chip("2.30 m", (3.0, -1.0, 0))]
        show(self, *chips, strips=chips)


class TreesChrome(load(ARCH + "🦗️grasshopper.🎬️clip.🌳️trees.🐍️").Beat5_DataTrees):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        trees = load(ARCH + "🦗️grasshopper.🎬️clip.🌳️trees.🐍️")
        sheet = self.gh_background(at=trees.SHEET_AT, width=trees.SHEET_W, height=trees.SHEET_H)
        plate = self._plate((-3.5, 0.8), 4.6, 2.4)
        label = self.text("Sample plate text row", (-3.5, 0.8, 0), size=trees.SMALL, color=trees.INK)
        chips = self._chips(("B, level 2: 8 capsules", trees.GOOD), ("tower A: teal", trees.ARCH), ("tower B: orange", trees.WARN))
        tag = self._tag("{A;B;C;D}(i)", (3.0, 0.0, 0), trees.DATA)
        show(self, sheet, plate, label, chips, tag, strips=(plate, chips, tag))


class CastingChrome(load(ARCH + "🦗️grasshopper.🎬️clip.🫗️casting.🐍️").Beat4_Casting):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        cast = load(ARCH + "🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
        self._d, self._cache, self._towers = cast._model.canvas("casting"), {}, {}
        sheet = self.gh_background(at=cast.SHEET_AT, width=cast.SHEET_SIZE[0], height=cast.SHEET_SIZE[1])
        tag = self._tag("Type A", cast.SHEET_AT + np.array([-2.0, 2.0, 0]), cast.INK_RED)
        order = self._order(0.6)
        port = self.gh_viewport(at=cast.PORT_AT, width=cast.PORT_SIZE[0], height=cast.PORT_SIZE[1], label="Perspective")
        chips = self._chips(self._sol())
        show(self, sheet, port, order, tag, chips, strips=(chips,))


class CastingHabits(load(ARCH + "🦗️grasshopper.🎬️clip.🫗️casting.🐍️").Beat4_Casting):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        cast = load(ARCH + "🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
        sheet = self.gh_background(at=cast.SHEET_AT, width=cast.SHEET_SIZE[0], height=cast.SHEET_SIZE[1])
        habits = self._habits()
        show(self, sheet, habits, strips=(habits,))


class ExcelReferences(load(ARCH + "📊️excel.🎬️clip.📍️references.🐍️").Beat2_References):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        card = self._formula_card([("F8", "=D8*E8"), ("F9", "=D9*E9*attached_factor")], (-3.2, 1.2), caption="Back wall to the unheated core, b = 0.5", width=5.4)
        plain = self._card(3.0, 1.2, (3.2, 1.2))
        show(self, card, plain, strips=(card, plain))


class ExcelEnergy(load(ARCH + "📊️excel.🎬️clip.⚡️energy.🐍️").Beat4_Energy):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        self._show = lambda sheet, a1: "123.4 kWh"
        self._num = lambda sheet, a1: 1.0
        self.T, self.CL = "Tower", "Climate"
        panel = self._chain((-4.5, 1.0), [("T", "B11", "Fabric loss"), ("T", "B12", "Vent loss"), ("T", "B14", "Annual demand")])
        tiles = self._annual_tiles((2.6, 0.8))
        chip = self._chip("sourced", (-5.5, 2.8, 0))
        show(self, panel, tiles, chip, strips=(panel, tiles))


class CoolingControl(Scene):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        cooling = load(ENERGY + "❄️cooling.🎬️clip.👥️internal-gains.🐍️")
        room = Rectangle(width=7.0, height=2.25, stroke_color="#34d1bf", stroke_width=2, fill_opacity=0).move_to([0, 0.35, 0])
        panel, title = cooling._control_panel(room, 7.0, "#34d1bf", "#2a7f78")
        track = Line(LEFT * 1.2, RIGHT * 1.2, color="#ffffff", stroke_width=5).move_to(panel.get_center() + DOWN * 0.2)
        show(self, room, panel, title, track, strips=(panel,))


class HeatingSpectrum(Scene):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        spectrum = load(ENERGY + "🔥️heating.🎬️clip.🧲️fundamentals.🐍️")._spectrum()
        self.add(spectrum["group"].scale(0.9).move_to([0, 0, 0]))


class PrimaryInset(Scene):
    def construct(self):
        FONTS.apply_scene_style(self)
        backdrop(self)
        mod = load(ENERGY + "📊️demand.🎬️clip.🏭️primary-energy.🐍️")
        ground = -0.4
        inset, label = mod._upstream_inset(ground)
        line = Line([-7, ground, 0], [-3, ground, 0], color="#8a8a8a", stroke_width=2)
        derrick = mod._derrick(np.array([-6.1, ground]), 0.14)
        plant = mod._power_plant(np.array([-4.45, ground]), 0.14)
        self.add(line, inset, label, derrick, plant)
