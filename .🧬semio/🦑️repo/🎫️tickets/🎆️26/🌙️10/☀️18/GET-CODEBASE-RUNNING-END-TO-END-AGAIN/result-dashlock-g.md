# dashlock-g: a running dashboard must never fail or block a build (result, 2026-10-26 19:20)

FOR gate-g: batch ready `dashlock-g-tools/staged/dashlock/` (11 files, 6 modified + 5 new; bases are the live bytes of 19:12, `python3 -I dashlock-g-tools/stage.py check` says `pending` for all). Apply: `python3 -I dashlock-g-tools/stage.py apply`; laws: `sh dashlock-g-tools/staged/dashlock/verify.sh`; revert: `python3 -I dashlock-g-tools/stage.py revert`. It touches `📤️publication/🟦️.ts` (process), two re-export files and the dashboard installation module; it re-keys the dashboard Rust build only through the installation module, so run `repo-dashboard-rs:build` after landing.

## Root cause (not the one first assumed)
Dashboard installs were already content addressed (`dashboard-cli/<mode>/<digest>/semio`, installed.json swapped by rename) and the runner already holds only a SHARED lease on `artifact:<digest dir>` (`withInstalledDashboardV1`). The failure is the publish side: `pinExecutableArtifact` -> `stageArtifacts` takes an EXCLUSIVE lease on `artifact:<digest dir>` BEFORE it looks whether that digest already holds the identical bytes. Cargo is reproducible, so a rebuild after a peer edit that does not change the binary (tests, docs, mtimes: the installation's source digest is mtime based) yields the SAME digest as the one a human is running. The exclusive request then waits behind the runner's shared lease while the builder holds `artifact-capture:<dir>` exclusively -> after the grace period `LeaseOrderErrorV1 holdAndWait` ("Refusing to wait ... dashboard-cli/prototype/60a7..."), after a 5 min cargo build. A different digest would have published fine (20 digests, 569 MB, exist under prototype; the failing digest `60a7...` is the installed one).

## Design (landed in the staged batch)
- Immutable, content-addressed: a digest directory is never replaced while intact. `stageExecutableDigestV1(directory, owner, source, digest, options)`:
  1. fast path: under a SHARED lease of the directory (the same one runners hold, so they never conflict) verify marker owner, exact file list, single-link regular file, exec bit, sha256 == directory name; intact -> reuse, no exclusive lease, no copy, no waiting;
  2. absent digest = new path nobody holds -> normal exclusive publish (atomic directory rename), the previous digest and its inode stay untouched (Windows: a running .exe is never overwritten or renamed, we publish beside it);
  3. only an absent or DAMAGED digest takes the exclusive lease, as a non-waiting try-lock (`timeoutMs:0`, new optional `ArtifactPublicationOptions.timeoutMs`); if somebody holds it the builder re-reads (a concurrent publisher may have finished) with bounded backoff and the original deadline/cancellation, never holding-and-waiting. The only case that can still wait is a tampered executable that is also running (cannot happen for a previous build, which has another digest).
- Retention: `retireArtifactDirectoriesV1` claims each candidate with a non-waiting EXCLUSIVE lease (a running dashboard or reader holds its shared lease -> reported `held`, kept), renames it out of the namespace and deletes it. Policy in the installation module (`retireOldDashboardExecutablesV1`, schema const `ExecutableRetentionProtocol.keepNewest = 2`): after installed.json is swapped keep the current digest and the 2 newest (reuses `planNxToolingRetentionV1`), reclaim the rest when unheld; failures never fail an installation (next install plans again). Before this nothing ever reclaimed old digests (569 MB and growing).
- Exports: `stageExecutableDigestV1`, `retireArtifactDirectoriesV1` via both package re-export files.

## Laws (plain `bun test`, single file, real leases + real files + live holder PROCESSES, no official run)
`📤️publication/🧊️digest/🧪️tests/🟦️.ts` (4 tests, 41 expects, ~6 s):
1. same digest rebuilt while a holder process runs it: returns < 2 s, inode and mtime unchanged;
2. contrast = the incident: the exclusive in-place replace (`stageArtifacts`) of the held digest is blocked until the deadline;
3. new digest published beside a running one: old directory keeps inode and bytes, both digests present;
4. 5 scenarios (rebuild while running, new digest beside running, many runners, damaged digest repaired only when unheld, retire never-built) replayed against the real implementation equal the independent Python table `🐍️model.py` (shares no code; names executables by Python's sha256, compared with Node's).
`📦️installation/🧪️tests/🟦️.ts` (1 test): retention keeps current + 2 newest, a held old digest survives, is reclaimed once the holder ends; keep set computed by Python (`🐍️model.py`).
Before: the digest test file fails at import (`retireArtifactDirectoriesV1` not exported by the base module, checked by restoring the base file); the contrast test documents the legacy blocking. After: 4 + 1 pass. `tsc --strict` clean for the 4 changed/new TS files. A live digest directory written by the old code (`60a7...`) passes the intact check (probe in a clone, nothing published, inode+mtime unchanged).
Updated existing law: `📦️artifacts/📌️executable/🧪️tests/🟦️.ts` `testCoreExecutablePinControlV1` port `stageArtifacts` -> `stageExecutableDigestV1` (needs the official route; NOT run here: no official runs before 19:35 and the batch is staged).

## Proof recipe for gate-g (after landing, one official run)
1. `bun dashlock-g-tools/hold-installed.ts prototype` in a side process (stands in for a human's dashboard: shared lease on the installed digest directory, prints HELD, ends when stdin closes; do not use the human's real process);
2. touch a dashboard source file that does not change the binary, run `repo-dashboard-rs:build`; expected: exit 0, "Installed .../60a7..." with the same digest (before: LeaseOrderErrorV1 after ~5 min);
3. stop the holder. Also run the dashboard `:test` target (existing install test expects a, b to coexist; retention keeps them).

## Not done / caveats
- Official-route laws (pin port test, dashboard :test, real build with a holder) not run: staged. The lease order of the added shared acquire (rank-6 `artifact:` after the source-directory shared lease) is the same class pair the old exclusive request used.
- Docs: `📓️build-guide.md` could get one sentence ("a running dashboard never blocks a build; old digests are reclaimed when unheld"), EN+DE; not edited (docs2-g file).
- Incident note: at ~19:00 I ran `bun repo leases status` twice (read-only status, before I noticed the quiet window); no other `bun repo` command; no human process touched; mirror clone in /tmp/dlg removed at the end.
