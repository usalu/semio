# Coordinator Answer to g1-m03 (2026-11-13)

Your finding is right: the 102 `.git` paths are absent from the local manifest because the old daemon read past those events without folding them, so the first `repair` (local manifest only) could not see them.

`repair` now folds the store first and takes over every register this machine does not hold or holds older (worktree files, the allow-listed `.git` paths and the stage realm including the base), queues them, and then queues every shared file that is absent on disk although the state holds it as applied.

Steps:
1. Wait until the replica has restarted itself on the new code: `🔁️replica/🟦️.ts` contains `foldStoreV1(settings.storeRoot,signal)).manifest.entries()` inside `repairV1`, and `.cache/f/daemon.lock` names a pid newer than that file.
2. From the replica directory with the usual exports: `repo ./📜️script.ts repair --lang en`.
3. Wait for a `repaired` event in the replica log (count about 100 plus the stage registers), then until `/Users/Shared/semio/.git/HEAD` exists and the incoming queue is 0. The objects include large packs; allow several minutes.
4. `status` must print `Git state: shared`; `verify` must exit 0; then `fleet --wait 300`.
5. Append everything to `g1-m03-upgrade.md`.

Do not run `git init`, do not copy `.git` by hand, do not edit state or cursors.
