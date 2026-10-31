# Independent Directory Anchor Native Publication Audit

Read-only source audit, 2026-10-29. No builds, tests, cache operations, anchor effects, shared payload reads or live publication were performed. The ticket remains under its owning agent.

## Finding: Unknown Trailing Intent Records Are Deleted

**P1 — retain/refuse is bypassed by the initial-marker fallback.**

In native-build storage publication `🎛️owner/⚓️envelope/🟦️.ts:163–164`, a journal decode failure falls back to parsing only the first line as an initial marker. It rejects the second line only when its parsed purpose equals `witnessed-directory-envelope` or `prepare-current-directory-envelope`. A valid canonical observed-only initial header followed by `{"purpose":"foreign-record"}\n` therefore supplies a nonnull marker to `resetInitialDirectoryMetadata`.

That reset attests entry shape, but does not establish exact initial-only intent bytes (`130–136`). The neutral inspector accepts equal-current bounds whenever a matching marker is supplied (directory envelope codec `109–116`). The reset then removes the intent and any bounded receipt entry, and removes the namespace. Thus foreign/unknown trailing records are erased, including at equal-current, despite the stated unknown-record retain/refuse contract.

The same branch also ignores extra trailing lines after a valid first-line marker. A concrete correction must require the entire record to be precisely the canonical observed-only marker before using its equal-current exception; unknown suffixes must retain/refuse. A neutral malformed/unknown suffix vector and actual owner recovery test should cover this branch. This audit did not execute the reproducer.

## Source Controls Confirmed

Paths below are relative to `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`.

- `📤️publication/🎛️owner/⚓️envelope/🟦️.ts:33–43` requires actual issued control refinement, accepted physical cleanup enrollment, genuine Native writer proof and stable physical ancestry. Envelope handles are private WeakMap entries (`27–31,45`); persisted DATA does not issue that handle.
- `📤️publication/📚️witness/🟦️.ts:18–23` binds the exact captured witness object to control, path and root identity through a private WeakMap. Copied serialized witnesses cannot pass the actual-current requirement.
- Cold preparation writes and awaits the complete bounded before-witness baseline before descriptor stage preparation (`owner/⚓️envelope:79–85`). The prepared hook appends durable actual stage facts before the SDK permits target mutation. Anchor publication uses real captureFileRecovery/recoverFile (`89–92`).
- The main owner captures one directory runtime Source and retains its cleanup entry (`owner/🟦️.ts:84`), then uses that cleanup entry for cleanup commands (`87`). The general backing identity guard remains strict (`63`); the accepted new tuple comes from the actual SDK result (`owner/⚓️envelope:89–97`).
- Fresh EX recovery settles a pending defining-directory swap before mapping observation (`owner/⚓️envelope:160–161`). Cleanup similarly settles that swap before original-volume equality (`95–97`).
- Original exact volume identity is explicitly compared in cold commit (`91`), cleanup (`96`), qualification (`106`) and fresh recovery (`170`).
- The neutral codec carries complete Windows parent/prior/stage supplemental metadata, reconstructs it in decoding, and canonical-reencodes the whole journal (`📁️directory/⚓️envelope/🟦️.ts:91–104`). Actual unprepared metadata retirement compares Windows parent/prior facts and repeats same-entry full attestation before removal (`owner/🧾️metadata/🟦️.ts:57–65`).
- Missing-canonical baseline recovery selects a complete agreeing unprepared keeper, freshly attests current anchor/platform facts, removes only the other reserved entry, publishes and fsyncs/readbacks canonical intent, then removes the keeper (`owner/⚓️envelope:140–155`). It does not adopt a historical stage inode as authority. The namespace is capped at two entries.
- Qualified retained-anchor cleanup delegates the completed recovery shape to the descriptor API; the descriptor implementation explicitly supports `completed`. Missing retired old metadata after a qualified cleanup interruption is therefore not itself a demonstrated defect.

## Verification Limits

Read `dependency-layout-working-files.json` (37 paths) and `directory-envelope-publication.md`, the four primary anchor/owner/metadata modules, defining witness provenance, descriptor completed-state support, and the genuine private fixture/vector inputs. Applicable root AGENTS.md and build skill were read. No applicable deeper AGENTS.md exists beneath the primary framework process sources.

The ticket reports normal54391 GREEN (12 tests, 242 assertions), followed by new original-volume guards. That earlier result does not execute the final changes. Final canonical strict60686 and genuine declared private fixture remain gates; no new pass is claimed here.

The declared private fixture has real issued controls and actual image funding, but its current cut vectors are progress callback failures at observed, witnessed, staged, published, attached and verified. This is source evidence of intended coverage, not evidence that every descriptor-internal partial append, Windows old-displaced rename or process-death cut has been executed. The fixture remains unexecuted according to the owning ticket.

Mac initial-helper and root-unready audits are separate and excluded. No conclusion here authorizes live effects or makes SourceReady true.

## Focused Recheck: P1 Closed in Current Source

Read-only recheck on 2026-10-29; no builds, tests or live effects executed.

The previously reported P1 unknown trailing intent record deletion is **closed in the inspected source**. The actual defining function is `readNativeDirectoryInitialRecoveryV1` (directory envelope codec lines 145–154). The Native owner fallback calls it before `resetInitialDirectoryMetadata` (`owner/⚓️envelope/🟦️.ts:164`).

The inspector rejects any LF that is not the final byte, so a valid typed marker followed by unknown, known, partial, foreign or additional newline suffix cannot reach reset. A parsed preserving-purpose first record refuses. A parsed typed initial header without its LF refuses; a typed header with extra fields fails its closed schema; and a complete header must reencode to exactly the original canonical bytes. UTF-8 roundtrip is also checked by the actual owner before inspection.

Only the exact complete canonical observed marker returns a nonnull marker. Opaque single-record or partial predecessor DATA returns null, preserving the existing strict-predecessor-only reset role. That null marker still hits the equal-current refusal in `inspectNativeDirectoryInitialMetadataV1:113`; it cannot use the exact-marker exception. This distinction intentionally retains the earlier predecessor reset policy rather than expanding equal-current admission.

The new neutral test at `📁️directory/⚓️envelope/🧪️tests/🟦️.ts:195–200` includes unknown suffix, equal-current unknown suffix, known/partial witness suffix, extra LF, extra field, foreign prepared first record and missing-LF marker cases, with independent SQLite classification. Execution results are reported by the owning lane: full normal66553 GREEN 13 tests/272 assertions and targeted17656 GREEN 1 test/29 assertions after tightening missing-LF handling. Root reports canonical ordinary strict41640 EXIT0 across 14 roots. These results were not rerun by this auditor.

No remaining concrete finding was established in this focused fallback recheck. The original audit's **genuine private fixture remains pending** limitation still applies. Neither source closure nor the reported neutral/strict passes establish successful private image publication or authorize shared live effects. Root capsule durability work is separate and was not inspected here.
