"""🖼️ Check the actual native one-pixel solve and its durable owner bytes."""
import hashlib
import json
import math
import os
from pathlib import Path
import sqlite3
import subprocess
import tempfile
import time

generated = Path(__file__).resolve().parent / "🗑️generated"
fixture = json.loads(Path("✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🧫️initial-artifact.json").read_text())
binary = generated / "wfc-pair" / ("wfc-pinned-pair.exe" if os.name == "nt" else "wfc-pinned-pair")
with tempfile.TemporaryDirectory(dir=generated, prefix="native-pinned-") as location:
    started = time.monotonic()
    result = subprocess.run([binary, location, "wfc-seeded-solve", "--solve"],
        text=True, input=json.dumps(fixture["solve"]), capture_output=True, timeout=120)
    if result.returncode:
        raise RuntimeError(result.stderr)
    database = sqlite3.connect(":memory:")
    for field in fixture["result"]:
        path = "$." + field
        query = "select fullkey, atom from json_tree(?, ?) where type not in ('object','array') order by fullkey"
        actual = database.execute(query, (result.stdout, path)).fetchall()
        expected = database.execute(query, (json.dumps(fixture["result"]), path)).fetchall()
        assert actual == expected, (field, actual, expected)
    for value in json.loads(result.stdout)["entropy"]:
        assert math.copysign(1, value) == 1, ("entropy sign", value)
    digests = {key: hashlib.sha256((Path(location) / key).read_bytes()).hexdigest() for key in ["PACK", "SPR"]}
    record = {"seconds": time.monotonic() - started, "result": json.loads(result.stdout), "sha256": digests}
    (generated / "native-one-cell-solver.json").write_text(json.dumps(record))
    print(json.dumps(record))
