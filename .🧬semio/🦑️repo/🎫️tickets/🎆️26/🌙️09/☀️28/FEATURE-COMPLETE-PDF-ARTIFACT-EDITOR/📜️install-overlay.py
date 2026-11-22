"""📦 Installs exact-preimage overlay rows on the shared replica without Git mutation."""
import hashlib, json, os, sys, tempfile
from pathlib import Path

workspace = Path.cwd().resolve()
rows = json.loads(Path(sys.argv[2]).read_text(encoding="utf8"))
apply = sys.argv[1] == "--install"
bad = []
for path, preimage, body in rows:
    target = workspace / path
    assert len(str(target).encode("utf-16-le")) // 2 <= 256 and "AGENTS.md" not in target.parts, target
    data = target.read_bytes() if target.is_file() else None
    state = "missing" if data is None else "pre" if hashlib.sha256(data).hexdigest() == preimage else "done" if data == body.encode("utf8") else "drift"
    print(state, path[-80:])
    if state in ("missing", "drift"):
        bad.append(path)
if apply and not bad:
    for path, _, body in rows:
        target = workspace / path
        if target.read_bytes() == body.encode("utf8"):
            continue
        fd, name = tempfile.mkstemp(dir=target.parent)
        with os.fdopen(fd, "wb") as handle:
            handle.write(body.encode("utf8"))
        os.replace(name, target)
sys.exit(1 if bad else 0)
