# Typing Runtime Source and Failure Evidence

Observed OS outcome: `typing-net-os-commit-reboot11220615.outcome.json`, status 0, signal null, closed 2026-10-30T02:14:31.293Z. Actual production Cargo unit run enumerated three passing tests (policy/move-only rows; cancellation/paid close; issued reader surviving 65 funded prefix insertions over 8197 leaves and committing the same net backing). No ignored or filtered tests; Nx 0/4 cache hits.

Public Cargo Running coordinate (observed identity, no private binary read):

`.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p18f946/b/prototype/build/semio-framework-plugin-instance-typed-ops/1f6e93042b178a0b/out/semio_framework_plugin_instance_typed_ops-1f6e93042b178a0b`

Source SHA-256 snapshot at 2026-10-30T02:34:28.454Z. These are current after-execution source identities, not a runner-attested input manifest. The selected typing source has not been edited since this acceptance run.

| Source | SHA-256 |
|---|---|
| `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/🗃️resident/🦀️.rs` | `0419040b1d013a5343e17827816087c222080fd16e61f7f72e6dc429e24457ec` |
| `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs` | `22a0733efa6c72f13c13d3a66984a71f2fadf54767c54965fe59fabacb107d34` |
| `🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/🦀️.rs` | `07da3468e4bbc3eb12187cd2c39f4cb4d25379ff35157ba12fd5a6aa970d316d` |
| `🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/📜️script.ts` | `a8528d52758e96616e2f6c3b291ea0ee6454585ccc4bf5c0fc500aac21d3a365` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🧪️tests/🦀️.rs` | `a29a7abd633bef9f0b988a0896ca94effbd4f6e072e5deddc8a05027d270c627` |

## Parent Build Failure

`typing-net-parent-runtime-four-checks-accepted.outcome.json` closed status 1, signal null, at 2026-10-30T02:01:57.505Z. The outer actual command was `repo run @semio-tech/framework-tool-machine-rs:test --output=static -- --status-level all --final-status-level all`. TS prerequisites passed 37 tests / 25,663 assertions. No parent Rust assertion inventory ran.

At public log lines 4134–4136, immediately after ordinary value-control-contract warnings: `error: Resource temporarily unavailable (os error 11)`, `warning: build failed, waiting for other jobs to finish...`, and `error: cargo exited with status 101`. The failure reached the Cargo runner build phase and its bounded diagnostic replay is now effective. The process runner close-path error proves the Cargo child started and closed 101; it was not a Node child spawn error. It does not identify Cargo internal spawn, rustc worker, linker, descriptor, or memory cause. There is no compiler process argv, syscall, panic/backtrace, or resource snapshot at failure time in this log. The preceding warning is not evidence its crate caused the failure.

Source projection: framework process testing cargo/TS build phase uses `cargo nextest list --cargo-profile prototype --list-type binaries-only --message-format json --config-file <actual> --profile <level>`, actual metadata or manifest, both production and companion packages, locked/offline. Its alternative without nextest is `cargo test --no-run --profile prototype --manifest-path <actual>` with the same packages/locked/offline. Historical nextest selection and complete admitted argv were not persisted in the public log, so neither exact leaf command is asserted. The separate assertions phase was not reached. `runBudgetedTestCommand` spawns the admitted command and throws the observed status on close; stack location is cargo/TS:218 and testing execution/TS:61. No limits, grants, workers, or stack budgets were changed and no blind rerun was submitted. Current resource readings would be after-the-fact evidence only.

Additional actual production Source identities (same qualified after-execution snapshot):

| Source | SHA-256 |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🎟️original/🦀️.rs` | `c5d6ccc05813a3e99d3c49fceea905c380a8c05ae3d84fa6d2d1ce0a619ae10f` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🎮️operations/📖️source/🦀️.rs` | `602d7435e0d627facaca7c03005b45293bdf21ecb8b407ec68155e625025a0d4` |

Root later audited actual cgroup mount membership and ancestor PID counters at 03:59:21 UTC: [Cgroup PID Audit](cgroup-pids-audit.md). These are after-the-fact readings, and existing ancestor max-event counters were zero. The earlier Rust child membership and failing syscall were not observed; this does not establish the cause of EAGAIN. No limits were changed.
