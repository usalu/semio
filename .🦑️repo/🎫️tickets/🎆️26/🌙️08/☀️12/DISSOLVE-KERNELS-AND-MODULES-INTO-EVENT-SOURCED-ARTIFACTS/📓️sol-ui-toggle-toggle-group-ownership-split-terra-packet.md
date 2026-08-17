# Terra Packet UI-Toggle-01: Toggle and Toggle Group Ownership Split

## Preconditions

- Read root/UI AGENTS and `📓️sol-ui-toggle-group-multiconsumer-retention-audit.md`.
- Apply patches only; no modifying Git commands.
- Require clean source hashes:
  - `🎛️ToggleGroup/🟦️component.tsx`: `06c2e3b7f0468108f56a32966a4701f613fa99bad7d7967cfcd147f6013fb8bf`
  - `🎚️Toggle/🟦️component.tsx`: `357f383e92385ea61288588615e090929725454547b151f155fb6a14a2bd5b15`
  - `🪵️Tree/🟦️component.tsx`: `b97d40a3e35e871339026750af7ba4aa7cc4e6dbc3e86fd3c4db0a43cbc99edc`
- Shared React barrel is coordinator-owned and must not be edited. Its expected pre-registrar SHA-256 is `fdd7e8ec24ea5288b386bab04f2627d81194712e2461860e8e2abcead71a4a23`.

## Terra Writable Closure

1. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Toggle/🟦️component.tsx`
2. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
3. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx`
4. Unique acceptance `📓️terra-ui-toggle-toggle-group-ownership-split-acceptance.md`

## Required Result

- Move the executable `Toggle` wrapper and `addIconSize` private helper from ToggleGroup to the specific Toggle component.
- Keep all Toggle contracts and executable behavior together in Toggle. It may depend on public `ToggleGroup` as its rendering primitive.
- Move `toggleVariants` into ToggleGroup as a private implementation detail because active-source scanning found no independent production consumer; do not export it from either component.
- ToggleGroup retains only its context, contracts, root/item implementation, and private styling.
- Update Tree's only direct `Toggle` import to the specific Toggle path.
- Avoid a dependency cycle: Toggle imports ToggleGroup; ToggleGroup must not import Toggle or Toggle contracts.
- Preserve package-level public `Toggle`, `ToggleProps`, `ToggleItem`, `ToggleGroup`, and `ToggleGroupItem` behavior. `toggleVariants` is removed as dead public glue.
- Do not edit shared barrel, stories, manifests/locks, generated files, protected renderer, or plugins.

Stop after source move and send exact hashes, dependency-direction scan, and scoped diff. Coordinator will update the shared barrel and remove its unused raw Radix imports. After signal, run stale-path/symbol checks and the UI React lint/typecheck/test-quick/build once without unrelated repair.
