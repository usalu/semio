from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub("for start in ([.75, .1], [.15, 1], [.3, .3], [.8, 1.2]):", "for start in ([.75, .1], [.3, .3], [.7, .9], [.2, .2], [.5, .5]):")
sub('angles = np.arange(*self.study["constraint"], 1) if False else np.arange(self.study["constraint"][0], self.study["constraint"][1] + 1)', 'angles = np.arange(self.study["constraint"][0], self.study["constraint"][1] + 1)')
sub('self.assertEqual({(item["design"]["wwr"], item["design"]["insulation"], item["design"]["overhang"]) for item in designs}.__len__(), 30)', 'self.assertEqual(len({(item["design"]["wwr"], item["design"]["insulation"], item["design"]["overhang"]) for item in designs}), 30)')
sub('self.assertEqual({len({item["design"][key] for item in designs}) for key in ("wwr",)}, {5})', 'self.assertEqual(len({item["design"]["wwr"] for item in designs}), 5)')
sub('''            expected = {key: before["step"][key] * (.5 if not item["moved"] and False else 1) for key in before["step"]}
            self.assertEqual(item["step"].keys(), expected.keys())
''', '')
sub('self.assertEqual({item["title"]["en"] == item["title"]["de"]}, {False})', 'self.assertNotEqual(item["title"]["en"], item["title"]["de"])')
sub('        self.assertGreater(path[1]["step"]["wwr"] * path[0]["step"]["wwr"] + 1, 0)\n', '')
sub('        self.assertEqual(self.study["tariff"], self.study["tariff"])\n', '        self.assertEqual(self.study["tariff"], .25)\n')
sub('        self.assertLess(genetic[0], 1e9)\n', '')
sub('for before, item in zip(path, path[1:]):\n            self.assertEqual([probe["direction"]', 'for before, item in zip(path, path[1:]):\n            self.assertEqual([probe["direction"]')
p.write_text(s, encoding="utf-8")
