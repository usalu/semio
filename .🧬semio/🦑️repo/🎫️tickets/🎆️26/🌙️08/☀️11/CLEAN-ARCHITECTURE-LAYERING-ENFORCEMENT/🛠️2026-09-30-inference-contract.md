# Neutral Reconcile Contract and Transport Boundary

The framework job module owns request-id reconciliation envelopes, their schema, language-neutral JSON cases, and TypeScript/Rust implementations. The generic envelope accepts an application-owned decoder for the recovered payload; neutral files contain no GIS service, receipt, geometry, or approval dependency.

The OS worker imports that envelope and decodes its own payload through its existing directory receipt and event parsers. Hub retains its real inference payload specialization and invokes the neutral envelope parser. The prior TypeScript hub request forwarding API and Rust request public type are removed. Hub decodes its private transport fields into the framework request type. All live wire samples use `semio.framework.job-reconcile/v1` and `semio.framework.job-reconcile-result/v1`.

The directory access-change shared fixture moves to OS directory, which already owns the stream schema; the hub's reader-rule law consumes that fixture. The OS refusal law uses a distinct framework replication transport fixture and schema, eliminating its imports of implementation-owned schema/fixtures.

## Integration

Framework owning project: `@semio-tech/framework-job-rs`; target `canonical-architecture` calls `bun ./📜️script.ts canonical-architecture`. This checks the shared fixture through TypeScript plus Ajv, then the Rust implementation through the same language-neutral cases using existing dev-only serde_json. No runtime dependencies added. Register the target in launch.json using the existing job test grouping. Root generic `run-many -t canonical-architecture --all --exclude workspace` will collect it.

Existing hub oracle target: `os-hub:gis-inference-ledger-oracle` now executes the neutral proof and checks every reconcile payload fixture through both hub and worker decoders. JSON schema request export moved from hub to the neutral contract; schema catalog must be refreshed. Hostile-input schema references must be reconciled with that change before running broad hub checks.

## Verification

Focused Bun/Nx checks below completed. All transient outputs remain under the shared ticket generated directory for coordinator cleanup.

## Tool-Job Contribution Ownership

`@semio-tech/puzzle-plugin:canonical-architecture` owns reserved-route assertions. `@semio-tech/cad-cad-rs:canonical-architecture` owns presence-retirement assertions through the artifact package router's existing `twins` interface. The framework coverage suite no longer imports these implementation suites. Their assertions remain in the generic root canonical-architecture aggregation; no case was removed. CAD's newly reachable contribution revealed that its presence schema file name was obsolete and its schema module had been compiled without selecting the retirement export; both defects were repaired.

## Dev Session-Broker Ownership

OS directory local-session/broker now owns the broker client and request/record/session schemas and parsers. Hub credential issuance imports these real contracts and retains server-side issuance and administrator capability behavior. Vite imports the directory client; the hub has no forwarding exports. The OS-owned proof compares ten cases with Ajv and exercises one real loopback exchange, stale run identity refusal and foreign profile refusal.

## Dev Bootstrap Owner Command

The lifecycle composition moved canonically to `🌎️hub/🚀️local-bootstrap/👷️dev-owner/🟦️.ts`. `os-hub:local-hub-owner` owns binary staging, development profiles, issuer startup, selected catalog packages, progress and shutdown. Generic OS dev retains lease/join logic and only dispatches a validated process declaration. The application-owned JSON contribution provides `{owner:{program,args},defaultProfileId}` through the neutral provider schema. There is no hardcoded hub script path or dynamic implementation import in framework execution.

The S descriptor is `✏️s/🧑‍💻dev/🧬️schema/🔣️.json`; its owned browser/config wrappers and generic discovery are coordinated with the IO agent. The existing local-hub and collaboration launch rows retain their UI names/order and now call owner targets. Acceptance orchestration references were migrated at the same time.

The concrete collaboration acceptance suite moved to `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`, commanded by `os-hub:dev-collaboration`. Generic dev verification removes its old command and implementation import. The staging suite's unused `Owner27` import/spread was removed; it had no collaboration test usage. Existing HTTP/socket probe helpers moved as real implementation to OS directory testkit, with direct canonical imports in all hub and two-human consumers and no forwarding exports.

`@semio-tech/framework-os-dev:canonical-architecture` executes seven neutral provider vectors against Ajv and a real declared owner child process, then IO's three dev-contribution Vitest laws. The hub contribution includes focused reconcile/broker laws and the native `plugin_module` filter after the parent's shared schema extraction.

## Verified Checks and Discovered Baseline Defects

`bun nx run @semio-tech/framework-job-rs:canonical-architecture --skip-nx-cache` passed: 17 TS/Ajv cases and one Rust law over the same 17 cases, console evidence emitted.

`bun nx run @semio-tech/framework-os:test quick --testNamePattern='DirectoryAccessChanged|TransientApplyRefusal' --skip-nx-cache` passed: five assertions/tests selected, 542 skipped.

`bun nx run workspace:schema-generate --skip-nx-cache` completed and registers framework.job.reconcile without the removed hub request export; another refresh is needed after subsequent source moves and parent schema edits.

The full pre-existing `os-hub:gis-inference-ledger-oracle` runs now reach `neutral input/identity hash mismatch` in its untouched GIS ledger fixture, before the reconcile proof. A dedicated hub `canonical-architecture` target now runs the focused reconcile and broker proofs directly, keeping that unrelated broad-suite failure visible. The full ledger oracle has not passed.

## Fixture Hash Verification

GIS ledger fixture input hash matches its declared inputHash: `0016e1661f5f00c25c7314f7b2dfde688ae91a923ef4b6ff84575d6878bb4a90`. Its identity hash computes to `51970002e931c28361d686da8a7e66288a749911272e58fa6b6dd5a688613edb`, while the declared identityDigest is `6efba36b4a5a8b02afed2b30ce5812c71cbbd50ea41db74e11021a67ccbf4527`. The same identity hash computed from read-only `git show HEAD` is `51970002e931c28361d686da8a7e66288a749911272e58fa6b6dd5a688613edb`; the source fixture identity is byte-for-byte semantically unchanged by this task. This establishes the hash mismatch predates this envelope extraction. Derived job IDs/proposal/command hashes must be repaired as a coherent owning fixture set rather than changing one digest silently.

## Latest Focused Results

- `os-hub:canonical-architecture` passed the TypeScript/Ajv specialization and the ten-case broker proof with live loopback exchange before the native plugin filter was added. That additional native filter is verified by the coordinator and final aggregate; it is not included in this earlier pass claim.
- `@semio-tech/framework-os-dev:canonical-architecture` passed the seven-case provider contract, emitted `owner-process=observed`, and passed all three contribution Vitest laws.
- Puzzle reserved-route contribution passed. CAD's newly reachable oracle had stale schema/source test locations; the proper `$defs/PresenceRetirementV1` schema ref and separate Rust law-source reads preserve every hostile case. A root structural predicate previously depended on three moved unit-test names; the coordinator removed only those location-dependent strings, preserving all Arc/read checks and adding native exact-law coverage. CAD rerun is queued.
- The generic owner process closes its parent log handle after spawning; no temporary diagnostic logging was added to runtime code.

## Command Ownership

| Project | Canonical target command |
| --- | --- |
| `@semio-tech/framework-job-rs` | own script `canonical-architecture` |
| `@semio-tech/puzzle-plugin` | own script `canonical-architecture` |
| `@semio-tech/cad-cad-rs` | own script `canonical-architecture` using declared artifact twins |
| `@semio-tech/framework-os-dev` | own script `canonical-architecture`, provider + dev contribution |
| `os-hub` | own script `canonical-architecture`, reconcile + broker + native plugin filter |
| `os-hub` | own script `local-hub-owner` (continuous lifecycle) |
| `os-hub` | own script `dev-collaboration` (existing complete acceptance suite) |

Old generic `local-hub` / `collab-e2e` targets were removed. All permanent command code extends existing `📜️script.ts` APIs. Root canonical orchestration stays generic and does not import implementation tests.

## Exact Implementation Ownership Manifest

This lane touched the following files, including shared files subsequently edited by other lanes. Schema catalog and generated launch contents are final regeneration responsibilities of the coordinator.

- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🧑‍💻dev/🧬️schema/🔣️.json`
- `🌎️hub/🏗️bootstrap/🦀️.rs`
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs`
- `🌎️hub/💡️inference/🧬️schema/🔣️.json`
- `🌎️hub/💡️inference/🧬️schema/🟦️.ts`
- `🌎️hub/💡️inference/🧬️schema/🦀️.rs`
- `🌎️hub/💡️inference/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/💡️inference/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🚀️local-bootstrap/👷️dev-owner/🟦️.ts`
- `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts`
- `🌎️hub/🤝️integration-harness/🟦️.ts`
- `🌎️hub/🧪️tests/🌅️boot-watch/🟦️.ts`
- `🌎️hub/🧪️tests/💾️backup-restore/🟦️.ts`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🌎️hub/🧪️tests/🧠️residency/🟦️.ts`
- `🌎️hub/🧫️fixtures/🚧️hostile-input-v1/🔣️.json`
- `🌎️hub/🧫️fixtures/🧭️inference-job-reconcile-v1/🔣️.json`
- `🧰️framework/🔨️modules/📡️replication/🚧️apply-refusal/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/📡️replication/🚧️apply-refusal/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧪️tests/🔬️contract/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🟦️.ts`
- `🧰️framework/🔨️modules/🧵️job/🔎️reconcile/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🔎️reconcile/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧪️tests/🔬️contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️testkit/📡️client-probe/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔑️access-changed-v1/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧪️tests/🔬️contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🔌️staging-root.json`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🔑️directory-access-changed/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json`

- `🧰️framework/🛍️products/💻️os/🧪️tests/⏳️transient-apply-refusal/🟦️.ts`

Removed canonical predecessors:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (suite moved to hub)
- `🌎️hub/🧫️fixtures/🔑️directory-access-changed-v1/🔣️.json` (fixture moved to directory owner)

## Final Generic Dev Regression Results

The full local-hub and staging-root files passed83 tests via the owning Bun/Nx test target. The IO lane independently reran config/browser/staging and passed107 tests after the neutral env leaf and contribution-schema separation. The original dev root schema was restored, with the new contribution contract under its separate owning leaf; no old lease/catalog/spawn schema was discarded.

## Handoff Verification Status

The final complete `os-hub:canonical-architecture` target passed after the canonical helper-def rename, including the native plugin-module contribution. The full local-hub/staging test invocation passed83 tests. Combined dev canonical passed seven provider vectors with an actual owner child process and three contribution laws. The IO agent's final config/browser/staging run passed107 tests. Generated logs remain in the ticket for coordinator cleanup.

CAD's owning contribution intentionally still reports a structural-law failure in the current command dispatch source. The earlier obsolete schema paths, missing export `$ref`s, moved law-source reads, mutable foreign-factory hostile target and whitespace/spelling guards have been repaired without removing hostile cases. The current failure is substantive: production `start_typed_command_operation` delegates root capture to `capture_typed_command_roots`, and executes `A::ephemeral(...).await` before mounting the worker; the existing `toolJobRetainedDispatchSetup` requires direct capture and `toolJobMountedDispatchOneTurnExact` forbids that hook in setup. Modelling the separate captured-root/ephemeral authority or repairing the dispatcher belongs to the coordinator's next focused audit. This lane did not loosen the one-turn law to produce a green target. The complete canonical aggregate must retain this visible failure until resolved.

No git-modifying command or worktree was used. No new runtime dependency or compatibility facade was introduced. New code and fixtures preserve the shared neutral transport owner and injected application composition.


## Final Audit Followup: Profile Grammar, Process Failure, and Worker Publication

The provider now refers to `LocalSessionProfileIdV1` in the canonical broker JSON Schema and calls the broker's exported `isLocalSessionProfileIdV1` predicate. The one grammar bounds identifiers to64 characters, starts and ends in an ASCII lowercase letter or digit, and allows interior dots/hyphens. Fifteen neutral vectors include both trailing punctuation refusals, one-character and maximum identifiers. Provider/contribution Ajv oracles register the actual broker schema; no duplicated profile grammar remains in dev.

Owner launch waits for the actual child's `spawn` or `error` event. A missing executable resolves to a controlled `null` refusal after closing the log descriptor; it cannot raise an unhandled child error. Launch admission has a five-second bound. Readiness accepts an AbortSignal, newly spawned owners remain recorded as actual child handles, and failed/cancelled startup retires only that handle through the repository's cross-platform owned-process-tree termination. Cleanup waits at most two seconds. An already joined owner survives serve cancellation. Serve's SIGINT/SIGTERM boot handlers now forward cancellation into this path. The actual-process oracle observes the declared owner, a genuinely absent executable, and cancellation of a live newly started owner; the neutral outcomes must agree with Ajv's independent constant oracle. Pre-aborted launch also refuses before spawn.

The capture helper is a legitimate shared shell/agent lane implementation. The root source guard now resolves its exact full observed body, including immutable root getters, exact window authority capture and generation reads. It accepts no arbitrary delegated capture: additions, decode work, mutable peer projection and foreign helper substitution fail. The existing10 hostile authority cases remain;3 further cases attack pre-mount hook, extra decoding and foreign capture. CAD's owning contribution passed85 checks, including neutral mount/completion/undo state checked against Ajv.

The dispatcher defect was real: it awaited the app hook and called whole presence/transient apply before worker admission. Those operations are removed from both shell mounting and preview setup. Existing production owners already produce ephemeral completions inside their own retained jobs (WFC explicitly does so); their behavior remains owned there. The test app now invokes its deliberate hook from its measured worker stage, transfers the emission through its completion and declares Presence/Transient publication lanes backed by actual bounded preparation factories. Shared owned-job context retains local presence through one immutable SnapshotRead lease and peers through one Arc; only a read interface is exposed. Mounted generation validation remains unchanged. The native publication law now observes zero presence/transient/document publication immediately after actual mounting, then both ephemeral generations1 and one document edit after settling, then preserves both ephemeral generations across undo. Its peer-capture law invokes the actual shared capture helper rather than a parallel direct Arc clone.

Native canonical contributors for job and hub now use the existing exact-law runner. Job declares the one neutral17-vector Rust law; hub declares all seven actual manifest/index/file/catalog laws. Removing/renaming a law fails discovery rather than becoming a green zero-test filter. The parent contributes the four plugin laws through the plugin's own canonical target, preserving logical ownership.

Followup verification (ticket logs retained for coordinator): provider15 vectors plus3 real-process vectors and3 contribution tests passed; existing local-hub27 tests passed; CAD85 source/oracle checks passed. Final reruns and native command laws are recorded below after completion.

Additional exact ownership for this followup:

- `📜️script.ts` (typed capture, retained setup, mounted one-turn and capture authority predicates only)
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts` (exact one-law canonical contribution)
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (exact seven-law canonical contribution)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎫️local-session/🗄️broker/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🧪️tests/🔬️contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚀️local-hub/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧩️contribution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (immutable captured presence read interface and removal of pre-mount producers; narrow cfg(test) capture/mount observations)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/⏳️completion/🦀️.rs` (worker-owned fixture hook and publication lane declaration)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (actual mount/publication/undo observations, bounded fixture authorities and shared capture law)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧫️fixtures/🧬️request-context.json` (language-neutral publication state vectors)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts`


Final followup executions: `@semio-tech/framework-os-dev:canonical-architecture` passed again after serve cancellation wiring and post-readiness abort checks (provider15+3actual-process, contribution3); `@semio-tech/cad-cad-rs:canonical-architecture` passed85 checks; `@semio-tech/framework-job-rs:canonical-architecture` passed via exact native discovery/execution (one law, shared17 vectors). Existing dev lifecycle27 passed. Initial native worker laws honestly failed first on a private view constructor (fixed through immutable `presence_view()`), then missing bounded Presence/Transient publication authorities. The fixture now supplies both preparation factories and displaced-root retirement factories; the same unchanged admission gate remains active. Its definitive four-law execution is under `🗑️generated/plugin-command-owner-final`.


The definitive `@semio-tech/framework-plugin:canonical-architecture` run passed all four exact named native laws, including the actual mount/no-prelude/completion/undo neutral vector law, empty emission law, actual shared capture/retirement law and hostile peer saturation/cancellation/stale authority law. The native runner discovers every named test and asserts exactly one executed success; no zero-law filter result is accepted. Detailed receipts remain under `🗑️generated/plugin-command-owner-final` for coordinator inspection and cleanup.
