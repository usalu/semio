from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")
failures = []


def sub(old, new, count=1):
    global text
    found = text.count(old)
    if found != count:
        failures.append((found, count, old[:80]))
        return
    text = text.replace(old, new)


sub(r'tr(r"\text{Final energy}\; Q_{\mathrm{E}} = 1\,\mathrm{kWh}", r"\text{Endenergie}\; Q_{\mathrm{E}} = 1\,\mathrm{kWh}")',
    r'tr(r"{\text{Final energy} \atop Q_{\mathrm{E}} = 1\,\mathrm{kWh}}", r"{\text{Endenergie} \atop Q_{\mathrm{E}} = 1\,\mathrm{kWh}}")')
sub(r'tr(r"\text{Ambient heat}\; Q_{\mathrm{U}} = 3\,\mathrm{kWh}", r"\text{Umweltwärme}\; Q_{\mathrm{U}} = 3\,\mathrm{kWh}")',
    r'tr(r"{\text{Ambient heat} \atop Q_{\mathrm{U}} = 3\,\mathrm{kWh}}", r"{\text{Umweltwärme} \atop Q_{\mathrm{U}} = 3\,\mathrm{kWh}}")')
sub(r'tr(r"\text{Useful heat}\; Q_{\mathrm{N}} = 4\,\mathrm{kWh}", r"\text{Nutzwärme}\; Q_{\mathrm{N}} = 4\,\mathrm{kWh}"), np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.05, 0.0]),',
    r'tr(r"{\text{Useful heat} \atop Q_{\mathrm{N}} = 4\,\mathrm{kWh}}", r"{\text{Nutzwärme} \atop Q_{\mathrm{N}} = 4\,\mathrm{kWh}}"), np.array([stack_x - 0.42, h_frame.get_top()[1] + 0.2, 0.0]),')
sub('np.array([6.9, 2.0, 0.0]),\n                 size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")', 'np.array([6.9, 2.6, 0.0]),\n                 size=LABEL_FONT_SIZE, color=P_PRIMARY, edge="right")')
sub('np.array([6.9, 1.62, 0.0]),', 'np.array([6.9, 2.2, 0.0]),')
sub(']).arrange(RIGHT, buff=0.4).move_to(np.array([3.0, 2.15, 0.0]))', ']).arrange(RIGHT, buff=0.4).move_to(np.array([0.0, 1.95, 0.0]))')
sub('np.array([-3.5, 2.15, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY)', 'np.array([0.0, 2.55, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY)')
sub('''        self.play(FadeIn(row), Create(box), run_time=0.9)
        hold_for(self, N, "cop")''', '''        self.play(FadeIn(row), Create(box), FadeOut(lbl_valve), run_time=0.9)
        hold_for(self, N, "cop")''')
ROOT.write_text(text, encoding="utf-8")
print(failures)
