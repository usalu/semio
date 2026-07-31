// Live metabolism DB only — removed top 2 storeys per tower on Nakagin Capsule Tower (2026-05-21).
// Small tower (_c0): t_f7, t_f8 (+ shaft/capital/bridge pieces on those storeys).
// Large tower (_c1): t_f9, t_f10 (+ shaft/capital pieces on those storeys).
// Result: 142 pieces (was 180), 141 connections (was 179).

// Step 1 — connections (+ sides) touching removed storey pieces (38 ids from kit)
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)
WHERE c.id IN $connectionIds
OPTIONAL MATCH (c)-[:HAS]->(s:Side)
DETACH DELETE c, s;

// Step 2 — pieces (+ blueprints) on removed storeys (38 ids)
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.id IN $pieceIds
OPTIONAL MATCH (p)-[:HAS]->(b:Blueprint)
DETACH DELETE p, b;

// Verify remaining storey tambours
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name =~ 't_f[0-9]+_b_c[01]'
MATCH (p)-[:HAS]->(:Blueprint)-[:IS]->(t:Type)
RETURN p.name AS name, t.name AS kind ORDER BY name;
