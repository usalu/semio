# conv-g: docstring, language and policy convergence of the 1025f/1026g production modules

Source: `audit-docstr-1026g.md` (aud-docstr-g). Lane: conv-g. Nothing in the live library tree was edited; the whole batch is staged under `conv-g-tools/staged` (66 files: 65 edited, 1 new).

## Apply

- `python3 -I conv-g-tools/stage.py check|apply|revert|status` (three-way `git merge-file` on drift, backup under `C/🗑️generated/conv-g/backup`, `revert` merges the reverse change).
- Apply AFTER `oracle2-g-tools/stage.py apply`: seven read-oracle files started from oracle2-g's staged versions (oracle2-g had already made every docstring emoji there unique), so `check` reports `PREREQ-ORACLE2` for them until oracle2-g is applied and `apply` refuses everything.
- Overlaps with other staged batches were merge-checked: `xplat3-g` (ledger texts), `jobsrv-g` (unit-cache schema) and `ci-g` (`suites/🟦️.ts`) merge clean in either order.
- Checks without a repo run (no official build or test ran): `zsh conv-g-tools/verify.sh static` = stage check, apply/revert self-test in a scratch root (passes), 24 laws (JSON schemas, EN/DE placeholder parity, key usage, Bun transpile of every staged file, emoji uniqueness per file). After apply: `zsh conv-g-tools/verify.sh suites` runs the graph-free single-file suites of 21 touched modules (not run yet).

## What changed against the audit

| Audit check | Result |
|---|---|
| 90 exports without docstring | all 90 documented (types of `leases/standing`, `host-capacity/phases`, `testing/workers`, `libc`, sealed, `witness`, `cutoff`, `frontier-walk`, `yield`, `claims`, `test-owners`, change journal, read-oracle) |
| 3 non-emoji docstrings (+ keycap false positive) | the three interface members of `changes/🧭️client` now start with an emoji |
| 190 duplicate emojis in 69 groups | every docstring emoji is unique per file in all 53 files (Rust `🦀️.rs` included) |
| 1 body comment (`witness/🟨️.mjs:168`) | removed, the reason is in the docstring |
| 18 EN-only rows | read-oracle CLI, request refusal, availability reasons and the Landlock launcher are now catalogue messages in both languages (six new keys in `read-oracle/🔣️.json`, Python `refuse(english, german)`); machine budget policy errors are ten `machine.policy.*` keys in the ledger texts (old prefix kept, so the law still matches); the change-journal "cannot vouch" refusal carries EN / DE |
| 13 hard-coded policy values | moved into JSON: read-oracle `memoryLimitMegabytes` and `tailBytes`; authorizer memo `MemoPolicy` (4 MiB, 24 lanes); TypeScript check `RetentionPolicy` (2 programs, max 8, 2 GiB, 120 s); frontier-walk `Policy` (new `🧬️schema/🔣️.json`); store-bound `probePatienceMs`; host-capacity `wait.jitterBasisPoints` 2500 (repository policy, schema, type, fixture); unit-cache lock and pool `pollBackoffBasisPoints` 15000; change journal `startPollMilliseconds` 20; sealed `maximumTextCharacters`, `maximumPathCharacters`, `maximumCommandLineCharacters` (policy, schema, interface) |
| Python missing docstrings | `refuse` and `kernel` documented |

Value-preserving: 15000 bp gives the same integers as the old factor 1.5 for every poll up to 4096 (law), 2500 bp jitter equals `0.75 + 0.5 * random`.

## Not changed on purpose

- `nx-tooling:census|probe|wait|retire` progress strings are machine keys, not text; the renderer owner decides about localisation.
- `ci-report/🗂️suites/📜️script.ts:93` prints the message of a non-repository error as it is (internal invariant).
- `🐍️.py` `UNAVAILABLE = 125` stays an exit-code constant.
- Internal EN-only invariant throws (about 15 files) stay as they are: the audit lists them only as a count.

## Files

All staged paths: `find conv-g-tools/staged/files -type f`. Tools: `conv-g-tools/{ops.py,build.py,make-staged.py,stage.py,selftest.py,laws.py,verify.sh}`.
