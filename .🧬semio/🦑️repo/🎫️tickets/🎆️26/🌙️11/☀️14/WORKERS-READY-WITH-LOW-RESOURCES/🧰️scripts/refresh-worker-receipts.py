"""🧾 Refresh the ticket’s fleet table from the replicated origin receipts."""
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

origins = Path(sys.argv[1])
report = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(__file__).resolve().parent.parent / "verification-2026-11-19.md"
rows = []
ready = 0
workers = sorted((entry for entry in origins.iterdir() if entry.is_dir() and re.fullmatch(r"g1-m[0-9]+", entry.name)), key=lambda entry: int(entry.name.split("-m")[1]))
if not workers:
    raise ValueError("No registered group-one workers are visible")
for directory in workers:
    origin = directory.name
    path = directory / "ready.json"
    moment = "—"
    digest = "—"
    if path.is_file():
        receipt = json.loads(path.read_bytes())
        if receipt["origin"] != origin or type(receipt["ready"]) is not bool or receipt["ready"] != (receipt["state"] == "ready"):
            raise ValueError(f"Inconsistent readiness receipt for {origin}")
        if re.fullmatch(r"[a-f0-9]{64}", receipt.get("digest", "")) is None:
            raise ValueError(f"Invalid bootstrap digest for {origin}")
        digest = receipt["digest"][:12]
        ready += int(receipt["ready"])
        stage = receipt.get("stage")
        state = "Ready" if receipt["ready"] else receipt["state"] + (f" ({stage})" if stage else "")
        moment = datetime.fromtimestamp(receipt["updatedMs"] / 1000, timezone.utc).strftime("%Y-%m-%d %H:%M:%S")
    elif (directory / "attest.json").is_file():
        state = "Attested, awaiting readiness receipt"
    else:
        state = "No attestation or readiness receipt"
    rows.append(f"| {origin} | {state} | {digest} | {moment} |")
now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
table = "\n".join([
    f"Snapshot from the replicated origin store, taken {now}.", "",
    "| Worker | Last reported state | Bootstrap digest | Receipt time (UTC) |", "| --- | --- | --- | --- |", *rows, "", "",
])
source = report.read_text(encoding="utf-8")
start = source.index("Snapshot from the replicated origin store")
end = source.index("## Native workers", start)
updated = re.sub(r"\d+ latest receipts report ready", f"{ready} latest receipts report ready", source[:start] + table + source[end:])
report.write_text(updated, encoding="utf-8")
print(f"{ready}/{len(workers)} ready at {now}")
