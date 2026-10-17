# Independent Native Private Publication Proof — 2026-10-17

This auditor parsed the existing private run log and its three retained Node trace receipts, read the private helper source, computed Source/log/trace SHA256, and checked the retired directory is absent. No native operation, mount, tooling store access, or build was launched by this auditor.

## Exact Evidence

- Log: `🗑️generated/coordination-17/native-private.log`, 81 complete lines, SHA256 `6935c8198552f2bba5f97ebb6d9a62c8c911d981334c0533ceffa68fca3dd0ee`.
- Witness Source: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts`, SHA256 `7c5a4a1af131d36077e855613cfc5288e5c040b2141ea01efd8a10dde47e50d2` at audit time. This Source identity is distinct from the runtime witness digest.
- Fixture base: `🗑️generated/np-YD5rBG`. The parent `p` is absent at audit time. The log ends with the exact canonical parent-retirement success marker, after qualification.

## Parsed Runtime Proof

All ten stage events occur exactly once and in order: accepted, observed, witnessed, detached, underlying, staged, published, attached, verified, settled. Both the helper progress hook and implementation stage lines are present. The qualified record follows settled; parent retirement follows qualification. No error line appears in this log. Parent reports process terminal status 0; the log itself does not embed an exit-status receipt, so this auditor independently establishes complete success-path markers rather than claiming to have observed process termination.

Both independent payload receipts identify Node v24.15.0 and SHA256 `62973580cb0d8f44b830898e019c50399ddc804352fbade1f66ee187e9617871`. An independent Python hashlib calculation over the helper-authored 65,536 bytes of value 91 gives the same digest. Helper source checks Node status 0, no signal, runtime identity, and digest before logging each receipt.

The three retained Node trace JSON files parse. Their labels are before, after, reopened. Every trace contains exactly 23 hardlink alias files with one inode and nlink 23, and a separate same-content independent file. Logged hardlink relation SHA256 is identical at all three points: `2c76a6d281e75f02ed8d6511c38aed521ee0a197b52206bf10d1a8906adcf495`. Helper source checks complete relation equality across labels, rather than merely digest equality.

| Trace | Records | Logical bytes | Retained trace SHA256 |
| --- | ---: | ---: | --- |
| before | 35 | 66222 | `5ec724026f71d67f361ed086b0d39b19223ba3a008c2cc3d397e57522f915a13` |
| after | 37 | 66649 | `e3f7bb2a7df3e716525c183efaa2ce8d9bd4175f59c6b59e32cb3f3b0c216fa8` |
| reopened | 37 | 66649 | `e3f7bb2a7df3e716525c183efaa2ce8d9bd4175f59c6b59e32cb3f3b0c216fa8` |

The exact after/reopened trace bytes match. Before differs because service content changes; equality of the entire before/after trace is neither asserted nor inferred.

The parsed qualification is exactly `mounted-application-preserved`, with sockets 1. The final record affirms hardlinksPreserved 23, independentFileDistinct, socketMetadataPreserved, rawJournalPreserved, ordinaryReopen, childAbsent, and parentHealthy. Source requires exact socket metadata, raw base64 journal equality, payload bytes, symlink target, zero-length file, SQLite empty-journal projection, ordinary child reopen, canonical child retirement, parent recheck, canonical parent close/retirement. Those markers occur only after the checks. The final parent marker additionally requires directory absence.

## Scope and Outstanding Gates

This proves the specific disposable macOS private fixture's physical publication success path, including remount and ordinary reopen. It does not prove full SDK acceptance, public graph/cache admission, Windows/Linux runtime behavior, or cancellation/recovery paths. Full serial publication/witness Source gates failed their 55-second original resource deadline; the earlier witness-only run passed 27 tests with 2,842 assertions and strict TypeScript returned 0 as reported by the parent. Neither success should be conflated with a passing combined serial gate. The private run does not override those gate failures.
