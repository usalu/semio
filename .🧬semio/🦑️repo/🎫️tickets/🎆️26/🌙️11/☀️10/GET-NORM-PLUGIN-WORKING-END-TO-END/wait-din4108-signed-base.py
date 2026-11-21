"""🔏 Wait for a changed signed Norm receipt and pin the exact Play delivery."""

from hashlib import sha256
import json
from pathlib import Path
import sys
import time
from urllib.error import URLError
from urllib.parse import quote
from urllib.request import Request, urlopen

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PROTOTYPE = ROOT / ".cache/b/p/m/browser/prototype"
ORIGIN, BASELINE = sys.argv[1:3]
STEM = sys.argv[3] if len(sys.argv) > 3 else "din4108-signed-successor"
PLUGIN = sys.argv[4] if len(sys.argv) > 4 else "norm"
LABEL, MODULE, CORE = {
    "norm": ("Base", "📕️norm", "semio_hub_norm_component"),
    "norm-energy": ("Energy", "📒️norm-energy", "semio_hub_norm_energy_component"),
}[PLUGIN]
RECEIPT = "🧾️receipt.json"
FILES = (RECEIPT, "🌉️bridge.js", CORE + ".js", CORE + ".core.wasm")
OUT = TICKET / "🗑️generated"


def request(name):
    path = quote(f"/🔌️plugin-modules/{MODULE}/{name}", safe="/")
    return urlopen(Request(ORIGIN + path, headers={"Cache-Control": "no-cache"}), timeout=25)


def digest(stream):
    result = sha256()
    while chunk := stream.read(1024 * 1024):
        result.update(chunk)
    return result.hexdigest()


def local_digest(path):
    with path.open("rb") as source:
        return digest(source)


def bind(receipt_bytes, receipt):
    module = PROTOTYPE / "modules" / MODULE
    wrapper = next((p for p in (PROTOTYPE / "components").iterdir()
                    if (p / RECEIPT).is_file() and (p / RECEIPT).read_bytes() == receipt_bytes), None)
    if wrapper is None or (module / RECEIPT).read_bytes() != receipt_bytes:
        raise ValueError("Browser and wrapper stage receipts differ")
    if local_digest(wrapper / "📦️component.wasm") != receipt["wasmSha256"]:
        raise ValueError("Signed wrapper digest differs from installed bytes")
    served = {}
    for name in FILES:
        with request(name) as source:
            served[name] = digest(source)
        if served[name] != local_digest(module / name):
            raise ValueError(f"Served {name} differs from installed bytes")
    if sha256(receipt_bytes).hexdigest() != served[RECEIPT]:
        raise ValueError("Signed receipt changed during delivery verification")
    return served


print(f"[DEBUG] Waiting on Play {ORIGIN} for a signed {LABEL} successor to {BASELINE}", flush=True)
previous = None
while True:
    try:
        with request(RECEIPT) as source:
            receipt_bytes = source.read()
        receipt = json.loads(receipt_bytes)
        candidate = receipt.get("wasmSha256")
        if receipt.get("schema") != "semio.plugin.component-receipt/v1" or receipt.get("pluginId") != PLUGIN:
            raise ValueError(f"{LABEL} component receipt is unavailable")
        if not candidate or not receipt.get("sourceTreeDigest") or not receipt.get("provenanceSha256"):
            raise ValueError(f"{LABEL} receipt lacks wrapper or provenance digest")
        if candidate != BASELINE and receipt_bytes == previous:
            served = bind(receipt_bytes, receipt)
            (OUT / f"{STEM}-receipt.json").write_bytes(receipt_bytes)
            proof = {"origin": ORIGIN, "component": receipt, "servedSha256": served}
            (OUT / f"{STEM}-delivery.json").write_text(json.dumps(proof, indent=2) + "\n")
            print("[DEBUG] " + json.dumps(proof), flush=True)
            break
        if candidate != BASELINE and receipt_bytes != previous:
            print(f"[DEBUG] Candidate {LABEL} receipt {candidate}", flush=True)
        previous = receipt_bytes
    except (OSError, URLError, ValueError) as error:
        previous = None
        print(f"[DEBUG] Waiting for a coherent {LABEL} delivery: {error}", flush=True)
    time.sleep(10)
