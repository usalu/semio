"""🌊 Independently validates the shared PDF Flate pipeline fixture."""

import json
from pathlib import Path
import sys
import zlib

import fitz


def verify(cases):
    for case in cases:
        source = bytes.fromhex(case["hex"])
        framing = case["framing"]
        if framing in {"raw", "rawTrailing"}:
            decoded = zlib.decompress(source, -zlib.MAX_WBITS)
        elif framing == "damaged":
            decoded = zlib.decompress(source[2:-4], -zlib.MAX_WBITS)
        else:
            decoded = zlib.decompress(source.lstrip(b"\r\n "))
        assert decoded == bytes.fromhex(case["inflated"]), case
        if case["predictor"] != 1:
            with fitz.open() as doc:
                doc.new_page()
                stream = doc.get_new_xref()
                doc.update_object(stream, "<<>>")
                doc.update_stream(stream, source, compress=False)
                doc.xref_set_key(stream, "Filter", "/FlateDecode")
                doc.xref_set_key(stream, "DecodeParms", f"<< /Predictor {case['predictor']} /Columns 3 >>")
                assert doc.xref_stream(stream) == bytes.fromhex(case["decoded"]), case
        else:
            assert decoded == bytes.fromhex(case["decoded"]), case
    print(f"zlib matched {len(cases)} frame cases and MuPDF matched both predictor outputs")


if __name__ == "__main__":
    verify(json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")))
