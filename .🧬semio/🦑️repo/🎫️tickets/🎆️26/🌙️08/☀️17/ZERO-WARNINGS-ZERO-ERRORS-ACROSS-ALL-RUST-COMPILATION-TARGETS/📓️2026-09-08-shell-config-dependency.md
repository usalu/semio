# Shell Configuration Dependency Diagnostic

Native604 reported E0433 for the shell schema's `semio_framework_os_config::opening_config` re-export. The Cargo metadata captured when this run started has no shell-to-config dependency. By the time the diagnostic was inspected, the shell Cargo manifest already contained `semio-framework-os-config = { workspace = true }`, and the root workspace mapped it to the existing configuration crate.

No source or manifest edit was needed from this ticket. A fresh Cargo invocation must resolve the updated graph before this diagnostic can be considered cleared; the current invocation's earlier dependency graph cannot establish that result.
