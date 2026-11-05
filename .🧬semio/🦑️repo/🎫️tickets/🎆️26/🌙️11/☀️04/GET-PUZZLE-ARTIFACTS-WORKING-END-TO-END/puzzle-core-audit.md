# Puzzle Core Artifact Audit

Scope: Puzzle2d standalone store retirement and presence source checks; Puzzle2d, Puzzle3d, Puzzle5d semantic SQLite snapshot source contracts.

The owners already include language-neutral JSON corpora, independent Ajv schema validation, Bun SQLite relational queries, binary64 edge words, cancellation, exact grants, malformed ownership rejection and authored fixture roundtrips.

Initial declared checks are queued through repo capacity. No source changes yet.

## Canonical Mutation Corpus

The suspected missing fixture is present. The TypeScript test resolves relative to its canonical-directory root and therefore reaches the fixture correctly. All 36 referenced mutation assets exist, retain matching wire tags, and cover 36 distinct tags; all corresponding native mutation leaf sources exist. The native counterpart is wired by the MoveNode preparation crate wrapper. No repair is required for this report.

## Reproduced Failures and Fixes

Registered `repo tools exec --light bun -- test` with explicit file operands ran the isolated source-only contracts after graph checks waited for capacity. The SQLite suites execute independent Ajv and Bun SQLite engines, declared mutation fixtures, malformed-file cases, exact binary64 preservation, grant limits and cancellation. They take seconds and do not compile native components.

Puzzle2d had two stale source checks requiring literal inverse implementations in per-mutation facade files. Those implementations now reside in the shared literal-ID owner. Assertions now inspect that actual owner and retain facade cursor checks and the same neutral RFC6902/SQLite cases. Puzzle5d had one stale inventory check treating five unit key-marker structs as DSL records. The inventory now selects struct bodies, retaining exhaustive comparison against authored diff-record fixture names and each DslRecord derivation check.

Validation:

- Store-close and canonical mutation source contracts: 3 passed, 0 failed, 693 expectations, 271 ms.
- Puzzle3d SQLite source suite: 31 passed, 0 failed, from the initial combined 3d/5d run.
- Puzzle2d and Puzzle5d SQLite source suites after fixes: 98 passed, 0 failed, 2329 expectations, 4.34 s.
- Total final source coverage: 132 passing tests across five files.

Native compiler/component checks were not run in this core slice. Original graph-only queue requests were canceled without starting tests. No generated ticket outputs remain.

Files updated:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`

## Native Mounted Composition Plan

Read `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`, its router and Cargo manifest. Its `test` target owns `semio-hub-puzzle`. The library surface suite mounts all three editors/viewers and compares the installed guest codec tables with all three declared native codecs. Selected bounded Cargo invocation: `repo cargo --store=shared --configuration=prototype -- test -p semio-hub-puzzle --lib`. Fleet coordination will avoid parallel native Cargo launches.

Native invocation status: the selected owned Cargo request remains queued after more than eleven minutes. Admission first waited behind external layout requests and a demonstrator run; it now waits for two admitted layout reservations to stabilize. No native compile or test has begun, so this request is not evidence of native success.

The first direct Cargo attempt was admitted after approximately thirteen minutes but failed during unit metadata because the repository-root workspace does not contain `semio-hub-puzzle`. The hub manifest belongs to `🌎️hub/Cargo.toml`; its membership is verified. Corrected invocation: `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml -p semio-hub-puzzle --lib`. This new request is capacity queued; no native tests have run.

## Native Compile Blocker Reproduced

Corrected hub invocation admitted, refreshed its stale Cargo.lock through the owned Cargo repair, admitted 869 units and began real compilation. It failed in `semio-framework-value-native-decoding`: the decoder continuation retirement module referenced `RetirementDemand`, `RetainedCloneGrant`, and `RetainedCloneProgress` without importing them (E0425/E0422, four compiler errors). Added only those missing owned type imports in `🧰️framework/🔨️modules/🌱️value/🛬️decode/♻️retirement/🎟️continuation/🦀️.rs`. Its existing scalar-receipt neutral-fixture law remains unchanged. Re-running the same hub command; no mounted native tests have been reached yet.

Additional changed files: the continuation source above and `🌎️hub/Cargo.lock` refreshed automatically by the declared Cargo owner.

The next native retry compiler-verified the decoder import fix and compiled through framework kernel, plugin, graphics, store and Puzzle3d model. It then reproduced E0499 in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔣️receive/🦀️.rs:57`: admission remained mutably borrowed across assignment to parsed ownership. Record its terminal state immediately after returning inputs, ending that borrow before the parsed assignment. Existing receiver laws remain unchanged. The same hub test is reissued; mounted tests still unexecuted at this point.

The following retry encountered concurrent Flow operator source/manifest churn: E0433 for `semio_framework_replication` in catalogue-work. Inspection of the live owning Cargo.toml already found that dependency in [dependencies], so no change was made to that manifest or operator source. Reissued the hub command against the current files. The Flow receiver borrow correction has not yet been compiler reached in this retry.

The next hub attempt refused source admission before native compilation: live Flow publication session work source grew to 6506 bytes while its admitted extent permits 4800 bytes. Offending file: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🎟️session/🧵️work/🦀️.rs`. No boundary was raised or bypassed and no unrelated publication source was changed. Selected independent validation of the borrow fix through the smaller owning `semio-framework-os-flow-host-source` library tests with its explicit manifest; currently waiting capacity. Hub mount/codec success remains unverified.

## Final Native Handoff Status

Canceled the small source-law validation request before capacity admission to give the integration agent exclusive fleet compiler ownership for the actual Puzzle WASM rebuild. No native compiler or queued Cargo request remains in the core lane. Decoder continuation import correction has passed real compilation. Flow receiver borrow-scope correction has not yet passed compiler validation because subsequent snapshots encountered external Flow dependency churn and publication source boundaries. All three mounted codecs/editors/viewers still require the hub native laws to complete. The final source-only result remains 132 passing tests.
