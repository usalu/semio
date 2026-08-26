"""🎵️ One-off derivation (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22).

Reads two REAL committed recordings of the SAME real source — the "Bauen mit Bestand" presentation
excerpt — and maps them onto the `s.stdio.semio.audio` model:

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🧫️fixtures/🔊️bauen-mit-bestand-ausschnitt.wav`, read with
  Python's own stdlib `wave` module, supplies the real audio: 8 000 Hz, 16-bit PCM, one channel, of
  which the first real second (8 000 real frames) becomes the channel's samples, each real signed
  16-bit value scaled by 2⁻¹⁵ — an exact binary32 conversion, no resampling and no filtering. The
  declared `format` is the file's own `pcm16`, a format arm no committed example carried.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3` supplies the
  real metadata: its real ID3v2.3 `TSSE`/`TIT2`/`TPE1`/`TLEN` frames, decoded here by a
  purpose-written ID3v2 frame reader, become the four tags in the file's own frame order.

Nothing is invented: every sample is a real sample of the real recording and every tag is a real
frame of the real file. Neither reader speaks a semio envelope, which is why they are the source of
the ARTIFACT and never the oracle.
"""

import importlib.util
import struct
import sys
import wave
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WAV = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🧫️fixtures/🔊️bauen-mit-bestand-ausschnitt.wav"
MP3 = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3"
CASE = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-audio"
FRAMES = 8000


def load_oracle():
    stub = type(sys)("semio_repo_test")
    stub.Adapter = type("Adapter", (), {"__init__": lambda self, name: None, "oracle": lambda self, *a: self})
    stub.Context = object
    stub.Outcome = object
    stub.digest = lambda data: ""
    sys.modules["semio_repo_test"] = stub
    spec = importlib.util.spec_from_file_location("audio_oracle", CASE / "🐍️component.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def id3_text(payload):
    """🏷️ One real ID3v2.3 text frame, by its own encoding byte (0 = ISO-8859-1, 1 = UTF-16)."""
    encoding, body = payload[0], payload[1:]
    text = body.decode("utf-16" if encoding == 1 else "latin-1", errors="replace")
    return text.rstrip("\x00")


def id3_tags(path):
    data = path.read_bytes()
    assert data[:3] == b"ID3", "the committed mp3 must carry a real ID3v2 tag"
    size = (data[6] & 0x7F) << 21 | (data[7] & 0x7F) << 14 | (data[8] & 0x7F) << 7 | (data[9] & 0x7F)
    tags, at = [], 10
    while at < 10 + size - 10:
        frame_id = data[at : at + 4]
        if frame_id == b"\x00\x00\x00\x00":
            break
        length = int.from_bytes(data[at + 4 : at + 8], "big")
        tags.append({"key": frame_id.decode("ascii"), "value": id3_text(data[at + 10 : at + 10 + length])})
        at += 10 + length
    return tags


def main():
    with wave.open(str(WAV)) as reader:
        assert reader.getsampwidth() == 2, "the committed wav is 16-bit PCM"
        channels, rate = reader.getnchannels(), reader.getframerate()
        raw = reader.readframes(FRAMES)
    values = struct.unpack("<%dh" % (len(raw) // 2), raw)
    lanes = [[float(values[at]) / 32768.0 for at in range(lane, len(values), channels)] for lane in range(channels)]

    document = {
        "schema": "stdio.semio.audio",
        "sampleRate": rate,
        "format": "pcm16",
        "channels": [{"samples": lane} for lane in lanes],
        "tags": id3_tags(MP3),
    }
    oracle = load_oracle()
    dsl = oracle.print_dsl(document)
    assert oracle.parse_dsl(dsl) == document, "the printed DSL does not re-parse to the same document"
    out = CASE / "🧫️fixtures"
    out.mkdir(exist_ok=True)
    (out / "🗣️bauen-mit-bestand-ausschnitt.dsl.semio").write_text(dsl, encoding="utf-8")
    print("sampleRate=%d format=%s channels=%d samples=%d tags=%d" % (document["sampleRate"], document["format"], len(document["channels"]), sum(len(c["samples"]) for c in document["channels"]), len(document["tags"])))
    print("tags:", [(t["key"], t["value"][:40]) for t in document["tags"]])
    print("dsl bytes=%d" % len(dsl.encode("utf-8")))
    print("first samples:", lanes[0][:6])


if __name__ == "__main__":
    main()
