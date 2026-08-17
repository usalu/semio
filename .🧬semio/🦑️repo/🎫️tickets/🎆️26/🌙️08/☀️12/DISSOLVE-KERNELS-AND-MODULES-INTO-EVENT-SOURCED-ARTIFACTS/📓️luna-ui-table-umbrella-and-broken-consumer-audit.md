# UI Table Umbrella and Broken Consumer Audit

## Classification

`Table/🟦️component.tsx` owns a coherent shared data-table/interaction component plus a separately owned `TableSkeleton` loading facet. Its core contracts include columns, table props, hierarchy, selection, focus, native and dnd-kit row interaction, and mobile rendering.

## Consumer Evidence

- Product TableHost is a valid runtime consumer through the UI package.
- VirtualFileSystem directly consumes Table contracts and intends to render Table, but its runtime value import is currently wrong: it imports `Table` from `Skeletons/🧪️story.tsx`, where the value is Storybook metadata rather than the component.
- `TableColumn` has two independent production type consumers: TableHost and VirtualFileSystem.
- `TableSkeleton`/props have zero production consumers; only stories and barrel glue reference them.

Correcting the VirtualFileSystem value import yields two independent runtime semantic consumers while remaining cycle-free.

## Inactive Contract Facets

- sort props and `TableColumn.sortable` are declared but Table renders no affordance and never invokes `onSort`;
- hierarchy toggle/render controls are destructured but unused;
- `canDrop` and drag overlay are declared but unused;
- all row-height variants currently map to the same height.

These require separate behavior decisions and must not be promoted to modules.

## Disposition

1. Immediately correct VirtualFileSystem's story import to direct Table component import.
2. Retain Table and its core contracts at the shared owner.
3. Move zero-production `TableSkeleton`/props into the existing Skeletons owner and mechanically re-register them there.
4. Audit/remove or implement inactive contract facets separately.

## Boundary and Cycle Evidence

- Public contracts expose React adapter types; dnd-kit remains runtime-only behind the component.
- Table -> Scrollable/direct owners is one-way; product consumers do not return into Table.
- The corrected VirtualFileSystem direct edge creates no SCC.

## Baseline SHA-256

- Table: `d8de6cc8375fd4856e5cd8f5a45a01ee7665a0c52cb945e986eb4c346de2ccb3`
- Table story: `169aabd0b37c1dab16d3f5fd800c3f97b3b819db546270eb1d2ebd1054333c9f`
- VirtualFileSystem: `3c1ce5cfc96b49967d1f9a1050fea59f0d91385e7198bc7b9b1857aabd9c7540`
- Skeletons: `ca6d4b5fa652355f59860a8466777c2462b5157568e8da8d5f7b724f2659e8bc`
- React barrel observed during audit: `4e916cf18ad6c1a44961405f6adddb20b0a7383e3283af306f5c756e016ca52d`
