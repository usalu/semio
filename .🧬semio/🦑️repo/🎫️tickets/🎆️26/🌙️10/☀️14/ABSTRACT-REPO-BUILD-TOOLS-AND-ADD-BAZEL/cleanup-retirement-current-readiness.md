# Exact Initial Fixture Retirement Readiness

Read-only audit of `cleanup-preflight/📜️script.ts --final-retire` against current native request and lease path/queue Source. No retirement, detach, deletion, SQLite opening, lease/custody probe, image opening, Main Tools/Main graph access or mount inventory was performed. No production or cleanup input was changed.

## Namespace and 256-Character Boundary

The wrapper passes **`leaseDirectory: budget.controlDirectory`**, resolving to the initial fixture's existing **`.cache/b/c`**. It does not pass the mounted private **`.cache/b/l/m/data`** namespace. Its resource remains exactly `native-fixed-image:<absolute initial fixture .cache/b/l directory>`, matching the retained earlier preflight metadata.

Actual current pure `resourceLeasePathV1` admitted the same SHA-256 SQLite path and all sidecars:

| Path | UTF-16 code units |
| --- | --- |
| Fixture image directory | 154 |
| Existing control directory | 154 |
| Canonical resource database | 226 |
| SQLite `-journal` | 234 |
| SQLite `-wal` / `-shm` | 230 |
| Mounted private data directory (not used) | 161 |

The current queued alternative with a valid deterministic 22-character owner would materialize a **267-character** ticket path and the actual pure guard refused it with `Materialized absolute path exceeds 256 characters`. This does not affect the final retirement: its closed request is **`wait:false`**, and the Mac backend takes the existing direct, nonwaiting exclusive `acquireResourceLease` branch, with the explicitly bounded 16384-byte database option. It does not generate a queue ticket or claim. Do not switch to the overlong queued namespace, or substitute a different short namespace that would not exclude historical readers.

The existing canonical control database was only lstat'd: regular, nonsymlink, nlink 1, **8192 bytes**, within the unchanged 16384-byte limit. No database was opened and no reader eligibility was inferred.

## Immutable Expected Image Proof

The actual current request parser accepted the closed request, returned frozen request/expectation snapshots, and preserved every input value. Bounded 120-byte nofollow receipt reading with pre/post descriptor/named stamps still yields:

- SHA-256 `cbf4906714eab0e828f8a98400c39e94508c0eab916150490813c9ec3b6c60e3`.
- Backing device **16777232**, inode **336463257**, size **66060288**, allocated bytes **66060288**.

These values match the unchanged authored request and earlier retained preflight exactly. Only receipt bytes and backing/control metadata were read; the mounted volume contents and allocation journal were not read. This is a current read-only byte/metadata witness, not retirement custody or future authorization.

## Actual Retirement Boundary

The dispatcher schema-validates the image envelope and snapshots the expected request before dispatch. The Mac implementation acquires the exact nonwait image EX before comparing expected receipt SHA and backing dev/ino/size/allocated bytes. It then retains the current receipt/image and closed filesystem/maximum envelope checks, zero-allocation-journal requirement, actual owned hdiutil mapping, normal nonforced detach, post-detach device/receipt/image checks, and a final expected-image comparison before removal. A busy EX returns false; the wrapper rejects that result and retains the image. It asserts exact path absence only after a true managed retirement and records the result in generated.

**No concrete path/API defect found. No Source change is proposed.** Actual active-reader eligibility, allocation-journal eligibility and future normal-detach success remain unknown until Root explicitly releases the genuine managed-retirement run. Root's expected SHA/device/inode proof is unchanged.

## Current Source Witnesses

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/cleanup-preflight/📜️script.ts` — `93953831ba6fd30177bcca136de33d13410cb6e94293d0ed7fe847f488e8d616`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/cleanup-preflight/🔣️.json` — `611f0edb02f4b81dc249e38a494801d22114a119940af2b35e219d66adaa1239`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/cleanup-preflight/🧬️schema/🔣️.json` — `1174cdbf4c9ce51211374c1fff0aa86be2ca3af2dcc0bcafbf3d7968de3bb49c`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` — `b985e99fb5d426e131e763be7cda63a0d7ee158290f67cf70487808be8289ed8`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧬️schema/📜️ticket.json` — `1168edc5de66acbfeb890c808dc6eb53a52d193ec47cf0c3f0d1342c8b895c21`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts` — `1093475635b685b2d9ddc5f4f04760146903d32994b31754d1b5d3962bf9d331`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts` — `93e8c873a23b4134eaf4a420eea27a231109fe78c56d2119fbde4b4f1a788139`
