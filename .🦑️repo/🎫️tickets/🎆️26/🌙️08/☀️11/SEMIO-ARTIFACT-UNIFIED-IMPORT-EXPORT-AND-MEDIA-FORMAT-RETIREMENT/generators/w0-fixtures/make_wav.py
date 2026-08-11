#!/usr/bin/env python3
"""Handcraft a real RIFF/WAVE file: PCM, 1 channel, 8000 Hz, 16-bit,
containing a real ~1 second 440 Hz sine wave (math.sin, not random bytes)."""
import math
import struct
import sys

OUT = sys.argv[1]

SAMPLE_RATE = 8000
NUM_CHANNELS = 1
BITS_PER_SAMPLE = 16
FREQ_HZ = 440.0
DURATION_S = 1.0
AMPLITUDE = 0.5  # of full-scale int16, to avoid clipping

def chunk(fourcc: bytes, data: bytes) -> bytes:
    padded = data if len(data) % 2 == 0 else data + b"\x00"
    return fourcc + struct.pack("<I", len(data)) + padded

def main():
    num_samples = int(SAMPLE_RATE * DURATION_S)
    samples = []
    max_amp = int(AMPLITUDE * 32767)
    for n in range(num_samples):
        t = n / SAMPLE_RATE
        v = math.sin(2.0 * math.pi * FREQ_HZ * t)
        samples.append(int(round(v * max_amp)))
    pcm = struct.pack(f"<{num_samples}h", *samples)

    byte_rate = SAMPLE_RATE * NUM_CHANNELS * (BITS_PER_SAMPLE // 8)
    block_align = NUM_CHANNELS * (BITS_PER_SAMPLE // 8)
    fmt_body = struct.pack(
        "<HHIIHH",
        1,                # wFormatTag = 1 (PCM)
        NUM_CHANNELS,
        SAMPLE_RATE,
        byte_rate,
        block_align,
        BITS_PER_SAMPLE,
    )
    fmt = chunk(b"fmt ", fmt_body)
    data_chunk = chunk(b"data", pcm)

    riff_body = b"WAVE" + fmt + data_chunk
    riff = chunk(b"RIFF", riff_body)

    with open(OUT, "wb") as fh:
        fh.write(riff)

    return {
        "sample_rate": SAMPLE_RATE,
        "channels": NUM_CHANNELS,
        "bits_per_sample": BITS_PER_SAMPLE,
        "freq_hz": FREQ_HZ,
        "num_samples": num_samples,
        "duration_s": num_samples / SAMPLE_RATE,
        "byte_rate": byte_rate,
        "block_align": block_align,
        "pcm_bytes": len(pcm),
        "total_size": len(riff),
        "max_amp": max_amp,
        "first_10_samples": samples[:10],
    }

if __name__ == "__main__":
    print(main())
