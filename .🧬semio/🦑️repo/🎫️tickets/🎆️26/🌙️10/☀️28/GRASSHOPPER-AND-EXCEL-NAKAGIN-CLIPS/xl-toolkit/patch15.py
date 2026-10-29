import sys
path = sys.argv[1]
src = open(path, encoding="utf-8").read()
src = src.replace("import ast\nimport importlib\n", "import ast\nimport copy\nimport importlib\n", 1)
old = "                tracker.set_value(7.0)\n"
assert old in src
src = src.replace(old, '''                twin = copy.deepcopy(window)
                self.assertEqual(len(twin.cells), len(window.cells))
                self.assertIs(window.scene, scene)
                from manim import FadeIn, FadeOut
                scene.play_compiled = [scene.compile_animations(FadeIn(window)), scene.compile_animations(FadeOut(window))]
''' + old, 1)
marker = "    def test_window_renders_in_both_languages"
new = '''    def test_punctuation_sits_on_the_baseline(self):
        """➖ A lone minus, asterisk or equals sign is placed by the text baseline, so the minus stays at x-height instead of dropping to an underscore."""
        capital = self.book._put("H", 0, 0, "l", 19.2)
        for sign in ("-", "*", "="):
            with self.subTest(sign=sign):
                shape = self.book._put(sign, 0, 0, "l", 19.2)
                self.assertGreater(shape.get_center()[1], capital.get_bottom()[1] + 0.25 * capital.height)
        group = self.book.formula_group("=B2-B3", size=19.2)
        minus = group[1]
        self.assertGreater(minus.get_center()[1], group[0].get_bottom()[1] + 0.25 * group[0].height)

'''
src = src.replace(marker, new + marker, 1)
open(path, "w", encoding="utf-8").write(src)
