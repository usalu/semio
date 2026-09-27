"""⚡️ EN2 probe: judge epJSON documents exactly the way the energy epJSON oracle's `_energyplus_run` does.

Usage: en2-eplus-probe.py <repo-root> <work-root> <case>=<epjson-path> [...]

Loads `🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py` from <repo-root> with a stub `semio_repo_test`
module, validates each document against Energy+.schema.epJSON, runs EnergyPlus 25.2.0 on the exact bytes
(all documents in parallel) and compares with the committed `🔮️energyplus.json` using the oracle's own
tolerances. Exit 0 only when every document is schema-clean and within tolerance.
"""
import importlib.util
import json
import sys
import types
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

repo_root = Path(sys.argv[1]).resolve()
work_root = Path(sys.argv[2]).resolve()
subset = repo_root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"

stub = types.ModuleType("semio_repo_test")
stub.Adapter = object
stub.Context = object
stub.Outcome = object
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("en2_oracle", subset / "🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


class Ctx:
    def __init__(self, label):
        self.repo_root = str(repo_root)
        self.work_dir = str(work_root / label)


def judge(argument):
    label, path = argument.split("=", 1)
    case = label.split(":", 1)[0]
    raw = Path(path).read_bytes()
    document = json.loads(raw.decode("utf-8"))
    ctx = Ctx(label.replace(":", "-"))
    lines = []
    violations = oracle._validate(ctx, document)
    free_float = not document.get("ZoneHVAC:IdealLoadsAirSystem")
    reference = json.loads((subset / ("🧫️fixtures/🏛️bestest-%s/🔮️energyplus.json" % case)).read_text(encoding="utf-8"))
    lines.append("== %s (%s): %d schema violations, idealLoads=%d, freeFloat=%s" % (label, path, len(violations), len(document.get("ZoneHVAC:IdealLoadsAirSystem", {})), free_float))
    for violation in violations[:10]:
        lines.append("   violation: %s" % violation)
    work = oracle._run(ctx, case, raw, str(subset / "🧫️fixtures/🌦️denver-tmy/🌦️.epw"))
    measured = oracle._results(case, work, free_float)
    lines.append("   zone: %s" % oracle._zone_information(work))
    ok = not violations
    if free_float:
        for metric in ("minC", "maxC", "meanC"):
            gap = measured["freeFloat"][metric] - reference["freeFloat"][metric]
            good = abs(gap) <= oracle.FREE_FLOAT_TOLERANCE_K
            ok = ok and good
            lines.append("   %s direct %.4f ref %.4f gap %+.4f K %s" % (metric, measured["freeFloat"][metric], reference["freeFloat"][metric], gap, "OK" if good else "RED"))
    else:
        for metric in ("heatingKwh", "coolingKwh"):
            expected = reference["annual"][metric]
            gap = (measured["annual"][metric] - expected) / expected
            good = abs(gap) <= oracle.ANNUAL_TOLERANCE
            ok = ok and good
            lines.append("   %s direct %.4f ref %.4f dev %+.4f %s" % (metric, measured["annual"][metric], expected, gap, "OK" if good else "RED"))
    return ok, "\n".join(lines)


with ThreadPoolExecutor(max_workers=4) as pool:
    results = list(pool.map(judge, sys.argv[3:]))
for _, text in results:
    print(text)
passed = sum(1 for ok, _ in results if ok)
print("PASSED %d/%d" % (passed, len(results)))
sys.exit(0 if passed == len(results) else 1)
