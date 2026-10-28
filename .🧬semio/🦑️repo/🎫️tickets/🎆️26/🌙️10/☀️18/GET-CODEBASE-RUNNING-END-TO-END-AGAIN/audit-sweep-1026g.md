# audit-sweep-1026g: what deletes ticket `🗑️generated` and ticket subtrees

Auditor aud-sweep-g (read-only). Ticket C = GET-CODEBASE-RUNNING-END-TO-END-AGAIN. No source edits, no git-modifying commands, no `bun repo` runs, ticket not closed. Times are local (CET = UTC+1).

## Verdict

Top cause, proven capability and fits both events: `repo ticket purge-artifacts` with no argument or with `--all`. Every implementation walks every ticket folder, RemoveAll-s any directory above 10 MiB (not only under `🗑️generated`) and removes files above 5 MiB, with no open/closed-ticket check. Second candidate: `repo clean` (workspace-cleanup) can remove a non-generated subtree of an OPEN ticket; it cannot remove C's `🗑️generated` while C is open. Manual `rm` is suspected only; no note or script shows it. Who ran the purge, and when, is unproven.

## Timeline evidence

| time | evidence |
|---|---|
| 17:16:16 | `🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT` dir mtime = 17:16:16 (an entry was removed). The removed entry is `🪓️monolith-split`: HEAD holds 8,024 files, 43,460,490 bytes, zero files above 5 MiB. The index no longer lists them (staged deletion), HEAD still has them. |
| 17:16:16 ticket doc | CLEAN ticket `🎫️ticket.json` still has mtime 10-18 04:16 and `"status": "open"`. FinishTicket saves ticket.json before it purges, so no close or reopen touched CLEAN at 17:16. |
| 17:45:11-17:45:22 | Earliest birthtime of any survivor in C/`🗑️generated` is 17:45:22 (`e2e-orch`), nx run id 20261026T164511 = 17:45:11 local. Every lane folder that existed before is gone. Sweep time is at or before ~17:45. |
| 17:48 | coordination.md 17:48 names `🗑️generated/chain14/` as the log dir. Absent now (`chain-tools/chain14.sh` exists). Removal time unknown, after 17:48. |
| 18:12:10 | Last write to `nx/` (119 MB; trace.zip 54 MB at 18:11:13 and 52 MB at 18:11:44). nx is still present at 19:05, so no size-based purge has run on C/`🗑️generated` since ~18:12. |
| 18:21:00 | C/`🗑️generated` dir mtime. Coincides with the birth of `oracle2-g/` (18:21:00), so it hides any removal time. |
| 18:35 | coordination.md cites `🗑️generated/round2-apply/`. No such folder in C or anywhere under tickets (find maxdepth 9). Unresolved. |

## Candidate causes, ranked

1. PROVEN CAPABILITY, fits 17:16 and 17:45 (purge with no open/closed guard).
   - Go ticket package, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🐹️go/🔓️finish-reopen/🐹️.go`: `PurgeOversizedTicketArtifacts` walks the whole ticket folder (absDir = ticket folder, not `🗑️generated`), RemoveAll on any subdirectory over 10 MiB (deepest first), os.Remove on files over 5 MiB except ticket.json. `PurgeAllOversizedTicketArtifacts` walks all ticket roots.
   - Entry point: `🧰️framework/.../⌨️cli/📦️packages/🐹️go/🎫️ticket/🐹️.go` (purge-artifacts, lines ~410-441): no args or `--all` calls PurgeAll.
   - Rust: `🎫️tickets/🔓️lifecycle/🦀️.rs` `purge_oversized_artifacts` (line 705) walks via `collect_real_tree` (line 757), the whole ticket folder. Verb `⌨️cli/🧭️repo-cli/🚦️verbs/🎫️ticket-verbs/🦀️.rs` `ticket_purge_artifacts`: no args or `--all` purges every ticket.
   - Go client variant, `💻️client/⌨️cli/🧹️ticket-purge/🐹️.go` (used by `🎫️ticket-command/🐹️.go` 465-495): walks only `🗑️generated`, size-only, no open check. Explains the 17:45 lane wipe alone.
   - Fit 17:16: monolith-split is one non-generated subtree above 10 MiB with no file above 5 MiB, so one RemoveAll removes exactly the 8,024 files, and the parent mtime is 17:16:16.
   - Fit 17:45: all pre-17:45 lane folders gone, all survivors born after 17:45, nx (above 10 MiB) untouched since 18:12.
2. PROVEN CAPABILITY, fits 17:16 only: `repo clean` (workspace-cleanup).
   - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts` `cleanTicketSizeRemovals` (line 179) removes ticket-file over 5 MiB and ticket-dir over 10 MiB in every ticket, open or closed.
   - `🗑️removal/🟦️.ts` `cleanRemovePath` (about line 39): `allowedOpenTicket` bypasses ancestor protection for ticket-dir and ticket-file. `🛡️protection/🟦️.ts` `cleanOpenTicketGeneratedOutputProtection` (about line 203) still protects anything inside an open ticket's `🗑️generated`, so C's lanes are safe; monolith-split (non-generated, open CLEAN ticket) is not.
   - Only caller is the `repo clean` command (`🎮️command/🟦️.ts`). No automatic invocation found.
3. SUSPECTED, no evidence: manual or lane-script `rm`. Lane scripts: only `keys3-g-tools/batch/verify.sh:8` removes `$C/🗑️generated/keys3-g/applied` (its own dir, narrow); `churn-c-tools/lock-law/run.sh:11` removes its own temp dir. No script removes `$C/🗑️generated/*`.

## Ruled out

- Test-output retention `🧪️test/🗑️retention/🟦️.ts` (`cleanTestOutputs`, `collectGarbage`): test-output cache root only, marker-guarded.
- Cache GC and `runCachePrune`: cache roots only.
- Go `:test` routes: proven for creates (fixture tickets, proof2-g), not for deletes. `🎫️tickets/🧪️tests/🔓️open-close-reopen-lifecycle/🐹️.go` (`anOversizedArtifactIsPurged`) runs on an in-memory store (`newWorld`, `SeedSizedFile`). No `*_test.go` calls a purge or PurgeAll on disk.
- `TicketService.Delete` (`🎫️tickets/📦️packages/🐹️go/🛎️service/🐹️.go` ~429, RemoveTree of the whole ticket): would remove ticket.json; it exists.
- files-plan move (`🗂️files-plan/🐹️.go` ~273, Rename or CopyDirTree plus RemoveAll): would leave a copy under the ticket; none exists (only an unrelated `monolith-census`).
- FinishTicket close purge: would only touch the closed ticket; neither CLEAN nor C was saved since Oct 18.

## Proven vs suspected

- Proven: the purge code walks whole ticket folders with no open/closed or generated-dir guard; the clean ticket-dir bypass; the open-ticket generated protection in clean; 8,024 files = a HEAD subtree above 10 MiB; the open status and unchanged ticket.json of CLEAN and C; nx survival bounds the last purge to before 18:12.
- Suspected: a `purge-artifacts` run (no args or `--all`) at 17:16:16 and at ~17:45; a later removal of chain14 (after 17:48) and maybe round2-apply.
- Unproven: the invoking process and binary (Go or Rust); no lane note records it.

## Fix proposal

1. Keep one purge implementation (AGENTS: repeated code must sit together). Scope it to `🗑️generated` of CLOSED tickets only, mirroring `cleanTicketGeneratedOutputRemovals`. Remove the whole-walk variants (Go finish-reopen PurgeAll, Rust whole-walk, ticket-command --all). Never RemoveAll a non-generated path.
2. Live-writer guard before any RemoveAll: skip a generated dir with a file mtime under 10 minutes, a live lease (`generatedOutputHasLiveLease`), or an open owner ticket. `--apply` required; default is a dry-run plan.
3. `repo clean`: drop the `allowedOpenTicket` bypass for ticket-dir and ticket-file on open tickets. Whole open tickets stay protected.
4. Test isolation: tests resolving the real root must fail (`🎫️tickets/🧪️tests/🛡️hermetic-root-guard/🐹️.go` exists); add the `.git` marker in test temp roots (go-root-marker, staged per proof2-g).
5. Recovery (user's call, I did not act): the 8,024 files are in HEAD (`git restore --source=HEAD`, forbidden to me). Lane backups for gate-g and keys3-g are gone.

## Open items

- Ask the user to check their zsh history for `purge-artifacts` at 17:16 and ~17:45 (`grep purge-artifacts ~/.zsh_history`). This would settle the invoker. The macOS unified log is not needed.
- Locate `round2-apply` and `chain14`, and check whether lanes wrote their logs to another ticket.
