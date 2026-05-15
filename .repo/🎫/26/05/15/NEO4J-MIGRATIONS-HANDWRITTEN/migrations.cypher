// SPDX-License-Identifier: AGPL-3.0-only
// Hand-maintained Neo4j migrations for the semio graph. Run via migrate.neo4j.script.ts in this folder (not chained from generate).

//#region RelabelFieldNodes
MATCH (f:Field {kind: 'EMBEDDED'})
SET f:Data
REMOVE f:Field, f.kind;

MATCH (f:Field)
WHERE f.kind IN ['COMPUTED', 'CACHED']
WITH f, f.kind AS k
SET f:Computation, f.cached = (k = 'CACHED')
REMOVE f:Field, f.kind;

MATCH (f:Field {kind: 'REFERENCE'})
SET f:Reference
REMOVE f:Field, f.kind;
//#endregion RelabelFieldNodes

//#region MaterializeTransitiveIsForKitMembers
MATCH (n:Data|Computation|Reference)-[:IS]->(i:Interface)
MATCH (i)-[:IS*1..25]->(b:Interface)
WHERE n <> b
MERGE (n)-[:IS]->(b);
MATCH (n:Data|Computation|Reference)-[:IS]->(c:Class)
MATCH (c)-[:IS*1..25]->(b:Interface)
MERGE (n)-[:IS]->(b);
//#endregion MaterializeTransitiveIsForKitMembers

//#region ReplaceFieldIndexes
DROP INDEX index_field_name IF EXISTS;
CREATE RANGE INDEX index_data_name IF NOT EXISTS FOR (n:Data) ON (n.name);
CREATE RANGE INDEX index_computation_name IF NOT EXISTS FOR (n:Computation) ON (n.name);
CREATE RANGE INDEX index_reference_name IF NOT EXISTS FOR (n:Reference) ON (n.name);
DROP INDEX semio_name_fulltext IF EXISTS;
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Constraint|Data|Computation|Reference|Interface|Module|Scalar) ON EACH [n.name];
//#endregion ReplaceFieldIndexes

//#region StripEntityModuleInterfaceEdges
MATCH (legacy:Module {name: 'Entity'})-[r:HAS]->(i:Interface)
WHERE i.name IN ['WeakEntity', 'StrongEntity', 'RichStrongEntity', 'Artifact', 'Document', 'Data', 'Event']
DELETE r;
//#endregion StripEntityModuleInterfaceEdges

//#region HoistEntityModuleChildrenToDomain
MATCH (dom:Module {name: 'Domain'})
MATCH (ent:Module {name: 'Entity'})
MATCH (ent)-[r:HAS]->(ch)
MERGE (dom)-[:HAS]->(ch)
DELETE r;
//#endregion HoistEntityModuleChildrenToDomain

//#region RemoveEntityModuleNode
MATCH (dom:Module {name: 'Domain'})-[r:HAS]->(ent:Module {name: 'Entity'})
DELETE r;
MATCH (ent:Module {name: 'Entity'})
DETACH DELETE ent;
//#endregion RemoveEntityModuleNode

//#region DomainInterfaceSubmoduleMirror
MATCH (dom:Module {name: 'Domain'})
MERGE (dom)-[:HAS]->(wmod:Module {name: 'WeakEntity'})
MERGE (dom)-[:HAS]->(smod:Module {name: 'StrongEntity'})
MERGE (smod)-[:HAS]->(rmod:Module {name: 'RichStrongEntity'})
MERGE (rmod)-[:HAS]->(amod:Module {name: 'Artifact'})
MERGE (amod)-[:HAS]->(dmod:Module {name: 'Document'})
MERGE (wmod)-[:HAS]->(dataMod:Module {name: 'Data'})
MERGE (wmod)-[:HAS]->(eventMod:Module {name: 'Event'});

MATCH (dom:Module {name: 'Domain'})-[:HAS]->(wmod:Module {name: 'WeakEntity'})
MATCH (i:Interface {name: 'WeakEntity'})
MERGE (wmod)-[:HAS]->(i);

MATCH (dom:Module {name: 'Domain'})-[:HAS]->(smod:Module {name: 'StrongEntity'})
MATCH (i:Interface {name: 'StrongEntity'})
MERGE (smod)-[:HAS]->(i);

MATCH (s:Module {name: 'StrongEntity'})-[:HAS]->(rmod:Module {name: 'RichStrongEntity'})
MATCH (i:Interface {name: 'RichStrongEntity'})
MERGE (rmod)-[:HAS]->(i);

MATCH (r:Module {name: 'RichStrongEntity'})-[:HAS]->(amod:Module {name: 'Artifact'})
MATCH (i:Interface {name: 'Artifact'})
MERGE (amod)-[:HAS]->(i);

MATCH (a:Module {name: 'Artifact'})-[:HAS]->(dmod:Module {name: 'Document'})
MATCH (i:Interface {name: 'Document'})
MERGE (dmod)-[:HAS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:HAS]->(dataMod:Module {name: 'Data'})
MATCH (i:Interface {name: 'Data'})
MERGE (dataMod)-[:HAS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:HAS]->(eventMod:Module {name: 'Event'})
MATCH (i:Interface {name: 'Event'})
MERGE (eventMod)-[:HAS]->(i);
//#endregion DomainInterfaceSubmoduleMirror

//#region RemoveEmptyKitSchemaStub
MATCH (m:Module {name: 'ScopedCommand'})
DETACH DELETE m;
MATCH (m:Module {name: 'KitSchema'})
DETACH DELETE m;
//#endregion RemoveEmptyKitSchemaStub

//#region RenameKitEntityModuleToKit
MATCH (parent:Module {name: 'KitEntity'})-[:HAS]->(inner:Module {name: 'Kit'})
MATCH (inner)-[r:HAS]->(c:Class {name: 'Kit'})
MERGE (parent)-[:HAS]->(c)
DELETE r;
MATCH (inner:Module {name: 'Kit'})<-[:HAS]-(parent:Module {name: 'KitEntity'})
DETACH DELETE inner;
MATCH (m:Module {name: 'KitEntity'})
SET m.name = 'Kit';
//#endregion RenameKitEntityModuleToKit

//#region ScalarModuleUnderSchemaGeneral
OPTIONAL MATCH (d:Module {name: 'Domain'})-[r:HAS]->(sm:Module {name: 'Scalar'})
DELETE r;
MATCH (schema:Module {name: 'Schema'})-[:HAS]->(gen:Module {name: 'General'})
MATCH (sm:Module {name: 'Scalar'})
MERGE (gen)-[:HAS]->(sm);
//#endregion ScalarModuleUnderSchemaGeneral

//#region RemoveFieldKindMetaModule
MATCH (fk:Module {name: 'FieldKind'})-[:HAS]->(e:Enum)
DETACH DELETE e;
MATCH (fk:Module {name: 'FieldKind'})
DETACH DELETE fk;
//#endregion RemoveFieldKindMetaModule
