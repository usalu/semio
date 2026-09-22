# Checkpoint 22 Intake

WASM 22 completed in 5 minutes 30 seconds, Puzzle materialization completed in 8 minutes 1 second, and canonical narrow prepare/activate targets completed for both React and WGPU. Both pages were reloaded from the resulting selected guest.

WGPU paints its two 3D panes but reports a panel render fault in the canvas; its accessibility tree exposes empty panel content. React raises `plugin-ui.intake-rejected:intake:Owned UI patch admission rejected` and logs missing requested panel surfaces. These failures prevent treating this checkpoint as a parity acceptance.

Source inspection found a concrete omission from the Tree row-extent change: `ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts:366` admits only `total` and `offset` in a Tree window. It rejects the freshly produced required `rowExtent` property. The Tree implementation owner is adding a fail-first owned-wire decoding law and checking all remaining TypeScript transport/copy paths.

Browser evidence is under `🗑️generated/astra-runtime/checkpoint21-iab/05-wgpu22-boot.*` and `06-react22-intake-fault.*`. The fresh WGPU tab uses the normal 1280×720 viewport; the React comparison viewport was aligned to it. Earlier 1600×1000 reference measurements remain separate evidence.

The development server also reports that another contributor updated Puzzle precompute source after the selected guest was produced. That freshness warning is recorded separately from the confirmed decoder omission; work continues without reverting or stopping the other contributor.

The independent window-close repair is physically verified on WGPU 22: opening Top's Window Options and its Orthographic Select, clicking outside at `(600,610)`, then clicking Top's close cap once at `(378,46)` leaves exactly one Perspective application and no Top application after presentation settles. The Select disappears, and no browser error-level log was emitted. Evidence: `08-wgpu22-rapid-close-settled.dom.txt`. This is a narrow positive receipt while the separate guest-panel intake fault remains open.
