import re
from pathlib import Path
p = next(Path.cwd().glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
t = p.read_text(encoding="utf-8")
i = t.index("Darum eine")
print(ascii(t[i:i + 120]))
j = t.index("0,5", i)
k = t.index("Vordach.", j)
t = t[:i] + "Darum eine Bestätigungssimulation in der Mitte: Fensteranteil 50 %, Vordach 0,5 m." + t[k + len("Vordach."):]
p.write_bytes(t.replace("\r\n", "\n").encode("utf-8"))
print(ascii(t[i:i + 120]))
