"""📇 Validate the bounded predictor fixtures against MuPDF."""

import json
from pathlib import Path
import sys
import zlib

import fitz


def verify(cases):
    for case in cases:
        source = bytes.fromhex(case["encoded"])
        assert len(zlib.decompress(source)) == case["inflatedLength"], case["name"]
        with fitz.open() as document:
            document.new_page()
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            document.update_stream(stream, source, compress=False)
            document.xref_set_key(stream, "Filter", "/FlateDecode")
            parameters = " ".join(
                f"/{name} {case[key]}"
                for name, key in (
                    ("Predictor", "predictor"),
                    ("Columns", "columns"),
                    ("Colors", "colors"),
                    ("BitsPerComponent", "bits"),
                )
            )
            document.xref_set_key(stream, "DecodeParms", f"<< {parameters} >>")
            expected = bytes.fromhex(case["decoded"])
            actual = document.xref_stream(stream)
            assert actual == expected, (case["name"], actual.hex(), expected.hex())
    print(f"MuPDF matched {len(cases)} PNG and TIFF predictor fixtures")


if __name__ == "__main__":
    verify(json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")))
