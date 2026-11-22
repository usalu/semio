# Release unit reuse across attempts

## Verdict
Enable the unit cache for the next attempt. The wrapper chain works, hits are real and outputs are byte-identical. Cargo-level reuse across attempts cannot work for this release as configured, so the unit cache is the only reuse path.

## Validation (real cargo, `repo cargo --store=shared --configuration=release`, wasm32-unknown-unknown, `SEMIO_UNIT_CACHE=on SEMIO_UNIT_CACHE_REMOTE=off SEMIO_OFFLOAD=never`, crate `semio-framework-value-intrinsic`, 32 units)
| Run | Lane | Result |
| --- | --- | --- |
| 1 cold | new lane p8bba2d | rc 0, no EACCES, 11.2 s build, 38 cache entries stored |
| 2 same crate set | same lane (warm affinity) | 0 units compiled, 2.7 s, Cargo fingerprints fresh |
| 3 other crate set (+paged-list) | new lane pc992f5 | rc 0, 6.7 s, only 2 new entries stored for 32 units, so about 30 served from the cache |
Rlibs of intrinsic, native-decoding and paged-list have identical sha256 in both lanes (cargo itself is the third-party reference). Run 4 (third lane) queued behind the release on host-capacity and was cancelled. The release lanes were not touched (pool had 8 lanes, below the 16-lane and quota bounds). Driver convention check: `compilerWrapperEnvironmentV1` with the real `semio-build-scheduler` as inner returns no `RUSTC`, wrapper is the shim, and `shim rustc -vV` exits 0 (probe in scratchpad, same as `rustc-wrapper-chain-probe.ts`). A full driver pass was not run (it would share the release lanes).

## Why "0 reused" and 5,000 recompiles per attempt
1. The warm generation shares one stable lane family, but compile happens in per-product lanes (`q<digest>`, product key stable). Inside a lane, Cargo reuse does work (serde units from 11:59 untouched by later attempts). The counter shows 0 because the progress line counts wrapper-invoked compiles, and fresh units never invoke the wrapper.
2. The target directory name `t/<hash>` is the digest of the recipe, which hashes `Cargo.lock`, `🧰️framework/Cargo.lock`, every manifest in the chain and the RUSTC_WRAPPER value. Both lock files changed between two attempts (diff of `recipe.json` in lane q53457c: only those two sha256 differ), and they are rewritten at each attempt start (mtime 12:34). Each attempt therefore gets a fresh target directory and the old one is reclaimed (6 MB left).
3. The pool trims idle lanes LRU to 16 lanes and about 23% of the image (about 16 GiB). The release needs about 32 GiB of lane output (one wasm lane alone was 9.2 GiB), so LRU thrashes and earlier products are evicted before the next attempt.
4. Lanes duplicate shared closures: p440491 and p6f4b0a each hold the same 909 rlibs. Without the unit cache each product recompiles the whole closure, even within one attempt.
The unit cache removes all four causes (content-keyed, lane-independent, deduplicated).

## Risks
- CAS lives on the host volume (about 29 GiB free, 88 percent used; CAS now 3.8 GiB, cap 32 GiB; collection starts at 12 GiB free, critical at 1.5 GiB). A unique release closure is estimated at under about 10 GiB (not measured). Watch `repo cache cas status`.
- The full driver chain (scheduler inside the shim inside a product lane) is unproven in a real driver pass. The launcher guard below limits the blast radius.

## Change (takes effect at the next attempt start; running attempt untouched)
`native-play.sh` was not edited: its `site` loop exports `SEMIO_UNIT_CACHE=off` for the whole run, and editing a running bash script is unsafe. The launcher is spawned per attempt, so the decision moved there:
- `native-build-launch.ts`: sets `SEMIO_UNIT_CACHE` per attempt via the policy; opt-out `SEMIO_RELEASE_UNIT_CACHE=off`; if a cache-on attempt fails within 120 s it writes `🗑️generated/native/unit-cache-demoted`, reruns once with the cache off and keeps it off afterwards.
- `native-unit-cache-policy.ts`: pure decision functions.
- `native-unit-cache-policy.vectors.json` and `native-unit-cache-policy.test.ts`: language-agnostic vectors, 2 tests pass (`bun test`).
Remove the marker to retry the cache after a demotion. Later: change the `native-play.sh` default to on once no run is active.

## Not changed (follow-ups)
- Do not key the target recipe on lock/manifest digests (Cargo fingerprints already handle them); needs its own design and tests.
- Raise or exempt product lanes from the pool quota, or fix the progress counter to include Cargo fresh units.
