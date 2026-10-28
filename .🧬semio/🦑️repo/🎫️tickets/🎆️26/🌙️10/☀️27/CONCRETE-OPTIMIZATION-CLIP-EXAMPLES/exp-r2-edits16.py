import re
from pathlib import Path

scene = next(Path.cwd().glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
text = scene.read_text(encoding="utf-8")
start = text.index("        if demand:\n")
end = text.index("        layers = {", start)
new = '''        if demand:
            size = _room.label_size(s) - 1
            first = _room.text([self.tr("Cooling load", "Kühllast"), *_room.parts(found["cooling_demand"], "kWh/a", 1, german)], (0, 0, 0), size=size, color=COOL)
            second = _room.text(["÷ EER", *_room.parts(EER, "", 1, german), "=", *_room.parts(found["cooling"], "kWh/a", 1, german)], (0, 0, 0), size=size, color=COOL)
            stack = mn.VGroup(first, second).arrange(DOWN, buff=.06, aligned_edge=RIGHT)
            notes.add(_room.place(stack, P(3.2, top + slab + 1.1), "right"))
'''
REPLACEMENTS = [(text[start:end], new)]
