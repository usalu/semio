"""🧾️ Identify staged release inputs and verify the copies served by the static site."""
import argparse
import filecmp
import json
import os
from pathlib import Path
import re

RECEIPT = "🧾️receipt.json"
ROOT = Path(__file__).resolve().parents[7]
CASES = ROOT / "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json"


def read_receipt(path, module):
    data = json.loads(path.read_bytes())
    if (data["pluginId"], data["schema"], data["target"], data["profile"]) != (
        module, "semio.plugin.component-receipt/v1", "wasm32-wasip2", "release"
    ):
        raise ValueError(f"unexpected release identity at {path}")
    for key in ("wasmSha256", "semanticDescriptorSha256", "sourceTreeDigest", "provenanceSha256"):
        if not re.fullmatch("[0-9a-f]{64}", data[key]):
            raise ValueError(f"invalid {key} at {path}")
    return data


def receipt_path(directory, module):
    paths = list(directory.glob(f"*️{module}/{RECEIPT}"))
    if len(paths) != 1:
        raise ValueError(f"expected one receipt for {module} in {directory}, found {len(paths)}")
    return paths[0]


def verify_deployed_cores(staged, deployed):
    originals = {path.relative_to(staged) for path in staged.rglob("*.wasm")}
    copies = {path.relative_to(deployed) for path in deployed.rglob("*.wasm")}
    if not originals or originals != copies:
        raise ValueError(f"WebAssembly file list differs or is empty in {deployed}")
    for name in sorted(originals):
        if not filecmp.cmp(staged / name, deployed / name, shallow=False):
            raise ValueError(f"deployed WebAssembly differs from its staged build: {deployed / name}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("operation", choices=("record", "verify"))
    parser.add_argument("modules", type=Path)
    parser.add_argument("evidence", type=Path)
    parser.add_argument("module", nargs="?")
    parser.add_argument("--site", type=Path)
    args = parser.parse_args()
    if args.operation == "record":
        if not args.module or not re.fullmatch("[a-z0-9-]+", args.module):
            parser.error("record requires a module id")
        data = read_receipt(receipt_path(args.modules, args.module), args.module)
        args.evidence.mkdir(parents=True, exist_ok=True)
        receipt = args.evidence / f"{args.module}.json"
        pending = receipt.with_suffix(f".{os.getpid()}.pending")
        pending.write_text(json.dumps(data, ensure_ascii=False) + "\n")
        pending.replace(receipt)
        return
    cases = json.loads(CASES.read_bytes())
    missing = []
    for group, directory in (("pluginIds", "🔌️plugin-modules"), ("extensionIds", "🧩️extension-modules")):
        for module in cases[group]:
            receipt = args.evidence / f"{module}.json"
            if not receipt.is_file():
                missing.append(module)
                continue
            expected = read_receipt(receipt, module)
            staged = receipt_path(args.modules, module)
            if read_receipt(staged, module) != expected:
                raise ValueError(f"{module} changed since its successful build")
            if args.site:
                deployed = receipt_path(args.site / directory, module)
                if read_receipt(deployed, module) != expected:
                    raise ValueError(f"{module} site differs from its successful build")
                verify_deployed_cores(staged.parent, deployed.parent)
    if missing:
        raise ValueError("awaiting successful queue receipts for " + ", ".join(missing))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError) as error:
        raise SystemExit(str(error))
