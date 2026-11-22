📦 Disk Cleanup Audit

Goal: free space on /System/Volumes/Data (needs 25 GB for native builds).

Before: `df -h /System/Volumes/Data` showed 15 Gi available (187 Gi used, 93%).
After: 13 Gi available (188 Gi used, 94%). Nothing was deleted by this audit. The 2 Gi drop came from other concurrent activity on the shared replica, not from this task.

Rules applied: delete only `🗑️generated` or tool-output dirs (`target`, `build`, `debug-out`, `work`, `target-*`, `build-*`) whose ticket `🎫️ticket.json` status is `closed`, with no file newer than 12 hours, and no git-tracked files. No `*.log` or `*.txt` over 50 MB exists under tickets.

Candidates found (all belong to open tickets, so none were deleted):

| Ticket | Dir | Size | Status | Git-tracked | Deleted |
|---|---|---|---|---|---|
| 26/07/23/GET-PROCESS-WORKING-END-TO-END | `🗑️generated` | ~94 GB | open | no | no (open ticket, excluded by rule) |
| 26/09/01/PROCESS-END-TO-END | `🗑️generated` | 368 KB | open | no | no |
| 26/07/07/PRINT-GLASS-PANELS | `debug-out` | 1.1 MB | open | yes | no |
| 26/08/06/S-AND-PLUGINS-END-TO-END | `work` | 1.0 MB | open | yes | no |
| 26/08/06/FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH | `work` | 8 KB | open | yes | no |
| 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT | `work` (2 dirs, rhu2/rhu3 tools) | 204 KB | open | yes | no |
| 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS | `work` | 72 KB | open | yes | no |
| 26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT | `build-driver`, `build-architecture` | 24 KB | open | yes | no |
| 26/10/13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES | `build-maintenance`, `build-mode` | 32 KB | open | yes | no |
| 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN | `work` (9 dirs) | ~3.4 MB | open | yes | no |

Ticket statuses were read from each `🎫️ticket.json`. No candidate had a file modified in the last 12 hours.

Notes:
- The `🗑️generated` dir of GET-PROCESS-WORKING-END-TO-END is the largest consumer (~94 GB), but its ticket is open and was excluded as instructed. It should be cleaned once that ticket is closed.
- The 167 closed tickets contain no `🗑️generated` or tool-output dirs matching the patterns, so no other closed-ticket space was reclaimable under these rules.
- Freeing 25 GB needs action beyond this rule set: close the GET-PROCESS-WORKING-END-TO-END ticket and then clean its `🗑️generated` dir, or get explicit approval to delete outside these rules.
