"""Generates the `oracle-comparison` case directory (feature, Python oracle adapter, Rust subject adapter) of each of the ten Eurocode artifacts and extends the oracle manifest (host packages with the `oracle-wiring` feature, third-party capability). Idempotent. Run with python3 -I oracle_comparison_gen.py."""
import json, pathlib, re
ARTS = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
COMPLIANCE = {"implementation": "rust", "package": "semio-s-plugin-norm-compliance", "path": "✏️s/🔌️plugins/📕️norm/⚖️compliance/📦️packages/🦀️rust", "rationale": "🎯️ The oracle comparison drives the blessed case files through the shared oracle driver (`oracle_driver::evaluate_case_text`), compiled by the `compliance-testing` feature.", "features": ["compliance-testing"]}
FEATURE = '''@capability-{code}-1-third-party-cases
@oracle-{oracle}
@comparison-method-spread-v1
Feature: {title} blessed third-party oracle cases against the production evaluators
  Every committed oracle case file of the artifact (`🧪️tests/🧫️fixtures/🔮️oracle-cases/*.json`, schema `🔮️oracles/🏃️harness/🧬️schema/🔣️.json`) holds SI inputs and quantities blessed by an independent third-party library. The Rust subject feeds each case to the production evaluator through the adapter registry of `oracle_wiring` and projects `<case>.<quantity>`; the Python oracle role projects the blessed values of the same cases. Quantities whose check has no evaluator entry point yet, and the documented known differences of the registry, are left out on both sides and stay reported by the `oracle_wiring` unit test. Per-quantity profiles and tolerances of the case file are asserted there; this feature compares under the widest profile.

  @id-oracle-comparison
  @level-quick
  @mode-differential
  Scenario Outline: Compare the quantities of <fixture>
    Given the committed oracle input shared://🔮️oracle-cases/<fixture>
    When the Python oracle and the kernel both evaluate it
    Then every quantity named by the oracle agrees within the comparison profile
    Examples:
      | id | fixture |
{rows}
'''
PY = '''"""🐍️ {title} oracle comparison, oracle role: the blessed quantities of a committed oracle case file, `<case>.<quantity>`, without the checks the production registry has no evaluator for and the documented known differences (read from the `oracle_wiring` source beside the subject, so the two sides cannot drift apart)."""

import json
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
WIRING = HERE.parents[1] / "{wiring}"


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def _registry():
    text = WIRING.read_text(encoding="utf8")
    checks = set(re.findall(r'Adapter \\{{\\s*check: "([^"]+)"', text))
    known = set(re.findall(r'KnownDifference \\{{\\s*case: "([^"]+)",\\s*quantity: "([^"]+)"', text))
    return checks, known


def blessed(path):
    checks, known = _registry()
    with open(path, encoding="utf8") as handle:
        document = json.load(handle)
    return {{
        "%s.%s" % (case["id"], name): float(entry["value"])
        for case in document["cases"]
        if case["check"] in checks
        for name, entry in case["expected"].items()
        if (case["id"], name) not in known
    }}


def adapter():
    steps = _layer()
    return steps.norm_adapter(steps.Binding(oracle_comparison=blessed))
'''
RS = '''//! 🦀️ {title} oracle comparison — the Rust SUBJECT adapter. Every case of the committed oracle case file is fed to the production evaluator through the `oracle_wiring` registry (`oracle_driver::evaluate_case_text`); the projection is `<case>.<quantity>` without known differences and gaps. The independent role is the Python adapter beside this file, which projects the blessed values.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {{
    use super::steps::Binding;
    use semio_s_plugin_norm_compliance::oracle_driver;

    pub fn binding() -> Binding {{
        let mut bound = Binding::none();
        bound.comparison = Box::new(|context, fixture| {{
            let text = String::from_utf8(context.input_bytes(fixture)?).map_err(|error| format!("{{fixture}}: {{error}}"))?;
            let registry = {crate_}::oracle_wiring::registry();
            let evaluated = oracle_driver::evaluate_case_text(&text, fixture, registry);
            Ok(oracle_driver::comparison_values(&evaluated, registry))
        }});
        bound
    }}
}}

/// 🧭️ Registration is by scenario family; the oracle role is the Python adapter beside this file.
pub fn adapter() -> Adapter {{
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {{
        built = steps::norm_adapter(subject::binding());
    }}
    built
}}
'''
for d in sorted(ARTS.iterdir()):
    m = re.search(r"en19\d\d", d.name)
    if not m or m[0] in ("en1900",): continue
    code = m[0]
    A = d / "🏅️standards/🔖️1/🪆️subsets/✳️any"
    wiring = next(p for p in A.rglob("🦀️.rs") if p.parent.name == "🎯️oracle-wiring")
    module = wiring.parent.parent.parent
    cargo = (module / "📦️packages/🦀️rust/Cargo.toml").read_text(encoding="utf8")
    package = re.search(r'^name = "([^"]+)"', cargo, re.M)[1]
    files = sorted(p.name for p in (A / "🧪️tests/🧫️fixtures/🔮️oracle-cases").glob("*.json"))
    manifest_path = A / "🔮️oracles/🔣️.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf8"))
    cands = [o["id"] for o in manifest["oracles"] if f"{code}-1-third-party-cases" in o["capabilities"]]
    oracle = (f"{code}-1-scipy" if f"{code}-1-scipy" in cands else cands[0])
    rel_module = str(module.relative_to(ARTS.parent.parent.parent.parent))
    hosts = manifest["subjectHostPackages"]
    path_rel = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + str((module / "📦️packages/🦀️rust").relative_to(ARTS))
    mine = next((h for h in hosts if h["package"] == package), None)
    if mine is None:
        hosts.append({"implementation": "rust", "package": package, "path": path_rel, "rationale": "🎯️ The oracle comparison reads the adapter registry of `oracle_wiring` (feature `oracle-wiring`) to evaluate the blessed case files.", "features": ["oracle-wiring"]})
    else:
        mine["features"] = sorted(set(mine.get("features", [])) | {"oracle-wiring"})
    comp = next((h for h in hosts if h["package"] == COMPLIANCE["package"]), None)
    if comp is None:
        hosts.append(dict(COMPLIANCE))
    else:
        comp["features"] = sorted(set(comp.get("features", [])) | {"compliance-testing"})
    manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    case_dir = A / f"🧪️tests/🎯️oracle-comparison-{code}-1"
    case_dir.mkdir(exist_ok=True)
    rows = "\n".join(f"      | {re.sub(r'[^a-z0-9]+', '-', f.removesuffix('.json'))} | {f} |" for f in files)
    title = code.upper().replace("EN", "EN ")
    (case_dir / "🥒️.feature").write_text(FEATURE.format(code=code, oracle=oracle, title=title, rows=rows), encoding="utf8")
    (case_dir / "🐍️.py").write_text(PY.format(title=title, wiring=str(wiring.relative_to(A))), encoding="utf8")
    (case_dir / "🦀️.rs").write_text(RS.format(title=title, crate_=package.replace("-", "_")), encoding="utf8")
    print(code, package, oracle, len(files), "case files")
