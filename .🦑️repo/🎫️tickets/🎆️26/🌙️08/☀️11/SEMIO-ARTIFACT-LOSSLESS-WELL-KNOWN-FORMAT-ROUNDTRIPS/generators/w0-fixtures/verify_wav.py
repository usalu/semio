#!/usr/bin/env python3
"""Re-parse the WAV file's RIFF chunks, decode the 16-bit PCM samples, and
confirm they match a real 440 Hz sine (correlate against a fresh reference
sine rather than trusting the generator)."""
import math
import struct
import sys

path = sys.argv[1]
data = open(path, "rb").read()

assert data[0:4] == b"RIFF"
riff_size = struct.unpack("<I", data[4:8])[0]
assert riff_size == len(data) - 8, f"riff size mismatch {riff_size} vs {len(data)-8}"
assert data[8:12] == b"WAVE"

off = 12
fmt = None
pcm = None
while off + 8 <= len(data):
    fourcc = data[off:off+4]
    size = struct.unpack("<I", data[off+4:off+8])[0]
    body = data[off+8:off+8+size]
    if fourcc == b"fmt ":
        fmt = struct.unpack("<HHIIHH", body[:16])
    elif fourcc == b"data":
        pcm = body
    off += 8 + size + (size % 2)

audio_format, channels, sample_rate, byte_rate, block_align, bits_per_sample = fmt
print(f"fmt: format={audio_format} (1=PCM) channels={channels} sample_rate={sample_rate} "
      f"byte_rate={byte_rate} block_align={block_align} bits_per_sample={bits_per_sample}")
assert audio_format == 1
assert channels == 1
assert sample_rate == 8000
assert bits_per_sample == 16
assert byte_rate == sample_rate * channels * bits_per_sample // 8
assert block_align == channels * bits_per_sample // 8

num_samples = len(pcm) // 2
samples = struct.unpack(f"<{num_samples}h", pcm)
print(f"decoded {num_samples} samples, duration={num_samples/sample_rate:.3f}s")

# Reference 440 Hz sine, freshly generated (independent of the writer)
freq = 440.0
max_amp = int(0.5 * 32767)
ref = [int(round(math.sin(2 * math.pi * freq * (n / sample_rate)) * max_amp)) for n in range(num_samples)]

max_diff = max(abs(a - b) for a, b in zip(samples, ref))
print(f"max abs diff between decoded samples and freshly-computed reference sine: {max_diff}")
assert max_diff <= 1, "decoded PCM does not match a real 440Hz sine within rounding tolerance"

# Sanity: zero crossings roughly match expected count for 440Hz over 1s
zero_crossings = sum(1 for i in range(1, num_samples) if (samples[i-1] < 0) != (samples[i] < 0))
expected_crossings = 2 * freq * (num_samples / sample_rate)
print(f"zero crossings: {zero_crossings} (expected ~{expected_crossings:.0f})")
assert abs(zero_crossings - expected_crossings) < expected_crossings * 0.15

print("\nALL WAV STRUCTURAL + SINE-SHAPE ASSERTIONS PASSED")
