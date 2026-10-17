# Cooperative Lease Inventory Source Checkpoint

This is qualified ordinary Source progress for the active monorepo build/cache goal. It does not certify a fixed physical Rust/cache/repository footprint. Full goal/ticket and all Native/default SDK/publication/compiler/cache-image/OS-sampling holds remain active. The full physical 65,536/65,537-entry inventory fixture is explicitly unqualified; its failed materialization evidence is preserved in `rust-lease-inventory-physical-follow-up-current.md`.

## Production Changes

`resourceLeaseRetirementNamesV1` now requires genuine privately issued ResourceLeaseControl and returns a Promise of frozen sorted immediate names. There is no old synchronous overload or compatibility alias. Both ordinary metadata retirement and queue-claim maintenance await the same helper. Copied/expired/failed controls fail before opening. A retained issued operation uses the existing tracked buffered directory port, exact guard checks around reads, cooperative yields every 128 entries, exact FD close/unresolved custody, and a final issued guard after closure before delivery. The operation is asynchronous/cooperative; individual directory reads and close use the existing buffered synchronous port. No promise-per-entry descriptor implementation remains.

Schema-authored progress publishes at the initial 128 names and then every 4,096 names; cancellation/yield remains every 128. This separates cooperative checks from progress traffic and keeps the controlled full-cardinality response within the unchanged 64KiB/256-line child transport. Directory inventory bound 65,536, candidate bound 8,192, candidate database extent 16,384 bytes, Original clocks/reserves and existing authority/cleanup limits remain unchanged. No new growing cursor metadata, Native provider, volume, cleanup grant or runtime dependency is introduced. Sort remains a bounded synchronous final projection; this checkpoint does not establish arbitrary native-directory latency or every possible expensive-operation guarantee.

`captureResourceLeaseCandidateV1` now rejects missing, nonregular, linked, zero-byte and oversized files before walking physical parent ancestry. Every potentially admitted candidate still requires physical ancestry, same device, sidecar absence and the existing later exact parent/file/inode/schema/hash/closure checks before deletion. A rejected candidate can return no candidate before a parent error; it gains no authority. Transparent real lstat instrumentation measured 16,388 ancestry-root stats before the change versus four after it across two actual 8,192-unfinished-file turns. This is a concrete syscall-work reduction; enclosing command times are not a causal performance benchmark.

## Schema-First Evidence And Its Limits

Neutral JSON corpus/schema precede production implementation and are independently checked with existing Ajv. Actual ordinary Resource controls, real directory FD opening/closure and smaller physical directories exercise the implementation. Existing SQLite/Node-assert retirement ownership and primary regressions remain intact. No external runtime library/type leaks into the API.

| Case | Actual Observation In Both Modes |
|---|---|
| Copied control | Refuses before directory open |
| Ordinary immediate names | Actual `d` directory with `inside` child and `z` file returns exactly `["d","z"]`, Promise=true, frozen=true; one actual open/close, three reads; no recursion |
| Read Error/undefined plus secondary close error | Exact arbitrary primary retained after three reads and one actual close |
| Close Error/undefined | Exact arbitrary close primary retained after 1,025 reads and one delegated close |
| Cancellation | Scheduled real event-loop cancellation at 128 reads, exact primary and one close |
| Controlled cardinality DATA on real FD | 0 and 65,536 names succeed; 65,537 refuses after real FD closes; frozen deterministic name/hash result independently matches SQLite |
| Actual full 8,192 unfinished-prefix retirement | Each of two turns examines two eligible candidates, retires the actual idle 8,192-allocated-byte database, and retains all unfinished/live files; four total root ancestry stats |
| Actual prefix cancellation | Exact primary after 255 candidate-capture checks at a real cooperative point, no live/unfinished deletion, zero capture ancestry stats |

The full-count name-response test deliberately uses test-controlled directory-row DATA over an actual tracked empty directory FD. SQLite independently generates the ordered 65,536-name sequence; digest `ee68a79cad39bb0339441baf72e4b58956e2e781ab50e724fc0a562c4229d9fb` agrees. This tests bounded cardinality/order/freeze/transport/closure logic. It is not a real 65,536-entry filesystem, Native provider, physical quota or a replacement for the unqualified physical gate.

The complete physical fixture remains in the full inventory target with its unchanged vectors and 25s law clock. Multiple materialization strategies failed before API results; at 32,768 entries actual timing showed filesystem work 22,477.740ms, yields 12.448ms, total 22,505.245ms. No production full-directory timing conclusion follows. The separately named behavior target explicitly selects five tests and filters out that physical fixture. No old bound, law, output cap or failed Original was raised, lowered, healed or reset to produce green.

## Final Selected Verification

Collector 19283 completed all ten selected commands with terminal status 0. Parent/child Bun transpiler and Node compile caches are disabled. Existing command/law clocks, reserves, serial Source-file scheduling, source/type roots and previous vectors remain. These are enclosing prototype/release Source gates, not actual Native/compiler builds.

| Gate | Prototype Seconds | Release Seconds | Selected Per Mode |
|---|---:|---:|---|
| Strict |25.590|15.426|55 ordered roots, prior 54 retained |
| Eleven-file Source |105.718|85.211|157 tests / 1,643 assertions / 11 files |
| Six-file artifact |83.204|71.602|73 tests / 706 assertions / 6 files; 27 preexisting Native tests filtered |
| Actual retirement frontier + existing suite |29.172|26.950|3 Bun tests / 34 Bun assertions / 1 file plus existing Node-assert/SQLite suite |
| Explicit inventory behavior |11.011|10.144|5 tests / 65 assertions / 1 file; full physical fixture filtered |

Per mode: 238 selected Bun tests / 2,448 Bun assertions / 19 files. Each of four test commands additionally runs the mandatory 10-test/215-assertion adapter gate. These counts exclude independent Node-assert assertions. Wall times include adapter, child and cleanup work; they are current-load observations, not warm-build, CPU-saturation or Native performance guarantees.

All 28 prior scoped launch rows remain equivalent; eight full/behavior inventory test/types prototype/release rows make 36. Full gate remains discoverable despite the explicit behavior qualification subset. The strict roster is the prior 54-root roster plus the final literal inventory root in unchanged wrapper Source, not a separate compiler-command event. Existing command owners remain 📜️script.ts; moving the large ticket workflow into the permanent domain architecture remains broad-goal work.

Historical terminal evidence remains in the work log and generated receipts: copied-control RED, full physical materialization failures, controlled-cardinality transport RED, first final matrix release-frontier deadline failure, buffered-port focused deadline failure, and actual ancestry-counter RED. One redundant unchanged syntax-error invocation is recorded as a tooling sequencing error, not useful validation. Final Source was changed before new changed-implementation validation; the failures were not relabeled success. A post-census counter-parser error was fixed to read the receipt object's commands and singular Bun `test` summaries; it did not restart any test or change the census.

## Source Identity Qualification

Final before/after validation static window: 1,775 observed paths; all 73 owned identities stable; zero missing. The maximum absolute observed path is 191 UTF-16 units; this scoped census does not prove every repository path meets the limit. One shared executable runtime contract changed during verification:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json`: before `e2b9cb2b7eae301a316344875d2ec47585ee5281b335918f4acb797f9f8e67f4` (47598 bytes); after `ee786c31d7357c2635ec82390e0329e52ccc7986768ce55b66cf799f3f2810c1` (48172 bytes).

Dependency-install storage schema is a runtime contract in the repository build/bootstrap closure. Its drift is not declared harmless/nonexecuting. Individual passing results apply to their actually loaded revisions. This is static scoped identity evidence, not an unchanged complete current execution closure or independent qualification of coedited storage machinery. No unchanged test/Original/clock is retried to erase drift.

## Remaining Acceptance

The full physical inventory gate remains unqualified. Eligible but foreign/kernel-held/unretirable candidate-prefix fairness remains open. Ordinary candidate-loop progress traffic across the largest ineligible scan is not certified solely by the separate inventory traffic test. Current Native lease-image Source already integrates generic ordinary/queue-claim retirement under actual kernelEX maintenance and writer proof; no duplicate cleanup authority is needed. Actual Native close/reopen/pressure/headroom behavior is still held and unqualified.

Required broad acceptance still includes enforcing physical backends/quotas, all cache/temp/journal/metadata/headroom accounting, historical/unmanaged contents, total Source/Git/ticket/repository bounds, successful actual Nx/Cargo/rustc unchanged/changed/transitive/sibling/restore/concurrent/warm behavior, CPU use, no giant selected-module closures at growth, prototype/release runtime/size and Windows/macOS/Linux/devcontainer qualification. Logical body/catalog limits and bounded ordinary metadata scans do not establish a hard disk cap or predictable total repository size. Full goal/ticket stay active.

## Changed Implementation And Verification Inputs

- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` — 53181 bytes; SHA256 `18103a8e194105e0f2fa967808795b94bbc777cd9e1fc180996b914ef12ea5d3`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🟦️.ts` — 7480 bytes; SHA256 `34111fc08584c18170b6c62a8541200409059ace9934c600cffd4a26136b22db`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧬️schema/🔣️.json` — 195 bytes; SHA256 `5cc33dae80358ec8333797ce410775c012dc13c6f3c0fc84210b0d92d638a15d`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️frontier/🟦️.ts` — 8181 bytes; SHA256 `5d5e441ec0ddec302c1697d59cdd4c7fb057b39b12711e2359c455afc7e039cc`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🔣️.json` — 699 bytes; SHA256 `e5e1dbf1d364569e6505b865989d3609227e00e5d5fd2a0111371545852cf93f`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🧬️schema/🔣️.json` — 1556 bytes; SHA256 `4afba21082d7e585ea686bd15ecc10d90b10d414919920cc877e41c1d1b3367e`.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts` — 14163 bytes; SHA256 `b0b15983c9249c92e759b9d27a674ddaec2cfc69fdae3212bb2b2990b8a328f3`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/typescript-artifact/📜️script.ts` — 127988 bytes; SHA256 `ff35a0fd87f9bfe2210eec0fbf3287136323a52a43aa040f6fdaff086b58e292`.
- `.vscode/launch.json` — 8939290 bytes; SHA256 `d1ec52fab4ae4cd6ea9117ca5b9955eeb4a18f07e6f1f390c0f173559683ad8a`.

Raw failed/successful output, terminal receipts and identity comparisons remain temporary evidence under this active ticket's 🗑️generated folder. Preserve authored reports/inputs and delete generated output only at actual ticket completion. Repo ticket MCP capabilities are still unavailable; three execution workers remain terminal after application network permission was revoked. No alternate route, restart, invented authority or independent review is used.
