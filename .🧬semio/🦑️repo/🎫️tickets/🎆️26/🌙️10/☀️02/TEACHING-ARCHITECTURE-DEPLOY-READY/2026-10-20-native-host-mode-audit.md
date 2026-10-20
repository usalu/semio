# Native Provisioning Host Mode Audit

Read-only review of the current retained helper and fresh Linux entry. No command, input edit, transfer, guest action or Source change ran.

## Concrete Windows Refusal

`🧰️native/script.ts:11–13` exports `stableRead`. Its executable admission at line12 requires a nonzero POSIX execute bit on every host: `executable && (named.mode & 0o111n) === 0n` refuses the file. The host pin path at line71 and install initial/checkpoint reads at line72 pass `true`. Fresh provisioning imports that same reader (`🌱️linux/entry.ts:12`) and passes `true` for host `🗑️generated/n3/repo` at line44. A legitimate Windows-host Linux ELF artifact with no meaningful POSIX execute bit is therefore refused before transfer, even when its bounded bytes, SHA and ELF identity agree. This is the same category as the artifact-copy host-mode mismatch identified independently.

The entry’s reviewed-input snapshots at line20 use actual mode only as stable identity; they do not compare it to a logical expected permission. That evidence should remain unchanged. The reader’s descriptor/name identity comparison at line13 likewise includes actual mode to detect replacement or mutation, and should remain unchanged.

## Minimal Boundary

The executable-permission predicate should admit Windows host observations (`process.platform === "win32"`) while retaining the actual execute-bit check on other hosts. This matches the existing publisher platform policy supplied by Root; its exact Source reference is being coordinated separately. No receipt should relabel Windows observed mode as POSIX mode. The bounded ordinary single-link/non-symlink check, special-bit policy, stable descriptor/name identity, SHA and ELF checks remain necessary.

The same function is serialized into the actual Linux guest by `guestProgram` at line57. There, `process.platform` is Linux, so the strict executable check remains effective. Guest receive still explicitly writes/fchmods0700, and both receive/probe require observed guest permission0700 (`448`); those checks must remain exact. Fresh entry line44 independently verifies received and probed guest mode448. The task does not justify weakening guest chmod or final Linux mode verification.

## Required Review Before Reopening Inputs

No held input was edited. A correction would change the native helper authority pinned by fresh entry admission; Root must coordinate the native4/controller12 closure and any live frozen33 dependency before resealing or retrying. A narrow neutral matrix should cover Windows host mode0644 accepted for executable admission, Linux/macOS0644 refused, Linux/macOS0755 accepted, stable actual-mode mutation refused, and Linux guest wrong0700 refused. Existing ELF/hash/descriptor vectors remain applicable. These are proposed cases, not executed results.

## Current Read Roster

- `🧰️native/script.ts`: 17611 bytes, SHA256 `3dc827e66bf8de7f1ec66109dba59668eef09bd9602125f205f12e8d74a0f2ba`.
- `🌱️linux/entry.ts`: 25837 bytes, SHA256 `7d56cc717ff38539411124e55579064d56fa96d3ecaf8d9ff04a36442350909c`.

Confirmed existing owner reference: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🟦️.ts:45` privately defines `modeMatches=(actual:bigint,expected:number)=>process.platform==="win32"||(actual&511n)===BigInt(expected)`. This is existing platform policy, not an exported helper to import into the ticket.
