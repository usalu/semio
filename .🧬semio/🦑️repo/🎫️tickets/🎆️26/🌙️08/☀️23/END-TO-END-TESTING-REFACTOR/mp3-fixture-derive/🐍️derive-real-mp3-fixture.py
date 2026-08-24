#!/usr/bin/env python3
"""🎵️ One-shot derivation of the real MPEG-1 Layer III fixture for `mutate-mp3-mpeg1-layer3`.

Run once; its output is committed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3`
(193,275 bytes, sha256 fe754e31d14d21474a22e3e4da87e5616c2393b8f6f9473a237451a86bc670a5).

Provenance, and why it is real rather than synthesised. The repository's only real recorded media is
`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4`, and `ffprobe`
confirms it carries NO audio stream — there is no soundtrack to extract. The sibling 🔊️wav case
already established the answer this repository accepts: use real measured data from that same real
camera-captured footage rather than shipping a synthetic tone. This script does the same thing at
the one sample rate MPEG-1 Layer III actually admits.

  1. 12 s of the REAL video is decoded to 8-bit grayscale at 25 fps, 42x42 — 25 * 42 * 42 = 44,100
     real per-pixel light-intensity measurements per second, which is exactly MPEG-1's 44.1 kHz.
     No resampling happens anywhere: the capture rate IS the sample rate.
  2. Each luma byte is centred (`(byte - 128) * 256`) into a signed 16-bit sample and written as
     canonical mono PCM with Python's standard-library `wave` module. No waveform math.
  3. `lame` (a real third-party MPEG-1 Layer III encoder, not this repository's code) encodes it at
     128 kbps CBR with a real ID3v2.3 tag and no ID3v1 trailer.

What the committed result actually contains, all verified in the oracle module's own tests:
  * a real 179-byte ID3v2.3.0 region LAME wrote — TSSE (its own encoder signature), TIT2 and TPE1
    in encoding 1 (UTF-16 with BOM, which is what a real-world encoder emits and what the previous
    handcrafted fixture never exercised), and TLEN;
  * 462 real MPEG-1 Layer III frames at 128 kbps / 44.1 kHz carrying real encoded audio, in which
    BOTH padding-slot values genuinely occur — 20 frames of 417 bytes and 442 of 418. The frame-size
    formula `144 * bitrate / rate + pad` is therefore exercised on both of its branches, which four
    frames of digital silence could not do;
  * no ID3v1 trailer, so `set-id3v1` remains an ADD and its inverse a genuine removal.
"""

import pathlib
import subprocess
import wave

REPO = pathlib.Path(__file__).resolve().parents[7]
SOURCE = REPO / "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🎥️bauen-mit-bestand.mp4"
TARGET = REPO / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🧫️fixtures/🎵️bauen-mit-bestand-ausschnitt.mp3"

SECONDS, FPS, SIDE = 12, 25, 42
RATE = FPS * SIDE * SIDE

here = pathlib.Path(__file__).parent
luma, pcm = here / "luma.raw", here / "luma.wav"

subprocess.run(
    ["ffmpeg", "-v", "error", "-y", "-i", str(SOURCE), "-t", str(SECONDS),
     "-vf", f"fps={FPS},scale={SIDE}:{SIDE},format=gray", "-f", "rawvideo", "-pix_fmt", "gray", str(luma)],
    check=True,
)

measured = luma.read_bytes()
assert len(measured) == RATE * SECONDS, f"{len(measured)} luma bytes, expected {RATE * SECONDS}"

with wave.open(str(pcm), "wb") as out:
    out.setnchannels(1)
    out.setsampwidth(2)
    out.setframerate(RATE)
    out.writeframes(b"".join(int.to_bytes((byte - 128) * 256, 2, "little", signed=True) for byte in measured))

subprocess.run(
    ["lame", "--quiet", "-m", "m", "-b", "128", "--cbr", "--id3v2-only",
     "--tt", "Bauen mit Bestand (Ausschnitt)", "--ta", "semio", str(pcm), str(TARGET)],
    check=True,
)

print(f"{TARGET} — {TARGET.stat().st_size} bytes at {RATE} Hz from {SECONDS}s of the real video")
