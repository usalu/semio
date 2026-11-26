"""🧾 Checks the Wasm actually served to Draw against its published module."""
from hashlib import sha256
from pathlib import Path
import sys
from urllib.parse import quote
from urllib.request import urlopen

def digest(stream):
    hasher = sha256()
    for block in iter(lambda: stream.read(1024 * 1024), b""):
        hasher.update(block)
    return hasher.hexdigest()


origin, directory = sys.argv[1:]
module = Path(directory)
cores = sorted(module.glob("*.wasm"))
assert cores, module
for core in cores:
    url = f"{origin.rstrip('/')}/{quote('🔌️plugin-modules')}/{quote(module.name)}/{quote(core.name)}"
    with core.open("rb") as source, urlopen(url, timeout=45) as served:
        expected = digest(source)
        observed = digest(served)
    if expected != observed:
        raise ValueError(f"{core.name} served {observed}, expected {expected}")
    print(core.name, core.stat().st_size, observed, flush=True)
