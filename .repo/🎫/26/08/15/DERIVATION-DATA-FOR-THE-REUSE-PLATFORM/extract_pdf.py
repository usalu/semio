import pathlib
import re
import sys

import pdfplumber


ROOT = pathlib.Path(__file__).parent
sys.stdout.reconfigure(encoding="utf-8")
JOBS = [
    ("arcelor.pdf", [57, 58, 67, 68, 99, 100, 103, 104, 107, 108], r"IPE 200|HE 200 [ABM]|IPN 200|UPE 200|UPN 200"),
]

for filename, page_numbers, pattern in JOBS:
    print(f"=== {filename} ===")
    with pdfplumber.open(ROOT / filename) as pdf:
        for page_number in page_numbers:
            if page_number >= len(pdf.pages):
                continue
            page_text = pdf.pages[page_number].extract_text(layout=True) or ""
            hits = [line for line in page_text.splitlines() if re.search(pattern, line, re.IGNORECASE)]
            if hits:
                print(f"-- page {page_number + 1} --")
                print("\n".join(hits))
