"""Wave 2 features v3 (input script): generates the `check-outcome` case `🧪️tests/🚦️check-outcome-enXXXX-1` of the artifacts that own a document-level independent Python oracle.
The Python side runs that oracle on a committed before-snapshot and maps its answer to check ids; the Rust side decodes the same snapshot, runs the artifact's `evaluate` and looks the check up. Scenarios are picked from the oracle itself.
Usage: .venv/bin/python -I wave2-features-v3-check-outcome.py [en1990 ...]"""
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path

NORM = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"

HEADER = '''"""🐍️ {NAME} check outcome, oracle role: {WHAT} The status and utilisation of one check id of a committed document, through the shared norm step layer."""

import functools
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
ORACLE = str(HERE.parents[1] / "🔮️oracles" / "⚖️compliance" / "🐍️.py")


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module

'''

FOOTER = '''

def adapter():
    steps = _layer()
    outcome = functools.lru_cache(maxsize=None)(lambda path: outcomes(steps, path))

    return steps.norm_adapter(steps.Binding(oracle_check=lambda fixture, check: steps.check_outcome(outcome(fixture), check)))
'''

BODIES = {
    "en1990": ("EN 1990", "the brute-force combination enumeration of `../../🔮️oracles/⚖️compliance/🐍️.py` (scipy FORM and Table D1) divided by the member's own resistance or criterion.", '''
def outcomes(steps, path):
    document = json.load(open(path, encoding="utf8"))
    answers = steps.run_oracle(ORACLE, ["snapshot"], {"d": document})["d"]
    rows = {}
    limits = [("str", "6.10", "rdStr"), ("str", "6.10a", "rdStr"), ("str", "6.10b", "rdStr"), ("geo", "6.10", "rdGeo"), ("sls", "6.14b", "slsLimitChar"), ("sls", "6.15b", "slsLimitFreq"), ("sls", "6.16b", "slsLimitQp")]
    for member in document["members"]:
        for kind, equation, field in limits:
            key = "en1990.%s.%s.%s" % (kind, member["id"], equation)
            if key in answers and member[field] > 0:
                utilisation = answers[key] / member[field]
                rows[key] = (steps.pass_or_fail(utilisation), utilisation)
    target, beta = answers["_limit.en1990.annex-c.beta"], document["betaComputed"]
    if beta > 0:
        rows["en1990.annex-c.beta"] = ("pass" if beta >= target else "fail", target / beta)
    return rows
'''),
    "en1991": ("EN 1991", "the independent required-versus-assumed pressures of `../../🔮️oracles/⚖️compliance/🐍️.py` (imposed floor and self-weight rows).", '''
def outcomes(steps, path):
    document = json.load(open(path, encoding="utf8"))
    rows = {}
    for row in steps.run_oracle(ORACLE, [], document):
        if row["assumed"] > 0 and (".imposed." in row["id"] or ".self-weight." in row["id"]):
            utilisation = row["required"] / row["assumed"]
            rows[row["id"]] = ("pass" if row["ok"] else "fail", utilisation)
    return rows
'''),
    "en1995": ("EN 1995", "the independent timber utilisations of `../../🔮️oracles/⚖️compliance/🐍️.py --json`.", '''
def outcomes(steps, path):
    answers = steps.run_oracle(ORACLE, ["--json"], json.load(open(path, encoding="utf8")))
    return {row["id"]: (steps.pass_or_fail(row["utilization"]), row["utilization"]) for row in answers["checks"]}
'''),
    "en1996": ("EN 1996", "the independent per-wall utilisations of `../../🔮️oracles/⚖️compliance/🐍️.py` (maximum over the load cases, so a check id names the family `<id>.<load case>`).", '''
KEYS = {"compression": "en1996.6.1.2.compression.", "mid": "en1996.6.1.2.compression.mid.", "slenderness": "en1996.5.5.1.slenderness.", "simplified": "en1996.3.simplified."}


def outcomes(steps, path):
    document = json.load(open(path, encoding="utf8"))
    answers = steps.run_oracle(ORACLE, [], {"mode": "snapshot", "snapshot": document, "report": {}})["expected"]
    rows = {}
    for wall, values in answers.items():
        for key, prefix in KEYS.items():
            if key in values:
                rows[prefix + wall] = (steps.pass_or_fail(values[key]), values[key])
    return rows
'''),
    "en1997": ("EN 1997", "the independent geotechnical utilisations of `../../🔮️oracles/⚖️compliance/🐍️.py --json`.", None),
    "en1998": ("EN 1998", "the independent seismic utilisations of `../../🔮️oracles/⚖️compliance/🐍️.py --json`.", None),
    "en1999": ("EN 1999", "the independent aluminium utilisations of `../../🔮️oracles/⚖️compliance/🐍️.py --json`.", None),
}
GENERIC = '''
def outcomes(steps, path):
    answers = steps.run_oracle(ORACLE, ["--json"], json.load(open(path, encoding="utf8")))
    return {row["id"]: (steps.pass_or_fail(row["utilization"]), row["utilization"]) for row in answers["checks"]}
'''
BODIES["en1997"] = BODIES["en1997"][:2] + (GENERIC,)
BODIES["en1998"] = BODIES["en1998"][:2] + (GENERIC,)
BODIES["en1999"] = BODIES["en1999"][:2] + (GENERIC,)

RUST = '''//! 🦀️ {NAME} check outcome — the Rust SUBJECT adapter. The committed before-snapshot is decoded with the io crate, evaluated by the production `evaluate` and the status and utilisation of the named check are compared by the feature's relative tolerance with the independent Python oracle in the oracle role. Step plumbing is the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {{
    use super::steps::{{pick_outcome, Binding}};
    use semio_s_artifact_norm_{LOW}_inferences::evaluate;
    use semio_s_artifact_norm_{LOW}_io::text::snapshot::decode_{LOW}_snapshot_json;
    use semio_s_plugin_norm_compliance::CheckStatus;

    fn status_name(status: &CheckStatus) -> &'static str {{
        match status {{
            CheckStatus::Pass => "pass",
            CheckStatus::Warning => "warning",
            CheckStatus::Fail => "fail",
            CheckStatus::NotApplicable => "notApplicable",
            CheckStatus::UnverifiedBasis => "unverifiedBasis",
            CheckStatus::Info => "info",
        }}
    }}

    pub fn binding() -> Binding {{
        let mut bound = Binding::none();
        bound.check = Box::new(|context, fixture, check| {{
            let text = String::from_utf8(context.input_bytes(fixture)?).map_err(|error| format!("{{fixture}}: {{error}}"))?;
            let document = decode_{LOW}_snapshot_json(&text).map_err(|error| format!("{{fixture}}: {{error}}"))?;
            let rows: Vec<(String, &'static str, f64)> = evaluate(&document).checks.iter().map(|row| (row.id.clone(), status_name(&row.status), row.utilization)).collect();
            pick_outcome(&rows, check)
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

FEATURE = '''@capability-{LOW1}-evaluate
@oracle-{LOW1}-compliance-independent
@comparison-numerical-v1
Feature: {NAME} check outcome of committed documents
  A committed before-snapshot of the mutation fixtures is evaluated by the production report and by {WHAT}
  Each row names one check id, its status (pass at a utilisation of at most 1, fail above 1) and its utilisation; rows were picked away from the near-limit band so the status is unambiguous.

  @id-check-outcome
  @level-quick
  @mode-differential
  Scenario Outline: <check> reports <status> on <fixture>
    Given the committed document <fixture>
    When the check "<check>" is evaluated
    Then its status is "<status>" with utilisation <utilisation> within relative tolerance <tolerance>
    Examples:
{TABLE}
'''


def load(path, alias):
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def layer():
    spec = importlib.util.spec_from_file_location("norm_steps", NORM / "🔮️oracles/🥒️steps/🐍️.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def pick(candidates):
    chosen, families = [], set()
    for want, count in (("pass", 4), ("fail", 3)):
        pool = [row for row in candidates if row[2] == want]
        taken = 0
        for row in pool:
            family = ".".join(row[1].split(".")[:3])
            if family in families or any(row[0] == other[0] and want == other[2] for other in chosen):
                continue
            chosen.append(row)
            families.add(family)
            taken += 1
            if taken == count:
                break
        for row in pool:
            if taken == count:
                break
            if row not in chosen:
                chosen.append(row)
                taken += 1
    return chosen


def main(names):
    steps = layer()
    for name in names or BODIES:
        title, what, body = BODIES[name]
        directory = next(NORM.glob("🗿️artifacts/*%s/%s" % (name, ANY)))
        case = directory / "🧪️tests" / ("🚦️check-outcome-%s-1" % name)
        case.mkdir(parents=True, exist_ok=True)
        (case / "🐍️.py").write_text(HEADER.replace("{NAME}", title).replace("{WHAT}", what) + body + FOOTER, encoding="utf8")
        module = load(case / "🐍️.py", "outcome_" + name)
        roots = directory / "🧪️tests" / "🧫️fixtures"
        seen, candidates = set(), []
        for path in sorted(roots.glob("🧬️mutations/*/*/📸️snapshot/*/🔣️.json")):
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            if digest in seen:
                continue
            seen.add(digest)
            try:
                rows = module.outcomes(steps, str(path))
            except Exception as error:
                print(name, "skip", path.parts[-5:-3], str(error)[:80])
                continue
            uri = "shared://" + str(path.relative_to(roots))
            for key, (status, utilisation) in rows.items():
                if status == "pass" and 0.01 < utilisation < 0.9 or status == "fail" and utilisation > 1.02:
                    candidates.append((uri, key, status, utilisation))
        chosen = pick(candidates)
        rows = ["      | id | fixture | check | status | utilisation | tolerance |"]
        for index, (uri, key, status, utilisation) in enumerate(chosen):
            rows.append("      | %s-%02d | %s | %s | %s | %s | 1e-6 |" % (status, index + 1, uri, key, status, "%.9g" % utilisation))
        low = "en" + name[2:]
        (case / "🥒️.feature").write_text(FEATURE.replace("{NAME}", title).replace("{WHAT}", what).replace("{LOW1}", low + "-1").replace("{TABLE}", "\n".join(rows)), encoding="utf8")
        (case / "🦀️.rs").write_text(RUST.format(NAME=title, LOW=low), encoding="utf8")
        print(name, len(seen), "documents", len(candidates), "candidates", len(chosen), "scenarios")


main(sys.argv[1:])
