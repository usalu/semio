# Isolated Native Workspace Gate — 2026-11-05

Read-only audit of the Remodel artifact workspace and its current native test roots. No build, compiler, installation or test execution was started. The isolated manifest currently explicitly lists 56 members; guest, oracle and bridge trees are excluded as workspace members. Oracle crates used as dev dependencies are still compiled where selected; exclusion is not evidence that their API is untested.

## Recommended Single Comprehensive Native Command

Run once after the currently owned focused invocations settle, through repository ingress:

```powershell
repo cargo --store=shared --configuration=prototype --parallelism=4 -- test --manifest-path "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml" --workspace
```

Do not append `--lib`, a test-name filter or `--no-run` to this final gate: they would omit integration targets or execution. Cargo's ordinary workspace test selection covers member unit libraries, integration tests and doctests; this is distinct from compiling dependencies as production libraries. Do not start a duplicate while existing owner jobs are compiling the same closure. Runtime browser/WASM/TypeScript acceptance remains separate.

## Scope Beyond Focused Engine/Session/Editor Gates

| Law Area | Actual Owning Sources / Examples of Assertions | Why Focused Tests Do Not Replace It |
|---|---|---|
| Artifact Family / Snapshot | artifact root unit: default placeholder mesh, default/populated JSON roundtrip, exact float32/byte buffers; standard/subset declarations and native artifact definition | An editor integration harness compiles its artifact dependency without executing that dependency's unit tests. |
| Mutation Vocabulary | mutations unit: create/delete stream/assets, sync/frame/source/calibration/rig/GCP/params/results inverse laws, middle-member position restoration, diff absorption, concurrent operation convergence, every variant op text roundtrip | Engine/session acceptance exercises selected result mutations, not every domain variant or canonical codec. |
| Neutral Mutation Vectors | committed vector law owner: applied/no-op/rejected/inverse/canonical laws; per-family cases for stream/media/calibration/params/results/commit-reconstruction | Vector helper package alone is not all consuming family crates. The workspace selects the split mutation owners and their own tests. |
| Model/Text/Binary | model package explicitly owns snapshot and snapshot text/binary/SQLite codec modules; mutations package explicitly owns text/binary mutation codec modules; text default/populated DSL roundtrips, binary op/text agreement and store/document replay | Successful photo import or reconstruction does not prove complete persistence roundtrip. |
| SQLite Ownership / Native Precision | artifact root explicitly includes SQLite typed and general tests: exact typed parent custody, actual SQLite comparisons, externally edited relations, all structured fragments, exact owned IEEE words/scalars, bounds and cancellation phases | These are not included in the external editor integration entry, nor in engine/session units. |
| I/O Interchange | IO package selected own module tests: PLY/LAS cloud recovery, OBJ/STL/GLTF durable mesh import/export, PNG input/output and no-content refusal, exact JSON/DSL, direct hop uniqueness | An editor mesh export assertion covers one route, not all declared directed format pairs. |
| Independent Mathematical Owners | algebra, lie, signal, spatial, optimize and inference members: matrix/transform/FFT roundtrips, deterministic/default inference, independent typed dependency oracle | Compiling these dependencies does not execute their independent numerical laws. |
| Additional Engines / Modes / Viewer | images/video/h264/camera/feature/SfM/geometry/dense/mesh subowners/motion/geo, modes/panels/kit/viewer/example quarters | The bridge test roster and external editor harness select only some tests. A workspace invocation executes each registered owner suite. |

## Source Binding Facts and Evidence Gaps

The artifact root explicitly includes root unit tests and two SQLite test modules under cfg(test). The root Cargo manifest declares a separate integration test named editor; its source includes the main editor, example, command, panel and window/config tests. The model and mutations package roots explicitly bind their codec source modules. These are actual module declarations, not filename-only assumptions.

A directory guessed as snapshot/🧪️tests/🔬️unit is absent; that is not itself missing coverage because snapshot codec laws live at their respective IO owners and artifact root tests. Likewise, many per-mutation sources use a shared test function name `holds_its_laws`; a global name filter cannot establish exhaustive family coverage. Source presence does not establish a pass count.

No new concrete current-contract source error was established in this static audit. The outstanding evidence gap is execution of all 56 member test units and the artifact editor integration target together after current repairs, including test-feature unification and actual native codec/dev-oracle APIs. The prior lightweight source/fixture oracles establish selected neutral vector values and bindings; they cannot substitute for these native laws or prove Rust type checking. The comprehensive gate may reveal errors in members that focused package tests only compiled as dependencies. Record actual result before calling this workspace verified.
