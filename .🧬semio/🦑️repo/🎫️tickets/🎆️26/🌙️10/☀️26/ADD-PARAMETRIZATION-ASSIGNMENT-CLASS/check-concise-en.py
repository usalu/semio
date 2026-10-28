from pathlib import Path
import json
import sys
import pypdfium2 as pdfium

source = Path(sys.argv[1])
document = pdfium.PdfDocument(str(source))
assert len(document) == 3, len(document)
texts = []
for page in document:
    textpage = page.get_textpage()
    texts.append(" ".join(textpage.get_text_range().split()))
    textpage.close()
    page.close()
document.close()
assert "Finger Exercise: Parametrization" in texts[0]
assert "Example 1.1" in texts[0]
assert "Dwelling Capsules 1.2" in texts[1]
assert "Table A" in texts[1] and "Table B" in texts[1]
assert "20 marked pairs" in texts[2] and "16 ring edges" in texts[2]
assert "lower to the higher" in texts[2] and "reverse all signs" in texts[2]
assert all(token in texts[0] for token in ["Unique.", "Consistent.", "Minimal."])
assert "1.3" not in "\n".join(texts)
assert "45" not in texts[0] and "submission" not in texts[0].lower()
print(json.dumps({"pages": 3, "text_characters": [len(text) for text in texts], "checks": "English page structure and exercise contract passed"}))
