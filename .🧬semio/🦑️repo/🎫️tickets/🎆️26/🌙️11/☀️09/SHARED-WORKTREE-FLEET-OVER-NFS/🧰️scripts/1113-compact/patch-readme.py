#!/usr/bin/env python3
"""Anchored edits of README.md for the compaction landing. Usage: patch-readme.py <module root>."""
import sys, os
path = os.path.join(sys.argv[1], "README.md")
text = open(path, encoding="utf8").read()
def rep(old, new):
    global text
    assert text.count(old) == 1, f"anchor occurs {text.count(old)} times: {old[:80]!r}"
    text = text.replace(old, new)
rep("| `🚧️boundary` | What is shared:", "| `🗜️compaction` | Store-wide compaction: where the newest snapshot is and what it covers, the retirement plan for journal segments, the reclaim of unreferenced blobs, the compaction lock and record, the size report of `status`. |\n| `🚧️boundary` | What is shared:")
rep("`snapshot`, `relay`, `compact`, `verify`, `fleet`, `retire`, `touch`, `bootstrap-archive`, `bootstrap-settle`. |", "`snapshot`, `relay`, `compact`, `compaction`, `verify`, `fleet`, `retire`, `touch`, `bootstrap-archive`, `bootstrap-settle`. |")
rep("| `verify` | `[--worktree --store --json]` | Rehash the tree", "| `compaction` | `--plan` or `--apply` `[--worktree --store --no-blobs]` | Store-wide compaction (see Compaction): `--plan` measures against the newest snapshot and writes nothing; `--apply` takes the compaction lock, writes a snapshot, removes the journal segments and the blobs that the rules allow, prunes old snapshots and writes `compaction.json`. Works whether or not `replica.compaction.automatic` is on. |\n| `verify` | `[--worktree --store --json]` | Rehash the tree")
rep("A machine that has been offline for\nlonger than the retention cannot follow the journal any more: `join` it again.", "A machine that has been offline for\nlonger than the retention resumes from the newest snapshot that is ahead of its cursor (see Compaction); only when no snapshot is ahead\nit reports `rejoin-required` and has to `join` again.")
rep("## Capacity\n", '''## Compaction

The journals grow with every publish, a republish storm multiplies them, and a machine that is behind, new or rebuilding reads all of it.
Compaction makes that cost follow the live paths. Everything automatic is **off** (`replica.compaction.automatic` is `false` in the
topology); the explicit command works regardless.

**Snapshot.** A snapshot is the fold of the whole store (all realms, stage registers included) plus the cursors it covers, one JSON file
`snapshots/<writer>/<hlc>.json` written through tmp, fsync and rename by any one machine (`compaction --apply`, the daily `compact`, the
group server when automatic). Two machines writing at once is safe: each file is a complete fold at its own cursors, readers take the newest by hlc
and then follow the journals from the cursors in the file, and since a fold is last-writer-wins per path by hlc, snapshot plus tail equals the
full fold (property-tested against an independent fold and `🗜️compaction/🧪️tests/🐍️.py`). A joining, rebuilding (`state-rebuilt`) or `verify`ing
machine loads the newest snapshot; a replica that is behind jumps to it (below).

**Catch-up.** With `automatic` on, a replica that is at least `catchUpEvents` (20,000) events behind (estimated from the heads) and finds a
snapshot that saves at least that many reads the snapshot instead (event `progress`, stage `jump`), applies its entries by the usual
rules (local edits that are newer are kept, older ones go to `lost/`) and follows the journals from the snapshot's cursors. Independent of the setting, a replica
whose cursor points into a segment that was removed resumes from the newest snapshot that is ahead of it (stage `rejoin`); only if there is none it reports
`rejoin-required`.

**Segment retirement.** A journal segment of any origin may be removed when all of these hold: it lies wholly below the snapshot's cursor for that origin and below the
head segment; every origin that attested within `staleHours` (12) with synchronisation on has attested a cursor past it (so a live machine is never forced to jump); it is
at least `safetyHours` (24) old; and it belongs to the prefix of that origin's segments that satisfy this (no holes). Origins that are silent for longer than `staleHours`,
have `sync` off, are retired (`retire`) or never attested do not hold segments back: they resume from the snapshot when they return. Only `wallMs`, `sync` and `cursors` of
`attest.json` are read, so a change of the rest of the attestation cannot make a machine invisible; an attestation that cannot be read holds back with no cursors until
its file is `staleHours` old. The plan is reported by `compaction --plan` (and `status`) and the Python oracle computes the same segments. Removal goes oldest first, one `unlink` at a time.

**Blob reclaim.** A blob is removed when no entry of the newest snapshot and no event after the snapshot's cursors names it and it is at least `safetyHours` old (mtime).
To be safe against a publish that is in flight, the blob is first renamed into `v1/trash`, the events appended meanwhile are read, and it is put back if it is named now or if
its mtime changed since the scan; a publisher that finds its blob already stored touches it (`utimes`) before it appends the event, and stores the blob again if the touch finds
it gone (`touchBlobV1`; the relay does the same for blobs it already holds). A blob written before its event is young and stays. A file that goes back to the content of a
reclaimed blob simply publishes that blob again. Blobs that only an event before the snapshot names (a superseded version) are reclaimable: a replica that still reads that old
event sees one `blob-missing` for a version that is superseded in a later slice. `.cache/f/lost` is local to a machine and is not a reference; nothing in it needs the store blobs.
A reclaim whose snapshot was overtaken by another compaction (a segment it needs vanished) puts everything back and stops (`aborted`).

**Crash safety.** One compaction at a time per store (`v1/compaction.lock`, `mkdir`, refreshed every minute, taken over after two hours; `locked` is reported). A kill at any
filesystem mutation leaves a store that folds to the same manifest; set-aside blobs of a dead run are put back by the next one (`recoverTrashV1`); the next run finishes the work
(`🗜️compaction/🧪️tests/💥️killpoints.ts` kills a full compaction before and after every mutation).

**Visibility.** `status` shows the journal (bytes, segments, origins), the newest snapshot and its age, what the retirement rule would remove right now, and what the last applied
compaction removed and left; `status --deep` also lists the blob directory and measures the blobs a reclaim would remove now (about 30 seconds for 240,000 blobs; `compaction --plan` reports the same). All texts exist in English and German.

**Enable it.** In the topology file set `replica.compaction.automatic` to `true` (thresholds: `snapshotEvents` 100,000, `safetyHours` 24, `staleHours` 12, `catchUpEvents` 20,000).
The group server (`<group>-server`) then checks every ten minutes whether the newest snapshot is missing, `snapshotEvents` events behind or, with anything behind, `safetyHours`
old, and runs the compaction in its daemon; every replica enables the jump. First try `repo ./📜️script.ts compaction --plan` on the group server, then `compaction --apply` once by hand.
The daily `compact` job stays as it is (snapshot and the retention-based cleanup of the own origin); it is no longer needed for retirement.

## Capacity
''')
open(path, "w", encoding="utf8").write(text)
print("README patched")
