# Gitlink 58 Scoped-Out Descendant: Actual RED

## Executed Result

The accepted source-review finding is now a retained actual regression, not only a control-flow inference.

[Actual run-OUFa4G](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🧫️run-OUFa4G/📓️receipt.md): **96/96 reference, 30/31 actual projector, 1/1 original invalid-file160000 retention**, exit 1. The new case is the only failing law. `failure:null`, `drift:[]`; all ten source/input endpoints were stable.

The actual output is structurally schema-valid and leaves its input unchanged, but violates the desired rejection policy:

```json
{
  "exact": false,
  "inputUnchanged": true,
  "outputSchema": {
    "valid": true,
    "errors": []
  },
  "existingFieldsEqual": false,
  "expected": {
    "schemaVersion": 1,
    "scope": "owned",
    "status": "rejected",
    "observations": [],
    "diagnostics": [
      {
        "code": "repository-boundary-descendant",
        "path": "foreign/module/file.rs",
        "message": "Candidate is below an index-owned repository boundary"
      }
    ]
  },
  "actual": {
    "schemaVersion": 1,
    "scope": "owned",
    "status": "complete",
    "observations": [],
    "diagnostics": []
  }
}
```

This executes the actual exported pure projector only. `foreign/module` and its descendant are supplied neutral input strings; no foreign/nested repository content read, lstat, walk, Git invocation or source ownership discovery is inferred.

Whole N was `d6922221a330e285cbc31232a90e30ece0991d08e90d904598cb352267585a2a`; exact projector declaration `df769355c0b1be62b002cdfd5ef55deb7c791927247b6b496f861981ee024460`. No N edit was made by this lane.

Receipt SHA-256: `51456a49d8aa10fec1c82daf14225dbc1bb4608b400a30084e8a9245f34f6ef7`.

## Authored Data Change

Exactly one reviewed case, `scoped-out-supplied-repository-descendant-still-rejects`, was appended to:

- [Canonical vectors](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🧪️source-admission/🔣️.json): 59 → 60 projector cases.
- [Ticket vectors](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🔣️vectors.json): 30 → 31 projector cases.

The input uses scope `owned`, a supplied safe directory040000 with one tracked stage-zero160000 index entry at `foreign/module`, and a supplied untracked file row at `foreign/module/file.rs`. The desired scoped observation list is empty, while the contradictory supplied descendant still produces `repository-boundary-descendant` and rejected status.

All existing case records and all 24 schema cases were retained. The existing schemas, canonical test code, ticket controllers, golden outcomes and prior receipts were not rewritten. The canonical roster is now **60 + 5 malformed candidates + 24 schema cases + 1 envelope = 90 tests**; this preserves the previous 89 laws and adds one.

[Reference/preservation run mount-reference-lX9Ufl](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-gitlink-boundary-58/🧫️mount-reference-lX9Ufl/📓️receipt.md): **261/261**, exit 0, no subject import/execution, all eight fixed endpoints stable. The strict actual canonical schema and all authored records validated, with original fields and reviewed complete vectors preserved. Receipt: `2cbd55c54454bed9956b8217727d251c7784d32de4093a3f0881e234ae488513`.

## Frozen Hashes for This RED

| Artifact | Before this append | Executed RED |
| --- | --- | --- |
| Canonical vectors | `9afd1dca50ab9addd82a1f84cb7bd832c67cd649c0a36df96d846ad52f362964` | `1545abf1efda95bb71bad64ad38457a34e8c295d92e5299cbc21b9dc41b0d099` |
| Ticket vectors | `6cd0dfa02223d11c2a1a86302daf14613eccbcb9be28d25134f8c3216407108e` | `eef269ec493fcfca9440968e9aa970de0836496f88f3155aad1c420d1fd629ef` |
| Canonical schema | unchanged | `1b88f7dfd1cd8f4809e690225af22251c798f7fac4526d993301eedca04afbc4` |
| Desired ticket schema | unchanged | `abf2569aa5517e76905f62c6f7a9c3cb5214e63c5020fc47e6a5398323d9ce83` |
| Ticket actual controller | unchanged | `dcae50e6751442c3d7ed3551618ada59a683903a22f64e4e73c4d33cde2a8838` |

Canonical test source remains `951c4fa318412ec9f616289306b875ea447ea8ba74b4f20b3fd9c837274d3d00`; canonical-reference controller remains `3d17841fcbd681c0c5ba2ae6d72c83d120a3b0fedac4ffc6669cec36e948977b`.

## Root Handoff

The required root-owned correction is still only diagnosis ordering: diagnose a safe supplied descendant before scope filtering, while retaining the scoped observation list. Source was released after the actual RED terminal receipt.

No production fix, canonical 90-test subject execution, IO/global roster run, native test, budget change or launch edit occurred here. Root owns the production correction and subsequent GREEN replay. Earlier 30/30 GREEN and original 0/30 RED evidence remain unchanged.

