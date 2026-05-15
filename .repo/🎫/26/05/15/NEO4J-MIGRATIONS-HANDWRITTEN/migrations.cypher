// SPDX-License-Identifier: AGPL-3.0-only
// Hand-maintained Neo4j migrations for the semio graph. Run via migrate.neo4j.script.ts in this folder (not chained from generate).
// Containment edges use relationship type OWNS (never HAS).

//#region RelabelHasRelationshipsToOwns
MATCH (a)-[r:HAS]->(b)
CREATE (a)-[r2:OWNS]->(b)
SET r2 += properties(r)
DELETE r;
//#endregion RelabelHasRelationshipsToOwns

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

//#region ReclassifyChangeSavedKitMemberAsComputation
MATCH (:Class {name: 'Change'})-[:OWNS]->(d:Data {name: 'saved'})
SET d:Computation, d.cached = false
REMOVE d:Data, d.isList;
MATCH (:Class {name: 'Change'})-[:OWNS]->(c:Computation {name: 'saved'})
MERGE (con:Constraint {description: '📌 saved derives from embedded sibling savedAt: non-null persisted timestamp implies the boolean reads true.'})
MERGE (c)-[:OWNS]->(con);
//#endregion ReclassifyChangeSavedKitMemberAsComputation

//#region ReplaceFieldIndexes
DROP INDEX index_field_name IF EXISTS;
CREATE RANGE INDEX index_data_name IF NOT EXISTS FOR (n:Data) ON (n.name);
CREATE RANGE INDEX index_computation_name IF NOT EXISTS FOR (n:Computation) ON (n.name);
CREATE RANGE INDEX index_reference_name IF NOT EXISTS FOR (n:Reference) ON (n.name);
DROP INDEX semio_name_fulltext IF EXISTS;
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Constraint|Data|Computation|Reference|Interface|Module|Scalar) ON EACH [n.name];
//#endregion ReplaceFieldIndexes

//#region FoldDupGeneralUnderDomain
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(genCanon:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[:OWNS]->(genDup:Module {name: 'General'})
WHERE id(genCanon) <> id(genDup)
MATCH (genDup)-[r:OWNS]->(ch)
MERGE (genCanon)-[:OWNS]->(ch)
DELETE r;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(genCanon:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[rx:OWNS]->(genDup:Module {name: 'General'})
WHERE id(genCanon) <> id(genDup)
DELETE rx
DETACH DELETE genDup;
//#endregion FoldDupGeneralUnderDomain

//#region HoistAllDomainChildrenToSchemaGeneral
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[r:OWNS]->(ch)
MERGE (gen)-[:OWNS]->(ch)
DELETE r;
//#endregion HoistAllDomainChildrenToSchemaGeneral

//#region EntityInterfaceSubmoduleMirror
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MERGE (gen)-[:OWNS]->(ent:Module {name: 'Entity'});
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
OPTIONAL MATCH (gen)-[gw:OWNS]->(:Module {name: 'WeakEntity'})
DELETE gw;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
OPTIONAL MATCH (gen)-[gs:OWNS]->(:Module {name: 'StrongEntity'})
DELETE gs;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (gen)-[:OWNS]->(ent:Module {name: 'Entity'})
MERGE (wmod:Module {name: 'WeakEntity'})
MERGE (smod:Module {name: 'StrongEntity'})
MERGE (rmod:Module {name: 'RichStrongEntity'})
MERGE (amod:Module {name: 'Artifact'})
MERGE (dmod:Module {name: 'Document'})
MERGE (dataMod:Module {name: 'Data'})
MERGE (eventMod:Module {name: 'Event'})
MERGE (ent)-[:OWNS]->(wmod)
MERGE (ent)-[:OWNS]->(smod)
MERGE (smod)-[:OWNS]->(rmod)
MERGE (rmod)-[:OWNS]->(amod)
MERGE (amod)-[:OWNS]->(dmod)
MERGE (wmod)-[:OWNS]->(dataMod)
MERGE (wmod)-[:OWNS]->(eventMod);

MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (gen)-[:OWNS]->(ent:Module {name: 'Entity'})
MATCH (ent)-[:OWNS]->(wmod:Module {name: 'WeakEntity'})
MATCH (i:Interface {name: 'WeakEntity'})
MERGE (wmod)-[:OWNS]->(i);

MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (gen)-[:OWNS]->(ent:Module {name: 'Entity'})
MATCH (ent)-[:OWNS]->(smod:Module {name: 'StrongEntity'})
MATCH (i:Interface {name: 'StrongEntity'})
MERGE (smod)-[:OWNS]->(i);

MATCH (s:Module {name: 'StrongEntity'})-[:OWNS]->(rmod:Module {name: 'RichStrongEntity'})
MATCH (i:Interface {name: 'RichStrongEntity'})
MERGE (rmod)-[:OWNS]->(i);

MATCH (r:Module {name: 'RichStrongEntity'})-[:OWNS]->(amod:Module {name: 'Artifact'})
MATCH (i:Interface {name: 'Artifact'})
MERGE (amod)-[:OWNS]->(i);

MATCH (a:Module {name: 'Artifact'})-[:OWNS]->(dmod:Module {name: 'Document'})
MATCH (i:Interface {name: 'Document'})
MERGE (dmod)-[:OWNS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:OWNS]->(dataMod:Module {name: 'Data'})
MATCH (i:Interface {name: 'Data'})
MERGE (dataMod)-[:OWNS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:OWNS]->(eventMod:Module {name: 'Event'})
MATCH (i:Interface {name: 'Event'})
MERGE (eventMod)-[:OWNS]->(i);
//#endregion EntityInterfaceSubmoduleMirror

//#region DeduplicateEntityLadderModulesByName
MATCH (m:Module {name: 'WeakEntity'})
WITH m ORDER BY id(m) ASC LIMIT 1
MATCH (dup:Module {name: 'WeakEntity'})
WHERE id(dup) <> id(m)
DETACH DELETE dup;
MATCH (m:Module {name: 'StrongEntity'})
WITH m ORDER BY id(m) ASC LIMIT 1
MATCH (dup:Module {name: 'StrongEntity'})
WHERE id(dup) <> id(m)
DETACH DELETE dup;
MATCH (m:Module {name: 'RichStrongEntity'})
WITH m ORDER BY id(m) ASC LIMIT 1
MATCH (dup:Module {name: 'RichStrongEntity'})
WHERE id(dup) <> id(m)
DETACH DELETE dup;
MATCH (m:Module {name: 'Artifact'})
WITH m ORDER BY id(m) ASC LIMIT 1
MATCH (dup:Module {name: 'Artifact'})
WHERE id(dup) <> id(m)
DETACH DELETE dup;
MATCH (m:Module {name: 'Document'})
WITH m ORDER BY id(m) ASC LIMIT 1
MATCH (dup:Module {name: 'Document'})
WHERE id(dup) <> id(m)
DETACH DELETE dup;
//#endregion DeduplicateEntityLadderModulesByName

//#region RemoveEmptyKitSchemaStub
MATCH (m:Module {name: 'ScopedCommand'})
DETACH DELETE m;
MATCH (m:Module {name: 'KitSchema'})
DETACH DELETE m;
//#endregion RemoveEmptyKitSchemaStub

//#region RenameKitEntityModuleToKit
MATCH (parent:Module {name: 'KitEntity'})-[:OWNS]->(inner:Module {name: 'Kit'})
MATCH (inner)-[r:OWNS]->(c:Class {name: 'Kit'})
MERGE (parent)-[:OWNS]->(c)
DELETE r;
MATCH (inner:Module {name: 'Kit'})<-[:OWNS]-(parent:Module {name: 'KitEntity'})
DETACH DELETE inner;
MATCH (m:Module {name: 'KitEntity'})
SET m.name = 'Kit';
//#endregion RenameKitEntityModuleToKit

//#region KitModuleUnderDomain
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(dom:Module {name: 'Domain'})
MATCH (schema)-[:OWNS]->(gen:Module {name: 'General'})
MATCH (kit:Module {name: 'Kit'})
WHERE (gen)-[:OWNS]->(kit)
MATCH (gen)-[rk:OWNS]->(kit)
MERGE (dom)-[:OWNS]->(kit)
DELETE rk;
//#endregion KitModuleUnderDomain

//#region VcsModuleUnderDomain
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(dom:Module {name: 'Domain'})
MATCH (schema)-[:OWNS]->(gen:Module {name: 'General'})
MATCH (vcs:Module {name: 'VCS'})
WHERE (gen)-[:OWNS]->(vcs)
MATCH (gen)-[rv:OWNS]->(vcs)
MERGE (dom)-[:OWNS]->(vcs)
DELETE rv;
//#endregion VcsModuleUnderDomain

//#region ScalarModuleUnderSchemaGeneral
OPTIONAL MATCH (d:Module {name: 'Domain'})-[r:OWNS]->(sm:Module {name: 'Scalar'})
DELETE r;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (sm:Module {name: 'Scalar'})
MERGE (gen)-[:OWNS]->(sm);
//#endregion ScalarModuleUnderSchemaGeneral

//#region RemoveFieldKindMetaModule
MATCH (fk:Module {name: 'FieldKind'})-[:OWNS]->(e:Enum)
DETACH DELETE e;
MATCH (fk:Module {name: 'FieldKind'})
DETACH DELETE fk;
//#endregion RemoveFieldKindMetaModule
