"""🪢 Confirm unchanged extension builds against their authored descriptors and delivered cores."""
import argparse
import json
from pathlib import Path
import runpy

receipt_tools = runpy.run_path(str(Path(__file__).with_name("release-receipts.py")))
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("staging", type=Path)
parser.add_argument("site", type=Path)
parser.add_argument("confirmations", type=Path)
parser.add_argument("modules", nargs="*")
args = parser.parse_args()

extension_ids = json.loads(receipt_tools["CASES"].read_bytes())["extensionIds"]
chosen = args.modules or extension_ids
if len(chosen) != len(set(chosen)) or not set(chosen).issubset(extension_ids):
    parser.error("expected distinct authored extension IDs")

digests = receipt_tools["authored_release_digests"](chosen)
sources, requests = [], []
for module in chosen:
    staged = receipt_tools["receipt_path"](args.staging, module)
    deployed = receipt_tools["receipt_path"](args.site / "🧩️extension-modules", module)
    receipt = receipt_tools["read_receipt"](staged, module)
    if receipt["semanticDescriptorSha256"] != digests[module]:
        raise ValueError(f"{module}: authored descriptor changed since staging")
    sources.append((module, staged, deployed, receipt))
    requests.append(receipt_tools["core_certification_request"](staged, receipt))

inventories = receipt_tools["certify_browser_cores"](requests)
if len(inventories) != len(sources):
    raise ValueError("Extension certification returned an incomplete inventory")

for (module, staged, deployed, receipt), inventory in zip(sources, inventories):
    if receipt_tools["read_receipt"](staged, module) != receipt or receipt_tools["read_receipt"](deployed, module) != receipt:
        raise ValueError(f"{module}: receipt changed during certification")
    receipt_tools["verify_recorded_cores"](staged.parent, inventory)
    receipt_tools["verify_recorded_cores"](deployed.parent, inventory)
    receipt_tools["verify_deployed_module"](staged.parent, deployed.parent)
    receipt_tools["persist_confirmation"](args.confirmations / f"{module}.json", receipt, inventory)
    print(f"Confirmed staged extension {module}", flush=True)
