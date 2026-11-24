"""🩹️ Replaces the exact-union part of the silhouette oracle (from `type Multi` to the end of the file) by the robust version in `exact-part.ts`; refuses when the head of the file is not the one the patch was written against.
python3 patch.py <oracle.ts> <exact-part.ts> <out.ts>"""
import sys

source = open(sys.argv[1], encoding="utf-8").read()
head, marker, _ = source.partition("type Multi = polygonClipping.MultiPolygon;\n")
if not marker:
    raise SystemExit("marker not found")
open(sys.argv[3], "w", encoding="utf-8").write(head + open(sys.argv[2], encoding="utf-8").read())
