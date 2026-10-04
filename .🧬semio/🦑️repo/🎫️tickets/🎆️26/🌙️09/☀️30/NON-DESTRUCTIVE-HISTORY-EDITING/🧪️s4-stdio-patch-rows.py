#!/usr/bin/env python3
"""🥒️ Adds the `patch-snapshot` Examples row of every stdio aggregate's mutate case (design §11 evidence rule, D4 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the row's params are exactly the leaf wire payload `{"patch": <one pointer
operation>}`, decoded generically by the subject adapter and read by the oracle against its own third-party reading of the
snapshot (`semio_repo_test_host::law::patched_snapshot`). One row per outline whose title contains a listed fragment (cells per outline title where the outlines' columns differ);
pptx rows are derived by `🧪️s4-stdio-pptx-rows.py` instead.
Idempotent; `--check` reports pending rows and exits 1 when any is pending.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs
"""
from __future__ import annotations

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ARTIFACTS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"


def patch(operation: str, path: str, **members) -> dict:
    return {"patch": {"operation": operation, "path": path, **members}}


ROWS = {
    "🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any": (["Apply <id> to the real mesh", "Undoing <id> restores the real mesh"], patch("set", "/solidName", value="patched-hexagonal-forest")),
    "☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header": (["Apply <id> to the real point cloud", "Undoing <id> restores the real point cloud"], patch("set", "/header/systemIdentifier", value="PATCHED-SYSTEM")),
    "🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any": (["Apply <id> to the real weather file's record grid", "Undoing <id> restores the real weather file"], patch("set", "/records/3/dryBulbTemp", value="12.3")),
    "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any": (["Apply <id> to the real table", "Undoing <id> restores the real table"], patch("set", "/records/1/fields/8/value", value="Beschreibung, Bilder, Preis")),
    "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any": (["Apply <id> to the real table", "Undoing <id> restores the real table"], patch("set", "/records/1/8", value="Beschreibung, Bilder, Preis")),
    "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base": (["Apply <id> to the real archive", "Undoing <id> restores the archive"], patch("set", "/comment", value="Projektfotos patched comment")),
    "🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any": (["Apply <id> to the real document", "Undoing <id> restores the original document"], patch("set", "/backgroundColorIndex", value=1)),
    "🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base": (["Apply <id> to the real animation", "Undoing <id> restores the real animation"], patch("set", "/backgroundColorIndex", value=3)),
    "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny": (["Apply <id> to the real drawing", "Undoing <id> restores the real drawing"], patch("set", "/doc/root/attrs/1/value", value="Layer_patched")),
    "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic": (["Apply <id> to the real drawing", "Undoing <id> restores the real drawing"], patch("set", "/doc/root/attrs/0/value", value="0 0 420 150")),
    "📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl": (["Apply <id> to the real video container", "Undoing <id> restores the real video container"], patch("set", "/mainHeader/suggestedBufferSize", value=2097152)),
    "🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header": (["Apply <id> to the real document", "Undoing <id> restores the document"], patch("set", "/headerVars/1", value={"name": "$INSBASE", "groupCode": 10, "value": {"kind": "point", "value": [15, 25, 0]}})),
    "🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry": (["Apply <id> to the real mesh", "Undoing <id> restores the real mesh"], patch("set", "/vertices/0/x", value=1.5)),
    "🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any": (["Apply <id> to the real document", "Undoing <id> restores the real document"], patch("set", "/comments/0", value="patched pattern-sphere comment")),
    "🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any": (["Apply <id> to the real exchange structure", "Undoing <id> restores the real exchange structure"], patch("set", "/header/fileDescription/0", value={"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[PatchedView]"}]})),
    "🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base": (["Apply <id> to the real building model", "Undoing <id> restores the real building model"], patch("set", "/document/header/fileDescription/0", value={"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [PatchedView]"}]})),
    "📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base": (["Apply <id> to the real exchange structure", "Undoing <id> restores the real exchange structure"], patch("set", "/header/fileDescription/description/0", value="patched description")),
    "🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any": (["Apply <id> to the R2010 container", "Undoing <id> restores the R2010 container"], patch("set", "/maintenanceVersion", value=7)),
    "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base": (["Apply <id> to the real document", "Undoing <id> restores the real document"], patch("set", "/value/members/1/value", value={"kind": "number", "lexeme": "7"})),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base": (["Apply <id> to the real document", "Undoing <id> restores the document"], patch("set", "/pages/15/mediaBox", value=[0, 0, 595, 842])),
    "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup": (["Apply <id> to the committed review pair", "Undoing <id> restores the committed review"], ["🔢️set-version-applied", json.dumps(patch("set", "/version", value="2.2"), ensure_ascii=False)]),
    "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any": (["Apply <id> to the real document", "Undoing <id> restores the real document"], {
        "Apply <id> to the real document": ["🪞️change-material-double-sided", json.dumps(patch("set", "/document/materials/0/doubleSided", value=True))],
        "Undoing <id> restores the real document": [json.dumps(patch("set", "/document/materials/0/doubleSided", value=True))],
    }),
    "🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any": (["Apply <id> to a small indexed document", "Undoing <id> restores a small indexed document"], patch("set", "/bytes/82", value=5)),
    "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline": (["Apply <id> to the real scan", "Undoing <id> puts the real scan"], ["stdio.jpg.baseline.sof-marker", json.dumps(patch("set", "/sofMarker", value=194))]),
    "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline": (["Apply <id> to the real scan", "Undoing <id> puts the real scan"], ["stdio.tiff.baseline.unsupported-compression", "{}", json.dumps(patch("set", "/ifds/0/entries/3/values/value", value=[5]))]),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation": (["Apply <id> to the real committed walk cycle", "Undoing <id> restores the real committed walk cycle"], {"mutation": "patchSnapshot", **patch("set", "/timelines/0/channels/1/interpolation", value="step")}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video": (["Apply <id> to the real recording", "Undoing <id> restores the real recording"], {"mutation": "patchSnapshot", **patch("set", "/streams/0/codec", value="vp9")}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad": (["Apply <id> to the real committed drawing", "Undoing <id> restores the real committed drawing"], {"mutation": "patchSnapshot", **patch("set", "/layers/0/colorIndex", value=5)}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio": (["Apply <id> to the real recording", "Undoing <id> restores the real recording"], {"mutation": "patchSnapshot", **patch("set", "/sampleRate", value=22050)}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow": (["Apply <id> to the real 180-node capsule network", "Undoing <id> restores the real 180-node capsule network"], {"mutation": "patchSnapshot", **patch("set", "/nodes/0/label", value="Kapselträger, Ostkern")}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model": (["Apply <id> to the real Nakagin Capsule Tower model", "Undoing <id> restores the real Nakagin Capsule Tower model"], {"mutation": "patchSnapshot", **patch("set", "/spatial/0/name", value="Kapselgeschoss")}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value": (["Apply <id> to the real building model", "Undoing <id> restores the real building model"], {"mutation": "patchSnapshot", **patch("set", "/nodes/0/value", value={"kind": "bytes", "value": [0, 1, 2, 255]})}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document": (["Apply <id> to the real committed memo", "Undoing <id> restores the real committed memo"], {"mutation": "patchSnapshot", **patch("set", "/styles/0/name", value="Patched style")}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text": (["Apply <id> to the real published article", "Undoing <id> restores the real published article"], [json.dumps({"PatchSnapshot": patch("set", "/runs/60/content", value="Baustellenblog Variowohnungen")}, ensure_ascii=False)]),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table": (["Apply <id> to the real 50-row survey table", "Undoing <id> restores the real 50-row survey table"], [json.dumps({"PatchSnapshot": patch("set", "/columns/0/name", value="Bewertung")}, ensure_ascii=False)]),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object": (["Apply <id> to the real committed crate object", "Undoing <id> restores the prepared crate object"], {"prepare": [], "mutation": {"PatchSnapshot": patch("set", "/transform/translation/x", value=-4.25)}}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph": (["Apply <id> to the real Nakagin Capsule Tower port gr", "Undoing <id> restores the real Nakagin Capsule Tower"], {"prepare": [], "mutation": {"PatchSnapshot": patch("set", "/nodes/0/label", value="Kapsel, gepatcht")}}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep": (["Apply <id> to the real concrete-forest structure", "Undoing <id> restores the prepared concrete-forest s"], {"prepare": [], "mutation": {"PatchSnapshot": patch("set", "/vertices/0/tol", value=0.0001)}}),
    "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit": (["Apply <id> to the real Nakagin Capsule Tower kit", "Undoing <id> restores the prepared Nakagin Capsule T"], {"prepare": [], "mutation": {"PatchSnapshot": patch("set", "/types/0/name", value="Kapsel, gepatcht")}}),
    "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base": (["Apply <id> to the real document", "Undoing <id> restores the document"], patch("set", "/doc/root/attrs/1/value", value="Layer_patched")),
    "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base": (["Apply <id> to the real workbook (independently reproducible)", "Undoing <id> restores the real workbook"], patch("set", "/xmlParts/5/document/root/children/0/children/0/children/0/text", value="Kennung (gepatcht)")),
    "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base": (["Apply <id> to the real document", "Undoing <id> restores the document"], patch("set", "/doc/root/children/0/children/0/children/0/children/0/attrs/0/value", value="Heading3")),
    "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid": (["Apply <id> to the real property-list document", "Undoing <id> restores the real property-list documen"], patch("set", "/doc/root/children/1/children/3/children/0/text", value="reuse-marketplaces-patched")),
}


def feature_of(subset: str, listed: list[str]) -> pathlib.Path:
    found = [name for name in listed if name.startswith(ARTIFACTS + subset + "/🧪️tests/") and name.endswith("/🥒️.feature") and "mutate" in name]
    if len(found) != 1:
        raise SystemExit(f"{subset}: expected one mutate feature, found {found}")
    return ROOT / found[0]


def with_rows(text: str, titles: list[str], cells: list[str] | dict[str, list[str]]) -> tuple[str, int]:
    lines = text.split("\n")
    out, added, position = [], 0, 0
    while position < len(lines):
        line = lines[position]
        out.append(line)
        position += 1
        stripped = line.strip()
        if not (stripped.startswith("Scenario Outline:") and any(title in stripped for title in titles)):
            continue
        while position < len(lines) and not lines[position].strip().startswith("Examples:"):
            out.append(lines[position])
            position += 1
        table = []
        out.append(lines[position])
        position += 1
        while position < len(lines) and lines[position].strip().startswith("|"):
            table.append(lines[position])
            position += 1
        if not any(row.strip().split("|")[1].strip() == "patch-snapshot" for row in table[1:]):
            indent = table[-1][: len(table[-1]) - len(table[-1].lstrip())]
            row_cells = next(value for title, value in cells.items() if title in stripped) if isinstance(cells, dict) else cells
            table.append(indent + "| " + " | ".join(row_cells) + " |")
            added += 1
        out.extend(table)
    return "\n".join(out), added


def main() -> int:
    check = "--check" in sys.argv
    listed = subprocess.run(["git", "ls-files", "-z", "--", ARTIFACTS], cwd=ROOT, capture_output=True, check=True).stdout.decode().split("\0")
    pending = 0
    for subset, (titles, row) in ROWS.items():
        feature = feature_of(subset, listed)
        cells = {title: ["patch-snapshot", *value] for title, value in row.items()} if isinstance(row, dict) and "patch" not in row and "mutation" not in row else ["patch-snapshot", *row] if isinstance(row, list) else ["patch-snapshot", json.dumps(row, ensure_ascii=False)]
        text, added = with_rows(feature.read_text(encoding="utf-8"), titles, cells)
        pending += added
        if added and not check:
            feature.write_text(text, encoding="utf-8")
        print(f"{subset}: {added} row(s) {'pending' if check else 'added'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
