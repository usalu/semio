# Result F7 (red B/C tests) - PARTIAL, no product or test edits landed

Method: red list of C = the 111 STILL RED rows of wave-1122 result-E43 (resolved to paths in `T/🧰️scripts/F7/red-C.txt`), run one file at a time by `T/🧰️scripts/F7/run.py` (load gate 40, memory gate 25 %, resumable; rows in scratch `F7/c1.jsonl`). Host load stayed 34-43 so the gate waited often; heavy files take 3-10 min. In about 55 minutes only 18 of 111 files were re-measured; B (17 red per E67) was not re-run.

## Before -> after for the 18 re-measured C files (before = E43/E70/E78/E83 state)
Now green (4): `♻️reset/⚡️recovery` 5/0, `🏪️store/📦️child` 9/0, `📥️opening/📐️layout` 3/0, `📏️headroom/…/🔄️cohort-controller` 1/0.
Still red (12 failed, 2 timeouts):
- `🌐️vite/💾️storage` 8/1: "Finite Original physical cleanup reserve required before effects" (test "actual Vite intent builds retain finite products...").
- `🏗️compiler/📏️capacity` 11/2: "Original physical body cutoff reached" at ~19 s inside the isolated child although the test declares bodyMilliseconds 100000/200000; the inner cutoff is not load-scaled (candidate for stretchedBudgetMillisecondsV1 in the invocation body deadline, `🏃️process/🧭️routing/📥️invocation`).
- `🏪️store/🧪️tests` 3/6: spawned child prints only the bun banner (child filter/entry not run).
- `📤️publication/⚓️anchor` 0/1: needs a case-sensitive volume (environment).
- `📤️publication/🏗️native-build/📦️owner` 10/2, `…/🏗️native-build` 3/4, `🧵️prepared-worker` 1/2 and its two `📜️script.ts` (0/1 each): "Original ... required" cluster.
- `📏️headroom/🧾️closures/♻️groups` 7/6, `📡️session` 4/1 (worker cancellation), `🔏️admission/admitted-worker-harness` 1/2.
- timeouts at the 590 s cap: `📏️headroom/🧪️tests/🟦️.ts`, `📏️headroom/🧾️closures/🧪️tests/🟦️.ts`.
Net in the sample: 4 of 18 now green (files fixed by peers since E43), 14 still red. Extrapolated, C stays near 100 red files.

## Not done
93 C files not re-measured; B not re-measured; no fix attempted (every remaining cause seen is the deep "Original ... required"/private-repository cluster or a product deadline, not a quick drift). No landings, no locks taken; claim line in `claims-tests.md` (F7) only.
Resume: `python3 T/🧰️scripts/F7/run.py T/🧰️scripts/F7/red-C.txt c1.jsonl 520` (skips done rows in the scratch jsonl).
