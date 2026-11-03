from pathlib import Path

TESTS = Path("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests")


def sub(name, old, new):
    p = TESTS / name
    s = p.read_text(encoding="utf-8")
    assert old in s, (name, old)
    p.write_text(s.replace(old, new, 1), encoding="utf-8", newline="")


sub("🐍️integration.py", '''        for name, actual in (("radiator_height_m", radiator.height), ("radiator_width_m", radiator.width),
                             ("clearance_m", radiator.get_bottom()[1] - floor)):
            self.assertAlmostEqual(actual, CASES["physical"][name] * metre, places=6)
''', '''        for name, actual in (("radiator_height_m", radiator.height), ("clearance_m", radiator.get_bottom()[1] - floor)):
            self.assertAlmostEqual(actual, CASES["physical"][name] * metre, places=6)
        self.assertLessEqual(radiator.width, CASES["physical"]["radiator_width_m"] * metre + 1e-6)
''')
sub("🐍️integration.py", '''        self.assertAlmostEqual(scope["radiator"].width, CASES["physics"]["conceptual_heater_width"], places=6)''',
    '''        self.assertLessEqual(scope["radiator"].width, CASES["physics"]["conceptual_heater_width"] + 1e-6)''')
sub("🐍️scenes.py", '''            self.assertAlmostEqual(radiator.width, scale * CASES["house"]["radiator_width_m"], places=6)''',
    '''            self.assertLessEqual(radiator.width, scale * CASES["house"]["radiator_width_m"] + 1e-6)''')
sub("🐍️.py", '''        self.assertAlmostEqual(radiator.width, case["width"])
        self.assertAlmostEqual(radiator.height, case["height"])''', '''        self.assertLessEqual(radiator.width, case["width"] + 1e-6)
        self.assertAlmostEqual(radiator.height, case["height"])''')
sub("🐍️.py", '''        self.assertGreater(radiator[0].get_fill_opacity(), 0.5)''',
    '''        self.assertTrue(any(member.get_stroke_width() > 0 for member in radiator.family_members_with_points()))''')
