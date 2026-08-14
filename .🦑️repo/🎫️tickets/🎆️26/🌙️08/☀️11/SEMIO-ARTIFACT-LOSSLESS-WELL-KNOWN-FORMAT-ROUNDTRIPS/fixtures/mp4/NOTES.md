# mp4/example.mp4 + example-partial-frame.mp4 — real MP4 fixtures (copied, not handcrafted)

Both fixtures are real, existing ISO Base Media (MP4) files copied verbatim from elsewhere in the repo — not handcrafted, per the plan's note: *"real 43KB mp4 exists ... avi/mp3/wav/epw/tsv/html must be handcrafted"*.

## example.mp4 (primary, 42992 bytes)

Copied from `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🎥️logo.mp4` via:
```
cp "🧰️framework/🔨️modules/🖼️assets/🪧️logos/🎥️logo.mp4" fixtures/mp4/example.mp4
```
- Box layout: `ftyp` (isom/iso2/avc1/mp41 brands) → `free` (8-byte skip box) → `mdat` (H.264 payload) → (moov + tracks follow later in the file, standard non-fast-start layout).
- `ffprobe`: `codec_name=h264`, `width=410`, `height=140`, `duration=24.016667s`, `nb_frames=1441`, `format_name=mov,mp4,m4a,3gp,3g2,mj2`, `nal_length_size=4`, `extradata_size=46` (SPS/PPS present).
- This is the intended `codec_retention_law` byte-preserving round-trip fixture referenced by the master plan for W3's mp4 codec (must decode-clean and round-trip byte-for-byte).

## example-partial-frame.mp4 (secondary, 1547 bytes)

Copied from `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/partial_movie_files/1a3defa0690d_0.mp4` — one of several tiny (~1.5–1.7KB) real single-frame H.264 MP4s left behind by the animate plugin's Manim-style partial-movie cache. Chosen as the smallest available real secondary MP4 fixture (several sibling files in the same directory are byte-identical in size class: `57ff4df90e8f_0.mp4`, `a44ec3d894a1_0.mp4` also 1547 bytes; `863e81542435_0.mp4`, `6509403d3ac8_0.mp4`, `3a2834934200_0.mp4` are 1728 bytes — any could serve as an alternate).
- `ffprobe`: `codec_name=h264`, `width=64`, `height=64`, `duration=0.066667s`, `nb_frames=1` (single-frame clip), `nal_length_size=4`, `extradata_size=44`.
- Useful as a minimal/fast round-trip smoke fixture (1 frame, tiny) distinct from the larger 1441-frame primary — good for quick codec unit tests that don't need the full 43KB asset.

## Verification performed

`ffprobe -show_format -show_streams` run against both files (see field values above) — both cleanly recognized as valid ISO-BMFF/H.264 MP4 by ffmpeg's own demuxer, confirming they are genuinely decodable, not corrupted copies. `file(1)` also independently reports both as `ISO Media, MP4 Base Media v1 [ISO 14496-12:2003]`.
