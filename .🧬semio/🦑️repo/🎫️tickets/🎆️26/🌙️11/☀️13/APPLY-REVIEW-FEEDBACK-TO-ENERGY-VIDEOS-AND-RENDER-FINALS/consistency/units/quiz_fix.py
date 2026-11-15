from pathlib import Path

N = "\u00a0"
PATH = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🌬️ventilation\❓️quiz\🔣️.json")
PAIRS = [
    (f"gives 28{N}×{N}25{N}m³/h + 60{N}m²{N}×{N}2.5{N}m³/(h·m²)",
     f"gives 28{N}persons{N}×{N}25.2{N}m³/(h·person) + 60{N}m²{N}×{N}2.52{N}m³/(h·m²)"),
    (f"ergibt 28{N}×{N}25{N}m³/h + 60{N}m²{N}×{N}2,5{N}m³/(h·m²)",
     f"ergibt 28{N}Pers.{N}×{N}25,2{N}m³/(h·Pers.) + 60{N}m²{N}×{N}2,52{N}m³/(h·m²)"),
    (f"(DIN 18032-1){N}×{N}30{N}={N}1,800", f"(DIN 18032-1){N}×{N}30{N}athletes{N}={N}1,800"),
    (f"(DIN 18032-1){N}×{N}30{N}={N}1.800", f"(DIN 18032-1){N}×{N}30{N}Personen{N}={N}1.800"),
]
text = PATH.read_bytes().decode("utf-8")
for old, new in PAIRS:
    if old in text:
        text = text.replace(old, new)
        print("ok", old[:30])
    else:
        print("MISS", old[:40])
PATH.write_bytes(text.encode("utf-8"))
