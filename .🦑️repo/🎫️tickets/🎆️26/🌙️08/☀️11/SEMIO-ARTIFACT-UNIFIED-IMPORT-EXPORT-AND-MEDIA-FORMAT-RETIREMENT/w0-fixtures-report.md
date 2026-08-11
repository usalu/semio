# W0 Fixtures Report — new-format artifact test fixtures

Scope: handcraft/collect real, spec-conformant fixture files for the 7 new format artifacts (avi, mp3, wav, epw, tsv, html) plus the existing mp4 asset, per master plan §"New format artifacts" and the "Fixtures" row of the recipe table (line 65-77 of `📋️master-plan.md`). All fixtures staged under `fixtures/<format>/example.<ext>` with a sidecar `NOTES.md` per format documenting exact byte/field values for W3 format-artifact agents to write assertions against.

Generator/verifier scripts (deterministic, no hand-typed binary bytes) live in `generators/w0-fixtures/`.

## Fixtures produced

| Format | File(s) | Size | Method | Verification |
|---|---|---|---|---|
| avi | `fixtures/avi/example.avi` | 732 B | Handcrafted with `generators/w0-fixtures/make_avi.py` (Python `struct`): RIFF/AVI, `hdrl`(avih+strl(strh+strf, vids/MJPG)), `movi`(3× real `00dc` MJPEG frame chunks, 16×16), `idx1` (3 index entries) | Own re-parser (`verify_avi.py`) walks the RIFF tree and confirms RIFF size, all 4 required chunks present, `idx1` offsets/sizes match `movi` exactly ⟶ passed. Cross-checked with `file(1)` ("RIFF ... AVI, 16 x 16, 10.00 fps, video: Motion JPEG") and `ffprobe` (`format_name=avi`, `probe_score=100`, `codec_name=mjpeg`, `nb_frames=3`) and `ffmpeg -f null -` (decodes all 3 frames, exit 0). |
| mp3 | `fixtures/mp3/example.mp3` | 1725 B | Handcrafted with `make_mp3.py`: ID3v2.3.0 tag (TIT2/TPE1 frames) + 4 MPEG-1 Layer III frames, 128 kbps/44100 Hz/mono, correct 11-bit sync + byte-accurate frame-size formula (`144×bitrate/samplerate+padding` = 417 B/frame) | Own re-scanner (`verify_mp3.py`) walks purely by re-deriving frame size from each header's own fields; confirms 4 valid sync words and **zero trailing bytes** after the last frame ⟶ passed. Cross-checked with `file(1)` ("MPEG ADTS, layer III, v1, 128 kbps, 44.1 kHz, Monaural") and `ffprobe` (`codec_name=mp3`, `sample_rate=44100`, `channels=1`, ID3 tags read back correctly). |
| wav | `fixtures/wav/example.wav` | 16044 B | Handcrafted with `make_wav.py`: RIFF/WAVE, `fmt ` (PCM, 1ch, 8000 Hz, 16-bit), `data` = 8000 samples of a real `math.sin()`-generated 440 Hz tone (not random bytes), 1.0 s exactly | Own re-parser + **independent re-synthesis** (`verify_wav.py`): decodes all 8000 samples, freshly recomputes a reference 440 Hz sine via a separate code path, diffs sample-by-sample ⟶ max abs diff = **0**; zero-crossing count (879) matches theoretical expectation (880) within tolerance ⟶ passed. Cross-checked with `file(1)` and `ffprobe` (`codec_name=pcm_s16le`, `sample_rate=8000`, `channels=1`, `bits_per_sample=16`). |
| epw | `fixtures/epw/example.epw` | 6124 B | Handcrafted with `make_epw.py`: all 8 EPW header lines (LOCATION with 10 fields, DESIGN CONDITIONS, TYPICAL/EXTREME PERIODS, GROUND TEMPERATURES, HOLIDAYS/DAYLIGHT SAVINGS, COMMENTS 1, COMMENTS 2, DATA PERIODS) + 24 hourly records (hour 1..24), **all 35 EPW columns** populated with plausible physically-derived values (diurnal temperature sine, daylight-bell solar radiation curves, non-zero-only-in-daytime illuminance, etc.), CRLF line endings matching real EPW files | Own re-parser (`verify_epw.py`): confirms 32 total lines, all 8 header keywords match exactly, LOCATION has exactly 10 fields, all 24 records have exactly 35 columns, hour sequence is 1..24 in order, temperature/humidity/radiation ranges are physically plausible ⟶ passed. |
| tsv | `fixtures/tsv/example.tsv` | 287 B | Handcrafted with `make_tsv.py`: header + 5 data rows, 5 columns, LF endings, trailing newline; one row's `name` field contains the literal characters `\t` (backslash+t) to illustrate — without actually breaking the format — the IANA-TSV "no quoting" edge case, documented in the sidecar NOTES.md | Own re-parser (`verify_tsv.py`): confirms zero `\r` bytes, every row has exactly 5 columns after `split('\t')`, and a **byte-exact round-trip** (split then rejoin with `\t`/`\n` reproduces the original file bytes exactly) ⟶ passed. |
| html | `fixtures/html/example.html` | 1185 B | Handcrafted directly (plain text, no generator needed): `<!DOCTYPE html>`, `<head>` with 2 meta + title + style block, `<body>` with nested div/p/span/ul/li/a, void elements `br` + `img` (data-URI, no network fetch), one valueless boolean attribute (`disabled`), one HTML comment, one inline `<script>` block | Re-parsed with Python's stdlib `html.parser.HTMLParser` via a purpose-built verifier: tag-stack balance check (empty at EOF = well-formed), confirms both void elements present without closing tags, confirms comment + valueless attribute + script/style/meta/title all present ⟶ passed (19 tags, 4 void elements, 1 comment, 1 valueless attr, balanced). |
| mp4 | `fixtures/mp4/example.mp4` (primary) | 42992 B | **Copied**, real asset: `cp "🧰️framework/🔨️modules/🖼️assets/🪧️logos/🎥️logo.mp4" fixtures/mp4/example.mp4` — no handcrafting per plan (this is the designated `codec_retention_law` byte-preserving fixture) | `ffprobe`: `codec_name=h264`, 410×140, `duration=24.02s`, `nb_frames=1441`, `format_name=mov,mp4,...`, SPS/PPS present (`extradata_size=46`). `file(1)`: "ISO Media, MP4 Base Media v1". |
| mp4 (secondary) | `fixtures/mp4/example-partial-frame.mp4` | 1547 B | **Copied**, real asset from `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/partial_movie_files/1a3defa0690d_0.mp4` — a tiny real single-frame H.264 MP4 from the animate plugin's Manim-style partial-movie cache (several byte-identical-size siblings exist in the same dir as alternates) | `ffprobe`: `codec_name=h264`, 64×64, `duration=0.067s`, `nb_frames=1`. `file(1)`: "ISO Media, MP4 Base Media v1". Chosen as a fast/minimal round-trip smoke fixture distinct from the 43KB primary. |

## Sidecar NOTES.md files

Every format directory has a `NOTES.md` documenting the exact byte layout / field values chosen (so W3 agents can write assertions against known values without re-deriving them):

- `fixtures/avi/NOTES.md`
- `fixtures/mp3/NOTES.md`
- `fixtures/wav/NOTES.md`
- `fixtures/epw/NOTES.md`
- `fixtures/tsv/NOTES.md`
- `fixtures/html/NOTES.md`
- `fixtures/mp4/NOTES.md`

## Verification methodology summary

Every binary/structured fixture was verified two ways:
1. **A from-scratch re-parser** (own Python script, independent of the generator's internal state — re-walks chunks/headers/records purely from the bytes on disk and re-derives expected values) — this is the authoritative check since it doesn't trust the writer.
2. **An independent external tool** where available: `file(1)` (magic-byte format sniffing) and/or `ffprobe`/`ffmpeg` (full demux, and for wav/mp3, real audio decode) for avi/mp3/wav/mp4. epw/tsv/html have no equivalent widely-available CLI validator, so those rely on the from-scratch re-parser plus (for html) Python's stdlib `html.parser`.

All 7 handcrafted fixtures plus both mp4 copies passed every check performed. No known structural defects. The one documented honest limitation is that the avi's MJPEG frame *entropy-coded scan bytes* and the mp3's frame *audio payload bytes* are placeholder/zero-filled (container- and header-level correctness only, not full lossy-codec-accurate payloads) — this is flagged in each format's NOTES.md and does not affect AVI/MP3 container-level structural validity, which is what the W0 requirement calls for.

## Files touched (created)

- `fixtures/avi/example.avi`, `fixtures/avi/NOTES.md`
- `fixtures/mp3/example.mp3`, `fixtures/mp3/NOTES.md`
- `fixtures/wav/example.wav`, `fixtures/wav/NOTES.md`
- `fixtures/epw/example.epw`, `fixtures/epw/NOTES.md`
- `fixtures/tsv/example.tsv`, `fixtures/tsv/NOTES.md`
- `fixtures/html/example.html`, `fixtures/html/NOTES.md`
- `fixtures/mp4/example.mp4` (copied), `fixtures/mp4/example-partial-frame.mp4` (copied), `fixtures/mp4/NOTES.md`
- `generators/w0-fixtures/make_avi.py`, `verify_avi.py`
- `generators/w0-fixtures/make_mp3.py`, `verify_mp3.py`
- `generators/w0-fixtures/make_wav.py`, `verify_wav.py`
- `generators/w0-fixtures/make_epw.py`, `verify_epw.py`
- `generators/w0-fixtures/make_tsv.py`, `verify_tsv.py`
- `w0-fixtures-report.md` (this file)

No hot files (glue.rs, script.ts, catalog.json, taxonomy.json) were touched. `git check-ignore -v` confirmed none of the new paths are gitignored.
