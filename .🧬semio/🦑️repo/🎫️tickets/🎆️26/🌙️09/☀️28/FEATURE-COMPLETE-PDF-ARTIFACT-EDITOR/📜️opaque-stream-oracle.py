import fitz
with fitz.open() as doc:
    page=doc.new_page()
    n=doc.get_new_xref()
    doc.update_object(n,"<<>>")
    doc.update_stream(n,b"abc")
    doc.xref_set_key(n,"Filter","/VendorFilter")
    doc.xref_set_key(n,"DecodeParms","<< /Mode 2 >>")
    doc.xref_set_key(page.xref,"PieceInfo",f"<< /Vendor << /Private {n} 0 R >> >>")
    doc.set_metadata({"title":"edited"})
    with fitz.open(stream=doc.tobytes(),filetype="pdf") as edited:
        assert edited.xref_get_key(n,"Filter")==("name","/VendorFilter")
        assert edited.xref_get_key(n,"DecodeParms/Mode")==("int","2")
        assert edited.xref_stream_raw(n)==b"abc"
print("MuPDF preserves unknown filter, parameters and raw bytes across an unrelated edit")
