# Terra Packet UI-Chrome-01: Dissolve Chrome Umbrella

## Preconditions

- Read root/UI AGENTS and `📓️luna-ui-chrome-umbrella-split-audit.md`.
- Apply patches only; no modifying Git commands.
- Require current SHA-256 values:
  - Chrome `ef751d89dc56a32d1d9012213c3806726099eaa7bbdcadc4e5e337c51d64fba0`
  - Skeletons component `04068e313f020cd6aa7fa20e43b6d765e758358dec1ae59d3d823730b44ef9c4`
  - Skeletons story `dc9e0154f4a0275f219a464665adac700bd22003c82df4f885131bb46f42e1fe`
  - Scrollable `171c330d50646a3b02b2f1e8fb7c58e8e6b10ac1392e7905df438dc738dfc1a5`
  - Window `54fa7f0686a5a4f732048d21e8e101f6e1631d1d99742bc1dc50a16a7dcd2ef4`
  - accepted-dirty Toggle `15a3aa7bb4f7bdddd74fe148b0df40327c8f5441df9d83a5509e02e5e6da50cb`
  - accepted-dirty PanelTabBar `b522d8a6cdc026d020562fc8c5a2a0e2ec8c1a204036457c8f1c18922d722cb3`
  - DragHandle `0941b7e210cfe27262f0fb78671ff6164d1c3d4d386660e0b2e84a1fcc724d47`
- Shared React barrel is coordinator-owned and expected at `e82f73a9fd61e5d140d69f7df7498fa1afcd2217fde523fb6f64c9e130844e81`; never edit it.

## Terra Writable Closure

1. Delete old `🎛️Chrome/🟦️component.tsx` after distribution.
2. Create `🪟️WindowSilhouette/🟦️component.tsx` containing only the former geometry region and its private helpers.
3. Create `💡️ChromeControlHint/🟦️component.tsx` containing only the accessible wrapper.
4. Create `🚧️WindowContentDeadLine/🟦️component.tsx` containing only the scroll/measurement responsibility.
5. Update direct imports in Scrollable, Window, Toggle, PanelTabBar, and DragHandle to their specific paths, preserving accepted unrelated content.
6. Inline `LoadingRow` privately in Skeletons, including the minimal imports it needs. Remove the standalone LoadingRow showcase/import from the Skeletons story while preserving all other stories.
7. Unique acceptance `📓️terra-ui-chrome-umbrella-dissolution-acceptance.md`.

## Contract

- Preserve public symbol names and behavior for geometry, ChromeControlHint, and window-content dead-line behavior; only ownership paths change.
- Do not preserve or export standalone `LoadingRow`: tests/stories cannot keep a one-consumer implementation public.
- Do not introduce new external dependencies or external-library types.
- Preserve regions and concise docstrings.
- Do not edit the shared React barrel, Storybook registrars, manifests/locks, generated files, plugins, or protected renderer.

Stop after source split and send exact new hashes, old-path scan, dependency direction, and scoped diff. Coordinator will serialize barrel and Storybook registrar changes, then signal final stale scans and UI React gates.
