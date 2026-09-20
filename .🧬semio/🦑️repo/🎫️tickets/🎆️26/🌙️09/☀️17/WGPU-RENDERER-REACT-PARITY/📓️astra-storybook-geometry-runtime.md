# Physical React Geometry Oracle

The fresh UI Storybook bundle was served on port 6327 and measured in Chromium at 1600×1000, DPR 1. Both physical Playwright laws passed in 3.0 seconds: nested Mode panel/separator layout and the Panel chrome drag handle extent. Receipt: `🗑️generated/astra-runtime/storybook-geometry-tests-6.log`.

The failed preceding run exposed two assumptions. React's window content plane uses the gap token for horizontal padding; vertical padding is the silhouette clearance and is canceled by equal negative vertical margins. The schema and fixture now state those distinct contracts. React's leading tab icon is 16 pixels, but its DragHandle explicitly measures 12 pixels. WGPU incorrectly used 16 pixels for both. The retained width calculation, retained trailing icon painter, and open-anchor painter now use the canonical 12-pixel tree-row icon size for the grip. The native navbar hit-width law consumes the same independently measured fixture; its next full renderer run is pending.

The Storybook build produced usable current bundles in 6m5s, but its post-build discovery guard failed because it joined the repository root onto the supplied absolute output directory twice. This is a build-command failure, not a green build receipt. The physical tests above ran against the produced bundle. The default browser runner separately failed before tests on a Node CJS `import.meta` import; a ticket-local Playwright config isolated these two existing stories without changing the production runner. The input config is retained at `🔬️storybook-geometry/📜️script.ts`.

These receipts establish the actual React oracle. A new paired WGPU browser checkpoint is still required to establish runtime geometry parity.
