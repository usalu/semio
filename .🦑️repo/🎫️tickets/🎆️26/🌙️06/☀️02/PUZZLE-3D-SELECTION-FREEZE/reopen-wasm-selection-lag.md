# Reopen: WASM post-selection lag (Jul 31 2026)

Same root cause as original ticket: post-commit selection refreshed composite + full document tree.

Implemented selection chrome patch (`HostEffect::PatchWorld3dChrome`), stable `instancesJson`, inspector-only `UiDirtyScope`, cached document sections + `selectedIds`, WGPU selection-only sync path, skip `sync_precompute_session` on pure picks.
