from pathlib import Path

BASE = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip")
for name in ("🔁️recap", "⚖️regulations", "⚡️demand-power"):
    p = BASE / name / "🐍️.py"
    s = p.read_text(encoding="utf-8")
    n0 = s
    s = s.replace("\\ \\mathrm{in\\ ", "\\;\\mathrm{in}\\;\\mathrm{")
    s = s.replace("\\ \\mathrm{in}\\ \\frac", "\\;\\mathrm{in}\\;\\frac")
    s = s.replace("\\quad (F_{C}=F_{F}=F_{W}=1)\\quad", "\\quad F_{C}=F_{F}=F_{W}=1\\quad")
    s = s.replace("""\\mathrm{m^{3}}}\\ (20\\,^{\\circ}\\mathrm{C})",
                                 r"\\rho \\approx 1{,}2""", """\\mathrm{m^{3}}}\\;\\text{at}\\;20\\,^{\\circ}\\mathrm{C}",
                                 r"\\rho \\approx 1{,}2""")
    s = s.replace("""\\mathrm{m^{3}}}\\ (20\\,^{\\circ}\\mathrm{C})",
                              np.array""", """\\mathrm{m^{3}}}\\;\\text{bei}\\;20\\,^{\\circ}\\mathrm{C}",
                              np.array""")
    p.write_text(s, encoding="utf-8")
    print(s != n0)
