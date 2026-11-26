"""🧪️ Pin the measured retired receipt guard against a language-neutral inventory."""
import json
from pathlib import Path
import runpy
import sqlite3
import tempfile

ticket = Path(__file__).resolve().parent
retired = json.loads((ticket / "retired-release-wasm.json").read_bytes())["wasmSha256ByPlugin"]
api = runpy.run_path(str(ticket / "release-receipts.py"))
test_digest = "b" * 64
with sqlite3.connect(":memory:") as database:
    database.execute("create table previous(plugin text, raw text, primary key(plugin,raw))")
    database.executemany("insert into previous values(?,?)", sorted(retired.items()))
    with tempfile.TemporaryDirectory(prefix="rf-", dir=ticket / "🗑️generated") as root:
        root = Path(root)
        for plugin, old in retired.items():
            directory = root / "staged" / ("🌊️" + plugin)
            directory.mkdir(parents=True)
            staged = directory / api["RECEIPT"]
            evidence = root / "confirmed"
            evidence.mkdir(exist_ok=True)
            confirmation = evidence / (plugin + ".json")
            row = {"pluginId": plugin, "schema": "semio.plugin.component-receipt/v1", "target": "wasm32-wasip2",
                   "profile": "release", "semanticDescriptorSha256": test_digest,
                   "sourceTreeDigest": test_digest, "provenanceSha256": test_digest}
            for raw in (old, test_digest):
                row["wasmSha256"] = raw
                staged.write_text(json.dumps(row))
                confirmation.write_bytes(staged.read_bytes())
                (expected,) = database.execute("select exists(select 1 from previous where plugin=? and raw=?)", (plugin, raw)).fetchone()
                missing, _ = api["missing_report"](root / "staged", evidence, {"pluginIds": [], "extensionIds": [plugin]})
                assert (missing == [plugin]) == bool(expected), (plugin, expected, missing)

            row["wasmSha256"] = old
            staged.write_text(json.dumps(row))
            recording = api["record_confirmation"]
            def impossible(*_):
                raise AssertionError("Archived graph should be refused before any costly core certification")
            recording.__globals__["certify_browser_cores"] = impossible
            try:
                recording(staged, confirmation, plugin)
            except ValueError as failure:
                assert "retired" in str(failure)
            else:
                raise AssertionError("Previously faulty release was recorded again")
print(f"{len(retired)} retired raw receipts and fresh controls matched SQL reference")
