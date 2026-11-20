"""🧾 Compare the ticket census with SQLite over a changing set of worker origins."""
import json
import sqlite3
import subprocess
import sys
from pathlib import Path
from tempfile import TemporaryDirectory

scripts = Path(__file__).resolve().parent
fixture = json.loads((scripts / "worker-census.cases.json").read_text())
generated = scripts.parent / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as tmp, sqlite3.connect(":memory:") as database:
    root = Path(tmp)
    origins = root / "origins"
    origins.mkdir()
    database.execute("create table origin(id text, ready integer)")
    for item in fixture["origins"]:
        key = item["id"]
        path = origins / key
        path.mkdir()
        database.execute("insert into origin values(?, ?)", (key, item.get("ready")))
        if "ready" in item:
            receipt = {"origin": key, "digest": item.get("digest", "0"*64), "ready": item["ready"], "state": "ready" if item["ready"] else "running", "stage": None, "updatedMs": 1000}
            (path / "ready.json").write_text(json.dumps(receipt))
        elif item["attested"]:
            (path / "attest.json").write_text("{}")
    rows = database.execute("select id, ready from origin where id glob 'g1-m[0-9]*' and substr(id,5) not glob '*[^0-9]*' order by cast(substr(id,5) as integer)").fetchall()
    assert [row[0] for row in rows] == fixture["workers"]
    assert sum(bool(row[1]) for row in rows) == fixture["ready"]
    report = root / "report.md"
    report.write_text("Snapshot from the replicated origin store\n\n## Native workers\n")
    command = [sys.executable, str(scripts / "refresh-worker-receipts.py"), str(origins), str(report)]
    result = subprocess.run(command, check=True, capture_output=True, text=True)
    assert result.stdout.startswith("2/5 ready")
    original = report.read_text()
    assert [s.split("|")[1].strip() for s in original.splitlines() if s.startswith("| g1-")] == fixture["workers"]
    for item in fixture["origins"]:
        if "digest" in item:
            assert f"| {item['digest'][:12]} |" in original
    altered = origins / "g1-m11" / "ready.json"
    receipt = json.loads(altered.read_text())
    receipt["origin"] = "g1-m12"
    altered.write_text(json.dumps(receipt))
    result = subprocess.run(command, capture_output=True, text=True)
    assert result.returncode != 0 and "Inconsistent readiness receipt" in result.stderr
    assert report.read_text() == original
print("Worker census receipt, registration and identity cases passed")
