import argparse
import hashlib
import json
import re
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("packet")
parser.add_argument("slug")
args = parser.parse_args()
workspace = Path.cwd()
ticket = workspace / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
packet = json.loads((ticket / args.packet).read_text())
manifest = Path(packet["manifest"])
expected = packet.get("package", packet.get("newPackage"))
blocks = re.findall(r"(?ms)^\[package\]\s*(.*?)(?=^\[|\Z)", manifest.read_text())
if len(blocks) != 1:
    raise RuntimeError("Exact package table required")
names = re.findall(r'(?m)^name\s*=\s*"([^"\n]+)"\s*$', blocks[0])
if names != [expected]:
    raise RuntimeError("Exact current companion name required")
owner = {"name": expected, "root": manifest.parent.as_posix(), "manifest": manifest.as_posix(), "packages": []}
path = workspace / "📋️build-owners.json"
before = path.read_bytes()
data = json.loads(before)
matches = [row for row in data["owners"] if row["name"] == expected]
if matches and matches != [owner]:
    raise RuntimeError("Conflicting current owner declaration")
changed = not matches
if changed:
    data["owners"].append(owner)
    data["owners"].sort(key=lambda row: row["name"].encode("utf-8"))
    after = (json.dumps(data, ensure_ascii=False, indent=2) + "\n").encode()
    if path.read_bytes() != before:
        raise RuntimeError("Shared owner manifest changed; reobserve")
    path.write_bytes(after)
else:
    after = before
receipt = {"version": 1, "authority": False, "owner": owner, "changed": changed,
           "beforeSha256": hashlib.sha256(before).hexdigest(),
           "afterSha256": hashlib.sha256(after).hexdigest()}
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
(generated / (args.slug + "-owner-registration.json")).write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
(ticket / (args.slug + "-owner-registration.md")).write_text(
    "# Layout Test Owner Registration\n\n"
    "Registered the exact current Cargo manifest. Other owners and catalog providers are retained. "
    "No Source policy or budget changed; registration is not admission or execution proof.\n\n"
    + json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"owner": expected, "changed": changed, "afterSha256": receipt["afterSha256"]}))
