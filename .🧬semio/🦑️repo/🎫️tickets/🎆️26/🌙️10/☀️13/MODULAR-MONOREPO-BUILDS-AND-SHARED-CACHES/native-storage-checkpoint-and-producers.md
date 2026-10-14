# Native Storage Checkpoint and Producer Custody

## Qualified Scope

The fixed macOS image constrains its current data file and mounted filesystem synchronously. Ordinary directories and unsupported physical backends refuse admission. Known Cargo home, target, build, captures and temporary paths belong inside the observed backing. This does not establish an aggregate repository, host snapshot, global lease, toolkit or arbitrary external writer ceiling. A four GiB compiler reservation remains an admission charge rather than an independently enforced subquota: a compiler can exhaust the shared finite image if it writes beyond that charge.

The actual image root is the authored `m` directory. Queue and journal transition leases use the original short `.cache/framework/leases` owner. Admission now uses the lower of declared capacity and observed backing maximum. Retiring and committed allocations remain charged until their original physical retirement callback acknowledges release; recovery does not infer release from age or PID disappearance.

## Durable Warm Journal

The schema now declares a checkpoint event containing the exact sequence and all current allocations. Only an empty projection admits this event. Automatic checkpointing begins before an append would exceed 512 KiB; the journal has a one MiB maximum. One canonical staging file is written and synchronized, renamed atomically over the journal, then its parent directory is synchronized. Initial journal creation also synchronizes its directory. Existing interrupted staging is handled only under the exact backing's exclusive transaction custody. Malformed authoritative journal data still refuses recovery.

Repeated replay originally exhausted the genuine five minute prototype grant. The corrected implementation retains at most eight private, frozen projections, reusing them only after a complete current journal read has exactly equal bytes. The cache never substitutes for physical custody or allocation presence. A stale recovered reservation still refuses after another owner has durably retired its allocation.

| Actual run | Outcome | Evidence |
| --- | --- | --- |
| Checkpoint neutral RED | Four passed, one failed | `native-storage-checkpoint-red1.log`; new checkpoint case genuinely failed before reducer support |
| Checkpoint neutral prototype | Five laws, 45 assertions, GREEN; Nx 158 ms | `native-storage-checkpoint-prototype1.log`; independent SQLite checks all three held phases and sequence |
| Physical checkpoint prototype4 | GREEN 1.7 s | Original PID 56430 ended with SIGKILL after a synchronized checkpoint; 196,608 bytes retained until physical retirement |
| Physical checkpoint release1 | GREEN 1.6 s | Original PID 56713; same retained charges and normal backing retirement |
| Warm prototype5 | RED 5 min | Repeated replay exhausted its original grant; cleanup also refused the expired caller; no pass claimed |
| Exact failed fixture recovery | GREEN 816 ms | Four charges, 200,704 bytes; complete process output 40,458 bytes, exclusive actual image custody and empty whole-filesystem lsof; owned acknowledgement followed normal detach |
| Warm release2 | GREEN 10.1 s | 1,767 real reservation/write/physical-retirement cycles; peak journal 524,039 bytes; automatic checkpoint; PID 58612 SIGKILL; all held phases survived; final journal 77 bytes and image allocation 33,554,432 bytes |
| Warm prototype6 | GREEN 10.3 s | Same 1,767 cycles and peak; PID 61146 SIGKILL; normal terminal retirement |

The actual crash occurs after checkpoint completion. An interrupted staging fixture additionally exercises reconstruction without discarding authoritative charges. These runs do not constitute a power-loss qualification at every instruction in rename or synchronization.

## Compiler Entry Points

Neutral Cargo test/lint execution now requires an issued native reservation, the original control and explicit Source limits before capture directory creation. It checks Cargo home and all temporary/output roots, performs actual metadata/unit-graph/cfg Source admission, and uses the original signal through lease acquisition and terminal child settlement. No ambient storage provider is installed in Framework.

The repository facade receives full original invocation authority. Sixty-one direct test/lint script owners and 77 repository test calls were changed to retain the full invocation. Twenty-one direct compiler command sites now use the bounded repository command facade. The native scheduler, neutral relay and component driver additionally require issued storage and Source limits before generation/compiler writes. The repository relay routes Cargo on Windows as well as POSIX, closing its earlier direct Windows bypass. These authored integrations require broader consumer qualification; the counts are edits, not a claim that every command has run.

`withRepositoryCargoInputsV1` uses an explicit original offline grant. It holds exclusive bounded Cargo-home custody for locked fetch, then a shared reader through metadata/planning/compiler settlement. Inputs never redirect to the previous external Cargo home. Additional declared manifests can share one acquisition operation. The network selector corpus passes one law, 12 assertions, Nx 128 ms, against Ajv and the neutral expected vectors.

| Actual bounded Cargo proof | Outcome |
| --- | --- |
| Prototype1 | RED before writes: the fixture supplied an invalid fixed-image envelope |
| Prototype2 | RED after image provision and correct offline refusal: fixture lacked its locked Cargo.lock; image retired normally |
| Prototype3 | GREEN 4.1 s: real locked offline fetch, metadata, Source admission, compilation and native assertion in a 32 MiB backing |
| Release1 | GREEN 6.8 s: the same real flow under release; image allocation remained 33,554,432 bytes; normal retirement |
| Neutral missing/forged test custody | GREEN one law, seven assertions, Nx 183 ms; neither captures nor producer writes created |
| Cargo test strict types | Genuine TS7053 RED corrected, GREEN 1.1 s |
| Broader producer strict types | Import-depth RED corrected; current included Core driver/scheduler/relay, repository facade, input physical proof and Framework Rust script GREEN 1.7 s |

The native proof uses an authored standalone language-neutral fixture in the owned image. It is distinct from qualification of all original extracted monorepo crates. Original identity Source had earlier direct rustc/library/native-law evidence in both modes. Cargo leaf integration earlier exposed missing bounded registry data, required authoritative lock changes and a 269-character Cargo intermediate; those RED attempts remain distinct and do not become GREEN from the small fixture.

## Permanent Inactive Retirement

Permanent maintenance selects genuinely inactive cache units under a retained-byte budget and a physical headroom goal. Recency is ordering only. Global store leases are acquired in canonical order for related build/target directories, then exact system compiler file locks and a complete descriptor inventory protect mutation. Selected units are rescanned before deletion. Active or uncertain custody refuses; no foreign compiler is killed or forcibly reset.

The neutral planner's SQLite comparison is GREEN one law, 12 assertions, Nx 143 ms after an initial incorrect import RED. Actual permanent preview is GREEN 5.5 s: 2,567 units, 18,078,747,124 logical bytes and 215,467,970,560 available physical bytes. The interim 80 GiB retained budget / 24 GiB headroom policy selected zero units. No large cache deletion by this maintenance is claimed. The observed large reduction was concurrent external cleanup and must not be attributed to this agent. Final retirement of former stores remains after all producers and readers migrate.

## Outstanding Work

- Finish explicit network propagation through component/renderer/dashboard helper consumers and original frontend activation.
- Complete bounded routes for remaining raw bridge or registry commands and actual original extracted crate qualification.
- Resolve output readers still naming former Cargo/publication paths; no fallback is authorized.
- Qualify all command paths and both modes after their final Source changes; old fake-port fixtures require actual admission fixtures.
- Provide qualified physical backends for Linux/devcontainer and Windows. Current macOS-only qualification must not be reported as universal zero-touch operation.
- Separately bound toolkit, graph and global lease footprints before claiming a whole repository ceiling.

No heavy prior owned-output/native-plugin retry was resumed. Its full output/manifest/Surface/WIT laws remain unqualified. The final old plugin compile failure included unresolved Pack imports and unresolved `semio_framework::FaultSource`, not only Pack diagnostics.
