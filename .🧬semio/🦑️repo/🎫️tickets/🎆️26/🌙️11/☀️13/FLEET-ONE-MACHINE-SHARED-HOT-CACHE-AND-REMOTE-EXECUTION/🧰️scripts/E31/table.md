# Table E31 (continuation of E30)

Latest result per file after E30 and E31 runs, failures only (`repo test-file --json`, one file per call, load 25-45, 2026-11-22 07:00-08:50). Passing files omitted.

Files run: 662 of 662; 81 failed, 581 passed.

| file (relative to build) | suite | status | pass/fail | s | cause / first error |
|---|---|---|---|---|---|
| ⏱️profile/📊️cpu/🧪️tests/🟦️.ts | build-cpu-profile | FIXED | - | - | fixed by E30, 5/5 |
| ⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts | build-command-line | FIXED | - | - | fixed by E30 (async pool), 1/1 |
| 🌐️remote-cache/🧶️nx/🚦️single-flight/🧪️tests/🟦️.ts | build-remote-cache | FIXED | - | - | fixed by E30, 15/15 |
| 🌐️remote-execution/🚀️offload/🧪️tests/⏱️timeouts/🟦️.ts | build-remote-execution | FIXED | - | - | fixed (env isolation), 6/6 |
| 🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster/🟦️.ts | build-remote-execution | FIXED | - | - | fixed (env isolation), 9/9 |
| 🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts | build-remote-execution | FIXED | - | - | fixed (env isolation), 31/31 |
| 🌐️remote-execution/🛰️sidecar/🚦️ready/⏳️tools-grace/🧪️tests/📈️standing/🟦️.ts | build-remote-execution | FIXED E37 | 0/2 | 1 | 1/0 (library envelope) |
| 🌐️remote-execution/🛰️sidecar/🚦️ready/⏳️tools-grace/🧪️tests/🔁️series/🟦️.ts | build-remote-execution | FIXED E37 | 1/2 | 0.9 | 2/0 (library envelope) |
| 🌐️remote-execution/🛰️sidecar/🚦️ready/⏳️tools-grace/🧪️tests/🔄️handoff/🟦️.ts | build-remote-execution | failed | 0/1 | 8.6 | remote-execution sidecar / bazel (see report): 1 test file(s): 🌐️remote-execution/🛰️sidecar/🚦️ready/⏳️tools-grace/🧪️tests/🔄️handoff/🟦️.ts |
| 🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🌱️graph-progress/🟦️.ts | build-remote-execution | failed | 2/1 | 2 | remote-execution sidecar / bazel (see report): ile(s): 🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/🌱️graph-progress/🟦️.ts |
| 🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🧪️tests/🟦️.ts | build-remote-execution | failed | 15/1 | 36.8 | remote-execution sidecar / bazel (see report): t file(s): 🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧹️containment/🧪️tests/🟦️.ts |
| 🌐️remote-execution/🛰️sidecar/🧪️tests/🧮️closure/🟦️.ts | build-remote-execution | FIXED | - | - | fixed (sidecar static import), 5/5 |
| 🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts | build-remote-execution | failed | 0/1 | 248.5 | Bazel hermetic runner cannot create /tmp/<hash> inside the test sandbox (bazel lane):  build-remote-execution failed in 1 test file(s): 🌐️remote-execution/🧪️tests/🔧️bazel/🟦️.ts |
| 🏃️execution/🧪️tests/🟦️.ts | build-execution | failed | None/None | 420.1 | not analysed (first error shown): Original Script deadline refused |
| 🏞️task-view/🧱️hermetic/🧪️tests/🟦️.ts | build-task-view | FIXED | - | - | fixed (short scratch), 15/15 |
| 🏡️resident/📜️journal/🔭️world/🧪️tests/🟦️.ts | build-resident | FIXED E37 | 0/1 | 12.3 | gated by nativeWatcherUsableV1 (skipped with console warning) |
| 🏡️resident/📜️journal/🧪️tests/🎲️fuzz/🟦️.ts | build-resident | FIXED E37 | 0/1 | 10.2 | gated by nativeWatcherUsableV1 |
| 🏡️resident/📜️journal/🧪️tests/🏠️host/🟦️.ts | build-resident | FIXED E37 | 0/1 | 10 | gated by nativeWatcherUsableV1 |
| 🏡️resident/📜️journal/🧪️tests/🟦️.ts | build-resident | FIXED E37 | 7/2 | 34.6 | 7/0, two native tests gated |
| 🏡️resident/🕰️graph-hooks/🧪️tests/📖️direct/🟦️.ts | build-resident | FIXED E37 | 0/1 | 10.2 | gated by nativeWatcherUsableV1 |
| 🏡️resident/🧪️tests/🌐️host/🟦️.ts | build-resident | FIXED E37 | 38/1 | 39.8 | 39/0 (failure block keeps stack frames) |
| 🏡️resident/🧪️tests/🏗️provision/🟦️.ts | build-resident | failed | 14/4 | 98.7 | environment: FSEvents delivers no events on this host (journal canary), measured with a bare fs.watch: Test suite build-resident failed in 1 test file(s): 🏡️resident/🧪️tests/🏗️provision/🟦️.ts |
| 🏡️resident/🧪️tests/🔁️restart/🟦️.ts | build-resident | FIXED E37 | 7/1 | 2.8 | 8/0 (timing flake, passes) |
| 🏡️resident/🧪️tests/🛑️cancel/🟦️.ts | build-resident | FIXED E37 | 26/11 | 12.9 | 37/0 (short runtime dir) |
| 👁️observation/🧪️tests/📦️projects/🟦️.ts | build-observation | failed | 0/2 | 92.6 | not analysed (first error shown): t suite build-observation failed in 1 test file(s): 👁️observation/🧪️tests/📦️projects/🟦️.ts |
| 📐️conformance/🧪️tests/🟦️.ts | build-doctor-preflight | failed | None/None | 465.2 | test-file envelope deadline (file slower than budget at load 30-45); crash of the run fixed: Original Script process deadline exhausted |
| 📖️storybook/💾️storage/🧪️tests/🟦️.ts | build-storybook | failed | 1/3 | 2.1 | not analysed (first error shown): Test suite build-storybook failed in 1 test file(s): 📖️storybook/💾️storage/🧪️tests/🟦️.ts |
| 📖️storybook/🧰️preparation/🕸️engines/🧪️tests/🟦️.ts | build-storybook | failed | 3/1 | 1.5 | not analysed (first error shown): uild-storybook failed in 1 test file(s): 📖️storybook/🧰️preparation/🕸️engines/🧪️tests/🟦️.ts |
| 📥️caller/🧪️tests/📥️birth/🟦️.ts | build-caller-birth | failed | 13/1 | 8.7 | not analysed (first error shown): Test suite build-caller-birth failed in 1 test file(s): 📥️caller/🧪️tests/📥️birth/🟦️.ts |
| 📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🟦️.ts | build-inputs-revision | failed | 2/1 | 8.6 | graph / inputs / source content test (other lane): uild-inputs-revision failed in 1 test file(s): 📥️inputs/🌳️owner-view/🧪️tests/🏝️reach/🟦️.ts |
| 📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts | build-inputs-selected | failed | 1/1 | 3.3 | graph / inputs / source content test (other lane): ild-inputs-selected failed in 1 test file(s): 📥️inputs/🎯️selected/📦️snapshot/🧪️tests/🟦️.ts |
| 📥️inputs/🎯️selected/🧪️tests/🟦️.ts | build-inputs-selected | failed | None/None | 7.6 | graph / inputs / source content test (other lane): st suite build-inputs-selected failed in 1 test file(s): 📥️inputs/🎯️selected/🧪️tests/🟦️.ts |
| 📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts | build-inputs-storage | failed | 6/1 | 135 | graph / inputs / source content test (other lane): e build-inputs-storage failed in 1 test file(s): 📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts |
| 📥️inputs/💾️storage/🧪️tests/🟦️.ts | build-inputs-storage | failed | 23/1 | 89.7 | graph / inputs / source content test (other lane): Test suite build-inputs-storage failed in 1 test file(s): 📥️inputs/💾️storage/🧪️tests/🟦️.ts |
| 📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts | build-inputs-revision | failed | 20/1 | 37.1 | graph / inputs / source content test (other lane): suite build-inputs-revision failed in 1 test file(s): 📥️inputs/🔭️read-oracle/🧪️tests/🟦️.ts |
| 📥️inputs/🧮️digest/🧪️tests/🔒️batch-snapshot/🟦️.ts | build-inputs-revision | failed | 1/1 | 6.8 | declared birth / suite row grant drift (needs `capabilities` on the suite row or the launcher; test-file and suite share it): inputs-revision failed in 1 test file(s): 📥️inputs/🧮️digest/🧪️tests/🔒️batch-snapshot/🟦️.ts |
| 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts | build-source-closure | failed | 17/1 | 165.9 | graph / inputs / source content test (other lane): ild-source-closure failed in 1 test file(s): 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts |
| 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧭️boundary/🟦️.ts | build-source-closure | failed | 3/2 | 37.5 | graph / inputs / source content test (other lane): closure failed in 1 test file(s): 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧭️boundary/🟦️.ts |
| 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts | build-source-closure | failed | 4/1 | 255.7 | graph / inputs / source content test (other lane): failed in 1 test file(s): 📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts |
| 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/🟦️.ts | build-source-closure | failed | 0/2 | 88 | test-file envelope deadline (file slower than budget at load 30-45); crash of the run fixed:  in 1 test file(s): 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/👓️reader-lifetime/🟦️.ts |
| 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/🟦️.ts | build-source-closure | failed | 0/2 | 26 | graph / inputs / source content test (other lane): test file(s): 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/⏱️tools/🟦️.ts |
| 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/🟦️.ts | build-source-closure | failed | 0/2 | 135.3 | graph / inputs / source content test (other lane): ed in 1 test file(s): 📥️source/🔏️closure/🚪️command/🔌️runtime/🧪️tests/📋️artifact-plan/🟦️.ts |
| 📥️source/🔏️closure/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts | build-source-closure | failed | 0/2 | 4 | declared birth / suite row grant drift (needs `capabilities` on the suite row or the launcher; test-file and suite share it): e-closure failed in 1 test file(s): 📥️source/🔏️closure/🛂️readmission/🧪️tests/🏃️tasks/🟦️.ts |
| 📥️source/🛗️zero-touch/🧪️tests/🟦️.ts | build-source-closure | failed | 33/4 | 84.1 | graph / inputs / source content test (other lane): t suite build-source-closure failed in 1 test file(s): 📥️source/🛗️zero-touch/🧪️tests/🟦️.ts |
| 🔒️leases/⌨️command/🧪️tests/🧮️status/🟦️.ts | build-leases-command | failed | 4/1 | 8.3 | not analysed (first error shown): e build-leases-command failed in 1 test file(s): 🔒️leases/⌨️command/🧪️tests/🧮️status/🟦️.ts |
| 🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts | build-bazel-execution | failed | 0/1 | 41.7 | closure copy lacks a dynamically imported module: el-execution failed in 1 test file(s): 🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️classes/🟦️.ts |
| 🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts | build-bazel-execution | failed | 0/1 | 46 | not analysed (first error shown): l-execution failed in 1 test file(s): 🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts |
| 🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts | build-bazel-snapshots | FIXED | - | - | fixed by E30, 1/1 |
| 🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts | build-bazel-snapshots | FIXED | - | - | fixed by E30, 5/5 |
| 🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔒️planned/🟦️.ts | build-bazel-snapshots | FIXED | - | - | fixed by E30, 2/2 |
| 🔧️tools/🏗️bazel/🧪️tests/📥️paths/🟦️.ts | build-bazel-contracts | FIXED | - | - | fixed by E30, 11/11 |
| 🔧️tools/🏗️bazel/🧪️tests/🟦️.ts | build-bazel-contracts | FIXED | - | - | fixed by E30, 15/15 |
| 🔬️test-file/🧪️tests/🧫️fixtures/🔴️failing/🟦️.ts | build-test-file | FIXED | - | - | intentional failing fixture (counted by the census, not a defect) |
| 🔭️witness/🧪️tests/🟦️.ts | build-observation | FIXED | - | - | fixed by E30, 13/13 |
| 🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts | build-inference-projects | FIXED | - | - | fixed by E30, 30/30 |
| 🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts | build-inference-projects | failed | 21/1 | 8.7 | graph / inputs / source content test (other lane): ld-inference-projects failed in 1 test file(s): 🕸️graph/📥️inference/🔖️stamps/🧪️tests/🟦️.ts |
| 🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts | build-inference | failed | 7/2 | 187.3 | graph / inputs / source content test (other lane): uild-inference failed in 1 test file(s): 🕸️graph/📥️inference/🧪️tests/🎓️course-inputs/🟦️.ts |
| 🕸️graph/📥️inference/🧪️tests/🚶️repository-walks/🟦️.ts | build-inference | failed | 6/2 | 53.1 | graph / inputs / source content test (other lane): d-inference failed in 1 test file(s): 🕸️graph/📥️inference/🧪️tests/🚶️repository-walks/🟦️.ts |
| 🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts | build-inference | failed | 20/2 | 25.6 | graph / inputs / source content test (other lane): uild-inference failed in 1 test file(s): 🕸️graph/📥️inference/🧪️tests/🧷️closure-facts/🟦️.ts |
| 🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts | build-inference-script-inputs | failed | 24/1 | 138.2 | graph / inputs / source content test (other lane): nputs failed in 1 test file(s): 🕸️graph/📥️inference/🧭️script-inputs/🔀️router/🧪️tests/🟦️.ts |
| 🕸️graph/🕰️host/🧪️tests/🌊️wave-prime/🧫️fixture.ts | build-graph-protocol | failed | 0/1 | 1.7 | graph / inputs / source content test (other lane): -graph-protocol failed in 1 test file(s): 🕸️graph/🕰️host/🧪️tests/🌊️wave-prime/🧫️fixture.ts |
| 🕸️graph/🚪️admission/🧪️tests/🔎️show-project-scope/🟦️.ts | build-graph-protocol | failed | 2/1 | 1.1 | graph / inputs / source content test (other lane): -protocol failed in 1 test file(s): 🕸️graph/🚪️admission/🧪️tests/🔎️show-project-scope/🟦️.ts |
| 🕸️graph/🧪️tests/🔓️sealed-reuse/🟦️.ts | build-graph-protocol | failed | 8/6 | 2.1 | graph / inputs / source content test (other lane):  suite build-graph-protocol failed in 1 test file(s): 🕸️graph/🧪️tests/🔓️sealed-reuse/🟦️.ts |
| 🕸️graph/🧪️tests/🔬️analysis-providers/🟦️.ts | build-graph-protocol | failed | 0/1 | 4.9 | graph / inputs / source content test (other lane):  build-graph-protocol failed in 1 test file(s): 🕸️graph/🧪️tests/🔬️analysis-providers/🟦️.ts |
| 🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts | build-command-line | FIXED E37 | 5/7 | 14.9 | 11/0 (library envelope) |
| 🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts | build-parsers | failed | 2/2 | 3.1 | not analysed (first error shown): Test suite build-parsers failed in 1 test file(s): 🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts |
| 🚢️fleet-status/🧪️tests/🟦️.ts | build-fleet-status | failed | 15/1 | 13.7 | not analysed (first error shown): Test suite build-fleet-status failed in 1 test file(s): 🚢️fleet-status/🧪️tests/🟦️.ts |
| 🧪️tests/🕸️graph/🎯️source-demand/📋️scope/🟦️.ts | build-workspace-graph-laws | failed | 3/1 | 5.4 | graph / inputs / source content test (other lane): rkspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/🎯️source-demand/📋️scope/🟦️.ts |
| 🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts | build-workspace-graph-laws | failed | 33/1 | 33.1 | graph / inputs / source content test (other lane): build-workspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts |
| 🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts | build-workspace-graph-laws | failed | 6/6 | 75.4 | declared birth / suite row grant drift (needs `capabilities` on the suite row or the launcher; test-file and suite share it): -workspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts |
| 🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts | build-workspace-graph-laws | failed | 2/1 | 3.8 | graph / inputs / source content test (other lane): orkspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts |
| 🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts | build-workspace-graph-laws | failed | 9/5 | 84.9 | graph / inputs / source content test (other lane): uild-workspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts |
| 🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts | build-workspace-graph-laws | failed | 10/6 | 6.7 | graph / inputs / source content test (other lane):  build-workspace-graph-laws failed in 1 test file(s): 🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts |
| 🧪️tests/🟦️.ts | build-workspace-laws | failed | 3/2 | 16.9 | test-file envelope deadline (file slower than budget at load 30-45); crash of the run fixed: Test suite build-workspace-laws failed in 1 test file(s): 🧪️tests/🟦️.ts |
| 🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/🧪️tests/🟦️.ts | build-command-line | failed | 6/1 | 9.7 | graph / inputs / source content test (other lane): d-line failed in 1 test file(s): 🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/🧪️tests/🟦️.ts |
| 🧭️selection/🎯️scope/🧪️tests/🧶️nx/🟦️.ts | build-targets-selection | failed | 0/2 | 25.5 | not analysed (first error shown): e build-targets-selection failed in 1 test file(s): 🧭️selection/🎯️scope/🧪️tests/🧶️nx/🟦️.ts |
| 🧭️selection/👁️watch/🧪️tests/🎟️phase/🟦️.ts | build-targets-selection | FIXED E37 | 2/18 | 3 | 19/0 (library envelope) |
| 🧭️selection/👁️watch/🧪️tests/🟦️.ts | build-targets-selection | failed | 2/2 | 5 | not analysed (first error shown):  suite build-targets-selection failed in 1 test file(s): 🧭️selection/👁️watch/🧪️tests/🟦️.ts |
| 🧭️selection/📥️closure/🧪️tests/🟦️.ts | build-targets-selection | failed | 0/3 | 33.9 | not analysed (first error shown): uite build-targets-selection failed in 1 test file(s): 🧭️selection/📥️closure/🧪️tests/🟦️.ts |
| 🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts | build-doctor-preflight | failed | 15/1 | 9.4 | not analysed (first error shown):  suite build-doctor-preflight failed in 1 test file(s): 🧾️ci-report/🗂️suites/🧪️tests/🟦️.ts |
| 🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts | build-doctor-preflight | FIXED | - | - | fixed (short scratch), 9/9 |


## E37 additions
- 🏡️resident/🧪️tests/🏗️provision: 14 pass / 1 skip / 2 fail (watcher laws gated); the two failures are `ReferenceError residentWatcherInstalledV1 is not defined` in the extracted `provisionNxTools` text (caching bootstrap tools owner).
- 📥️source readmission tasks / lifecycle / mismatch: birth now agrees with the row, remaining failure is the product refusal of `.cache` paths (`External implementation runtime dependency refused`) because scratch:checkout puts the artifact dir under `.cache/test-sandbox`.
- 🧪️tests/🕸️graph optional-cache 9/5, portable-cli 10/6, pinned-publication 6/6, inputs batch-snapshot 1/1: birth no longer the cause (graph/inputs lane).
- 🗂️workspaces/🦀️cargo/🧪️tests/🟦️.ts: still 16/11, fixture helpers now run in a child; next blocker is the fixture grant (repositoryRoot/codeRoot vs private repository workspace).
- 🗂️suites/🧪️tests/🪪️birth-row: new law, 1/1.
