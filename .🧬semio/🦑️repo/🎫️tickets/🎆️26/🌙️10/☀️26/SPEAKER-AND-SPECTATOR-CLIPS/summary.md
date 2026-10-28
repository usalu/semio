# Speaker and Spectator Clips

The clip render command now creates **Speaker** and **Spectator** videos by default. Speaker overlays the authored subtitles above scene content. Spectator creates no subtitle text or box. The same animation, caption-transition durations, narration clock and available audio are preserved.

Use `--audience speaker` or `--audience spectator` to render one variant. Repeat the option to select both explicitly. Python composition takes an explicit `audience` alongside the existing explicit `language`; lesson wrappers forward it through their render settings. Direct Manim renders use `SEMIO_TEACHING_AUDIENCE`, with Speaker as the direct-scene default.

Sources, traces, excerpts, logical chapters, lesson content and presentation outputs have separate audience namespaces. Filenames include `_speaker_` or `_spectator_`. Audience settings and the schema-backed policy participate in provenance, so one variant cannot be reused as the other.

## Verification

- Authored language-neutral schema, policy and cases before implementation. All five original regression cases failed against the original behavior; see `red.md`.
- AJV independently validates the policy and rejects reversed, missing and incorrectly subtitled variants. Its derived subtitle visibility matches the portable cases.
- Seven audience tests passed, including real Manim subprocess renders for both variants, caption fade/swap transitions, CLI selection, settings propagation, isolated output/cache paths, cache reuse and source/presentation fingerprint separation.
- PyAV and NumPy independently decode both native video variants. They verify identical frame timestamps, an empty spectator subtitle region throughout transitions, visible Speaker subtitle pixels, and unchanged scene content within encoding tolerance.
- The final selected aggregate ran 54 tests in 49.100 seconds: 52 passed, one existing native Windows symlink test skipped, and one source-provenance check caught concurrent source edits. The sole failing check passed immediately on its focused rerun (0.389 seconds), without changing or weakening the check. All 53 executable selected cases therefore passed across the final run and focused retry. Coverage includes real source composition, semantic excerpts, audio alignment, cancellation/process cleanup, presentation routing/contracts and semio style.
- `repo tools exec --light git -- diff --check` passed for all files changed by this ticket. The aggregate test entry includes the new audience module exactly once. The longest observed generated path was 158 characters.

The broader, concurrently evolving native presentation suite was also attempted. Its first FFmpeg presentation composition emitted a container header but no video frames for over two minutes, so that additional run was cancelled. This ticket does not certify full lesson presentation encoding or claim that all curriculum movies were rerendered. Audience source rendering, independent decoded subtitle behavior, and final compositor routing were verified as described above. Other source and output-path work in the same checkout was preserved.

## Reproduction

From the workspace root, run the seven audience checks and independent AJV validation:

```powershell
repo tools exec --light node -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/SPEAKER-AND-SPECTATOR-CLIPS/check.mjs'
```

The runner accepts unittest module/class/method names to reproduce the broader selection. It keeps generated files beneath this ticket's `🗑️generated` directory and reuses the existing teaching Python environment. No runtime dependency was added. All generated verification files were removed at completion; the runner and reports remain.

## Changed Files

Under `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/`:

- `🎬️clip/👥️audience/🐍️.py`, `🔣️.json`, `🧬️schema.json`, `🧪️tests/🐍️.py`, `🧪️tests/🔣️.cases.json`: policy, schema, portable cases and regression/native tests.
- `🎨️visuals/🐍️.py`: invisible spectator caption placeholder; Speaker overlay stacking.
- `🎬️clip/⚙️runtime/🐍️.py`: audience validation, subprocess propagation, cache and trace namespaces, filenames and provenance inputs.
- `🎬️clip/🖌️render/🐍️.py`: both variants by default; repeatable explicit audience selection.
- `🎬️clip/README.md`: variant and Python/CLI usage.
- `🎬️clip/🧪️tests/🐍️.py`, `🎬️clip/🪟️presentation/🧪️tests/🐍️.py`: explicit audience and namespaced fixture paths.
- `📜️script.ts`: register audience tests in the canonical aggregate.

This ticket retains `plan.md`, `red.md`, `summary.md` and `check.mjs`. No AGENTS.md, modifying Git command, worktree, IDE or dashboard was used.
