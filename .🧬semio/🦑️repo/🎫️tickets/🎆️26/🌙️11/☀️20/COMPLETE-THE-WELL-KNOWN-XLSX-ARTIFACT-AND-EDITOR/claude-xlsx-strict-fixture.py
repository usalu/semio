"""🔒️ Derives the genuine ISO/IEC 29500 Strict workbook from the Transitional one by renaming every Transitional namespace and relationship type to its Strict name and declaring `conformance="strict"` (the strict subset's own `reuse-marketplaces.xlsx` is a byte copy of the Transitional one). Usage: `python3 claude-xlsx-strict-fixture.py`."""

import zipfile
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets")
SOURCE = ROOT / "🧱️base/🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx"
TARGET = ROOT / "🔒️strict/🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx"
RENAMES = (
    ("http://schemas.openxmlformats.org/spreadsheetml/2006/main", "http://purl.oclc.org/ooxml/spreadsheetml/main"),
    ("http://schemas.openxmlformats.org/drawingml/2006/main", "http://purl.oclc.org/ooxml/drawingml/main"),
    ("http://schemas.openxmlformats.org/officeDocument/2006/relationships", "http://purl.oclc.org/ooxml/officeDocument/relationships"),
)

with zipfile.ZipFile(SOURCE) as source, zipfile.ZipFile(TARGET, "w", zipfile.ZIP_DEFLATED) as target:
    for info in source.infolist():
        data = source.read(info.filename)
        if info.filename.endswith((".xml", ".rels")):
            text = data.decode("utf-8")
            for old, new in RENAMES:
                text = text.replace(old, new)
            if info.filename == "xl/workbook.xml":
                text = text.replace("<workbook ", '<workbook conformance="strict" ', 1)
            data = text.encode("utf-8")
        entry = zipfile.ZipInfo(info.filename, date_time=(1980, 1, 1, 0, 0, 0))
        entry.compress_type = zipfile.ZIP_DEFLATED
        target.writestr(entry, data)
print(TARGET, TARGET.stat().st_size)
