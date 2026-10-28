"""🖱️ Sample the schema-owned single-topic review cursor without browser dependencies."""
from __future__ import annotations

import json
import math
from pathlib import Path

_HERE = Path(__file__).resolve().parent


def load_contract(path=None):
    """📜 Validate the retained portable motion contract against its exact schema values."""
    value = json.loads(Path(path or _HERE / "cursor-motion.json").read_text(encoding="utf-8"))
    schema = json.loads((_HERE / "cursor-motion.schema.json").read_text(encoding="utf-8"))
    def validate(item, rule):
        """🔎 Enforce the finite JSON subset used by the schema without a runtime dependency."""
        if "const" in rule:
            expected = rule["const"]
            if type(item) is bool or item != expected or isinstance(item, float) and not math.isfinite(item):
                raise ValueError("Motion contract differs from its schema")
        if rule.get("type") == "object":
            if not isinstance(item, dict) or set(item) != set(rule["required"]):
                raise ValueError("Motion contract requires its exact object fields")
            for key, child in rule["properties"].items():
                validate(item[key], child)
        if rule.get("type") == "array":
            if not isinstance(item, list) or len(item) != rule["minItems"]:
                raise ValueError("Motion contract requires its exact array length")
            for child, child_rule in zip(item, rule["prefixItems"]):
                validate(child, child_rule)
    validate(value, schema)
    return value


_CONTRACT = load_contract()
_DURATION = sum(stage["durationSeconds"] for stage in _CONTRACT["stages"])


def _finite(value):
    """♾️ Reject nonnumeric values before applying normalized motion arithmetic."""
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise ValueError("Motion needs finite numeric values")
    return float(value)


def _point(value):
    """📍 Accept exactly two finite screen coordinates."""
    if not isinstance(value, (list, tuple)) or len(value) != 2:
        raise ValueError("Motion coordinates need a pair")
    return tuple(_finite(component) for component in value)


def sample(t, park, target):
    """🎯 Return eased position, explicit reveal flag and the half-open timeline stage."""
    seconds = min(1.0, max(0.0, _finite(t))) * _DURATION
    park, target = _point(park), _point(target)
    start = 0.0
    for index, entry in enumerate(_CONTRACT["stages"]):
        end = start + entry["durationSeconds"]
        if seconds < end or index == len(_CONTRACT["stages"]) - 1:
            stage = entry["id"]
            if stage in ("idle", "settle"):
                return park, False, stage
            if stage == "hover":
                return target, True, stage
            amount = min(1.0, max(0.0, (seconds - start) / entry["durationSeconds"]))
            amount = amount * amount * (3.0 - 2.0 * amount)
            first, last = (park, target) if stage == "approach" else (target, park)
            return tuple(a * (1.0 - amount) + b * amount for a, b in zip(first, last)), False, stage
        start = end
