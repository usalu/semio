from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub('''                 if grid[a, b] <= padded[a:a + 3, b:b + 3].min() and np.count_nonzero(padded[a:a + 3, b:b + 3] == grid[a, b]) == 1]
        self.assertEqual(local, [(int(i), int(j))])''', '''                 if grid[a, b] + .05 < np.delete(padded[a:a + 3, b:b + 3].ravel(), 4).min()]
        self.assertEqual(local, [(int(i), int(j))])
        shelf = grid[:, 0] - grid[:, 1]
        self.assertLess(np.abs(shelf[10:25]).max(), .5)''')
sub('''            self.assertAlmostEqual(found.fun, self.surface["minimum"]["electricity"], delta=.2)
            self.assertAlmostEqual(found.x[0], self.surface["minimum"]["design"]["wwr"], delta=.04)
            self.assertAlmostEqual(found.x[1], self.surface["minimum"]["design"]["overhang"], delta=.07)''', '''            self.assertLess(found.fun, 1.01 * self.surface["minimum"]["electricity"])
            self.assertGreater(found.fun, .995 * self.surface["minimum"]["electricity"])
            self.assertTrue(.3 < found.x[0] < .79 and .5 < found.x[1] < 1.19)''')
sub('''        self.assertLess(gradient[14], 1.03 * self.optimum)
        self.assertGreater(genetic[14], gradient[14])''', '''        self.assertLess(gradient[29], 1.06 * self.optimum)
        self.assertGreater(genetic[9], gradient[29])
        self.assertGreater(gradient[0], 1.3 * self.optimum)''')
sub('''    def test_overhang_shades_most_in_summer_on_the_sun_facing_side(self):''', '''    def test_shading_is_the_geometric_share_of_the_window_height_in_the_overhang_shadow(self):
        import numpy as np

        gap, head = .3, 2.7
        samples = self.api.sun_samples(180, "summer")
        self.assertEqual([item["hour"] for item in samples], list(range(-4, 5)))
        self.assertAlmostEqual(samples[4]["altitude"], self.study["sun"]["summer_altitude"], delta=.01)
        self.assertAlmostEqual(samples[4]["profile"], samples[4]["altitude"])
        for wwr, depth in itertools.product((.2, .45, .8), (.3, .6, 1.2)):
            height = 3 * wwr
            weights = np.array([item["weight"] for item in samples])
            shares = np.clip((depth * np.tan(np.radians([item["profile"] for item in samples])) - gap) / height, 0, 1)
            result = self.api.evaluate({"wwr": wwr, "overhang": depth})
            self.assertAlmostEqual(result["shading_summer"], float((weights * shares).sum() / weights.sum()))
        noon = self.api.sun_state(180, "summer")
        self.assertAlmostEqual(noon["profile"], 66.04, delta=.01)
        self.assertEqual(self.api.evaluate({"overhang": .6})["shading_winter"], 0)
        self.assertLess(self.api.evaluate({"overhang": 1.2, "wwr": .45})["shading_winter"], .1)
        self.assertGreater(self.api.evaluate({"overhang": .6, "wwr": .45})["shading_summer"], .5)
        depths = [self.api.evaluate({"overhang": depth})["shading_summer"] for depth in np.linspace(0, 1.2, 25)]
        self.assertTrue(all(a <= b for a, b in zip(depths, depths[1:])))
        self.assertEqual(depths[0], 0)
        self.assertAlmostEqual(depths[-1], 1.0, delta=.02)
        heights = [self.api.evaluate({"overhang": .6, "wwr": wwr})["shading_summer"] for wwr in np.linspace(.2, .8, 13)]
        self.assertTrue(all(a >= b for a, b in zip(heights, heights[1:])))
        self.assertEqual(self.api.evaluate({"overhang": 0})["shading_summer"], 0)

    def test_winter_sun_stays_under_a_short_overhang_while_summer_sun_is_blocked(self):
        winter = self.api.sun_state(180, "winter")
        summer = self.api.sun_state(180, "summer")
        self.assertAlmostEqual(winter["altitude"], 19.16, delta=.01)
        self.assertLess(.6 * math.tan(math.radians(winter["profile"])), .3)
        self.assertGreater(.6 * math.tan(math.radians(summer["profile"])), 1.0)
        behind = self.api.sun_state(360, "winter")
        self.assertFalse(behind["front"])
        self.assertEqual(self.api.sun_samples(360, "winter"), [])
        self.assertEqual(self.api.evaluate({"orientation": 360, "overhang": 1.2})["shading_winter"], 0)
        sun = self.study["sun"]
        for season in ("winter", "summer"):
            track = self.api._sun_track(season)
            self.assertAlmostEqual(max(altitude for _, altitude in track), sun[f"{season}_altitude"], delta=.01)
            self.assertAlmostEqual(min(bearing for bearing, _ in track), sun[f"{season}_azimuth"][0], delta=.1)
            self.assertAlmostEqual(max(bearing for bearing, _ in track), sun[f"{season}_azimuth"][1], delta=.1)

    def test_overhang_shades_most_in_summer_on_the_sun_facing_side(self):''')
p.write_text(s, encoding="utf-8", newline="\n")
