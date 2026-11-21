"""🔏 Replay representative dev lanes against stable signed Play deliveries."""

import argparse
from hashlib import sha256
import json
import os
from pathlib import Path
import subprocess
import time
from urllib.parse import quote
from urllib.request import Request, urlopen

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUTPUT = TICKET / "🗑️generated"
MODULES = ROOT / ".cache/b/p/m/browser/prototype/modules"
RECEIPT = "🧾️receipt.json"
ROUTES = {
    "din4108": ("📕️norm", "norm"),
    "din18599": ("📒️norm-energy", "norm-energy"),
    "en1990": ("📗️norm-eurocode", "norm-eurocode"),
    "en1992": ("📓️norm-eurocode-structural", "norm-eurocode-structural"),
    "en1995": ("📙️norm-eurocode-materials", "norm-eurocode-materials"),
}
ENERGY = ("din18599",)
GROUP = ("en1990", "en1992", "en1995")


def snapshot(origin, routes):
    """🧾 Pair exact HTTP receipts with the current installed module receipts."""
    result = {}
    for route in routes:
        module, plugin = ROUTES[route]
        url = origin + quote(f"/🔌️plugin-modules/{module}/{RECEIPT}", safe="/")
        with urlopen(Request(url, headers={"Cache-Control": "no-cache"}), timeout=15) as response:
            served = response.read()
        if served != (MODULES / module / RECEIPT).read_bytes():
            raise ValueError(f"{route} served receipt differs from the local publication")
        receipt = json.loads(served)
        if (receipt.get("schema") != "semio.plugin.component-receipt/v1"
                or receipt.get("pluginId") != plugin
                or not all(receipt.get(key) for key in ("wasmSha256", "sourceTreeDigest", "provenanceSha256"))):
            raise ValueError(f"{route} lacks a complete signed component receipt")
        result[route] = served
    return result


def wait_for(origin, routes, ready):
    """⏱️ Require matching receipts twice across a publication poll interval."""
    previous = None
    while True:
        try:
            current = snapshot(origin, routes)
            if current == previous and ready(current):
                return current
            previous = current
        except (OSError, ValueError) as error:
            previous = None
            print(f"[DEBUG] Waiting on coherent lane receipts {error}", flush=True)
        time.sleep(10)


def replay(origin, routes, expected, stem):
    """🧪 Verify exact delivery around the existing native lane workflow."""
    name = stem + "-" + "-".join(routes)
    env = {**os.environ, "PLAYWRIGHT_BASE_URL": origin, "ONLY_LANES": " ".join(routes)}
    command = ["repo", "tools", "exec", "node", "--", str(TICKET / "verify-play-components.mjs")]
    if snapshot(origin, routes) != expected:
        raise ValueError("Published receipt moved before lane replay")
    for phase, suffix in (("delivery", "before"), ("lanes", "lanes"), ("delivery", "after")):
        with (OUTPUT / f"{name}-{suffix}.log").open("wb") as log:
            subprocess.run([*command, phase, *routes], cwd=ROOT, env=env, stdout=log,
                           stderr=subprocess.STDOUT, check=True)
        if snapshot(origin, routes) != expected:
            raise ValueError(f"Published receipt moved during {suffix}")
    existing = OUTPUT / ("verified-" + sha256(",".join(routes).encode()).hexdigest()[:8] + "-lanes.log")
    lane_log = existing.read_text()
    (OUTPUT / f"{name}-steps.log").write_text(lane_log)
    for route in routes:
        for phase in ("session", "prepare", "activate"):
            if f" {phase}-{route} rc=0" not in lane_log:
                raise ValueError(f"Missing {phase} success for {route}")
    result = {route: {"receiptSha256": sha256(expected[route]).hexdigest(),
                      "component": json.loads(expected[route])} for route in routes}
    (OUTPUT / f"{name}-receipts.json").write_text(json.dumps(result, indent=2) + "\n")
    print("[DEBUG] Signed lane verification passed for " + ", ".join(routes), flush=True)


def main():
    """🚦 Start with Energy and isolate any in-flight grouped publication."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("origin")
    parser.add_argument("energy_baseline")
    parser.add_argument("--expect-eurocode-group", action="store_true")
    parser.add_argument("--stem", default="signed-dev-lanes")
    args = parser.parse_args()
    origin = args.origin.rstrip("/")
    group_baseline = snapshot(origin, GROUP) if args.expect_eurocode_group else {}

    def new_energy(rows):
        return json.loads(rows["din18599"])["wasmSha256"] != args.energy_baseline

    def new_group(rows):
        return not group_baseline or all(rows[route] != group_baseline[route] for route in GROUP)

    print("[DEBUG] Waiting for a new signed Energy receipt", flush=True)
    energy = wait_for(origin, ENERGY, new_energy)
    try:
        current = snapshot(origin, ROUTES)
    except (OSError, ValueError) as error:
        if not group_baseline:
            raise
        print(f"[DEBUG] Group receipt transition {error}", flush=True)
        current = None
    if current and new_energy(current) and new_group(current):
        replay(origin, tuple(ROUTES), current, args.stem)
    else:
        replay(origin, ENERGY, energy, args.stem)
        print("[DEBUG] Waiting for the grouped Eurocode publication", flush=True)
        group = wait_for(origin, GROUP, new_group)
        replay(origin, GROUP, group, args.stem)


if __name__ == "__main__":
    main()
