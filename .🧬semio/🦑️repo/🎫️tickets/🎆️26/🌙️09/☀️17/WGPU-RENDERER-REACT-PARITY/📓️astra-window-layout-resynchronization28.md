# Window Layout Resynchronization

Latest build checkpoint (2026-09-27): WASM attempt 6 failed upstream in XML mutation `MutationLeaf` implementations after 3 minutes 8 seconds; it did not reach renderer validation. Native window attempt 3 failed upstream in SVG duplicate `SvgDiff` and unresolved `Entry`/`SvgMutation` after 27 minutes 42 seconds, before any selected assertion ran. Logs: `🗑️generated/astra-runtime/wasm27/run-6.log` and `maximize27/native-green-3.log`. These are build failures, not passing native receipts.

The React Mode component compares the serialized incoming layout and ordered window instance IDs. It clears ephemeral maximization only when either changes. WGPU's keyed dock diff currently preserves and reanchors maximization even after a changed split ratio; conversely, the default-layout branch reconstructs the DockState on every sync and loses maximization even when input is identical.

The actual ShellHost caller always supplies `effectiveModeLayout = shellLayout ?? resolvedDefaultLayout` to Mode. Consequently, changing raw override provenance alone does not necessarily change React's layout input: two effective layouts with the same serialized representation remain identical. The host roster contains base kind IDs followed by extra instance IDs; labels and icons are presentation, not roster identity.

A new neutral fixture covers identical override, identical default, changed ratio, reordered window roster, and presentation-only label update. The actual React Mode oracle rerenders those inputs after maximizing the right stack. The native law traverses Shell sync and the real focus command, and also asserts maximize does not write a persisted layout override. Test execution is pending at this checkpoint.

The intended fix records incoming effective layout and roster identity at the Shell boundary, leaves keyed DockState reanchoring available internally, and removes layout persistence from the two maximize-only handlers. Ordinary drag, resize, close, and new-window persistence remains part of those actions.

## Expanded Identity Contract

The actual `resolveFrameworkLayoutSeed` adapter plus mounted React Mode now pass nine checks: schema validation and eight neutral cases, including title change, template-only change, and an extra instance. Every case also re-maximizes if needed and proves an identical subsequent refresh preserves that maximize. Receipt: `🗑️generated/astra-runtime/maximize27/react-4.log` (9 passed, 18.63 seconds).

Application and maximize identities are now separate. Raw framework changes still update the dock and template map. The Mode projection excludes template and authored active fields, resolves titles, and combines with ordered window IDs for maximize invalidation. A locale/terminology change retitles an existing layout, including extra instances, and retains that interpretation on unchanged subsequent refreshes. Native checks remain pending. The first native attempt failed in unrelated plugin compilation before test execution; no assertion-level red or native pass is claimed.

The final React oracle now passes all 10 checks, adding actual locale retitling of a known extra window followed by an identical refresh. Receipt: `maximize27/react-5.log` (10 passed, 53.43 seconds). The native counterpart checks the same ephemeral maximize transition and the resulting dock title. Independent read-only source audit found the two-signature flow coherent. It separately identified unknown-kind ingress pruning as a remaining behavior gap; changing only roster filtering would not match React's Mode reconciliation.

## Declared Window Ingress

Four neutral cases now cover unknown extra removal, an unknown kind using a declared base ID, an all-unknown empty dock, and hoisting a surviving stack. The actual React seed plus mounted Mode initially passed 13/14 checks; the remaining expectation incorrectly assumed an empty stack has rendered chrome, while React renders no stack. The fixture now records no rendered stack in that case and is rerunning. WGPU now reconciles the dock against declared base IDs plus known extra instances, clears unknown-kind projection templates, hoists empty peers through the existing collapse function, and filters its published instance roster. Raw incoming layout identity remains available for maximize resynchronization. Native test execution is pending.

The expanded mounted React suite now passes all 14 tests, including all four ingress cases. Receipt: `🗑️generated/astra-runtime/maximize27/react-ingress-2.log`, 20.20 seconds. The native ingress law also checks surviving stack paths, published roster, projection map, and an identical repeat; it remains unexecuted while the existing compile queue runs.

WASM attempt 5 reached the renderer and failed after 20m36s on a missing `WindowLayout` import introduced by this packet, with consequent type-inference errors. The import is now corrected. This attempt produced no fresh runtime build; the cross-crate shape coordination gate was released so the Commands implementation can land before the next compile.

A fifth ingress case first mounts a valid extra with a projection, then replaces its kind with an unknown one under the same ID. The actual React seed and mounted Mode now pass all 15 checks (`maximize27/react-ingress-3.log`, 59.03 seconds). The native counterpart asserts the extra and projection retire on the same transition.
