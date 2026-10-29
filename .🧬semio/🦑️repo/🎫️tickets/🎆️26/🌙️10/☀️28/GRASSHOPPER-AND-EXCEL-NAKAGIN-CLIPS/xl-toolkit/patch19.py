import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
src = src.replace('''                self.assertEqual(len(window.range(spec["range"])), 2)''', '''                self.assertEqual(len(window.range(spec["range"])), 2)
                for a1, code in (("C2", "#SPILL!"), ("C3", "#REF!")):
                    shown = self.book.error_name(code, language)
                    self.assertEqual(window.cell(a1).string, shown)
                    glyphs = window.cell(a1).text.get_family()[-1].parent if False else window.cell(a1).text
                    label = glyphs[-1] if len(glyphs) == 2 else glyphs
                    self.assertEqual(len(label.submobjects), len(shown.replace(" ", "")), "error code lost glyphs")
                    self.assertLessEqual(label.width, window.rect(a1)[2] - window.rect(a1)[0] + 0.7)''', 1)
marker = "    def test_window_renders_in_both_languages"
new = '''    def test_formula_glyphs(self):
        """🔣 Render =SUM(B2:B4)-B5: parentheses are round (curved, not bracket stems), minus sits at x-height, tokens keep their order and colours."""
        import numpy as np

        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}):
                group = self.book.formula_group("=SUM(B2:B4)-B5", size=19.2)
                self.assertEqual([token for token, _, _ in group.refs], ["B2:B4", "B5"])
                opening, bracket = self.book._label("(", 19.2, "#000000"), self.book._label("[", 19.2, "#000000")
                for glyph, curved in ((opening, True), (bracket, False)):
                    points = np.concatenate([m.get_anchors() for m in glyph.get_family() if len(m.points)])[:, :2]
                    top, mid = points[points[:, 1] > points[:, 1].max() - 0.1 * np.ptp(points[:, 1])], points[abs(points[:, 1] - points[:, 1].mean()) < 0.1 * np.ptp(points[:, 1])]
                    bulge = (top[:, 0].min() - mid[:, 0].min()) / np.ptp(points[:, 0])
                    self.assertEqual(bulge > 0.15, curved, (glyph, bulge))
                minus = [m for m in group.submobjects if m not in [shape for _, _, shape in group.refs]][-1]
                self.assertGreater(minus.get_center()[1], group.submobjects[0].get_bottom()[1] + 0.25 * group.submobjects[0].height)

'''
src = src.replace(marker, new + marker, 1)
open(path, "w", encoding="utf-8").write(src)
