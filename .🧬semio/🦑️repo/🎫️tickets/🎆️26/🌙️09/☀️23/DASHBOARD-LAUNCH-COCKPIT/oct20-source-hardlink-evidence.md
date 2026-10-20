# Authored Source Hard-link Refusal

Read-only investigation on 2026-10-20. No Source file, foreign generated mirror, guard, lease or process was modified.

## Located Publication Evidence

The neighboring ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/coordination.md` records a 15:47 alert identifying an x-inputs hard-link mirror at `🗑️generated/x-inputs/mirror`, approximately 1580 canonical files with link count two, and the same Source revision reader refusal. A 15:50 entry and result-x-dev.md/result-x-proctor.md corroborate the startup blocker. These reports identify the stated publishing task; they do not authorize changes to it.

## Current Physical Proof

Node built-in fs.lstat with bigint metadata was used on both canonical and mirror pathnames. All three pairs are ordinary files, not symlinks, on device 16777232 and have link count two. The exact current inode matches prove that the mirror is a second pathname for these current authored Source files.

| Source relative path | Inode | Bytes | Same canonical/mirror identity |
|---|---:|---:|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs` | 380862593 | 7659 | true |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🧬️schema/🔣️.json` | 380839183 | 3017 | true |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧬️schema/🔣️.json` | 437761632 | 9954 | true |

The generated evidence file hardlink-mirror-readonly-evidence.json contains the exact canonical/mirror root and both metadata observations. This establishes the current second pathname independently of the reported task history.

## Scope And Recovery Boundary

The Source revision guard requires a single-link regular file before opening, and the newly refused installed command reached that guard before its help or maintenance body. No legitimate authored Source hard-link exception was identified. The evidence supports correcting the actual mirror publisher/its generated artifact while preserving canonical Source and other work; it does not support weakening the guard, guessing a process owner from a lease PID, unlinking unknown paths, or blanket rewriting canonical files. Parent owns recovery and any further coordination. No message was sent to another app chat.

## Follow-up Physical Observation

A later read-only Node lstat observation found all three canonical paths restored to link count one while retaining the exact original device and inode values listed above. This controls agent performed no mutation and does not attribute the repair to a process without further evidence. Actual installed help and guarded maintenance runtime acceptance remain required.
