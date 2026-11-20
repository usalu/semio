import fitz,sys
with fitz.open(sys.argv[1]) as pdf:
    assert not pdf.is_repaired
    assert pdf.xref_stream(4)==b"0 0 m 1 1 l S\n"
    assert pdf.xref_stream(5)==bytes((255,255,255,3,4,5))
    assert pdf.xref_stream(6)==bytes((0,0,0,0,20,30,40))
print("MuPDF matched ASCIIHex, ASCII85, and RunLength stream output")
