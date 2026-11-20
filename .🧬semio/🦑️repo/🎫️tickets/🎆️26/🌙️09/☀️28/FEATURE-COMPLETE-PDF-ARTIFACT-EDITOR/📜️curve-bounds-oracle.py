import fitz
doc=fitz.open()
page=doc.new_page(width=12,height=9)
stream=doc.get_new_xref()
doc.update_object(stream,"<<>>")
doc.update_stream(stream,b"1 0 0 rg 1 1 m 1 10 7 1 10 1 c h f")
page.set_contents(stream)
pix=page.get_pixmap(matrix=fitz.Matrix(64,64))
rows=[row for row in range(pix.height) if min(pix.samples_mv[row*pix.stride+1:(row+1)*pix.stride:3])<220]
height=(max(rows)-min(rows)+1)/64
assert abs(height-4)<=1/64,height
print("MuPDF raster bounds:",height)
