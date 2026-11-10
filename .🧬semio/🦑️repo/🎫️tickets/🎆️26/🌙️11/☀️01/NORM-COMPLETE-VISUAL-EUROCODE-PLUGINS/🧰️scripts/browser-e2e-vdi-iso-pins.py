import importlib.util, json, re, sys
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
VDI = ROOT / "🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any"
ISO = ROOT / "🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any"
MATRIX = ROOT / "🧪️tests/⏪️time-travel/🧫️witness/🔣️.json"

spec = importlib.util.spec_from_file_location("vdi_oracle", VDI / "🔮️oracles/⚖️compliance/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)

dsl = (VDI / "🖼️assets/blatt-3/🗣️.dsl.semio").read_text()
n_before = float(re.search(r"heatExponentN=([0-9.]+)", dsl).group(1))
n_after = 1.4
n_max = oracle.RADIATOR_N_MAX


def vdi_doc(n):
    record = {"family": "210", "fields": ["210", "standard_output_w", "1200", "n", str(n), "length_m", "0.6", "height_m", "0.6", "depth_m", "0.1", "connection_type", "flange"]}
    product = {"id": "RAD-600", "sheet": 3, "records": [record], "configuration": {"attributes": {"kind": "radiator", "standardOutputW": 1200.0, "heatExponentN": n}}}
    return {"catalog": {"file": {"manufacturer": "DEMO", "charset": "UTF-8", "recordCount": 1}, "products": [product]}}


def n_check(n):
    return next(c for c in oracle.oracle_checks(vdi_doc(n)) if c["id"] == "n:RAD-600")


assert n_check(n_before)["pass"] and n_check(n_after)["pass"]
vdi_states = {"before": {"value": n_before, "utilization": round(n_before / n_max, 4)}, "after": {"value": n_after, "utilization": round(n_after / n_max, 4)}}

iso = (ISO / "🖼️assets/🎬️demo/🗣️.dsl.semio").read_text()
limits = dict(zip(("steps", "recursion", "timeout"), map(int, re.search(r"script-limits=max-steps=(\d+) max-recursion=(\d+) timeout-ms=(\d+)", iso).groups())))


def capacity(steps, recursion=limits["recursion"], timeout=limits["timeout"]):
    return steps / (max(recursion, 1) + timeout / 1000.0)


steps_after = 5000
iso_states = {"before": {"value": limits["steps"], "utilization": round(capacity(limits["steps"]), 4)}, "after": {"value": steps_after, "utilization": round(capacity(steps_after), 4)}}
print(json.dumps({"vdi3805": vdi_states, "iso16757": iso_states, "limits": limits}, indent=1))

matrix = json.loads(MATRIX.read_text())
pins = {
    "vdi3805": {
        "field": {"path": "catalog.products[id=RAD-600].configuration.attributes.heatExponentN", "key": "norm-inputs.catalog.products[id=RAD-600].configuration.attributes.heatExponentN", "owner": "scope:1:16:catalog.products7:RAD-600", "ownerWindow": "norm-vdi3805-outliner", "argument": "newConfiguration", "label": {"en": "Heat exponent n", "de": "Heizexponent n"}, "limit": n_max, "unit": "1"},
        "check": "vdi3805.3.n.RAD-600",
        "states": vdi_states,
        "basis": "Blatt 3 radiator exponent check (checks/valve-radiator-and-pump-checks check_radiator): utilization = n / RADIATOR_N_MAX with the EN 442 band [1.1, 1.5]; blatt-3 RAD-600 starts at n = 1.3 (attributes and 210 record), 1.3/1.5 = 0.8667 to 1.4/1.5 = 0.9333. The edit is change-product-configuration, whose diff rewrites the mirrored 210 record n in the same patch, so the attribute/record sync check stays pass; only this check reads the exponent. Pass/fail of both states confirmed with the artifact Python oracle (n:RAD-600).",
        "linear": True,
    },
    "iso16757": {
        "field": {"path": "scriptLimits.maxSteps", "key": "norm-inputs.scriptLimits.maxSteps", "owner": "", "ownerWindow": "norm-iso16757-outliner", "argument": "newMaxSteps", "label": {"en": "Max steps", "de": "Max. Schritte"}, "limit": round(max(limits["recursion"], 1) + limits["timeout"] / 1000.0, 2), "unit": "1"},
        "check": "iso16757.5.8.scriptLimits.capacity",
        "states": iso_states,
        "basis": "Part 5 section 8 script capacity (checks/part5 check_script_limits): utilization = maxSteps / (max(maxRecursion, 1) + timeoutMs / 1000) = steps / 64.05 for the demo limits (10000 steps, 64 recursion, 50 ms), linear in maxSteps, status forced pass. scriptLimits is a singleton facet with no outliner row, so owner is empty and nothing is selected; change-script-limits carries all three limits and the part-number script dn*10+50 still runs, so only the capacity check changes (the recursion check reads maxRecursion only). Utilization is derived by an independent Python evaluation of the Rust formula; no third-party oracle exists for the capacity ratio.",
        "linear": True,
    },
}
for artifact in matrix["artifacts"]:
    if artifact["variant"] in pins:
        artifact["pin"] = pins[artifact["variant"]]
if "--write" in sys.argv:
    MATRIX.write_text(json.dumps(matrix, ensure_ascii=False, indent=2) + "\n")
    print("matrix written")
