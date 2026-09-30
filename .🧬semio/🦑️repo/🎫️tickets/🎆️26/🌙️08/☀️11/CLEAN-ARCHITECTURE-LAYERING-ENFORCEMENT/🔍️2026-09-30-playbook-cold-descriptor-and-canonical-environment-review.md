# Playbook Cold Descriptor and Canonical Environment Review

Read-only source/log review; no builds or tests executed by this auditor.

## Full Playbook failure attribution

`🗑️generated/playbook-full-admitted-final.log` records 24 tests, 22 passed and two SIGTRAP failures: `component::descriptor_is_fresh` and `component::tests::procedural_actor_descriptor_matches_the_json_oracle`. Both report `extension bundle requires terminal-empty retirement before drop` at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs:172`, followed by thread-local destruction abort. Compilation succeeded. This evidence identifies installed bundle cleanup failure, not a descriptor or JSON equality mismatch.

Current source contains the narrow caller corrections: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs:11–12` disposes and asserts terminal emptiness after descriptor assembly; `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs:52–53` does the same after obtaining wire bytes. The first is generic generated-test ownership, the second Playbook ownership. Neither correction relaxes the retirement Drop guard. The parent owns the replacement runtime run; this review cannot establish its result.

## Canonical environment source and routing

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts:11–13` resolves the supplied target root and returns a spread copy of caller input with both `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` set to that root. It does not capture ambient variables itself. Root `📜️script.ts:8960–8963` supplies `process.env`, uses this helper and explicitly passes `--skip-nx-cache` to owner discovery.

The closed schema and four vectors at library `🧬️schema/🏛️canonical-execution/🔣️.json` and `🧫️fixtures/🏛️canonical-execution/🔣️.json` cover default, relative, absolute and explicit-empty target inputs, stale build root replacement and ordinary caller variable preservation. Library `🧪️tests/🏛️canonical-execution/🟦️.ts:16–84` consumes them through AJV, verifies copy/ambient isolation, and uses independent TOML parsing plus real offline Cargo metadata. The test creates artifacts only beneath required `SEMIO_TEST_ARTIFACT_DIR`.

At this snapshot `.vscode/launch.json` contains neither `canonical-execution` nor `🏛️canonical-execution`. The executor was notified to identify the intended inferred registration and regenerate launcher coverage. This is a remaining launch-discovery verification item, not a runtime claim or evidence that the portable law passed.

## Settled generated launcher follow-up

After parent regeneration, `.vscode/launch.json:17341–17344` now registers `@semio-tech/repo-lib:test-canonical-execution`. The generated canonical selector at line 20484 retains 24 unique owners. `nx.json:9–11` disables inherited canonical target caching; root invocation retains explicit `--skip-nx-cache`. The helper's public environment interface at cargo caching source lines 6–8 is first-party and contains only built-in string/undefined values; its function signature exposes no external library types. These checks close the earlier launch verification item. Parent reports full Playbook 24/24 and focused environment green; those runs were not executed independently by this auditor.
