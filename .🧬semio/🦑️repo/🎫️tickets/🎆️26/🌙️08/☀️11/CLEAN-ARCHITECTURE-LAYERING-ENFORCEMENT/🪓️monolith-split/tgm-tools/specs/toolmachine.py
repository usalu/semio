SPEC = dict(
    name="toolmachine",
    children=[
        dict(folder="🌊️gesture", mod="gesture_drive", ranges=[(457, 915)],
             doc="🌊️ Tool gesture drive: gesture phases, host events, the gesture and press drives and the per-window gesture ledger."),
        dict(folder="🫧️scrub", mod="scrub_tool", ranges=[(917, 1289)],
             doc="🫧️ Scrub tool: the scrub statechart, its host and the scrub ledger that folds a scrubbed value into one transaction."),
        dict(folder="🕸️node-graph-edit", mod="node_graph_edit", ranges=[(1291, 1519)],
             doc="🕸️ Node graph editing: the node drag record, the once and drag emitters and the node graph edit rows."),
        dict(folder="⌨️typing", mod="typing_tool", ranges=[(1521, 1959)],
             doc="⌨️ Typing tool: the typing statechart, its idle-timer host and the typing ledger that commits a buffer as one transaction."),
    ],
)
