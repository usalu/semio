"""🧯️ S20: moves the faults overlay from record set v2 (`s20-patch-fault-code-v2.py`, snapshot `s20-fault-code-v2/`) to v3
(`s20-patch-fault-code.py`: the record carries `parameters`) — every v2 new file, whole-file replacement and hunk result
the overlay still holds is replaced by its v3 form. Idempotent. Usage: python3 s20-fault-code-v3-overlay.py"""
import importlib.util
import sys
from pathlib import Path

OVERLAY = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults"
sys.argv = [sys.argv[0], "--root", OVERLAY]
HERE = Path(__file__).resolve().parent


def load(name: str):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), HERE / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


v2, v3 = load("s20-patch-fault-code-v2"), load("s20-patch-fault-code")
root = Path(OVERLAY)
texts: dict[str, str] = {}
notes: list[str] = []
for (rel, old_source), (_, new_source) in zip(v2.NEW_FILES + [(r, s) for r, _, s in v2.WHOLE], v3.NEW_FILES + [(r, s) for r, _, s in v3.WHOLE]):
    current, old, new = (root / rel).read_text(), v2.block(old_source), v3.block(new_source)
    if current == new:
        notes.append(f"v3        {rel.split('/')[-1]}")
    elif current == old:
        texts[rel] = new
        notes.append(f"upgrade   {rel.split('/')[-2]}/{rel.split('/')[-1]}")
    else:
        notes.append(f"CONFLICT  {rel}")
for (rel, _, old_after), (_, _, new_after) in zip(v2.hunks(), v3.hunks()):
    if old_after == new_after:
        continue
    text = texts.setdefault(rel, (root / rel).read_text())
    if new_after != "" and text.count(new_after) == 1:
        notes.append(f"v3        hunk {rel.split('/')[-2]}")
    elif text.count(old_after) == 1:
        texts[rel] = text.replace(old_after, new_after)
        notes.append(f"upgrade   hunk {rel.split('/')[-2]}: {new_after.strip().splitlines()[0][:70]}")
    else:
        notes.append(f"CONFLICT  hunk {rel}: v2 result found {text.count(old_after)}×")
print("\n".join(notes))
if any(line.startswith("CONFLICT") for line in notes):
    raise SystemExit("conflict: nothing written")
for rel, text in texts.items():
    if text != (root / rel).read_text():
        (root / rel).write_text(text)
print(f"upgraded {sum(1 for rel, text in texts.items())} files")
