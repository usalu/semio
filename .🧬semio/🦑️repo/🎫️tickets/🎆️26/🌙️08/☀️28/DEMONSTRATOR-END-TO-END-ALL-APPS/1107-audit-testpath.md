# 🧭️ 1107 Audit: Pane Smoke Test Path (lane aud-testpath)

Read-only. No repo runs, no sub-agents, no edits outside this ticket folder.
Method: for each pane smoke, took the package's `Cargo.toml` (member, not workspace root), its `[[test]]` target, `[dev-dependencies]`, and feature-gated deps (`testkit`, `testing`, `derive-testing`, `artifact-app-testing`, `mutation-testing`, `protocol-laws`). Resolved each test-support dep to its crate source. Counted literal regex hits for the six old-API patterns in the pane's own artifact tree (own scope) and in each test-support dep crate (dep scope). Dep scope for `framework/.../💻️os` is the whole os product (about 5190 `.rs` files), so os hits are an upper bound, not verified on the path.

Caveats:
- Counts are literal regex hits, not compile verdicts. Some hits are in test files that a smoke does not compile.
- `cargo test --test <name>` compiles only that target plus the lib, dev-deps and build scripts. `energy` runs with no `--test` filter, so all its targets compile. `gis` runs `--lib` only.
- RetainedWorkPoll `.work`/`.result` field access is a coarse heuristic (any line with `.work` or `.result` in a file mentioning RetainedWorkPoll). Treat as a lead, not a count.
- Module wiring was not fully traced. "Lib" below means the file sits in a non-test source module of the crate.

## Smoke command lines (from `C/1105b-tools/panes.sh` and lane `smoke.sh` files)

| Pane (lane owner) | Member package | Target on compile path |
| --- | --- | --- |
| generator (`proc`) | semio-s-artifact-procedural-generation3d | `--test pane-smoke` (`generator_pane`) |
| koordinator (`koordinator`) | semio-s-artifact-cad-cad | `--test cad_pane_smoke` |
| aggregator (`aggregator`) | semio-s-artifact-puzzle-3d-editor | `--test editor-gestures` (filter `aggregator_`) |
| energie (`energie`) | semio-s-artifact-energy-model-editor-mounted-tests | all targets (no `--test`) |
| aussuchen (`aussuchen`) | semio-s-artifact-sourcing-curation | `--test aussuchen_pane_smoke` |
| bearbeiten (`bearbeiten`) | semio-s-artifact-process-process3d-editor | `--test process3d_bearbeiten_pane_smoke` |
| verfolgen (`verfolgen`) | semio-s-artifact-gis-gismap-gis2d-unit-tests | `--lib` (filter `verfolgen`) |
| statik (`statik`) | semio-s-artifact-fem-3d-editor | `--test statik_pane_smoke` |

Note: `panes.sh` covers only five panes (cad, process3d, energy, fem3d, gis). Aggregator, aussuchen and generator run from their lane `smoke.sh`. The fixture lane's `smoke.sh` re-runs four of them.

## Per-pane findings (own scope = artifact tree; dep scope listed separately)

### generator (`proc`), 15 files, ret 5, close 21
- Lib files on path: `editor/🎭️modes/.../👁️preview/🫧️transient` (test, close 1), `🔺️retained-reduce-gumball-and-bounded-job-factory` (close 1), `🧵️retained-commands/📥️provisional/🎛️input` (close 7), `.../📥️provisional/🧭️transform` (close 6), `🧿️commands-and-instance-operation-owner` (close 1), `viewer/🪪️view-constants-and-instance-operation-owner` (close 1), `🚪️io` (ret 1), `💡️inferences/📐️geometry/⚙️compute/🎟️original` (ret 1), `.../💎️value/⚠️fault` (ret 1), `.../🔌️inputs/🏗️constructor` (ret 1), `.../🚂️engine` (close 1), `🌊️flow-eval-and-document-io-routes` (ret 1), `📬️artifact-store-preparation` (close 1), `🔨️modules/🏠️host` (close 1, testkit-gated).
- Test-support on path: `🧪️tests/🧰️support/🔬️retirement-driver` (close 1). This is the procedural testkit crate, and it is what the `retirement()` impl edits must reach.
- Dep scope: `semio-framework-job` (testkit) has 218 `.rs` files with close 50, ret 4, ndw 3, and 4 poll-field files. `framework/🔨️modules/🧵️job` is the owner's runtime job layer.

### koordinator / cad, 4 files, ret 1, close 3
- On path (lib): `🧬️schema/📎️references` (ret 1), `✏️editor/🔧️config-store-preparation` (close 1), `✏️editor/🗄️artifact-store-preparation` (close 1).
- Off path (test target not in `cad_pane_smoke`): `✏️editor/👥️presence/♻️retirement/🧪️tests/🔬️unit` (close 1).
- Dep scope: `semio-framework-io-sqlite-snapshot-testing-backing` and `semio-framework-test-fixtures` have no old-API hits. `semio-s-artifact-cad-cad-testing` (sample-scene) has none. `semio-plugin` (artifact-app-testing) and os-kernel contribute the shared hits below.

### aggregator / puzzle3d, 7 files, ret 5, close 5 (plus testkit-gated dep test crates)
- On path (lib): `⏳️precompute/📐️geometry/♻️retirement` (ret 3), `📐️geometry/🧮️pose-math` (ret 1), `🪣️fill/⏯️fill-run` (close 1), `🪣️fill/🔍️fill-revalidation` (close 1), `✂️clipboard-job` (close 1), `📬️store-preparation` (close 2), `📸️snapshot/🌱️play-snapshot-authority-and-codecs` (ret 1).
- Dep scope: `semio-framework-job-worker` (testkit) close 7 in 25 files, `job-worker-authority` (testkit) close 3, `puzzle-3d-editor-session` and `precompute` (testkit) have no hits, `puzzle-jobs` and `puzzle-retained` (testing) close 2 and ret 1.
- Testing-gated own files: 12 (schema/snapshot, precompute-catalogs, precompute-commands, document-model, mutations reference/*, catalog/*, vortex/*). These are the `feature = "testing"` modules the smoke pulls in.

### verfolgen / gis, 1 own file, close 2; value dep is the main exposure
- Own: `🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️unit` (close 2), a test file, off the `--lib` path.
- Dep scope, feature-gated (testkit) crates on the path:
  - `semio-framework-value` (testkit): 623 files, close 96, ret 24, adv 40, ndw 68, rr 6, rrw 1, 95 poll-field files, 7 testkit-gated files. Owner `value`. The `ndw`/`rr` hits sit in `🛬️decode/🏭️construction/🧳️slots`, `🛬️decode/🧳️owned/📦️packages/🦀️rust/tests/native-original-projection.rs`, and `🛬️decode/♻️retirement`, which is lib code. The rest are test files.
  - `semio-framework-job-worker` (testkit): close 7.
  - `semio-os-store-testing-doubles`: close 2, adv 1.
  - `os-kernel` and `plugin` shared hits (see Shared below).

### energie / energy, 8 files, ret 3, close 13, RetainedWorkPoll poll-field 2
- All targets compile, so all files are on path, including test files:
  - `🧬️schema/🔺️diff/🩹️patch` (ret 3)
  - `🧵️simulation-session/🌦️weather` (close 4)
  - `🧵️simulation-session/🧵️run-job` (close 3)
  - `🧵️simulation-session/📮️publication/📶️prepared` (close 2)
  - `🧵️simulation-session/📮️publication` (close 1) and its `🧪️tests` (close 1)
  - `✏️editor/🧮️reduce-and-helpers` (close 1)
  - `🌱️initialization/🌱️state` (close 1)
- Poll-field lead: `🧬️schema/📸️snapshot/🧬️original/🌉️bridge` and `📸️snapshot/🧳️state/🧪️tests/🧰️helpers`.
- Member has no dev-deps, so its test-only exposure is the os-kernel/plugin shared set.

### aussuchen / sourcing, 2 files, close 2
- On path (lib): `✏️editor/📬️config-store-preparation` (close 1), `✏️editor/🗄️artifact-store-preparation` (close 1).
- Dep scope: `semio-framework-test-original-snapshot-owners` (close 1). Host-configuration testkit crate (3 gated files, no hits).

### bearbeiten / process3d, 2 files, close 6
- On path (lib): `✏️editor/📬️store-preparation` (close 2).
- Off path (test target `retained-laws` not in the smoke): `🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-laws` (close 4).
- Testkit-gated dep modules: `host/🧰️owned/🧵️retained-cursors`, `host/🧰️owned` (close 4 counted in `🧰️owned`), and `io/💾️binary/🧬️mutations`. These are the `feature = "testkit"` surfaces `semio-s-artifact-process-process3d-host` and `-mutations` expose to the smoke.

### statik / fem3d, 5 files, ret 1, close 1, ndw 6
- On path (lib): `✏️editor/📬️store-preparation` (close 1), `🧬️schema/📸️snapshot/native.rs` (ret 1).
- Off path (test targets not in `statik_pane_smoke`): `🧪️tests/identity-index-tests.rs` (ndw 2), `🧪️tests/loads/receiver-tests.rs` (ndw 2), `🧪️tests/native-tests.rs` (ndw 2). These are the NativeDecodeWork users. They will not block the smoke verdict unless someone adds them to the smoke.
- Dep: `semio-framework-test-fixtures` clean. `fem-3d-editor-model-window` (testkit) gated 1 file, no hits. Shared `plugin` hits as below.

## Dedup into shared crates (what blocks several panes at once)

| Shared crate (on compile path via) | Pattern hits (dep scope) | Panes affected | Suggested owner |
| --- | --- | --- | --- |
| `framework/.../🌱️value` (testkit) | ndw 68, rr 6, close 96, ret 24, adv 40; 95 poll-field files; 7 gated | gis (direct); every pane transitively if value lib changes | `value` (a1bf0499e1a315f51) |
| `framework/.../🧵️job/👷️worker` (testkit) and `worker/🎟️authority` (testkit) | close 7 + 3 | gis, aggregator | `runtime` (job layer) |
| `framework/.../🧵️job` (testkit, procedural's dep) | close 50, ret 4, ndw 3 | generator | `runtime` (job), `jobsplit` for context budget |
| `framework/.../💻️os` (os-kernel protocol-laws/mutation-testing) | ret 143, close 825, adv 109, rrw 10, ndw 19 (upper bound) | all eight panes | `runtime` (ret/adv/ndw), `runtime-b` (close/publication retirement) |
| `framework/.../🔌️plugin` (artifact-app-testing) | ret 50, close 256, adv 25, rrw 4 (upper bound) | all eight panes | `runtime` |
| `framework/.../🏪️store/🧪️testing/🎭️doubles` | close 2, adv 1 | gis | `fixture` (native pane harness) |
| `framework/.../🧪️test/🛬️original-snapshot-owners` | close 1 | aussuchen | `testdebt-b` (testing admission) or `fixture` |
| procedural testkit `🧪️tests/🧰️support/🔬️retirement-driver` | close 1 | generator | `proc` |
| Puzzle plugin jobs/retained (testing) | close 2, ret 1 | aggregator | `aggregator` |

Lib-source hits that are not in a shared crate are owned by their pane lane (see per-pane lists). The most repeated pattern across panes is `close_step(grant` in `store-preparation` modules (cad, process3d, puzzle3d, procedural, sourcing, fem3d, energy). Those can be ported in one sweep by the shared store-preparation convention, which is an owner decision, not a fix made here.

## Suggested priority for fleet (fastest route to smoke verdicts)

1. `value` lane: resolve the gis testkit exposure (ndw/rr in `decode/slots` and `decode/retirement` lib files). This is the only shared hit that sits in lib code and feeds a pane directly.
2. `runtime` and `runtime-b`: the os/plugin/job testkit closures. The upper-bound numbers are too coarse to act on; the runtime owner should split lib vs test first.
3. Per-pane lib-source files, owners in the table above. Start with generator (procedural lib close 21), then puzzle3d (lib ret 5), then energy (all targets compile).
4. Off-path test files (cad presence, process3d retained-laws, statik native/receiver/identity tests, gis host unit) can be deferred, because the named smoke target does not compile them.

## Lane owner lookup (from `C/1107-lanes.md`)
proc a9416d3c03e6a69e5 · koordinator a35a27af5fbf1186e · aggregator a78c08da65901d641 · energie ac7ed96ed1744f7ae · aussuchen a5aa2484f346ef32d · bearbeiten a02fc4a3e9a4931c6 · verfolgen a8bed27907dc35843 · statik ae8bea3f04b810457 · value a1bf0499e1a315f51 · runtime a8f570504dc3f4b48 · runtime-b a1b41fb32e86f0b71 · fixture a16c12027741661d5 · testdebt-a a0b8457b9a4c9ad6e · testdebt-b aa6f0842b625ee6e6.
