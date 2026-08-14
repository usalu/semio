#!/usr/bin/env python3
"""Handcraft a small MP3 file: ID3v2 tag prefix + N valid MPEG-1 Layer III
frame headers with correctly-sized silent payload, byte-accurate per the
MP3 frame header spec (ISO/IEC 11172-3)."""
import struct
import sys

OUT = sys.argv[1]

# ---- MPEG-1 Layer III frame header field tables ----
# Bitrate index table for MPEG version 1, Layer III (kbps), index 0..15
# (index 0 = "free", 15 = "bad" -- we never use those)
BITRATE_KBPS_V1_L3 = [
    None, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, None
]
SAMPLE_RATE_V1 = [44100, 48000, 32000, None]  # index -> Hz (MPEG-1)

MPEG_VERSION_1 = 0b11      # 2 bits: 11 = MPEG Version 1
LAYER_III = 0b01           # 2 bits: 01 = Layer III
PROTECTION_ABSENT = 1      # 1 bit: 1 = no CRC
BITRATE_INDEX = 9          # -> 128 kbps
SAMPLING_INDEX = 0         # -> 44100 Hz
PADDING = 0
PRIVATE = 0
CHANNEL_MODE = 0b11        # 11 = mono
MODE_EXTENSION = 0
COPYRIGHT = 0
ORIGINAL = 1
EMPHASIS = 0

def frame_size(bitrate_kbps: int, sample_rate: int, padding: int) -> int:
    # Layer III (Layer 1 uses a different formula): FrameSize = 144 * BitRate / SampleRate + Padding
    return (144 * bitrate_kbps * 1000) // sample_rate + padding

def build_header(bitrate_index: int, sampling_index: int, padding: int) -> bytes:
    b0 = 0xFF  # sync byte 1 (all 8 bits of the 11-bit sync)
    # byte 1: sync(3 remaining bits)=111, version(2)=11, layer(2)=01, protection(1)=1
    b1 = 0b111_00000
    b1 |= (MPEG_VERSION_1 & 0b11) << 3
    b1 |= (LAYER_III & 0b11) << 1
    b1 |= (PROTECTION_ABSENT & 0b1)
    # byte 2: bitrate_index(4), sampling_index(2), padding(1), private(1)
    b2 = (bitrate_index & 0xF) << 4
    b2 |= (sampling_index & 0b11) << 2
    b2 |= (padding & 0b1) << 1
    b2 |= (PRIVATE & 0b1)
    # byte 3: channel_mode(2), mode_extension(2), copyright(1), original(1), emphasis(2)
    b3 = (CHANNEL_MODE & 0b11) << 6
    b3 |= (MODE_EXTENSION & 0b11) << 4
    b3 |= (COPYRIGHT & 0b1) << 3
    b3 |= (ORIGINAL & 0b1) << 2
    b3 |= (EMPHASIS & 0b11)
    return bytes([b0, b1, b2, b3])

def build_id3v2(title: str, artist: str) -> bytes:
    """Minimal real ID3v2.3 tag: header + one TIT2 + one TPE1 text frame."""
    def text_frame(frame_id: bytes, text: str) -> bytes:
        payload = b"\x00" + text.encode("latin-1")  # encoding byte 0 = ISO-8859-1
        return frame_id + struct.pack(">I", len(payload)) + b"\x00\x00" + payload

    frames = text_frame(b"TIT2", title) + text_frame(b"TPE1", artist)
    size = len(frames)
    # synchsafe size: 4 bytes, 7 bits each
    synchsafe = bytes([
        (size >> 21) & 0x7F,
        (size >> 14) & 0x7F,
        (size >> 7) & 0x7F,
        size & 0x7F,
    ])
    header = b"ID3" + bytes([3, 0]) + bytes([0]) + synchsafe  # v2.3.0, flags=0
    return header + frames

def main():
    sample_rate = SAMPLE_RATE_V1[SAMPLING_INDEX]
    bitrate_kbps = BITRATE_KBPS_V1_L3[BITRATE_INDEX]
    fsize = frame_size(bitrate_kbps, sample_rate, PADDING)

    id3 = build_id3v2("semio fixture", "W0 handcraft")

    frames = []
    num_frames = 4
    for i in range(num_frames):
        header = build_header(BITRATE_INDEX, SAMPLING_INDEX, PADDING)
        payload_len = fsize - len(header)
        assert payload_len > 0
        # "silent" payload: zeroed main_data (decodes as near-silence once
        # side-info/scalefactors are interpreted; honest placeholder, no
        # real Huffman-coded audio -- documented in NOTES.md)
        payload = bytes(payload_len)
        frames.append(header + payload)

    data = id3 + b"".join(frames)
    with open(OUT, "wb") as fh:
        fh.write(data)

    return {
        "sample_rate": sample_rate,
        "bitrate_kbps": bitrate_kbps,
        "frame_size": fsize,
        "num_frames": num_frames,
        "id3_size": len(id3),
        "total_size": len(data),
        "header_hex": build_header(BITRATE_INDEX, SAMPLING_INDEX, PADDING).hex(),
    }

if __name__ == "__main__":
    info = main()
    print(info)
