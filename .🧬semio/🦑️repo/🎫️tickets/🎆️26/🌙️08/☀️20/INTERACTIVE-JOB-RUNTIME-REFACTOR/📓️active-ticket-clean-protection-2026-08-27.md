# Active Ticket Clean Protection

## Outcome

The root CleanScript now rejects removal candidates that intersect a ticket without an explicitly parsed object manifest whose own `status` equals `closed`. The same read-only guard runs before shallow candidate deduplication and immediately before the existing deletion call. No workspace cleanup or deletion was executed for this packet.

The audit in `📓️terra-artifact-retention-audit-2026-08-27.md` established that the previous implementation could remove ignored or oversized content from the open master ticket. It did not establish that CleanScript caused the historical disappearance of compiler artifacts or logs. This patch addresses that demonstrated safety gap without making an attribution claim.

## Implementation Boundary

- Root `📜️script.ts`, CleanScript `🛡️TicketProtection` subregion: a read-only filesystem view, explicit closed-manifest admission, protected-ancestor/subtree inspection, and safe candidate projection.
- Canonical and misplaced ticket-directory spellings use the existing four-level year/month/day/slug boundary. Explicit nested ticket manifests also establish ownership, including below build and misplaced-directory candidates.
- Open, missing, malformed, unknown, non-object, missing-status, unreadable, or non-file manifests protect the complete owning subtree. A closed ticket cannot authorize deletion of an open nested ticket.
- Unsafe candidates are removed before shallow deduplication, allowing an eligible closed sibling to remain independently eligible when its parent is protected.
- Workspace-root and outside-prefix candidates are rejected. Traversal never follows a symlink; unreadable or changed ancestors and unsafe child entries fail closed.
- Manifest status and candidate ancestors are rechecked after subtree inspection. The entire admission check is repeated immediately before deletion, so a planned closed ticket reopened before execution is protected.
- Git-ignored discovery and size enumeration are initiated only for explicitly closed top-level ticket folders. Existing size thresholds and eligible closed-ticket behavior remain unchanged.
- Protected results are reported as skipped paths rather than successful removals.

The immediate check is not a cross-process filesystem transaction: an external writer can still mutate a path between the final synchronous check and the operating-system removal call. No lock or atomic filesystem claim is made. Recursive safety inspection is synchronous maintenance work, not an interactive-runtime boundedness claim.

## Schema-First Tests

Language-neutral fixture and strict schema live in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧹️clean/`. The existing TypeScript library test suite consumes an entirely in-memory filesystem projection; it does not call workspace clean, write candidate files, or invoke a deletion API.

The fixture has 20 candidate cases covering open and closed content, all fail-closed manifest classes, a closed ancestor with an open nested child, an eligible closed sibling, year/build/misplaced ancestors, symlinks, unreadable paths, workspace-root deletion, and an outside sibling with a similar textual prefix. Existing third-party Ajv independently validates explicit closure and the strict fixture schema. Three malformed fixture variants are rejected.

Additional laws cover filtering before deduplication, reopening after planning and during traversal, ancestor replacement with a symlink, unreadable child enumeration, a symlink manifest, scalar/null statuses and manifests, static protected-prefix ancestors, and skipped-path reporting.

## Executed Evidence

Canonical command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/repo-lib:test --args='-t "active ticket clean protection"' --skip-nx-cache`.

- RED: one failing test before the guard export existed; captured in `🧪️active-ticket-clean-red-2026-08-27.txt`.
- Initial GREEN: 1 passed, 0 failed, 48 assertions; `🧪️active-ticket-clean-green-r1-2026-08-27.txt`.
- Final GREEN: 1 passed, 0 failed, 292 filtered, 58 assertions; `🧪️active-ticket-clean-green-r2-2026-08-27.txt`. Exit 0; test body 47.89 ms.
- Existing canonical registry generation completed with exit 0; `🧪️active-ticket-clean-launch-2026-08-27.txt`.
- Launch entry `⚖️gate🧹️clean🛡️active-ticket` is present in both authoritative `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json`.
- `git diff --check` passed after the final code and fixture edits.

No Rust compiler, actual workspace-clean invocation, process interruption, or git mutation was performed. The parent coordinator owns review and the subsequent native rebuild gate.
