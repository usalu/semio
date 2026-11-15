from pathlib import Path

BASE = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip")
for name in ("🔁️recap", "⚖️regulations", "⚡️demand-power"):
    p = BASE / name / "🐍️.py"
    s = p.read_text(encoding="utf-8")
    t = s.replace("\\,\\mathrm{", "\\;\\mathrm{").replace("\\,\\frac{", "\\;\\frac{").replace("\\,^{\\circ}", "\\;^{\\circ}")
    t = t.replace("\\,\\\\mathrm{", "\\;\\\\mathrm{")
    p.write_text(t, encoding="utf-8")
    print(s.count("\\,\\mathrm{"), t.count("\\;\\mathrm{"))
