"""🖼️ One-shot derivation of the real complex `s.stdio.semio.image` artifact and its per-kind
mutation payloads for `mutate-semio-image`.

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 16.

PROVENANCE. The source is the real committed animated GIF
`🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif` — 194x84, GIF89a, 16 frames,
130 ms per frame, NETSCAPE2.0 loop extension, palette-indexed. Pillow (PIL 11.3) decodes it; the
first three frames are taken at NATIVE resolution with no resampling, no cropping and no colour
conversion beyond the palette resolution to RGBA8 that the semio image model requires. The declared
`colorspace` is `indexed` and the `bitDepth` 8 because that is what the source genuinely is, the
delays are the file's own, and every metadata entry states a real fact about the source file. The
`set-icc` payload is a real sRGB profile emitted by littleCMS through `PIL.ImageCms`.

The DSL and pack files are written by the case's own independent Python implementation
(`🐍️component.py`), which was first checked to reproduce the committed swatch artifact byte for byte
in both encodings. The Rust subject then has to reproduce these same two files from its own reading
of the same grammar, which is what makes `identity-round-trip` a cross-implementation byte agreement
rather than a codec agreeing with itself.
"""

import importlib.util
import json
import os

from PIL import Image, ImageCms, ImageSequence

spec = importlib.util.spec_from_file_location("loader", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️load.py"))
loader = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loader)
img = loader.load("mutate-semio-image")

SOURCE = loader.REPO + "/🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif"
CASE = loader.REPO + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-image/🧫️fixtures"

frames = []
delays = []
source = Image.open(SOURCE)
width, height = source.size
for frame in ImageSequence.Iterator(source):
    frames.append(list(frame.convert("RGBA").tobytes()))
    delays.append(int(frame.info.get("duration", 0)))

artifact = {
    "schema": "s.stdio.semio.image",
    "width": width,
    "height": height,
    "colorspace": "indexed",
    "bitDepth": 8,
    "frames": [{"delayMs": delays[index], "rgba8": frames[index]} for index in range(3)],
    "icc": None,
    "metadata": [
        {"key": "source.file", "value": "🧰️framework/🔨️modules/🖼️assets/🖼️images/🖼️color-animated-text.gif"},
        {"key": "source.format", "value": "GIF89a"},
        {"key": "source.frames", "value": str(len(frames))},
        {"key": "source.loopCount", "value": "0"},
        {"key": "Beschreibung", "value": "Farbig animierter Text – 194×84 Pixel"},
    ],
}

srgb = ImageCms.ImageCmsProfile(ImageCms.createProfile("sRGB")).tobytes()

payloads = {
    "no-mutation": {"mutation": "noMutation"},
    "set-snapshot": {
        "mutation": "setSnapshot",
        "snapshot": {
            "schema": "s.stdio.semio.image",
            "width": width,
            "height": height,
            "colorspace": "rgba",
            "bitDepth": 16,
            "frames": [{"delayMs": delays[index], "rgba8": frames[index]} for index in (8, 9, 10)],
            "icc": list(srgb),
            "metadata": [{"key": "source.file", "value": "🖼️color-animated-text.gif"}, {"key": "source.slice", "value": "frames 8-10"}],
        },
    },
    "set-dimensions": {"mutation": "setDimensions", "width": height, "height": width},
    "set-colorspace": {"mutation": "setColorspace", "colorspace": "rgba"},
    "set-bit-depth": {"mutation": "setBitDepth", "bit_depth": 4},
    "set-icc": {"mutation": "setIcc", "icc": list(srgb)},
    "insert-frame": {"mutation": "insertFrame", "index": 1, "frame": {"delayMs": 420, "rgba8": frames[12]}},
    "remove-frame": {"mutation": "removeFrame", "index": 1},
    "move-frame": {"mutation": "moveFrame", "from": 2, "to": 0},
    "set-frame-delay": {"mutation": "setFrameDelay", "index": 1, "delay_ms": 640},
    "set-frame-pixels": {"mutation": "setFramePixels", "index": 2, "rgba8": frames[15]},
    "set-metadata-entry": {"mutation": "setMetadataEntry", "key": "source.format", "value": "GIF89a (NETSCAPE2.0)"},
    "remove-metadata-entry": {"mutation": "removeMetadataEntry", "key": "Beschreibung"},
}

os.makedirs(CASE, exist_ok=True)
dsl = img.print_dsl(artifact).encode("utf-8")
pack = img.pack_bytes(artifact)
assert img.parse_dsl(dsl.decode("utf-8")) == artifact, "the derived DSL does not read back as the document it was written from"
assert img.parse_pack(pack) == artifact, "the derived pack does not read back as the document it was written from"
with open(os.path.join(CASE, "🗣️artifact.dsl.semio"), "wb") as handle:
    handle.write(dsl)
with open(os.path.join(CASE, "🎒️artifact.pack.semio"), "wb") as handle:
    handle.write(pack)
for kind, payload in payloads.items():
    with open(os.path.join(CASE, "🦠️%s.json" % kind), "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, separators=(",", ":"))
        handle.write("\n")

for kind, payload in payloads.items():
    applied = img.apply_mutation(artifact, payload)
    undone = img.apply_mutation(applied, img.inverse_mutation(artifact, payload))
    assert undone == artifact, "%s: the independent inverse does not restore the derived artifact" % kind
    print("%-24s applied ok, inverse restores" % kind)

print("dsl bytes", len(dsl), "pack bytes", len(pack))
print("raster", json.dumps(img.pillow_report(artifact))[:400])
