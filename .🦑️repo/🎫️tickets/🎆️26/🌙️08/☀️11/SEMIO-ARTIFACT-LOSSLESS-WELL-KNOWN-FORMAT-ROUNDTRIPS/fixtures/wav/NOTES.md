# wav/example.wav — handcrafted RIFF/WAVE PCM fixture

Generator: `../../generators/w0-fixtures/make_wav.py`.
Verifier: `../../generators/w0-fixtures/verify_wav.py`.

## Byte structure (exact)

```
RIFF (16036 bytes body)
└─ 'WAVE'
   ├─ 'fmt '  (16-byte PCM format chunk)
   └─ 'data'  (16000 bytes = 8000 samples × 2 bytes, 16-bit PCM)
total file size: 16044 bytes
```

## Exact field values chosen

- `fmt `: `wFormatTag=1` (PCM), `nChannels=1`, `nSamplesPerSec=8000`, `nAvgBytesPerSec=16000` (=8000×1×2), `nBlockAlign=2` (=1×2), `wBitsPerSample=16`.
- `data`: **8000 samples** (exactly 1.0 second at 8000 Hz) of a real `sin()`-generated 440 Hz tone: `sample[n] = round(sin(2π·440·n/8000) * 0.5 * 32767)`, amplitude scaled to 50% full-scale (max ±16383) to avoid clipping. First 10 sample values: `0, 5550, 10443, 14102, 16093, 16181, 14357, 10834, 6031, 515`.

## Verification performed

1. **Own re-parser + independent re-synthesis** (`verify_wav.py`): re-walks the RIFF chunk tree, decodes `fmt ` fields and confirms PCM/mono/8000 Hz/16-bit, decodes all 8000 samples from `data`, and — critically — **freshly recomputes** a reference 440 Hz sine (independent code path, not reusing the writer's sample array) and diffs sample-by-sample: **max absolute difference = 0** (exact match after rounding). Also counts zero-crossings (879 observed vs. 880 expected for 440 Hz over 1s) as an independent sanity check on the waveform shape. → **all assertions passed**.
2. **`file(1)`**: reports `RIFF (little-endian) data, WAVE audio, Microsoft PCM, 16 bit, mono 8000 Hz` — confirms format/channels/rate/bit-depth independently.
3. **`ffprobe`**: `codec_name=pcm_s16le`, `sample_rate=8000`, `channels=1`, `bits_per_sample=16` — full external confirmation.

No known limitations — this fixture is a genuine, decodable 440 Hz PCM tone.
