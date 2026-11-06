# Rust Extern Namespace Audit

Static narrow inspection; no Cargo build. Exact source files, owners, candidate dependency keys, packages and declared library names are in rust-extern-namespace-audit.json. Conditional compilation, macro expansion and target-specific dependencies were not evaluated. Shared mounted files are recorded per owner. Missing candidates require review.

The initial broader scan was terminated after verifying its process identity because it traversed the framework tree too slowly; it produced no output.

5 selected manifests; 8 file/owner/root findings.

| Old root | Candidate | Evidence | Findings |
| --- | --- | --- | ---: |
| `semio_framework_os_kernel` | `semio_os_kernel` | matching_direct_dependency | 3 |
| `semio_framework_plugin_guest_contract` | `semio_plugin_guest_contract` | matching_direct_dependency | 2 |
| `semio_framework_plugin_interpreter` | `semio_plugin_interpreter` | matching_direct_dependency | 2 |
| `semio_framework_ui_contract` | `semio_ui_contract` | matching_direct_dependency | 1 |
