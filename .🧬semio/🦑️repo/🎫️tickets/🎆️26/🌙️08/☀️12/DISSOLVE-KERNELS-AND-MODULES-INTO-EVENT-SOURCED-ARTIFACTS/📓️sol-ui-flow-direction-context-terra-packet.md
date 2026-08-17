# Terra Packet: UI Flow Direction Context Module

## Objective

Relocate the coherent, qualified shared logical-direction context from the visual element collection to the specific UI-owner `flow-direction-context` module. Preserve defaults, nested partial overrides, and all seven independent component consumers. Add no forwarding identity.

## Baseline SHA-256

- old Flow source: `163e978a3635d7b0fd2654187b7782fa9c5f5637e3ba128d7ecc426cdd4953d3`
- Popover: `02ba7a119116b9e8bc5e962ffd3240925e2172d823aedf744979ca8b7badaba6`
- PanelTabBar: `ab5d52c9d9fe201e873adbf17a36601721e112d7a6b52aad38ef40d27ced5bd9`
- ContextMenu: `e6d0b25fdb3637f21a65d46b65fa0603fd2bb72cc8fa358c6061445942799446`
- Select: `74a319ed0427947f9191e4bdc8fe7400b414633ca642a9a44a4abf84b4f44198`
- Panel: `8dd7e066f8646e8fd920c4489c462d0edd0caef98fc076810255dd4a56b06c85`
- Dialog: `9fd40c71222528fd24f2e399d6541be459e08947975d76f65e6e520400425a3d`
- Tree: `837c514ed223178c9327ad097185f70537c0dda0e6d33f727fbe83b3f84ab40e`
- protected React barrel after ElementIdentity registrar: `a9a764971875336ed637b8be0ec1dae23150dfce09985ddf7cd5d69cafc774f6`

Rehash all paths after the prior lease's final gate. Abort and report any source-content mismatch.

## Writable Source Lease

- delete `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️Flow/🟦️component.tsx`
- create `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🧭️flow-direction-context/🟦️component.tsx`
- Popover, PanelTabBar, ContextMenu, Select, Panel, Dialog, and Tree component sources listed above
- one unique Terra acceptance Markdown in this ticket

The React barrel remains coordinator-only. Do not edit it; stop at a registrar handshake.

## Implementation Contract

1. Move `FlowInline`, `FlowBlock`, `Flow`, the default, context, provider, and reader hook intact into the specific module.
2. Keep `FlowInline`, `FlowBlock`, and `Flow` repository-owned and public.
3. Replace exported `React.FC`/`React.ReactNode` provider typing with a private named props contract using opaque children, a private React cast at the adapter boundary, and inferred component return type. Do not expose external-library types.
4. Preserve exact `ltr`/`down` defaults and merge `inline ?? parent.inline`, `block ?? parent.block` semantics.
5. Rewire all seven elements directly to `../../🔨️modules/🧭️flow-direction-context/🟦️component.tsx`.
6. Delete the old source and empty directory. No compatibility alias, bridge, or wrapper.
7. Do not change component behavior, tests, stories, CSS, product sources, manifests, lockfiles, or generated census output.

## Coordinator Registrar

After the source handshake, the coordinator will mechanically change the React barrel's Flow import/export path to the new module while preserving the exact package API symbols used by its authored behavior and inline tests.

## Final Gates

After registrar signal:

- old Flow element path/directory scan is zero;
- exactly seven direct production elements plus the mechanical barrel import the module;
- public module API has no React/external-derived type;
- default/nested override tests still execute through the registered UI test target;
- scoped ordinary and cached `git diff --check` pass;
- run once: UI React lint, typecheck, test-quick, and build through Nx with `--skip-nx-cache`;
- record exact outcomes, hashes, imports, and blockers in the acceptance Markdown; do not repair unrelated baseline failures.
