# Navigator Camera Parity

Current React `Paint2dHost` accepts Navigator wheel input and publishes `setCamera` using the shared Composite camera and Composite viewport. WGPU still returned early for every Navigator wheel. The older audit's inert-wheel description is superseded by this current-source comparison.

The new language-neutral four-case contract covers an authored Composite extent, the existing missing-extent fallback, and both zoom limits. The actual mounted React host publishes one matching action for each case. A separate Three `Matrix3` inversion oracle verifies that the resulting camera preserves the cursor's world-space anchor.

The focused React run is **GREEN, 3/3**, Vitest 22.54 s, Nx 24.6 s, exit 0 (`🗑️generated/astra-runtime/gate34/navigator-react2.log`). The preceding test attempt failed because this repository's testing facade does not expose `fireEvent.wheel`; it did not expose a product failure. The corrected adapter dispatches the DOM WheelEvent inside React `act`.

WGPU now uses the existing canonical infinite-canvas wheel math for the shared camera and publishes the existing bounded `setCamera` action. It preserves the Navigator host's own fitted camera. The native fixture law verifies the exact payload and unchanged local fit camera; the earlier inert-pointer law now covers only editing input. Those native assertions are authored but **unrun** and are outside the active batch 35 filter. Physical cross-window acceptance remains outstanding.
