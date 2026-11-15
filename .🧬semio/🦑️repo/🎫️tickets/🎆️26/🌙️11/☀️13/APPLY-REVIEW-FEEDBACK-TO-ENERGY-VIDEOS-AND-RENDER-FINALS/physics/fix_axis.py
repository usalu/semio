import re
from pathlib import Path

path = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = path.read_text(encoding="utf-8")
pattern = re.compile(r'axis_lbl = Text\(tr\("Useful heat from[^\n]*\n')
new = ('axis_lbl = _m(tr(r"\\text{Useful heat}\\; Q_{\\mathrm{N}}\\; \\text{from}\\; Q_{\\mathrm{E}} = 1\\,\\mathrm{kWh}\\; \\text{final energy}",\n'
       '                         r"\\text{Nutzwärme}\\; Q_{\\mathrm{N}}\\; \\text{aus}\\; Q_{\\mathrm{E}} = 1\\,\\mathrm{kWh}\\; \\text{Endenergie}"),\n')
text, n = pattern.subn(lambda m: new, text)
print(n)
text = text.replace("            np.array([-3.5, 2.15, 0.0]))\n        legend", "                      np.array([-3.5, 2.15, 0.0]), size=LABEL_FONT_SIZE, color=P_PRIMARY)\n        legend")
path.write_text(text, encoding="utf-8")
