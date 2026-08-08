#!/usr/bin/env python3
"""Fix JSON schema types for string/enum fields in generated leaves."""
import json
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")

STRING_FIELDS = {
    "en1995": {"annex", "service_class", "load_duration"},
    "en1996": {"annex", "design_situation", "unit", "masonry_class", "exposure", "mortar"},
    "en1997": {"annex", "design_approach"},
    "en1998": {
        "ground_type",
        "importance_class",
        "structural_system",
        "annex",
        "en_ground_type",
        "en_spectrum_type",
        "retrofit_knowledge_level",
        "retrofit_limit_state",
    },
    "en1999": {"alloy", "annex"},
}

BOOL = {
    "en1998": {"multiple_resisting_systems", "tower_is_chimney"},
}

INT = {
    "en1996": {"fire_resistance_min", "storeys"},
    "en1997": {"pile_n_profiles"},
    "en1998": {"seismic_zone"},
}


def snake_to_camel(s: str) -> str:
    parts = s.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:])


def fix_file(path: Path, key: str) -> None:
    data = json.loads(path.read_text())
    props = data.get("properties", {})
    for name, spec in list(props.items()):
        # reverse camel to snake rough match
        snake = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
        if snake in STRING_FIELDS.get(key, set()) or name in ("serviceClass", "loadDuration"):
            props[name] = {"type": "string", "x-semio-state": spec.get("x-semio-state", "persistent")}
        elif snake in BOOL.get(key, set()):
            props[name] = {"type": "boolean", "x-semio-state": spec.get("x-semio-state", "persistent")}
        elif snake in INT.get(key, set()):
            props[name] = {"type": "integer", "x-semio-state": spec.get("x-semio-state", "persistent")}
    data["properties"] = props
    path.write_text(json.dumps(data, indent=2) + "\n")


def main() -> None:
    for key in ("en1995", "en1996", "en1997", "en1998", "en1999"):
        for facet in ("🧬️schema", "📸️snapshot/🧬️schema", "🔺️diff/🧬️schema"):
            p = ROOT / f"📘️{key}" / facet / "🔣️component.json"
            if p.exists():
                fix_file(p, key)


if __name__ == "__main__":
    main()
