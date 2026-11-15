# Resolution 2026-11-12 Memory Governor

Goal: 🎯consistentrepohistory. Three unmerged paths, stash labels `Updated upstream` and `Stashed changes`. No merge, rebase, or cherry-pick was in progress.

- `🏗️build/🧠️memory/🟨️.mjs`
- `🏗️build/🧠️memory/🟨️.d.mts`
- `🏗️build/🧠️memory/🧪️tests/🟦️.ts`

The governor stays on the upstream admission law: `attributable`, sample windows (`peakSamples`), and pressure against the host floor. The stashed `used` counter stored one number per kind and does not match that law, so it was not restored.

The stashed process-tree reader is kept, because the type surface already exports it: `psProcessRowsV1`, `processTreeBytesV1`, and `createProcessTreeMemoryV1`. The parent governor still attributes this process's managed bytes when no cgroup is readable. It does not bill the process-tree RSS.

Tests kept from upstream: attributable charging, the sample window, and malformed learned entries. The stashed owner-cost test is rewritten onto `attributable` and sample arrays. The process-tree reader test is kept as written. `repo tools exec --light bun -- test` on the memory test file: 23 pass, 0 fail.

`<<<<<<< ANCHOR` blocks under the non-destructive history-editing ticket are fixture text. The copies under `GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror` are a snapshot and were left in place. `LICENSE.md` separator lines are not conflict markers.

The three paths were staged as resolved. Nothing was committed.
