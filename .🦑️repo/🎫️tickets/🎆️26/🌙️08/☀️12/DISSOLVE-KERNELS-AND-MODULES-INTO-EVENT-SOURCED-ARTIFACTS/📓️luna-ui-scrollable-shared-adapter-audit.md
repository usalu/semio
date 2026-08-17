# UI Scrollable Shared Adapter Audit

## Classification

`Scrollable/🟦️component.tsx` is a coherent shared native-scroll adapter. It owns orientation, outer/viewport slots, forwarded host refs, and shared window-content dead-line behavior. It has no reverse barrel edge.

## Consumers

Independent active component consumers are Panel, Table, LayoutMobilePanel, Animate MarkdownMorphView, and Animate JsonMorphView. `orientation="both"` is shared by the two Animate views; `viewportClassName` is an internal adapter escape hatch used by Panel. The mechanical UI barrel and tests do not count.

Scrollable therefore has strong multi-consumer proof and remains at its current UI owner. It should not be split or inlined.

## Boundary and Cycle Evidence

- Public typing exposes React/native DOM adapter types, but no external component library.
- Runtime imports are direct repository-owned Ports, class-name composition, and WindowContentDeadLine.
- Graph is one-way: barrel -> Scrollable -> direct owners. Panel/Table/Layout barrel dependencies do not return through Scrollable.
- The dead-line policy is also consumed by another chrome surface; splitting it from Scrollable lacks consumer-driven evidence.

## Disposition

Retain Scrollable unchanged. A separate Table audit should evaluate the reported zero-consumer `TableSkeleton` facet. No Scrollable source lease is justified.

## Baseline SHA-256

- Scrollable: `bb590f54c3e374e4010c960d4ad131a54b215deaae9845a4e54da2024303c2aa`
- story: `6c53220c3ad4084d58ae4944c7d1ed0ede14a7babd632a9cecebae1089fc09a3`
- Panel: `92f29a27b6e7c62cc2083aba5600751945964e174cd63549bb8ca7f0161beade`
- Table: `d8de6cc8375fd4856e5cd8f5a45a01ee7665a0c52cb945e986eb4c346de2ccb3`
- Layout: `bdd658a1dd3e4729ac1917138b53b7d4ad9458cc40ea4129f038d3955c39ef64`
- Animate renderer: `4d55cdf5f29b1d06c4505b69446497bc0c5866edb1e14903fbda042ccc34a6e6`
