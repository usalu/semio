"""🧪️ Runs the Remodel reference handlers with the repo Python host and checks an asset-id negative control."""

import importlib.util
import copy
import json
from pathlib import Path
import sys


def load(name, path):
    """📥️ Loads a file-backed module under the name its consumers import."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def verify_custody(owner):
    """🔗️ Replays the shared asset fixture through the independent Python mutation oracle."""
    fixture = json.loads((owner / "🧪️tests/🧫️fixtures/🧫️asset-custody/🔣️.json").read_text())
    appliers = sys.modules["mutate_remodeling_1_appliers"]
    rules = sys.modules["mutate_remodeling_1_rules"]
    payloads, ids = fixture["payloads"], fixture["contentIds"]
    assert {name: appliers.asset_content_id(asset) for name, asset in payloads.items()} == ids
    leaves = {name: appliers.durable_leaf(asset) for name, asset in payloads.items()}
    checked = 0
    for case in fixture["cases"]:
        base = {
            "assets": {}, "durableArtifacts": {ids[name]: leaves[name] for name in case.get("unbound", [])},
            "calibration": {"cameras": [], "rig": []}, "streams": [], "gcps": [], "params": {},
            "results": {"mesh": {"textureAssetId": None}, "geo": None, "sparse": None, "trajectory": None, "qc": None},
        }
        for index, step in enumerate(case["steps"], 1):
            context = f"{case['id']} step {index}"
            kind = {"createAsset": "create-asset", "deleteAsset": "delete-asset"}[step["op"]]
            payload = {"key": step["key"]}
            if "asset" in step:
                payload["asset"] = payloads[step["asset"]]
            diagnostic = rules.refusal_of(kind, base, payload)
            assert diagnostic == step.get("diagnostic"), (context, diagnostic)
            after = appliers.APPLIERS[kind](base, payload) if diagnostic is None else base
            assert {key: handle["childId"] for key, handle in after["assets"].items()} == {key: ids[name] for key, name in step["bindings"].items()}, context
            assert after["durableArtifacts"] == {ids[name]: leaves[name] for name in step["leaves"]}, context
            restored = copy.deepcopy(after)
            if diagnostic is None:
                for inverse, undo_payload in rules.inverse_mutation(kind, base, payload):
                    restored = rules.apply_inverse_step(restored, inverse, undo_payload)
                    assert all(handle["childId"] in restored["durableArtifacts"] for handle in restored["assets"].values()), context
            assert restored == base, context
            base = after
            checked += 1
    print(f"{checked} custody sequence steps passed")


def main():
    """🧭️ Resolves the feature's fixture paths and checks each registered asset oracle outcome."""
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    parser = load("fixture_scenarios", ticket / "🐍️python-oracle-dryrun.py")
    host = load("semio_repo_test", root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
    owner = root / parser.OWNER_REL
    case = root / parser.CASE_REL
    adapter = load("remodel_oracle", case / "🐍️.py").adapter()
    scenarios = parser.scenarios_from((case / "🥒️.feature").read_text(encoding="utf-8"))
    input_roots = {"shared": owner / "🧪️tests/🧫️fixtures", "asset": owner / "🖼️assets"}
    failures = []
    checked = 0
    negative_control_checked = False
    for scenario in scenarios:
        payload = json.loads(scenario["steps"][0]["docString"])
        if payload["kind"] not in {"create-asset", "delete-asset"}:
            continue
        checked += 1
        inputs = []
        for key in ("before", "mutation", "after", "carrier"):
            if key in payload:
                uri = payload[key]
                scheme, path = uri.split("://", 1)
                inputs.append({"uri": uri, "path": str(input_roots[scheme] / path)})
        plan = {"workDir": str(ticket / "🗑️generated"), "inputs": inputs}
        context = host.Context(plan, scenario, "oracle", str(root))
        handler = adapter.handler(scenario, "oracle")
        try:
            assert handler(context).projection is not None
        except Exception as error:
            failures.append((scenario["id"], str(error).splitlines()[0][:230]))
            continue
        if negative_control_checked or not scenario["id"].startswith("mutate-") or payload["kind"] != "create-asset" or "code" in payload:
            continue
        negative_control_checked = True
        after = json.loads(context.input_bytes(payload["after"]))
        key = json.loads(context.input_bytes(payload["mutation"]))["key"]
        real = after["assets"][key]["childId"]
        forged = "remodeling-asset-0000000000000000"
        assert real != forged
        after["assets"][key]["childId"] = forged
        after["durableArtifacts"][forged] = after["durableArtifacts"].pop(real)
        original_input = context.input_bytes
        context.input_bytes = lambda uri: json.dumps(after).encode() if uri == payload["after"] else original_input(uri)
        try:
            handler(context)
        except AssertionError:
            continue
        failures.append((scenario["id"], "oracle accepted a forged asset and matching durable key"))
    for scenario_id, error in failures:
        print(f"[DEBUG] {scenario_id} {error}")
    print(f"{checked} asset oracle scenarios, {len(failures)} failures, forged-content control ran: {negative_control_checked}")
    verify_custody(owner)
    return int(bool(failures) or not negative_control_checked)


if __name__ == "__main__":
    sys.exit(main())
