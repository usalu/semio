"""🧪️ W2-W-bim: rewrites every ifc (2x3, 4) and step (ap214) `🥒️.feature` Examples row to the leaf wire payload.

`params` becomes exactly the leaf's `payload_value()` shape (design §11, `📓️w2-s-report.md` F10): the hand grammar
`{"t": …, "v": …}` becomes the artifact's own value wire (`Part21Value` for ifc 2x3, `IfcValue` for ifc 4, `StepValue`
for step), `set-snapshot` carries a whole snapshot record, and `no-mutation` rows and baseline scenarios are removed.
Idempotent: an already-wire row is left as it is. Usage: `python3 🧪️w2-w-bim-features.py [--check]`.
"""

import json
import pathlib
import re
import sys
from decimal import Decimal

ROOT = pathlib.Path(__file__).resolve().parents[7]
ARTIFACTS = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

IFC2X3_SNAPSHOT = {"snapshot": {"schema": "stdio.ifc.2x3", "document": {
    "header": {
        "fileDescription": [{"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [CoordinationView_V2.0]"}]}, {"kind": "str", "value": "2;1"}],
        "fileName": [{"kind": "str", "value": "wellness-center-sama-project"}, {"kind": "str", "value": "2021-11-21T06:45:25"}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "str", "value": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"kind": "str", "value": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"kind": "str", "value": ""}],
        "fileSchema": [{"kind": "list", "values": [{"kind": "str", "value": "IFC2X3"}]}],
    },
    "instances": [{"id": 120, "entities": [{"typeName": "IFCPROJECT", "arguments": [{"kind": "str", "value": "0a3v3dJi10mxIqGCSrYdxN"}, {"kind": "unset"}, {"kind": "str", "value": "0001"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "str", "value": "Project Name"}, {"kind": "str", "value": "Project Status"}, {"kind": "unset"}, {"kind": "unset"}]}]}],
}}}

IFC4_SNAPSHOT = {"snapshot": {"schema": "stdio.ifc", "header": {
    "fileDescription": [{"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[DesignTransferView]"}]}, {"kind": "string", "value": "2;1"}],
    "fileName": [{"kind": "string", "value": "nakagin-capsule-tower-project.ifc"}, {"kind": "string", "value": "2026-03-20T21:51:27+00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "Nobody"}],
    "fileSchema": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4"}]}],
}, "entities": [{"id": 1, "name": "IFCPROJECT", "args": [{"kind": "string", "value": "1K7GYWVD13lBqV_e0J2CvF"}, {"kind": "unset"}, {"kind": "string", "value": "Metabolism"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}]}]}}

STEP_SNAPSHOT = {"snapshot": {"schema": "stdio.step", "header": {
    "fileDescription": {"description": [""], "implementationLevel": "2;1"},
    "fileName": {"name": "", "timestamp": "", "author": [""], "organization": [""], "preprocessorVersion": "", "originatingSystem": "", "authorization": ""},
    "fileSchema": {"schemas": ["AUTOMOTIVE_DESIGN"]},
}, "entities": [
    {"id": 1, "name": "PRODUCT", "args": [{"string": "Document"}, {"string": "Document"}, {"string": ""}]},
    {"id": 2, "name": "PRODUCT_DEFINITION_FORMATION", "args": [{"string": "A"}, "unset", {"reference": 1}]},
    {"id": 3, "name": "PRODUCT_DEFINITION", "args": [{"string": "A"}, "unset", {"reference": 2}, "unset"]},
]}}


def decimal(value: float) -> dict:
    """🔢️ `Part21Decimal` wire of a real, spelled like `Part21Decimal::from_f64`."""
    text = format(Decimal(repr(float(value))), "f")
    text = text.rstrip("0").rstrip(".") if "." in text else text
    negative = text.startswith("-")
    integer, _, fraction = text.lstrip("-").partition(".")
    return {"negative": negative, "coefficient": integer + fraction, "scale": len(fraction)}


def part21_value(value):
    """🧾️ `{t, v}` → `Part21Value` wire (`{kind: str|ref|int|real|enum|list|typed|unset|derived}`)."""
    if not isinstance(value, dict) or "kind" in value:
        return value
    tag = value["t"]
    simple = {"string": "str", "reference": "ref", "integer": "int", "enum": "enum"}
    if tag in ("unset", "derived"):
        return {"kind": tag}
    if tag in simple:
        return {"kind": simple[tag], "value": value["v"]}
    if tag == "real":
        return {"kind": "real", "value": decimal(value["v"])}
    if tag == "aggregate":
        return {"kind": "list", "values": [part21_value(item) for item in value["v"]]}
    if tag == "typed":
        return {"kind": "typed", "typeName": value["name"], "values": [part21_value(value["v"])]}
    raise ValueError(tag)


def ifc_value(value):
    """🧾️ `{t, v}` → `IfcValue` wire (adjacently tagged `{kind, value}`)."""
    if not isinstance(value, dict) or "kind" in value:
        return value
    tag = value["t"]
    if tag in ("unset", "derived"):
        return {"kind": tag}
    if tag in ("string", "reference", "integer", "real", "enum"):
        return {"kind": tag, "value": value["v"]}
    if tag == "aggregate":
        return {"kind": "aggregate", "value": [ifc_value(item) for item in value["v"]]}
    if tag == "typed":
        return {"kind": "typedValue", "value": {"name": value["name"], "items": [ifc_value(value["v"])]}}
    raise ValueError(tag)


def step_value(value):
    """🧾️ `{t, v}` → `StepValue` wire (externally tagged: `"unset"` or `{"real": 2.5}`)."""
    if not isinstance(value, dict) or "t" not in value:
        return value
    tag = value["t"]
    if tag in ("unset", "derived"):
        return tag
    if tag in ("string", "reference", "integer", "real", "enum"):
        return {tag: value["v"]}
    if tag == "aggregate":
        return {"aggregate": [step_value(item) for item in value["v"]]}
    if tag == "typed":
        return {"typedValue": {"typeName": value["name"], "value": step_value(value["v"])}}
    raise ValueError(tag)


def ifc2x3_row(kind: str, params: dict) -> dict:
    if kind == "set-snapshot":
        return IFC2X3_SNAPSHOT
    if kind == "upsert-instance":
        instance = params["instance"]
        entities = [entity if "typeName" in entity else {"typeName": entity["name"], "arguments": [part21_value(arg) for arg in entity["args"]]} for entity in instance["entities"]]
        return {"instance": {"id": instance["id"], "entities": entities}}
    if kind == "set-header":
        return {"header": {key: [part21_value(item) for item in values] for key, values in params["header"].items()}}
    return params


def ifc4_row(kind: str, params: dict) -> dict:
    if kind == "set-snapshot":
        return IFC4_SNAPSHOT
    converted = dict(params)
    if "values" in converted:
        converted["values"] = [ifc_value(item) for item in converted["values"]]
    if "value" in converted:
        converted["value"] = ifc_value(converted["value"])
    if "entity" in converted:
        converted["entity"] = {**converted["entity"], "args": [ifc_value(arg) for arg in converted["entity"]["args"]]}
    return converted


def step_row(kind: str, params: dict) -> dict:
    if kind == "set-snapshot":
        return STEP_SNAPSHOT
    converted = dict(params)
    if "value" in converted:
        converted["value"] = step_value(converted["value"])
    if "entity" in converted:
        converted["entity"] = {**converted["entity"], "args": [step_value(arg) for arg in converted["entity"]["args"]]}
    return converted


def family(path: pathlib.Path):
    text = str(path)
    if "/📐️step/" in text:
        return step_row
    if "/4️⃣4/" in text:
        return ifc4_row
    return ifc2x3_row


def cell(value) -> str:
    rendered = json.dumps(value, ensure_ascii=False, separators=(", ", ": "))
    if "|" in rendered:
        raise ValueError("a table cell must not carry `|`: " + rendered)
    return rendered


def rewrite(path: pathlib.Path) -> str:
    convert = family(path)
    lines = path.read_text().split("\n")
    out = []
    table = []

    def flush():
        if not table:
            return
        rows = [row for row in table if row[0] != "no-mutation"]
        width = [max(len(row[index]) for row in rows) for index in range(2)]
        indent = table_indent[0]
        out.extend(f"{indent}| {row[0].ljust(width[0])} | {row[1].ljust(width[1])} |" for row in rows)
        table.clear()

    table_indent = [""]
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("|") and stripped.endswith("|"):
            cells = [part.strip() for part in stripped[1:-1].split("|", 1)]
            table_indent[0] = line[: len(line) - len(line.lstrip())]
            if cells[0] in ("id",):
                table.append(cells)
            else:
                table.append([cells[0], cell(convert(cells[0], json.loads(cells[1])))])
            continue
        flush()
        out.append(line)
    flush()
    text = "\n".join(out)
    return re.sub(r"\n  @id-no-mutation-baseline-[a-z-]+\n(?:  @[^\n]+\n)*  Scenario:[^\n]*\n(?:    [^\n]*\n)*", "", text)


def main() -> int:
    check = "--check" in sys.argv
    changed = 0
    for path in sorted(list((ARTIFACTS / "🏗️ifc").rglob("🥒️.feature")) + list((ARTIFACTS / "📐️step").rglob("🥒️.feature"))):
        before = path.read_text()
        after = rewrite(path)
        if after != before:
            changed += 1
            print(("would rewrite " if check else "rewrote ") + str(path.relative_to(ROOT)))
            if not check:
                path.write_text(after)
    print(f"{changed} feature file(s) {'would change' if check else 'changed'}")
    return 1 if check and changed else 0


if __name__ == "__main__":
    sys.exit(main())
