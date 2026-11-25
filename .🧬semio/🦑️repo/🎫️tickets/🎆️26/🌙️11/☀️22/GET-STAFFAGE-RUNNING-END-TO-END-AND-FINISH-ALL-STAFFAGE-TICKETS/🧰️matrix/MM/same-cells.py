"""🟰️ Compares the cells of a record of the generator with a cells file of one.ts byte by byte: `python3 same-cells.py <cells/<asset>.json> <before.cells.json>`; prints the sizes, the SHA-256 of both and `identical` or `DIFFERENT`."""
import hashlib
import sys

record = open(sys.argv[1], "rb").read()
before = open(sys.argv[2], "rb").read()
cells = record[record.index(b'"cells":') + 8 : -1]
print(len(cells), hashlib.sha256(cells).hexdigest()[:16], len(before), hashlib.sha256(before).hexdigest()[:16], "identical" if cells == before else "DIFFERENT")
