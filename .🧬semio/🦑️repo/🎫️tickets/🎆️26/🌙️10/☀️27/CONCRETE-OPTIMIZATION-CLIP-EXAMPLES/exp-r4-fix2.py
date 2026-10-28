from pathlib import Path
p = next(Path.cwd().glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
t = p.read_text(encoding="utf-8")
a = 'self.tr("at this design (70 %, 1.0 m) a thicker wall keeps summer heat in:", "bei diesem Entwurf (70 %, 1,0 m) hält eine dickere Wand Sommerwärme drin:")'
b = 'self.tr("at 70 %, 1.0 m a thicker wall keeps heat in:", "bei 70 %, 1,0 m hält eine dickere Wand Wärme drin:")'
print(t.count(a))
p.write_bytes(t.replace(a, b).replace("\r\n", "\n").encode("utf-8"))
