# Native Release Module Pairing

The runtime roster contains 32 components. The four primary builds leave seven application roots and 21 extension roots. The explicit paired queue consumes adjacent entries in caller-supplied order and runs each pair to settlement before admitting the next. Wait for the primary supervisor to settle before launching it.

~~~sh
JOBS=2 ./native-queue.sh --pairs flow-extension-text flow-extension-math
~~~

The same mode accepts several adjacent pairs, with an optional trailing singleton. The two-root command inside the queue is the existing release environment and browser path through `native-lanes.sh step`, invoking `repo ./📜️script.ts components describe <left> <right> --facet browser`. It selects two declared components and invokes no publication shortcut.

## Cargo and staging

The live production planner assigns the seven remaining application roots to `🌎️hub/Cargo.toml`. Flow and Playbook therefore plan a single pass with both `-p semio-hub-flow-component` and `-p semio-hub-playbook-component`. The current foreground process may prefer those roots separately when priority or link memory warrants it. The planner groups by declared workspace, and all 21 remaining extensions each own a different workspace. Pairing similar extension roots uses two ordered Cargo passes inside one private target/build lane. It cannot obtain one unified resolution without changing actual workspace ownership.

The component driver predicts the joint retained footprint from the real unit graph and per-unit measurements and can split even a common-workspace pair into two passes. It then verifies live free space, performs its existing reclaim path, and refuses admission when below the required margin. Separately, the Cargo launcher holds one learned image-headroom reservation for the selected pair. A new pair crate-set key uses the published general component history or prior until it earns history. Single-root lanes are not warm matches for a new pair key.

Passes under the same release profile run serially. Their linked components can progress independently through header checks, Artifact capture, description, model publication, transpile, optimize, browser publication, complete and cleanup. The pass retains output identities that a later graph still needs, including package, target, feature and build-script identities, and reclaims outputs outside those remaining graphs. Each component receipt uses its own root closure and committed descriptor. There is still a separate link and browser stage chain for each component. A pair also shares one driver failure lifetime, so upstream edits or an unrelated failed stage can force both roots to fall back.

## Evidence and how to use it

These overlaps count package-name multiplicities from the actual retained Cargo unit source-admission rows, keyed by unit ordinal inside the largest available single captured graph for each root. Intersections use the smaller multiplicity and unions the larger. They are opportunity estimates. Matching package names do not establish equal artifact fingerprints, build-script output, timing, or graph resolution.

| Candidate | Actual planned Cargo relationship | Shared / union rows |
| --- | --- | ---: |
| Flow + Playbook | One hub Cargo pass before optional footprint split | 876 / 923 |
| Flow Text + Flow Math | Separate Cargo passes, one lane | 694 / 698 |
| Process Metal + Process Wood | Separate Cargo passes, one lane. Captured rows cover 726 of 793 declared units per root. | 724 / 728 |
| Sourcing Beams + Sourcing Slabs | Separate Cargo passes, one lane | 826 / 830 |
| CAD AEC Building Energy + CAD AEC Building Structure | Separate Cargo passes, one lane | 612 / 616 |

The previous `browser-partC.log.gz` showed 3,528 reused artifacts alongside 2,665 compiled units over seven separate passes. The five-pass `browser-partD.log` showed 1,211 reused alongside 2,430 compiled. These are actual Cargo freshness observations in shared lanes, though some may come from warm outputs present at startup. The corresponding elapsed times were 8,751 and 7,182 seconds and neither run hit the browser stage cache. The earlier individual Flow Text / Math and Sourcing Beams / Slabs logs reached 0 Cargo fresh units within their captured attempts. Some were early failures and cannot establish elapsed release savings. Prior large groups also had several collective failures, including an eight-root attempt that compiled 2,102 units and staged zero. The two-root cap reduces that exposure.

Use the Flow Text / Math pair as the first comparison after the primary builds. Capture elapsed time, each pass completion, first/second pass reuse progress, all browser receipts and any fallback. The net benefit is skipping repeated Cargo dependency builds and image admission, where both single roots would otherwise be cold or wait at the same headroom gate. The pair does not reduce two final links, skips no staging, and can lose wall time relative to independently warm singles. There is no measured wall-time speedup for the new mode yet.

## Queue acceptance

A pair is successful only on driver status 0 or the existing wrapper status 75 after a completed run, plus the latest final summary showing exactly two staged roots and zero failures. Because the driver receives exactly the two selected IDs, this count covers the two roots. The queue checks each root with the existing release-receipt logic and publishes both confirmation records only after both validate. Any invalid output, ambiguous result, or unexpected exit discards the pair evidence, and each root gets its existing singleton attempts. A failed singleton still makes the supervisor fail while the queue continues. Pair mode rejects duplicate IDs and any `JOBS` value other than 2, and starts no other pair or singleton until the active pair and any fallback settle.

The legacy queue fixture is green. The paired fixture exercises full success, status 75 with valid proof, stale earlier summary, incomplete status-0 result, missing second receipt, unrelated nonzero exit, existing stale evidence, singleton failure and continuation, invalid concurrency, repeated selection and odd tail. SQLite compares the exact launches and confirmations with the neutral fixture. Shell syntax and the read-only real component plan were checked. No module builds were started.

A singleton process exiting zero without a staging summary previously copied a pre-existing receipt into confirmation. An isolated fixture reproduced the false confirmation. The singleton and paired paths now both require the current attempt's final summary with the selected root count and zero failures. The singleton and paired suites pass. The original BIM and Dictionary singleton logs independently satisfy the stricter rule (795 and 47 units respectively, each with a single component staged and zero failures).
