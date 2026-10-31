# Canonical OS Typing Net Gates

Read-only actual manifests, Cargo lib mounts and target inference review, 2026-10-29. No compiler, test, Native/container, process or lease operation ran. Root72636 remains owner-reported queued; this report neither duplicates it nor claims its result.

## Exact Existing Owners

All OS paths below are under framework/🛍️products/💻️os/🔨️modules/🔌️plugin/.

| Source | Existing Project And Cargo Owner |
| --- | --- |
| 🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs and its tests | @semio-tech/plugin-instance-typed-ops-rs; Cargo semio-framework-plugin-instance-typed-ops; manifest in 🧵️instance/🧾️typed-operations/📦️packages/🦀️rust. Cargo lib ../../🦀️.rs directly mounts pub mod original_typing at the production path. |
| 🛠️tool-machine/🔁️follow/🦀️.rs | @semio-tech/plugin-tool-run-rs; Cargo semio-framework-plugin-tool-run; manifest ⏯️tool-run/📦️packages/🦀️rust. Its lib mounts ../🛠️tool-machine/🦀️.rs, which mounts original_follow. There is no separate tool-machine package at this OS path. |
| ⏯️tool-run/🎞️tick/🎮️operations/📖️source/🦀️.rs and overlay fold | @semio-tech/plugin-tool-run-operations-rs; Cargo semio-framework-plugin-tool-run-operations; manifest ⏯️tool-run/🎞️tick/🎮️operations/📦️packages/🦀️rust. The actual production reader type is OriginalToolLedgerRead, not a newly invented OriginalRetainedRead wrapper. |
| 🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs | @semio-tech/plugin-instance-render-rs; Cargo instance render owner; manifest 🧵️instance/🖼️render/📦️packages/🦀️rust. This mounts actual follow_tool_machines consumers. |

The four compact project manifests declare names/tags without explicit targets. Existing repo build inference supplies native build/check/test from Cargo.toml, not a missing-wrapper workaround: library/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs cargoTargets emits repo:script, native/test scope, no outputs, nativeTestSources/^nativeTestSources, serial test/check, and routes through existing caching/🦀️cargo/📜️script.ts native cargo <operation> --manifest <actual path>. Project assembly combines inferred targets with authored overrides. No new target or empty wrapper is required.

## Normal Commands To Select After Current Job

```sh
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo run @semio-tech/plugin-instance-typed-ops-rs:test -- --lib -E 'test(original_typing_policy_and_move_only_rows_match_the_neutral_contract) | test(original_typing_cancel_keeps_every_accepted_owner_until_paid_close)'
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo run @semio-tech/plugin-tool-run-rs:check
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo run @semio-tech/plugin-tool-run-operations-rs:check
SEMIO_LANGUAGE=en /home/deck/.local/bin/repo run @semio-tech/plugin-instance-render-rs:check
```

Use existing argument transport/normal admission; no --light, direct cargo, invented Original, target reset or source copying. The existing NativeScript combines request args and transported owner args. The expression uses the actual two Rust method names and retains normal native per-test ownership. Require the test inventory/outcome to show both matches executed; zero-selected tests is not success. This exact-filter run proves only these two methods, not the full package.

Both methods are in original_typing::tests and #[cfg(test)] mounts the exact adjacent tests. The move-only test asserts original String leaf pointer identities, net Vec pointer/capacity, original transaction id pointer and withdrawn ledger state, using actual fixture Original/decoder and heap receipt equality on every turn. Cancellation test requires all accepted owners stay retained until paid close. Require terminal0 and both tests passing on current Source, no ignored/skipped/filter-only result and no unaccounted requested/released heap bytes.

The three check commands prove actual production compilation of follow/reader/render, not runtime behavior. No dedicated follow-overlay or OriginalToolLedgerRead native tests were found in their owner subtrees during this bounded audit; do not invent a test filter that selects zero tests. Full native test routes remain available as <same-project>:test for owner-complete verification after focused fixes; report actual discovered counts and explicit skips.

## Prerequisites And Broader Consumer Gate

Actual inferred targets acquire canonical native preparation through withNativePreparation: current Cargo/toolchain/lock inputs, selected test dev-dependencies, generated dependencies and native input-check admission on unresolved source boundaries. These are derived from the actual dependency graph, not manually reconstructed lists. Preserve normal Source/Original/compiler/Store admission and existing valid artifacts. No browser setup or new general dependency sync is intrinsically required merely to select these native Rust owners.

The top-level authored project @semio-tech/framework-plugin also has an existing forwarded test target and check target in plugin/📦️packages/🦀️rust/📋️project.json. Its test script runs several JS/oracle prerequisites before admitted Cargo package tests; it is broader and does not replace the typed-ops package's own cfg(test) laws. Use its full canonical test only when the complete OS integration budget is intended; do not add a cross-package --package override to pretend a different root is its own canonical owner.

For final follow/read runtime acceptance, require a real mounted consumer exercise retaining a large typing reader across other-window prefix insertion and backing growth, followed by original commit and paid reader/factory/residue closure. Existing framework tool-machine reader/transfer laws are already in Root72636 scope and must not be duplicated while queued. The new large-window case identified in typing-net-original-review.md must actually exist and execute before claiming that stronger scenario. Compile checks plus the two commit methods do not establish that missing mounted runtime witness.
