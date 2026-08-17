# UI Element Props and Dead Transaction Audit

## Responsibilities

The current `🐹️ElementProps` source combines:

- a shared `{ id: string }` element identity contract;
- an optional transaction lifecycle contract and module-evaluated React context/provider/hook.

## Consumer Evidence

The element identity contract has nine independent framework UI consumers: HistoryTable, Ring, Toggle, Textarea, Stepper, Slider, Tree, Select, and Input. It therefore meets the shared-module threshold at the UI owner.

The transaction provider has zero active production mounts. `Transaction`, `TransactionProvider`, and `ElementBaseProps` have zero active production consumers. Seven components call `useTransaction`, but every call resolves `undefined` in the active graph because no provider is mounted; all lifecycle calls are inert. The React barrel is glue and does not count. ActionDropdown's explicit transaction callback props are a separate component responsibility and remain unchanged.

## Runtime and Boundary Findings

The current source is outside the barrel SCC because it imports Ports directly. The dead `TransactionContext` is nevertheless created at module evaluation and exposes anonymous React provider props. Removing it eliminates both inert runtime initialization and an external React-shaped public surface.

## Disposition

- Delete `Transaction`, `TransactionContext`, `TransactionProvider`, `useTransaction`, and `ElementBaseProps`.
- Remove inert hook reads and lifecycle calls from ActionGroup, Ring, Textarea, Stepper, Slider, Select, and Input.
- Preserve explicit transaction callback props owned by other components.
- Move the retained shared `ElementProps { id: string }` contract from the element collection to a specific UI-owner `element-identity` module. Nine independent consumers prove qualified shared ownership; no visual element identity remains.
- Rewire every direct consumer and the explicit React-barrel type registration to the module. Delete the old element source/directory without a forwarding compatibility export.

This lease is independent of Label and can execute after the active keybinding registrar releases the shared barrel.
