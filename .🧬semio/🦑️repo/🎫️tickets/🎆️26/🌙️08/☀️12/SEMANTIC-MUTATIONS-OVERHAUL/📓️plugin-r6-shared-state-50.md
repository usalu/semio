# Plugin R6 Shared State Ownership

## Source Repair

R6's six TestConfig/TestSnapshot/TestDiff import failures came from fixture roots pointing named private types through `crate::app`, while the definitions actually lived inside `plugin_runtime::plugin_builder_contract_tests`.

The canonical `🧪️tests/🧬️test-app-mutations/🧬️document/🦀️.rs` now owns TestSnapshot, TestDiff, their unchanged codecs, and the original maximum-child clone/encoding probes. The canonical config root owns TestConfig, its unchanged text/binary codecs and ConfigRecord implementation. The fixture aggregate reexports the real types; the existing builder tests import them. Duplicate main declarations and the old TestConfig reexport were removed. Snapshot fields retain their existing pub(crate) visibility. One-item preparation, commands, runtime routes, lifecycle, Interaction, and mutation leaf behavior were not edited by this shared-state change.

## Executed Checks

- Document target source regression was retained RED at `🧪️test-mutation-direct-leaves/🧫️run-av6255` (41/43), then passed 43/43 at `run-vKqqkB`. Adding the canonical single-owner check and main/fixture capture produced 44/44 at `run-lRnzo8`.
- Config replay after the source move retained 35/36 at `🧪️test-config-selection/🧫️run-3NtnKZ`: only its obsolete private-module reexport expectation failed. Replacing that premise with canonical root ownership and main import checks produced 37/37 at `run-2lErG1`. This is a corrected source-test premise, not a newly observed native behavioral RED.
- Both final controllers ran through Bun/Nx and recorded first/final input equality. The document controller uses Ajv2020; the config controller uses Ajv2020 and jsonc-parser against neutral fixtures. They are scoped source/reference checks, not full AST resolution, robust global discovery, or Rust execution.

Final focused document input SHA-256: `e335fa356b142e86a52bccecc862e1a4fa65567fd0ccfb6905cabce8333d7241`; config root: `ede41011b3565fa1ab32950283476053d3a5b2376a98dabcfdd32606bfa6a40d`; fixture aggregate: `30b5446747f7a8bbcafe50a69c99b318a282ff6ae981b5805ee1ddffdb069a35`.

The final local checks captured Plugin main `02aa65f6d55aea517b3b19396286217ffc0e61d8b730763e9eba87804e45b5d5`. This is a run endpoint, not a continuing shared-file freeze: the separate R6 private-count and no-state lifecycle repairs are still active.

## Remaining Gate

Native Plugin compilation and all native fixture tests remain unrun from this lane. Runtime owns the next coordinated compiler inventory after all 17 mutation-fixture repairs are coherent. The no-state transient false-terminal defect identified in `📓️plugin-r6-no-state-independent-review-50.md` is being corrected separately; its previous 18/18 source result is not acceptance of that lifecycle behavior.

No cleanup, restoration, modifying Git command, or excluded-directory access occurred.

