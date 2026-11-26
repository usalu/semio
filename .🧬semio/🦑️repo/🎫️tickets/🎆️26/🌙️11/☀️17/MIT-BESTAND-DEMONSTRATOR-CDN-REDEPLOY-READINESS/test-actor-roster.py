"""🧪 Check the JavaScript actor roster against the published JSON using SQLite."""
import json
from pathlib import Path
import sqlite3
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
fixture = root / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"
database = sqlite3.connect(":memory:")
with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as scratch:
    site = Path(scratch)
    for key, folder in [("pluginIds", "🔌️plugin-modules"), ("extensionIds", "🧩️extension-modules")]:
        for (plugin,) in database.execute("select value from json_each(?,?)", (fixture.read_text(), "$." + key)):
            path = site / folder / plugin / "🧾️receipt.json"
            path.parent.mkdir(parents=True)
            path.write_text(json.dumps({"pluginId": plugin}))
    command = ["repo", "tools", "exec", "--light", "node", "--", "-e",
               "const rows=require(process.argv[1]).actorRoster(process.argv[2],false);console.log(JSON.stringify(rows));",
               str(ticket / "probe-release-actors.cjs"), str(site)]
    good = subprocess.run(command, capture_output=True, text=True)
    assert good.returncode == 0, good.stderr
    rows = json.loads(good.stdout)
    assert sorted(row["name"] for row in rows) == sorted(path.parent.name for path in site.rglob("🧾️receipt.json"))
    path.write_text(json.dumps({"pluginId": "replacement"}))
    bad = subprocess.run(command, capture_output=True, text=True)
    assert bad.returncode and "actor roster" in bad.stderr, bad
print("Verified exact actor membership and equal-count substitution against SQLite.")
