from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub("def test_grid_covers_five_window_ratios_three_insulations_and_two_overhangs(self):", "def test_grid_covers_six_window_ratios_three_insulations_and_two_overhangs(self):")
sub('self.assertEqual(len({(item["design"]["wwr"], item["design"]["insulation"], item["design"]["overhang"]) for item in designs}), 30)', 'self.assertEqual(len({(item["design"]["wwr"], item["design"]["insulation"], item["design"]["overhang"]) for item in designs}), 36)')
sub('self.assertEqual(len({item["design"]["wwr"] for item in designs}), 5)', 'self.assertEqual(len({item["design"]["wwr"] for item in designs}), 6)')
sub('''        self.assertLess(path[-1]["electricity"], 1.005 * self.optimum)
        self.assertAlmostEqual(path[-1]["electricity"], CASES["algorithms"]["gradient"]["last"], places=4)
        self.assertGreater(path[0]["electricity"], 1.1 * self.optimum)
        target = self.surface["minimum"]["design"]["wwr"]
        sides = [item["design"]["wwr"] > target for item in path[:3]]
        self.assertEqual(sides[0], True)
        self.assertEqual(sides[1], False)
        self.assertLess(path[1]["design"]["wwr"], target)
        self.assertGreater(path[2]["design"]["wwr"], path[1]["design"]["wwr"])
''', '''        self.assertLess(path[-1]["electricity"], 1.025 * self.optimum)
        self.assertAlmostEqual(path[-1]["electricity"], CASES["algorithms"]["gradient"]["last"], places=4)
        self.assertGreater(path[0]["electricity"], 1.3 * self.optimum)
        values = [item["electricity"] for item in path]
        rises = [k for k in range(1, len(values)) if values[k] > values[k - 1]]
        self.assertGreaterEqual(len(rises), 1)
        self.assertLess(min(values[rises[0]:]), values[rises[0] - 1])
        self.assertLess(values[-1], values[rises[0] - 1])
        target = self.surface["minimum"]["design"]["wwr"]
        sides = [item["design"]["wwr"] > target for item in path]
        self.assertTrue(sides[0] and not all(sides))
''')
sub('        self.assertLess(abs(self.gradient[-1]["gradient"]["wwr"]) * .7, 20)\n', '        self.assertLess(abs(self.gradient[-1]["slope"]["wwr"]), abs(self.gradient[0]["slope"]["wwr"]))\n')
p.write_text(s, encoding="utf-8", newline="\n")
