"""🏃️ Shared Eurocode oracle harness: validates case files against the JSON Schema, runs the per-library adapters and compares under the five tolerance profiles.

Usage, always with the repo-resolved interpreter and `-I`:
  python -I 🐍️.py --json < cases.json     stdin protocol, prints {"checks": [...]}
  python -I 🐍️.py --cases a.json b.json   verify blessed expectations, exit 1 on any failure
  python -I 🐍️.py --cases a.json --bless  rewrite expectations from the first reference after a spread check
  python -I 🐍️.py --list                  print registered adapters and their checks
  python -I 🐍️.py --dry-run a.json b.json validate the inputs of every reference that has a dry run, without importing a library or solving

Adapters live in `🔌️adapters/<name>/🐍️.py` and export `CHECKS: dict[str, Callable[[dict], dict[str, float]]]`.
Missing libraries fail loudly; nothing is skipped.
"""

import importlib.util
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(path, name):
    """📦️ Load a module from a file path whose name is not a Python identifier."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


TOLERANCE = load(HERE / "📏️tolerance" / "🐍️.py", "norm_oracle_tolerance")


def adapter(name):
    """🔌️ Import the adapter `name`; an uninstalled library raises with the adapter and library named."""
    path = HERE / "🔌️adapters" / name / "🐍️.py"
    if not path.is_file():
        raise LookupError(f"no adapter {name!r} at {path}")
    return load(path, f"norm_oracle_adapter_{name.replace('-', '_')}")


def adapter_names():
    """📋️ Names of all adapter directories."""
    return sorted(p.parent.name for p in (HERE / "🔌️adapters").glob("*/🐍️.py"))


def validate(document):
    """🧬️ Validate a case file against the schema; raises `jsonschema.ValidationError`."""
    import jsonschema

    schema = json.loads((HERE / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8"))
    jsonschema.Draft202012Validator(schema).validate(document)


def compute(case):
    """🧮️ Run every reference of a case; returns `{reference oracle: {quantity: value}}`."""
    out = {}
    for ref in case["references"]:
        module = adapter(ref["adapter"])
        check = ref.get("check", case["check"])
        if check not in module.CHECKS:
            raise LookupError(f"adapter {ref['adapter']!r} has no check {check!r}")
        values = {k: float(v) for k, v in module.CHECKS[check](case["inputs"]).items()}
        invalid = sorted(k for k, v in values.items() if not math.isfinite(v))
        if invalid:
            raise ArithmeticError(f"{case['id']}: {ref['oracle']} returned non-finite {invalid}")
        out[ref["oracle"]] = values
    return out


def judge(case, computed):
    """⚖️ Compare computed quantities with the blessed expectations and with each other; returns failure strings."""
    failures = []
    for name, want in case.get("expected", {}).items():
        values = {oracle: q[name] for oracle, q in computed.items() if name in q}
        if not values:
            failures.append(f"{case['id']}: no reference computes {name!r}")
            continue
        override = want.get("tolerance")
        for oracle, value in values.items():
            if not TOLERANCE.within(want["profile"], want["value"], value, override):
                failures.append(f"{case['id']}: {name} expected {want['value']!r} {oracle} {value!r} rel {TOLERANCE.relative_error(want['value'], value):.3e} profile {want['profile']}")
        pool = list(values.values())
        if not TOLERANCE.within(want["profile"], min(pool), max(pool), override):
            failures.append(f"{case['id']}: oracle disagreement on {name}: {values}")
    return failures


def dry_run(document):
    """🧭️ Plan every reference of every case without solving: an adapter directory may hold `🧭️dry-run/🐍️.py` exporting `DRY_RUN: {check: plan(inputs) -> dict}` that validates the inputs in pure Python. Returns `{"id", "oracle", "status", "plan"}` rows with status `planned`, `none` (no dry run for the check) or `invalid` (the plan raised, `plan` holds the message)."""
    rows = []
    for case in document["cases"]:
        for ref in case["references"]:
            path = HERE / "🔌️adapters" / ref["adapter"] / "🧭️dry-run" / "🐍️.py"
            check = ref.get("check", case["check"])
            row = {"id": case["id"], "oracle": ref["oracle"], "status": "none", "plan": None}
            if path.is_file():
                plans = load(path, f"norm_oracle_dry_run_{ref['adapter'].replace('-', '_')}").DRY_RUN
                if check in plans:
                    try:
                        row.update(status="planned", plan=plans[check](case["inputs"]))
                    except ValueError as error:
                        row.update(status="invalid", plan=str(error))
            rows.append(row)
    return rows


def run(document, bless=False):
    """🚦️ Evaluate a case file; with `bless` the first reference becomes the expected value unless references disagree."""
    results = []
    for case in document["cases"]:
        computed = compute(case)
        if bless:
            for name, want in case["expected"].items():
                first = next((q for q in computed.values() if name in q), None)
                if first is None:
                    raise LookupError(f"{case['id']}: no reference computes {name!r}")
                want["value"] = first[name]
        failures = judge(case, computed)
        results.append({"id": case["id"], "quantities": computed, "failures": failures, "utilization": next((q["utilization"] for q in computed.values() if "utilization" in q), None)})
    return results


def main(argv):
    """🚪️ Command line entry; returns the process exit code."""
    if "--list" in argv:
        for name in adapter_names():
            try:
                print(name, sorted(adapter(name).CHECKS))
            except Exception as error:
                print(name, "UNAVAILABLE", type(error).__name__, error)
                return 1
        return 0
    if "--dry-run" in argv:
        files = [a for a in argv[argv.index("--dry-run") + 1:] if not a.startswith("--")]
        if not files:
            print("--dry-run needs at least one file", file=sys.stderr)
            return 2
        bad = 0
        for file in map(Path, files):
            document = json.loads(file.read_text(encoding="utf-8"))
            validate(document)
            for row in dry_run(document):
                bad += row["status"] == "invalid"
                print("dry", row["status"], row["id"], row["oracle"], json.dumps(row["plan"], ensure_ascii=False))
        return 1 if bad else 0
    if "--json" in argv:
        document = json.load(sys.stdin)
        results = run(document)
        json.dump({"checks": results}, sys.stdout, ensure_ascii=False)
        return 1 if any(r["failures"] for r in results) else 0
    if "--cases" in argv:
        files = [a for a in argv[argv.index("--cases") + 1:] if not a.startswith("--")]
        if not files:
            print("--cases needs at least one file", file=sys.stderr)
            return 2
        bless = "--bless" in argv
        bad = 0
        for file in map(Path, files):
            document = json.loads(file.read_text(encoding="utf-8"))
            validate(document)
            results = run(document, bless)
            if bless:
                validate(document)
                file.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
            for r in results:
                bad += len(r["failures"])
                print("FAIL" if r["failures"] else "ok", r["id"], *r["failures"])
        return 1 if bad else 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
