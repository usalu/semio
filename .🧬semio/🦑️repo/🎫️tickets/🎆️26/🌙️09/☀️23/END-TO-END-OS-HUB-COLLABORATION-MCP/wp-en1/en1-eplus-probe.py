"""⚡️ EN1 probe: run the energy epJSON case's OWN Python oracle functions on given epJSON files.

Usage: en1-eplus-probe.py <repo-root> <work-root> <case>=<epjson-path> [...]

Loads `🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py` from <repo-root> with a stub `semio_repo_test`
module (only its `_validate`, `_run`, `_results`, `_zone_information`, `document_facts` are used), validates
each document against Energy+.schema.epJSON, runs EnergyPlus 25.2.0 on the exact bytes and compares with the
committed `🔮️energyplus.json` exactly the way the oracle's `_energyplus_run` handler does.
"""
import importlib.util
import json
import sys
import types
from pathlib import Path

repo_root = Path(sys.argv[1]).resolve()
work_root = Path(sys.argv[2]).resolve()
subset = repo_root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"

stub = types.ModuleType("semio_repo_test")
stub.Adapter = object
stub.Context = object
stub.Outcome = object
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("en1_oracle", subset / "🧪️tests/🏛️export-epjson-runs-in-energyplus/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


class Ctx:
    def __init__(self, case):
        self.repo_root = str(repo_root)
        self.work_dir = str(work_root / case)


for argument in sys.argv[3:]:
    case, path = argument.split("=", 1)
    raw = Path(path).read_bytes()
    document = json.loads(raw.decode("utf-8"))
    ctx = Ctx(case)
    violations = oracle._validate(ctx, document)
    free_float = not document.get("ZoneHVAC:IdealLoadsAirSystem")
    reference = json.loads((subset / ("🧫️fixtures/🏛️bestest-%s/🔮️energyplus.json" % case)).read_text(encoding="utf-8"))
    print("== case %s: %d schema violations, idealLoads=%d, freeFloat=%s, constructions=%s" % (case, len(violations), len(document.get("ZoneHVAC:IdealLoadsAirSystem", {})), free_float, sorted(document.get("Construction", {}).keys())))
    for violation in violations[:10]:
        print("   violation:", violation)
    work = oracle._run(ctx, case, raw, str(subset / "🧫️fixtures/🌦️denver-tmy/🌦️.epw"))
    measured = oracle._results(case, work, free_float)
    print("   zone:", oracle._zone_information(work))
    if free_float:
        for metric in ("minC", "maxC", "meanC"):
            gap = measured["freeFloat"][metric] - reference["freeFloat"][metric]
            print("   %s direct %.4f ref %.4f gap %+.4f K %s" % (metric, measured["freeFloat"][metric], reference["freeFloat"][metric], gap, "OK" if abs(gap) <= oracle.FREE_FLOAT_TOLERANCE_K else "RED"))
    else:
        for metric in ("heatingKwh", "coolingKwh"):
            expected = reference["annual"][metric]
            gap = (measured["annual"][metric] - expected) / expected
            print("   %s direct %.4f ref %.4f dev %+.4f %s" % (metric, measured["annual"][metric], expected, gap, "OK" if abs(gap) <= oracle.ANNUAL_TOLERANCE else "RED"))
    sys.stdout.flush()
