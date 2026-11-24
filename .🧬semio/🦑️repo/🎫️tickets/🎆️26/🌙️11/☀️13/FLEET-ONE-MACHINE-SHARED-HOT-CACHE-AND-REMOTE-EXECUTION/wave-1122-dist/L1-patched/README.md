# L1-patched: the lane pool keeps the lanes of a starting build (play snapshot)

Patches are made from the play snapshot's own file contents (`/Users/ueli/.sinc/play`), 8 files plus the new test directory `🌡️flavour-need`. `make.py <snapshot> <live>` regenerates everything.

## Apply
1. Copy the 8 files of `list.tsv` (column 2) into a scratch root and run `./apply.sh <scratch>`; it refuses on any pre-image sha256 mismatch and writes nothing unless every patch applies (stage 1), then moves all files in one pass (stage 2).
2. Then `./apply.sh /Users/ueli/.sinc/play` between two release chain runs. Never while the chain runs (read-only rule); the lane view, launcher and compiler store files are read at driver start (D), the pool/lane modules per `repo cargo` process.

## Verify
- `repo test-file <root>/…/🧵️lane/🏊️pool/🧪️tests/🌡️flavour-need/🟦️.ts` (5 tests; Python oracle and real lane directories).
- One at a time afterwards: `…/🏊️pool/🧪️tests/🏊️pool-model/🟦️.ts`, `…/🛡️product-retention/🟦️.ts`, `…/🔐️pool-claims/🟦️.ts`, `…/🏊️pool/🎯️product/🧪️tests/🟦️.ts`.
- Observable: a release start on an 80 GiB image with restored lanes prints no `lane pool trimmed` line; lanes written after the patch carry `usedMs` and `flavour` in `lane.json`. Lanes restored from before the patch have neither: they fall back to file times, so the first start after the patch can still trim them once; avoid it by running the build once with the patch before the restore, or by touching `lane.json` of each restored lane.
- Remove with the files the snapshot had: restore the 8 files from the snapshot (pre-image hashes in `list.tsv`).
