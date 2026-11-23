"""🔎 Prints the snapshot SQLite verify command paths."""
from pathlib import Path
root = Path("/home/deck/semio/✏️s/🔌️plugins/🎥️shooting")
for path in root.rglob("*.ts"):
    text = path.read_text(errors="replace")
    if "snapshot-sqlite-source" in text and "tsc" in text:
        print(path)
        print(text)
