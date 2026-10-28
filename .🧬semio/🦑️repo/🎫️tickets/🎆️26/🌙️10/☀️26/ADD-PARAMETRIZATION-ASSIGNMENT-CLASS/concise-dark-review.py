"""🔎 Independently inspect and render the concise German dark assignment."""
from pathlib import Path
import json
import pdfplumber
import pypdfium2 as pdfium
from pypdf import PdfReader
from PIL import Image

ticket = Path(__file__).resolve().parent
source = ticket / "🗑️generated/compiled-dark/parametrization-de-dark.pdf"
output = ticket / "🗑️generated/review-concise-dark"
output.mkdir(parents=True, exist_ok=True)
reader = PdfReader(source)
assert len(reader.pages) == 3
assert reader.trailer["/Root"]["/Lang"] == "de-DE"
log = source.with_suffix(".log").read_text(encoding="utf-8")
assert "Overfull" not in log
assert "Underfull" not in log
with pdfplumber.open(source) as pdf:
    pages = []
    for page in pdf.pages:
        outside = [char["text"] for char in page.chars if char["text"].strip() and (char["x0"] < 0 or char["x1"] > page.width + 1 or char["top"] < 0 or char["bottom"] > page.height + 1)]
        assert not outside, outside
        pages.append(page.extract_text())
    first = pdf.pages[0]
    edges = [line for line in first.lines if abs(line.get("linewidth", 0) - .8) < .05]
    assert len(edges) == 3, len(edges)
    edge_bounds = [min(e["x0"] for e in edges), min(e["top"] for e in edges), max(e["x1"] for e in edges), max(e["bottom"] for e in edges)]
    graph_objects = [obj for obj in first.lines + first.rects if abs(obj.get("linewidth", 0) - .5) < .05 and obj["top"] > edge_bounds[1] - 30 and obj["bottom"] < edge_bounds[3] + 30]
    graph_objects += [char for char in first.chars if char["text"] in {"1", "2", "3"} and char["top"] > edge_bounds[1] - 30 and char["bottom"] < edge_bounds[3] + 30]
    graph_width = (max(obj["x1"] for obj in graph_objects) - min(obj["x0"] for obj in graph_objects)) / (72 / 25.4)
    graph_height = (max(obj["bottom"] for obj in graph_objects) - min(obj["top"] for obj in graph_objects)) / (72 / 25.4)
    assert 165 < graph_width < 171, graph_width
    assert graph_height < 55, graph_height
    for marker in ["Beispiel", "Länge", "Breite", "Höhe", "+1", "Konsistent"]:
        assert marker in pages[0], marker
result = {"pages": len(reader.pages), "bytes": source.stat().st_size, "title": reader.metadata.title, "language": reader.trailer["/Root"]["/Lang"], "overfull": log.count("Overfull"), "underfull": log.count("Underfull"), "k3SelectedEdges": len(edges), "k3GraphWidthMm": graph_width, "k3GraphHeightMm": graph_height, "k3EdgeBoundsPoints": edge_bounds, "pageTexts": pages}
(output / "evidence.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
document = pdfium.PdfDocument(source)
thumbs = []
for index in range(len(document)):
    image = document[index].render(scale=2).to_pil()
    image.save(output / f"page-{index + 1}.png")
    image.thumbnail((420, 600))
    thumbs.append(image.copy())
contact = Image.new("RGB", (420 * len(thumbs), 600), "#dddddd")
for index, image in enumerate(thumbs):
    contact.paste(image, (420 * index, 0))
contact.save(output / "contact.png")
print(json.dumps({key: value for key, value in result.items() if key != "pageTexts"}))
