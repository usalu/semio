# Open Procedural 3d Journey Items (2026-11-22)

Last live runtime evidence: 2026-09-15. Neither 6018 (React) nor 6118 (WGPU) serves; the Rust image `.cache/b/r/m` reports 0 bytes available (APFS `spaceman_alloc ... host free 0`).

## P0 blockers
1. Rust image ENOSPC blocks every activation → recovery in [rust-volume-zero-avail-recovery.md](./rust-volume-zero-avail-recovery.md).
2. Undo chord (mod+z) moves the document but not the 3d preview (`📓️react-final-sweep-2026-09-15.md` §7.1).
3. React `interact` battery red: Rectangle Wire Preview select/inspector, Hexagonal Mushroom Column inspector (same file §9.2 row 3).
4. Add Generation chord (mod+shift+g) unverified (`📓️react-end-to-end-verification-2026-09-13.md` §5.2).
5. WGPU battery 10/16 (2026-09-14): wheel zoom in edit, viewer offers translateSelection, frame-loop quarantine, chrome row click misses, ARIA/German/status pill, Fit camera (`📓️wgpu-end-to-end-verification-2026-09-14.md`).

## P1
Wgpu example picker/mode/role controls, wgpu import/export, viewer cancel-eval, catalogue drag payload, txt codec stub, German action labels, guest teardown panic in ordered replication value.

## Probes
- Launch: `repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype` (6018), `…:dev-generation3d-wgpu-prototype` (6118).
- React battery: `bun 🐍️react-battery.mjs` → `🗑️generated/react-verify/scoreboard.json`.
- WGPU battery: `bun 🐍️wgpu-battery.mjs` → `🗑️generated/wgpu-verify/scoreboard.json`.
- Boot gate: `bun 🔍️browser-probe.ts --port=6018`.
