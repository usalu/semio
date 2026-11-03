"""🧾 pa: JS-compatible JSON read and write for catalog assets (Python prints 1.6e-05 where the Node writer prints 0.000016)."""
import json
import re
from decimal import Decimal
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CATALOG = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
EXPONENT = re.compile(r"^(\s*(?:\"[^\"]*\": )?)(-?\d+(?:\.\d+)?e-\d+)(,?)$", re.MULTILINE)


def locate(asset_id):
    return next(CATALOG.glob(f"*/{asset_id}/🔣️.json"))


def dumps(data):
    text = json.dumps(data, ensure_ascii=False, indent=2)
    return EXPONENT.sub(lambda m: m.group(1) + format(Decimal(m.group(2)), "f") + m.group(3), text) + "\n"


def load(asset_id):
    path = locate(asset_id)
    raw = path.read_text(encoding="utf8")
    data = json.loads(raw)
    assert dumps(data) == raw, f"{asset_id}: roundtrip differs"
    return path, data


def save(path, data):
    path.write_text(dumps(data), encoding="utf8", newline="\n")
