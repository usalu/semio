from specs.lib import *
F = DB + '/🔍️query/🦀️.rs'
CH = [
    ch('🧮', 'value-and-path', 'value_and_path', [(41, 365)],
       'Value and path: the cursor control, the ordered query bytes, the `Value` model with its comparison and rendering, and the `Path` segments.'),
    ch('🎚', 'consistency-and-query-model', 'consistency_and_query_model', [(369, 633)],
       'Consistency and query model: the consistency levels and resolvers, predicates, selects, sort keys and the `Query` builder.'),
    ch('🧾', 'limits-and-rows', 'limits_and_rows', [(637, 993)],
       'Limits and rows: the query limits, the retained row sets with their retirement reservations and maintenance step.'),
    ch('🔌', 'query-source-and-projection', 'query_source_and_projection', [(995, 1117)],
       'Query source and projection: the `QuerySource` and full-text lookup traits and the projection bridge.'),
    ch('🗺', 'planner-execute-and-live-query', 'planner_execute_and_live_query', [(1121, 1457)],
       'Planner, execute and live query: the query plan, execution, the result stream and the live query with its diffs.'),
]
SPEC = dict(orig='query.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='query.rs'))
