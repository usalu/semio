"""🧪️ Exercises the release gate with a genuine confirmed browser artifact in disposable custody."""
from pathlib import Path
import importlib.util
import json
import shutil
import sys
import tempfile

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
spec = importlib.util.spec_from_file_location("release_receipts", TICKET / "release-receipts.py")
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)
module = "flow-extension-dictionary"
modules = gate.ROOT / ".cache/b/p/m/browser/release/modules"
source = gate.receipt_path(modules, module).parent
receipt = GENERATED / "confirmed" / f"{module}.json"
with tempfile.TemporaryDirectory(prefix="core-law-", dir=GENERATED) as temporary:
    fixture = Path(temporary)
    staged = fixture / "modules" / source.name
    deployed = fixture / "site" / "🧩️extension-modules" / source.name
    copied = fixture / "confirmed" / receipt.name
    shutil.copytree(source, staged)
    copied.parent.mkdir()
    _, original = gate.read_confirmation(receipt, module)
    gate.record_confirmation(staged / gate.RECEIPT, copied, module)
    _, recorded = gate.read_confirmation(copied, module)
    assert recorded == original
    module_files = json.loads((staged / ".semio-artifact.json").read_bytes())["files"]
    for name in module_files:
        path = deployed / name
        path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(staged / name, path)
    sys.argv = ["release-receipts.py", "verify", str(staged.parent), str(copied.parent), "--site", str(fixture / "site")]
    gate.CASES = fixture / "cases.json"
    gate.CASES.write_text(json.dumps({"pluginIds": [], "extensionIds": [module]}))
    for name in module_files:
        if name.endswith(".js"):
            path = deployed / name
            path.write_bytes(path.read_bytes().replace(b"../" + "🪞️vendor".encode(), b"../../" + "🔌️plugin-modules".encode() + b"/" + "🪞️vendor".encode()))
    def retired_stage_cache(_):
        raise AssertionError("durable confirmation must verify without retained stage-cache evidence")

    gate.certify_browser_cores = retired_stage_cache
    gate.main()
    core, = [name for name in module_files if name.endswith(".core.wasm")]
    assert (staged / core).read_bytes() == (deployed / core).read_bytes()
    for root in (staged, deployed):
        with (root / core).open("ab") as f:
            f.write(bytes((0, 4, 3)) + b"tag")
    try:
        gate.main()
    except ValueError as error:
        assert "optimized" in str(error).lower(), error
    else:
        raise AssertionError("Release verification accepted a different optimized core in both locations with the same published receipt")
print("Verified the real extension and refused the matching staged/site core substitution")
