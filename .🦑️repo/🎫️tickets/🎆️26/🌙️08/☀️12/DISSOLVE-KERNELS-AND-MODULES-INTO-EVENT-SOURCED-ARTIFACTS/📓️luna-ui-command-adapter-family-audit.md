# UI Command Adapter Family Audit

## Classification

`Command/🟦️component.tsx` is a coherent `cmdk` adapter family. Its wrappers jointly own command-dialog hosting, input, list, empty/group/item shells, and shortcut presentation. It is not a mixed semantic umbrella and has no reverse React-barrel edge.

## Production Consumers

- OS `ShellSearch` owns separately named `UISearch` and `UIFind` semantic components, both of which use the dialog/input/list/empty/group/item family.
- The framework `Search` semantic component currently misplaced in the React barrel independently uses Command, list, empty, group, and item.
- The protected OS and UI barrels are assembly/glue and do not count themselves.

These are independent terminal semantic components, not merely multiple calls inside one component. The principal wrappers therefore satisfy the shared-family consumer threshold.

`Command` is also internal to `CommandDialog`; `CommandShortcut` is retained as a cohesive primitive/story surface. `CommandSeparator` is private and has zero consumers, so delete it.

## Boundary Evidence

- Inferred wrapper signatures expose React/cmdk-derived types.
- `CommandDialog` also exposes Dialog-derived props.
- Repository-owned presentation and labels are already imported from direct owners.
- The component graph is one-way: React barrel -> Command -> cmdk/Dialog/specific modules. No SCC exists.

External-type cleanup requires defining repository-owned adapter contracts without changing protected OS consumers; it is a separate larger lease. The minimum dead-code closure is the Command source only.

## Disposition

- Retain the command adapter family and its public wrappers.
- Delete only unused private `CommandSeparator` now.
- Do not split/inline into protected OS ShellSearch.

## Baseline SHA-256

- Command: `a42551eb3cf50b3b1284db3ce9c7f2afb900ddc787434abc2cef486c90f09b3e`
- story: `c9a22b979bca4bf6c1760d899aaafb74ea971ef9a253ec36f7a97885a3047443`
- React barrel: `537138eb89f28302991e6b38f2aea879f7ee19cacbd495d5e23517a7755b4e5d`
- OS ShellSearch: `bcc3d9e4756556a1f64c1fadf9455be03564da168c033d889718337841ecd33a`
