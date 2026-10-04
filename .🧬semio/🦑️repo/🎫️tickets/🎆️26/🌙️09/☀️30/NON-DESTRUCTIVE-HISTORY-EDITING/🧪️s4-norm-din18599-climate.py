#!/usr/bin/env python3
"""🌦️ S4-NORM: DIN V 18599 climate as parent-owned state + derived `climateTable` child (design §20.15 model (a)).

Rewrites the snapshot / artifact / diff JSON schemas and every committed snapshot-shaped JSON fixture of the din18599
subset text-preservingly: the old `climate` child identity becomes `climateTable` (id re-derived from the climate as
`din18599-climate-<sha256(canonical climate JSON)[:16]>`), and `climate` carries the Potsdam monthly climate every
committed document holds (both historic ids are the Potsdam climate). `--check` prints the pending count and writes nothing.
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
SUBSET = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = SUBSET / "🧬️schema"
SNAPSHOT_SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/norm/din18599/snapshot/schema.json"
POTSDAM = {
    "thetaEC": [-0.4, 0.6, 4.1, 8.4, 13.4, 16.6, 18.4, 17.9, 14.0, 9.2, 4.4, 1.0],
    "gHWM2": [25.0, 50.0, 95.0, 145.0, 185.0, 200.0, 195.0, 170.0, 120.0, 70.0, 35.0, 20.0],
}
HISTORIC_POTSDAM_IDS = {"din18599-climate-e44b9e389c197214", "din18599-climate-36af55c7f03468dc"}


def canonical(climate: dict) -> str:
    """🧮 `semio_framework_pack_json::to_json_string` of a `MonthlyClimate`: compact, camelCase, binary64 shortest form."""
    return '{"thetaEC":[%s],"gHWM2":[%s]}' % (",".join(repr(float(v)) for v in climate["thetaEC"]), ",".join(repr(float(v)) for v in climate["gHWM2"]))


def derived_id(climate: dict) -> str:
    return "din18599-climate-" + hashlib.sha256(canonical(climate).encode()).hexdigest()[:16]


def climate_block(indent: str, climate: dict, table: dict) -> str:
    inner = indent + "  "
    def array(values: list[float]) -> str:
        return "[\n" + ",\n".join(f"{inner}  {repr(float(v))}" for v in values) + f"\n{inner}]"
    table_text = json.dumps(table, indent=2, ensure_ascii=False).replace("\n", "\n" + indent)
    return (f'{indent}"climate": {{\n{inner}"thetaEC": {array(climate["thetaEC"])},\n{inner}"gHWM2": {array(climate["gHWM2"])}\n{indent}}},\n'
            f'{indent}"climateTable": {table_text}')


def rewrite_fixture(text: str) -> str | None:
    document = json.loads(text)
    if not isinstance(document, dict) or not isinstance(document.get("climate"), dict) or "childId" not in document["climate"]:
        return None
    child = document["climate"]
    start = text.index('\n  "climate": {') + 1
    end = text.index("\n  }", start) + len("\n  }")
    if json.loads("{" + text[start:end] + "}")["climate"] != child:
        raise SystemExit("climate block is not the top-level child identity")
    if child["childId"] in HISTORIC_POTSDAM_IDS:
        identity = derived_id(POTSDAM)
        table = {"childId": identity, "target": {"artifactId": identity, "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"}}}
    else:
        table = child
    return text[:start] + climate_block("  ", POTSDAM, table) + text[end:]


def rewrite_schema(path: Path, diff: bool) -> str | None:
    text = path.read_text()
    schema = json.loads(text)
    properties = schema["properties"]
    if "climateTable" in properties:
        return None
    snapshot = json.loads((SCHEMA / "📸️snapshot/🔣️.json").read_text())
    monthly = {key: value for key, value in snapshot["$defs"]["MonthlyClimate"].items() if key != "title"}
    rebuilt = {}
    for key, value in properties.items():
        if key == "climate":
            rebuilt["climate"] = {"anyOf": [{"$ref": f"{SNAPSHOT_SCHEMA_ID}#/$defs/MonthlyClimate"}, {"type": "null"}], "x-semio-state": "artifact"} if diff else monthly
            rebuilt["climateTable"] = value
        else:
            rebuilt[key] = value
    schema["properties"] = rebuilt
    if "required" in schema:
        required = schema["required"]
        required.insert(required.index("climate") + 1, "climateTable")
    return json.dumps(schema, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")


def derivation_fixture() -> str:
    """🧾️ Language-neutral expectation of the derived climate table: canonical JSON, content id (Python `hashlib`, an
    independent SHA-256), and the table rows as Rust `f64` Display lexemes."""
    lexeme = lambda value: str(int(value)) if float(value).is_integer() else repr(float(value))
    fixture = {
        "climate": POTSDAM,
        "canonicalJson": canonical(POTSDAM),
        "childId": derived_id(POTSDAM),
        "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "table"},
        "columns": ["thetaEC", "gHWM2"],
        "rows": [[lexeme(theta), lexeme(g)] for theta, g in zip(POTSDAM["thetaEC"], POTSDAM["gHWM2"])],
    }
    return json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"


def dsl_climate() -> str:
    """🗣️ The canonical `.din18599` DSL fields of the Potsdam climate (fixed-arity `[f64; 12]` = packed `a,b,…` tuple, `f64`
    Display spelling) and its derived table handle."""
    number = lambda value: str(int(value)) if float(value).is_integer() else repr(float(value))
    identity = derived_id(POTSDAM)
    return (f'climate=theta-e-c={",".join(number(v) for v in POTSDAM["thetaEC"])} g-h-w-m2={",".join(number(v) for v in POTSDAM["gHWM2"])} '
            f'climate-table=child_id={identity} target="{identity}!s.stdio.semio@v1/table"')


def rewrite_dsl(text: str) -> str | None:
    bracketed = re.search(r"climate=theta-e-c=\[ [^]]* \] g-h-w-m2=\[ [^]]* \] climate-table=child_id=\S+ target=\"[^\"]*\"", text)
    if bracketed and bracketed.group(0) != dsl_climate():
        return text.replace(bracketed.group(0), dsl_climate())
    for historic in sorted(HISTORIC_POTSDAM_IDS):
        old = f'climate=child_id={historic} target="{historic}!s.stdio.semio@v1/table"'
        if old in text:
            return text.replace(old, dsl_climate())
    return None


def main(argv: list[str]) -> int:
    check = "--check" in argv
    if derived_id(POTSDAM) != "din18599-climate-e44b9e389c197214":
        raise SystemExit("canonical climate JSON drifted from the Rust content id")
    pending: list[tuple[Path, str]] = []
    for path, diff in ((SCHEMA / "📸️snapshot/🔣️.json", False), (SCHEMA / "🔣️.json", False), (SCHEMA / "🔺️diff/🔣️.json", True)):
        new = rewrite_schema(path, diff)
        if new is not None:
            pending.append((path, new))
    for path in sorted(SUBSET.rglob("*.json")):
        if path.parent.name == "🧬️schema" or path.name != "🔣️.json" or "🧬️schema" in path.parts and "🧫️fixtures" not in path.parts:
            continue
        new = rewrite_fixture(path.read_text())
        if new is not None:
            pending.append((path, new))
    for path in sorted(SUBSET.glob("🖼️assets/*/🗣️.dsl.semio")):
        new = rewrite_dsl(path.read_text())
        if new is not None:
            pending.append((path, new))
    derivation = REPO / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🧫️fixtures/🧫️climate-table-derivation/🔣️.json"
    if not derivation.exists() or derivation.read_text() != derivation_fixture():
        pending.append((derivation, derivation_fixture()))
    if not check:
        for path, new in pending:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(new)
    print(f"pending={len(pending)}" if check else f"written={len(pending)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
