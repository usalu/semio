"""🖼️ One still of the presentation focus frame through the Python geometry: canvas band, frame, glass chips and the edge-to-edge body below the chip row."""
import importlib

from manim import Scene

PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation"
presentation = importlib.import_module(PACKAGE + ".🐍️")
geometry = importlib.import_module(PACKAGE + ".📐️geometry.🐍️")
typography = importlib.import_module(PACKAGE + ".🔤️typography.🐍️")


class FocusFrame(Scene):
    def construct(self):
        contract = presentation.presentation_contract()
        self.camera.background_color = "#0a1a20"
        canvas = contract["canvas"]
        self.add(presentation._box({"x": 0, "y": 0, "width": 1600, "height": 900}, color="#0a1a20", stroke=0))
        self.add(presentation._box(canvas, color="#13303a", opacity=1, stroke=0))
        advance = typography.text_metrics("Physical Fundamentals", 11.2)["advance"] + 22.4 + 3.2
        window = geometry.window_geometry(contract["focus"]["rect"], advance)
        self.add(presentation._box(window["body"], color="#34d1bf", opacity=0.55, stroke=0))
        self.add(presentation._box({"x": window["body"]["x"] + 40, "y": window["body"]["y"] + 40, "width": 300, "height": 120}, color="#fccf05", opacity=0.9, stroke=0))
        for chip in window["chip_regions"]:
            self.add(presentation._box(chip, color="#ffffff", opacity=0.14, stroke=0))
        self.add(presentation._box(window["rect"], color="#f7f3e3", fill="#000000", opacity=0, stroke=2))
        self.add(presentation._text("Physical Fundamentals", window["title_text"]["x"] + 3.2, window["title_text"]["y"], color="#f7f3e3"))
        viewport = contract["focus"]["viewport"]
        self.add(presentation._box(viewport, color="#ff4d4d", opacity=0, stroke=1))
