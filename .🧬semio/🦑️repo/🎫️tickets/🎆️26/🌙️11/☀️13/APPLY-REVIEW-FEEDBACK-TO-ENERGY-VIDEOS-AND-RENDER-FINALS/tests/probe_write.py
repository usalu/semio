import os
from pathlib import Path

energy = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
for rel in ("❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py", "❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py"):
    path = energy / rel
    for mode in ("r+b", "ab"):
        try:
            with open(path, mode):
                print("ok", mode, rel.encode("ascii", "backslashreplace"))
        except OSError as error:
            print("fail", mode, error.errno, rel.encode("ascii", "backslashreplace"))
    print([c for c in os.listdir(path.parent)][:5].__len__())
    print(path.parent.name.encode("ascii", "backslashreplace"), [hex(ord(c)) for c in path.parent.name])
