"""📘 Bind the Stdio PDF development contribution after Rust admission is settled."""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("--apply", action="store_true")
args = parser.parse_args()
fixture_path = Path("✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf/🧪️tests/🚪️route/🧫️fixtures/🔣️.json")
route = json.loads(fixture_path.read_text(encoding="utf-8"))
manifest = Path(route["manifest"])
original = manifest.read_bytes()
newline = b"\r\n" if b"\r\n" in original else b"\n"
marker = newline + f'variant = "{route["variant"]}"'.encode() + newline
binding = f'devContribution = "{route["contribution"]}"'.encode() + newline
assert original.count(marker) == 1
if marker + binding in original:
    print("[DEBUG] PDF variant owns its development contribution")
else:
    assert binding not in original
    if args.apply:
        assert manifest.read_bytes() == original
        manifest.write_bytes(original.replace(marker, marker + binding))
    print("[DEBUG]", binding.decode().strip())
