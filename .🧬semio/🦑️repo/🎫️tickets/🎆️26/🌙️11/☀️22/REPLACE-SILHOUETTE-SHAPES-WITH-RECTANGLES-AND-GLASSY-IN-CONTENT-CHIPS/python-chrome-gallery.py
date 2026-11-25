"""🖼️ Chrome gallery: the shared frame owner (frame, glass chips, cards, stat cards, watt anchor, free chips) on one still, rendered before and after the frame-model migration."""
import importlib

from manim import DOWN, LEFT, RIGHT, UP, Scene, Rectangle, VGroup, Circle, Line

V = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
F = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
U = V.UI_SPACING


class Gallery(Scene):
    def construct(self):
        self.camera.background_color = "#0a1a20"
        backdrop = VGroup(*[Line([-7, y, 0], [7, y, 0], color="#2e5a66", stroke_width=1) for y in (-3.4, -2.2, -1.0, 0.2, 1.4, 2.6)],
                          *[Line([x, -4, 0], [x, 4, 0], color="#2e5a66", stroke_width=1) for x in (-6, -3, 0, 3, 6)])
        self.add(backdrop)
        cards = V.card_grid([("Power · Leistung", "1200 W"), ("Area", "24 m²"), ("Heat", "7.5 kW", V.P_ORANGE)], rows=1)
        cards.scale(0.8).move_to([0, 2.6, 0])
        anchor = V.watt_anchor(1000, compare="sun", title="Solar")
        anchor.scale(0.8).move_to([-4.6, 0.1, 0])
        card = V.window_card(F.body_text("Frame", font_size=F.LABEL_FONT_SIZE, color=V.CHROME_FOREGROUND), F.body_text("body text row", font_size=F.BODY_FONT_SIZE, color=V.P_WHITE), width=3.4)
        card.move_to([0, 0.1, 0])
        chips = V.chip("Chip", V.P_PRIMARY).move_to([0.0, -1.3, 0])
        picture = VGroup(*[Rectangle(width=1.0, height=0.6, stroke_width=0, fill_color=c, fill_opacity=0.9).shift([i * 0.9 - 1.0, 0, 0]) for i, c in enumerate(("#d65a31", "#34d1bf", "#fccf05"))])
        sky = Rectangle(width=4.0, height=1.8, stroke_width=0, fill_color="#34d1bf", fill_opacity=0.55)
        picture_frame = V.window_frame(4.0, 1.8, top_depth=0.5, top_chips=((0, 1.3), (1.3, 2.6)), bottom_depth=0.4, bottom_chips=((2.6, 4.0),), color=V.P_PRIMARY) if hasattr(V, "frame_chips") else V.window_frame(4.0, 1.8, title_width=1.3, title_depth=0.5, color=V.P_PRIMARY)
        group = VGroup(sky, picture, picture_frame)
        for part in (sky, picture):
            part.move_to(picture_frame)
        picture.shift(DOWN * 0.1)
        if hasattr(V, "frame_chips"):
            group = VGroup(picture_frame, sky.move_to(picture_frame), picture.move_to(picture_frame).shift(UP * 0.0), V.frame_chips(4.0, 1.8, top_depth=0.5, top_chips=((0, 1.3), (1.3, 2.6)), bottom_depth=0.4, bottom_chips=((2.6, 4.0),), color=V.P_PRIMARY, center=picture_frame.get_center()))
        group.move_to([-3.0, -2.1, 0])
        light = V.window_frame(3.0, 1.4, color="#8A8A8A", stroke_width=1.2, fill_color="#FFFFFF", fill_opacity=0.95, top_depth=0.45, top_chips=((0, 3.0),), appearance="light") if hasattr(V, "frame_chips") else V.window_frame(3.0, 1.4, color="#8A8A8A")
        light.move_to([3.6, -2.1, 0])
        panels = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🧾️panels.🐍️")
        bim = panels.frame_box(3.0, 1.5, top_depth=0.46) if hasattr(V, "frame_chips") else panels.frame_box(3.0, 1.5)
        bim.move_to([3.6, 0.9, 0])
        self.add(cards, anchor, card, chips, group, light, bim)
