# Live metabolism export

Command:

```powershell
$env:NEO4J_EXTRA_GRAPH_DATABASES="metabolism"
bun ./script.ts generate neo4j metabolism
```

Output: `.repo/🛂/metabolism.cypher` (3780 nodes, 5980 relationships, ~1.06 MB, generated 2026-05-21).
