from pathlib import Path
p = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🔁️recap\🐍️.py")
s = p.read_text(encoding="utf-8")
old = '''        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar_block"))
        self.add(s_card["rows"][4])
        self.play(FadeOut(fs_g["static"]), FadeOut(ring_f), FadeIn(patches), *_terms(row_s, 9, 10),
                  *sunny(1.0), run_time=1.0)'''
new = '''        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "weitere"))
        self.play(FadeOut(fs_g["static"]), FadeOut(ring_f), *_terms(row_s, 9, 10), *sunny(1.2), run_time=1.2)
        hold_for(self, self.NARRATION, "weitere", during=sunny)

        caption = swap_caption(self, caption, subtitle_text(self.NARRATION, "solar_block"))
        self.add(s_card["rows"][4])
        self.play(FadeIn(patches), *sunny(1.0), run_time=1.0)'''
assert s.count(old) == 1
p.write_text(s.replace(old, new), encoding="utf-8")
print("done")
