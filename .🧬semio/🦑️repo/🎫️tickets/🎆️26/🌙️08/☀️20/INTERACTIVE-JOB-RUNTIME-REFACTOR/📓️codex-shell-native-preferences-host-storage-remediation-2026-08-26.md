# Shell Native Preferences Host-Storage Remediation

## Outcome

The native Shell preference field path no longer performs synchronous filesystem operations from renderer-owned source. One already-retained frame-maintenance owner is admitted to the process-wide worker pool on Lane::Io; its single-field turn now delegates the platform read/write to the OS host-storage service.

The full deny-mode static interactivity gate is GREEN. This packet is not runtime-accepted until an independent audit and the serialized Rust/runtime matrix complete.

## Implementation

- Shell/🧊️component.rs
  - retains key/path/value ownership and the existing 16 KiB field boundary;
  - delegates native reads to storage_worker_read_fixed_file_page;
  - delegates native writes to storage_worker_write_fixed_file_page;
  - contains no std::fs or std::io call in the production preference boundary.
- 🛎️services/🦀️component.rs
  - owns the fixed 16 KiB file-page authority;
  - rejects an oversized configured limit before opening a file;
  - rejects a metadata length over the admitted bytes before allocating an output;
  - reads through one fixed stack page and detects a post-metadata trailing byte;
  - preflights write bytes before directory creation, file opening, truncation, or publication.
- 📜️script.ts
  - verifies the renderer-to-service delegation, service-side cap/growth laws, and absence of UI-owned filesystem calls;
  - includes hostile mutations for dynamic pages, UI bypasses, whole-JSON materialization, and missing max-plus-one/oracle law.
- .vscode/🧩️launch.seed.jsonc
  - repairs the canonical native zero-warning/clean seam;
  - registers the six interactivity/action/dependency gates so plugin-registry generation preserves them.
- .vscode/launch.json
  - is generated from that seed; a concurrent pre-repair generator can transiently rewrite the old bytes, so the final serialized verification must regenerate/check it after all active generators settle.
- plugin registry 🖥️launch.ts
  - fails closed if either the canonical seed or fully substituted generated output is invalid JSONC, preventing malformed bytes from being published again.

## Verification

| Check | Result |
| --- | --- |
| rustfmt on Shell and OS services | Exit 0 |
| rustfmt check on Shell and OS services | Exit 0 |
| bun ./📜️script.ts verify interactivity apps | GREEN: 32 descriptors, 4 extensions, 101 apps, 4,754 actions, 1,955 migrated actions, 2,799 missing actions, 185 launches, 0 failures, 14 self-tests |
| bun ./📜️script.ts verify interactivity | GREEN: deny mode clean; only the structurally invisible recorded test-only block_on allowlist entry remains |

Cargo, Nx, Wasm, browser, and runtime checks were not run in this packet.
