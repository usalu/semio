"""📦 Verify both editor hosts serve the freshly packaged BRep component."""
import hashlib
import sys
from pathlib import Path
from urllib.parse import quote
from urllib.request import urlopen

def digest(stream):
    identity = hashlib.sha256()
    length = 0
    for chunk in iter(lambda: stream.read(1024 * 1024), b""):
        identity.update(chunk)
        length += len(chunk)
    return length, identity.hexdigest()

def compare(root, origins):
    base = "semio_s_plugin_flow_brep_component"
    files = [*sorted(root.glob(base + ".core*.wasm")), root / (base + ".js"),
             root / "🛂️.descriptor.semio", root / "🔣️.json", root / "🌉️bridge.js"]
    assert root / (base + ".core.wasm") in files
    for path in files:
        with path.open("rb") as candidate:
            expected = digest(candidate)
        assert expected[0] > 0
        for origin in origins:
            route = "/🧩️extension-modules/🧊️flow-extension-brep/" + path.name
            with urlopen(origin + quote(route), timeout=120) as response:
                actual = digest(response)
            if actual != expected:
                raise ValueError(f"{origin} {path.name} delivered {actual}, expected {expected}")
            print("[DEBUG]", origin, path.name, *expected, flush=True)

if __name__ == "__main__":
    compare(Path(sys.argv[1]), sys.argv[2:])
