# PR1: can play's unit-cache root drop its stale entries? (2026-11-25)

Play asked for a supported command that retires the stale entries of `/Users/ueli/.sinc/play/.cache/b/r/m/u` only.

- **No such command.**
  - `retireStaleUnitsV1` (`⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts`) judges one root against one workspace.
  - It runs only from `repo cache gc` or the watching collector, in the policy mode `gc.staleness.mode` (`dry-run`).
  - No CLI flag selects enforce or a single root.
- **It would free nothing on play's root anyway.**
  - `judgeStalenessV1` treats every declaration below v2 as undeclared (`🍂️staleness/🟦️.ts:22`).
  - Play's sealed wrappers write v1. Dry-run with the real judge (`PR1-play-root-retire-dryrun.ts`): 8,192 entries (7,628 v1, 564 none), all undeclared, 0 GiB.
- **Independent estimate from the v1 sources** (`PR1-play-root-v1-staleness.ts`):
  - Method: each entry's recorded source sha256 against the play tree, plus older entries of a slot that has a newer one.
  - 2,575 entries doomed: 2,556 superseded, 13 source-changed, 6 both.
  - That is **13.22 GiB** of host-store blobs only they reference, out of 47.37 GiB the root holds.
- **Fix candidate (not done):** the judge reads v1 `sources` for direct staleness. It reaches play only through a new sealed wrapper, so it cannot help the running attempt.
