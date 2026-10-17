from specs.lib import *
import c4p45x as X

FR = [
    ('🧭' + V + 'match-parse-and-run-laws', (4, 301), '🧭' + V + ' Match parse and run laws of the graph DSL: the synchronous test executor, the match, edge and port parses and the DAG fixture and port-filtered query runs.'),
    ('🕸' + V + 'graph-and-error-laws', (303, 539), '🕸' + V + ' Graph and error laws of the graph DSL: the split endpoint fixtures, the queryable graph, snapshot and manifest helpers and the parse and lex error reports.'),
    ('🔤' + V + 'lexer-and-language-service-laws', (541, 746), '🔤' + V + ' Lexer and language service laws of the graph DSL: the tokenizer, the formatter, completion, hover, lint and semantic tokens.'),
    ('⚙' + V + 'parser-and-executor-laws', (748, 1056), '⚙' + V + ' Parser and executor laws of the graph DSL: the clause parsers, the WITH, UNWIND and CALL execution, the executor semantics and the dash connector.'),
]

SPEC = dict(
    orig='gdt.rs',
    file=GDT + '/🦀️.rs',
    build=lambda t: X.build_include(t, FR),
    expect_extra=['/// ' + d for _, _, d in FR],
    children=[dict(folder=f, mod=None) for f, _, _ in FR],
)
