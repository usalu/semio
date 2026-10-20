### 🥾️ Recovery after a reboot or a crash

A reboot, a kernel panic or a killed run leaves state behind: publication intents of a killed publisher, queue tickets of dead processes, disk images of finished tests, grants of the host capacity governor and device numbers that no longer mean what they meant. The boot sweep ({{@boot-sweep}}) recovers all of it at the next `repo` command, with progress and cancellation, and never blocks the command if a recovery step fails. You do not run a recovery command.

1. **Run the command you wanted.** The first partition open of the process runs the sweep. Its steps publish `boot-recovery:<stage>:<done>/<total>:<target>` progress lines; a failed step is reported as `boot-recovery:failed:<reason>` and the command continues; Ctrl+C cancels the sweep and the command.
2. **What the sweep does.**

| Leftover | Recovery |
|---|---|
| `.publication/intent.json` or `receipt.json` of a killed publisher (every command refused with "Pending Native image publication must settle") | a pure plan (rolled back, completed before the crash, torn, empty) executed under the exclusive image lease the opener already takes; ambiguous or unknown residue is refused with its reason and never deleted |
| queue tickets and claim databases of dead processes in `.cache/b/c` | tickets of an earlier boot are retired one by one, only after the repository's own exclusive claim-database proof; tickets of a dead process in the same boot are reaped by the next waiter behind them; once per boot, recorded in a ledger |
| disk images of finished or killed tests (the scratch-image class) and images whose backing file vanished (macOS) | an image under a scratch root (`/private/tmp`, `/private/var/folders`, `/var/tmp`, `/tmp`, `$TMPDIR`, `$SEMIO_TEST_ARTIFACT_DIR`), outside this workspace, idle for 15 minutes and with its own lease provably free is detached without force; a busy volume is skipped; workspace and foreign images are never touched; at most one sweep per 5 minutes |
| device numbers stored in receipts (they change after a reboot or a parent image re-attach) | no longer persisted truth: inode, mode, size and platform stay exact and devices are compared only through their pairing; image receipts are rebound to the new boot session without rewriting them |
| grants of the capacity governor of an earlier boot or of a dead process | reclaimed when the owner dies or the boot session changes |
| sandboxes and port leases of dead test runs | reclaimed by the next sandbox creation (dead owner, or heartbeat older than 1 hour) |

3. **What the sweep leaves.** Funded compiler-state children (an Nx namespace) with a reserved row and no commit stay until their owner resumes them; stale Bazel and Nx server files (`server.pid.txt`, cookies) are left to the tools' own stale-server handling; a reboot inside a live Bazel or Nx server session is not repaired by the sweep.
4. **Check afterwards.** `repo capacity status` lists no grant of the previous boot; `repo dependencies ensure` installs only what is missing. For the one refusal that still names a state ("Exact canonical persistent Native publication required …") and for the Nx cache, follow Recovery in the reference: use the dashboard control that owns the operation.
5. **Never** detach an image by hand, force an unmount, edit or move a receipt, or delete a directory below `.cache/b`. Cache partitions, Bazel output bases and the Cargo store are derived and safe to remove only when nothing runs and only through their owning controls (`repo cache clear`, the dashboard).

Verified in copies of the tree with real `hdiutil` (an abandoned and a fresh scratch image, a SIGKILL before each state-changing call and every torn write of the publication ceremony, the directory swap, a record transaction and the ledger append: recovery reaches the old or the new receipt and the second run is clean); the sweep measured 17 ms for the roster and 10 ms for a no-op. Not verified: a live drill after a real reboot, Linux and Windows (the boot epoch parsers are tested for all three platforms from fixtures).
