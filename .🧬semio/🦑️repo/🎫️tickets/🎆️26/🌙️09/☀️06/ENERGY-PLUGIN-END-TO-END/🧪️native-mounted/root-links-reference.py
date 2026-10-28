"""🧪️ The real independent Python reference consumes the shared full-link corpus and restores original pins."""
import importlib.util
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True
workspace = Path(__file__).resolve().parents[8]
base = workspace / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
adapter = base / "🧪️tests/🏛️mutate-energy-model-1"


def load(alias, source):
    """📖️ Loads the genuine authored portable implementation without copying or substituting its source."""
    spec = importlib.util.spec_from_file_location(alias, source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


load("mutate_energy_model_1_mutation_vocabulary", adapter / "🧱️mutation-vocabulary/🐍️.py")
reference = load("mutate_energy_model_1_building", adapter / "🏠️building/🐍️.py")
law = json.loads((base / "🧬️schema/🧬️mutations/🧳️native/🔗️root/🧪️tests/🔣️.json").read_text())
for case in law["cases"]:
    weather = case["slot"] == "weather_link"
    field = "weatherLink" if weather else "referencedModel"
    kind = ("bind-weather-file" if weather else "connect-referenced-model") if case["value"] is not None else ("unbind-weather-file" if weather else "disconnect-referenced-model")
    before = {"weatherLink": None, "referencedModel": None}
    before[field] = case["before"]
    payload = {"link": case["value"]} if case["value"] is not None else {}
    after, outcome = reference.VOCABULARY[kind](before, payload)
    assert after[field] == case["after"], case["id"]
    expected = "applied" if case["outcome"] == "changed" else "no-op" if case["outcome"] == "no-op" else "rejected"
    assert outcome["status"] == expected, (case["id"], outcome)
    if expected == "applied":
        for inverse_kind, inverse in reference.EXTRA_INVERT[kind](before, payload):
            after, restored = reference.VOCABULARY[inverse_kind](after, inverse)
            assert restored["status"] == "applied", (case["id"], restored)
        assert after == before, case["id"]
print(f"PASS: {len(law['cases'])} real Python full-link reference and exact inverse cases")
