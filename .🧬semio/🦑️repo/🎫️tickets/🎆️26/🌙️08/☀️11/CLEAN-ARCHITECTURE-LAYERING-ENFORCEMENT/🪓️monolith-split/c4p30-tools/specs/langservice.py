from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs'
SPEC = dict(
    orig='jack-langservice.rs.txt',
    file=F,
    wiring='each',
    unwrap_removed=['pub mod queryable {', '}'],
    children=[
        dict(folder='🔗️queryable-graph', mod='queryable', unwrap='queryable', modvis='pub', glob=False, reexport=False, nowiden=True,
             ranges=[(10, 116)],
             doc='//! 🔗️ Jack queryable graph: the borrowed and owned `QueryableGraph` adapters of the jack graph for the graph query language.'),
        dict(folder='🩺️completion-and-lint', mod='completion_and_lint',
             ranges=[(122, 496)],
             doc='//! 🩺️ Jack completion and lint: the completion context, the semantic lints, the formatting, the hover and the semantic tokens of the jack language service.'),
        dict(folder='🔍️query-parser', mod='query_parser',
             ranges=[(506, 710)],
             doc='//! 🔍️ Jack query parser: the recursive descent `Parser` of the semantic `Query` AST.'),
        dict(folder='🌳️spanned-parser', mod='spanned_parser',
             ranges=[(711, 1002)],
             doc='//! 🌳️ Jack spanned parser: the span-tracked `SpannedNode` tree for editor tree, hover and outline surfaces.'),
        dict(folder='🧩️example-fixture', mod='example_fixture',
             ranges=[(497, 503), (1003, 1031)],
             doc='//! 🧩️ Jack example fixture: the example snapshot and its resolved in-memory graph.'),
    ],
    external=_ext.mounted(F),
)
