"""🖼️ Rasterizes the pages of the new reports that carry the Anlage-8 obligations."""

import sys

import fitz

TICKET = r"E:\semio\.repo\🎫\26\08\12\ABSCHLUSSBERICHTE-FORSCHUNGSBERICHT-UND-KOMPAKTBERICHT-AUFSETZEN"
JOBS = [
    (r"E:\semio\mit-bestand\bericht\forschungsbericht\dist\forschungsbericht.pdf", "fb", [0, 1, 2, 11]),
    (r"E:\semio\mit-bestand\bericht\kompaktbericht\dist\kompaktbericht.pdf", "kb", [0, 1, 2]),
]

for path, tag, pages in JOBS:
    doc = fitz.open(path)
    sys.stdout.buffer.write(f"{tag}: {doc.page_count} pages\n".encode("utf-8"))
    for index in pages:
        if index >= doc.page_count:
            continue
        doc[index].get_pixmap(dpi=100).save(f"{TICKET}\\{tag}-p{index + 1}.png")
        text = " | ".join(line.strip() for line in doc[index].get_text().splitlines() if line.strip())
        sys.stdout.buffer.write(f"--- {tag} p{index + 1}: {text[:300]}\n".encode("utf-8"))
