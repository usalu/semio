# P2-D Global Artifact-CAS Delete Barrier — Final Audit

## Verdict

**ACCEPT — within the evidence scope below.** The P2-D barrier now establishes the required safety law for the exercised independent-process topology: a physical CAS object referenced by a successfully published checkpoint cannot be removed by an older sweeper. This replaces the earlier rejection, whose material blocker was the absence of a real successor reserve → advance → stage → publish race.

The acceptance is intentionally narrow: SQLite-directory plus filesystem-CAS behavior is runtime-tested; PostgreSQL, Neo4j, and Windows behavior are source/compile reviewed only. No conclusion here implies a runtime result for an unavailable backend or platform.

## Exact invariant

For each `space`, the directory owns a durable, monotonically advancing CAS epoch. A sweeper may physically delete only while its durable delete fence remains current. Before deletion it acquires the fence, advances the CAS epoch, renews it, and performs a final validation under the same directory writer authority. Reservation advances the epoch before staging; publish installs the canonical reference atomically. Therefore, if a successor reserves, advances, stages, and publishes after an old sweeper's final validation, the old fence cannot match the filesystem object's epoch and deletion fails closed with `AuthorityError::Store("artifact CAS deletion fence is stale")`; it must not delete the successor's bytes.

The law does not rely on a process-local mutex, cross-backend transaction, locator exposure, or generic `PayloadStorage` behavior.

## Final source evidence

### Durable ordering and error preservation

`🌎️hub/📇️directory/🦀️.rs` implements the execute ordering in `DirectoryService::sweep_artifact_cas`: acquire fence → advance CAS epoch → renew → final directory validation → conditional CAS delete → release. Dry-run takes the preview path and does not acquire a fence or advance an epoch.

The final deletion/release handling preserves the primary CAS result: when `delete_if_unreferenced` returns the stale-fence error, best-effort lease release cannot replace it with a release-conflict error. This is necessary for the exact stale-fence oracle to prove the physical fence, rather than merely any failed cleanup.

The directory implementations keep the coordinator state with the directory, rather than with the independently selected CAS backend:

| Directory implementation | Reviewed barrier state |
| --- | --- |
| memory | Per-space epoch/fence state and expiry semantics |
| SQLite | Durable per-space lease/epoch rows and conditional acquire/renew/validate/release |
| PostgreSQL | Equivalent durable lease/epoch queries and conditional transitions |
| Neo4j | Equivalent per-space lease/epoch node transitions |

The final all-feature compile gate covers the feature-backed code; it is not a PG/Neo runtime proof.

### Filesystem conditional-delete fence

`🌎️hub/🗿️artifact-authority/🗂️chunk-cas/🦀️.rs` contains one `ArtifactCasFileFence` definition and enforces the expected epoch against the CAS leaf before unlinking. Unix opens leaves atomically with `O_NOFOLLOW | O_CLOEXEC`, then validates descriptor identity against `lstat`; Windows opens with `FILE_FLAG_OPEN_REPARSE_POINT` and rejects reparse points. The Unix test also checks close-on-exec. This removes the previous duplicate-definition and path-following concerns. Windows execution remains untested in this audit.

### The required independent process race

`artifact_chunk_cas_filesystem_process_sweep_and_publication_race_preserves_exact_bytes` in `🌎️hub/📇️directory/🦀️.rs` is now the primary race proof. It is not the older raw epoch-only filesystem subprocess test.

The parent creates a unique on-disk SQLite directory and filesystem CAS root. It launches two distinct test-binary children, each opening its own SQLite connection and filesystem-CAS handle:

1. The **old-sweep child** begins execute-mode sweeping of an expired, staged object. Its test wrapper signals only once `delete_if_unreferenced` is entered — after fence acquisition, epoch advance, renewal, and final directory validation — then pauses before the real filesystem delete.
2. The **successor-publication child** independently performs the complete reservation → coordinator/epoch advance → pack and SPR staging → atomic checkpoint publication sequence, then reads the exact pair bytes.
3. The parent waits for the successor to finish, releases the old child, and requires `expect_err` with the exact stale-fence message above. The old child therefore cannot pass because of an arbitrary release error.
4. The parent opens fresh directory and CAS handles and verifies the published checkpoint plus the exact pack and SPR bytes.

The synchronization has a bounded watchdog and cancellation-aware marker wait. The successor starts only after the old child has reached the post-final-validation pause; the old child resumes only after the successor finishes. Thus the test exercises precisely the formerly missing stale-delete versus full-successor-publication window.

The prior `artifact_chunk_cas_filesystem_process_epoch_fences_stale_delete` remains useful as a lower-level filesystem epoch check, but is **superseded as the acceptance proof** because it does not itself perform the full successor reservation/stage/publish sequence.

### Neutral schema oracle

`🌎️hub/🗿️artifact-authority/🧪️fixtures/🧬️artifact-chunk-cas/🔣️.json` describes both delete-first and successor-first orders. `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts` independently evaluates the successor-first sequence: expired old lease → successor reservation/epoch advance → stage → publish → old delete rejection → exact read. It is a language-neutral model oracle complementary to, not a substitute for, the Rust two-child process test.

## Attributed gate evidence

I did not run builds or tests. The following are final-source results reported by the Sol lane and parent, and were rechecked against the source/test shape above:

| Gate | Reported result | What it establishes |
| --- | --- | --- |
| Focused two-child process race, session `87775` | Exit `0`; parent `1 passed/53 filtered`, each old/successor child `1 passed`, `0.38s` | Separate SQLite/FS actors execute the complete stale-delete race; old child gets the exact stale-fence error and fresh parent handles recover exact bytes. |
| Independent TypeScript oracle, session `6425` | Nx success; `1 passed/9 skipped`; focused `147ms`, total `1.37s` | Schema fixture and independent Node model agree with the successor-first ordering. |
| Final P2-D Nx gate, session `52100` | Nx success: 16 CAS laws passed/38 filtered in `7.92s`; maintenance checkpoint `1 passed/32 filtered` in `0.19s`; all-feature `os-hub` check `8.10s` | The focused packet includes both process laws, retention/rebuild, continuation beyond 4,128, cancellation/max+1, memory/SQLite/FS behavior, maintenance checkpoint, and feature compilation. |

## Earlier rejection closure

| Earlier concern | Final status |
| --- | --- |
| Only a raw filesystem epoch subprocess test, no real successor publication | Closed by the two-child SQLite+FS test above. |
| Release conflict could obscure stale conditional-delete result | Closed: deletion error takes precedence over best-effort release. |
| Duplicate filesystem fence definition | Closed; one definition remains. |
| Filesystem leaf could follow a symlink/reparse path | Closed in source: Unix no-follow descriptor/identity validation and Windows reparse rejection. |
| Maintenance state dropped after 16 batches/ticks | Closed structurally: checkpoint survives the bounded inner batch and is retained across supervisor iterations; reported focused checkpoint gate passed. |
| Dry-run could mutate fencing state | Closed by preview-only dry-run path and the focused CAS laws. |
| No neutral independent oracle | Closed by the fixture-driven TypeScript/Node oracle. |

## Residual evidence limits (not acceptance blockers)

- PostgreSQL and Neo4j have reviewed source/feature compile coverage only; no live service or two-process runtime race was reported.
- Windows reparse protection is source reviewed but not executed here; the reported filesystem runtime race is Unix-host evidence.
- The maintenance checkpoint test directly drives continuation. It does not constitute a long-duration timer, shutdown, or injected-transient-failure runtime soak.
- The neutral TypeScript oracle models ordering; the independent-process physical proof is the SQLite plus filesystem-CAS child race.

## Decision

Accept the P2-D global deletion-barrier repair for integration, with the stated runtime scope. Later backend/platform qualification should add live PostgreSQL/Neo4j and Windows executions of the same shared fixture and two-actor law; it must not weaken the durable per-space fence or introduce a process-local locking fallback.
