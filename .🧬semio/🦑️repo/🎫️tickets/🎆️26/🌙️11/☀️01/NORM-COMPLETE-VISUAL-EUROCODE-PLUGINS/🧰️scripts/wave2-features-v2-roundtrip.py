"""Wave 2 features v2: writes the `mutation-round-trip` feature and its two adapters for the ten Eurocode artifacts from their committed `mutate-<id>-1` cases (rows are picked from the committed `@id-inverse` table).
Usage: .venv/bin/python -I wave2-features-v2-roundtrip.py"""
import re
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
IDS = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"]

FEATURE = """@capability-{n}-1-round-trip
@oracle-{n}-1-python-independent
@comparison-ordered-json-v1
Feature: EN {num} mutations are undone by their own computed inverse
  Representative committed vectors of the typed {n} mutations (one change, one insert, one remove) are applied to their committed before-snapshot and then undone by the mutation's own computed inverse. Both implementations must restore the before-snapshot exactly; the exhaustive form of the same law over every kind is the `inverse-<kind>` scenarios of `{mutate}`.

  @id-mutation-round-trip
  @level-quick
  @mode-differential
  Scenario Outline: Undoing <id> restores the before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    When the mutation is applied and then its computed inverse
    Then the document equals the before-snapshot
    Examples:
      | id | dir | fixture |
{rows}
"""

PY = '''"""🐍️ {NUM} mutation round trip, oracle role: the shared mutation vocabulary engine (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution`) applies each committed mutation and its own computed inverse independently of the Rust producer, through the shared norm step layer."""

import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent


def _layer():
    import importlib.util

    path = next(parent / "🔮️oracles" / "🥒️steps" / "🐍️.py" for parent in HERE.parents if (parent / "🔮️oracles" / "🥒️steps" / "🐍️.py").is_file())
    spec = importlib.util.spec_from_file_location("norm_steps", path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["norm_steps"] = module
    spec.loader.exec_module(module)
    return module


def adapter():
    steps = _layer()
    mutate = steps.load_module("{n}_mutate_adapter", str(HERE.parent / "{mutate}" / "🐍️.py"))

    return steps.norm_adapter(steps.Binding(oracle_round_trip=steps.vocabulary_round_trip(mutate.KINDS)))
'''

RS = '''//! 🦀️ {NUM} mutation round trip — the Rust SUBJECT adapter. The committed mutation is applied by the production dispatch and undone by its own computed inverse; the before-snapshot must be restored exactly. The independent Python mutation vocabulary answers the same rows in the oracle role. Step plumbing is the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {{
    use super::steps::Binding;
    use semio_s_artifact_norm_{n}_io::text::mutations::decode_{n}_mutation_json;
    use semio_s_artifact_norm_{n}_io::text::snapshot::decode_{n}_snapshot_json;
    use semio_s_artifact_norm_{n}_mutations::mutation_bridge::{{apply_{n}_mutation, inverse_{n}_mutation}};

    pub fn binding() -> Binding {{
        let mut bound = Binding::none();
        bound.round_trip = Box::new(|context, before, mutation| {{
            let read = |uri: &str| -> Result<String, String> {{ String::from_utf8(context.input_bytes(uri)?).map_err(|error| format!("{{uri}}: {{error}}")) }};
            let base = decode_{n}_snapshot_json(&read(before)?).map_err(|error| format!("{{before}}: {{error}}"))?;
            let mutation = decode_{n}_mutation_json(&read(mutation)?).map_err(|error| format!("{{mutation}}: {{error}}"))?;
            let (mut current, _) = apply_{n}_mutation(&base, &mutation).map_err(|error| format!("forward mutation: {{error}}"))?;
            let moved = current != base;
            let steps = inverse_{n}_mutation(&mutation, &base).map_err(|error| format!("computed inverse: {{error}}"))?;
            for step in &steps {{
                current = apply_{n}_mutation(&current, step).map_err(|error| format!("inverse step: {{error}}"))?.0;
            }}
            Ok(moved && !steps.is_empty() && current == base)
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

for n in IDS:
    tests = ROOT.glob("*%s/%s/🧪️tests" % (n, ANY))
    tests = next(tests)
    mutate = next(p for p in tests.iterdir() if p.name.endswith("mutate-%s-1" % n))
    feature = (mutate / "🥒️.feature").read_text(encoding="utf8")
    block = feature[feature.index("@id-inverse"):]
    table = [[c.strip() for c in line.strip().strip("|").split("|")] for line in block.splitlines() if line.strip().startswith("|") and "✅apply" in line]
    pick = []
    for verb in ("change-", "insert-", "remove-"):
        hit = next((r for r in table if r[0].startswith(verb)), None)
        if hit:
            pick.append(hit)
    rows = "\n".join("      | %s | %s | %s |" % tuple(r[:3]) for r in pick)
    out = tests / ("↩️round-trip-%s-1" % n)
    out.mkdir(exist_ok=True)
    (out / "🥒️.feature").write_text(FEATURE.format(n=n, num=n[2:], mutate=mutate.name, rows=rows), encoding="utf8")
    (out / "🐍️.py").write_text(PY.format(n=n, NUM=n.upper().replace("EN", "EN "), mutate=mutate.name), encoding="utf8")
    (out / "🦀️.rs").write_text(RS.format(n=n, NUM=n.upper().replace("EN", "EN ")), encoding="utf8")
    print(n, len(pick), out.name)
