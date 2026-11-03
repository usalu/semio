"""🍲 Pot geometry and masses for 2 L of water (57 mm depth)."""
from pathlib import Path

F = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
s = F.read_text(encoding="utf-8")


def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old, s.count(old))
    s = s.replace(old, new)


sub("with 3 L of water that warms", "with 2 L of water that warms")
sub("mit 3 l Wasser, das auf einer Flamme", "mit 2 l Wasser, das auf einer Flamme")
sub("filled with 3 L of water to 90 mm.", "filled with 2 L of water to 57 mm.")
sub("mit 3 l Wasser bis 90 mm gefüllt.", "mit 2 l Wasser bis 57 mm gefüllt.")
sub("Pot and handles are 1.04 kg, the water 3.0 kg;", "Pot and handles are 1.04 kg, the water 2.0 kg;")
sub("wiegen 1,04 kg, das Wasser 3,0 kg;", "wiegen 1,04 kg, das Wasser 2,0 kg;")
sub('"massKg": 4.04', '"massKg": 3.04')
sub('"lengthM": 0.09,', '"lengthM": 0.057,')
sub('"y": 0.09,', '"y": 0.057,')
sub('"kg": 3,\n            "comX": 0,\n            "comY": 0.045,', '"kg": 2,\n            "comX": 0,\n            "comY": 0.0285,')
sub('"h": 0.09,', '"h": 0.057,', 2)
sub('"h": 0.085,', '"h": 0.052,')
sub("water to 90 mm and a handle", "water to 57 mm and a handle")
sub("Wasser bis 90 mm und je ein Griff", "Wasser bis 57 mm und je ein Griff")
sub("The 4 kg filled pot stands", "The 3 kg filled pot stands")
sub("Der 4 kg schwere gefüllte Topf", "Der 3 kg schwere gefüllte Topf")
F.write_text(s, encoding="utf-8")
