"""📚 Validate the PDF Flate corpus and checksum failures with Python's zlib."""

import json
from pathlib import Path
import sys
import zlib


def verify(fixture):
    block_types = {"stored": 0, "fixed": 1, "dynamic": 2}
    for case in fixture:
        stream = bytes.fromhex(case["hex"])
        assert ((stream[2] >> 1) & 3) == block_types[case["kind"]]
        assert zlib.decompress(stream) == (case["unit"] * case["repeat"]).encode("ascii")
        corrupt = stream[:-1] + bytes([stream[-1] ^ 1])
        try:
            zlib.decompress(corrupt)
        except zlib.error:
            continue
        raise AssertionError(f"accepted corrupt {case['kind']} Adler-32 checksum")
    print(f"{len(fixture)} zlib block modes match declared output and reject corrupt checksums")


if __name__ == "__main__":
    verify(json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")))
