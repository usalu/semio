from . import _ext
F = '✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs'
SPEC = dict(
    orig='draw-pointerdown.rs.txt',
    file=F,
    wiring='each',
    children=[
        dict(folder='🎯️tool-context', mod='tool_context',
             ranges=[(60, 340)],
             doc='//! 🎯️ Drawing tool context: the lasso polygon, the grab and tool context records, the interaction effects and the gesture preview layers.'),
        dict(folder='🛠️canvas-tool', mod='canvas_tool_machine',
             ranges=[(344, 768)],
             doc='//! 🛠️ Drawing canvas tool: the pointer records, the guards and actions of the canvas statechart, the statechart itself and the `DrawingTool` runner.'),
        dict(folder='🧵️trace-pointer-job', mod='trace_pointer_job',
             ranges=[(772, 1066), (1557, 1601)],
             doc='//! 🧵️ Drawing trace pointer job: the bounded trace pick job, its candidates and path helpers and the trace continuation queue.'),
        dict(folder='🖌️drawing-session', mod='drawing_session',
             ranges=[(1071, 1552)],
             doc='//! 🖌️ Drawing session: the retained gesture session with its previews, grab preparation, point queries and the pointer entry points of the canvas tool.'),
    ],
    external=_ext.mounted(F),
)
