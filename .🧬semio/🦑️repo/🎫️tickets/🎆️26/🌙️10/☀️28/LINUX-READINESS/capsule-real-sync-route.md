# Real Dependency Sync Acceptance Route

The narrow existing production entry is the dependency script scoped sync, not a test helper or full workspace setup:

```sh
SEMIO_LANGUAGE=en repo "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts" sync --scope .
```

The root scope `.` is an actual authored workspace owner. The command restricts generation/publication selection to that owner; it does not mean a single package, because the owner includes its declared workspace members. Passing a member package path that is not an independent owner is refused. The existing root `workspace:deps-javascript` target invokes this same script without a scope and therefore selects ALL owners; it is broader than necessary for scoped acceptance. No smaller dedicated per-package canonical target was found. Do not fabricate an Original or call the private writer with plain objects.

## Actual Ownership and Retirement

The script authenticates its received invocation through `receiveScriptProcessInvocation`; `SyncScript.run` checks it, issues the existing dependency-storage phase, enters `withJavascriptDependencyStorageV1`, and calls `prepareJavascriptDependencies` with the fixed scope selection. The private Store frame is mandatory.

Storage `runJavascriptDependencyStoreGenerationsV1` (lines 497–507) runs selected build preparation, acquires actual bindings EX custody, publishes selected bindings unless the build returned current, then ALWAYS calls `retireJavascriptDependencyPreservationsInStoreV1` before releasing custody. Thus a successful no-change sync still reaches the real retirement owner. It then performs owned generation collection.

Retirement intentionally authenticates ALL current ownership, even for a scoped publication. It reads current completed generation/binding receipts, verifies readiness fingerprints, pins generations, checks Store and Source ownership before/after, and invokes descriptor-based retirement under bindings EX. It refuses unresolved, foreign, live or unproved payloads. This all-owner readiness validation is a correctness boundary, not a reason to widen publication to ALL.

## Cost and Evidence Limits

This is an existing real Store operation; it is not guaranteed compiler/download-free merely because selected scope is small. A current installed generation can avoid installer execution; stale or absent current generation goes through the real installer/adoption path. The public sync parser offers only `--all` or `--scope <owner>`, with no no-install/check-only switch. A strict guarantee of no download cannot be obtained by inventing an offline flag or bypassing the installer. Root should run this route only when actual current scope readiness is established by existing supported evidence, and retain its normal refusal if stale.

The fastest sync-current receipt shortcut is restricted to ALL selection, so scoped sync still observes its actual owner and completes the production publish/no-change path. The retirement report is returned privately but the sync wrapper does not print retired/retained counts. Successful exit proves the call settled; a separate no-follow Source census proves current exact capsule/journal counts. Do not claim a no-op sync alone forced a new preservation/exchange event: with existing correct links it legitimately creates zero capsules. Current expected Source counts remain zero from the independent census, not a newly executed sync.

For explicit retirement-only maintenance, the existing `retireJavascriptDependencyPreservationsV1` requires the same genuine private Store writer and bindings EX; ordinary root clean already owns that production delegation. It does not exercise dependency publication and therefore is not a replacement for this scoped sync acceptance.

No sync, test, Native operation, process/lease command or deletion was executed during this read-only audit.

## Actual Route Correction 2026-10-29

The previously suggested direct dependency script invocation failed terminal1 before Store acquisition: `Original parent Script process invocation required`. Its receiving-only main correctly requires an already issued parent, and the script-path CLI route does not supply one. This refusal was retained; no envelope was constructed or guard altered. The declared public `repo dependencies sync --scope .` entry is being exercised instead. The earlier direct-script recommendation is not a valid standalone acceptance command. Generated log: 🗑️generated/capsule-real-root-sync.log.

## Lost Observation 2026-10-29T20:47:04.167Z

The old API handle62135 disappeared on tool reconnect. Its actual child1534497, logger1534490 and outer1534454 were initially revalidated live, with genuine Source admission and Store build-custody waiting. They are now 1534497:ENOENT, 1534490:ENOENT, 1534454:ENOENT. The log ends in custody waiting without terminal status; there is no successful sync claim. Exact matching current processes:0. No process was signalled or lease changed by this census. New logger invocations persist observed child-close receipts; that cannot retroactively establish this old operation's outcome. A fresh acceptance run must follow the current publication checkpoint after authoritative no-live confirmation.

## Real Publication Checkpoint And Independent Census

The course operation owner reported genuine dependency preparation preserving2 Source endpoints, followed by a successful switch of 🏢️semio-tech to dependency epoch5 at20:44:39UTC. Subsequent normal generation collection reported6 superseded generations retained because they were pinned/recent/rollback targets. Its official bindings holder observation then became empty. The operation's outer issuer1504845 remained live; full sync and UI test success were not claimed. No manual capsule cleanup, lease reset or forced release was performed for this observation.

The independent Root Source census at2026-10-29T20:48:30.652Z subsequently visited138333directories with411symlink skips,61protected/generated exclusions and0filesystem errors. It found **0capsules and0retirement journals**, retaining .prettierignore and .prettierrc.json. This supplies a real post-publication Source hygiene observation in addition to the prior focused retirement laws. Kept dependency generations are separate from Source preservation capsules; the six kept generations were neither deleted nor counted as capsule pollution. This does not certify completion of the enclosing sync, setup, UI or full Linux goal.

The separate Root scoped sync lost observation on tooling reconnect and later all exact processes disappeared without terminal receipt. It remains outcome unknown, not successful. New command loggers now persist exact observed child-close outcomes; three neutral actual-command cases passed33assertions with Ajv, including status0,7and frontend143 for an owned self-signalled Node payload. See typing-logger-outcomes.md.
