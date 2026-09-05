#!/usr/bin/env python3
"""📷️ Pillow-backed PNG oracle for lowpoly's production byte stream.

The Rust subject supplies `serialize_bytes()` output through the feature's
`@oracle-input-subject-raw` contract. This adapter opens those exact bytes with Pillow; it neither
constructs a substitute PNG nor predicts lowpoly's DSL text.
"""

import io

from PIL import Image

from semio_repo_test import Adapter, Outcome


LOWPOLY_DSL_TEXT_KEYWORD = "semio-lowpoly-dsl"


def roundtrip_png(ctx):
    """🔮️ Decodes the production PNG with Pillow and exposes its independently observed raster facts."""
    raw = ctx.subject_raw_bytes("rust")
    source = io.BytesIO(raw)
    image = Image.open(source)
    image.verify()
    source.seek(0)
    image = Image.open(source)
    image.load()
    if image.format != "PNG":
        raise AssertionError("roundtrip-png: Pillow reported %r, not PNG" % image.format)
    if image.mode != "RGBA":
        raise AssertionError("roundtrip-png: Pillow decoded pixel format %r, not RGBA" % image.mode)
    pixels = image.tobytes()
    expected_length = image.width * image.height * 4
    if len(pixels) != expected_length:
        raise AssertionError("roundtrip-png: Pillow decoded %d RGBA bytes for %dx%d pixels" % (len(pixels), image.width, image.height))
    if LOWPOLY_DSL_TEXT_KEYWORD not in image.text:
        raise AssertionError("roundtrip-png: Pillow found no %r tEXt chunk" % LOWPOLY_DSL_TEXT_KEYWORD)
    return Outcome(
        {
            "width": image.width,
            "height": image.height,
            "pixelFormat": image.mode,
            "pixels": list(pixels),
            "textChunkKeyword": LOWPOLY_DSL_TEXT_KEYWORD,
        },
        raw=raw,
    )


def adapter():
    """🧭️ Registers Pillow only as the external oracle for the split PNG scenario."""
    return Adapter("python").oracle("roundtrip-png", roundtrip_png)
