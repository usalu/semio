// SPDX-License-Identifier: AGPL-3.0-only
// Neo4j Cypher bundle for semio schema (generated from semio/client/schema/semio/schema.yaml).
// Replay: load into database `semio` (see NEO4J_DATABASE). Bloom: pick database `semio`, auto-generate Perspective, search e.g. `kit` or `interface`.

MATCH (n) WHERE n:module OR n:interface OR n:class OR n:field OR n:scalar OR n:command DETACH DELETE n;

MERGE (n:scalar { id: 'scalar:string' })
SET n = { id: 'scalar:string', name: 'string' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:scalar { id: 'scalar:number' })
SET n = { id: 'scalar:number', name: 'number' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:scalar { id: 'scalar:boolean' })
SET n = { id: 'scalar:boolean', name: 'boolean' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:boolean' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:scalar { id: 'scalar:timestamp' })
SET n = { id: 'scalar:timestamp', name: 'timestamp' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:scalar { id: 'scalar:color' })
SET n = { id: 'scalar:color', name: 'color' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:color' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:scalar { id: 'scalar:uri' })
SET n = { id: 'scalar:uri', name: 'uri' };

MATCH (a:module { id: 'module:general' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:module { id: 'module:general' })
SET n = { id: 'module:general', name: 'general' };

MERGE (n:module { id: 'module:domain' })
SET n = { id: 'module:domain', name: 'domain' };

MERGE (n:`interface` { id: 'interface:entity' })
SET n = { id: 'interface:entity', name: 'entity' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:weakEntity' })
SET n = { id: 'interface:weakEntity', name: 'weakEntity' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:data' })
SET n = { id: 'interface:data', name: 'data' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:strongEntity' })
SET n = { id: 'interface:strongEntity', name: 'strongEntity' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:richStrongEntity' })
SET n = { id: 'interface:richStrongEntity', name: 'richStrongEntity' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:richStrongEntity' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:artifact' })
SET n = { id: 'interface:artifact', name: 'artifact' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:document' })
SET n = { id: 'interface:document', name: 'document' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:document' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:event' })
SET n = { id: 'interface:event', name: 'event' };

MATCH (a:module { id: 'module:general' }), (b:`interface` { id: 'interface:event' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:workspace' })
SET n = { id: 'interface:workspace', name: 'workspace' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:workspace' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:diff' })
SET n = { id: 'interface:diff', name: 'diff' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:diff' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:modification' })
SET n = { id: 'interface:modification', name: 'modification' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:modification' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:operation' })
SET n = { id: 'interface:operation', name: 'operation' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:operation' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:backbone' })
SET n = { id: 'interface:backbone', name: 'backbone' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`interface` { id: 'interface:provider' })
SET n = { id: 'interface:provider', name: 'provider' };

MATCH (a:module { id: 'module:domain' }), (b:`interface` { id: 'interface:provider' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`command` { id: 'command:command' })
SET n = { id: 'command:command', name: 'command' };

MATCH (a:module { id: 'module:domain' }), (b:`command` { id: 'command:command' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:vector' })
SET n = { id: 'class:vector', name: 'vector' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:vector' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:point' })
SET n = { id: 'class:point', name: 'point' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:point' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:coordinate' })
SET n = { id: 'class:coordinate', name: 'coordinate' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:coordinate' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:offset' })
SET n = { id: 'class:offset', name: 'offset' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:offset' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:plane' })
SET n = { id: 'class:plane', name: 'plane' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:plane' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:position' })
SET n = { id: 'class:position', name: 'position' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:position' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:location' })
SET n = { id: 'class:location', name: 'location' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:location' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:attribute' })
SET n = { id: 'class:attribute', name: 'attribute' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:place' })
SET n = { id: 'class:place', name: 'place' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:place' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:family' })
SET n = { id: 'class:family', name: 'family' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:family' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:folder' })
SET n = { id: 'class:folder', name: 'folder' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:folder' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:file' })
SET n = { id: 'class:file', name: 'file' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:file' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:author' })
SET n = { id: 'class:author', name: 'author' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:author' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:prop' })
SET n = { id: 'class:prop', name: 'prop' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:prop' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:benchmark' })
SET n = { id: 'class:benchmark', name: 'benchmark' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:benchmark' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:quality' })
SET n = { id: 'class:quality', name: 'quality' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:quality' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:tag' })
SET n = { id: 'class:tag', name: 'tag' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:tag' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:concept' })
SET n = { id: 'class:concept', name: 'concept' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:concept' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:port' })
SET n = { id: 'class:port', name: 'port' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:connector' })
SET n = { id: 'class:connector', name: 'connector' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:connector' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:representation' })
SET n = { id: 'class:representation', name: 'representation' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:representation' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:type' })
SET n = { id: 'class:type', name: 'type' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:type' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:layer' })
SET n = { id: 'class:layer', name: 'layer' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:layer' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:piece' })
SET n = { id: 'class:piece', name: 'piece' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:piece' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:side' })
SET n = { id: 'class:side', name: 'side' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:side' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:connection' })
SET n = { id: 'class:connection', name: 'connection' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:connection' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:group' })
SET n = { id: 'class:group', name: 'group' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:group' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:design' })
SET n = { id: 'class:design', name: 'design' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:design' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:kit' })
SET n = { id: 'class:kit', name: 'kit' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:kit' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:edit' })
SET n = { id: 'class:edit', name: 'edit' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:edit' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:change' })
SET n = { id: 'class:change', name: 'change' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:change' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:checkpoint' })
SET n = { id: 'class:checkpoint', name: 'checkpoint' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:checkpoint' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:theKit' })
SET n = { id: 'class:theKit', name: 'theKit' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:theKit' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:alternative' })
SET n = { id: 'class:alternative', name: 'alternative' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:alternative' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:graph' })
SET n = { id: 'class:graph', name: 'graph' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:graph' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:conflict' })
SET n = { id: 'class:conflict', name: 'conflict' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:conflict' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:store' })
SET n = { id: 'class:store', name: 'store' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:store' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:fileBackbone' })
SET n = { id: 'class:fileBackbone', name: 'fileBackbone' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:fileBackbone' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:websocketBackbone' })
SET n = { id: 'class:websocketBackbone', name: 'websocketBackbone' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:websocketBackbone' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:localProvider' })
SET n = { id: 'class:localProvider', name: 'localProvider' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:localProvider' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:remoteProvider' })
SET n = { id: 'class:remoteProvider', name: 'remoteProvider' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:remoteProvider' })
MERGE (a)-[:OWNS]->(b);

MERGE (n:`class` { id: 'class:session' })
SET n = { id: 'class:session', name: 'session' };

MATCH (a:module { id: 'module:domain' }), (b:`class` { id: 'class:session' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:weakEntity' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:data' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:strongEntity' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:`interface` { id: 'interface:richStrongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:document' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:event' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:diff' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:modification' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:operation' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:backbone' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`interface` { id: 'interface:provider' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`command` { id: 'command:command' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:vector' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:point' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:coordinate' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:offset' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:plane' }), (b:`interface` { id: 'interface:data' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:position' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:location' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:attribute' }), (b:`interface` { id: 'interface:weakEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:place' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:family' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:folder' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:file' }), (b:`interface` { id: 'interface:document' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:author' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:prop' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:benchmark' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:quality' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:tag' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:concept' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:port' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:connector' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:representation' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:type' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:layer' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:piece' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:side' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:connection' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:group' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:theKit' }), (b:`interface` { id: 'interface:workspace' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:alternative' }), (b:`interface` { id: 'interface:workspace' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:conflict' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:fileBackbone' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:websocketBackbone' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:localProvider' }), (b:`interface` { id: 'interface:provider' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:remoteProvider' }), (b:`interface` { id: 'interface:provider' })
MERGE (a)-[:EXTENDS]->(b);

MATCH (a:`class` { id: 'class:session' }), (b:`interface` { id: 'interface:strongEntity' })
MERGE (a)-[:EXTENDS]->(b);

MERGE (n:field { id: 'field:general:interface:entity:id' })
SET n = { id: 'field:general:interface:entity:id', name: 'id', path: 'id', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:entity:id' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:entity' }), (b:field { id: 'field:general:interface:entity:id' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:entity:id' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:entity:hash' })
SET n = { id: 'field:general:interface:entity:hash', name: 'hash', path: 'hash', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:entity:hash' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:entity' }), (b:field { id: 'field:general:interface:entity:hash' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:entity:hash' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:entity:owner' })
SET n = { id: 'field:general:interface:entity:owner', name: 'owner', path: 'owner', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:entity:owner' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:entity' }), (b:field { id: 'field:general:interface:entity:owner' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:entity:owner' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:entity:owns' })
SET n = { id: 'field:general:interface:entity:owns', name: 'owns', path: 'owns', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:entity:owns' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:entity' }), (b:field { id: 'field:general:interface:entity:owns' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:entity:owns' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:richStrongEntity:data.name' })
SET n = { id: 'field:general:interface:richStrongEntity:data.name', name: 'name', path: 'data.name', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:richStrongEntity:data.name' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:field { id: 'field:general:interface:richStrongEntity:data.name' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:richStrongEntity:data.name' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:richStrongEntity:data.description' })
SET n = { id: 'field:general:interface:richStrongEntity:data.description', name: 'description', path: 'data.description', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:richStrongEntity:data.description' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:field { id: 'field:general:interface:richStrongEntity:data.description' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:richStrongEntity:data.description' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:richStrongEntity:data.icon' })
SET n = { id: 'field:general:interface:richStrongEntity:data.icon', name: 'icon', path: 'data.icon', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:richStrongEntity:data.icon' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:field { id: 'field:general:interface:richStrongEntity:data.icon' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:richStrongEntity:data.icon' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:richStrongEntity:computed.createdAt' })
SET n = { id: 'field:general:interface:richStrongEntity:computed.createdAt', name: 'createdAt', path: 'computed.createdAt', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:richStrongEntity:computed.createdAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:field { id: 'field:general:interface:richStrongEntity:computed.createdAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:richStrongEntity:computed.createdAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:richStrongEntity:computed.createdBy' })
SET n = { id: 'field:general:interface:richStrongEntity:computed.createdBy', name: 'createdBy', path: 'computed.createdBy', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:richStrongEntity:computed.createdBy' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:richStrongEntity' }), (b:field { id: 'field:general:interface:richStrongEntity:computed.createdBy' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:richStrongEntity:computed.createdBy' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.authoredBy' })
SET n = { id: 'field:general:interface:artifact:computed.authoredBy', name: 'authoredBy', path: 'computed.authoredBy', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.authoredBy' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.authoredBy' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.authoredBy' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.changedIn' })
SET n = { id: 'field:general:interface:artifact:computed.changedIn', name: 'changedIn', path: 'computed.changedIn', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.changedIn' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.changedIn' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.changedIn' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.lastChangedAt' })
SET n = { id: 'field:general:interface:artifact:computed.lastChangedAt', name: 'lastChangedAt', path: 'computed.lastChangedAt', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.lastChangedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.lastChangedBy' })
SET n = { id: 'field:general:interface:artifact:computed.lastChangedBy', name: 'lastChangedBy', path: 'computed.lastChangedBy', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedBy' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedBy' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.lastChangedBy' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.lastChangedIn' })
SET n = { id: 'field:general:interface:artifact:computed.lastChangedIn', name: 'lastChangedIn', path: 'computed.lastChangedIn', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedIn' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.lastChangedIn' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.lastChangedIn' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.changes' })
SET n = { id: 'field:general:interface:artifact:computed.changes', name: 'changes', path: 'computed.changes', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.changes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.changes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.changes' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:artifact:computed.edits' })
SET n = { id: 'field:general:interface:artifact:computed.edits', name: 'edits', path: 'computed.edits', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:artifact:computed.edits' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:artifact' }), (b:field { id: 'field:general:interface:artifact:computed.edits' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:artifact:computed.edits' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:document:reference.previewImage' })
SET n = { id: 'field:general:interface:document:reference.previewImage', name: 'previewImage', path: 'reference.previewImage', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:document:reference.previewImage' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:document' }), (b:field { id: 'field:general:interface:document:reference.previewImage' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:document:reference.previewImage' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:event:data.timestamp' })
SET n = { id: 'field:general:interface:event:data.timestamp', name: 'timestamp', path: 'data.timestamp', list: false };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:event:data.timestamp' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:event' }), (b:field { id: 'field:general:interface:event:data.timestamp' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:event:data.timestamp' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:general:interface:event:reference.involves' })
SET n = { id: 'field:general:interface:event:reference.involves', name: 'involves', path: 'reference.involves', list: true };

MATCH (a:module { id: 'module:general' }), (b:field { id: 'field:general:interface:event:reference.involves' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:event' }), (b:field { id: 'field:general:interface:event:reference.involves' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:general:interface:event:reference.involves' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:workspace:data.savedChanges' })
SET n = { id: 'field:domain:interface:workspace:data.savedChanges', name: 'savedChanges', path: 'data.savedChanges', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:workspace:data.savedChanges' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:field { id: 'field:domain:interface:workspace:data.savedChanges' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:workspace:data.savedChanges' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:workspace:data.unsavedChanges' })
SET n = { id: 'field:domain:interface:workspace:data.unsavedChanges', name: 'unsavedChanges', path: 'data.unsavedChanges', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:workspace:data.unsavedChanges' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:field { id: 'field:domain:interface:workspace:data.unsavedChanges' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:workspace:data.unsavedChanges' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:workspace:computed.latestWipCheckpointAncestor' })
SET n = { id: 'field:domain:interface:workspace:computed.latestWipCheckpointAncestor', name: 'latestWipCheckpointAncestor', path: 'computed.latestWipCheckpointAncestor', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:workspace:computed.latestWipCheckpointAncestor' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:field { id: 'field:domain:interface:workspace:computed.latestWipCheckpointAncestor' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:workspace:computed.latestWipCheckpointAncestor' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:workspace:computed.kit' })
SET n = { id: 'field:domain:interface:workspace:computed.kit', name: 'kit', path: 'computed.kit', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:workspace:computed.kit' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:field { id: 'field:domain:interface:workspace:computed.kit' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:workspace:computed.kit' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:workspace:reference.checkpoint' })
SET n = { id: 'field:domain:interface:workspace:reference.checkpoint', name: 'checkpoint', path: 'reference.checkpoint', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:workspace:reference.checkpoint' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:workspace' }), (b:field { id: 'field:domain:interface:workspace:reference.checkpoint' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:workspace:reference.checkpoint' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:modification:reference.before' })
SET n = { id: 'field:domain:interface:modification:reference.before', name: 'before', path: 'reference.before', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:modification:reference.before' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:modification' }), (b:field { id: 'field:domain:interface:modification:reference.before' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:modification:reference.before' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:modification:reference.diff' })
SET n = { id: 'field:domain:interface:modification:reference.diff', name: 'diff', path: 'reference.diff', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:modification:reference.diff' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:modification' }), (b:field { id: 'field:domain:interface:modification:reference.diff' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:modification:reference.diff' }), (b:`interface` { id: 'interface:diff' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:modification:reference.after' })
SET n = { id: 'field:domain:interface:modification:reference.after', name: 'after', path: 'reference.after', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:modification:reference.after' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:modification' }), (b:field { id: 'field:domain:interface:modification:reference.after' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:modification:reference.after' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:operation:data.input' })
SET n = { id: 'field:domain:interface:operation:data.input', name: 'input', path: 'data.input', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:operation:data.input' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:operation' }), (b:field { id: 'field:domain:interface:operation:data.input' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:operation:data.input' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:operation:computed.modification' })
SET n = { id: 'field:domain:interface:operation:computed.modification', name: 'modification', path: 'computed.modification', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:operation:computed.modification' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:operation' }), (b:field { id: 'field:domain:interface:operation:computed.modification' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:operation:computed.modification' }), (b:`interface` { id: 'interface:modification' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:operation:reference.scope' })
SET n = { id: 'field:domain:interface:operation:reference.scope', name: 'scope', path: 'reference.scope', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:operation:reference.scope' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:operation' }), (b:field { id: 'field:domain:interface:operation:reference.scope' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:operation:reference.scope' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:backbone:data.uri' })
SET n = { id: 'field:domain:interface:backbone:data.uri', name: 'uri', path: 'data.uri', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:backbone:data.uri' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:backbone' }), (b:field { id: 'field:domain:interface:backbone:data.uri' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:backbone:data.uri' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:backbone:data.status' })
SET n = { id: 'field:domain:interface:backbone:data.status', name: 'status', path: 'data.status', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:backbone:data.status' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:backbone' }), (b:field { id: 'field:domain:interface:backbone:data.status' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:backbone:data.status' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:provider:data.backbones' })
SET n = { id: 'field:domain:interface:provider:data.backbones', name: 'backbones', path: 'data.backbones', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:provider:data.backbones' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:provider' }), (b:field { id: 'field:domain:interface:provider:data.backbones' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:provider:data.backbones' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:interface:provider:computed.backbone' })
SET n = { id: 'field:domain:interface:provider:computed.backbone', name: 'backbone', path: 'computed.backbone', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:interface:provider:computed.backbone' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`interface` { id: 'interface:provider' }), (b:field { id: 'field:domain:interface:provider:computed.backbone' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:interface:provider:computed.backbone' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:command:command:computed.response' })
SET n = { id: 'field:domain:command:command:computed.response', name: 'response', path: 'computed.response', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:command:command:computed.response' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`command` { id: 'command:command' }), (b:field { id: 'field:domain:command:command:computed.response' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:command:command:computed.response' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:vector:x' })
SET n = { id: 'field:domain:class:vector:x', name: 'x', path: 'x', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:vector:x' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:vector' }), (b:field { id: 'field:domain:class:vector:x' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:vector:x' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:vector:y' })
SET n = { id: 'field:domain:class:vector:y', name: 'y', path: 'y', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:vector:y' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:vector' }), (b:field { id: 'field:domain:class:vector:y' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:vector:y' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:vector:z' })
SET n = { id: 'field:domain:class:vector:z', name: 'z', path: 'z', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:vector:z' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:vector' }), (b:field { id: 'field:domain:class:vector:z' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:vector:z' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:point:x' })
SET n = { id: 'field:domain:class:point:x', name: 'x', path: 'x', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:point:x' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:point' }), (b:field { id: 'field:domain:class:point:x' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:point:x' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:point:y' })
SET n = { id: 'field:domain:class:point:y', name: 'y', path: 'y', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:point:y' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:point' }), (b:field { id: 'field:domain:class:point:y' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:point:y' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:point:z' })
SET n = { id: 'field:domain:class:point:z', name: 'z', path: 'z', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:point:z' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:point' }), (b:field { id: 'field:domain:class:point:z' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:point:z' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:coordinate:u' })
SET n = { id: 'field:domain:class:coordinate:u', name: 'u', path: 'u', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:coordinate:u' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:coordinate' }), (b:field { id: 'field:domain:class:coordinate:u' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:coordinate:u' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:coordinate:v' })
SET n = { id: 'field:domain:class:coordinate:v', name: 'v', path: 'v', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:coordinate:v' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:coordinate' }), (b:field { id: 'field:domain:class:coordinate:v' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:coordinate:v' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:offset:x' })
SET n = { id: 'field:domain:class:offset:x', name: 'x', path: 'x', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:offset:x' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:offset' }), (b:field { id: 'field:domain:class:offset:x' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:offset:x' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:offset:y' })
SET n = { id: 'field:domain:class:offset:y', name: 'y', path: 'y', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:offset:y' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:offset' }), (b:field { id: 'field:domain:class:offset:y' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:offset:y' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:plane:origin' })
SET n = { id: 'field:domain:class:plane:origin', name: 'origin', path: 'origin', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:plane:origin' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:plane' }), (b:field { id: 'field:domain:class:plane:origin' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:plane:origin' }), (b:`class` { id: 'class:point' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:plane:xAxis' })
SET n = { id: 'field:domain:class:plane:xAxis', name: 'xAxis', path: 'xAxis', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:plane:xAxis' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:plane' }), (b:field { id: 'field:domain:class:plane:xAxis' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:plane:xAxis' }), (b:`class` { id: 'class:vector' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:plane:yAxis' })
SET n = { id: 'field:domain:class:plane:yAxis', name: 'yAxis', path: 'yAxis', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:plane:yAxis' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:plane' }), (b:field { id: 'field:domain:class:plane:yAxis' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:plane:yAxis' }), (b:`class` { id: 'class:vector' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:plane:zAxis' })
SET n = { id: 'field:domain:class:plane:zAxis', name: 'zAxis', path: 'zAxis', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:plane:zAxis' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:plane' }), (b:field { id: 'field:domain:class:plane:zAxis' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:plane:zAxis' }), (b:`class` { id: 'class:vector' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:position:coordinate' })
SET n = { id: 'field:domain:class:position:coordinate', name: 'coordinate', path: 'coordinate', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:position:coordinate' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:position' }), (b:field { id: 'field:domain:class:position:coordinate' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:position:coordinate' }), (b:`class` { id: 'class:coordinate' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:position:offset' })
SET n = { id: 'field:domain:class:position:offset', name: 'offset', path: 'offset', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:position:offset' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:position' }), (b:field { id: 'field:domain:class:position:offset' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:position:offset' }), (b:`class` { id: 'class:offset' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:position:plane' })
SET n = { id: 'field:domain:class:position:plane', name: 'plane', path: 'plane', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:position:plane' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:position' }), (b:field { id: 'field:domain:class:position:plane' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:position:plane' }), (b:`class` { id: 'class:plane' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:location:position' })
SET n = { id: 'field:domain:class:location:position', name: 'position', path: 'position', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:location:position' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:location' }), (b:field { id: 'field:domain:class:location:position' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:location:position' }), (b:`class` { id: 'class:position' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:location:place' })
SET n = { id: 'field:domain:class:location:place', name: 'place', path: 'place', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:location:place' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:location' }), (b:field { id: 'field:domain:class:location:place' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:location:place' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:attribute:key' })
SET n = { id: 'field:domain:class:attribute:key', name: 'key', path: 'key', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:attribute:key' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:attribute' }), (b:field { id: 'field:domain:class:attribute:key' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:attribute:key' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:attribute:value' })
SET n = { id: 'field:domain:class:attribute:value', name: 'value', path: 'value', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:attribute:value' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:attribute' }), (b:field { id: 'field:domain:class:attribute:value' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:attribute:value' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:place:locations' })
SET n = { id: 'field:domain:class:place:locations', name: 'locations', path: 'locations', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:place:locations' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:place' }), (b:field { id: 'field:domain:class:place:locations' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:place:locations' }), (b:`class` { id: 'class:location' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:family:artifacts' })
SET n = { id: 'field:domain:class:family:artifacts', name: 'artifacts', path: 'artifacts', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:family:artifacts' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:family' }), (b:field { id: 'field:domain:class:family:artifacts' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:family:artifacts' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:folder:children' })
SET n = { id: 'field:domain:class:folder:children', name: 'children', path: 'children', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:folder:children' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:folder' }), (b:field { id: 'field:domain:class:folder:children' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:folder:children' }), (b:`interface` { id: 'interface:artifact' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:file:uri' })
SET n = { id: 'field:domain:class:file:uri', name: 'uri', path: 'uri', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:file:uri' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:file' }), (b:field { id: 'field:domain:class:file:uri' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:file:uri' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:file:mediaType' })
SET n = { id: 'field:domain:class:file:mediaType', name: 'mediaType', path: 'mediaType', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:file:mediaType' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:file' }), (b:field { id: 'field:domain:class:file:mediaType' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:file:mediaType' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:file:size' })
SET n = { id: 'field:domain:class:file:size', name: 'size', path: 'size', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:file:size' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:file' }), (b:field { id: 'field:domain:class:file:size' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:file:size' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:author:email' })
SET n = { id: 'field:domain:class:author:email', name: 'email', path: 'email', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:author:email' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:author' }), (b:field { id: 'field:domain:class:author:email' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:author:email' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:author:url' })
SET n = { id: 'field:domain:class:author:url', name: 'url', path: 'url', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:author:url' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:author' }), (b:field { id: 'field:domain:class:author:url' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:author:url' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:quality:attributes' })
SET n = { id: 'field:domain:class:quality:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:quality:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:quality' }), (b:field { id: 'field:domain:class:quality:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:quality:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:tag:attributes' })
SET n = { id: 'field:domain:class:tag:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:tag:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:tag' }), (b:field { id: 'field:domain:class:tag:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:tag:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:concept:qualities' })
SET n = { id: 'field:domain:class:concept:qualities', name: 'qualities', path: 'qualities', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:concept:qualities' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:concept' }), (b:field { id: 'field:domain:class:concept:qualities' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:concept:qualities' }), (b:`class` { id: 'class:quality' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:concept:tags' })
SET n = { id: 'field:domain:class:concept:tags', name: 'tags', path: 'tags', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:concept:tags' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:concept' }), (b:field { id: 'field:domain:class:concept:tags' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:concept:tags' }), (b:`class` { id: 'class:tag' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:concept:attributes' })
SET n = { id: 'field:domain:class:concept:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:concept:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:concept' }), (b:field { id: 'field:domain:class:concept:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:concept:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:port:concept' })
SET n = { id: 'field:domain:class:port:concept', name: 'concept', path: 'concept', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:port:concept' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:port' }), (b:field { id: 'field:domain:class:port:concept' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:port:concept' }), (b:`class` { id: 'class:concept' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:port:position' })
SET n = { id: 'field:domain:class:port:position', name: 'position', path: 'position', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:port:position' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:port' }), (b:field { id: 'field:domain:class:port:position' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:port:position' }), (b:`class` { id: 'class:position' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:port:qualities' })
SET n = { id: 'field:domain:class:port:qualities', name: 'qualities', path: 'qualities', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:port:qualities' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:port' }), (b:field { id: 'field:domain:class:port:qualities' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:port:qualities' }), (b:`class` { id: 'class:quality' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:port:attributes' })
SET n = { id: 'field:domain:class:port:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:port:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:port' }), (b:field { id: 'field:domain:class:port:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:port:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connector:from' })
SET n = { id: 'field:domain:class:connector:from', name: 'from', path: 'from', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connector:from' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connector' }), (b:field { id: 'field:domain:class:connector:from' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connector:from' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connector:to' })
SET n = { id: 'field:domain:class:connector:to', name: 'to', path: 'to', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connector:to' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connector' }), (b:field { id: 'field:domain:class:connector:to' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connector:to' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connector:qualities' })
SET n = { id: 'field:domain:class:connector:qualities', name: 'qualities', path: 'qualities', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connector:qualities' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connector' }), (b:field { id: 'field:domain:class:connector:qualities' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connector:qualities' }), (b:`class` { id: 'class:quality' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connector:attributes' })
SET n = { id: 'field:domain:class:connector:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connector:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connector' }), (b:field { id: 'field:domain:class:connector:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connector:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:representation:file' })
SET n = { id: 'field:domain:class:representation:file', name: 'file', path: 'file', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:representation:file' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:representation' }), (b:field { id: 'field:domain:class:representation:file' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:representation:file' }), (b:`class` { id: 'class:file' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:representation:attributes' })
SET n = { id: 'field:domain:class:representation:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:representation:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:representation' }), (b:field { id: 'field:domain:class:representation:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:representation:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:type:concepts' })
SET n = { id: 'field:domain:class:type:concepts', name: 'concepts', path: 'concepts', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:type:concepts' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:type' }), (b:field { id: 'field:domain:class:type:concepts' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:type:concepts' }), (b:`class` { id: 'class:concept' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:type:ports' })
SET n = { id: 'field:domain:class:type:ports', name: 'ports', path: 'ports', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:type:ports' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:type' }), (b:field { id: 'field:domain:class:type:ports' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:type:ports' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:type:representations' })
SET n = { id: 'field:domain:class:type:representations', name: 'representations', path: 'representations', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:type:representations' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:type' }), (b:field { id: 'field:domain:class:type:representations' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:type:representations' }), (b:`class` { id: 'class:representation' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:type:attributes' })
SET n = { id: 'field:domain:class:type:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:type:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:type' }), (b:field { id: 'field:domain:class:type:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:type:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:layer:visible' })
SET n = { id: 'field:domain:class:layer:visible', name: 'visible', path: 'visible', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:layer:visible' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:layer' }), (b:field { id: 'field:domain:class:layer:visible' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:layer:visible' }), (b:scalar { id: 'scalar:boolean' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:layer:locked' })
SET n = { id: 'field:domain:class:layer:locked', name: 'locked', path: 'locked', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:layer:locked' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:layer' }), (b:field { id: 'field:domain:class:layer:locked' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:layer:locked' }), (b:scalar { id: 'scalar:boolean' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:piece:type' })
SET n = { id: 'field:domain:class:piece:type', name: 'type', path: 'type', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:piece:type' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:piece' }), (b:field { id: 'field:domain:class:piece:type' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:piece:type' }), (b:`class` { id: 'class:type' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:piece:location' })
SET n = { id: 'field:domain:class:piece:location', name: 'location', path: 'location', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:piece:location' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:piece' }), (b:field { id: 'field:domain:class:piece:location' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:piece:location' }), (b:`class` { id: 'class:location' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:piece:ports' })
SET n = { id: 'field:domain:class:piece:ports', name: 'ports', path: 'ports', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:piece:ports' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:piece' }), (b:field { id: 'field:domain:class:piece:ports' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:piece:ports' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:piece:attributes' })
SET n = { id: 'field:domain:class:piece:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:piece:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:piece' }), (b:field { id: 'field:domain:class:piece:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:piece:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:side:piece' })
SET n = { id: 'field:domain:class:side:piece', name: 'piece', path: 'piece', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:side:piece' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:side' }), (b:field { id: 'field:domain:class:side:piece' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:side:piece' }), (b:`class` { id: 'class:piece' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:side:port' })
SET n = { id: 'field:domain:class:side:port', name: 'port', path: 'port', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:side:port' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:side' }), (b:field { id: 'field:domain:class:side:port' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:side:port' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connection:connector' })
SET n = { id: 'field:domain:class:connection:connector', name: 'connector', path: 'connector', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connection:connector' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connection' }), (b:field { id: 'field:domain:class:connection:connector' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connection:connector' }), (b:`class` { id: 'class:connector' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connection:from' })
SET n = { id: 'field:domain:class:connection:from', name: 'from', path: 'from', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connection:from' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connection' }), (b:field { id: 'field:domain:class:connection:from' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connection:from' }), (b:`class` { id: 'class:side' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connection:to' })
SET n = { id: 'field:domain:class:connection:to', name: 'to', path: 'to', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connection:to' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connection' }), (b:field { id: 'field:domain:class:connection:to' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connection:to' }), (b:`class` { id: 'class:side' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:connection:attributes' })
SET n = { id: 'field:domain:class:connection:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:connection:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:connection' }), (b:field { id: 'field:domain:class:connection:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:connection:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:group:pieces' })
SET n = { id: 'field:domain:class:group:pieces', name: 'pieces', path: 'pieces', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:group:pieces' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:group' }), (b:field { id: 'field:domain:class:group:pieces' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:group:pieces' }), (b:`class` { id: 'class:piece' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:group:connections' })
SET n = { id: 'field:domain:class:group:connections', name: 'connections', path: 'connections', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:group:connections' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:group' }), (b:field { id: 'field:domain:class:group:connections' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:group:connections' }), (b:`class` { id: 'class:connection' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:design:pieces' })
SET n = { id: 'field:domain:class:design:pieces', name: 'pieces', path: 'pieces', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:design:pieces' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:field { id: 'field:domain:class:design:pieces' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:design:pieces' }), (b:`class` { id: 'class:piece' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:design:connections' })
SET n = { id: 'field:domain:class:design:connections', name: 'connections', path: 'connections', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:design:connections' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:field { id: 'field:domain:class:design:connections' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:design:connections' }), (b:`class` { id: 'class:connection' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:design:layers' })
SET n = { id: 'field:domain:class:design:layers', name: 'layers', path: 'layers', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:design:layers' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:field { id: 'field:domain:class:design:layers' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:design:layers' }), (b:`class` { id: 'class:layer' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:design:groups' })
SET n = { id: 'field:domain:class:design:groups', name: 'groups', path: 'groups', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:design:groups' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:field { id: 'field:domain:class:design:groups' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:design:groups' }), (b:`class` { id: 'class:group' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:design:attributes' })
SET n = { id: 'field:domain:class:design:attributes', name: 'attributes', path: 'attributes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:design:attributes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:design' }), (b:field { id: 'field:domain:class:design:attributes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:design:attributes' }), (b:`class` { id: 'class:attribute' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:places' })
SET n = { id: 'field:domain:class:kit:places', name: 'places', path: 'places', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:places' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:places' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:places' }), (b:`class` { id: 'class:place' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:families' })
SET n = { id: 'field:domain:class:kit:families', name: 'families', path: 'families', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:families' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:families' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:families' }), (b:`class` { id: 'class:family' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:folders' })
SET n = { id: 'field:domain:class:kit:folders', name: 'folders', path: 'folders', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:folders' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:folders' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:folders' }), (b:`class` { id: 'class:folder' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:files' })
SET n = { id: 'field:domain:class:kit:files', name: 'files', path: 'files', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:files' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:files' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:files' }), (b:`class` { id: 'class:file' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:authors' })
SET n = { id: 'field:domain:class:kit:authors', name: 'authors', path: 'authors', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:authors' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:authors' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:authors' }), (b:`class` { id: 'class:author' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:props' })
SET n = { id: 'field:domain:class:kit:props', name: 'props', path: 'props', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:props' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:props' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:props' }), (b:`class` { id: 'class:prop' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:benchmarks' })
SET n = { id: 'field:domain:class:kit:benchmarks', name: 'benchmarks', path: 'benchmarks', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:benchmarks' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:benchmarks' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:benchmarks' }), (b:`class` { id: 'class:benchmark' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:qualities' })
SET n = { id: 'field:domain:class:kit:qualities', name: 'qualities', path: 'qualities', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:qualities' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:qualities' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:qualities' }), (b:`class` { id: 'class:quality' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:tags' })
SET n = { id: 'field:domain:class:kit:tags', name: 'tags', path: 'tags', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:tags' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:tags' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:tags' }), (b:`class` { id: 'class:tag' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:concepts' })
SET n = { id: 'field:domain:class:kit:concepts', name: 'concepts', path: 'concepts', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:concepts' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:concepts' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:concepts' }), (b:`class` { id: 'class:concept' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:ports' })
SET n = { id: 'field:domain:class:kit:ports', name: 'ports', path: 'ports', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:ports' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:ports' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:ports' }), (b:`class` { id: 'class:port' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:connectors' })
SET n = { id: 'field:domain:class:kit:connectors', name: 'connectors', path: 'connectors', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:connectors' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:connectors' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:connectors' }), (b:`class` { id: 'class:connector' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:representations' })
SET n = { id: 'field:domain:class:kit:representations', name: 'representations', path: 'representations', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:representations' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:representations' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:representations' }), (b:`class` { id: 'class:representation' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:types' })
SET n = { id: 'field:domain:class:kit:types', name: 'types', path: 'types', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:types' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:types' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:types' }), (b:`class` { id: 'class:type' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:layers' })
SET n = { id: 'field:domain:class:kit:layers', name: 'layers', path: 'layers', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:layers' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:layers' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:layers' }), (b:`class` { id: 'class:layer' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:pieces' })
SET n = { id: 'field:domain:class:kit:pieces', name: 'pieces', path: 'pieces', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:pieces' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:pieces' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:pieces' }), (b:`class` { id: 'class:piece' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:connections' })
SET n = { id: 'field:domain:class:kit:connections', name: 'connections', path: 'connections', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:connections' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:connections' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:connections' }), (b:`class` { id: 'class:connection' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:groups' })
SET n = { id: 'field:domain:class:kit:groups', name: 'groups', path: 'groups', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:groups' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:groups' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:groups' }), (b:`class` { id: 'class:group' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:kit:designs' })
SET n = { id: 'field:domain:class:kit:designs', name: 'designs', path: 'designs', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:kit:designs' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:kit' }), (b:field { id: 'field:domain:class:kit:designs' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:kit:designs' }), (b:`class` { id: 'class:design' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:forwards' })
SET n = { id: 'field:domain:class:edit:forwards', name: 'forwards', path: 'forwards', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:forwards' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:forwards' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:forwards' }), (b:`interface` { id: 'interface:operation' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:backwards' })
SET n = { id: 'field:domain:class:edit:backwards', name: 'backwards', path: 'backwards', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:backwards' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:backwards' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:backwards' }), (b:`interface` { id: 'interface:operation' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:sequenceNumber' })
SET n = { id: 'field:domain:class:edit:sequenceNumber', name: 'sequenceNumber', path: 'sequenceNumber', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:sequenceNumber' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:sequenceNumber' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:sequenceNumber' }), (b:scalar { id: 'scalar:number' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:startedAt' })
SET n = { id: 'field:domain:class:edit:startedAt', name: 'startedAt', path: 'startedAt', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:startedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:startedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:startedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:finishedAt' })
SET n = { id: 'field:domain:class:edit:finishedAt', name: 'finishedAt', path: 'finishedAt', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:finishedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:finishedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:finishedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:finished' })
SET n = { id: 'field:domain:class:edit:finished', name: 'finished', path: 'finished', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:finished' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:finished' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:finished' }), (b:scalar { id: 'scalar:boolean' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:description' })
SET n = { id: 'field:domain:class:edit:description', name: 'description', path: 'description', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:description' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:description' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:description' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:edit:origin' })
SET n = { id: 'field:domain:class:edit:origin', name: 'origin', path: 'origin', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:edit:origin' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:edit' }), (b:field { id: 'field:domain:class:edit:origin' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:edit:origin' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:edits' })
SET n = { id: 'field:domain:class:change:edits', name: 'edits', path: 'edits', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:edits' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:edits' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:edits' }), (b:`class` { id: 'class:edit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:startedAt' })
SET n = { id: 'field:domain:class:change:startedAt', name: 'startedAt', path: 'startedAt', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:startedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:startedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:startedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:savedAt' })
SET n = { id: 'field:domain:class:change:savedAt', name: 'savedAt', path: 'savedAt', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:savedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:savedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:savedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:saved' })
SET n = { id: 'field:domain:class:change:saved', name: 'saved', path: 'saved', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:saved' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:saved' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:saved' }), (b:scalar { id: 'scalar:boolean' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:description' })
SET n = { id: 'field:domain:class:change:description', name: 'description', path: 'description', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:description' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:description' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:description' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:change:origin' })
SET n = { id: 'field:domain:class:change:origin', name: 'origin', path: 'origin', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:change:origin' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:change' }), (b:field { id: 'field:domain:class:change:origin' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:change:origin' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:timestamp' })
SET n = { id: 'field:domain:class:checkpoint:timestamp', name: 'timestamp', path: 'timestamp', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:timestamp' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:timestamp' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:timestamp' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:message' })
SET n = { id: 'field:domain:class:checkpoint:message', name: 'message', path: 'message', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:message' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:message' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:message' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:authors' })
SET n = { id: 'field:domain:class:checkpoint:authors', name: 'authors', path: 'authors', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:authors' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:authors' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:authors' }), (b:`class` { id: 'class:author' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:parent' })
SET n = { id: 'field:domain:class:checkpoint:parent', name: 'parent', path: 'parent', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:parent' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:parent' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:parent' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:ancestors' })
SET n = { id: 'field:domain:class:checkpoint:ancestors', name: 'ancestors', path: 'ancestors', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:ancestors' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:ancestors' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:ancestors' }), (b:`interface` { id: 'interface:entity' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:initial' })
SET n = { id: 'field:domain:class:checkpoint:initial', name: 'initial', path: 'initial', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:initial' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:initial' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:initial' }), (b:`class` { id: 'class:kit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:kit' })
SET n = { id: 'field:domain:class:checkpoint:kit', name: 'kit', path: 'kit', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:kit' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:kit' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:kit' }), (b:`class` { id: 'class:kit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:changes' })
SET n = { id: 'field:domain:class:checkpoint:changes', name: 'changes', path: 'changes', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:changes' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:changes' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:changes' }), (b:`class` { id: 'class:change' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:checkpoint:edits' })
SET n = { id: 'field:domain:class:checkpoint:edits', name: 'edits', path: 'edits', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:checkpoint:edits' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:checkpoint' }), (b:field { id: 'field:domain:class:checkpoint:edits' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:checkpoint:edits' }), (b:`class` { id: 'class:edit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:alternative:name' })
SET n = { id: 'field:domain:class:alternative:name', name: 'name', path: 'name', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:alternative:name' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:alternative' }), (b:field { id: 'field:domain:class:alternative:name' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:alternative:name' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:graph:initialKit' })
SET n = { id: 'field:domain:class:graph:initialKit', name: 'initialKit', path: 'initialKit', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:graph:initialKit' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:field { id: 'field:domain:class:graph:initialKit' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:graph:initialKit' }), (b:`class` { id: 'class:kit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:graph:theKit' })
SET n = { id: 'field:domain:class:graph:theKit', name: 'theKit', path: 'theKit', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:graph:theKit' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:field { id: 'field:domain:class:graph:theKit' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:graph:theKit' }), (b:`class` { id: 'class:theKit' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:graph:alternatives' })
SET n = { id: 'field:domain:class:graph:alternatives', name: 'alternatives', path: 'alternatives', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:graph:alternatives' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:field { id: 'field:domain:class:graph:alternatives' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:graph:alternatives' }), (b:`class` { id: 'class:alternative' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:graph:checkpoints' })
SET n = { id: 'field:domain:class:graph:checkpoints', name: 'checkpoints', path: 'checkpoints', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:graph:checkpoints' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:field { id: 'field:domain:class:graph:checkpoints' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:graph:checkpoints' }), (b:`class` { id: 'class:checkpoint' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:graph:releases' })
SET n = { id: 'field:domain:class:graph:releases', name: 'releases', path: 'releases', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:graph:releases' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:graph' }), (b:field { id: 'field:domain:class:graph:releases' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:graph:releases' }), (b:`class` { id: 'class:checkpoint' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:conflict:authoritativeChange' })
SET n = { id: 'field:domain:class:conflict:authoritativeChange', name: 'authoritativeChange', path: 'authoritativeChange', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:conflict:authoritativeChange' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:conflict' }), (b:field { id: 'field:domain:class:conflict:authoritativeChange' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:conflict:authoritativeChange' }), (b:`class` { id: 'class:change' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:conflict:wipChange' })
SET n = { id: 'field:domain:class:conflict:wipChange', name: 'wipChange', path: 'wipChange', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:conflict:wipChange' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:conflict' }), (b:field { id: 'field:domain:class:conflict:wipChange' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:conflict:wipChange' }), (b:`class` { id: 'class:change' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:conflict:reasons' })
SET n = { id: 'field:domain:class:conflict:reasons', name: 'reasons', path: 'reasons', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:conflict:reasons' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:conflict' }), (b:field { id: 'field:domain:class:conflict:reasons' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:conflict:reasons' }), (b:scalar { id: 'scalar:string' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:store:wip' })
SET n = { id: 'field:domain:class:store:wip', name: 'wip', path: 'wip', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:store:wip' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:field { id: 'field:domain:class:store:wip' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:store:wip' }), (b:`class` { id: 'class:graph' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:store:stage' })
SET n = { id: 'field:domain:class:store:stage', name: 'stage', path: 'stage', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:store:stage' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:field { id: 'field:domain:class:store:stage' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:store:stage' }), (b:`class` { id: 'class:graph' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:store:authoritative' })
SET n = { id: 'field:domain:class:store:authoritative', name: 'authoritative', path: 'authoritative', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:store:authoritative' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:field { id: 'field:domain:class:store:authoritative' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:store:authoritative' }), (b:`class` { id: 'class:graph' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:store:conflicts' })
SET n = { id: 'field:domain:class:store:conflicts', name: 'conflicts', path: 'conflicts', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:store:conflicts' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:field { id: 'field:domain:class:store:conflicts' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:store:conflicts' }), (b:`class` { id: 'class:conflict' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:store:backbone' })
SET n = { id: 'field:domain:class:store:backbone', name: 'backbone', path: 'backbone', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:store:backbone' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:store' }), (b:field { id: 'field:domain:class:store:backbone' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:store:backbone' }), (b:`interface` { id: 'interface:backbone' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:localProvider:uri' })
SET n = { id: 'field:domain:class:localProvider:uri', name: 'uri', path: 'uri', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:localProvider:uri' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:localProvider' }), (b:field { id: 'field:domain:class:localProvider:uri' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:localProvider:uri' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:localProvider:stores' })
SET n = { id: 'field:domain:class:localProvider:stores', name: 'stores', path: 'stores', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:localProvider:stores' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:localProvider' }), (b:field { id: 'field:domain:class:localProvider:stores' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:localProvider:stores' }), (b:`class` { id: 'class:store' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:remoteProvider:uri' })
SET n = { id: 'field:domain:class:remoteProvider:uri', name: 'uri', path: 'uri', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:remoteProvider:uri' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:remoteProvider' }), (b:field { id: 'field:domain:class:remoteProvider:uri' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:remoteProvider:uri' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:remoteProvider:url' })
SET n = { id: 'field:domain:class:remoteProvider:url', name: 'url', path: 'url', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:remoteProvider:url' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:remoteProvider' }), (b:field { id: 'field:domain:class:remoteProvider:url' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:remoteProvider:url' }), (b:scalar { id: 'scalar:uri' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:session:stores' })
SET n = { id: 'field:domain:class:session:stores', name: 'stores', path: 'stores', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:session:stores' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:session' }), (b:field { id: 'field:domain:class:session:stores' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:session:stores' }), (b:`class` { id: 'class:store' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:session:localProvider' })
SET n = { id: 'field:domain:class:session:localProvider', name: 'localProvider', path: 'localProvider', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:session:localProvider' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:session' }), (b:field { id: 'field:domain:class:session:localProvider' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:session:localProvider' }), (b:`class` { id: 'class:localProvider' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:session:remoteProviders' })
SET n = { id: 'field:domain:class:session:remoteProviders', name: 'remoteProviders', path: 'remoteProviders', list: true };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:session:remoteProviders' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:session' }), (b:field { id: 'field:domain:class:session:remoteProviders' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:session:remoteProviders' }), (b:`class` { id: 'class:remoteProvider' })
MERGE (a)-[:REFERENCES]->(b);

MERGE (n:field { id: 'field:domain:class:session:startedAt' })
SET n = { id: 'field:domain:class:session:startedAt', name: 'startedAt', path: 'startedAt', list: false };

MATCH (a:module { id: 'module:domain' }), (b:field { id: 'field:domain:class:session:startedAt' })
MERGE (a)-[:OWNS]->(b);

MATCH (a:`class` { id: 'class:session' }), (b:field { id: 'field:domain:class:session:startedAt' })
MERGE (a)-[:HAS_FIELD]->(b);

MATCH (a:field { id: 'field:domain:class:session:startedAt' }), (b:scalar { id: 'scalar:timestamp' })
MERGE (a)-[:REFERENCES]->(b);

// Bloom / Explore: indexes on names and ids
CREATE INDEX bloom_field_path IF NOT EXISTS FOR (n:field) ON (n.path);
CREATE INDEX bloom_field_name IF NOT EXISTS FOR (n:field) ON (n.name);
CREATE INDEX bloom_class_name IF NOT EXISTS FOR (n:`class`) ON (n.name);
CREATE INDEX bloom_interface_name IF NOT EXISTS FOR (n:`interface`) ON (n.name);
CREATE INDEX bloom_scalar_name IF NOT EXISTS FOR (n:scalar) ON (n.name);
CREATE INDEX bloom_command_name IF NOT EXISTS FOR (n:`command`) ON (n.name);

