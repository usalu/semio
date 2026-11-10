# wincfg result (semio-plugin-window-config)

## Done
- `WindowConfigOwner::Mutation` (📐️owner-contract-and-mutation/🦀️.rs:14) += `protocol::OpBinaryReceiving` (fixes the 122x E0277 once at the trait). EDITED-unverified.

## Checks
- W-wincfg-103: blocked upstream (value-control-contract E0063 effect/🦀️.rs:55, a peer fixed it at 19:18).
- W-wincfg-104 (native lib, rc 1, 35 errors): none in window-config; the build stops in upstream value crates mid-port by a peer lane:
  value-shared-issuance (arc-custody/sealed: SharedOriginalBirth::publication, duplicate), value-native-encoding (♻️retirement close_step / OriginalRetirementCloseLoan, 🏭️construction), value-retained-work (list/persistent/work:58), value-retirement-queue (OriginalRetirementCloseLoan, RetainedWorkOutcome::map_err).
- The 259 window-config errors (store slot methods, RetainedWorkPoll.result, typed-owner signature) are therefore not yet re-measured on the typedpack/capture/epoch ports.

## Rerun
`chk.sh W-wincfg-105 🌎️hub/Cargo.toml native -p semio-plugin-window-config`, then wasm, then `--tests`.
Source gate not re-measured (was 250,370 / 262,144 B before the sibling ports).

STATUS: BLOCKED upstream value crates (shared-issuance, native-encoding, retained-work, retirement-queue) red at W-wincfg-104

## Update (foundation work)
W-wincfg-107: window-config still not reached; remaining red crates upstream are native-list-receiver (nested) and job-outcome/escrow-publication (declint). See 1108-result-foundation.md.
