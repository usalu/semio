"""🔍 Inspect final course PDFs and render every page from ticket-local native QA output."""

import importlib.util
import json
import sys
from pathlib import Path

import pdfplumber
import pypdfium2 as pdfium
from PIL import Image
from pypdf import PdfReader

TICKET = Path(__file__).resolve().parent
OUTPUT = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else TICKET / "🗑️generated/course-qa"
assert OUTPUT.is_relative_to(TICKET / "🗑️generated")
MM = 72 / 25.4
spec = importlib.util.spec_from_file_location("matrix_grid_oracle", TICKET / "verify-matrix-pdf.py")
matrix = importlib.util.module_from_spec(spec)
spec.loader.exec_module(matrix)
results = {}
for language, expected_language in (("en", "en-US"), ("de", "de-DE")):
    source = OUTPUT / "compiled" / language / f"{language}.pdf"
    reader = PdfReader(source)
    assert len(reader.pages) == 3
    assert reader.trailer["/Root"].get("/Lang") == expected_language
    assert "Parametr" in reader.metadata.title
    log = source.with_suffix(".log").read_text(encoding="utf-8")
    assert "Overfull" not in log and "Underfull" not in log
    review = OUTPUT / "review" / language
    review.mkdir(parents=True, exist_ok=True)
    document = pdfium.PdfDocument(source)
    thumbs = []
    for index in range(len(document)):
        page = document[index]
        image = page.render(scale=2).to_pil()
        image.save(review / f"page-{index + 1}.png")
        image.thumbnail((420, 600))
        thumbs.append(image.copy())
        page.close()
    document.close()
    contact = Image.new("RGB", (1260, 600), "#dddddd")
    for index, image in enumerate(thumbs):
        contact.paste(image, (420 * index, 0))
    contact.save(review / "contact.png")
    with pdfplumber.open(source) as pdf:
        for page in pdf.pages:
            assert abs(page.width / MM - 210) < .1 and abs(page.height / MM - 297) < .1
            outside = [char["text"] for char in page.chars if char["text"].strip() and (char["x0"] < 0 or char["x1"] > page.width + 1 or char["top"] < 0 or char["bottom"] > page.height + 1)]
            assert not outside, outside
        page = pdf.pages[1]
        grids = sorted(matrix.grids(page), key=lambda grid: grid[1][0])
        assert len(grids) == 2
        assert grids[0][1][-1] < grids[1][1][0]
        image_files = list((OUTPUT / "source/resources").glob("*.png"))
        assert len(image_files) == 16
        offsets, insets = [], []
        for xs, ys in grids:
            assert all(5.8 < (bottom - top) / MM < 6.7 for top, bottom in zip(ys[1:], ys[2:]))
            blank = page.crop((xs[0] + 1, ys[1] + 1, xs[-1] - 1, ys[-1] - 1))
            assert not blank.chars and not blank.images
            for column in range(1, 9):
                cell = (xs[column], ys[0], xs[column + 1], ys[1])
                images = [image for image in page.images if image["x0"] > cell[0] and image["x1"] < cell[2] and image["top"] > cell[1] and image["bottom"] < cell[3]]
                assert len(images) == 1
                image = images[0]
                dx = abs((image["x0"] + image["x1"] - cell[0] - cell[2]) / 2) / MM
                dy = abs((image["top"] + image["bottom"] - cell[1] - cell[3]) / 2) / MM
                assert dx < .3 and dy < .3, (dx, dy)
                inset = min(image["x0"] - cell[0], image["top"] - cell[1], cell[2] - image["x1"], cell[3] - image["bottom"]) / MM
                assert inset >= 1.8, inset
                offsets.append((dx, dy))
                insets.append(inset)
        capsules = lambda page: [image for image in page.images if image["srcsize"][0] == 2048]
        assert len(capsules(pdf.pages[1])) == len(capsules(pdf.pages[2])) == 16
        results[language] = {"pages": 3, "bytes": source.stat().st_size, "language": expected_language, "title": reader.metadata.title, "overfull": 0, "underfull": 0, "capsuleImages": 16, "maximumHeaderCenterOffsetMm": [max(row[axis] for row in offsets) for axis in (0, 1)], "minimumHeaderInsetMm": min(insets), "review": str(review)}
(OUTPUT / "native-qa.json").write_text(json.dumps(results, indent=2), encoding="utf-8")
print(json.dumps(results))
