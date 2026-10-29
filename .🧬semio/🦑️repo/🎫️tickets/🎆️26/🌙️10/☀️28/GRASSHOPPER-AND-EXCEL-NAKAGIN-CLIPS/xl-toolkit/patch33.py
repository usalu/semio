import sys
f = sys.argv[1]
s = open(f, encoding="utf-8").read()
marker = "    def test_window_renders_in_both_languages"
new = '''    def test_title_bar_keeps_the_file_name_readable(self):
        """🟢 In narrow windows the search pill shrinks to a magnifier and finally disappears; nothing ever sits under the file name."""
        from manim import tempconfig

        for language in CASES["languages"]:
            for width in (3.4, 4.2, 5.2, 6.8, 12.4):
                with self.subTest(language=language, width=width), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), tempconfig({
                        "pixel_width": 480, "pixel_height": 270, "frame_rate": 1, "renderer": "cairo", "dry_run": True, "disable_caching": True,
                        "media_dir": tempfile.mkdtemp(prefix="excel-title-")}):
                    probe = type("Probe", (self.visuals.ClipScene,), {"xl_localize": lambda scene, formula, lang=None: formula,
                                                                        "xl_display": lambda scene, value, fmt="General", lang=None: f"{value:g}"})
                    window = probe().xl_window(FakeWorkbook(CASES["window"]), "Type A", width=width, height=3.2, cols="A:B", rows=(1, 4))
                    members = window.title.submobjects
                    name = members[10]
                    others = [member for member in members[11:] if len(member.points)]
                    self.assertGreater(name.width, 0.5)
                    self.assertLessEqual(name.get_right()[0], window.right - 0.9)
                    for member in others:
                        self.assertGreaterEqual(member.get_left()[0], name.get_right()[0] + 0.2 - 1e-6, member)
                    if width >= 6.8:
                        self.assertGreaterEqual(len(others), 4)

'''
s = s.replace(marker, new + marker, 1)
open(f, "w", encoding="utf-8").write(s)
