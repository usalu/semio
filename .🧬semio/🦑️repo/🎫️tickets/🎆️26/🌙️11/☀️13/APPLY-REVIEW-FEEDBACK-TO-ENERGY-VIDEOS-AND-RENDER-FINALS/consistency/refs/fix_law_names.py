import re
import sys
from pathlib import Path

ENERGY = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
RULES = [
    (re.compile(r"German Buildings Energy Act \(Gebäudeenergiegesetz, GModG\)(?: 2024)?"), "GModG (formerly GEG)"),
    (re.compile(r"Gebäudeenergiegesetz \(GModG\)(?: 2024)?"), "GModG (vormals GEG)"),
    (re.compile(r"German Buildings Energy Act, GModG, §"), "GModG, formerly GEG, §"),
    (re.compile(r"GModG §14 \(German Buildings Energy Act\)"), "GModG §14 (formerly GEG)"),
    (re.compile(r"GModG 2024"), "GModG"),
]


def rewrite(path: Path, data: bytes) -> None:
    old = path.stat().st_size
    with open(path, "r+b") as handle:
        handle.write(data)
        try:
            handle.truncate(len(data))
        except OSError:
            handle.write(b" " * max(0, old - len(data)))


for name in sys.argv[1:]:
    path = ENERGY / name
    text = path.read_bytes().decode("utf-8")
    new = text
    for pattern, replacement in RULES:
        new = pattern.sub(replacement, new)
    if new != text:
        rewrite(path, new.encode("utf-8"))
        print("changed", name)
