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
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Command|Constraint|Data|Computation|Reference|Input|Interface|Module|Scalar) ON EACH [n.name];
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

//#region RelabelRenameOperationCommands
// Generated — relabel legacy `Class` operation kit nodes to `Command`, then rename when `HEAD` golden differed.
MATCH (c:Class)-[:IS]->(op:Interface|Class)
WHERE toLower(op.name) = 'operation'
REMOVE c:Class
SET c:Command;
UNWIND [
{from: "AddedAttributesToConcept", to: "AddAttributesToConcept"}, {from: "AddedAttributesToDesign", to: "AddAttributesToDesign"}, {from: "AddedAttributesToPiece", to: "AddAttributesToPiece"}, {from: "AddedAttributesToPort", to: "AddAttributesToPort"}, {from: "AddedAttributesToQuality", to: "AddAttributesToQuality"}, {from: "AddedAttributesToTag", to: "AddAttributesToTag"}, {from: "AddedAttributesToType", to: "AddAttributesToType"}, {from: "AddedAttributeToConcept", to: "AddAttributeToConcept"}, {from: "AddedAttributeToDesign", to: "AddAttributeToDesign"}, {from: "AddedAttributeToPiece", to: "AddAttributeToPiece"}, {from: "AddedAttributeToPort", to: "AddAttributeToPort"}, {from: "AddedAttributeToQuality", to: "AddAttributeToQuality"}, {from: "AddedAttributeToTag", to: "AddAttributeToTag"}, {from: "AddedAttributeToType", to: "AddAttributeToType"}, {from: "AddedChildPiecesWithParentConnections", to: "AddChildPiecesWithParentConnections"}, {from: "AddedChildPieceWithParentConnection", to: "AddChildPieceWithParentConnection"}, {from: "AddedConnector", to: "AddConnector"}, {from: "AddedConnectors", to: "AddConnectors"}, {from: "AddedHangingChildPiecesWithParentConnections", to: "AddHangingChildPiecesWithParentConnections"}, {from: "AddedHangingChildPieceWithParentConnection", to: "AddHangingChildPieceWithParentConnection"}, {from: "ChangedDescription", to: "ChangeDescription"}, {from: "ChangedPiecesToType", to: "ChangePiecesToType"}, {from: "ChangedPieceToType", to: "ChangePieceToType"}, {from: "CreatedConcept", to: "CreateConcept"}, {from: "CreatedConcepts", to: "CreateConcepts"}, {from: "CreatedDesign", to: "CreateDesign"}, {from: "CreatedDesigns", to: "CreateDesigns"}, {from: "CreatedFixedPiece", to: "CreateFixedPiece"}, {from: "CreatedPort", to: "CreatePort"}, {from: "CreatedPorts", to: "CreatePorts"}, {from: "CreatedQualities", to: "CreateQualities"}, {from: "CreatedQuality", to: "CreateQuality"}, {from: "CreatedTag", to: "CreateTag"}, {from: "CreatedTags", to: "CreateTags"}, {from: "CreatedType", to: "CreateType"}, {from: "CreatedTypes", to: "CreateTypes"}, {from: "DeletedConcept", to: "DeleteConcept"}, {from: "DeletedConcepts", to: "DeleteConcepts"}, {from: "DeletedDesign", to: "DeleteDesign"}, {from: "DeletedDesigns", to: "DeleteDesigns"}, {from: "DeletedPiece", to: "DeletePiece"}, {from: "DeletedPieces", to: "DeletePieces"}, {from: "DeletedPiecesAndConnections", to: "DeletePiecesAndConnections"}, {from: "DeletedPort", to: "DeletePort"}, {from: "DeletedPorts", to: "DeletePorts"}, {from: "DeletedQualities", to: "DeleteQualities"}, {from: "DeletedQuality", to: "DeleteQuality"}, {from: "DeletedTag", to: "DeleteTag"}, {from: "DeletedTags", to: "DeleteTags"}, {from: "DeletedType", to: "DeleteType"}, {from: "DeletedTypes", to: "DeleteTypes"}, {from: "DraggedPiece", to: "DragPiece"}, {from: "DraggedPieces", to: "DragPieces"}, {from: "FixedPiece", to: "FixPiece"}, {from: "FixedPieces", to: "FixPieces"}, {from: "FlattenedDesign", to: "FlattenDesign"}, {from: "MovedPiece", to: "MovePiece"}, {from: "MovedPieces", to: "MovePieces"}, {from: "RemovedAttributeFromConcept", to: "RemoveAttributeFromConcept"}, {from: "RemovedAttributeFromDesign", to: "RemoveAttributeFromDesign"}, {from: "RemovedAttributeFromPiece", to: "RemoveAttributeFromPiece"}, {from: "RemovedAttributeFromPort", to: "RemoveAttributeFromPort"}, {from: "RemovedAttributeFromQuality", to: "RemoveAttributeFromQuality"}, {from: "RemovedAttributeFromTag", to: "RemoveAttributeFromTag"}, {from: "RemovedAttributeFromType", to: "RemoveAttributeFromType"}, {from: "RemovedAttributesFromConcept", to: "RemoveAttributesFromConcept"}, {from: "RemovedAttributesFromDesign", to: "RemoveAttributesFromDesign"}, {from: "RemovedAttributesFromPiece", to: "RemoveAttributesFromPiece"}, {from: "RemovedAttributesFromPort", to: "RemoveAttributesFromPort"}, {from: "RemovedAttributesFromQuality", to: "RemoveAttributesFromQuality"}, {from: "RemovedAttributesFromTag", to: "RemoveAttributesFromTag"}, {from: "RemovedAttributesFromType", to: "RemoveAttributesFromType"}, {from: "RemovedConnector", to: "RemoveConnector"}, {from: "RemovedConnectors", to: "RemoveConnectors"}, {from: "RenamedConcept", to: "RenameConcept"}, {from: "RenamedConnector", to: "RenameConnector"}, {from: "RenamedKit", to: "RenameKit"}, {from: "RenamedPiece", to: "RenamePiece"}, {from: "RenamedPort", to: "RenamePort"}, {from: "RenamedQuality", to: "RenameQuality"}, {from: "RenamedTag", to: "RenameTag"}, {from: "RenamedType", to: "RenameType"}, {from: "UpdatedConceptDescription", to: "UpdateConceptDescription"}, {from: "UpdatedConceptIcon", to: "UpdateConceptIcon"}, {from: "UpdatedConnectorDescription", to: "UpdateConnectorDescription"}, {from: "UpdatedConnectorIcon", to: "UpdateConnectorIcon"}, {from: "UpdatedPieceDescription", to: "UpdatePieceDescription"}, {from: "UpdatedPortDescription", to: "UpdatePortDescription"}, {from: "UpdatedPortIcon", to: "UpdatePortIcon"}, {from: "UpdatedQualityDescription", to: "UpdateQualityDescription"}, {from: "UpdatedQualityIcon", to: "UpdateQualityIcon"}, {from: "UpdatedTagDescription", to: "UpdateTagDescription"}, {from: "UpdatedTagIcon", to: "UpdateTagIcon"}, {from: "UpdatedTypeDescription", to: "UpdateTypeDescription"}, {from: "UpdatedTypeIcon", to: "UpdateTypeIcon"}
] AS row
MATCH (c:Command {name: row.from})
SET c.name = row.to;
//#endregion RelabelRenameOperationCommands

//#region MergeOperationConcreteCommands
UNWIND ["AddAttributesToConcept", "AddAttributesToDesign", "AddAttributesToPiece", "AddAttributesToPort", "AddAttributesToQuality", "AddAttributesToTag", "AddAttributesToType", "AddAttributeToConcept", "AddAttributeToDesign", "AddAttributeToPiece", "AddAttributeToPort", "AddAttributeToQuality", "AddAttributeToTag", "AddAttributeToType", "AddChildPiecesWithParentConnections", "AddChildPieceWithParentConnection", "AddConnector", "AddConnectors", "AddHangingChildPiecesWithParentConnections", "AddHangingChildPieceWithParentConnection", "ChangeDescription", "ChangePiecesToType", "ChangePieceToType", "CreateConcept", "CreateConcepts", "CreateDesign", "CreateDesigns", "CreateFixedPiece", "CreatePort", "CreatePorts", "CreateQualities", "CreateQuality", "CreateTag", "CreateTags", "CreateType", "CreateTypes", "DeleteConcept", "DeleteConcepts", "DeleteDesign", "DeleteDesigns", "DeletePiece", "DeletePieces", "DeletePiecesAndConnections", "DeletePort", "DeletePorts", "DeleteQualities", "DeleteQuality", "DeleteTag", "DeleteTags", "DeleteType", "DeleteTypes", "DragPiece", "DragPieces", "FixPiece", "FixPieces", "FlattenDesign", "MovePiece", "MovePieces", "RemoveAttributeFromConcept", "RemoveAttributeFromDesign", "RemoveAttributeFromPiece", "RemoveAttributeFromPort", "RemoveAttributeFromQuality", "RemoveAttributeFromTag", "RemoveAttributeFromType", "RemoveAttributesFromConcept", "RemoveAttributesFromDesign", "RemoveAttributesFromPiece", "RemoveAttributesFromPort", "RemoveAttributesFromQuality", "RemoveAttributesFromTag", "RemoveAttributesFromType", "RemoveConnector", "RemoveConnectors", "RenameConcept", "RenameConnector", "RenameKit", "RenamePiece", "RenamePort", "RenameQuality", "RenameTag", "RenameType", "UpdateConceptDescription", "UpdateConceptIcon", "UpdateConnectorDescription", "UpdateConnectorIcon", "UpdatePieceDescription", "UpdatePortDescription", "UpdatePortIcon", "UpdateQualityDescription", "UpdateQualityIcon", "UpdateTagDescription", "UpdateTagIcon", "UpdateTypeDescription", "UpdateTypeIcon"] AS opName
MERGE (c:Command {name: opName})
WITH c
MATCH (op:Interface|Class)
WHERE toLower(op.name) = 'operation'
WITH c, op
ORDER BY id(op) ASC
LIMIT 1
MERGE (c)-[:IS]->(op);
//#endregion MergeOperationConcreteCommands

//#region ReparentOperationCommandsUnderOwnerOperationModules
// Each domain `Class` / `Interface` (Piece, Quality, …) OWNS `Module(operation)` which OWNS concrete operation `Command` nodes (golden `Operation` subtypes).
UNWIND [{op: 'AddAttributesToConcept', own: 'Concept'}, {op: 'AddAttributesToDesign', own: 'Design'}, {op: 'AddAttributesToPiece', own: 'Piece'}, {op: 'AddAttributesToPort', own: 'Port'}, {op: 'AddAttributesToQuality', own: 'Quality'}, {op: 'AddAttributesToTag', own: 'Tag'}, {op: 'AddAttributesToType', own: 'Type'}, {op: 'AddAttributeToConcept', own: 'Concept'}, {op: 'AddAttributeToDesign', own: 'Design'}, {op: 'AddAttributeToPiece', own: 'Piece'}, {op: 'AddAttributeToPort', own: 'Port'}, {op: 'AddAttributeToQuality', own: 'Quality'}, {op: 'AddAttributeToTag', own: 'Tag'}, {op: 'AddAttributeToType', own: 'Type'}, {op: 'AddChildPiecesWithParentConnections', own: 'Piece'}, {op: 'AddChildPieceWithParentConnection', own: 'Piece'}, {op: 'AddConnector', own: 'Connector'}, {op: 'AddConnectors', own: 'Connector'}, {op: 'AddHangingChildPiecesWithParentConnections', own: 'Piece'}, {op: 'AddHangingChildPieceWithParentConnection', own: 'Piece'}, {op: 'ChangeDescription', own: 'Workspace'}, {op: 'ChangePiecesToType', own: 'Piece'}, {op: 'ChangePieceToType', own: 'Piece'}, {op: 'CreateConcept', own: 'Concept'}, {op: 'CreateConcepts', own: 'Concept'}, {op: 'CreateDesign', own: 'Design'}, {op: 'CreateDesigns', own: 'Design'}, {op: 'CreateFixedPiece', own: 'Piece'}, {op: 'CreatePort', own: 'Port'}, {op: 'CreatePorts', own: 'Port'}, {op: 'CreateQualities', own: 'Quality'}, {op: 'CreateQuality', own: 'Quality'}, {op: 'CreateTag', own: 'Tag'}, {op: 'CreateTags', own: 'Tag'}, {op: 'CreateType', own: 'Type'}, {op: 'CreateTypes', own: 'Type'}, {op: 'DeleteConcept', own: 'Concept'}, {op: 'DeleteConcepts', own: 'Concept'}, {op: 'DeleteDesign', own: 'Design'}, {op: 'DeleteDesigns', own: 'Design'}, {op: 'DeletePiece', own: 'Piece'}, {op: 'DeletePieces', own: 'Piece'}, {op: 'DeletePiecesAndConnections', own: 'Piece'}, {op: 'DeletePort', own: 'Port'}, {op: 'DeletePorts', own: 'Port'}, {op: 'DeleteQualities', own: 'Quality'}, {op: 'DeleteQuality', own: 'Quality'}, {op: 'DeleteTag', own: 'Tag'}, {op: 'DeleteTags', own: 'Tag'}, {op: 'DeleteType', own: 'Type'}, {op: 'DeleteTypes', own: 'Type'}, {op: 'DragPiece', own: 'Piece'}, {op: 'DragPieces', own: 'Piece'}, {op: 'FixPiece', own: 'Piece'}, {op: 'FixPieces', own: 'Piece'}, {op: 'FlattenDesign', own: 'Design'}, {op: 'MovePiece', own: 'Piece'}, {op: 'MovePieces', own: 'Piece'}, {op: 'RemoveAttributeFromConcept', own: 'Concept'}, {op: 'RemoveAttributeFromDesign', own: 'Design'}, {op: 'RemoveAttributeFromPiece', own: 'Piece'}, {op: 'RemoveAttributeFromPort', own: 'Port'}, {op: 'RemoveAttributeFromQuality', own: 'Quality'}, {op: 'RemoveAttributeFromTag', own: 'Tag'}, {op: 'RemoveAttributeFromType', own: 'Type'}, {op: 'RemoveAttributesFromConcept', own: 'Concept'}, {op: 'RemoveAttributesFromDesign', own: 'Design'}, {op: 'RemoveAttributesFromPiece', own: 'Piece'}, {op: 'RemoveAttributesFromPort', own: 'Port'}, {op: 'RemoveAttributesFromQuality', own: 'Quality'}, {op: 'RemoveAttributesFromTag', own: 'Tag'}, {op: 'RemoveAttributesFromType', own: 'Type'}, {op: 'RemoveConnector', own: 'Connector'}, {op: 'RemoveConnectors', own: 'Connector'}, {op: 'RenameConcept', own: 'Concept'}, {op: 'RenameConnector', own: 'Connector'}, {op: 'RenameKit', own: 'Kit'}, {op: 'RenamePiece', own: 'Piece'}, {op: 'RenamePort', own: 'Port'}, {op: 'RenameQuality', own: 'Quality'}, {op: 'RenameTag', own: 'Tag'}, {op: 'RenameType', own: 'Type'}, {op: 'UpdateConceptDescription', own: 'Concept'}, {op: 'UpdateConceptIcon', own: 'Concept'}, {op: 'UpdateConnectorDescription', own: 'Connector'}, {op: 'UpdateConnectorIcon', own: 'Connector'}, {op: 'UpdatePieceDescription', own: 'Piece'}, {op: 'UpdatePortDescription', own: 'Port'}, {op: 'UpdatePortIcon', own: 'Port'}, {op: 'UpdateQualityDescription', own: 'Quality'}, {op: 'UpdateQualityIcon', own: 'Quality'}, {op: 'UpdateTagDescription', own: 'Tag'}, {op: 'UpdateTagIcon', own: 'Tag'}, {op: 'UpdateTypeDescription', own: 'Type'}, {op: 'UpdateTypeIcon', own: 'Type'}] AS row
MATCH (c:Command {name: row.op})
MATCH (own:Class|Interface {name: row.own})
MERGE (own)-[:OWNS]->(m:Module {name: 'operation'})
MERGE (m)-[:OWNS]->(c)
WITH c, m
OPTIONAL MATCH (p:Module)-[r:OWNS]->(c)
WHERE id(p) <> id(m)
DELETE r;
//#endregion ReparentOperationCommandsUnderOwnerOperationModules

//#region DetachCommandKitInterfaceNode
OPTIONAL MATCH (ic:Interface {name: 'Command'})
DETACH DELETE ic;
//#endregion DetachCommandKitInterfaceNode
