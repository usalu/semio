# UI Toggle Group Multiconsumer Retention Audit

## Snapshot

- Toggle-group implementation: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
- SHA-256: `06c2e3b7f0468108f56a32966a4701f613fa99bad7d7967cfcd147f6013fb8bf`
- Story SHA-256: `742763c8372df6d8e7230e77845debb6c2da42f53ce58eb717a7ea407d6eb8f5`
- Both definition and story were clean at audit time.
- Shared React barrel SHA-256: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.

## Production Closure

- `ToggleGroup` is consumed by the UI React engagement/rendering implementation and by the protected OS renderer `UtilityTree` and `ShellHelpers` components.
- `Toggle` is consumed by the UI React component implementation, by `Tree`, and by protected OS renderer components.
- Window and Ribbon occurrences in stories are non-production evidence only.
- The similarly named CAD `SpatialToggleGroup*` state helpers are independent plugin behavior and are not consumers.

## Disposition

Retain the current family for this wave. It has multiple independent active production terminal consumers and protected renderer consumers, so it is neither a zero-consumer deletion nor a safe one-consumer inline lease. The adjacent `🎚️Toggle` component currently owns the Toggle contracts and variants while `🎛️ToggleGroup` owns the executable Toggle wrapper; that ownership split is an audit finding, but repairing it requires one atomic SCC lease covering both components, the shared React registrar, `Tree`, and protected renderer consumers. It must not be folded into an unrelated cleanup lease.
