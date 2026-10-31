import sys
f = sys.argv[1]
s = open(f, encoding="utf-8").read()
marker = "    def test_window_renders_in_both_languages"
new = '''    @unittest.skipUnless(ENGINE, "XL engine not built yet")
    def test_recalculate_retypes_cells_in_place(self):
        """♻️ Mid-animation every glyph of a recalculated cell stays inside its own cell box, the old text is gone by half time and the new text arrives after, and the end state equals the engine's text."""
        from manim import tempconfig

        model = importlib.import_module(f"{SERIES}.🧱️model.🐍️")
        book = model.workbook()
        after = model.apply(book, [{"type": "set-cell", "sheet": "Type A", "cell": "B2", "value": 3.0}])
        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), tempconfig({
                    "pixel_width": 480, "pixel_height": 270, "frame_rate": 1, "renderer": "cairo", "dry_run": True, "disable_caching": True,
                    "media_dir": tempfile.mkdtemp(prefix="excel-retype-")}):
                scene = type("Probe", (self.visuals.ClipScene,), {})()
                window = scene.xl_window(book, "Type A", cols="A:F", rows=(2, 9), selected="B2")
                animation = window.recalculate(after)
                self.assertGreaterEqual(len(animation.retexts), 3)
                animation.begin()
                values = model.evaluate(after)
                for alpha in (0.25, 0.5, 0.75, 1.0):
                    animation.interpolate(alpha)
                    for holder, old, new, glow in animation.retexts:
                        left, top, right, bottom = window.rect(next(a1 for a1, item in window.cells.items() if item is holder))
                        for label in (old, new):
                            for member in (label.get_family() if label is not None else ()):
                                if len(member.points):
                                    self.assertGreaterEqual(member.get_left()[0], left - 0.03)
                                    self.assertLessEqual(member.get_right()[0], right + 0.03)
                                    self.assertLessEqual(member.get_top()[1], top + 0.03)
                                    self.assertGreaterEqual(member.get_bottom()[1], bottom - 0.03)
                        if alpha == 0.5 and old is not None and new is not None:
                            self.assertEqual(max(member.get_fill_opacity() for member in old.get_family() if len(member.points)), 0.0)
                animation.finish()
                for a1, holder in window.cells.items():
                    self.assertEqual(holder.string or "", model.shown(after, values, "Type A", a1, language), a1)
                    self.assertLessEqual(len(holder.submobjects), 2)

'''
s = s.replace(marker, new + marker, 1)
open(f + ".new", "w", encoding="utf-8").write(s)
