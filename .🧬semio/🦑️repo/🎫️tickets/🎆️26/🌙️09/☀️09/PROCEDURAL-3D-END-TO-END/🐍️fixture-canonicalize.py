#!/usr/bin/env python3
"""🧬️ Retags the committed `🦠️mutation/🔣️.json` quintet member from the pre-`dsl::Mutations`
externally-tagged shape `{"Variant": {...}}` onto the artifact's declared internally-tagged wire
contract `#[value(tag = "mutation", rename_all = "camelCase")]` — `{"mutation": "variant", ...}`.
Ticket 26/09/09/PROCEDURAL-3D-END-TO-END."""
import json, sys, pathlib, collections

def camel(name: str) -> str:
    return name[0].lower() + name[1:]

def retag(path: pathlib.Path) -> bool:
    raw = path.read_text(encoding="utf8")
    doc = json.loads(raw, object_pairs_hook=collections.OrderedDict)
    if "mutation" in doc:
        return False
    if len(doc) != 1:
        raise SystemExit(f"{path}: expected exactly one externally-tagged variant, saw {list(doc)}")
    variant, payload = next(iter(doc.items()))
    if not isinstance(payload, dict):
        raise SystemExit(f"{path}: variant {variant} payload is not an object")
    out = collections.OrderedDict()
    out["mutation"] = camel(variant)
    out.update(payload)
    path.write_text(json.dumps(out, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    return True

changed = 0
for root in sys.argv[1:]:
    for path in sorted(pathlib.Path(root).rglob("🦠️mutation/🔣️.json")):
        if retag(path):
            changed += 1
            print("retagged", path)
print(f"{changed} mutation fixture(s) retagged")


def apply_canonical(dump: str) -> None:
    """🔣️ Rewrites each committed fixture JSON from the artifact's OWN encoder output (captured as
    `[DEBUG] FIXTURE <path>\\t<canonical json>` lines), pretty-printed at the committed 2-space
    indent. Decode→encode preserves every value exactly; only the ENCODING is normalised, so the
    committed expectations stay the contract."""
    import json, pathlib
    count = 0
    for line in pathlib.Path(dump).read_text(encoding="utf8").splitlines():
        if not line.startswith("[DEBUG] FIXTURE "):
            continue
        path, canonical = line[len("[DEBUG] FIXTURE "):].split("\t", 1)
        target = pathlib.Path(path).resolve()
        text = json.dumps(json.loads(canonical), indent=2, ensure_ascii=False) + "\n"
        if target.read_text(encoding="utf8") != text:
            target.write_text(text, encoding="utf8")
            count += 1
    print(f"{count} fixture encoding(s) canonicalised from {dump}")
