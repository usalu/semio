import re
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🧲️physics\🎬️clip\🧲️fundamentals\🐍️.py")
text = ROOT.read_text(encoding="utf-8")
en = 'r"Per kilowatt-hour of final energy $Q_{\\mathrm{E}}$: wood chips give about 0.85 kWh of useful heat $Q_{\\mathrm{N}}$, oil 0.94, gas 0.96.",\n'
de = 'r"Je 1 kWh Endenergie $Q_{\\mathrm{E}}$ liefert Holz rund 0,85 kWh Nutzwärme $Q_{\\mathrm{N}}$, Öl 0,94, Erdgas 0,96."),\n'
text, n1 = re.subn(r'r"Per kilowatt-hour of final energy[^\n]*\n', lambda m: en, text)
text, n2 = re.subn(r'r"Je 1 kWh Endenergie \$Q_[^\n]*\n', lambda m: de, text)
print(n1, n2)
ROOT.write_text(text, encoding="utf-8")
