# mit-bestand lock refusal

## Root cause
Unicode normalization mismatch, not a stale lock. Member `🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript`:

- Directory on disk: `70 72 c3 a4 73 65 6e ...` = `pr` + U+00E4 (NFC).
- `♻️mit-bestand/bun.lock` (workspaces key and `@workspace:` ref): `70 72 c3 a4 73 ...` (NFC).
- `♻️mit-bestand/package.json` workspaces entry (uncommitted working-tree change, mtime Nov 13 12:11; HEAD was NFC): `70 72 61 cc 88 73 ...` = `pra` + U+0308 (NFD).
- Emoji variation selectors (EF B8 8F) are identical everywhere.

`checkBunLockV1` and `memberKeyV1` compared raw strings, so the NFD manifest key was "not listed" and the NFC lock key was "no member". The lock had no real drift. The writer that produced the NFD line was not identified (macOS or replica daemon); the code now tolerates either form.

## Changes
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🟦️.ts`: NFC-normalize lock workspace keys and manifest paths in `checkBunLockV1`.
- `.../📦️dependencies/🔒️lock-sync/🟦️.ts`: `memberKeyV1` NFC-normalizes members.
- `.../🔌️managers/🍞️bun/🧪️tests/🔣️.json`: two new language-agnostic cases (NFD manifest vs NFC lock, NFC manifest vs NFD lock; expect no drift).
- `.../🔌️managers/🍞️bun/🧪️tests/🐍️.py`: independent Python oracle normalizes NFC as well.
- `♻️mit-bestand/package.json`: the one NFD entry restored to NFC (file is now identical to HEAD). `bun.lock` untouched, no lock regeneration needed.
- Ticket input script: `lock-check-probe.ts`.

Repo implementation edited, so the project graph may be stale.

## Results
- `repo tools exec --light bun -- test` bun manager tests: 104 pass, 0 fail (includes the new cases and the Python oracle).
- lock-sync tests: 22 pass, 1 fail: "every install path asks the lock check before it queues..." greps for the string `buildOperationRequiresV1(command)==="dependencies"` in the build runtime source and no longer finds it (guard index -1). Unrelated to this change (source-shape test, another agent's runtime edit); not touched.
- Probe calling `requireJavascriptLocksInSyncV1` on the whole checkout, with the NFD manifest and again after restoring NFC: "all scopes in sync". The refusal is gone.
- No `repo run` was started; the pre-queue lock check is exactly this function, so the run's refusal no longer applies.
