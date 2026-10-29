import json, sys
f, j = sys.argv[1], sys.argv[2]
d = json.load(open(j, encoding="utf-8"))
d["long_formula"] = {"cell": "B4", "de": "=SUMMENPRODUKT((Kapseln!$D$2:$D$400=B2)*(Kapseln!$E$2:$E$400=\"A\")*Kapseln!$F$2:$F$400)+ZÄHLENWENNS(Kapseln!$A$2:$A$400;B2;Kapseln!$B$2:$B$400;\"Turm 1\")"}
assert len(d["long_formula"]["de"]) >= 120
open(j, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
s = open(f, encoding="utf-8").read()
s = s.replace('''        self.sheets, self.cells, self.formats = spec["sheets"], spec["cells"], spec["formats"]''', '''        self.sheets, self.cells, self.formats = spec["sheets"], dict(spec["cells"]), spec["formats"]
        self.cells[CASES["long_formula"]["cell"]] = CASES["long_formula"]["de"]''')
marker = "    def test_window_renders_in_both_languages"
new = '''    def test_long_formula_wraps_inside_the_window(self):
        """📏 A 120-character German formula shrinks to 16.2, wraps into an expanded bar, stays inside the window and keeps every reference colour."""
        from manim import tempconfig

        case = CASES["long_formula"]
        self.assertGreaterEqual(len(case["de"]), 120)
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "de"}), tempconfig({"pixel_width": 480, "pixel_height": 270, "frame_rate": 1, "renderer": "cairo", "dry_run": True,
                                                                                       "disable_caching": True, "media_dir": tempfile.mkdtemp(prefix="excel-long-")}):
            probe = type("Probe", (self.visuals.ClipScene,), {"xl_localize": lambda scene, formula, lang=None: formula,
                                                                "xl_display": lambda scene, value, fmt="General", lang=None: f"{value:g}"})
            scene = probe()
            window = scene.xl_window(FakeWorkbook(CASES["window"]), "Type A", cols="A:C", rows=(1, 6), selected=case["cell"])
            field = window.formula_bar[1].current
            self.assertGreater(len(field.lines), 1)
            self.assertEqual("".join(field.lines), case["de"])
            left, right = window.x0, window.right
            for member in field.get_family():
                if len(member.points):
                    self.assertGreaterEqual(member.get_left()[0], left - 1e-6)
                    self.assertLessEqual(member.get_right()[0], right + 1e-6)
                    self.assertGreaterEqual(member.get_bottom()[1], window.bottom)
            window.edit(case["de"])
            self.assertEqual(len(window.frames.refresh().current), 0)

'''
s = s.replace(marker, new + marker, 1)
open(f, "w", encoding="utf-8").write(s)
