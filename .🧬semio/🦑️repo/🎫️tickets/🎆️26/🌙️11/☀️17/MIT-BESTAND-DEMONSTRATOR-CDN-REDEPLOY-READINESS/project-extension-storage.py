"""📦️ Estimate published storage after restoring all staged CAD and process extensions."""
from pathlib import Path
import sys
import zlib


def published_bytes(path):
    """🗜️ Measure the same streaming gzip level and wrapper as the browser distributor."""
    if path.suffix != ".wasm":
        return path.stat().st_size
    encoder = zlib.compressobj(wbits=31)
    size = 0
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            size += len(encoder.compress(chunk))
    return size + len(encoder.flush())


site, modules = map(Path, sys.argv[1:])
size = sum(path.stat().st_size for path in site.rglob("*") if path.is_file())
for extension in sorted(modules.iterdir()):
    if not any(kind in extension.name for kind in ("cad-extension-", "process-extension-")):
        continue
    if (site / "🧩️extension-modules" / extension.name).exists():
        continue
    extra = sum(published_bytes(path) for path in extension.rglob("*") if path.is_file())
    size += extra
    print(extension.name, extra)
print("Estimated site bytes before binding rewrites and checksum manifest",size)
