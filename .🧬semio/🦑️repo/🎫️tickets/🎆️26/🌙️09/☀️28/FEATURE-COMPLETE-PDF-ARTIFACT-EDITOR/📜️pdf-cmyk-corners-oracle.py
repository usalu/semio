"""🧪️ Measures the MuPDF DeviceCMYK to sRGB result at the 16 corners of the CMYK hypercube."""

import fitz

cells = [[(i >> 3) & 1, (i >> 2) & 1, (i >> 1) & 1, i & 1] for i in range(16)]
content = "".join(f"{c} {m} {y} {k} k {col * 10} 0 10 10 re f\n" for (c, m, y, k), col in zip(cells, range(16)))
with fitz.open() as document:
    page = document.new_page(width=160, height=10)
    page.insert_text((0, 0), "")
    xref = document.get_new_xref()
    document.update_object(xref, "<<>>")
    document.update_stream(xref, content.encode())
    page.set_contents(xref)
    pixmap = page.get_pixmap(colorspace=fitz.csRGB)
    print([list(pixmap.pixel(col * 10 + 5, 5)) for col in range(16)])
