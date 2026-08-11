# mp3/example.mp3 — handcrafted MPEG-1 Layer III fixture

Generator: `../../generators/w0-fixtures/make_mp3.py`.
Verifier: `../../generators/w0-fixtures/verify_mp3.py`.

## Byte structure (exact)

```
bytes [0..57)     ID3v2.3.0 tag
                    'ID3', version=(3,0), flags=0, synchsafe-size=47
                    frame TIT2 "semio fixture"
                    frame TPE1 "W0 handcraft"
bytes [57..474)   MPEG-1 Layer III frame 0  (417 bytes: 4-byte header + 413-byte zeroed payload)
bytes [474..891)  MPEG-1 Layer III frame 1  (417 bytes)
bytes [891..1308) MPEG-1 Layer III frame 2  (417 bytes)
bytes [1308..1725) MPEG-1 Layer III frame 3 (417 bytes)
total: 1725 bytes
```

## Exact header field values chosen (identical for all 4 frames)

Header bytes (hex): `FF FB 90 C4`

- 11-bit sync: `0xFFE` (byte0=`0xFF`, top 3 bits of byte1=`111`) — present in all 4 headers.
- MPEG Version ID: `11` = MPEG-1.
- Layer: `01` = Layer III.
- Protection bit: `1` = no CRC (protection absent).
- Bitrate index: `1001` = 9 → **128 kbps** (MPEG-1 Layer III table).
- Sampling rate index: `00` → **44100 Hz**.
- Padding: `0`.
- Channel mode: `11` = **mono**.
- Mode extension / copyright / emphasis: all `0`.
- Original: `1`.
- Frame size formula (Layer III): `144 * bitrate_bps / sample_rate + padding` = `144 * 128000 / 44100 + 0` = **417 bytes** (integer division) — matches byte-accurately for all 4 frames, back-to-back with zero gap/overlap.
- Frame payload: zero-filled `413` bytes (`417 − 4` header bytes) — an honest "silent placeholder", not real Huffman-coded audio data (see limitation below).

## Verification performed

1. **Own re-scanner** (`verify_mp3.py`): parses the ID3v2 header's synchsafe size field, then walks the frame stream purely by re-deriving each frame's size from its own header fields (sync/version/layer/bitrate-index/sample-rate-index/padding) and confirms: (a) every 11-bit sync word is valid, (b) computed frame boundaries consume the file exactly to the last byte with **zero trailing bytes**, (c) 4 frames found. → **all assertions passed**.
2. **`file(1)`**: reports `Audio file with ID3 version 2.3.0, contains: MPEG ADTS, layer III, v1, 128 kbps, 44.1 kHz, Monaural` — independent confirmation of every header field.
3. **`ffprobe`**: `codec_name=mp3`, `sample_rate=44100`, `channels=1`, `channel_layout=mono`, `bit_rate=128000`, plus `TAG:title=semio fixture` / `TAG:artist=W0 handcraft` read back from the ID3v2 frames correctly.

## Known honest limitation

Frame payloads are zero-filled, not real Huffman-coded MPEG audio (no side-info/scalefactor/granule encoding) — sufficient for frame-header/sync-boundary assertions (the W0 requirement: "byte-accurate per the MP3 header spec") but decoders will produce silence, not a real tone, when decoding the audio content itself.
