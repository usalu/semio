from hashlib import sha256 as sha256_digest
import json
from pathlib import Path
import sys


def sha256(path):
    digest = sha256_digest()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1048576), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path):
    return json.loads(path.read_bytes())


plugin, report_path, *flags = sys.argv[1:]
assert flags in ([], ["--receipt-only"])
root = Path(".cache/b/p/m/browser/prototype")
module, = (row for row in (root / "modules").iterdir() if row.name.endswith(plugin))
receipt_name = "🧾️receipt.json"
receipt_bytes = (module / receipt_name).read_bytes()
receipt = json.loads(receipt_bytes)
assert receipt["pluginId"] == plugin and receipt["packageId"] == f"semio:{plugin}"
assert receipt["profile"] == "prototype" and receipt["target"] == "wasm32-wasip2"
wrapper, = (
    row for row in (root / "components").iterdir()
    if (row / receipt_name).is_file() and (row / receipt_name).read_bytes() == receipt_bytes
)
wasm = wrapper / "📦️component.wasm"
assert wasm.stat().st_size == receipt["wasmBytes"] and sha256(wasm) == receipt["wasmSha256"]
descriptor = "🛂️.descriptor.semio"
assert sha256(wrapper / descriptor) == sha256(module / descriptor) == receipt["semanticDescriptorSha256"]
core, = module.glob("*.core.wasm")
bridge = module / "🌉️bridge.js"
glue = core.with_suffix("").with_suffix(".js")
assert "now + admission.maximumElapsedUs" in bridge.read_text()

provenance = Path(report_path).parent / "provenance"
reads = read_json(provenance / "reads.json")
assert reads["closed"] and reads["files"] > 0
package = f"semio-hub-{plugin}-component"
owner, = (row for row in read_json(provenance / "leaves.json").values() if package in row["components"])
dependency_closure = owner["components"][package]
assert dependency_closure["root"] == receipt["provenanceSha256"]
assert dependency_closure["sourceTreeDigest"] == receipt["sourceTreeDigest"]
assert dependency_closure["units"] > 0
compiler_receipt = provenance / owner["receipt"]
assert compiler_receipt.is_file()
verified_sources = 0
if not flags:
    completed = read_json(compiler_receipt)
    assert completed["status"] == 0 and not completed["cancelled"]
    workspace = Path.cwd()
    for string_index, kind, expected in completed["inputs"]:
        requested = Path(completed["strings"][string_index])
        if kind == "file" and requested.is_relative_to(workspace):
            relative = requested.relative_to(workspace)
            if relative.parts[0] == ".cache":
                continue
            assert sha256(requested) == expected, f"Current compiler input moved: {relative}"
            verified_sources += 1
    assert verified_sources > 0
assert (module / receipt_name).read_bytes() == receipt_bytes
print(json.dumps({
    "plugin": plugin,
    "wrapper": receipt["wasmSha256"],
    "provenance": receipt["provenanceSha256"],
    "sourceTree": receipt["sourceTreeDigest"],
    "sourcesRead": reads["files"],
    "currentSourcesVerified": verified_sources,
    "unitsInClosure": dependency_closure["units"],
    "browser": {asset.name: sha256(asset) for asset in (module / receipt_name, bridge, glue, core)},
}))
