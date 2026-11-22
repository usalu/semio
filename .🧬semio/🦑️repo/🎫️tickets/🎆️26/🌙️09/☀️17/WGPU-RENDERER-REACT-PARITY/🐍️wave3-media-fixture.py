#!/usr/bin/env python3
"""Generates the language-agnostic WAV fixture of the native MediaTransportHost (WAV decode, G.711 tables).

The expectations come from Python's standard `wave` and `audioop` modules (third-party validation of the in-repo decoder). The transport
rules are pinned by the window kit's own fixture `🔌️plugin/🪟️window-kits/🎬️media/🧪️tests/⏱️time-and-keys/🧫️fixture.json`, which both twins read.
"""
import audioop, io, json, struct, sys, wave

def riff(chunks):
    body = b"WAVE" + b"".join(chunks)
    return b"RIFF" + struct.pack("<I", len(body)) + body

def chunk(kind, body):
    return kind + struct.pack("<I", len(body)) + body + (b"\0" if len(body) % 2 else b"")

def fmt(tag, channels, rate, bits, extensible=False):
    block = channels * bits // 8
    body = struct.pack("<HHIIHH", 0xFFFE if extensible else tag, channels, rate, rate * block, block, bits)
    if extensible:
        body += struct.pack("<HHI", 22, bits, 3) + struct.pack("<H", tag) + bytes.fromhex("000000001000800000aa00389b71")
    return chunk(b"fmt ", body)

def wav_case(case_id, data, tag, channels, rate, bits, expected, extensible=False, extra=()):
    return {"id": case_id, "hex": riff([fmt(tag, channels, rate, bits, extensible), *extra, chunk(b"data", data)]).hex(), "sampleRate": rate, "channels": channels, "samples": expected}

cases = []
pcm16 = [0, 1000, -1000, 32767, -32768, 256]
buffer = io.BytesIO()
with wave.open(buffer, "wb") as writer:
    writer.setnchannels(1); writer.setsampwidth(2); writer.setframerate(8000); writer.writeframes(struct.pack("<6h", *pcm16))
cases.append({"id": "pcm16-mono-from-python-wave", "hex": buffer.getvalue().hex(), "sampleRate": 8000, "channels": 1, "samples": [v / 32768 for v in pcm16]})
with wave.open(io.BytesIO(buffer.getvalue()), "rb") as reader:
    assert list(struct.unpack("<6h", reader.readframes(6))) == pcm16
cases.append(wav_case("pcm8-stereo", bytes([0, 128, 255, 64]), 1, 2, 11025, 8, [(b - 128) / 128 for b in (0, 128, 255, 64)]))
cases.append(wav_case("pcm24-mono", b"".join(struct.pack("<i", v << 8)[1:] for v in (0, 4194304, -4194304, 8388607, -8388608)), 1, 1, 44100, 24, [v / 8388608 for v in (0, 4194304, -4194304, 8388607, -8388608)]))
cases.append(wav_case("pcm32-mono", struct.pack("<4i", 0, 1073741824, -1073741824, -2147483648), 1, 1, 48000, 32, [0.0, 0.5, -0.5, -1.0]))
cases.append(wav_case("float32-stereo", struct.pack("<4f", 0.0, 0.25, -0.5, 1.0), 3, 2, 22050, 32, [0.0, 0.25, -0.5, 1.0]))
cases.append(wav_case("float64-mono", struct.pack("<3d", 0.125, -0.75, 0.5), 3, 1, 96000, 64, [0.125, -0.75, 0.5]))
cases.append(wav_case("extensible-pcm16-stereo", struct.pack("<4h", 100, -100, 200, -200), 1, 2, 16000, 16, [v / 32768 for v in (100, -100, 200, -200)], extensible=True))
cases.append(wav_case("extra-chunks-and-padding", struct.pack("<3h", 5, 6, 7), 1, 1, 8000, 16, [v / 32768 for v in (5, 6, 7)], extra=(chunk(b"LIST", b"odd"), chunk(b"fact", struct.pack("<I", 3)))))

everything = bytes(range(256))
g711 = {"alaw": list(struct.unpack("<256h", audioop.alaw2lin(everything, 2))), "mulaw": list(struct.unpack("<256h", audioop.ulaw2lin(everything, 2)))}

refusals = [
    {"id": "not-riff", "hex": b"RIFXxxxxWAVE".hex(), "code": "notWav"},
    {"id": "truncated-format", "hex": (b"RIFF" + struct.pack("<I", 8) + b"WAVEfmt " + struct.pack("<I", 16) + b"\x01\x00").hex(), "code": "truncated"},
    {"id": "no-data", "hex": riff([fmt(1, 1, 8000, 16)]).hex(), "code": "malformed"},
    {"id": "mp3-in-wave", "hex": riff([fmt(85, 1, 8000, 16), chunk(b"data", b"\0\0")]).hex(), "code": "unsupported"},
    {"id": "block-align-mismatch", "hex": riff([chunk(b"fmt ", struct.pack("<HHIIHH", 1, 2, 8000, 32000, 2, 16)), chunk(b"data", b"\0\0\0\0")]).hex(), "code": "malformed"},
]

json.dump({"schema": "semio.media.native-wav.v1", "wav": cases, "wavRefusals": refusals, "g711": g711}, sys.stdout, ensure_ascii=False, indent=1)
