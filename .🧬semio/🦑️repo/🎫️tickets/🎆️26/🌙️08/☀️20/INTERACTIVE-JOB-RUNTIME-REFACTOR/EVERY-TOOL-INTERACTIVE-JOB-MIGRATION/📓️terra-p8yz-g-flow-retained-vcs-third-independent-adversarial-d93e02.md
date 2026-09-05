# P8yz-g Flow Retained VCS Third Independent Adversarial Audit

Date: 2026-08-26  
Auditor: Codex — independent, read-only source/static review  
Verdict: **RED — the 13-operation oracle, byte vectors, complete terminal parsing, hostile scalar signatures, and retained production route are now source-static GREEN; five rollback-boundary `fault` vectors still do not exercise a fault at their stated boundary.**

## Scope And Method

Read in full:

- `AGENTS.md`;
- `📓️codex-p8yz-g-flow-retained-vcs-second-fresh-adversarial-audit-2026-08-26.md`;
- `📓️codex-p8yz-g-flow-retained-vcs-source-static-implementation-2026-08-26.md`;
- the current retained VCS component; and
- all three current fixture files.

This audit used only read-only fixture parsing/census, `rustfmt --check`, `git diff --check`, and static source inspection. It did not run Cargo, Nx, a Bun repository gate, Wasm, browser, cache-writing command, or make a production edit. Consequently, the references to fixture "execution" below mean the checked-in Rust test drives the live type in source; they are not a compiled/runtime-pass claim.

## GREEN — Independent 13-Feature Oracle Is Still Live

`FlowSemanticOracle::evaluate_operations` owns a separate `serde_json::Value` document/history/version evaluator for all thirteen fixture operations (`component.rs:3037–3114`). `expected_operations` separately decodes the fixture's document, page, history, versions, and fingerprint references (`3117–3143`). The test compares those two independently before feeding every operation into a real `FlowRetainedVcs`, taking/ACKing its page, incrementally closing it, extracting the actual retained state, and comparing the complete vector (`3844–3867`).

The actual extractor reads canonical document, all ten page fields, undo/redo history, and the 16-field handback (`3498–3535`). The old semantic label loop remains absent. This is a source-static GREEN for the requested independent oracle boundary.

## GREEN — Fixture-Owned Byte Protocols And Terminal Parsing

The fixture parser census found exactly three byte protocols:

| Vector | Characters | UTF-8 bytes | Expected result |
| --- | ---: | ---: | --- |
| `acceptedMultibyte` (`é界🌊️`) | 4 | 12 | accepted |
| `maximumMultibyte` (`é` × 32,768) | 32,768 | 65,536 | accepted |
| `maximumPlusOne` (`a` × 65,537) | 65,537 | 65,537 | limit |

The checked-in byte fixture law reconstructs the exact fixture encoding, asserts character and byte counts, calls live `begin_remove_widget`, checks retained source/handle/admission state, carries out fixture-defined cleanup, and compares the complete terminal state (`3920–3964`).

Every expected state resolves to an explicit document, explicit `null` page, exact history, and a complete 16-field fingerprint. The hostile parser reads all seven credits plus active/leased, undo/redo, retired action/surface owners, revision, parent revision, document generation/digest, version state, edit owner, retention, and closing (`3294–3399`). The fresh fixture census found all 13 oracle cases complete; 4 authority, 3 malformed, 5 grant, and all 24×2 transfer outputs resolve to these complete terminal states.

## GREEN — Hostile Scalar Signatures And Production Retained Route

`flow_hostile_assert_every_scalar_is_signed` recomputes a canonical fixture digest, recursively obtains every scalar path, mutates each scalar in memory, and rejects any mutation that leaves the stored signature unchanged (`3440–3496`). The law applies this to all five hostile-vector collections plus `fingerprints`, `expectedStates`, and `protocolDocuments` (`3880–3905`). Fixture signatures cover 3 byte, 4 authority, 3 malformed, 5 grant, 24 transfer, 6 fingerprint, 6 expected-state, and 1 protocol-document records.

The retained VCS production region remains lines 959–2757. The fresh route census found all ten required bounded-route tokens and none of the 23 prohibited whole-operation/whole-collection spellings:

~~~text
route_start=959 route_end=2757 route_lines=1799 forbidden=none required_present=10
~~~

A broad static raw-mutation census outside the retained VCS reports eight paths: surface node graph; Flow bridge, Wasm, artifact, catalogue, and host; renderer EngineCanvas; and plugin. It is an eight-path present-tree count, not a claim that historical labels or a historical nine-path total were freshly observed. The retained VCS production route itself is not among those external paths.

## RED — Five Declared Fault Cases Are Only Duplicate Calls After A Prior Cancel

The required control matrix is not fully adversarial at every declared transfer boundary. Five fixture boundaries use `target.primaryControl: "cancel"`:

- `afterRollbackVisibility`;
- `afterRollbackSurface`;
- `afterRollbackHistory`;
- `afterRollbackRedo`; and
- `afterRollbackSemanticOwner`.

For each, both declared outer controls expect `duplicateControl`, including the declared `fault` control. The static fixture census is:

~~~json
{
  "rollbackBoundaries": [
    "afterRollbackVisibility",
    "afterRollbackSurface",
    "afterRollbackHistory",
    "afterRollbackRedo",
    "afterRollbackSemanticOwner"
  ],
  "primaryControls": ["cancel", "cancel", "cancel", "cancel", "cancel"],
  "outerExpected": [
    [["cancel", "duplicateControl"], ["fault", "duplicateControl"]],
    [["cancel", "duplicateControl"], ["fault", "duplicateControl"]],
    [["cancel", "duplicateControl"], ["fault", "duplicateControl"]],
    [["cancel", "duplicateControl"], ["fault", "duplicateControl"]],
    [["cancel", "duplicateControl"], ["fault", "duplicateControl"]]
  ]
}
~~~

The test first polls to publication, invokes that hard-coded primary `cancel`, steps the cancellation rollback to the target, and only then loops over `cancel` and `fault` (`4062–4129`). At that point the operation is already cancelled; hence the subsequent `fault` invocation checks only duplicate-control idempotence, not `FlowRetainedVcs::fault` at `afterRollback*`.

This means 48 live method invocations are present, but the five `afterRollback*` fault protocols do **not** prove the required fault path at their named rollback state. The fixture cannot show whether a real fault entering/carrying rollback reaches the same exact terminal document/page/history/16-field handback outcome as cancel. That directly disproves the requested “every … transfer boundary … for both cancel and fault” acceptance condition.

## Required Remediation

For each of the five `afterRollback*` cases, construct a fresh session per primary control: one enters rollback with `cancel`, the other enters rollback with `fault`; then advance the identical fixture-declared rollback step and compare each complete terminal state. Preserve separate duplicate-control cases if desired, but do not use them as the only `fault` vector for a named rollback boundary. The language-neutral fixture must name the primary control and exact post-step fingerprint for both runs.

## Commands And Results

~~~sh
bun -e '<fixture-only JSON completeness census>'
rustfmt --edition 2021 --check '.../flow/vcs/component.rs'
git diff --check -- '.../flow/vcs/component.rs' '.../flow/vcs/fixtures/'
~~~

Results:

~~~json
{"json":true,"operations":13,"oracleLiveCases":true,"bytes":[{"name":"acceptedMultibyte","bytes":12,"chars":4,"result":"accepted","state":true},{"name":"maximumMultibyte","bytes":65536,"chars":32768,"result":"accepted","state":true},{"name":"maximumPlusOne","bytes":65537,"chars":65537,"result":"limit","state":true}],"authority":[true,true,true,true],"malformed":[true,true,true],"grants":[true,true,true,true,true],"transfers":24,"controls":48,"transferBoth":true,"transferComplete":true,"stateNames":["base","undoOne","redoThree","admittedZeroBytes","admittedTwelveBytes","admittedMaximumBytes"],"allFpsComplete":true,"digests":{"byteVectors":3,"authorityVectors":4,"malformedVectors":3,"grantVectors":5,"transferControlLedger":24,"fingerprints":6,"expectedStates":6,"protocolDocuments":1}}
~~~

`rustfmt --check` and `git diff --check` exited successfully with no output. The line-number evidence captured during this audit is retained as `🧪️flow-vcs-third-audit-line-evidence-2026-08-26.txt` in this ticket folder.

Until the five fresh fault-at-rollback protocols exist and their focused Rust law has been run when the embargo permits, this packet remains **RED**.
