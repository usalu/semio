"""🧪️ S3-GRAPHS: runs a `mutate-<artifact>-1` Python second implementation standalone against every committed vector.

A stub `semio_repo_test` (Adapter/Context/Outcome) resolves `shared://🧬️mutations/…` to the dag subset's own
`🧫️fixtures/🧬️mutations/…`, so every registered `mutate-<kind>` / `inverse-<kind>` oracle handler runs exactly as the
repository host would call it. It also checks that the handler set equals the feature's Examples ids and the oracle
catalog's kinds. Usage from the repository root: `python3 🧪️s3-graphs-dag-oracle-vectors.py [<subset dir> <case dir name>]`
(default: the dag subset and `🌳️mutate-dag-1`; wires: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any
📡️mutate-wires-1`).
"""

import importlib.util
import json
import re
import sys
import types
from pathlib import Path

ROOT = Path.cwd()
SUBSET = ROOT / (sys.argv[1] if len(sys.argv) > 2 else "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any")
CASE = SUBSET / "🧪️tests" / (sys.argv[2] if len(sys.argv) > 2 else "🌳️mutate-dag-1")


class Outcome:
    """📦️ Mirrors the host's Outcome."""

    def __init__(self, projection=None, raw=b""):
        self.projection, self.raw = projection, raw


class Context:
    """🧫️ Resolves `shared://` fixture URIs onto the subset's fixture root."""

    def fixture_bytes(self, uri: str) -> bytes:
        assert uri.startswith("shared://🧬️mutations/"), uri
        return (SUBSET / "🧫️fixtures" / uri[len("shared://"):]).read_bytes()


class Adapter:
    """🧭️ Collects oracle handlers by scenario id."""

    def __init__(self, implementation: str):
        self.implementation, self.handlers = implementation, {}

    def oracle(self, scenario: str, handler):
        self.handlers[scenario] = handler
        return self


stub = types.ModuleType("semio_repo_test")
stub.Adapter, stub.Context, stub.Outcome = Adapter, Context, Outcome
sys.modules["semio_repo_test"] = stub
spec = importlib.util.spec_from_file_location("dag_oracle", CASE / "🐍️.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

adapter = module.adapter()
feature = (CASE / "🥒️.feature").read_text(encoding="utf-8")
rows = [(kind, directory, fixture, next(cell for cell in rest if cell.startswith("mutation."))) for kind, directory, fixture, *rest in (tuple(cell.strip() for cell in line.strip().strip("|").split("|")) for line in feature.splitlines() if re.match(r"^\s*\| [a-z]+-[a-z-]+\s*\|", line))]
expected_ids = {f"{role}-{kind}" for kind, *_ in rows for role in ("mutate", "inverse")}
catalog = json.loads((SUBSET / "🔮️oracles/🔣️.json").read_text(encoding="utf-8"))["mutationCatalogs"][0]
failures = []
if set(adapter.handlers) != expected_ids:
    failures.append(f"handler ids != feature ids: {sorted(set(adapter.handlers) ^ expected_ids)}")
if {kind for kind, *_ in rows} != set(catalog["kinds"]) or catalog.get("deferredKinds"):
    failures.append("feature kinds != catalog kinds, or the catalog still defers kinds")
for kind, directory, fixture, code in rows:
    if not (SUBSET / "🧫️fixtures/🧬️mutations" / directory / fixture).is_dir():
        failures.append(f"{kind}: the feature names a missing fixture {directory}/{fixture}")
    declared = json.loads((SUBSET / "🧫️fixtures/🧬️mutations" / directory / fixture / "🎯️outcome/🔣️.json").read_text(encoding="utf-8"))
    declared_code = declared.get("code") or declared["messages"][0]["code"]
    if declared_code != code:
        failures.append(f"{kind}: the feature row says {code} but the outcome declares {declared_code}")
passed = 0
for scenario, handler in sorted(adapter.handlers.items()):
    try:
        handler(Context())
        passed += 1
    except Exception as error:
        failures.append(f"{scenario}: {error}")
print(f"[s3-graphs] {CASE.name} python second implementation: {passed}/{len(adapter.handlers)} scenarios passed, {len(rows) // 2} kinds")
for failure in failures:
    print(f"FAIL {failure}")
sys.exit(1 if failures else 0)
