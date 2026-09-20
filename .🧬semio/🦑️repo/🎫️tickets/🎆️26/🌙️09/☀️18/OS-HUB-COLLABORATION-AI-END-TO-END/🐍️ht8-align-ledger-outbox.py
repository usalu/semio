"""🔧️ Re-derives the GIS ledger fixture's `outbox` block from the WAL proof fixture's producer
read-out, so one job id names ONE approval across both corpora. HT4 regenerated
`🧾️inference-wal-proof-v1` from the real producer (real CreateRegion/DeleteRegion payloads in
place of the `010203` placeholders); the ledger corpus kept the placeholder-derived digests, so a
committed witness stopped matching its own prepared outbox row. Only the five coupled keys move."""

import binascii
import hashlib
import json
import pathlib
import re

root = pathlib.Path("/Users/ueli/Documents/semio/🌎️hub/🧫️fixtures")
wal = json.loads((root / "🧾️inference-wal-proof-v1" / "🔣️.json").read_text(encoding="utf-8"))
ledger_path = root / "🗺️gis-inference-job-v1" / "🔣️.json"
text = ledger_path.read_text(encoding="utf-8")

proposal = binascii.unhexlify(wal["command"]["diff"]["payloadHex"]).decode("utf-8")
proposal_hash = hashlib.sha256(proposal.encode("utf-8")).hexdigest()
assert proposal_hash == wal["proposalHash"], (proposal_hash, wal["proposalHash"])

replacements = {
    "proposal": json.dumps(proposal),
    "commandHex": json.dumps(wal["encodedHex"]),
    "proposalHash": json.dumps(wal["proposalHash"]),
    "commandHash": json.dumps(wal["commandHash"]),
    "mutationId": json.dumps(wal["command"]["mutationId"]),
}

start = text.index('"outbox": {')
end = text.index("\n  },", start)
block = text[start:end]
for key, value in replacements.items():
    pattern = re.compile(r'("%s": )"[^"]*"' % key)
    assert pattern.search(block), key
    block = pattern.sub(lambda match, value=value: match.group(1) + value, block, count=1)
ledger_path.write_text(text[:start] + block + text[end:], encoding="utf-8")

check = json.loads(ledger_path.read_text(encoding="utf-8"))["outbox"]
for key in replacements:
    print(key, "=", json.dumps(check[key])[:96])
print("jobId unchanged:", check["jobId"] == wal["jobId"])
