"""🧮 Recover two archived core confirmations from the previously certified exact hashes."""
import argparse
from hashlib import sha256
import json
from pathlib import Path
import runpy
import tarfile

tools = runpy.run_path(str(Path(__file__).with_name("release-receipts.py")))
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("staging", type=Path)
parser.add_argument("site", type=Path)
parser.add_argument("confirmations", type=Path)
parser.add_argument("archive", type=Path)
args = parser.parse_args()

def digest(handle):
    value = sha256()
    for chunk in iter(lambda: handle.read(1024 * 1024), b""):
        value.update(chunk)
    return value.hexdigest()

pins = {
    "flow-extension-bim": ("semio_s_plugin_flow_bim_component.core.wasm", 26059931,
                           "686297e69eb19649a39e82c9dfdec8190e3dcb462647f5b06b348d93fed15c9d"),
    "flow-extension-dictionary": ("semio_s_plugin_flow_dictionary_component.core.wasm", 25984975,
                                  "476c131ee13a6fc3cdad1cdffd3d25284f7d5dcbe5e26ff30c30f5085ea8a604"),
}
models = {}
for marker in (args.staging.parent / "components").glob(f"*/{tools['RECEIPT']}"):
    plugin = json.loads(marker.read_bytes()).get("pluginId")
    if plugin in pins:
        if plugin in models:
            raise ValueError(f"Ambiguous retained component for {plugin}")
        models[plugin] = marker

digests = tools["authored_release_digests"](list(pins))
archive_files, confirmations = {}, []
for module, (core, length, expected_hash) in pins.items():
    staged = tools["receipt_path"](args.staging, module)
    deployed = tools["receipt_path"](args.site / "🧩️extension-modules", module)
    receipt = tools["read_receipt"](staged, module)
    model = models[module]
    if tools["read_receipt"](model, module) != receipt or tools["read_receipt"](deployed, module) != receipt:
        raise ValueError(f"{module}: archived module release identity changed")
    if receipt["semanticDescriptorSha256"] != digests[module]:
        raise ValueError(f"{module}: authored descriptor changed")
    for name, expected in (("📦️component.wasm", receipt["wasmSha256"]),
                           ("🛂️.descriptor.semio", receipt["semanticDescriptorSha256"])):
        with (model.parent / name).open("rb") as source:
            if digest(source) != expected:
                raise ValueError(f"{module}: retained {name} changed")
    inventory = {"version": 1, "files": [{"path": core, "bytes": length, "sha256": expected_hash}]}
    tools["verify_recorded_cores"](staged.parent, inventory)
    tools["verify_recorded_cores"](deployed.parent, inventory)
    tools["verify_deployed_module"](staged.parent, deployed.parent)
    directory = f"🧩️extension-modules/{deployed.parent.name}"
    archive_files[f"{directory}/{core}"] = (length, expected_hash)
    archived_receipt = staged.read_bytes()
    archive_files[f"{directory}/{staged.name}"] = (len(archived_receipt), sha256(archived_receipt).hexdigest())
    confirmations.append((module, receipt, inventory))

remaining = dict(archive_files)
with tarfile.open(args.archive, "r|gz") as archive:
    for member in archive:
        name = member.name.removeprefix("./")
        if name not in archive_files:
            continue
        if name not in remaining:
            raise ValueError(f"Repeat archived file {name}")
        length, expected = remaining.pop(name)
        if not member.isfile() or member.size != length or digest(archive.extractfile(member)) != expected:
            raise ValueError(f"Unexpected archived file {name}")
if remaining:
    raise ValueError(f"Archived copies missing: {sorted(remaining)}")

for module, receipt, inventory in confirmations:
    tools["persist_confirmation"](args.confirmations / f"{module}.json", receipt, inventory)
    print(f"Confirmed archived extension {module}", flush=True)
