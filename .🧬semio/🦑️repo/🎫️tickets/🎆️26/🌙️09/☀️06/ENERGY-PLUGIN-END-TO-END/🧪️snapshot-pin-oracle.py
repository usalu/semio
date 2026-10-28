"""🔏️ Independent third-party BLAKE3 verification of the neutral snapshot fixture."""
import json
from pathlib import Path
import blake3
fixture = json.loads(Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧷️actor-reservation/🔗️linked-read/🧪️tests/🔣️.json").read_text())["snapshot"]
assert blake3.blake3(bytes(fixture["bytes"])).hexdigest() == fixture["hash"]
print("[DEBUG] Independent blake3 snapshot pin oracle PASS")
