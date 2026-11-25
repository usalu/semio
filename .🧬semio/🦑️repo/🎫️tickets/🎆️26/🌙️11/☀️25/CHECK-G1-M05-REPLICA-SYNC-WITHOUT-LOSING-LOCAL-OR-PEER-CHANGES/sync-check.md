# 🔄 g1-m05 Replica Sync Check (2026-11-25)

This machine is `DESKTOP-6NSLEUO`, which is `g1-m05` according to `.cache/f/machine.json` (group `g1`, store `S:\`).
Everything below was checked read-only: replica `status`, `fleet --diff`, `verify`, `.cache/f/lost`, `git hash-object`.
Nothing was written to the working tree.

## Daemon and store

- The store `S:\` is reachable, sync is on and the daemon is running (pid 36992, code `96d644fb9453`, the same as every current peer).
- This machine is 0 events behind every origin. g1-m02 and g1-m03 were 2 events behind at the time of `status`, which is normal for edits that are still being published.
- Trust is current: `trust.verifiedMs` came after the last `lostMs` (`round`), so the manifest was re-verified against the tree after the last failure.
- Recurring failures: `loop-blocked-restart` ×81 (the last one was about 51 h ago), `round` ×9 (the last one about 20 min before the check), and `service-definition` ×3. None is active.
- Publish and apply are slow on this machine compared with the macOS peers: publish p95 15.5 s and apply p95 20–30 s, against about 2 s on the others. This is why `fleet` lists g1-m05 as `lagging (not settled)` instead of `ok`.

## Changes from other workers (incoming)

`fleet --diff` against every healthy peer (m01–m04, m06, m08, m10–m12) showed 3 lines that exist only on the peers and 0 lines that exist only here:

| Path | Result |
|---|---|
| `🎓️teaching/…/sleeping-person/🔣️.json` | Applied here at 12:46:10. `git status` now shows ` M`, the same as on the peers. |
| `.🧬semio/…/MH-4/proof-lines.py` | Applied here at 12:46:10. `git status` now shows `??`, the same as on the peers. |
| `✏️s/…/fem/…/loads/receiver-tests.rs` | The local bytes are equal to HEAD and to the index (`e28594ef…`). The macOS peers show `M`. See "Verify" below. |

The `view.txt` that `fleet --diff` reads for g1-m05 is the attested view. Its git view tier runs about every 90 s, so it lags the live tree. The 3 lines are paths that were still in flight, not lost changes.

## Local changes (outgoing)

- On the healthy peers, `fleet --diff` lists no line that exists only on g1-m05. Every local change on this machine is published.
- The long list of lines "only on g1-m05 (here)" comes only from the comparison with **g1-m07**. Its attestation is about 3.8 h old (`stale`, its view is on code `8493edad8d8f`). That list is g1-m07 falling behind, not changes that exist only here.

## Overwritten files (`.cache/f/lost`)

- There are 621 entries. The newest are `1795483333267-0000xx-g1-m04`, from 2026-11-24 02:22. No file was overwritten in the 30 h before this check.
- The older entries are `bootstrap-*`, `old-repo-*`, `index-*` and `sealed-restore-*`. They are kept backups from the join and bootstrap, not current work.

## Verify result

`verify --json` ran from 11:47 to 12:10 UTC (23 min). It checked 471,198 files and exited 1: 48 paths drifted and 0 were refused.

| Kind | Count | Explanation |
|---|---|---|
| `content`, unchanged since the scan | 29 | The local file was rewritten during the scan, between 11:48 and 12:10 UTC. The scan hashed the new bytes while its state copy still held the old ones. |
| `content`, changed again after the scan | 10 | Files that were still being edited: puzzle and staffage tickets, `os/plugin/interpreter` and `host`, with the last write at 12:10:31 UTC. |
| `state` | 7 | The state moved to a newer event (`g1-m02`, `g1-m03`, `g1-m14`) during the scan. |
| `extra-local` | 2 | This ticket's own `🎫️ticket.json` and `📌️important/📝️.md`. Both were created at 11:47 UTC and are now in the attested views of g1-m01, g1-m06 and g1-m13. |

Every drifted path has an mtime inside the scan window. On a tree that other workers keep editing, a 23-minute rehash cannot match exactly. The paths are live edits, not stale or lost data.

## Re-check after verify (12:11 UTC)

- `health.json`: `trust.verifiedMs` is 12:06:44Z, after the last `trust.lostMs` (`round`) at 12:02:10Z, so the manifest is trusted again.
- Publishing works: the 3 files of this ticket, `sync-check.md` included, reached the peers within about one minute.
- This machine is 0 events behind g1-m01, m05–m08 and m10–m14. It was 1 to 5 events behind g1-m02, m03 and m04, which were sending at that moment.

## Verdict

g1-m05 is in sync. No local change is unpublished, no change from another worker was overwritten (`lost/` has no entry newer than 2026-11-24 02:22), and every difference that was found was a live edit in flight.

## Open points for W23 and the coordinator (not changed here)

- **`round` failures are rising:** 9 → 29 between 12:27 and 13:02 local time. Every time they reset `trust.lostMs` until the next sweep. They coincide with the long `dependencies` stage and with the busy disk.
- **Slow publish on this machine:** publish p95 rose from 15 s to 183 s during the check, against about 2 s on the macOS peers. The apply p95 is 22 s. This is what keeps g1-m05 `lagging (not settled)`.
- **Leftover temporary files:** about 80 `beat.json.*.tmp` in `.cache/f/boot/`, and many `pool.json.*.tmp` and `memory.json.*.tmp` in `S:\v1\origins\g1-m05\`. These are renames that never completed, which is consistent with Windows sharing violations. They are harmless but show how often writes fail here.
- **`receiver-tests.rs`:** the macOS peers show `M` while the bytes here equal HEAD. This is most likely only the executable bit (Windows is `modeless`). It does not count as a divergence.
