# 🛰️ W22: g1-m07 offline and g1-m05 dependencies failure (2026-11-25 11:31 to ~12:10)

All evidence was read on the coordinator: `/Users/Shared/semio-store/v1/origins/g1-m0{5,7}/`, `repo fleet status --lang=en`, `nettop` and `lsof` on the coordinator's own sockets, and the code. Nothing ran on a worker, nothing was restarted, no git command touched the index. There was no sub-agent. The 1-minute load stood at 20 to 71 during the work. Lock: `lock.fleet-status`.

## (a) g1-m07: offline. Its tree did not diverge; its whole fleet stack stopped while the host stayed up

### The "4809 file(s)" difference is a stale view, not a divergence (hypothesis refuted)
- The view of m07 (`origins/g1-m07/view.txt`, written 08:57) compared line by line with `g1-server/view.txt` (11:32) gives 2,757 lines only at m07 and 2,069 only at the server.
  - 2,737 of the 2,757 are ` D 🧰️…/🤝️shared-worktree/🔁️replica/sa/…`. m07 has every `sa/` path twice, as `AD` and as ` D`. g1-server and m01 now have it once (`AD`). This is the duplicate-line defect W19 fixed at 09:06 (`🌿️git/🟦️.ts`). m07 wrote its view at 08:57 with the old code and has written none since.
  - The other 20 lines on m07's side and all 2,069 on the server's side are ordinary fleet edits since 08:58: tickets, `📕️norm` and `🔋️energy` artifacts.
- m07 had already converged on the SA1 mass delete (01:43). At 08:53 it was Ready and its view showed the `sa/` paths deleted, the same as everywhere else. The replica handled the mass delete and needs no fix for it.
- The trigger is simply that the attestation is old: `attest.json` and `head.json` stopped at 08:58:05 (`files 470478`, cursors current at that time).

### What stopped, and when (local times)
| record | last write | note |
|---|---|---|
| journal `segments/000000000010.jsonl`, `heads/g1-m07_10_…` | 08:58:00 | the last puts are UI and ticket files edited on m07 (`🫳️pointer-drag/places.ts`, `REPLACE-SILHOUETTE…/react-drag-verify.ts`) |
| `attest.json`, `view.txt` | 08:58:05, 08:57 | the attestation is written on its own `setInterval` (`🔁️replica/🟦️.ts` `attest`), so it does not depend on the main loop |
| `ready.json` | 08:57:17 (`state ready`, beat 120 s) | no beat was written after it |
| `memory.json` | 08:59:18, `{"absent":true}` | the host warden had already gone; the ready sidecar was still alive at that moment |
| `pool.json` (owner pool, pid 22370) | 09:12:57 | at 09:06:03 the pool SIGKILLed 5 children at once (code 137, "memory limit or yielding") |
| scheduler machine report | ~09:13 ("machine 2.3 h") | the agent's load reporter stopped too |

- The host is up and busy. It answers ping (192.168.178.81). From 11:35 it held 25 to 27 connections to the cache server on :6094, which started at 09:54, so these connections are new; over 10 s m07 uploaded 118 MB to it. It also opened new connections to the scheduler on :8981. Someone on m07 is building and offloading work.
- The store's NFS mount is idle. The single NFS connection (`2049 <-> 81:59389`) has carried 19.8 GB since the 01:16 boot but moved 0 bytes in 10 s. Every live replica moved 0.25 to 1.7 MB in the same 10 s (`nettop` diff, all clients). So no process on m07 touches the store any more. The replica, the ready sidecar and the warden are gone, not wedged in an NFS call: a client stuck on the mount would retransmit or reconnect, and there is no such traffic.
- Reading of the chain (from the code): the remote-execution sidecars (`agent`, `graph`, `spool`) are children of the replica daemon (`createSidecarsV1`, `🔁️replica/🟦️.ts:2992`). On a daemon restart the daemon only *releases* them (`sidecars?.release()`, `:3293`) and leaves them for the next daemon to adopt. That fits the timeline: the replica ended at about 08:58, the agent and pool ran on until about 09:13, and nothing restarted them when they ended. The agent restarts itself for new code with exit 75, and W16 and W19 landed agent and closure code at 09:05 to 09:10. Neither the replica nor its boot supervisor came back. The supervisor's kill path covers only a blocked event loop (`🛟️supervisor/🟦️.ts:306`); launchd's `KeepAlive` would restart the job unless the GUI session that hosts the LaunchAgent (`gui/501`) went away. I cannot see from the coordinator *why* the daemon and its supervisor ended. The memory signs point to memory pressure on this 16 GB machine during the local build (the warden disappeared before 08:59, and the pool SIGKILLed 5 children at 09:06). That is not proven.
- Not fixed in code, for lack of evidence: I found no replica or attestation defect that explains the stop, and I did not want to guess a change into the boot path. To settle it, a person on m07 should read: `launchctl print gui/501/tech.semio.fleet.replica` (state, last exit code, runs), `.cache/f/boot/beat.json` and `loop-blocked-restart.json`, `.cache/f/health.json`, `.cache/f/sidecars/*.log`, and `log show --last 4h --predicate 'eventMessage contains "jetsam" OR eventMessage contains "memorystatus"'`. Whether the user logged out after 08:58 would explain it too.
- Side effect to know about: m07's own edits since 08:58 (its session is still working) do not reach the fleet until its replica runs again.

### Fix landed: fleet status no longer reports an old view as a tree difference
- `📚️library/🏗️build/🚢️fleet-status/🟦️.ts:356` (`treeText`): when the replica verdict is `stale` (attestation old), the row now reads ", tree not compared: the replica's attestation is old, so its view (N differing line(s)) no longer describes the worker's tree". It no longer counts files "only here at the worker / only at g1-server". The old wording led the coordinator to look for a divergence.
- Schema first: new text `workers.tree.old` (EN and DE) in `…/🚢️fleet-status/🧬️schema/🔣️.json`.
- Test: `…/🚢️fleet-status/🧪️tests/🟦️.ts`, the tree-row test gained a `gone` worker with the m07 numbers (EN text, no "only here" counts, DE text).

## (b) g1-m05: dependencies stage attempt 45, "failed (exit status 1)"

### The current failure is not the W19 lease refusal
`origins/g1-m05/ready-log-dependencies-failed.txt` (10:12:09Z): the run started at 08:26Z and was adopted by supervisor 51908. The ownership walk took 5,362 s for 1.79 M steps, and one retry over the delta came from "Bun current ownership changed during verification" (`…\🎭️actor\…\package.json`). After 6,353 s it died in binding publication:

```
Store dependency sync incomplete {… "completedBindingsChanged":0, "sourceTransitionMayBeUnresolved":true,
  "activeSourceDirectory":"C:\\semio\\🧰️framework\\🔨️modules\\🖱️ui\\🎨️styling\\📦️packages\\🟦️typescript\\node_modules" …}
Internal error: Original Windows relative directory facts changed (reference d2bf1710)
```

- Source: `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:391`, the Windows `internal()` guard of a directory frame: `if(!same(expected,facts(owned)))throw Error("Original Windows relative directory facts changed")`. `same` compares the whole identity of the open directory handle, including `mtimeNs` (FILE_BASIC_INFO LastWriteTime) and `ctimeNs` (ChangeTime).
- The owning frame is the parent `🎨️styling/📦️packages/🟦️typescript`, which holds the `node_modules` binding. Nothing in that directory changed in the fleet since 2026-11-22 18:32 (coordinator listing). The replica therefore did not touch it, and `node_modules` is ignored.
- I traced every mutation of the Windows binding path. Stage create, in-place reparse `set`, both `rename`s and the displaced delete each either adopt the parent (`bindingAdopt`, which compares only `anchor` without times and then takes the new facts) or are not followed by a guard before the adopt. So publication does not trip over its own deterministic changes. The POSIX path is the same: adopt after `symlinkat`, `renameatx` exchange and `unlinkat`.
- So on Windows the directory times (or attributes) move without any change to its entries. This is the second time it has been seen: the 11-23 ticket `ARCHITECTURE-COURSE-REDEPLOY-READINESS/🚀️redeploy-readiness.md` (Windows, attempt three) hit the same `Original Windows relative directory facts changed` in Rust generator tasks, and `Print Source directory changed while discovering` "although no file of the assignment folders had changed for an hour". That print check also fingerprints directory `mtimeMs`/`ctimeMs` (`📓️print/🔨️modules/📥️source-staging/🟦️.ts:26`).
- A candidate mechanism, unproven: NTFS flushes deferred metadata of children, such as last-access times, into the parent's index after a handle closes ([Fsutil behavior](https://learn.microsoft.com/en-us/previous-versions/orphan-topics/ws.10/cc773059(v=ws.10)?redirectedfrom=MSDN)). Whether that moves the directory's own ChangeTime is not documented, and the docs say last-access updates have been off by default since Vista ([MS-FSA appendix](https://learn.microsoft.com/en-us/openspecs/windows_protocols/MS-FSA/4e3695bd-7574-4f24-a223-b4679c065b63)). It can only be settled on m05 by sampling `facts()` of a quiet directory over time.
- Two more Windows-only defects keep the failure fatal:
  1. POSIX raises the same detection as a typed concurrent edit (`concurrentEditV1("roster",root,"Original relative directory facts changed")`, `:260`). Windows raises a plain `Error`, which surfaces as `Internal error`. The same holds for the Windows twins "Windows relative entry changed while observed" and "Windows relative directory changed during scan".
  2. Even when typed, the dependency sync would not retry, because `sourceTransitionMayBeUnresolved:true` sets `retryState.unresolved`, and `retryableSyncFailureV1` (`📦️dependencies/🧾️incremental/🟦️.ts:194`) refuses unresolved transitions. The supervisor then retries the whole 1.5 to 2 h stage from the start.
- Not landed. The descriptor module is the fail-closed filesystem core, and its Windows FFI branch cannot run on this host. A change to which facts `internal()` compares (`anchor` plus roster instead of `same`), or to the error type, would ship blind. The 08:55 addendum forbids scratch builds, and m05 cannot be driven from here. Proposed change for a lane with a Windows test host: (i) on Windows, compare `anchor` plus the entry roster in `internal()`, not timestamps, since timestamps are not a membership witness on NTFS; (ii) raise the Windows twins as `concurrentEditV1` like POSIX; (iii) a Windows test that samples a quiet directory's `facts()` across handle closes in it.

### State on the fleet now
- **Attempt 45 failed the same way** at 10:58:25Z (log 11:58), in the binding phase again but in another directory: `activeSourceDirectory …\🎒️pack\📦️packages\🟦️typescript\node_modules`, `completedBindingsChanged 0`, `sourceTransitionMayBeUnresolved true`, `Internal error: Current exact Windows relative path descriptor changed (reference 1aed865a)`. That message *is* one of the retry patterns (`^Current exact (?:Windows )?relative path descriptor changed$`), but `unresolved:true` blocks the retry. So two runs in a row died on a Windows descriptor-facts mismatch during binding publication while 0 bindings had changed. That is the reproducible Windows defect described above, and it is not peer churn. Attempt 46 is running.
At 11:41 m05 is in attempt 45 again (`state running`, `dependencies:store-input-replay` 5,696 of about 7,810 edges, 740 s in). The same failure can come back in its publish phase. The other open items from W19 remain: a slow `store-ownership:readText` on NTFS (5,362 s for 1.79 M steps here), and the edge checks hit by peers' edits.

## Changed files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts`

## Tests
- `repo test-file …/🚢️fleet-status/🧪️tests/🟦️.ts`: **28 pass / 0 fail**, 2738 expects (run at 12:00, load 20.6).
- Fleet confirmation (12:01, `repo fleet status --lang=de`): "g1-m07 macos: offline, … Baum nicht verglichen: der Nachweis der Replika ist alt, ihre Ansicht (4931 abweichende Zeile(n)) beschreibt den Baum des Workers nicht mehr". The EN run of the same minute printed no tree clause for m07 at all, which means the tree reader returned nothing in that run; a later run is needed to see the EN text on the fleet.

## Open items
1. m07: on-machine read of why the replica daemon and its supervisor ended (commands above), then restart by a person. Possible code follow-up: a released sidecar set whose successor daemon never arrives should not run on unsupervised.
2. m05: the Windows directory-facts guard and its plain-`Error` typing (proposal above), on a Windows test host.
3. The coordinator index still holds the 2,737 staged `🔁️replica/sa/` entries (SA1/W19). They are harmless for the views now.
