import sys
try:
    from pypdf import PdfReader
except ImportError:
    from PyPDF2 import PdfReader
for p in sys.argv[1:]:
    r = PdfReader(p)
    print("=====", p)
    for page in r.pages:
        print(page.extract_text())
