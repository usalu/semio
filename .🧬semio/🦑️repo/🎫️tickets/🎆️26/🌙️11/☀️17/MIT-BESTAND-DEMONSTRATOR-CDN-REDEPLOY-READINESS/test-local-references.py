#!/usr/bin/env python3
"""🧪️ Synthetic release local-reference validation cases."""
import importlib.util
import tempfile
from pathlib import Path

HERE = Path(__file__).parent
spec = importlib.util.spec_from_file_location("validator", HERE / "validate-local-references.py")
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
generated = HERE / "🗑️generated"
generated.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(dir=generated) as directory:
    root = Path(directory)
    assets = root / "assets"
    assets.mkdir()
    html = root / "index.html"
    js = assets / "🌐️-test.js"
    pdf = assets / "pdf-test.js"
    html.write_text("<script src=\"/assets/main.js\"></script>")
    js.write_text(";".join(validator.HOST))
    pdf.write_text(";".join(validator.PDF))
    assert validator.scan(root) == []
    for name, data in [("index.html", b"<img src=\"http://localhost/x\">"), ("assets/main.js", b"\x00fetch('http://127.0.0.1/x')"), ("assets/main.mjs", b"import 'file:///tmp/x'"), ("assets/other.js", validator.HOST[2].encode()), ("assets/main.js", b"fetch('http://[::1]/')")]:
        path = root / name
        previous = path.read_bytes() if path.exists() else None
        path.write_bytes(data)
        assert validator.scan(root), name
        if previous is None: path.unlink()
        else: path.write_bytes(previous)
    js.write_text(validator.HOST[2] + ";fetch('http://127.0.0.1:7777')")
    assert len(validator.scan(root)) == 1
print("local reference validation: synthetic cases passed")
