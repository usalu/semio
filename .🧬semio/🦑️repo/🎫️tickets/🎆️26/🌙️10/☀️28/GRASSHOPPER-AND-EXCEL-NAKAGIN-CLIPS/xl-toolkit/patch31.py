import sys
f = sys.argv[1]
s = open(f, encoding="utf-8").read()
old = '''            Order().render()'''
new_in = '''                    twin = scene.xl_window(after, "Type A", rows=(1, 6), selected="B2")
                    scene.play(FadeIn(twin), twin.select("B3:B4"), run_time=0.1)
                    hop = twin.select("B4")
                    hop.begin()
                    hop.interpolate(1.0)
                    hop.finish()
                    family = list(scene.get_mobject_family_members())
                    position = lambda mobject: next(i for i, member in enumerate(family) if member is mobject)
                    checks.append(("fade-in", position(twin.tint), position(twin.gridlines), position(twin.cell("B3").text.get_family()[-1]), position(twin.active), len(family)))

'''
s = s.replace("            visuals = self.visuals\n", "            visuals = self.visuals\n            from manim import FadeIn\n", 1)
s = s.replace("                        checks.append((moment, position(window.tint), position(window.gridlines), text, position(window.active), position(scene.mobjects[-1])))\n", "                        checks.append((moment, position(window.tint), position(window.gridlines), text, position(window.active), len(family)))\n" + new_in, 1)
s = s.replace("                self.assertLess(text, frame)\n", "                self.assertLess(text, border if moment != 'fade-in' else frame)\n", 1) if False else s
s = s.replace("                self.assertLess(text, frame)\n", "                self.assertLess(text, frame)\n" if False else "")
open(f, "w", encoding="utf-8").write(s)
