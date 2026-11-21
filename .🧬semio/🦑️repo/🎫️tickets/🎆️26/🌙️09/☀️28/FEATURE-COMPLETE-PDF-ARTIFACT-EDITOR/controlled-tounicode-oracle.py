"""🗺️ Independently witnesses controlled ToUnicode inputs through real PDF fonts."""

import hashlib
import json
from pathlib import Path
import re

import pymupdf

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "font-unicode"
PREFIX = b"""/CIDInit /ProcSet findresource begin
12 dict begin
begincmap
/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def
/CMapName /ControlledFixture def
/CMapType 2 def
"""
SUFFIX = b"""endcmap
CMapName currentdict /CMap defineresource pop
end end
"""


def witness(specimen, tag, width, replacement=None):
    with pymupdf.open(GENERATED / specimen) as pdf:
        page = pdf[0]
        previous_raster = hashlib.sha256(page.get_pixmap(alpha=False).samples).hexdigest()
        owner = page.get_fonts(full=True)[0][0]
        kind, value = pdf.xref_get_key(owner, "ToUnicode")
        if kind == "xref":
            cmap = int(value.split()[0])
        else:
            assert replacement is not None
            cmap = pdf.get_new_xref()
            pdf.update_object(cmap, "<< >>")
            pdf.xref_set_key(owner, "ToUnicode", f"{cmap} 0 R")
        if replacement is not None:
            pdf.update_stream(cmap, PREFIX + replacement + SUFFIX, compress=False)
        pdf.save(OUT / (tag + ".pdf"))
    with pymupdf.open(OUT / (tag + ".pdf")) as pdf:
        assert not pdf.is_repaired
        page = pdf[0]
        content = pdf.xref_stream(page.get_contents()[0]).decode("ascii")
        body = bytes.fromhex(re.search(r"<([0-9a-fA-F]+)>\s*Tj", content)[1])
        codes = [int.from_bytes(body[index:index + width], "big") for index in range(0, len(body), width)]
        text = page.get_text().strip("\n")
        assert len(codes) == len(text)
        assert hashlib.sha256(page.get_pixmap(alpha=False).samples).hexdigest() == previous_raster
        (OUT / (tag + ".cmap")).write_bytes(pdf.xref_stream(cmap))
        return {"name": tag, "width": width, "codes": codes, "text": text, "pageRgbSha256": previous_raster}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    rows = [
        witness("raw-cff.pdf", "authored-bfchar", 2),
        witness("raw-cff.pdf", "edited-bfrange", 2, b"""1 begincodespacerange
<0000> <FFFF>
endcodespacerange
2 beginbfchar
<2548> <0041>
<93e5> <D83DDE00>
endbfchar
1 beginbfrange
<4e44> <4efc> <0042>
endbfrange
"""),
        witness("cff-opentype.pdf", "one-byte-simple", 1, b"""1 begincodespacerange
<00> <FF>
endcodespacerange
1 beginbfchar
<E9> <00E9>
endbfchar
1 beginbfrange
<41> <67> <0041>
endbfrange
"""),
    ]
    receipt = {"oracle": "PyMuPDF " + pymupdf.VersionBind, "cases": rows}
    (OUT / "expect.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(receipt["oracle"], [(row["name"], row["text"]) for row in rows])


if __name__ == "__main__":
    main()
