"""Apply literal (old, new) replacements read from a python file's REPLACEMENTS list to the experiments scene; retries while another process holds the file."""
import importlib.util
import sys
import time
from pathlib import Path

root = Path.cwd()
scene = next(root.glob("*teaching/*architecture/*optimization/*clip/*experiments/*.py"))
for path in sys.argv[1:]:
    spec = importlib.util.spec_from_file_location("edits", path)
    edits = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(edits)
    text = scene.read_text(encoding="utf-8")
    for old, new in edits.REPLACEMENTS:
        count = text.count(old)
        if count != 1:
            sys.stdout.buffer.write(f"MISS {count}: {old[:70]!r}\n".encode("utf-8"))
            continue
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            scene.write_bytes(text.replace(chr(13) + chr(10), chr(10)).encode("utf-8"))
            break
        except OSError:
            time.sleep(3)
    else:
        sys.stdout.buffer.write(b"WRITE FAILED\n")
sys.stdout.buffer.write(b"done\n")
