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

//#region StripLegacySemioGraphNodeProperties
// Persist only graph-native kit metadata: `rank` is sibling order (string); `isList` flags list-shaped members; `cached` only on Computation.
MATCH (n:Data)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false) };
MATCH (n:Reference)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false) };
MATCH (n:Computation)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false), cached: coalesce(n.cached, false) };
MATCH (n:Class|Interface|Scalar|Module)
SET n = { name: n.name };
MATCH (n:Constraint)
WITH n,
  CASE
    WHEN coalesce(n.description, '') <> '' THEN n.description
    WHEN coalesce(toString(n.name), '') <> '' THEN toString(n.name)
    ELSE 'constraint'
  END AS d
SET n = { description: d };
//#endregion StripLegacySemioGraphNodeProperties

//#region RemoveResidualKindProperty
MATCH (n)
WHERE n.kind IS NOT NULL
REMOVE n.kind;
//#endregion RemoveResidualKindProperty

//#region ReparentProviderBackboneKitMembers
// `backbone` / `backbones` are defined on `Provider` in schema.yaml; concrete backbones (`FileBackbone`, `WebsocketBackbone` classes) still `IS` `Backbone`. Subclasses must not OWNS those shared kit members.
MATCH (provider:Interface|Class)
WHERE toLower(provider.name) = 'provider'
WITH provider ORDER BY id(provider) ASC LIMIT 1
MATCH (sub:Class)
WHERE sub.name IN ['LocalProvider', 'RemoteProvider']
MATCH (sub)-[rw:OWNS]->(f:Data|Computation|Reference)
WHERE f.name IN ['backbone', 'backbones']
MERGE (provider)-[:OWNS]->(f)
DELETE rw;
//#endregion ReparentProviderBackboneKitMembers

//#region ReparentTypeConnectorKitMembersFromOperation
// `connector` / `connectors` belong on `Type` (see schema.yaml + GraphQL `Type.connector(s)`), not on `Operation`.
MATCH (t:Class|Interface)
WHERE toLower(t.name) = 'type'
WITH t ORDER BY id(t) ASC LIMIT 1
MATCH (op:Class|Interface)
WHERE toLower(op.name) = 'operation'
MATCH (op)-[rw:OWNS]->(f:Data|Computation|Reference)
WHERE f.name IN ['connector', 'connectors']
MERGE (t)-[:OWNS]->(f)
DELETE rw;
//#endregion ReparentTypeConnectorKitMembersFromOperation

//#region EnsureOperationsSubmoduleUnderVcs
MERGE (vcs:Module {name: 'VCS'})
MERGE (ops:Module {name: 'Operations'})
MERGE (vcs)-[:OWNS]->(ops);
//#endregion EnsureOperationsSubmoduleUnderVcs

//#region ReparentModuleCommandChildrenToOperations
MATCH (cmdm:Module {name: 'Command'})
OPTIONAL MATCH (cmdm)-[r:OWNS]->(ch)
WITH cmdm, r, ch
WHERE ch IS NOT NULL
MERGE (ops:Module {name: 'Operations'})
MERGE (ops)-[:OWNS]->(ch)
DELETE r;
MATCH (cmdm:Module {name: 'Command'})
DETACH DELETE cmdm;
//#endregion ReparentModuleCommandChildrenToOperations

//#region MergeOperationConcreteClasses
UNWIND ["AddedAttributesToConcept", "AddedAttributesToDesign", "AddedAttributesToPiece", "AddedAttributesToPort", "AddedAttributesToQuality", "AddedAttributesToTag", "AddedAttributesToType", "AddedAttributeToConcept", "AddedAttributeToDesign", "AddedAttributeToPiece", "AddedAttributeToPort", "AddedAttributeToQuality", "AddedAttributeToTag", "AddedAttributeToType", "AddedChildPiecesWithParentConnections", "AddedChildPieceWithParentConnection", "AddedConnector", "AddedConnectors", "AddedHangingChildPiecesWithParentConnections", "AddedHangingChildPieceWithParentConnection", "ChangedDescription", "ChangedPiecesToType", "ChangedPieceToType", "CreatedConcept", "CreatedConcepts", "CreatedDesign", "CreatedDesigns", "CreatedFixedPiece", "CreatedPort", "CreatedPorts", "CreatedQualities", "CreatedQuality", "CreatedTag", "CreatedTags", "CreatedType", "CreatedTypes", "DeletedConcept", "DeletedConcepts", "DeletedDesign", "DeletedDesigns", "DeletedPiece", "DeletedPieces", "DeletedPiecesAndConnections", "DeletedPort", "DeletedPorts", "DeletedQualities", "DeletedQuality", "DeletedTag", "DeletedTags", "DeletedType", "DeletedTypes", "DraggedPiece", "DraggedPieces", "FixedPiece", "FixedPieces", "FlattenedDesign", "MovedPiece", "MovedPieces", "RemovedAttributeFromConcept", "RemovedAttributeFromDesign", "RemovedAttributeFromPiece", "RemovedAttributeFromPort", "RemovedAttributeFromQuality", "RemovedAttributeFromTag", "RemovedAttributeFromType", "RemovedAttributesFromConcept", "RemovedAttributesFromDesign", "RemovedAttributesFromPiece", "RemovedAttributesFromPort", "RemovedAttributesFromQuality", "RemovedAttributesFromTag", "RemovedAttributesFromType", "RemovedConnector", "RemovedConnectors", "RenamedConcept", "RenamedConnector", "RenamedKit", "RenamedPiece", "RenamedPort", "RenamedQuality", "RenamedTag", "RenamedType", "UpdatedConceptDescription", "UpdatedConceptIcon", "UpdatedConnectorDescription", "UpdatedConnectorIcon", "UpdatedPieceDescription", "UpdatedPortDescription", "UpdatedPortIcon", "UpdatedQualityDescription", "UpdatedQualityIcon", "UpdatedTagDescription", "UpdatedTagIcon", "UpdatedTypeDescription", "UpdatedTypeIcon"] AS opName
MERGE (c:Class {name: opName})
WITH c
MATCH (op:Interface|Class)
WHERE toLower(op.name) = 'operation'
WITH c, op
ORDER BY id(op) ASC
LIMIT 1
MERGE (c)-[:IS]->(op);
//#endregion MergeOperationConcreteClasses

//#region ReparentOperationClassesUnderOwnerOperationModules
// Each domain `Class` / `Interface` (Piece, Quality, …) OWNS `Module(operation)` which OWNS concrete operation classes (golden `Operation` subtypes).
UNWIND [{op: 'AddedAttributesToConcept', own: 'Concept'}, {op: 'AddedAttributesToDesign', own: 'Design'}, {op: 'AddedAttributesToPiece', own: 'Piece'}, {op: 'AddedAttributesToPort', own: 'Port'}, {op: 'AddedAttributesToQuality', own: 'Quality'}, {op: 'AddedAttributesToTag', own: 'Tag'}, {op: 'AddedAttributesToType', own: 'Type'}, {op: 'AddedAttributeToConcept', own: 'Concept'}, {op: 'AddedAttributeToDesign', own: 'Design'}, {op: 'AddedAttributeToPiece', own: 'Piece'}, {op: 'AddedAttributeToPort', own: 'Port'}, {op: 'AddedAttributeToQuality', own: 'Quality'}, {op: 'AddedAttributeToTag', own: 'Tag'}, {op: 'AddedAttributeToType', own: 'Type'}, {op: 'AddedChildPiecesWithParentConnections', own: 'Piece'}, {op: 'AddedChildPieceWithParentConnection', own: 'Piece'}, {op: 'AddedConnector', own: 'Connector'}, {op: 'AddedConnectors', own: 'Connector'}, {op: 'AddedHangingChildPiecesWithParentConnections', own: 'Piece'}, {op: 'AddedHangingChildPieceWithParentConnection', own: 'Piece'}, {op: 'ChangedDescription', own: 'Workspace'}, {op: 'ChangedPiecesToType', own: 'Piece'}, {op: 'ChangedPieceToType', own: 'Piece'}, {op: 'CreatedConcept', own: 'Concept'}, {op: 'CreatedConcepts', own: 'Concept'}, {op: 'CreatedDesign', own: 'Design'}, {op: 'CreatedDesigns', own: 'Design'}, {op: 'CreatedFixedPiece', own: 'Piece'}, {op: 'CreatedPort', own: 'Port'}, {op: 'CreatedPorts', own: 'Port'}, {op: 'CreatedQualities', own: 'Quality'}, {op: 'CreatedQuality', own: 'Quality'}, {op: 'CreatedTag', own: 'Tag'}, {op: 'CreatedTags', own: 'Tag'}, {op: 'CreatedType', own: 'Type'}, {op: 'CreatedTypes', own: 'Type'}, {op: 'DeletedConcept', own: 'Concept'}, {op: 'DeletedConcepts', own: 'Concept'}, {op: 'DeletedDesign', own: 'Design'}, {op: 'DeletedDesigns', own: 'Design'}, {op: 'DeletedPiece', own: 'Piece'}, {op: 'DeletedPieces', own: 'Piece'}, {op: 'DeletedPiecesAndConnections', own: 'Piece'}, {op: 'DeletedPort', own: 'Port'}, {op: 'DeletedPorts', own: 'Port'}, {op: 'DeletedQualities', own: 'Quality'}, {op: 'DeletedQuality', own: 'Quality'}, {op: 'DeletedTag', own: 'Tag'}, {op: 'DeletedTags', own: 'Tag'}, {op: 'DeletedType', own: 'Type'}, {op: 'DeletedTypes', own: 'Type'}, {op: 'DraggedPiece', own: 'Piece'}, {op: 'DraggedPieces', own: 'Piece'}, {op: 'FixedPiece', own: 'Piece'}, {op: 'FixedPieces', own: 'Piece'}, {op: 'FlattenedDesign', own: 'Design'}, {op: 'MovedPiece', own: 'Piece'}, {op: 'MovedPieces', own: 'Piece'}, {op: 'RemovedAttributeFromConcept', own: 'Concept'}, {op: 'RemovedAttributeFromDesign', own: 'Design'}, {op: 'RemovedAttributeFromPiece', own: 'Piece'}, {op: 'RemovedAttributeFromPort', own: 'Port'}, {op: 'RemovedAttributeFromQuality', own: 'Quality'}, {op: 'RemovedAttributeFromTag', own: 'Tag'}, {op: 'RemovedAttributeFromType', own: 'Type'}, {op: 'RemovedAttributesFromConcept', own: 'Concept'}, {op: 'RemovedAttributesFromDesign', own: 'Design'}, {op: 'RemovedAttributesFromPiece', own: 'Piece'}, {op: 'RemovedAttributesFromPort', own: 'Port'}, {op: 'RemovedAttributesFromQuality', own: 'Quality'}, {op: 'RemovedAttributesFromTag', own: 'Tag'}, {op: 'RemovedAttributesFromType', own: 'Type'}, {op: 'RemovedConnector', own: 'Connector'}, {op: 'RemovedConnectors', own: 'Connector'}, {op: 'RenamedConcept', own: 'Concept'}, {op: 'RenamedConnector', own: 'Connector'}, {op: 'RenamedKit', own: 'Kit'}, {op: 'RenamedPiece', own: 'Piece'}, {op: 'RenamedPort', own: 'Port'}, {op: 'RenamedQuality', own: 'Quality'}, {op: 'RenamedTag', own: 'Tag'}, {op: 'RenamedType', own: 'Type'}, {op: 'UpdatedConceptDescription', own: 'Concept'}, {op: 'UpdatedConceptIcon', own: 'Concept'}, {op: 'UpdatedConnectorDescription', own: 'Connector'}, {op: 'UpdatedConnectorIcon', own: 'Connector'}, {op: 'UpdatedPieceDescription', own: 'Piece'}, {op: 'UpdatedPortDescription', own: 'Port'}, {op: 'UpdatedPortIcon', own: 'Port'}, {op: 'UpdatedQualityDescription', own: 'Quality'}, {op: 'UpdatedQualityIcon', own: 'Quality'}, {op: 'UpdatedTagDescription', own: 'Tag'}, {op: 'UpdatedTagIcon', own: 'Tag'}, {op: 'UpdatedTypeDescription', own: 'Type'}, {op: 'UpdatedTypeIcon', own: 'Type'}] AS row
MATCH (c:Class {name: row.op})
MATCH (own:Class|Interface {name: row.own})
MERGE (own)-[:OWNS]->(m:Module {name: 'operation'})
MERGE (m)-[:OWNS]->(c)
WITH c, m
OPTIONAL MATCH (p:Module)-[r:OWNS]->(c)
WHERE id(p) <> id(m)
DELETE r;
//#endregion ReparentOperationClassesUnderOwnerOperationModules

//#region DetachCommandKitInterfaceNode
OPTIONAL MATCH (ic:Interface {name: 'Command'})
DETACH DELETE ic;
//#endregion DetachCommandKitInterfaceNode
