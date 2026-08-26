"""🎥️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads two REAL committed recordings of the SAME real source — the "Bauen mit Bestand" presentation
excerpt — and maps them onto the `s.stdio.semio.video` two-stream model:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi` supplies the `V`
  stream, read with a purpose-written RIFF/AVI reader: its real `strh` four-cc `MJPG`, its real
  `avih` frame size 480×432, its real 1/15 scale-rate pair, and the first eight real `00dc` frame
  chunks of the real `movi` list as samples, each carrying its real JPEG bytes and a real
  presentation timestamp counted in the file's own frame ticks. Every MJPEG frame is a key frame by
  construction of the codec, which is what the `key` flag records.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3` supplies the
  `A` stream, read with a purpose-written MPEG-1 Layer III frame reader: its real sample rate as the
  stream rate, and its first twenty-four real audio frames as samples, each carrying its real frame
  bytes and a real timestamp counted in the layer's own 1 152-sample granule.

Nothing is invented: every byte, timestamp and flag comes out of one of the two real files. Neither
reader speaks a semio envelope, which is why they are the source of the ARTIFACT and never the
oracle.
"""

import importlib.util
import struct
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
AVI = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🧫️fixtures/📼️bauen-mit-bestand-mjpeg.avi"
MP3 = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-video"
VIDEO_FRAMES = 8
AUDIO_FRAMES = 24

BITRATES = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 0]
RATES = [44100, 48000, 32000, 0]


def load_oracle():
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("video_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def video_stream():
    data = AVI.read_bytes()
    at = data.find(b"avih")
    header = data[at + 8 : at + 8 + struct.unpack("<I", data[at + 4 : at + 8])[0]]
    width, height = struct.unpack("<II", header[32:40])
    at = data.find(b"strh")
    stream_header = data[at + 8 : at + 8 + struct.unpack("<I", data[at + 4 : at + 8])[0]]
    codec = stream_header[4:8].decode("latin-1")
    scale, rate = struct.unpack("<II", stream_header[20:28])
    movi = data.find(b"movi") + 4
    samples, at, index = [], movi, 0
    while index < VIDEO_FRAMES:
        chunk = data[at : at + 4]
        if chunk[:2] != b"00":
            break
        size = struct.unpack("<I", data[at + 4 : at + 8])[0]
        samples.append({"pts": index * scale, "key": True, "data": data[at + 8 : at + 8 + size].hex()})
        at += 8 + size + (size & 1)
        index += 1
    return {"kind": "V", "codec": codec, "width": width, "height": height, "rate": {"num": rate, "den": scale}, "samples": samples}


def audio_stream():
    data = MP3.read_bytes()
    at = 0
    if data[:3] == b"ID3":
        at = 10 + ((data[6] & 0x7F) << 21 | (data[7] & 0x7F) << 14 | (data[8] & 0x7F) << 7 | (data[9] & 0x7F))
    samples, index, sample_rate = [], 0, 0
    while index < AUDIO_FRAMES and at + 4 <= len(data):
        if data[at] != 0xFF or (data[at + 1] & 0xE0) != 0xE0:
            at += 1
            continue
        bitrate = BITRATES[data[at + 2] >> 4]
        sample_rate = RATES[(data[at + 2] >> 2) & 0x03]
        padding = (data[at + 2] >> 1) & 0x01
        if bitrate == 0 or sample_rate == 0:
            at += 1
            continue
        length = (144 * bitrate * 1000) // sample_rate + padding
        samples.append({"pts": index * 1152, "key": True, "data": data[at : at + length].hex()})
        at += length
        index += 1
    return {"kind": "A", "codec": "mp3", "width": 0, "height": 0, "rate": {"num": sample_rate, "den": 1152}, "samples": samples}


def main():
    document = {"schema": "stdio.semio.video", "streams": [video_stream(), audio_stream()]}
    oracle = load_oracle()
    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️bauen-mit-bestand-ausschnitt.dsl.semio").write_text(dsl, encoding="utf-8")
    for stream in document["streams"]:
        print("stream", stream["kind"], stream["codec"], stream["width"], stream["height"], stream["rate"], "samples", len(stream["samples"]))
        print("   pts", [s["pts"] for s in stream["samples"][:5]], "bytes", [len(s["data"]) // 2 for s in stream["samples"][:5]])
    print("dsl bytes=%d" % len(dsl.encode("utf-8")))


if __name__ == "__main__":
    main()
