from specs.inl import inline_range

N = "dsl"
SPEC = dict(
    name=N,
    children=[
        dict(folder="🔍️queryable", mod="queryable", kind="inline", hoist_doc=True, ranges=[inline_range(N, "queryable")]),
        dict(folder="🔌️wire", mod="wire", kind="inline", hoist_doc=True, ranges=[inline_range(N, "wire")]),
        dict(folder="🌳️syntax", mod="jack_syntax", ranges=[(551, 955)],
             doc="🌳️ Jack syntax: the query abstract syntax tree and the lexer that turns text into tokens and token spans."),
        dict(folder="🧭️language-service", mod="jack_language_service", ranges=[(957, 1639)],
             doc="🧭️ Jack language service: completion, lint, format, hover and semantic tokens, plus the embedded `DslIdiom` hooks."),
        dict(folder="🧮️parser", mod="jack_parser", ranges=[(1641, 1941)],
             doc="🧮️ Jack parser: tokens to the query abstract syntax tree."),
        dict(folder="⚡️executor", mod="jack_executor", ranges=[(1943, 2249)],
             doc="⚡️ Jack executor: matches a parsed query against a queryable graph and shapes the result."),
    ],
)
