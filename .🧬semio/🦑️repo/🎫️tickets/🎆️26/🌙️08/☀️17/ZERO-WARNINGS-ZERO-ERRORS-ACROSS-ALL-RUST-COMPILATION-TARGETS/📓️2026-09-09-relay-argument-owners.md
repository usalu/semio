# Relay Argument Ownership

Grouped each relay attempt's retained runtime, instance, cancellation and completion handles into one owned value shared by the execution and recovery paths. Named the recovery callback type and grouped the initial job kind/payload. The HTTP helper now takes the existing first-party request record. Borrowed lifecycle-probe controls without redundant Arc handoffs and updated all discovered constructor fixtures. Three Rust sources parsed before guarded writes; strict compilation and lifecycle regressions remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️guest-cold-relay/🦀️.rs
