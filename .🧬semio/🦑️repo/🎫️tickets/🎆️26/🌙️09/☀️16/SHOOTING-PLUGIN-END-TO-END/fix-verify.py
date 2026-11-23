"""🩹 Points snapshot verification at the SQLite suite that actually exists."""
import os
from pathlib import Path

path = next(p for p in Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting").rglob("*.ts") if "snapshot-sqlite-source" in p.read_text(errors="replace") and "tsc" in p.read_text(errors="replace"))
text = path.read_text()
marker = 'resolve(snapshot,"'
second = text.find(marker, text.find(marker) + len(marker))
end = text.find('")', second)
current = text[second + len(marker):end]
plugin = Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting")
test = next(p for p in plugin.rglob("*.ts") if p.parent.name.endswith("tests") and "snapshot" in p.as_posix() and "sqlite" in p.as_posix() and "independent" not in p.as_posix() and "every document facet owns emblem" in p.read_text(errors="replace"))
snapshot = next(p.parent for p in plugin.rglob("*.ts") if p.parent.name.endswith("snapshot") and p.parent.parent.name.endswith("schema") and p.name.endswith(".ts"))
relative = Path(os.path.relpath(test, snapshot)).as_posix()
print("current", current)
print("relative", relative)
print("target exists", (snapshot / relative).exists())
updated = text[:second + len(marker)] + relative + text[end:]
if updated != text:
    path.write_text(updated)
    print("updated")
