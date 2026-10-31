# Transcripts And README Rows

- Fingerprint check: `segment_state` in `🎬️clip/⚙️runtime/🐍️.py` hashes `.py` sources, assets with suffix json, pdf, svg, png, jpg, webp, fonts and audio, timing and theme files. Markdown is not fingerprinted, so only `🗣️transcript.{en,de}.md` and `README.md` were written. No scene, toolkit, model or `🔣️.json` file was touched, and `registry/apply.py` (which writes manifests) was not run.
- `registry/write-transcripts.py` imports each scene class and reads `NARRATION` and `TITLE` at runtime (f-strings resolved). Grasshopper: 115 clauses over 5 videos. Excel: 95 clauses over 4 videos. The files were already identical to the regenerated output.
- `registry/readme_rows.py` had stale hard-coded rows (they predated the narration, for example the energy row still described a wrong climate value). It now holds nine rows (EN and DE) derived from the current narration, and refreshed the rows in both `🎬️clip/README.md` files. Run it with `PYTHONUTF8=1`.
- Tests: `🧪️tests/📇️registry/🐍️.py` in both clip folders, 5 tests each, OK (transcripts checked against NARRATION).
