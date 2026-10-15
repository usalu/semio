# Compact Recency Independent Audit

Read-only Source and existing-log review on 2026-10-15. No tests, images, process probes, filesystem census, deletion, or cap changes.

## Observed Evidence

Existing `🗑️generated/actor-current-source-prototype.log:9080` records acquired physical locks for two profiles and complete descriptor inventory. Line 9083 records the same unit path `prototype/build/semio-framework-io-base64/2f5745745ec5f9bc`, 609608 aggregate bytes, and `lockHeld:false` in both scans, with recency decreasing from 1792064587000 to 1792064516000 (71 seconds). Lines 9091-9092 record refusal at execution line 49. This proves timestamp-derived recensus disagreement; it does not identify which file or timestamp changed.

## Actual Derivation

`library/⚡️caching/🧹️pruning/🟦️.ts:112-123` measures a unit by walking its regular files and taking the maximum of every file's `atimeMs` and `mtimeMs`. Directory timestamps and symlinks are excluded. `CacheWalk.stat` and `entries` at lines 99-104 use lstat and readdir; this scanner does not read file content. `scanCargoBuildUnits` lines 147-158 emits that aggregate and hardcodes `lockHeld:false`, so that field is not independent evidence that this build unit never changed.

`budget/⚙️execution/🟦️.ts:45` plans from the initial scan, line 46 subsequently acquires physical profile locks, line 48 obtains and validates lsof, and line 49 scans again and requires exact aggregate recency equality before deletion. `pruning/🟦️.ts:40-44` retains actual exclusive build/target store leases across both scans. The canonical retirement path also retains compiler writer custody. Cooperative canonical writers are therefore excluded, but the initial census precedes physical profile custody.

## Candidates and Guard Assessment

No guard defect established. Same byte total and path do not prove an identical file set, inode set, byte content, or per-file timestamps. Candidates requiring evidence are a changed/removed max-recency file with an equal-sized replacement, explicit timestamp restoration, or platform/filesystem timestamp observation behavior. The scanner's own content reads cannot explain it because there are none; directory traversal timestamps are excluded. A simple newer atime update alone would increase rather than decrease the maximum.

The existing log contains only aggregate maxima, so it cannot distinguish these candidates. Preserve the refusal until a bounded per-file identity/timestamp witness or equivalent independently justified stable census establishes the cause. In particular, dropping equality solely because recency moved backward would silently accept an unexplained material change. Storage implementation owner has received the independent derivation and evidence limits.
