// Metabolism live Neo4j — copy-paste queries (each block is standalone).
// Database: metabolism. Design: Nakagin Capsule Tower unless noted.
// Tower suffix: _c0 = small tower, _c1 = large tower.

// --- designs ---

// list-all-designs
MATCH (d:Design)
RETURN d.id AS id, d.name AS name
ORDER BY name;

// nakagin-design-id
MATCH (d:Design {name: 'Nakagin Capsule Tower'})
RETURN d.id AS id;

// piece-count-per-design
MATCH (d:Design)-[:HAS]->(p:Piece)
RETURN d.name AS design, count(p) AS pieces
ORDER BY design;

// --- tambours ---

// all-tambour-pieces-by-type
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Tambour'})
RETURN p.id AS id, p.name AS name
ORDER BY name;

// tambours-small-tower-c0
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Tambour'})
WHERE p.name ENDS WITH '_c0'
RETURN p.id AS id, p.name AS name
ORDER BY name;

// tambours-large-tower-c1
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Tambour'})
WHERE p.name ENDS WITH '_c1'
RETURN p.id AS id, p.name AS name
ORDER BY name;

// tambour-storey-piece-names-t-f
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name =~ 't_f[0-9]+_b_c[01]'
RETURN p.name AS name
ORDER BY name;

// count-tambours-by-tower
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Tambour'})
WHERE p.name ENDS WITH '_c0'
RETURN 'small_c0' AS tower, count(p) AS tambours
UNION ALL
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Tambour'})
WHERE p.name ENDS WITH '_c1'
RETURN 'large_c1' AS tower, count(p) AS tambours;

// --- towers c0 / c1 ---

// all-pieces-small-tower-c0
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name ENDS WITH '_c0'
RETURN p.id AS id, p.name AS name
ORDER BY name;

// all-pieces-large-tower-c1
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name ENDS WITH '_c1'
RETURN p.id AS id, p.name AS name
ORDER BY name;

// small-tower-base-first-storey
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece {name: 't_f0_b_c0'})-[:HAS]->(:Blueprint)-[:IS]->(t:Type)
RETURN p.name AS piece, t.name AS typeName;

// large-tower-top-tambours
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name IN ['t_f9_b_c1', 't_f10_b_c1']
RETURN p.name AS name
ORDER BY name;

// floor-5-tambours-both-towers
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name IN ['t_f5_b_c0', 't_f5_b_c1']
RETURN p.name AS name
ORDER BY name;

// --- capsules ---

// all-capsule-storeys-cs-prefix
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name STARTS WITH 'cs_'
RETURN p.name AS name
ORDER BY name;

// capsules-small-tower
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name STARTS WITH 'cs_' AND p.name ENDS WITH '_c0'
RETURN count(p) AS capsuleCount;

// capsules-large-tower
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name STARTS WITH 'cs_' AND p.name ENDS WITH '_c1'
RETURN count(p) AS capsuleCount;

// capsules-floor-5-small-tower
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name CONTAINS '_t_f5_' AND p.name ENDS WITH '_c0'
RETURN p.name AS name
ORDER BY name;

// third-storey-large-tower-floor-2
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)
WHERE p.name CONTAINS '_t_f2_' AND p.name ENDS WITH '_c1'
RETURN p.name AS name
ORDER BY name;

// --- storeys and kinds ---

// first-and-last-storey-pieces
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type)
WHERE t.name IN ['First Storey', 'Last Storey']
RETURN t.name AS typeName, p.name AS pieceName
ORDER BY typeName, pieceName;

// piece-count-by-type-nakagin
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type)
RETURN t.name AS typeName, count(p) AS pieceCount
ORDER BY pieceCount DESC;

// bridge-pieces-nakagin
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(p:Piece)-[:HAS]->(:Blueprint)-[:IS]->(t:Type {name: 'Bridge'})
RETURN p.name AS name
ORDER BY name;

// --- kit schema (types, connectors, ports) ---

// all-type-names
MATCH (t:Type)
WHERE t.name IS NOT NULL
RETURN DISTINCT t.name AS name, t.isAbstract AS isAbstract
ORDER BY name;

// connectors-per-type
MATCH (ty:Type)-[:HAS]->(conn:Connector)
RETURN ty.name AS typeName, collect(DISTINCT conn.name) AS connectors
ORDER BY typeName;

// tambour-type-connectors
MATCH (ty:Type {name: 'Tambour'})-[:HAS]->(conn:Connector)
RETURN collect(DISTINCT conn.name) AS connectors;

// base-and-blob-tower-connectors
MATCH (ty:Type)-[:HAS]->(conn:Connector)
WHERE ty.name IN ['Base', 'Blob']
RETURN ty.name AS typeName, collect(DISTINCT conn.name) AS connectors
ORDER BY typeName;

// all-ports
MATCH (p:Port)
RETURN p.name AS name
ORDER BY name;

// door-and-tambour-ports
MATCH (p:Port)
WHERE toLower(p.name) CONTAINS 'tambour' OR toLower(p.name) CONTAINS 'capsule'
RETURN p.name AS name
ORDER BY name;

// --- connections ---

// connection-count-nakagin
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)
RETURN count(c) AS connections;

// connector-pair-histogram-named-only
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)-[hp:HAS]->(sp:Side)
WHERE hp.parent = true
MATCH (c)-[hc:HAS]->(sc:Side)
WHERE hc.parent = false
MATCH (sp)-[:REFERENCES]->(pc:Connector)
OPTIONAL MATCH (sc)-[:REFERENCES]->(cc:Connector)
RETURN pc.name AS parentConn, coalesce(cc.name, '(none)') AS childConn, count(*) AS cnt
ORDER BY cnt DESC;

// base-to-first-tambour-c0-c1-to-b
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)-[hp:HAS]->(sp:Side)
WHERE hp.parent = true
MATCH (c)-[hc:HAS]->(sc:Side)
WHERE hc.parent = false
MATCH (sp)-[:REFERENCES]->(pc:Connector)
WHERE pc.name IN ['c0', 'c1']
MATCH (sc)-[:REFERENCES]->(cc:Connector {name: 'b'})
RETURN pc.name AS towerPort, count(c) AS connections;

// tambour-stack-t-to-b
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)-[hp:HAS]->(sp:Side)
WHERE hp.parent = true
MATCH (c)-[hc:HAS]->(sc:Side)
WHERE hc.parent = false
MATCH (sp)-[:REFERENCES]->(pc:Connector {name: 't'})
MATCH (sc)-[:REFERENCES]->(cc:Connector {name: 'b'})
RETURN count(c) AS tambourStackConnections;

// bridge-on-shaft-sl0-d2-to-e
MATCH (d:Design {name: 'Nakagin Capsule Tower'})-[:HAS]->(c:Connection)-[hp:HAS]->(sp:Side)
WHERE hp.parent = true
MATCH (c)-[hc:HAS]->(sc:Side)
WHERE hc.parent = false
MATCH (sp)-[:REFERENCES]->(pc:Connector {name: 'sl0_d2'})
MATCH (sc)-[:REFERENCES]->(cc:Connector {name: 'e'})
RETURN count(c) AS bridgeConnections;

// --- cross-design / selection ---

// piece-by-name-all-designs
MATCH (d:Design)-[:HAS]->(p:Piece {name: 't_f3_b_c1'})
RETURN d.name AS design, p.id AS id
ORDER BY design;

// selection-by-piece-names
MATCH (d:Design {id: '9a890dd4-0a9c-48ac-920a-9e62666465ef'})-[:HAS]->(p:Piece)
WHERE p.name IN ['t_f1_b_c0', 't_f2_b_c0', 'cs_sl2_d0_t_f9_b_c1']
RETURN p.id AS id, p.name AS name
ORDER BY name;

// small-tower-by-design-id-literal
MATCH (d:Design {id: '9a890dd4-0a9c-48ac-920a-9e62666465ef'})-[:HAS]->(p:Piece)
WHERE p.name ENDS WITH '_c0'
RETURN p.id AS id, p.name AS name
ORDER BY name;
