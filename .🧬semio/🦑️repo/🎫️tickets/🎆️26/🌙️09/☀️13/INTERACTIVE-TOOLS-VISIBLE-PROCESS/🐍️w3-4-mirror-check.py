"""🪞️ W3-4 mirrors: runs the independent Python remodeling reference over every `@id-mutate`/`@id-inverse` row of
`📸️mutate-remodeling-1/🥒️.feature` through the real Python test host `Context`, then cross-checks the reference against
the TypeScript twin on synthesized content and commit cases (`🐍️w3-4-mirror-twin-probe.ts`)."""
import importlib.util
import json
import os
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
ANY = os.path.join(ROOT, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
CASE = os.path.join(ANY, "🧪️tests/📸️mutate-remodeling-1")
TICKET = os.path.dirname(os.path.abspath(__file__))
HOST = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


host = load("semio_repo_test", HOST)
reference = load("remodeling_reference", os.path.join(CASE, "🐍️.py"))


def scenarios():
    text = open(os.path.join(CASE, "🥒️.feature"), encoding="utf-8").read()
    rows = []
    for block in re.split(r"\n  @id-", text)[1:]:
        tag = block.split("\n", 1)[0].strip()
        if tag not in ("mutate", "inverse"):
            continue
        doc = block.split('"""', 2)[1]
        table = block.split("Examples:", 1)[1].strip().splitlines()
        header = [cell.strip() for cell in table[0].split("|")[1:-1]]
        for line in table[1:]:
            if not line.strip().startswith("|"):
                break
            values = dict(zip(header, [cell.strip() for cell in line.split("|")[1:-1]]))
            filled = doc
            for key, value in values.items():
                filled = filled.replace(f"<{key}>", value)
            rows.append((f"{tag}-{values['id']}", filled))
    return rows


def run_feature():
    adapter = reference.adapter()
    results = {"passed": 0, "failed": []}
    planned = scenarios()
    for scenario_id, doc in planned:
        spec = json.loads(doc)
        uris = [spec[key] for key in ("before", "mutation", "after") if key in spec]
        fixtures = [{"uri": uri, "path": os.path.relpath(os.path.join(ANY, "🧫️fixtures", uri[len("shared://") :]), ROOT)} for uri in uris]
        plan = {"workDir": os.path.join(TICKET, "🗑️generated/W3-4-mirrors/work"), "fixtures": fixtures, "case": "mutate-remodeling-1"}
        scenario = {"id": scenario_id, "steps": [{"docString": doc}]}
        handler = adapter.handler(scenario_id, "oracle")
        try:
            assert handler is not None, "unregistered"
            handler(host.Context(plan, scenario, "oracle", ROOT))
            results["passed"] += 1
        except Exception as error:
            results["failed"].append((scenario_id, str(error)[:400]))
    registered = {name for name in reference.MUTATE_SCENARIOS} | {name for name in reference.INVERSE_SCENARIOS}
    planned_ids = {scenario_id.split("-", 1)[1] for scenario_id, _ in planned}
    return results, len(planned), sorted(registered ^ planned_ids)


def committed_quartets():
    root = os.path.join(ANY, "🧫️fixtures/🧬️mutations")
    for kind_dir in sorted(os.listdir(root)):
        for case in sorted(os.listdir(os.path.join(root, kind_dir))):
            base = os.path.join(root, kind_dir, case)
            read = lambda *parts: json.load(open(os.path.join(base, *parts), encoding="utf-8"))
            yield kind_dir, case, read("📸️snapshot", "⬅️before", "🔣️.json"), read("🦠️mutation", "🔣️.json"), read("📸️snapshot", "➡️after", "🔣️.json"), read("🎯️outcome", "🔣️.json")


def run_quartets():
    """🧫️ Exact equality against every committed quartet of the kinds this wave changed."""
    changed = ("📦append-content", "🔪remove-content", "🏁commit-reconstruction", "🧷create-asset", "🗞️delete-asset", "🧱replace-mesh-result")
    report = []
    for kind_dir, case, before, mutation, after, outcome in committed_quartets():
        if kind_dir not in changed:
            continue
        kind = kind_dir.lstrip("📦🔪🏁🧷🗞️🧱")
        tag, payload = reference.unwrap(mutation)
        verdict = reference.refusal_of(kind, before, payload)
        expected = outcome.get("code") or next((message["code"] for message in outcome.get("messages", [])), None)
        if verdict is not None:
            ok = verdict == expected and after == before
        else:
            produced = reference.APPLIERS[kind](before, payload)
            if kind == "create-asset":
                produced = reference._mint_asset(before, produced, payload["key"], after["assets"][payload["key"]]["childId"], payload["asset"])
            ok = expected is None and produced == after
            if ok:
                current = produced
                for step_kind, step_payload in reference.inverse_mutation(kind, before, payload):
                    current = reference.apply_inverse_step(current, step_kind, step_payload)
                ok = current == before
        report.append((f"{kind_dir}/{case}", verdict, expected, ok))
    return report


def run_twin():
    """🪞️ Synthesized content/commit cases answered by both the Python reference and the TypeScript twin."""
    probe = os.path.join(TICKET, "🐍️w3-4-mirror-twin-probe.ts")
    completed = subprocess.run(["bun", probe], capture_output=True, text=True, cwd=ROOT)
    assert completed.returncode == 0, completed.stderr
    cases = json.loads(completed.stdout)
    report = []
    for case in cases:
        kind = case["kind"]
        tag, payload = reference.unwrap(case["mutation"])
        verdict = reference.refusal_of(kind, case["before"], payload)
        twin_codes = [message["code"] for message in case["messages"]]
        python_after = case["before"] if verdict is not None else reference.APPLIERS[kind](case["before"], payload)
        agree = (verdict is None and twin_codes == []) or twin_codes == [verdict]
        agree = agree and python_after == case["after"]
        inverse_ok = None
        if verdict is None:
            current = python_after
            for step_kind, step_payload in reference.inverse_mutation(kind, case["before"], payload):
                current = reference.apply_inverse_step(current, step_kind, step_payload)
            inverse_ok = current == case["before"]
        report.append((case["name"], verdict, twin_codes, agree, inverse_ok))
    return report


if __name__ == "__main__":
    feature, planned, drift = run_feature()
    print(f"feature rows planned={planned} passed={feature['passed']} failed={len(feature['failed'])} registration-drift={drift}")
    for scenario_id, message in feature["failed"]:
        print(f"  FAIL {scenario_id}: {message}")
    quartets = run_quartets()
    print(f"committed quartets of changed kinds={len(quartets)} exact={sum(1 for row in quartets if row[3])}")
    for name, verdict, expected, ok in quartets:
        print(f"  {'OK  ' if ok else 'FAIL'} {name} python={verdict} committed={expected}")
    twin = run_twin()
    print(f"twin cases={len(twin)} agree={sum(1 for row in twin if row[3])} inverse-restores={sum(1 for row in twin if row[4])}/{sum(1 for row in twin if row[4] is not None)}")
    for name, verdict, twin_codes, agree, inverse_ok in twin:
        print(f"  {'OK  ' if agree and inverse_ok is not False else 'FAIL'} {name} python={verdict} ts={twin_codes} inverse={inverse_ok}")
    failed = feature["failed"] or drift or not all(row[3] for row in quartets) or not all(row[3] and row[4] is not False for row in twin)
    sys.exit(1 if failed else 0)
