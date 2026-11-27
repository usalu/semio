# Duplicate Row in 📋️build-owners.json

## Writers
Only `reconcileBuildOwnerRegistry` (`🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts`) writes the file; all registration callers (carve, mutation-tree, scaffold, move workflow, `verify build-owners sync`, registry-follow) go through it. Everything else reads.

## Cause
A single reconcile validates its output, so it cannot emit a duplicate. The likeliest cause is cross-machine (ticket 26/11/23/REPLICA-MERGES-STALE-LOCAL-EDITS-AT-APPLY): the replica daemon line-merges concurrent edits of the file from two machines, leaving two identical rows; the registry lock is machine-local and cannot see this. Every reader then refused with "Canonical build owner identity collision".
Secondary hole: the lock was taken over after 60 s even when its holder was alive.

## Fix
- `readBuildOwnerRegistryV1` (admission, used by every reader and by reconcile) canonicalizes on every read: rows identical in name, root, manifest, packages and native roots merge into one; rows that differ but share a name, root, native root or package still refuse with the collision error naming field, value and both owners. So `repo run` no longer fails on an identical duplicate.
- Reconcile compares its result with the raw file, so a merged duplicate counts as a change and the file is rewritten canonical (self-heal on disk); conflicting rows leave the file untouched.
- The lock keeps a live holder's lock regardless of age (takeover only for ended or unreadable holders).
- The earlier standalone `upsertBuildOwnerRowsV1` was deleted (it had no callers); one path remains.

## Tests
- `🧪️tests/🔣️.json` key `duplicates` (schema `🧬️schema/🧾️owner-consistency.json`), run by `owner duplicates` cases: identical rows merge and heal the file, non-adjacent identical rows merge, canonical file untouched, same name other root / same root other name / shared package refuse and leave the file byte-identical.
- Lock test: a live holder is not taken over by age.
- `repo test-file ... --lang=en`: registry-consistency 37 pass, admission 13 pass.
