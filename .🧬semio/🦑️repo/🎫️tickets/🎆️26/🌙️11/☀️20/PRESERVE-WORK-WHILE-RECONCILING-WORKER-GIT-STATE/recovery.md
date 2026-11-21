# Worker Git Recovery

On 2026-11-20, worker `g1-m07` had Git HEAD `3d62b2ef22c562d836e27f09600a7f24fe84f5e8` and substantial staged, unstaged and untracked work. The fleet server and peers `g1-m02`, `g1-m03`, `g1-m05`, `g1-m06` and `g1-m08` were still updating their NFS attestation files around 17:06 CET. The worker’s last attestation was timestamped 05:03 CET.

## Recovery copy

`/Users/ueli/Documents/semio-recovery.VM9RiFR0` is a private APFS copy outside the replicated worktree, on this Mac’s disk. The top-level `.git` holds the original HEAD, refs, index, config, shallow boundary and object database. `worktree/` holds the saved files. `later/` holds two entries captured after the initial copy, alongside the earlier versions in `worktree/`. `replica/` contains the checkpoint, WAL, conflict recovery files, adoption journal and machine and health records.

The saved HEAD, shallow file, packed refs and loose refs matched the live versions. The index, checkpoint and WAL matched by SHA-256. Git’s connectivity check passed and all 245,101 distinct objects referenced by the saved index were readable. Git reported the same tracked change counts from the saved index and worktree as in the initial live inventory. The saved worktree had 1,069 untracked files.

The worktree copy covered 308,121 file and symlink entries outside `.git` and `.cache`. The copy skipped the ordinary root cache and one APFS lease mount under another ticket’s generated output. Both remain on the worker. This was a live, file-by-file copy. The separate `later/` files reflect changes seen during the copy.

## Replication

The local replica repeatedly failed to become ready. A sample of the daemon’s child showed it waiting in a file-open call while loading Bun from the workspace’s tooling volume. The cached Bun binary could print its version, but the replica status command also stalled on a file open. The fleet NFS server accepted connections and the mounted export returned fresh metadata, while attempts to open the attestation contents stalled or returned `Interrupted system call`. The current shared HEAD and stage tree could therefore not be verified from this worker.

The worker’s local replica switch (`.cache/f/sync.json`) was set to disabled before restarting its own LaunchAgent. The restart accepted the new job but did not produce a fresh daemon health record or worker attestation during verification. This keeps the worker’s pending changes from being published on an eventual replica startup until its Git state can be compared with the fleet. The live Git index and refs were left in place. No peer or fleet store data was changed during recovery.

At this point reconciliation remained open because the worker needed working reads through the fleet mount and a replica that could start.

## Reconciliation after the NFS mount recovered

The follow-up used a second private copy of the worker’s Git data and replica state under `before-receive/`. The replica’s normal publishing path stayed paused through four receive passes. The passes applied 75,309 ordinary entries and 6,729 staging entries. The outgoing journal head retained its original SHA-256 throughout these passes.

Git could read `5db034ffe534a1581684b9d978004efabee6594c` after the objects arrived. The previous local commit `3d62b2ef22c562d836e27f09600a7f24fe84f5e8` is an ancestor. Git’s connectivity check passed. The local stage subsequently matched the archivist at `26c9a570c699f96ab5b6767559754bf143d41739`.

The local `Cargo.lock` differed from the fleet: it removed six dependencies and lacked a dependency added on the fleet. Its exact bytes are saved in `before-receive/incoming-conflicts/0/Cargo.lock`. The fleet version was applied at the live path.

The source audit initially found 98 local additions or edits without an upstream content change, plus 48 paths that needed reconciliation. The before-receive manifest and immutable store blobs supplied the three-way comparison. All 15 variants for which no clean merge could be generated matched earlier journal contents byte for byte. Those historical copies remain available under `before-receive/source-variants/` with a path map, while the current fleet versions were applied. The 41 skipped entries were ignored Git metadata such as commit graph and pack cache files.

Another batch of fleet updates arrived during verification. A final receive pass applied those updates and saved 26 existing local variants with a path map under `before-receive/last-receive/`.

Normal synchronization was enabled again. A fresh attestation reported an empty incoming queue and the same HEAD and staged tree as the archivist. All 98 local-only additions and edits were confirmed in the outgoing journal at the hashes on disk. At the first completed scan, the outgoing journal contained 102 worktree puts and zero worktree deletes. For worktree updates with a predecessor available in the retained journals, every update used that predecessor as its base or carried identical content. The replica continued to follow newer events from active workers.

The final check confirmed that `g1-m07` had replayed the archivist’s journal through the current head at segment 172, byte 256,212. The local attestation was three seconds old, synchronization was on and the incoming queue was empty. The last fleet comparison named no differing field for `g1-m07`. It still reported lag while other origins were publishing, so fleet-wide agreement was not asserted.
