# Closure Retention Recovery Audit

This read-only audit covers the recovered plugin ledger and the ticket-private close scripts. It did not execute the private MCP build or a ticket lifecycle call.

## Exact Ledger Recovery

The restored [plugin authored file manifest](./📓️plugins-authored-file-manifest-2026-09-09.md) is byte-identical to its indexed source at the former `ASSETS-FIXTURES-SEPARATION` path:

- Git blob: `c8e1f7a5565bee64158b52a8b22c320e09291ae2`
- Bytes: `13,622,727`
- SHA-256: `83c55435bec5d6d3dccb636f71ca406bf2ff961f17b8237f5af77a4a29429b68`
- Records: `17,252` — 21 creates, 3,409 modifications, and 13,822 moves
- Unique nonempty paths: `29,848`

The Markdown has one JSON array and two fences. Every record has the eight required fields and valid optional SHA-256 values; no duplicate record, directory path, generated path, or `AGENTS.md` path was found. The independently retained [recovery record](./📓️plugins-authored-file-manifest-recovery-2026-09-09.md) reports the same blob identity and counts.

## Retention Overlay And Restore

`🧑‍💻coordination/🔌️ticket-mcp/📜️script.ts` copies only private generated overlay files, asserts the unmodified repository constants, and builds Go with that overlay. It raises the MCP request ceiling to 32 MiB and both close-time purge thresholds to `1 << 60`; repository source files are not written by this script. Its final-close title is the exact indexed original, `Assets Fixtures Separation`, whose slug restores `ASSETS-FIXTURES-SEPARATION` through the repository title-update rename. Current-ticket paths are converted from the temporary `ASSETS-AND-FIXTURES-SEPARATION` prefix before submission.

The final restore route now explicitly requires open ticket metadata before it collects files. The former closed-ticket reopen branch has been removed, so no cached pre-rename ticket directory can be used in the submitted path collection. The current-prefix conversion remains bounded to the temporary ticket prefix and is applied before the title-driven folder rename.

## Close Verification And Cleanup

`🧑‍💻coordination/📋️ticket-close/📜️script.ts` requires the restored title, exact overview summary, a deduplicated 30,000-plus path array, exclusion of generated paths and `AGENTS.md`, closure-input SHA-256 agreement, resolved overview links, and removal of the important-marker preimage. Cleanup excludes `🗑️generated`, hashes retained files, removes generated output, then verifies the captured retained hashes.

Cleanup now records SHA-256 snapshots for every retained non-generated file, removes generated output, and verifies every recorded byte sequence. It then appends the lifecycle completion statement and independently compares the exact final on-disk lifecycle bytes with the exact completion bytes written. This covers the earlier lifecycle-report gap. Static review found no remaining issue in the final open-state restore, retention overlay, close verification, or cleanup safeguards.

## Coordinator Incident And Execution Record

The first actual repository close invoked `purgeOversizedTicketArtifacts`: any file above 5 MiB or subtree above 10 MiB was removed, without an exemption for inputs or Markdown. This deleted the generated directory, full closure JSON input and large plugin path ledger. The supplied title also changed the ticket slug to `ASSETS-AND-FIXTURES-SEPARATION`. The ticket was reopened solely to restore its original title and retain the completed evidence. No fixture source work changed, and the final 114,552-path zero-finding scan remains the completed source verification.

The exact indexed ledger was recovered read-only; no content was reconstructed or invented. The [retained-evidence loss audit](./📓️retained-evidence-loss-audit-2026-09-09.md) confirmed the remaining scripts and report chain. The small plugin-followup JSON was intentionally removed as generated output, with all 41 paths already retained inline in Markdown, and was not restored.

The coordinator ran the protected Go build and actual MCP tools probe successfully. The recovered collector then ran successfully with 32,050 paths before normalization of the temporary ticket prefix, and zero directories. The complete close request is regenerated from those exact ledgers into retained Markdown. The original title, final close, exact summary, input SHA-256, report links and byte-preserving cleanup are verified in [ticket lifecycle completion](./📓️ticket-lifecycle-completion-2026-09-09.md). The private size-limit overlay preserves the inputs and reports explicitly required by the user; repository lifecycle sources remain unchanged, and generated outputs are removed explicitly after verification.
