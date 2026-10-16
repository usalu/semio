# Bazel Snapshot Directory Handles

Both snapshot directory permission passes now retain an actual directory file descriptor opened with `O_RDONLY | O_NOFOLLOW` (where the platform exposes the flag). Before `handle.chmod`, bigint `fstat` must match the captured directory device/inode and ordinary directory kind. Stable leaf checks before and after the descriptor mutation refuse replacement. Original cancellation/deadline checks follow every awaited acquisition/stat/chmod operation. The descriptor closes even when its body fails; arbitrary primary values, including `undefined`, survive simultaneous close failure. A close failure after successful work remains fatal.

Existing output directories capture their identity during the real traversal; newly restored directories capture identity immediately after creation, before population. The preparatory writable pass and deepest-first final mode pass use the same private helper. Files retain the existing file-mode path. No new authority, clock, policy, grant, filesystem cleanup or tooling custody is introduced.

## Actual RED and GREEN

- `generated/bazel-snapshot-directory-race-red.log`: first actual restore incorrectly accepted a replacement. This initial vector used a foreign mode equal to the writable mode, so it proves missing refusal rather than an independent foreign-mode change.
- `generated/bazel-snapshot-directory-race-semantic-red.log`, session **43275**, terminal **1**: the corrected literal vector's foreign directory began at **0700 (448)** and ended at **0555 (365)** after the real path chmod followed the replacement symlink. The preservation assertion failed. Aggregate: **14 pass, 9 skip, 1 fail, 384 assertions, 1.457 s**.
- `generated/bazel-snapshot-directory-handle-green.log`, session **82662**, terminal **0**: **16 pass, 9 skip, 419 assertions, 3.80 s**, authentic existing Original **120 s**, no SDK/tooling owner. The two actual syscall interleavings swap a real directory into a symlink immediately before mode mutation, for both preparatory and final passes. Both refuse and preserve foreign permissions. Five real FileHandle body/close cases check actual FD closure, exact Error/undefined primary preservation, and successful-body close refusal.

The language-neutral JSON vectors agree with independent SQLite outcome/mode oracles; cleanup vectors also pass Ajv's closed schema. The real filesystem run used Bun 1.3.14 on native macOS arm64. Windows/Linux runtime behavior is not claimed. The existing actual capture/restore, readonly-directory idempotence/replacement, link-kind, TCP framing and Source dispatcher tests also passed in this run. The nine real Bazel integration phases were intentionally skipped; their fresh runtime matrix remains separate.

Low's final narrow Source review is clean. Production Source is held after this run.

## Current files

| Path | Bytes | SHA-256 |
| --- | ---: | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | 62271 | `0c81e92041b4a4867e29c03c4841df67bf752e37ef483f2b00e24094a304b156` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧬️schema/📦️snapshot.json` | 1144 | `a57b0831a86888a14186b6e3806920dc3d5fdece754d725f47b66c5379b19741` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🟦️.ts` | 9252 | `7862833b135985149229bbaaed4a7412634d846d8a9b67d65bda478531107639` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🔣️.json` | 1414 | `db04867ce98cee0ea9e1b189056539c1842c89a3a1a7d562072a1349c8eb9390` |
