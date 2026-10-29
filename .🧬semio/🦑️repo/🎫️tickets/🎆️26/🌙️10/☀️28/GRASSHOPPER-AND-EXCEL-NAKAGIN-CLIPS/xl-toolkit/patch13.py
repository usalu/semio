import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
marker = '\n\nif __name__ == "__main__":'
new = '''

    @unittest.skipUnless(ENGINE, "XL engine not built yet")
    def test_window_over_the_nakagin_workbook(self):
        """📗 Every visible cell shows what the engine says it shows, in both languages, and a recalculation morphs exactly the changed cells."""
        from manim import tempconfig

        model = importlib.import_module(f"{SERIES}.🧱️model.🐍️")
        book = model.workbook()
        after = model.apply(book, [{"type": "set-cell", "sheet": "Type A", "cell": "B2", "value": 3.0}])
        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), tempconfig({
                    "pixel_width": 480, "pixel_height": 270, "frame_rate": 1, "renderer": "cairo", "dry_run": True, "disable_caching": True,
                    "media_dir": tempfile.mkdtemp(prefix="excel-engine-")}):
                scene = type("Probe", (self.visuals.ClipScene,), {})()
                window = scene.xl_window(book, "Type A", cols="A:F", rows=(1, 10), selected="B9")
                values = model.evaluate(book)
                for a1, holder in window.cells.items():
                    self.assertEqual(holder.string or "", model.shown(book, values, "Type A", a1, language), a1)
                self.assertEqual(window.cell("B9").string, model.shown(book, values, "Type A", "B9", language))
                window.recalculate(after)
                changed = {a1 for a1, holder in window.cells.items() if holder.string != model.shown(book, values, "Type A", a1, language)}
                self.assertTrue({"B2", "D3", "D4"} <= changed)
                self.assertEqual(window.cell("B2").string, model.shown(after, model.evaluate(after), "Type A", "B2", language))
                self.assertEqual(window.cell("B5").string, model.shown(book, values, "Type A", "B5", language))
'''
src = src.replace(marker, new + marker)
open(path, "w", encoding="utf-8").write(src)
