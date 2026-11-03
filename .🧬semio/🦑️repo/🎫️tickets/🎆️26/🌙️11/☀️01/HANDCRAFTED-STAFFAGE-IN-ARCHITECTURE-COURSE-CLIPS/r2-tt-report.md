# R2 speech re-synthesis check (agent tt)

Scope: the two clauses changed by agent nv (see `r2-nv-report.md`). Clip text was not changed and nothing was synthesized.

- Cooling internal-gains, Beat1 "interior" (clause index 2, en and de). Clip folder `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains`, identity `f1c82c405a`.
- Heating internal-gains, Beat2 "power" (clause index 1, en and de). Clip folder `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/👥️internal-gains`, identity `4ffd7465c8`.

## Answer: not re-synthesized automatically

The identity is not content-hashed. Clause audio is keyed by folder and clause index.

- Path scheme: `source_speech_root` (`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:432-437`) gives `<speech>/<language>/<identity>`. `_output.speech_root` (`🎬️clip/📁️output/🐍️.py:24-28`) hashes only the clip folder path (`sha256(owner relative path)[:10]`). `source_narration` (`⚙️runtime/🐍️.py:440-458`) resolves `<root>/beat_N_clauses/clause_NN.<ext>`. The file name depends on the index and the text does not enter it. Changing the wording leaves the path unchanged.
- The render never synthesizes. `source_narration` only resolves existing files and `render_segment` attaches them (`⚙️runtime/🐍️.py:492`, `:530`). The scene `.py` is part of the segment state, so the clip re-renders, but it picks up whatever audio sits at the same path.
- Cache check: `.🧬semio/🎓️teaching/⚡️cache/videos/speech/` contains only `de/a5b15db352/Shared_clauses` and `de/b47b2f4be4/beat_1_clauses`. There is no `en/` directory. Neither `f1c82c405a` nor `4ffd7465c8` has any speech directory. Nothing is stale on disk. These two beats currently have no narration audio at all, so the premise of a stale file does not hold.
- Because no audio exists for these identities, there is no cached identity to compare against. Clause durations were not measured and the caption-fit check was not run.

## Synthesis not possible now

- No TTS key: `TTS_API_KEY` and `OPENROUTER_API_KEY` are unset, and there is no `🎓️teaching/.env`. `synthesize_clause_audio` returns `None` for every clause without a key (`🔊️speech/🐍️.py:272-273`), and `cmd_synth` prints "No audio produced". Nothing was generated.

## Tooling mismatch found (affects any synthesis, not only these clauses)

1. German only. `subtitle_clauses` takes `text_de` (`🔊️speech/🐍️.py:88-90`) and `synthesize_clause_audio` ignores language. Nothing synthesizes English through the repo tooling.
2. Wrong output layout. The per-clip modules write to `speech_root(BASE_DIR)` without the `<language>` segment (for example `❄️cooling/.../👥️internal-gains/🔊️speech/🐍️.py`, `SPEECH_DIR`). `source_narration` reads `speech/<language>/<identity>`, so the files would never be found.
3. Beat granularity. `cmd_synth --beat N` regenerates every clause of that beat, not only the changed one. The cooling module's `cmd_align` also uses `text_de` for indices.
4. The heating module (`🔥️heating/.../👥️internal-gains/🔊️speech/🐍️.py`) is legacy. It writes one whole-beat `beat_i_audio.mp3` from `narration_text`, not per-clause clips, and has no language segment. It cannot feed the runtime.

## Open

- Provide a TTS key (`TTS_API_KEY` or `OPENROUTER_API_KEY`, or `teaching/.env`).
- Make the synthesis entry language-aware and per-clause, writing `speech/<en|de>/<identity>/beat_N_clauses/clause_NN.<ext>`. Then synthesize only clause_02 for cooling Beat1 and clause_01 for heating Beat2, in both languages.
- Then check that each duration fits its `hold_for` window (`probe_duration` against the VO trace marks, `🔊️speech/🐍️.py:101`, `:330`) and re-run the clip render. This was not done because it needs both the key and the tooling change.

No files other than this report were written. No generated output was produced.
