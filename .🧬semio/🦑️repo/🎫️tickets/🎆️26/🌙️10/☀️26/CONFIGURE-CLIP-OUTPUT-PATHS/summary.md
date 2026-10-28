# Clip Output Configuration — Completed

Clip output now defaults to `.🧬semio/🎓️teaching/⚡️cache/videos`, independently of the working directory. Cache and output overrides resolve from the workspace; explicit Manim media directories remain supported. Shared typography configures compact ignored paths before generating SVGs.

Narration writers, composition readers, timing consumers and canonical narration lookup share generated speech paths. Audio and timing changes invalidate cached renders. Workers retain the speech root when trace output is scoped by audience.

Removed 139 generated files: 128 untracked Pango SVGs under root `media/texts`, including the reported example, and 11 tracked timing/trace JSON outputs beside sources. The root media directory is gone. Tracked removals are working-tree deletions; the shared index remains untouched. Ignore rules prevent these paths from being newly checked in.

## Validation

- Tests reproduced the text-output and ignore failures, 25 narration-path failures and canonical narration lookup failure before their respective fixes.
- Final focused regression: **53 tests successful, one existing Windows symlink skip**, 47.834 seconds. Real Manim/Pango output, upstream config parsing, FFmpeg narration assembly, Git ignore matching and existing fontTools comparisons are covered.
- AJV validated the portable environment cases. TypeScript parsed the launcher successfully. Python syntax and Git whitespace checks passed.
- Native Windows was exercised; other operating systems were not executed in this session.
- Repository graph inspection hit unrelated source-admission failure (`.cache/b/l/fixed.dir` missing); focused checks ran through registered `repo tools exec` and the existing teaching environment.
- Temporary test output was removed. Reproducible input scripts and Markdown reports remain.

No runtime dependency, modifying Git command, worktree, IDE/dashboard use or AGENTS.md edit was introduced. Concurrent edits remain intact, including subsequent removal of the institutional intro speech entry point by other work.

See [callers.md](callers.md), [removed.md](removed.md) and [files.md](files.md) for the complete scope.
