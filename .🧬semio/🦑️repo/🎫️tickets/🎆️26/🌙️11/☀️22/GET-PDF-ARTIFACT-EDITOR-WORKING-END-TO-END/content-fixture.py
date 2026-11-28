"""🧺️ Writes `content.pdf`, a two-page PDF fixture authored by pymupdf for the content and page action sweep: text in two fonts, a red rectangle, a blue line and a PNG-like image on page one, text on page two.

usage: uv run --offline --with pymupdf==1.26.7 python -I content-fixture.py <out dir>
"""
import pathlib
import sys

import pymupdf

folder = pathlib.Path(sys.argv[1])
document = pymupdf.open()
document.new_page(width=612, height=792)
document.new_page(width=612, height=792)
one, two = document[0], document[1]
one.insert_text((72, 72), "Rich Fixture Helvetica", fontname="helv", fontsize=18)
one.insert_text((72, 110), "Second font Times", fontname="tiro", fontsize=12)
one.draw_rect(pymupdf.Rect(72, 150, 272, 250), color=(0, 0, 0), fill=(1, 0, 0), width=2)
one.draw_line((72, 300), (400, 300), color=(0, 0, 1), width=3)
pixmap = pymupdf.Pixmap(pymupdf.csRGB, pymupdf.IRect(0, 0, 8, 8), False)
pixmap.set_rect(pixmap.irect, (0, 128, 255))
one.insert_image(pymupdf.Rect(400, 150, 480, 230), pixmap=pixmap)
two.insert_text((72, 72), "Page two body", fontname="helv", fontsize=14)
document.set_metadata({"title": "Content Fixture"})
target = folder / "content.pdf"
document.save(str(target), garbage=1, deflate=False)
print(f"ok {target}")
