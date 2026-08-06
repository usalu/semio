# OS host unblocked (default features)

- `semio-framework-os` `cargo check --lib` GREEN with default features (no `os-host-full`).
- Gated behind `os-host-full`: `host`, `backbone`, `host_runtime`, `instance`, `workflow`, `registry` (need dissolved `store_sync`/`space` remounted into kernel).
- Media export registration inlined into `media_export_raster` so plugins keep `register_*` APIs.
- Glue path: `../../🦀️component.rs` (host component). Product-level `os/🦀️component.rs` shim inlined separately.
- Kernel: playbook left unwired (needs ui_wgpu). `semio-framework-os-kernel` GREEN.

See integration-requests for full remount of sync/space/workflow into kernel.
