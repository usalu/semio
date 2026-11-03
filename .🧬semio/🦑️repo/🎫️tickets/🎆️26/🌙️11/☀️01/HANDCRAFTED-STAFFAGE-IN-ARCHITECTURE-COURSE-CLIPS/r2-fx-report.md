# r2 fx: clip runtime `language` test failures

C = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip`.

## Root cause
Pre-existing, not caused by the staffage migration. `C/⚙️runtime/🐍️.py` is unchanged against HEAD and already requires an explicit language: `segment_state` (line 427) and `render_segment` (lines 492, 517, 518) read `settings["language"]`, and `source_narration` / `source_speech_root` (lines 432-453) take a keyword-only `language` and store speech under `speech/<language>/<identity>`. The tests predate that contract. No staffage loader, cast manifest or `ensure_casts --lang` code is involved. Same cause as the `KeyError: 'language'` found by C1 in the semio-style test.

## Fix (tests only, nothing weakened)
- `C/👥️audience/🧪️tests/🐍️.py`: the three settings dicts now carry `"language": "en"`.
- `C/📁️output/🧪️tests/🐍️.py`: `source_narration(..., language=...)`, `segment_state` gets the language, and the clause fixture is written at the language-scoped path `speech/<language>/<identity>/...` (computed from `output.speech_root`, independent of the runtime helper).
- `C/📁️output/🧪️tests/🔣️.cases.json`: narration case gains `"language": "en"` (case data stays language-agnostic).

## Results
- audience: Ran 7, OK (was 3 errors).
- output: Ran 9, OK (was 1 error).
- Mid-session an unrelated concurrent edit left `heating/clip/solar-gains/🐍️.py` with an IndentationError for a few minutes, which broke imports in both modules; it resolved itself and is not ours.

Nothing open.
