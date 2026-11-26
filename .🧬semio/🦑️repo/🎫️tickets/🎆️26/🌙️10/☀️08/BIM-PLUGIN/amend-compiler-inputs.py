"""📥️ Capture project-relative rustc inputs outside the tracked compiler inventory."""
import hashlib
import json
import os
import shutil
import sys
import unicodedata
from pathlib import Path

root = Path.cwd()
image = Path('/Volumes/BIMInput')
out = Path(__file__).resolve().parent / '🗑️generated'
manifest = out / 'compiler-image-inputs.json'
rows = json.loads(manifest.read_bytes())
known = {unicodedata.normalize('NFC', name) for name in rows}
missing = set()
for cache in ('bs', 'bre'):
    for dep in (out / cache).glob('**/out/*.d'):
        with dep.open(encoding='utf8', errors='replace') as handle:
            line = handle.readline().partition(': ')[2]
        for part in line.split():
            name = os.path.normpath(part)
            if name.startswith(str(root) + '/'):
                name = os.path.relpath(name, root)
            name = unicodedata.normalize('NFC', name)
            if name.split('/', 1)[0] in ('🧰️framework', '🌎️hub', '✏️s') and name not in known:
                missing.add(name)
files = sorted(name for name in missing if Path(name).is_file() and not (image / name).exists())
print('Missing physical compiler inputs', len(files), flush=True)
for name in files:
    print(name, flush=True)
    if '--copy' in sys.argv:
        assert len(str(image / name)) <= 256 and not Path(name).is_symlink()
        dest = image / name
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(name, dest)
        data = dest.read_bytes()
        assert data == Path(name).read_bytes()
        rows[unicodedata.normalize('NFD', name)] = hashlib.sha256(data).hexdigest()
if '--copy' in sys.argv:
    encoded = json.dumps(rows, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()
    manifest.write_bytes(encoded)
    print('Manifest SHA-256', hashlib.sha256(encoded).hexdigest(), flush=True)
