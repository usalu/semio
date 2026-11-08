# lockfix 1107

## Root cause
- The refresh (`ensureCargoLockV1`, `🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/🟦️.ts`) never writes lock text: it only runs `cargo update --workspace`. Cargo cannot emit one name+version twice, so the refresh is not the writer of the duplicates.
- Both files were committed corrupt in 6f6cde5764b (amend of 731, 02:58); fbf4da6a5aa and the 0b2d54d02c4 fast-forward (02:21) had them clean. mtime of both is 02:30. The first copy of each duplicated row is the older dependency list and the second one is the newer; dependency lists inside rows are unsorted (cargo always sorts), and the second run of rows restarts the alphabet. That is the signature of a text-level three-way merge of two diverged lock versions (local fleet refresh vs the fast-forwarded tree) which keeps both hunks, not of a cargo write. Duplicates are path crates (no source): 61 rows in remodeling, 8 in raster.
- Scan of all 302 tracked Cargo.lock: only these two have duplicate (name, version, source) rows.

## Fix (writer side, hardened because the corruption cannot be prevented at the cargo writer)
- `cargoLockDeduplicatedV1` (pure) + `repairCargoLockV1`: under the per-workspace refresh lease and before the first probe, the lock is rewritten atomically without repeated rows (repeated registry row: first copy kept; repeated path row: all copies dropped, cargo regenerates them from the manifests). New message `deduplicated` en + de in the schema.
- Tests: new corpus case in `🧪️tests/🧫️fixtures/🔣️.json` (language-agnostic: a lock with a repeated path package is refreshed, then current, oracle = real `cargo update --locked`) and a unit test of the pure function. Verified: `bun test ./…/🔒️lock/🧪️tests/🟦️.ts` 13 pass, 0 fail.

## Repair and verification (ran)
- Both locks repaired by the fixed refresh itself through `slot.sh` + `repo cargo --store=shared --configuration=prototype -- check --manifest-path <M> --target wasm32-wasip2 --keep-going -p <crate>`: remodeling (`lockfix-1`, -p semio-s-artifact-remodel-remodeling-editor) reported "listed 61 packages more than once ... dropped", "behind its manifests", "current again", then compiled past the lock step; it ended rc 1 from upstream `semio-os-store-replay-operation` (29 errors, not lockfix). Raster (`lockfix-2`, -p semio-s-artifact-raster-raster) reported the same with 8 packages and "current again".
- Afterwards neither Cargo.lock has a repeated (name, version) pair (checked by grep/uniq -d). Logs: `🗑️generated/1105b/lockfix-{1,2}.txt`.
- Not done: no `dependencies lock`/`sync`; no git commands. Residual risk: if the merge that produced the duplicates recurs, the refresh now self-heals instead of refusing.
