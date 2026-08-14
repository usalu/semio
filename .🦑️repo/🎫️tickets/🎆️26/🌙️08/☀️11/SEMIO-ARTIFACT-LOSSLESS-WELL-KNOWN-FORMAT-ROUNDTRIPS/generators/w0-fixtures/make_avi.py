#!/usr/bin/env python3
"""Handcraft a structurally valid RIFF/AVI file with hdrl(avih+strl(strh+strf)),
movi list with real MJPG frame chunks, and idx1 index.

AVI layout (per ODML/OpenDML AVI spec, all chunk sizes byte-accurate):
RIFF('AVI '
  LIST('hdrl'
    avih(<MainAVIHeader>)
    LIST('strl'
      strh(<AVIStreamHeader> vids/MJPG)
      strf(<BitmapInfoHeader>)
    )
  )
  LIST('movi'
    00dc(frame0 jpeg bytes)
    00dc(frame1 jpeg bytes)
    00dc(frame2 jpeg bytes)
  )
  idx1(<AVIOLDINDEX entries>)
)
"""
import struct
import sys

OUT = sys.argv[1]

WIDTH = 16
HEIGHT = 16
FPS = 10
NUM_FRAMES = 3

def pad_even(b: bytes) -> bytes:
    return b if len(b) % 2 == 0 else b + b"\x00"

def chunk(fourcc: bytes, data: bytes) -> bytes:
    assert len(fourcc) == 4
    padded = pad_even(data)
    return fourcc + struct.pack("<I", len(data)) + padded

def list_chunk(list_type: bytes, body: bytes) -> bytes:
    assert len(list_type) == 4
    inner = list_type + body
    return chunk(b"LIST", inner)

def make_minimal_jpeg(width: int, height: int, fill: int) -> bytes:
    """A tiny, structurally valid baseline JPEG (SOI, APP0/JFIF, minimal
    DQT/SOF0/DHT/SOS with 1 solid MCU, EOI). Not a full encoder — just
    enough real JPEG markers to be a plausible MJPG frame payload."""
    soi = b"\xff\xd8"
    app0 = b"\xff\xe0\x00\x10JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00\x00"
    # A flat 8x8 luminance quant table (arbitrary but valid, all mid-value)
    dqt = b"\xff\xdb\x00\x43\x00" + bytes([16] * 64)
    sof0_payload = (
        b"\x08"  # precision
        + struct.pack(">HH", height, width)
        + b"\x01"          # num components = 1 (grayscale)
        + b"\x01\x11\x00"  # component: id=1, sampling=1x1, qtable=0
    )
    sof0 = b"\xff\xc0" + struct.pack(">H", 2 + len(sof0_payload)) + sof0_payload
    # Minimal DHT: DC table id 0, exactly 1 symbol of code length 1
    dht_counts = bytes([1] + [0] * 15)  # 16 count bytes, one code of length 1
    dht_symbols = bytes([0])
    dht_payload = bytes([0x00]) + dht_counts + dht_symbols  # class/id byte + counts + symbols
    dht = b"\xff\xc4" + struct.pack(">H", 2 + len(dht_payload)) + dht_payload
    sos_header = bytes([0x01, 0x01, 0x00, 0x00, 0x3f, 0x00])  # 1 component, DC/AC table 0, spectral 0..63, Ah/Al 0
    sos = b"\xff\xda" + struct.pack(">H", 2 + len(sos_header)) + sos_header
    scan = bytes([fill & 0xFF]) * 4  # entropy-coded payload stand-in (no 0xFF bytes to avoid marker collision)
    eoi = b"\xff\xd9"
    return soi + app0 + dqt + sof0 + dht + sos + scan + eoi

def main():
    frames = [make_minimal_jpeg(WIDTH, HEIGHT, 0x10 * (i + 1)) for i in range(NUM_FRAMES)]

    # ---- avih: MainAVIHeader (14 DWORDs = 56 bytes) ----
    us_per_frame = int(1_000_000 / FPS)
    max_bytes_per_sec = max(len(f) for f in frames) * FPS
    padding_granularity = 0
    flags = 0x10  # AVIF_HASINDEX
    total_frames = NUM_FRAMES
    initial_frames = 0
    streams = 1
    suggested_buffer_size = max(len(f) for f in frames)
    width = WIDTH
    height = HEIGHT
    avih_body = struct.pack(
        "<IIIIIIIIIIIIII",
        us_per_frame,
        max_bytes_per_sec,
        padding_granularity,
        flags,
        total_frames,
        initial_frames,
        streams,
        suggested_buffer_size,
        width,
        height,
        0, 0, 0, 0,  # reserved[4]
    )
    avih = chunk(b"avih", avih_body)

    # ---- strh: AVIStreamHeader (vids / MJPG) ----
    strh_body = struct.pack(
        "<4s4sIHHIIIIIIiIiiii",
        b"vids",
        b"MJPG",
        0,        # dwFlags
        0,        # wPriority
        0,        # wLanguage
        0,        # dwInitialFrames
        1,        # dwScale
        FPS,      # dwRate -> dwRate/dwScale = fps
        0,        # dwStart
        total_frames,  # dwLength
        suggested_buffer_size,  # dwSuggestedBufferSize
        -1,       # dwQuality (unset = -1, signed)
        0,        # dwSampleSize (0 = variable, video)
        0, 0,     # rcFrame left, top (signed LONGs)
        width, height,  # rcFrame right, bottom
    )
    strh = chunk(b"strh", strh_body)

    # ---- strf: BITMAPINFOHEADER ----
    strf_body = struct.pack(
        "<IiiHHIIiiII",
        40,          # biSize
        width,       # biWidth
        height,      # biHeight
        1,           # biPlanes
        24,          # biBitCount
        struct.unpack("<I", b"MJPG")[0],  # biCompression as fourcc int (little-endian bytes == 'MJPG')
        max(len(f) for f in frames),  # biSizeImage
        0, 0,        # biXPelsPerMeter, biYPelsPerMeter
        0, 0,        # biClrUsed, biClrImportant
    )
    strf = chunk(b"strf", strf_body)

    strl = list_chunk(b"strl", strh + strf)
    hdrl = list_chunk(b"hdrl", avih + strl)

    # ---- movi list with 00dc frame chunks ----
    movi_entries = []
    frame_offsets = []  # offset of each chunk's data relative to start of 'movi' fourcc+data (i.e. relative to the 'movi' list's data start, per idx1 convention: offset from start of 'movi' 4CC)
    running = 4  # first 4 bytes of movi LIST body are the 'movi' fourcc itself
    for f in frames:
        frame_offsets.append(running)
        c = chunk(b"00dc", f)
        movi_entries.append(c)
        running += len(c)
    movi_body = b"".join(movi_entries)
    movi = list_chunk(b"movi", movi_body)

    # ---- idx1: AVIOLDINDEX ----
    idx1_entries = b""
    for off, f in zip(frame_offsets, frames):
        idx1_entries += struct.pack(
            "<4sIII",
            b"00dc",
            0x10,          # AVIIF_KEYFRAME
            off,           # offset relative to 'movi' fourcc (start of movi data)
            len(f),        # size
        )
    idx1 = chunk(b"idx1", idx1_entries)

    riff_body = b"AVI " + hdrl + movi + idx1
    riff = chunk(b"RIFF", riff_body)

    with open(OUT, "wb") as fh:
        fh.write(riff)

    return {
        "width": width,
        "height": height,
        "fps": FPS,
        "num_frames": NUM_FRAMES,
        "frame_sizes": [len(f) for f in frames],
        "frame_offsets": frame_offsets,
        "total_size": len(riff),
        "us_per_frame": us_per_frame,
        "max_bytes_per_sec": max_bytes_per_sec,
        "suggested_buffer_size": suggested_buffer_size,
    }

if __name__ == "__main__":
    info = main()
    print(info)
