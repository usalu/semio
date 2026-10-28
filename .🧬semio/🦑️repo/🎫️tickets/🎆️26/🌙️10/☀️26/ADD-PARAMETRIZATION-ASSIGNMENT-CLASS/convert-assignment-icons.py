import argparse
from io import BytesIO
import hashlib
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET

parser = argparse.ArgumentParser()
parser.add_argument("--library", type=Path, required=True)
args = parser.parse_args()
sys.path.insert(0, str(args.library))
from svglib.svglib import svg2rlg
from reportlab.graphics import renderPDF
from pypdf import PdfReader, PdfWriter

workspace = Path.cwd()
root = workspace / "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment"
plan = json.loads((root / "parametrization-icons.json").read_text(encoding="utf-8"))
for resource in plan["resources"]:
    source = workspace / resource["source"]
    content = source.read_bytes()
    if hashlib.sha256(content).hexdigest() != resource["sha256"]:
        raise ValueError("Canonical SVG changed: " + resource["source"])
    drawing = svg2rlg(str(source))
    if drawing is None:
        raise ValueError("SVG drawing required: " + resource["source"])
    svg = ET.fromstring(content)
    width, height = (float(svg.attrib[key][:-2]) for key in ["width", "height"])
    drawing.scale(width / drawing.width, height / drawing.height)
    drawing.width, drawing.height = width, height
    document = PdfReader(BytesIO(renderPDF.drawToString(drawing, invariant=1)))
    writer = PdfWriter()
    writer.clone_document_from_reader(document)
    writer.add_metadata({
        "/Title": source.stem,
        "/SourceSVG": resource["source"],
        "/SourceSHA256": resource["sha256"],
        "/AuthoringTransform": "svglib.svg2rlg + reportlab.renderPDF; canonical vector geometry",
    })
    output = root / resource["output"]
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("wb") as stream:
        writer.write(stream)
print("Authored 16 canonical SVG-derived vector PDF icon assets")
