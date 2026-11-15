# Graph service EPERM below Documents (2026-11-15 ~08:56)

## Verdict
No code regression found; it is a macOS file-access (TCC) state, not a change of how the service is started. The service started on the "calling app" route inherits the caller's responsibility (posix spawn child, no disclaim, no `launchctl asuser`), so it only reads Documents when the process that called `repo` (its responsible app) has the consent.

## Evidence
- Routes: launchd first (`resident/🏗️composition/🌐️host/🍎️launchd/🟨️.mjs:45`, a job whose program is the launcher `repo`), refusal -> `invalidateLauncherMarkerV1` + `rememberLaunchdRefusalV1` and restart with `SEMIO_SERVICE_LAUNCHD=off` (`🧭️invocation/🛰️plan/🟦️.ts:96-114`, the "access of the calling app" text at line 106). The direct child is spawned plainly (`spawn` in host `🟨️.mjs`), no disclaim/setsid of the service itself; responsibility is read with `responsibility_get_pid_responsible_for_pid` (`🔐️consent/🧷️responsibility/🟨️.mjs:9`).
- `git log --since 2026-11-15 04:00` on resident/: no commits; last commit of host/consent files is 2026-11-12. The working tree carries ~30 uncommitted files of other agents (new access gate, mount reproof, responsibility watch), none changes the spawn attributes. Only detached fork: `sampleAccessInChildV1` (`🔐️consent/🟨️.mjs:80`), a probe child, inherits responsibility too.
- Launchd route refused at 08:46 (`~/Library/Application Support/semio/access/3f08bbea4d32/launchd-refused.json`, launcher sha d8266a63...; `exit.log` row tag 74145656f35a `access-denied route=launchd responsible=58552`). Since then the memo forces the direct route, as designed.
- Direct service 58711 (07:46Z) logged `access-proof ... route=direct responsible=39644`, 39644 = the Claude Code session process `.../Application Support/Claude/claude-code/2.1.325/485fdac4332e/claude.app/Contents/MacOS/claude` (self-responsible through Claude.app `Helpers/disclaimer`). It worked until superseded at 08:01Z.
- 08:58 local: two direct starts (pids 64119, 64212, `exit.log`, no access-proof, no exit row, log host-25cab2808032.log) died inside `build:implementation-source` with fstatat EPERM before the gate, i.e. their responsible app had no Documents access. Their caller is gone and was not recorded (start rows carry no responsible; see recommendation).
- Reproduction from this session (claude pid 39644): `repo run @semio-tech/mit-bestand-demonstrator:prepare-prototype` read all sources (new service 64982, `responsible=39644`, `service.json state=ok`); it only ended by my 170 s timeout while waiting for memory (capacity message), no EPERM.
- Two live sessions exist: 38683 (started 05:05, owns `repo mcp` 39171) and 39644 (05:08, owns `repo mcp` 39689). Same binary, both `disclaimer`-wrapped, so each is its own responsible app.

## What to grant (user action, I did not touch TCC)
System Settings > Privacy & Security > Files and Folders (or Full Disk Access) for both:
1. `/Users/ueli/.local/share/semio/launcher/bin/repo` - Documents Folder (the launchd route; refused at 08:46, so the consent is missing or was reset for launcher sha d8266a63...; the digest-keyed receipt invalidates when the launcher binary changes).
2. `/Users/ueli/Library/Application Support/Claude/claude-code/2.1.325/485fdac4332e/claude.app` (Claude Code) - Documents Folder, for the calling-app route; fully restart the failing Claude session (38683) after granting, since a responsible app that has not been re-prompted stays refused.
Full Disk Access for the launcher covers both launchd and every descendant.

## Why it works now
Service 64982 (started 09:00 from session 39644) runs under that session's access; every `repo` client (also session 38683) reuses it while the implementation hash is unchanged and 39644 lives. When 39644 ends, the `responsible-gone` watch stops the service and the next caller starts one under its own access; a caller without consent then fails again with this EPERM. Source edits by other agents change the implementation hash and force a fresh start, which is why the failure reappears intermittently.

## Hardening recommendation (not done, shared tree with in-flight edits in the same files)
Record the responsible pid/path of the client in the `start` row of `exit.log` and in the client error ("service could not answer") so a caller without consent is named instead of a generic cgroup-gone message.
