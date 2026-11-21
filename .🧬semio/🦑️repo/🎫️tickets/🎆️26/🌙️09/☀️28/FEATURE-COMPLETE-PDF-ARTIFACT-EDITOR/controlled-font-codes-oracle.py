"""🔎️ Extract authored physical codes and independent Unicode evidence with MuPDF."""

import json
from pathlib import Path
import re

import pymupdf


def _native(out, specimen, width):
    with pymupdf.open(out / specimen) as doc:
        page = doc[0]
        content = doc.xref_stream(page.get_contents()[0]).decode("ascii")
        payload = bytes.fromhex(re.search(r"<([0-9A-Fa-f]+)>\s*Tj", content)[1])
        assert len(payload) % width == 0 and page.get_fonts(full=True)
        codes = [int.from_bytes(payload[index:index + width], "big") for index in range(0, len(payload), width)]
        return {"name": specimen, "width": width, "program": content,
                "show": codes, "array": [], "adjustments": [],
                "independentText": page.get_text(), "independentEncoding": page.get_fonts(full=True)[0][5]}


def main():
    out = Path(__file__).resolve().parent / "🗑️generated"
    cases = [_native(out, "cff-opentype.pdf", 1), _native(out, "raw-cff.pdf", 2)]
    cases.extend([
        {"name": "one-byte Tj and TJ", "width": 1,
         "program": "BT /F1 12 Tf <4142a000> Tj [(A) -25 <4180> 12 <ff>] TJ ET",
         "show": [65, 66, 160, 0], "array": [[65], [65, 128], [255]], "adjustments": [-25, 12]},
        {"name": "two-byte Tj and TJ", "width": 2,
         "program": "BT /F1 12 Tf <00410042FFFF> Tj [<00000041> 120 <2042>] TJ ET",
         "show": [65, 66, 65535], "array": [[0, 65], [8258]], "adjustments": [120]},
    ])
    data = {"oracle": "PyMuPDF " + pymupdf.VersionBind, "cases": cases}
    (out / "font-codes" / "expect.json").write_text(json.dumps(data, indent=2) + "\n")
    print(data["oracle"], [(row["name"], row.get("independentText")) for row in cases])


if __name__ == "__main__":
    main()
