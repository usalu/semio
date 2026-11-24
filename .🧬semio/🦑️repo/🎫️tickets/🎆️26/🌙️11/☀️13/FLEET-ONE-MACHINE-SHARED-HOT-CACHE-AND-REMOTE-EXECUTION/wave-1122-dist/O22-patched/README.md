# O22-patched: oversize inputs declined at plan time, compile-slot tickets keep the arrival time (REQUESTER-ONLY, HOT-LOADABLE)

## Apply: sealed mode FIRST (picked up by every wrapper process spawned afterwards; no restart of cargo, driver, workers or servers)
```
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist/O22-patched/apply.sh /Users/ueli/.sinc/play/.cache/b/r/m/u/sealed/4114c4d6a52d7c25 sealed
```
Tree mode (next seal; tests included): same script with `/Users/ueli/.sinc/play` and no second argument. Stage 1 checks every target against its pre-image sha256 (the play clone's current files, equal in the tree and in sealed `4114c4d6a52d7c25`) and refuses on any mismatch before it changes anything; a file that already has the post-image is skipped; stage 2 writes `<name>.o22tmp` and renames. Nothing under /Users/ueli/.sinc/play was written by this lane.

## What the sealed mode hot-loads (exactly THREE files, all imported per wrapper call)
1. `🏗️build/🌐️remote-execution/🚀️offload/📜️action/🟦️.ts`: `shipOffloadActionV1` stats every input first; a file larger than the content store's entry maximum (`min(remoteCache.server.maximumEntryBytes, maximumBytes/2)` of the policy, 512 MiB today) is declined as `inputs-too-large` naming file and size, before any hashing, store call or upload (5 ms measured on a sparse 1.09 GB file). Covers the rustc offload and the command offload (one shipping path). Before: 26 units in 12:25-13:07 uploaded 600 to 900 MB, got `413 Entry exceeds the maximum entry size` after 5-20 s (one 195 s), together 721 s.
2. `⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts`: `acquireSlotV1` takes `since`; the ticket is named after `min(now, since)`.
3. `⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts`: both compile-slot acquisitions of the wrapper (the gate's timed wait of an unmeasured unit and the `compile()` one) pass `since: started`: a unit that waited in the gate (up to 15 s) or tried the fleet (5-195 s) and then compiles here keeps the place of its arrival instead of joining the back of the 4-slot queue.
Not hot-loaded: nothing else; no driver, worker or server file.

## Files
| file | kind | pre sha256 (16) | post sha256 (16) |
|---|---|---|---|
| `action.ts` | patched | `67e10680d08bcef1` | `2cfe3c6f5ba87f29` |
| `lock.ts` | patched | `5754b206e200466f` | `61c0cd7c9579e892` |
| `script.ts` | patched | `b90ffa14fb4017cd` | `cc0cbfc58327d12b` |
| `oversize-test.ts` | new (tree only) | `NEW` | `a2113ff443fdd235` |
| `ticket-since-test.ts` | new (tree only) | `NEW` | `f3e94c6e0b394bc3` |
`edit-o22.py` is the scripted edit (anchored, idempotent); `build.sh <clone sealed library dir> <scratch>` regenerates the three patched files and `list.tsv` from the clone's files.

## Verified
- Scratch copy of the sealed dir (5.2 MB, `/private/tmp/claude-501/o22/sealedcopy`, deleted after): `apply.sh <copy> sealed` writes 3 files, second run 0; `bun build` of its `script.ts` ok; `shipOffloadActionV1` from the copy on a sparse 1,086,841,728-byte rlib: `{"declined":"inputs-too-large","detail":"... libbig.rlib is 1086841728 bytes, the content store takes at most 536870912 per entry"}` in 5 ms with a store that throws on any call.
- Live tree: same edits landed; tests `repo test-file`: oversize 2/0, ticket-since 2/0, light-slot 4/0, offload main 41/0, eligible 4/0, command 11/0.
