from specs.inl import inline_range

N = "machine"
SPEC = dict(
    name=N,
    children=[
        dict(folder="🌐️host", mod="host", kind="inline", ranges=[inline_range(N, "host")]),
        dict(folder="🔎️inspect", mod="inspect", kind="inline", ranges=[inline_range(N, "inspect")]),
        dict(folder="🧠️kernel", mod="kernel", kind="inline", ranges=[inline_range(N, "kernel")]),
        dict(folder="💾️persist", mod="persist", kind="inline", ranges=[inline_range(N, "persist")]),
        dict(folder="🎬️runtime", mod="runtime", kind="inline", ranges=[inline_range(N, "runtime")]),
        dict(folder="🔁️step", mod="step", kind="inline", ranges=[inline_range(N, "step")]),
        dict(folder="🧭️testing", mod="testing", kind="inline", ranges=[inline_range(N, "testing")]),
    ],
)
