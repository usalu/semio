PLAN = dict(
    name="constraints",
    orig="svm-tools/orig/constraints.rs",
    real="🧰️framework/🔨️modules/🧮️math/🎯️sampling/🚧️constraints/🦀️.rs",
    real_lib="🧰️framework/🔨️modules/🧮️math/🎯️sampling/🚧️constraints/🦀️.rs",
    children=[
        dict(key="stops", folder="🚦️stop-conditions", mod="stop_conditions", doc="🚦️ Stop conditions: Aho-Corasick byte matcher and the token and text stop conditions.", ranges=[(45, 211)]),
        dict(key="automata", folder="🤖️regex-automata", mod="regex_automata", doc="🤖️ Byte-level regex parser, NFA builder and lazily memoised DFA.", ranges=[(220, 740)]),
        dict(key="rules", folder="🧱️mask-constraints", mod="mask_constraints", doc="🧱️ Token-mask constraints: regex, trie, must-include, JSON mode, EBNF and JSON schema.", ranges=[(742, 1571)]),
    ],
    oracle=dict(trees=["🧰️framework/🔨️modules/🧮️math/🎯️sampling/🚧️constraints"], crate="semio_framework_math_sampling_constraints", cfgs=["lib", "test", "wasm", "p2"]),
    no_widen=["field AhoCorasick.nodes", "field TextStopCondition.", "field Dfa.", "field TrieConstraint.", "field EbnfGrammar.start", "tuple field of EbnfConstraint"],
)
