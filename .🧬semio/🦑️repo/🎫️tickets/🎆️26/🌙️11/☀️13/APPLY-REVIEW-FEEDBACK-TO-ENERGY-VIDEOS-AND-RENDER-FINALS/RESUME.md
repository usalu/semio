# Resume notes (halted 2026-11-13 ~13:15 for a computer restart, on the owner's instruction)

Nothing of this ticket is running. Source work is complete in the shared tree; what is owed is
rendering, verification and closing.

## State
- Feedback F01–F29, consistency pass, reference registry, posters, roadmap-first: done (see the
  reports in this folder).
- Morph engine: `split` now accepts `copy: true` (`🎓️teaching/🏛️architecture/🔀️morph/🎞️full/🐍️.py`
  guard + `README.md`). Without it the final profile crashed in physics Beat6
  (`physics.fundamentals.tiles.first-to-grid`); heating conduction and cooling internal-gains use
  the same recipe.
- Finals (profile final, de): none complete.
  - ventilation: presenter mp4 from 12:02 predates the morph fix; audience run aborted by the
    change guard. Default root `.🧬semio/🎓️teaching/⚡️cache/videos`.
  - physics: failed before the fix, segments reusable, default root.
  - heating / cooling / energy_balance: just started in roots `⚡️cache/vb`, `vc`, `vd`.
- Tests: full `script.ts test` hung 4 h, stopped without result. Modules were green one by one
  (`tests/REPORT.md`, `references/REPORT.md`) except visuals integration
  `test_physics_scene_constructor_bounds` (radiator height 0.6812 vs 0.7384); integration needs
  `SEMIO_TEACHING_PROFILE=preview` and `SEMIO_TEACHING_LANGUAGE=en`.
- Speech: no key (`🎓️teaching/.env` missing), no cached audio → silent voice track.

## Next steps
1. Four lanes in parallel, `--jobs 3`, `repo tools exec --light bun -- <script.ts> render
   --language de --video <id> --profile final --no-play [--output-root <root>]`:
   ventilation then physical_fundamentals (default root), heating (`vb`), cooling (`vc`),
   energy_balance (`vd`). Do not edit scene, runtime or morph sources while they run.
2. Check frames of each final against FEEDBACK.md (overview cards, roadmap first, shower beat in
   physics, key feedback scenes); `ffmpeg` is not on PATH, use the repo tool runner.
3. Re-run test modules one by one; settle the radiator-height check.
4. Copy or re-home the `vb`/`vc`/`vd` results, delete `🗑️generated`, close the ticket.
