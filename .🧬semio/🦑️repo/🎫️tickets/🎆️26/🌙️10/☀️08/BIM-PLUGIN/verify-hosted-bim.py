from pathlib import Path
from hashlib import sha256
import json
import os
import re
import subprocess
import time

root = Path.cwd()
ticket_root = Path(__file__).resolve().parent
ticket = ticket_root / "🗑️generated"
room = root / "🌎️hub/🧩️compositions/🏙️bim"
contract = json.loads((room / "🧪️tests/🧫️fixtures/🛂️committed-descriptor/🔣️.json").read_text())
component = root / contract["componentDirectory"] / contract["deliverables"]["prototype"] / contract["componentName"]
linked_digest = ticket / "guest/linked-component.sha256"
def component_digest():
    digest = sha256()
    with component.open("rb") as wasm:
        for block in iter(lambda: wasm.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()

while True:
    if linked_digest.is_file() and component.is_file():
        expected_digest = linked_digest.read_text().strip()
        if re.fullmatch(r"[0-9a-f]{64}", expected_digest) and component_digest() == expected_digest:
            break
    time.sleep(5)

staged = ticket / "bim-descriptor-current"
staged.mkdir(exist_ok=True)
emitter = ticket / "host/debug/semio-plugin-describe"
print(f"Emitting the semantic descriptor from the linked BIM Wasm ({expected_digest}).", flush=True)
subprocess.run([str(emitter), "describe", str(component), "--out", str(staged)], check=True)
assert component_digest() == expected_digest
subprocess.run(["repo", "tools", "exec", "--light", "bun", "--", str(ticket_root / "verify-bim-pair.ts"), str(staged)], check=True)

descriptor = json.loads((staged / "🔣️.json").read_text())
manifest = descriptor["manifest"]
assert manifest["pluginId"] == contract["pluginId"]
assert sorted(app["id"] for app in manifest["apps"]) == contract["applicationIds"]
records = []
for entry in descriptor["contributions"]["ioEntries"]:
    dialect = entry["counterpart"]
    records.append([entry["direction"], f'{dialect["artifactKind"]}@{dialect["standard"]}/{dialect["subset"]}'])
assert sorted(records) == contract["ioHops"]
assets = root / "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets"
expected_examples = {name: (assets / folder / "🗣️.dsl.semio").read_text()
                     for name, folder in {"demo": "🎬️demo", "house": "🏡️house", "office": "🏢️office"}.items()}
assert {entry["id"]: entry["artifactJson"] for entry in manifest["examples"]} == expected_examples
print("The emitted examples, applications and IO routes match their source witnesses.", flush=True)

owner = (root / contract["descriptorPath"]).parent
pair = ["🛂️.descriptor.semio", "🔣️.json"]
pending = {}
previous_pair = {}
emitted_pair = {}
for filename in pair:
    file = owner / filename
    original_bytes = file.read_bytes()
    new_bytes = (staged / filename).read_bytes()
    if original_bytes != new_bytes:
        previous_pair[file] = original_bytes
        emitted_pair[file] = new_bytes
        pending[file] = staged / f".bim-desc-{len(pending)}-{os.getpid()}"
        with pending[file].open("xb") as output:
            output.write(new_bytes)
            output.flush()
            os.fsync(output.fileno())
published = []
try:
    for file, temporary in pending.items():
        assert file.read_bytes() == previous_pair[file]
        temporary.replace(file)
        published.append(file)
except BaseException:
    for file in published:
        if file.read_bytes() == emitted_pair[file]:
            pending[file].write_bytes(previous_pair[file])
            pending[file].replace(file)
    raise
finally:
    for temporary in pending.values():
        temporary.unlink(missing_ok=True)
print("The emitted descriptor pair is published.", flush=True)
descriptor_digest = staged / f".component-{os.getpid()}.sha256"
descriptor_digest.write_text(expected_digest + "\n")
descriptor_digest.replace(staged / "component.sha256")

env = dict(os.environ, SEMIO_BIM_GUEST_COMPONENT=str(component), CARGO_MANIFEST_DIR=str(room / "📦️packages/🦀️rust"))
subprocess.run([str(ticket / "native-host-smoke"), "--nocapture"], env=env, check=True)
