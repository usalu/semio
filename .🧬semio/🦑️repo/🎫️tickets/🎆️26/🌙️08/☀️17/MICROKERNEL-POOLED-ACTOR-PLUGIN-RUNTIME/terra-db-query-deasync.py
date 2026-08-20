#!/usr/bin/env python3
"""🔍 R9 targeted de-asyncify for the pure Value/Predicate/Path cluster in db_query
(🛢️db/🔍️query/🦀️component.rs), db-dedyn packet.

WHY
---
Unlike `db_state` (whole file pure), `db_query` is MIXED: `QuerySource`/`FullTextLookup`/
`ConsistencyResolver`/`execute`/`LiveQuery` genuinely touch storage I/O and must stay async, but the
`Value`/`Predicate`/`Path` core (comparison, tree-walk, wire encode/decode) is pure in-memory logic
that got blind-codemod-asyncified along with everything else — including one outright E1 violation
(`impl From<Vec<u8>> for Value` declared `async fn from`, illegal on an externally-declared trait).
Every function below was individually read and confirmed I/O-free before being listed here.

WHAT IT DOES
------------
Strips exactly the `async` keyword from the listed fn signatures (matched on the FULL declaration
line, not a bare name, so it cannot collide with an unrelated fn sharing one of these common names
elsewhere in the file — R10 is about call-site name-keyed `.await` insertion; this is a signature-level
edit against a hand-verified, hand-enumerated list in one owned file, same discipline as
`terra-number-deasync.py`/`terra-db-state-deasync.py`). Does NOT touch call sites — every `.await` this
creates residue for is cleaned up afterward by the shared `remove-bad-await.py` (diagnostic-driven).

USAGE
-----
    python3 terra-db-query-deasync.py --apply <file>
"""
import re
import sys

# Full signature-line patterns (start-of-declaration through the opening brace or generics), each
# anchored so it cannot match an unrelated fn/method with an incidentally shared short name.
SIGNATURES = [
    r'async fn from\(v: Vec<u8>\) -> Self \{',
    r'async fn value_rank\(value: &Value\) -> u8 \{',
    r'pub async fn compare_values\(a: &Value, b: &Value\) -> Ordering \{',
    r'async fn compare_op\(path: &Path, expected: &Value, value: &Value, accept: fn\(Ordering\) -> bool\) -> bool \{',
    r'async fn eval_predicate\(predicate: &Predicate, value: &Value\) -> bool \{',
    r'async fn compare_rows\(a: &Value, b: &Value, sort: &\[SortKey\]\) -> Ordering \{',
    r'pub async fn plan\(query: &Query\) -> QueryPlan \{',
    r'async fn encode_value\(value: &Value, out: &mut Vec<u8>\) \{',
    r'async fn decode_value\(cursor: &mut ValueCursor<\'_>\) -> Result<Value, DbError> \{',
    r'pub async fn empty\(\) -> Path \{',
    r'pub async fn field\(name: impl Into<String>\) -> Path \{',
    r'pub async fn push_field\(mut self, name: impl Into<String>\) -> Path \{',
    r'pub async fn push_index\(mut self, index: usize\) -> Path \{',
    r'pub async fn parse\(spec: &str\) -> Path \{',
    r"pub async fn get<'a>\(&self, value: &'a Value\) -> Option<&'a Value> \{",
    r'async fn project\(&self, value: &Value\) -> Value \{',
    r'pub async fn ascending\(path: Path\) -> SortKey \{',
    r'pub async fn descending\(path: Path\) -> SortKey \{',
    r"async fn take\(&mut self, n: usize\) -> Result<&'a \[u8\], DbError> \{",
    r'async fn take_u8\(&mut self\) -> Result<u8, DbError> \{',
    r'async fn take_u32\(&mut self\) -> Result<u32, DbError> \{',
]


def main() -> None:
    if len(sys.argv) != 3 or sys.argv[1] not in ("--scan", "--apply"):
        print(__doc__)
        sys.exit(1)
    mode, path = sys.argv[1], sys.argv[2]
    with open(path, encoding="utf-8") as f:
        content = f.read()
    total = 0
    for sig in SIGNATURES:
        pat = re.compile(sig)
        matches = pat.findall(content)
        if not matches:
            print(f"NOT FOUND: {sig}")
            continue
        if len(matches) > 1:
            print(f"AMBIGUOUS ({len(matches)} matches), skipping: {sig}")
            continue
        total += 1
        if mode == "--apply":
            content = pat.sub(lambda m: re.sub(r'\basync\s+', '', m.group(0), count=1), content, count=1)
    print(f"{'would edit' if mode=='--scan' else 'edited'} {total}/{len(SIGNATURES)} signatures")
    if mode == "--apply":
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)


if __name__ == "__main__":
    main()
