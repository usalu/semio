"""🔢️ Rewrites every integral JSON number that sits at a `"type": "number"` position in the 35 committed
puzzle3d mutation quintets to its float spelling (`8` → `8.0`), leaving every `"type": "integer"`
position alone.

Why. `os_pack::json` — the canonical JSON writer every `dsl::json::to_json_string` call goes through —
documents its own contract in `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`: "A float and an integer of
the same magnitude are still never spelled the same way (`42.0` always keeps its `.0`)". The committed
fixtures spell integral `f64` fields as bare integers, so `committed_json_is_canonical`,
`committed_diff_is_canonical` and `produces_committed_diff` (87 cargo tests, one per verb × role) all
compare `Number(1.0)` against `Number(1)` and fail. The writer holds the contract; the fixtures are
stale. See 📓️2026-09-09-wave-D-production-defects.md §P9.

Why schema-driven and not "add .0 to every integer". `index` (create-object / create-reference /
create-target-volume / add-object-vortex), `order`, `revealIndex` and the fill-preview counters are
genuine integers; blanket conversion would corrupt them. The JSON Schemas beside the types already
carry the distinction (`number` vs `integer`), which is the schema-first source of truth this repo
requires.

Idempotent: re-running changes nothing. Verified by the Rust suite, which is the writer's own verdict.
"""

import glob
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = SUBSET / "🧬️schema"
ARTIFACT_ID = "https://semio.tech/schema/s/puzzle/puzzle3d/artifact.json"


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


ARTIFACT = load(SCHEMA / "🔣️.json")

SCALE = {"oneOf": [{"type": "number"}, {"type": "array", "items": {"type": "number"}}]}
"""📐️ `Puzzle3dScale` — `🗿️artifacts/🧊️3d/🦀️.rs:82`, a scalar-or-triple union of `f64`."""

SUPPLEMENT = {
    "Puzzle3dObject": {"scale": SCALE},
    "Puzzle3dTargetVolume": {"origin": {"type": "array", "items": {"type": "number"}}, "orientation": {"type": "array", "items": {"type": "number"}}, "scale": SCALE},
    "Puzzle3dReference": {"origin": {"type": "array", "items": {"type": "number"}}, "widthWorld": {"type": "number"}},
}
"""🩹 The number-vs-integer declarations the committed artifact JSON Schema leaves unstated: it spells
`Puzzle3dObject.scale` as `{}` and both `Puzzle3dTargetVolume` and `Puzzle3dReference` as
`additionalProperties: true` stubs carrying only `id`. Taken from the Rust records that ARE the
authority for those three types — `🗿️artifacts/🧊️3d/🦀️.rs:227` (`scale: Option<Puzzle3dScale>`), `:308`
(`origin: [f64; 3]`, `orientation: Option<[f64; 4]>`, `scale: Option<Puzzle3dScale>`) and `:347`
(`origin: [f64; 3]`, `width_world: f64`). Completing the committed schema itself is a separate change:
its GraphQL/proto/TypeScript siblings and the design-parity harness move with it."""

for title, properties in SUPPLEMENT.items():
    ARTIFACT["$defs"][title].setdefault("properties", {}).update(properties)


def resolve(node, root):
    """🔗️ Follows a `$ref` to the schema object it names and to the document that object's own local
    refs are relative to: an empty base is local to `root`, the artifact `$id` reaches the shared
    `$defs` every fixture role borrows its record types from."""
    seen = 0
    while isinstance(node, dict) and "$ref" in node and seen < 32:
        seen += 1
        base, _, pointer = node["$ref"].partition("#")
        if base == ARTIFACT_ID:
            root = ARTIFACT
        elif base != "":
            return {}, root
        node = root
        for token in [part for part in pointer.split("/") if part]:
            node = node.get(token.replace("~1", "/").replace("~0", "~"), {}) if isinstance(node, dict) else {}
    return (node if isinstance(node, dict) else {}), root


def types_of(node):
    declared = node.get("type")
    if isinstance(declared, str):
        return {declared}
    if isinstance(declared, list):
        return set(declared)
    return set()


def branches(node, root):
    """🌿️ Every `(alternative schema, its document)` a value may be measured against."""
    node, root = resolve(node, root)
    out = [(node, root)]
    for keyword in ("oneOf", "anyOf", "allOf"):
        for alternative in node.get(keyword, []):
            out.extend(branches(alternative, root))
    return out


def canonicalize(value, node, root):
    """🔢️ Returns `value` with every integral number at a `number` position respelled as a float."""
    options = branches(node, root)
    if isinstance(value, bool):
        return value
    if isinstance(value, int):
        if any("number" in types_of(option) for option, _ in options) and not any("integer" in types_of(option) for option, _ in options):
            return float(value)
        return value
    if isinstance(value, list):
        items = next(((option["items"], scope) for option, scope in options if option.get("items") is not None), None)
        return value if items is None else [canonicalize(entry, items[0], items[1]) for entry in value]
    if isinstance(value, dict):
        out = {}
        for key, entry in value.items():
            child = None
            for option, scope in options:
                properties = option.get("properties")
                if isinstance(properties, dict) and key in properties:
                    child = (properties[key], scope)
                    break
                additional = option.get("additionalProperties")
                if isinstance(additional, dict):
                    child = (additional, scope)
                    break
            out[key] = entry if child is None else canonicalize(entry, child[0], child[1])
        return out
    return value


def role_schema(path):
    """🗂️ The schema that governs one fixture file, by its role in the quintet."""
    name = path.parent.name
    if name in ("⬅️before", "➡️after"):
        return load(SCHEMA / "📸️snapshot/🔣️.json")
    if name == "🔺️diff":
        return load(SCHEMA / "🔺️diff/🔣️.json")
    if name == "🦠️mutation":
        verb = path.parents[3]
        return load(verb / "🧬️schema/🔣️.json")
    return None


def main():
    files = sorted(glob.glob(str(SCHEMA / "🧬️mutations/*/🧪️tests/*/**/🔣️.json"), recursive=True))
    changed = []
    for name in files:
        path = pathlib.Path(name)
        schema = role_schema(path)
        if schema is None:
            continue
        raw = path.read_text(encoding="utf-8")
        document = json.loads(raw)
        rewritten = canonicalize(document, schema, schema)
        out = json.dumps(rewritten, indent=2, ensure_ascii=False) + "\n"
        if out != raw:
            path.write_text(out, encoding="utf-8")
            changed.append(str(path.relative_to(ROOT)))
    print(f"canonicalized {len(changed)} of {len(files)} fixture files")
    for name in changed:
        print("  ", name)
    return 0


if __name__ == "__main__":
    sys.exit(main())
