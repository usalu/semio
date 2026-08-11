#!/usr/bin/env python3
"""Re-scan the handcrafted MP3 for valid 11-bit sync words + decode header
fields, confirming byte-accurate frame boundaries."""
import struct
import sys

BITRATE_KBPS_V1_L3 = [None, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, None]
SAMPLE_RATE_V1 = [44100, 48000, 32000, None]

path = sys.argv[1]
data = open(path, "rb").read()

off = 0
id3_size = 0
if data[0:3] == b"ID3":
    flags = data[5]
    size_bytes = data[6:10]
    size = ((size_bytes[0] & 0x7F) << 21) | ((size_bytes[1] & 0x7F) << 14) | ((size_bytes[2] & 0x7F) << 7) | (size_bytes[3] & 0x7F)
    id3_size = 10 + size
    print(f"ID3v2.{data[3]}.{data[4]} tag found, header-declared size={size}, total ID3 block={id3_size} bytes")
    off = id3_size

frames_found = []
while off + 4 <= len(data):
    b0, b1, b2, b3 = data[off], data[off+1], data[off+2], data[off+3]
    if b0 != 0xFF or (b1 & 0xE0) != 0xE0:
        break  # not a sync word
    version = (b1 >> 3) & 0b11
    layer = (b1 >> 1) & 0b11
    protection = b1 & 0b1
    bitrate_index = (b2 >> 4) & 0xF
    sampling_index = (b2 >> 2) & 0b11
    padding = (b2 >> 1) & 0b1
    channel_mode = (b3 >> 6) & 0b11
    assert version == 0b11, f"expected MPEG version 1, got {version:02b}"
    assert layer == 0b01, f"expected Layer III, got {layer:02b}"
    bitrate = BITRATE_KBPS_V1_L3[bitrate_index]
    sample_rate = SAMPLE_RATE_V1[sampling_index]
    assert bitrate is not None and sample_rate is not None
    fsize = (144 * bitrate * 1000) // sample_rate + padding
    frames_found.append({
        "offset": off,
        "sync_ok": True,
        "version": "MPEG-1",
        "layer": "III",
        "protection_absent": bool(protection),
        "bitrate_kbps": bitrate,
        "sample_rate": sample_rate,
        "padding": padding,
        "channel_mode": ["stereo", "joint_stereo", "dual_channel", "mono"][channel_mode],
        "frame_size": fsize,
    })
    off += fsize

print(f"Frames found: {len(frames_found)}")
for f in frames_found:
    print(" ", f)

remaining = len(data) - off
print(f"Bytes consumed: {off} / {len(data)} total (remaining trailer: {remaining})")
assert remaining == 0, "trailing bytes after last frame -- frame size math is off"
assert len(frames_found) >= 2, "need at least 2 valid frames"
print("\nALL MP3 SYNC/FRAME ASSERTIONS PASSED")
