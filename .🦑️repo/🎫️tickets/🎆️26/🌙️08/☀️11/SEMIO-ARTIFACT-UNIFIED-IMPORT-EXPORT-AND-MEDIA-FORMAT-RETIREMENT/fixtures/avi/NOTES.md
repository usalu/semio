# avi/example.avi — handcrafted RIFF/AVI fixture

Generator: `../../generators/w0-fixtures/make_avi.py` (deterministic, no randomness).
Verifier: `../../generators/w0-fixtures/verify_avi.py`.

## Byte structure (exact)

```
RIFF (0x2DC = 732 bytes total)
└─ 'AVI '
   ├─ LIST 'hdrl'
   │  ├─ 'avih'  (56-byte MainAVIHeader, 14 DWORDs)
   │  └─ LIST 'strl'
   │     ├─ 'strh'  (64-byte AVIStreamHeader, fccType='vids', fccHandler='MJPG')
   │     └─ 'strf'  (40-byte BITMAPINFOHEADER)
   ├─ LIST 'movi'
   │  ├─ '00dc'  (140-byte MJPG frame 0, offset 4 from start of movi data)
   │  ├─ '00dc'  (140-byte MJPG frame 1, offset 152)
   │  └─ '00dc'  (140-byte MJPG frame 2, offset 300)
   └─ 'idx1'  (3 × 16-byte AVIOLDINDEX entries)
```

## Exact field values chosen

- `avih`: `dwMicroSecPerFrame=100000` (10 fps), `dwMaxBytesPerSec=1400`, `dwFlags=0x10` (AVIF_HASINDEX), `dwTotalFrames=3`, `dwStreams=1`, `dwSuggestedBufferSize=140`, `dwWidth=16`, `dwHeight=16`.
- `strh`: `fccType='vids'`, `fccHandler='MJPG'`, `dwScale=1`, `dwRate=10` (rate/scale = 10 fps), `dwLength=3`, `dwQuality=-1` (unset), `dwSampleSize=0` (variable, video stream), `rcFrame=(0,0,16,16)`.
- `strf` (BITMAPINFOHEADER): `biSize=40`, `biWidth=16`, `biHeight=16`, `biPlanes=1`, `biBitCount=24`, `biCompression='MJPG'` (fourcc as little-endian u32), `biSizeImage=140`.
- 3 `00dc` chunks in `movi`, each holding a minimal-but-real baseline JPEG (SOI, APP0/JFIF, DQT, SOF0 1-component grayscale 16×16, DHT, SOS, entropy-stub scan data, EOI). Frame N has scan fill byte `0x10*(N+1)`.
- `idx1`: 3 entries, fourcc `00dc`, `dwFlags=0x10` (AVIIF_KEYFRAME), offsets/sizes exactly matching the `movi` chunk layout (offsets are relative to the start of the `movi` list's data, i.e. to the `'movi'` fourcc itself, per the AVI 1.0 spec convention).

## Verification performed

1. **Own re-parser** (`verify_avi.py`): walks the RIFF tree generically, confirms `RIFF` size field == file size − 8, confirms `hdrl`/`movi`/`idx1` all present, decodes `avih`/`strh`/`strf` fields back out, extracts the 3 `00dc` chunks and confirms `idx1` offsets/sizes match exactly what was written → **all assertions passed**.
2. **`file(1)`**: reports `RIFF (little-endian) data, AVI, 16 x 16, 10.00 fps, video: Motion JPEG` — external tool confirms container + stream metadata independently.
3. **`ffprobe`**: `format_name=avi`, `probe_score=100`, `codec_name=mjpeg`, `width=16 height=16`, `r_frame_rate=10/1`, `nb_frames=3` — full external validation of container structure.
4. **`ffmpeg -f null -`**: decodes all 3 frames successfully (exit 0). One benign warning (`overread 8`) on the entropy-coded scan payload — the JPEG frames use a hand-stubbed (non-Huffman-real) scan body, since AVI container validity (the actual W0 requirement) doesn't require decodable pixel data. Documented here so a W3 agent isn't surprised if it also decodes pixels.

## Known honest limitation

The MJPEG frame *payload* markers (SOI/APP0/DQT/SOF0/DHT/SOS/EOI) are all real and correctly sized, but the entropy-coded scan bytes inside each frame are a fixed 4-byte stub, not a real Huffman-encoded image — sufficient for AVI/MJPG container-level parsing (which is what stdio's avi codec needs to assert against) but not for pixel-exact JPEG decode assertions.
