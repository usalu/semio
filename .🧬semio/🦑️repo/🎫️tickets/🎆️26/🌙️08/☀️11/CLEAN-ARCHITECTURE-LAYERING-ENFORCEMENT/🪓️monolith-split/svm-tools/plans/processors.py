PLAN = dict(
    name="processors",
    orig="svm-tools/orig/processors.rs",
    real="🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎛️processors/🦀️.rs",
    real_lib="🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎛️processors/🦀️.rs",
    children=[
        dict(key="warpers", folder="🌡️warpers", mod="warpers", doc="🌡️ Warpers: temperature, top-k, top-p, min-p, typical, tail-free, cutoffs and truncations.", ranges=[(35, 457)]),
        dict(key="penalties", folder="⚖️penalties", mod="penalties", doc="⚖️ Repetition, presence, frequency, decaying, class, n-gram and phrase penalties.", ranges=[(498, 1033)]),
        dict(key="controls", folder="🎚️logit-controls", mod="logit_controls", doc="🎚️ Logit biases, token allow and forbid lists, bad words, and length controls.", ranges=[(1035, 1302)]),
        dict(key="selection", folder="🎯️selection", mod="selection", doc="🎯️ Selection samplers: greedy, multinomial, Gumbel-max and Gumbel top-k over an alias table.", ranges=[(1304, 1509)]),
        dict(key="adaptive", folder="🌀️adaptive", mod="adaptive", doc="🌀️ Adaptive controllers: Mirostat, entropy PID, repetition and confidence control.", ranges=[(1531, 1753)]),
    ],
    oracle=dict(trees=["🧰️framework/🔨️modules/🧮️math/🎯️sampling/🎛️processors"], crate="semio_framework_math_sampling_processors", cfgs=["lib", "test", "wasm", "p2"]),
    no_widen=["field FreqTable.counts", "method AliasTable::sample"],
)
