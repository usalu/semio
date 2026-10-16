SPEC = dict(
    name="engine",
    children=[
        dict(folder="🗄️storage", mod="graph_storage", ranges=[(179, 864)],
             doc="🗄️ Graph storage: the adjacency-map `Storage`, its view traits and weight sources, and the CSR snapshot hot algorithms read."),
        dict(folder="🪟️views", mod="graph_views", ranges=[(866, 1346)],
             doc="🪟️ Graph views: subgraph, edge-subgraph, reversed, filtered and undirected adapters over any `GraphView`."),
        dict(folder="🧰️support", mod="graph_support", ranges=[(1348, 1617)],
             doc="🧰️ Graph support: the interner, `GraphError`, the numeric tolerances and the mapped heap."),
        dict(folder="🔖️kinds", mod="graph_kinds", ranges=[(1625, 1751)],
             doc="🔖️ Graph kinds: node, handle and element vocabulary with the property-bag value bridges."),
        dict(folder="🌊️max-flow", mod="graph_flow", ranges=[(1753, 1876)],
             doc="🌊️ Graph max-flow: the flow network over index-based edges."),
    ],
)
