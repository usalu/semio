from specs.inl import inline_range

N = "layoutrun"
SPEC = dict(
    name=N,
    children=[
        dict(folder="📖️vocabulary", mod="layout_vocabulary", ranges=[(44, 210)],
             doc="📖️ Layout run vocabulary: stages, counters, reasons, stop causes and the static `ToolRunDefinition` of the run."),
        dict(folder="🕸️input-graph", mod="layout_graph", ranges=[(212, 380)],
             doc="🕸️ Layout run input: the points, nodes and edges a run lays out, their validation and the positions digest."),
        dict(folder="🎛️config", mod="layout_config", ranges=[(382, 598)],
             doc="🎛️ Layout run configuration: force laws and falloff, the op encoder seam and the resumable checkpoint."),
        dict(folder="🏃️job", mod="layout_job", ranges=[(600, 1578)],
             doc="🏃️ Layout run job: the Barnes-Hut quadtree and the stepped, cancellable `InteractiveJob` that drives iterations."),
        dict(folder="🔮️testing", mod="testing", kind="inline", ranges=[inline_range(N, "testing")],
             doc="🔮️ Layout run testing: the third-party Fruchterman-Reingold oracle, stress metric and the ledger-style run driver."),
    ],
)
