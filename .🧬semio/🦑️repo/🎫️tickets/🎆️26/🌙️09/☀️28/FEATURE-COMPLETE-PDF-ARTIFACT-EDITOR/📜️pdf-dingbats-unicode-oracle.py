"""🔠 Extract Adobe Dingbats glyph text and verify the source font's ordinary Unicode probes."""
import json
from pathlib import Path

from fontTools import agl
from fontTools.pens.boundsPen import BoundsPen
from fontTools.ttLib import TTFont
import pymupdf

ticket=Path(__file__).resolve().parent
out=ticket/"🗑️generated"/"pdf-base14"
font=TTFont(out/"D050000L.otf")
glyphs=font.getGlyphSet()
rows=[]
for glyph in sorted(font.getGlyphOrder()):
    if not glyph.startswith("a") or not glyph[1:].isdigit():
        continue
    text=agl.toUnicode(glyph,isZapfDingbats=True)
    if text:
        assert len(text)==1,(glyph,text)
        rows.append({"name":glyph,"text":text})
assert len(rows)>=190, len(rows)
builtin=pymupdf.Font("ZapfDingbats")
checked=0
variations=[]
for row in rows:
    code=ord(row["text"])
    if not builtin.has_glyph(code,fallback=False):
        continue
    pen=BoundsPen(glyphs)
    glyphs[row["name"]].draw(pen)
    if pen.bounds is None:
        continue
    difference=max(abs(a / 1000 - b) for a,b in zip(pen.bounds,builtin.glyph_bbox(code)))
    if difference<0.002:
        checked+=1
    else:
        variations.append((row["name"],round(difference,6)))
assert checked>=180,(checked,variations)
assert all(name not in {"a1","a2","a202"} for name,_ in variations)
(out/"dingbats-map.json").write_text(json.dumps(rows,ensure_ascii=False)+"\n",encoding="utf8")
print(f"[DEBUG] Verified {checked} Zapf Dingbats Unicode characters against MuPDF and retained {len(rows)} Adobe glyph-name mappings.")

if variations:
    print("[DEBUG] MuPDF replacement variants with differing bounds",variations)
