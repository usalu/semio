# Canonical Wasm Consumer Audit — 2026-11-05

No actionable filename/ID coupling defect found. Read-only current Source trace; no Source edits, gates, process changes, server or browser. Preserve live79785/83525/86673; no runtime completion inferred.

One naming clarification: semio-renderer-wgpu is underlying library (crate semio_renderer_wgpu). Browser actual compiled cdylib is semio-renderer-wgpu-component (crate/raw basename semio_renderer_wgpu_component), declared WGPU/component/packages/rust/Cargo.toml:3 and lib crate-type cdylib. This is intentional, not a mismatch. Compiler 🌐️wasm/script.ts:38 selects componentRoot/Cargo.toml; :49 selects authentic completed artifact by exact manifest, nontest profile and .wasm staged output, not stale raw filename. :52 passes actual manifest package name to plan; plan:8 uses exact --out-name name. Consequently published files are semio-renderer-wgpu-component.js and semio-renderer-wgpu-component_bg.wasm; release optimizer and receipt records preserve these keys.

Current consumers relative to WGPU owner:

- 🌐️browser-host/🟦️.ts:41–42 uses those exact JS/Wasm component filenames.
- 🎬️renderer-boot/🟦️.ts:47 default JS matches; :59 derives _bg.wasm from selected module URL.
- 🌐️server/🟦️.ts:63 mounts /renderer-modules/wgpu from compilerRoot; :109 requires exact component pair. Config:24 selects canonical renderer publication directory.
- Runtime config nativeBrowserManifest explicitly names same componentCargo owner; nativeBrowserProject stays @semio-tech/framework-renderer-wgpu. JS project IDs intentionally unaffected by Rust package shortening.
- library/discovery/runtime/verification/🟦️.ts:103–106 reads CURRENT component manifest package name, actual workspace/lock, and passes it to transformation verifier. Verification/wasm:34 requires exact bindgen argv with that name; :35–40 checks binding JS/Wasm keys, release optimizer continuity, physical rosters and complete final output map. No stale underscore/hyphen guessing. Runtime ledger:463–465 consumes owned wasm.json receipts, not old Trunk names.

Tracked old semio-framework-os-renderer-wgpu/underscore search found only intentional historical path-envelope fixture vectors (host253 and two-overflow model), preserved by design. No production old raw/output basename reference found in this bounded audit. Existing browser6179/layout/demo journey readiness remains separate; namespace Source consistency is not activation, native compile, receipt-currentness or actual browser proof.
