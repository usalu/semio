import sys
f = sys.argv[1]
s = open(f, encoding="utf-8").read()
marker = "    def test_window_renders_in_both_languages"
new = '''    def test_selection_tint_stays_under_text(self):
        """🟩 Draw order is fill, tint, gridlines, text, frames, border before and after a played selection and recalculation: the range tint never greys the values."""
        from manim import tempconfig

        model = importlib.import_module(f"{SERIES}.🧱️model.🐍️")
        book = model.workbook()
        after = model.apply(book, [{"type": "set-cell", "sheet": "Type A", "cell": "B3", "value": 5.0}])
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en", "SEMIO_TEACHING_AUDIENCE": "spectator"}), tempconfig({
                "pixel_width": 480, "pixel_height": 270, "frame_rate": 4, "renderer": "cairo", "dry_run": True, "disable_caching": True,
                "media_dir": tempfile.mkdtemp(prefix="excel-order-")}):
            visuals = self.visuals
            checks = []

            class Order(visuals.ClipScene):
                def construct(scene):
                    window = scene.xl_window(book, "Type A", rows=(1, 6), selected="B2")
                    scene.add(window, scene.xl_ref_frame(window, "B3:B4", color="#D93025"))
                    for moment in ("start", "select", "recalculate"):
                        if moment == "select":
                            scene.play(window.select("B3:B4"), run_time=0.1)
                        elif moment == "recalculate":
                            scene.play(window.recalculate(after), run_time=0.1)
                        family = list(scene.get_mobject_family_members())
                        position = lambda mobject: next(i for i, member in enumerate(family) if member is mobject)
                        text = position(window.cell("B3").text.get_family()[-1] if window.cell("B3").text.submobjects else window.cell("B3").text)
                        checks.append((moment, position(window.tint), position(window.gridlines), text, position(window.active), position(scene.mobjects[-1])))

            Order().render()
        for moment, tint, grid, text, border, frame in checks:
            with self.subTest(moment=moment):
                self.assertLess(tint, grid)
                self.assertLess(grid, text)
                self.assertLess(text, border)
                self.assertLess(text, frame)

'''
s = s.replace(marker, new + marker, 1)
open(f, "w", encoding="utf-8").write(s)
