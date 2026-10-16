SPEC = dict(
    name="toolrun",
    children=[
        dict(folder="🚦️lifecycle", mod="lifecycle", ranges=[(133, 552)],
             doc="🚦️ Tool run lifecycle: slot state, events, effects, rejections and the pure reducer machine that folds them."),
        dict(folder="🧮️step-progress", mod="step_progress", ranges=[(554, 769)],
             doc="🧮️ Tool run progress: verdicts, reasoned steps, the bounded step ring, counters and the progress snapshot."),
        dict(folder="🗄️trace-store", mod="trace_store", ranges=[(771, 1095)],
             doc="🗄️ Tool run trace: subjects, ops, pages, deltas, cursors and the resident trace store."),
        dict(folder="⏱️tick-writer", mod="tick_writer", ranges=[(1097, 1444)],
             doc="⏱️ Tool run tick: the tick record, its limit errors and the bounded tick writer."),
        dict(folder="🔤️codec", mod="record_codec", ranges=[(1446, 1865)],
             doc="🔤️ Tool run codec: record specs and the pure encode and decode of identity, step, progress and page records."),
        dict(folder="📐️definition", mod="definition", ranges=[(1867, 2140)],
             doc="📐️ Tool run definition: the static `ToolRunDefinition` vocabulary and the ToolRun panel group keys."),
        dict(folder="🏷️actions-labels", mod="actions_labels", ranges=[(2142, 2397)],
             doc="🏷️ Tool run actions and labels: the framework-reserved action ids, chords and the EN/DE panel text."),
    ],
)
