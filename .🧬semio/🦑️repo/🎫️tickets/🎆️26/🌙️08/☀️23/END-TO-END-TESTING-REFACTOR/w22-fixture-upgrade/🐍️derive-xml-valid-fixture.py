"""📰️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

XML 1.0 Fifth Edition §2.8 makes a document *valid* only if it carries a document type declaration
whose Name is the document element's name. Exactly ONE committed document in this repository
satisfies that — `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/macos/tech.semio.document.uttype.plist`,
the real Uniform Type Identifier declaration macOS reads to associate `.semio` files with the app —
and it is 631 bytes.

This script derives a large one from real committed CONTENT, in the same Apple PropertyList 1.0
dialect and under the same real Apple DOCTYPE, by reading the real 50-row German
building-material-reuse survey `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv`
with Python's own stdlib `csv` module and serialising it with Python's own stdlib `plistlib` — two
independent stdlib implementations, neither of which knows anything about this repository. Every
key is a real column header of the real survey and every string is a real cell of it; the DOCTYPE,
the `<plist version="1.0">` document element and the indentation are `plistlib`'s own output for
Apple's own format.
"""

import csv
import plistlib
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
CSV = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv"
OUT = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧫️fixtures/📰️reuse-marketplaces-plist.xml"


def main():
    with CSV.open(encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle))
    document = {"SurveyIdentifier": "reuse-marketplaces", "SurveyRowCount": len(rows), "SurveyRows": rows}
    OUT.write_bytes(plistlib.dumps(document, fmt=plistlib.FMT_XML, sort_keys=False))
    print("rows=%d columns=%d bytes=%d" % (len(rows), len(rows[0]), OUT.stat().st_size))
    print(OUT.read_text(encoding="utf-8")[:420])


if __name__ == "__main__":
    main()
