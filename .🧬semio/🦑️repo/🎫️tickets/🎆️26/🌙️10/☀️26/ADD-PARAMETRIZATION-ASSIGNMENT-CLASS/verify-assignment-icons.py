import argparse
import hashlib
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET

parser = argparse.ArgumentParser()
parser.add_argument("--library", type=Path, required=True)
args = parser.parse_args()
sys.path.insert(0, str(args.library))
from pypdf import PdfReader
from pypdf.generic import ContentStream

workspace = Path.cwd()
root = workspace / "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment"
plan = json.loads((root / "parametrization-icons.json").read_text(encoding="utf-8"))
catalog = json.loads((workspace / "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json").read_text(encoding="utf-8"))
for resource in plan["resources"]:
    source = workspace / resource["source"]
    output = root / resource["output"]
    assert hashlib.sha256(source.read_bytes()).hexdigest() == resource["sha256"]
    assert output.exists(), str(output)
    pdf = PdfReader(output)
    assert len(pdf.pages) == 1
    assert pdf.metadata["/SourceSVG"] == resource["source"]
    assert pdf.metadata["/SourceSHA256"] == resource["sha256"]
    page = pdf.pages[0]
    svg = ET.parse(source).getroot()
    for name, dimension in [("width", page.mediabox.width), ("height", page.mediabox.height)]:
        assert svg.attrib[name].endswith("pt")
        assert abs(float(svg.attrib[name][:-2]) - float(dimension)) < 0.001, (resource["output"], name, svg.attrib[name], str(dimension))
    operations = ContentStream(page.get_contents(), pdf).operations
    assert sum(operator in [b"m", b"l", b"c", b"h"] for _, operator in operations) > 20
    for value in page.get("/Resources", {}).get("/XObject", {}).values():
        assert value.get_object().get("/Subtype") != "/Image"
    path = "🧾️template/📝️assignment/" + resource["output"]
    for document in catalog["documents"]:
        if document["id"] in ["parametrization-en", "parametrization-de"]:
            assert path in document["sources"], path
print("16 canonical icon hashes, vector PDF paths/dimensions/provenance and both document source stages: passed")
