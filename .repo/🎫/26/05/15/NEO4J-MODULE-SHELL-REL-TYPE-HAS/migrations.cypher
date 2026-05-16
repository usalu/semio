// SPDX-License-Identifier: AGPL-3.0-only
// Hand-maintained Neo4j migrations for the semio graph. Run via migrate.neo4j.script.ts in this folder (not chained from generate).
// Containment: `OWNS` for module trees, commands, kit rows, and legacy `HAS`→`OWNS` relabel targets; structural `HAS` for `Module`→`Class`/`Interface`/`Scalar` (see `semio/dev/schema/neo4j/schema.graphql`).

//#region RelabelHasRelationshipsToOwns
MATCH (a)-[r:HAS]->(b)
WHERE NOT (a:Module AND (b:Class OR b:Interface OR b:Scalar))
CREATE (a)-[r2:OWNS]->(b)
SET r2 += properties(r)
DELETE r;
//#endregion RelabelHasRelationshipsToOwns

//#region RelabelLegacyPartOfStructuralEdgesToHas
MATCH (m:Module)-[r:PART_OF]->(x)
WHERE x:Class OR x:Interface OR x:Scalar
MERGE (m)-[:HAS]->(x)
DELETE r;
//#endregion RelabelLegacyPartOfStructuralEdgesToHas

//#region RelabelFieldNodes
MATCH (f:Field {kind: 'EMBEDDED'})
SET f:Data
REMOVE f:Field, f.kind;

MATCH (f:Field)
WHERE f.kind IN ['COMPUTED', 'CACHED']
WITH f, f.kind AS k
SET f:Derived, f.cached = (k = 'CACHED')
REMOVE f:Field, f.kind;

MATCH (f:Field {kind: 'REFERENCE'})
SET f:Reference
REMOVE f:Field, f.kind;
//#endregion RelabelFieldNodes

//#region ReclassifyChangeSavedKitMemberAsDerived
MATCH (:Class {name: 'Change'})-[:OWNS]->(d:Data {name: 'saved'})
SET d:Derived, d.cached = false
REMOVE d:Data, d.isList;
MATCH (:Class {name: 'Change'})-[:OWNS]->(c:Derived {name: 'saved'})
MERGE (con:Constraint {description: '📌 saved derives from embedded sibling savedAt: non-null persisted timestamp implies the boolean reads true.'})
MERGE (c)-[:OWNS]->(con);
//#endregion ReclassifyChangeSavedKitMemberAsDerived

//#region ReplaceFieldIndexes
DROP INDEX index_field_name IF EXISTS;
CREATE RANGE INDEX index_data_name IF NOT EXISTS FOR (n:Data) ON (n.name);
DROP INDEX index_computation_name IF EXISTS;
CREATE RANGE INDEX index_derived_name IF NOT EXISTS FOR (n:Derived) ON (n.name);
CREATE RANGE INDEX index_reference_name IF NOT EXISTS FOR (n:Reference) ON (n.name);
DROP INDEX index_input_name IF EXISTS;
DROP INDEX semio_name_fulltext IF EXISTS;
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Command|Constraint|Data|Derived|Reference|Interface|Module|Scalar) ON EACH [n.name];
//#endregion ReplaceFieldIndexes

//#region RelabelLegacyComputationLabelToDerived
MATCH (n:Computation)
REMOVE n:Computation
SET n:Derived;
//#endregion RelabelLegacyComputationLabelToDerived

//#region FoldDupGeneralUnderDomain
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(genCanon:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[:OWNS]->(genDup:Module {name: 'General'})
WHERE id(genCanon) <> id(genDup)
MATCH (genDup)-[r:OWNS]->(ch)
WHERE ch:Class OR ch:Interface OR ch:Scalar
MERGE (genCanon)-[:HAS]->(ch)
DELETE r;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(genCanon:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[:OWNS]->(genDup:Module {name: 'General'})
WHERE id(genCanon) <> id(genDup)
MATCH (genDup)-[r:OWNS]->(ch)
WHERE NOT (ch:Class OR ch:Interface OR ch:Scalar)
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
WHERE ch:Class OR ch:Interface OR ch:Scalar
MERGE (gen)-[:HAS]->(ch)
DELETE r;
MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (dom:Module {name: 'Domain'})-[r:OWNS]->(ch)
WHERE NOT (ch:Class OR ch:Interface OR ch:Scalar)
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
MERGE (wmod)-[:HAS]->(i);

MATCH (schema:Module {name: 'Schema'})-[:OWNS]->(gen:Module {name: 'General'})
MATCH (gen)-[:OWNS]->(ent:Module {name: 'Entity'})
MATCH (ent)-[:OWNS]->(smod:Module {name: 'StrongEntity'})
MATCH (i:Interface {name: 'StrongEntity'})
MERGE (smod)-[:HAS]->(i);

MATCH (s:Module {name: 'StrongEntity'})-[:OWNS]->(rmod:Module {name: 'RichStrongEntity'})
MATCH (i:Interface {name: 'RichStrongEntity'})
MERGE (rmod)-[:HAS]->(i);

MATCH (r:Module {name: 'RichStrongEntity'})-[:OWNS]->(amod:Module {name: 'Artifact'})
MATCH (i:Interface {name: 'Artifact'})
MERGE (amod)-[:HAS]->(i);

MATCH (a:Module {name: 'Artifact'})-[:OWNS]->(dmod:Module {name: 'Document'})
MATCH (i:Interface {name: 'Document'})
MERGE (dmod)-[:HAS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:OWNS]->(dataMod:Module {name: 'Data'})
MATCH (i:Interface {name: 'Data'})
MERGE (dataMod)-[:HAS]->(i);

MATCH (w:Module {name: 'WeakEntity'})-[:OWNS]->(eventMod:Module {name: 'Event'})
MATCH (i:Interface {name: 'Event'})
MERGE (eventMod)-[:HAS]->(i);
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
MERGE (parent)-[:HAS]->(c)
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
// Persist only graph-native kit metadata: `rank` is sibling order (string); `isList` flags list-shaped members; `cached` only on Derived.
MATCH (n:Data)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false), soleOwnerKey: n.soleOwnerKey };
MATCH (n:Reference)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false) };
MATCH (n:Derived)
SET n = { name: n.name, rank: coalesce(n.rank, ''), isList: coalesce(n.isList, false), cached: coalesce(n.cached, false) };
MATCH (n:Class|Interface|Scalar|Module|Command)
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
MATCH (sub)-[rw:OWNS]->(f:Data|Derived|Reference)
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
MATCH (op)-[rw:OWNS]->(f:Data|Derived|Reference)
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
// Generated — relabel legacy `Class` operation kit nodes to `Command`, then sync names to imperative verbs (deduped).
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

//#region DedupeDuplicateOperationCommands
// Collapse accidental duplicate `:Command` rows sharing `name` (e.g. legacy `RenamedKit` renamed to `RenameKit` while `MERGE (Command {name:'RenameKit'})` already created a second row). Requires APOC `apoc.refactor.mergeNodes`.
MATCH (c:Command)
WITH c.name AS nm, collect(c) AS grp
WHERE size(grp) > 1
CALL apoc.refactor.mergeNodes(grp, {properties: "discard", mergeRels: true})
YIELD node
RETURN count(*) AS mergedCommandGroups;

MATCH (d:Data)
WHERE d.soleOwnerKey IS NOT NULL
WITH d.soleOwnerKey AS sk, d.name AS nm, collect(d) AS grp
WHERE size(grp) > 1
CALL apoc.refactor.mergeNodes(grp, {properties: "discard", mergeRels: true})
YIELD node
RETURN count(*) AS mergedArgumentDataGroups;
//#endregion DedupeDuplicateOperationCommands

//#region MergeCommandInputSurfaces
MATCH (inp:Input)-[:OWNS]->(ch)
DETACH DELETE ch;
MATCH (inp:Input)
DETACH DELETE inp;
MATCH (r:Reference)
WHERE toLower(r.name) = 'input' AND NOT ()-[:OWNS]->(r)
DETACH DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToConcept'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToDesign'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToPiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToPort'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToQuality'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToTag'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributesToType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attributes']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToConcept'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToDesign'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToPiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToPort'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToQuality'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToTag'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddAttributeToType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['attribute']
DELETE r;

MATCH (cmd:Command {name: 'AddChildPiecesWithParentConnections'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['pieces', 'connections']
DELETE r;

MATCH (cmd:Command {name: 'AddChildPieceWithParentConnection'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['piece']
DELETE r;

MATCH (cmd:Command {name: 'AddConnector'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['connector']
DELETE r;

MATCH (cmd:Command {name: 'AddConnectors'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['connectors']
DELETE r;

MATCH (cmd:Command {name: 'AddHangingChildPiecesWithParentConnections'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['pieces', 'connections']
DELETE r;

MATCH (cmd:Command {name: 'AddHangingChildPieceWithParentConnection'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['piece']
DELETE r;

MATCH (cmd:Command {name: 'ChangeDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'ChangePiecesToType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['blueprintId']
DELETE r;

MATCH (cmd:Command {name: 'ChangePieceToType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['blueprintId']
DELETE r;

MATCH (cmd:Command {name: 'CreateConcept'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['concept']
DELETE r;

MATCH (cmd:Command {name: 'CreateConcepts'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['concepts']
DELETE r;

MATCH (cmd:Command {name: 'CreateDesign'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['design']
DELETE r;

MATCH (cmd:Command {name: 'CreateDesigns'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['designs']
DELETE r;

MATCH (cmd:Command {name: 'CreateFixedPiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['blueprintId', 'position']
DELETE r;

MATCH (cmd:Command {name: 'CreatePort'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['port']
DELETE r;

MATCH (cmd:Command {name: 'CreatePorts'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['ports']
DELETE r;

MATCH (cmd:Command {name: 'CreateQualities'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['qualities']
DELETE r;

MATCH (cmd:Command {name: 'CreateQuality'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['quality']
DELETE r;

MATCH (cmd:Command {name: 'CreateTag'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['tag']
DELETE r;

MATCH (cmd:Command {name: 'CreateTags'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['tags']
DELETE r;

MATCH (cmd:Command {name: 'CreateType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['type']
DELETE r;

MATCH (cmd:Command {name: 'CreateTypes'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['types']
DELETE r;

MATCH (cmd:Command {name: 'DragPiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['offset']
DELETE r;

MATCH (cmd:Command {name: 'DragPieces'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['offset']
DELETE r;

MATCH (cmd:Command {name: 'MovePiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['position']
DELETE r;

MATCH (cmd:Command {name: 'MovePieces'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['offset']
DELETE r;

MATCH (cmd:Command {name: 'RenameConcept'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['name']
DELETE r;

MATCH (cmd:Command {name: 'RenameConnector'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['code']
DELETE r;

MATCH (cmd:Command {name: 'RenameKit'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['name']
DELETE r;

MATCH (cmd:Command {name: 'RenamePiece'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['name']
DELETE r;

MATCH (cmd:Command {name: 'RenamePort'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['code', 'label']
DELETE r;

MATCH (cmd:Command {name: 'RenameQuality'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['key']
DELETE r;

MATCH (cmd:Command {name: 'RenameTag'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['name']
DELETE r;

MATCH (cmd:Command {name: 'RenameType'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['name']
DELETE r;

MATCH (cmd:Command {name: 'UpdateConceptDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdateConceptIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

MATCH (cmd:Command {name: 'UpdateConnectorDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdateConnectorIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

MATCH (cmd:Command {name: 'UpdatePieceDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdatePortDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdatePortIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

MATCH (cmd:Command {name: 'UpdateQualityDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdateQualityIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

MATCH (cmd:Command {name: 'UpdateTagDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdateTagIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

MATCH (cmd:Command {name: 'UpdateTypeDescription'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['description']
DELETE r;

MATCH (cmd:Command {name: 'UpdateTypeIcon'})
OPTIONAL MATCH (cmd)-[r:OWNS]->(pivot:Data)
WHERE pivot.name IN ['icon']
DELETE r;

// Drop detached kit `Data` (no `OWNS` parent) that still carry `IS` — stale shared argument rows after `soleOwnerKey` split.
MATCH (d:Data)
WHERE NOT ()-[:OWNS]->(d) AND EXISTS { (d)-[:IS]->(:Class|Interface|Scalar|Command) }
DETACH DELETE d;

MATCH (cmd:Command {name: 'AddAttributesToConcept'})
MERGE (arg_AddAttributesToConcept_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToConcept', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToConcept_attributes)
WITH arg_AddAttributesToConcept_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToConcept_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToConcept_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToDesign'})
MERGE (arg_AddAttributesToDesign_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToDesign', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToDesign_attributes)
WITH arg_AddAttributesToDesign_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToDesign_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToDesign_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToPiece'})
MERGE (arg_AddAttributesToPiece_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToPiece', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToPiece_attributes)
WITH arg_AddAttributesToPiece_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToPiece_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToPiece_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToPort'})
MERGE (arg_AddAttributesToPort_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToPort', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToPort_attributes)
WITH arg_AddAttributesToPort_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToPort_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToPort_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToQuality'})
MERGE (arg_AddAttributesToQuality_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToQuality', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToQuality_attributes)
WITH arg_AddAttributesToQuality_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToQuality_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToQuality_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToTag'})
MERGE (arg_AddAttributesToTag_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToTag', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToTag_attributes)
WITH arg_AddAttributesToTag_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToTag_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToTag_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributesToType'})
MERGE (arg_AddAttributesToType_attributes:Data {name: 'attributes', soleOwnerKey: 'AddAttributesToType', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddAttributesToType_attributes)
WITH arg_AddAttributesToType_attributes
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributesToType_attributes, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributesToType_attributes)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToConcept'})
MERGE (arg_AddAttributeToConcept_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToConcept', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToConcept_attribute)
WITH arg_AddAttributeToConcept_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToConcept_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToConcept_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToDesign'})
MERGE (arg_AddAttributeToDesign_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToDesign', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToDesign_attribute)
WITH arg_AddAttributeToDesign_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToDesign_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToDesign_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToPiece'})
MERGE (arg_AddAttributeToPiece_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToPiece', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToPiece_attribute)
WITH arg_AddAttributeToPiece_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToPiece_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToPiece_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToPort'})
MERGE (arg_AddAttributeToPort_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToPort', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToPort_attribute)
WITH arg_AddAttributeToPort_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToPort_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToPort_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToQuality'})
MERGE (arg_AddAttributeToQuality_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToQuality', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToQuality_attribute)
WITH arg_AddAttributeToQuality_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToQuality_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToQuality_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToTag'})
MERGE (arg_AddAttributeToTag_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToTag', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToTag_attribute)
WITH arg_AddAttributeToTag_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToTag_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToTag_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddAttributeToType'})
MERGE (arg_AddAttributeToType_attribute:Data {name: 'attribute', soleOwnerKey: 'AddAttributeToType', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddAttributeToType_attribute)
WITH arg_AddAttributeToType_attribute
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Attribute'
WITH arg_AddAttributeToType_attribute, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddAttributeToType_attribute)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddChildPiecesWithParentConnections'})
MERGE (arg_AddChildPiecesWithParentConnections_pieces:Data {name: 'pieces', soleOwnerKey: 'AddChildPiecesWithParentConnections', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddChildPiecesWithParentConnections_pieces)
WITH arg_AddChildPiecesWithParentConnections_pieces
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Piece'
WITH arg_AddChildPiecesWithParentConnections_pieces, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddChildPiecesWithParentConnections_pieces)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddChildPiecesWithParentConnections'})
MERGE (arg_AddChildPiecesWithParentConnections_connections:Data {name: 'connections', soleOwnerKey: 'AddChildPiecesWithParentConnections', rank: '1', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddChildPiecesWithParentConnections_connections)
WITH arg_AddChildPiecesWithParentConnections_connections
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Connection'
WITH arg_AddChildPiecesWithParentConnections_connections, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddChildPiecesWithParentConnections_connections)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddChildPieceWithParentConnection'})
MERGE (arg_AddChildPieceWithParentConnection_piece:Data {name: 'piece', soleOwnerKey: 'AddChildPieceWithParentConnection', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddChildPieceWithParentConnection_piece)
WITH arg_AddChildPieceWithParentConnection_piece
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Piece'
WITH arg_AddChildPieceWithParentConnection_piece, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddChildPieceWithParentConnection_piece)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddConnector'})
MERGE (arg_AddConnector_connector:Data {name: 'connector', soleOwnerKey: 'AddConnector', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddConnector_connector)
WITH arg_AddConnector_connector
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Connector'
WITH arg_AddConnector_connector, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddConnector_connector)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddConnectors'})
MERGE (arg_AddConnectors_connectors:Data {name: 'connectors', soleOwnerKey: 'AddConnectors', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddConnectors_connectors)
WITH arg_AddConnectors_connectors
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Connector'
WITH arg_AddConnectors_connectors, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddConnectors_connectors)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddHangingChildPiecesWithParentConnections'})
MERGE (arg_AddHangingChildPiecesWithParentConnections_pieces:Data {name: 'pieces', soleOwnerKey: 'AddHangingChildPiecesWithParentConnections', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddHangingChildPiecesWithParentConnections_pieces)
WITH arg_AddHangingChildPiecesWithParentConnections_pieces
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Piece'
WITH arg_AddHangingChildPiecesWithParentConnections_pieces, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddHangingChildPiecesWithParentConnections_pieces)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddHangingChildPiecesWithParentConnections'})
MERGE (arg_AddHangingChildPiecesWithParentConnections_connections:Data {name: 'connections', soleOwnerKey: 'AddHangingChildPiecesWithParentConnections', rank: '1', isList: true})
MERGE (cmd)-[:OWNS]->(arg_AddHangingChildPiecesWithParentConnections_connections)
WITH arg_AddHangingChildPiecesWithParentConnections_connections
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Connection'
WITH arg_AddHangingChildPiecesWithParentConnections_connections, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddHangingChildPiecesWithParentConnections_connections)-[:IS]->(t));

MATCH (cmd:Command {name: 'AddHangingChildPieceWithParentConnection'})
MERGE (arg_AddHangingChildPieceWithParentConnection_piece:Data {name: 'piece', soleOwnerKey: 'AddHangingChildPieceWithParentConnection', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_AddHangingChildPieceWithParentConnection_piece)
WITH arg_AddHangingChildPieceWithParentConnection_piece
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Piece'
WITH arg_AddHangingChildPieceWithParentConnection_piece, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_AddHangingChildPieceWithParentConnection_piece)-[:IS]->(t));

MATCH (cmd:Command {name: 'ChangeDescription'})
MERGE (arg_ChangeDescription_description:Data {name: 'description', soleOwnerKey: 'ChangeDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_ChangeDescription_description)
WITH arg_ChangeDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_ChangeDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_ChangeDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'ChangePiecesToType'})
MERGE (arg_ChangePiecesToType_blueprintId:Data {name: 'blueprintId', soleOwnerKey: 'ChangePiecesToType', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_ChangePiecesToType_blueprintId)
WITH arg_ChangePiecesToType_blueprintId
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'ID'
WITH arg_ChangePiecesToType_blueprintId, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_ChangePiecesToType_blueprintId)-[:IS]->(t));

MATCH (cmd:Command {name: 'ChangePieceToType'})
MERGE (arg_ChangePieceToType_blueprintId:Data {name: 'blueprintId', soleOwnerKey: 'ChangePieceToType', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_ChangePieceToType_blueprintId)
WITH arg_ChangePieceToType_blueprintId
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'ID'
WITH arg_ChangePieceToType_blueprintId, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_ChangePieceToType_blueprintId)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateConcept'})
MERGE (arg_CreateConcept_concept:Data {name: 'concept', soleOwnerKey: 'CreateConcept', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateConcept_concept)
WITH arg_CreateConcept_concept
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Concept'
WITH arg_CreateConcept_concept, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateConcept_concept)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateConcepts'})
MERGE (arg_CreateConcepts_concepts:Data {name: 'concepts', soleOwnerKey: 'CreateConcepts', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreateConcepts_concepts)
WITH arg_CreateConcepts_concepts
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Concept'
WITH arg_CreateConcepts_concepts, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateConcepts_concepts)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateDesign'})
MERGE (arg_CreateDesign_design:Data {name: 'design', soleOwnerKey: 'CreateDesign', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateDesign_design)
WITH arg_CreateDesign_design
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Design'
WITH arg_CreateDesign_design, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateDesign_design)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateDesigns'})
MERGE (arg_CreateDesigns_designs:Data {name: 'designs', soleOwnerKey: 'CreateDesigns', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreateDesigns_designs)
WITH arg_CreateDesigns_designs
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Design'
WITH arg_CreateDesigns_designs, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateDesigns_designs)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateFixedPiece'})
MERGE (arg_CreateFixedPiece_blueprintId:Data {name: 'blueprintId', soleOwnerKey: 'CreateFixedPiece', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateFixedPiece_blueprintId)
WITH arg_CreateFixedPiece_blueprintId
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'ID'
WITH arg_CreateFixedPiece_blueprintId, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateFixedPiece_blueprintId)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateFixedPiece'})
MERGE (arg_CreateFixedPiece_position:Data {name: 'position', soleOwnerKey: 'CreateFixedPiece', rank: '1', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateFixedPiece_position)
WITH arg_CreateFixedPiece_position
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Position'
WITH arg_CreateFixedPiece_position, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateFixedPiece_position)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreatePort'})
MERGE (arg_CreatePort_port:Data {name: 'port', soleOwnerKey: 'CreatePort', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreatePort_port)
WITH arg_CreatePort_port
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Port'
WITH arg_CreatePort_port, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreatePort_port)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreatePorts'})
MERGE (arg_CreatePorts_ports:Data {name: 'ports', soleOwnerKey: 'CreatePorts', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreatePorts_ports)
WITH arg_CreatePorts_ports
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Port'
WITH arg_CreatePorts_ports, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreatePorts_ports)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateQualities'})
MERGE (arg_CreateQualities_qualities:Data {name: 'qualities', soleOwnerKey: 'CreateQualities', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreateQualities_qualities)
WITH arg_CreateQualities_qualities
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Quality'
WITH arg_CreateQualities_qualities, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateQualities_qualities)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateQuality'})
MERGE (arg_CreateQuality_quality:Data {name: 'quality', soleOwnerKey: 'CreateQuality', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateQuality_quality)
WITH arg_CreateQuality_quality
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Quality'
WITH arg_CreateQuality_quality, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateQuality_quality)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateTag'})
MERGE (arg_CreateTag_tag:Data {name: 'tag', soleOwnerKey: 'CreateTag', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateTag_tag)
WITH arg_CreateTag_tag
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Tag'
WITH arg_CreateTag_tag, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateTag_tag)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateTags'})
MERGE (arg_CreateTags_tags:Data {name: 'tags', soleOwnerKey: 'CreateTags', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreateTags_tags)
WITH arg_CreateTags_tags
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Tag'
WITH arg_CreateTags_tags, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateTags_tags)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateType'})
MERGE (arg_CreateType_type:Data {name: 'type', soleOwnerKey: 'CreateType', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_CreateType_type)
WITH arg_CreateType_type
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Type'
WITH arg_CreateType_type, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateType_type)-[:IS]->(t));

MATCH (cmd:Command {name: 'CreateTypes'})
MERGE (arg_CreateTypes_types:Data {name: 'types', soleOwnerKey: 'CreateTypes', rank: '0', isList: true})
MERGE (cmd)-[:OWNS]->(arg_CreateTypes_types)
WITH arg_CreateTypes_types
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Type'
WITH arg_CreateTypes_types, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_CreateTypes_types)-[:IS]->(t));

MATCH (cmd:Command {name: 'DragPiece'})
MERGE (arg_DragPiece_offset:Data {name: 'offset', soleOwnerKey: 'DragPiece', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_DragPiece_offset)
WITH arg_DragPiece_offset
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Offset'
WITH arg_DragPiece_offset, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_DragPiece_offset)-[:IS]->(t));

MATCH (cmd:Command {name: 'DragPieces'})
MERGE (arg_DragPieces_offset:Data {name: 'offset', soleOwnerKey: 'DragPieces', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_DragPieces_offset)
WITH arg_DragPieces_offset
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Offset'
WITH arg_DragPieces_offset, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_DragPieces_offset)-[:IS]->(t));

MATCH (cmd:Command {name: 'MovePiece'})
MERGE (arg_MovePiece_position:Data {name: 'position', soleOwnerKey: 'MovePiece', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_MovePiece_position)
WITH arg_MovePiece_position
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Position'
WITH arg_MovePiece_position, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_MovePiece_position)-[:IS]->(t));

MATCH (cmd:Command {name: 'MovePieces'})
MERGE (arg_MovePieces_offset:Data {name: 'offset', soleOwnerKey: 'MovePieces', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_MovePieces_offset)
WITH arg_MovePieces_offset
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'Offset'
WITH arg_MovePieces_offset, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_MovePieces_offset)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameConcept'})
MERGE (arg_RenameConcept_name:Data {name: 'name', soleOwnerKey: 'RenameConcept', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameConcept_name)
WITH arg_RenameConcept_name
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameConcept_name, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameConcept_name)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameConnector'})
MERGE (arg_RenameConnector_code:Data {name: 'code', soleOwnerKey: 'RenameConnector', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameConnector_code)
WITH arg_RenameConnector_code
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameConnector_code, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameConnector_code)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameKit'})
MERGE (arg_RenameKit_name:Data {name: 'name', soleOwnerKey: 'RenameKit', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameKit_name)
WITH arg_RenameKit_name
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameKit_name, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameKit_name)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenamePiece'})
MERGE (arg_RenamePiece_name:Data {name: 'name', soleOwnerKey: 'RenamePiece', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenamePiece_name)
WITH arg_RenamePiece_name
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenamePiece_name, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenamePiece_name)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenamePort'})
MERGE (arg_RenamePort_code:Data {name: 'code', soleOwnerKey: 'RenamePort', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenamePort_code)
WITH arg_RenamePort_code
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenamePort_code, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenamePort_code)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenamePort'})
MERGE (arg_RenamePort_label:Data {name: 'label', soleOwnerKey: 'RenamePort', rank: '1', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenamePort_label)
WITH arg_RenamePort_label
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenamePort_label, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenamePort_label)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameQuality'})
MERGE (arg_RenameQuality_key:Data {name: 'key', soleOwnerKey: 'RenameQuality', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameQuality_key)
WITH arg_RenameQuality_key
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameQuality_key, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameQuality_key)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameTag'})
MERGE (arg_RenameTag_name:Data {name: 'name', soleOwnerKey: 'RenameTag', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameTag_name)
WITH arg_RenameTag_name
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameTag_name, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameTag_name)-[:IS]->(t));

MATCH (cmd:Command {name: 'RenameType'})
MERGE (arg_RenameType_name:Data {name: 'name', soleOwnerKey: 'RenameType', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_RenameType_name)
WITH arg_RenameType_name
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_RenameType_name, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_RenameType_name)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateConceptDescription'})
MERGE (arg_UpdateConceptDescription_description:Data {name: 'description', soleOwnerKey: 'UpdateConceptDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateConceptDescription_description)
WITH arg_UpdateConceptDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateConceptDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateConceptDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateConceptIcon'})
MERGE (arg_UpdateConceptIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdateConceptIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateConceptIcon_icon)
WITH arg_UpdateConceptIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateConceptIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateConceptIcon_icon)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateConnectorDescription'})
MERGE (arg_UpdateConnectorDescription_description:Data {name: 'description', soleOwnerKey: 'UpdateConnectorDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateConnectorDescription_description)
WITH arg_UpdateConnectorDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateConnectorDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateConnectorDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateConnectorIcon'})
MERGE (arg_UpdateConnectorIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdateConnectorIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateConnectorIcon_icon)
WITH arg_UpdateConnectorIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateConnectorIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateConnectorIcon_icon)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdatePieceDescription'})
MERGE (arg_UpdatePieceDescription_description:Data {name: 'description', soleOwnerKey: 'UpdatePieceDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdatePieceDescription_description)
WITH arg_UpdatePieceDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdatePieceDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdatePieceDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdatePortDescription'})
MERGE (arg_UpdatePortDescription_description:Data {name: 'description', soleOwnerKey: 'UpdatePortDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdatePortDescription_description)
WITH arg_UpdatePortDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdatePortDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdatePortDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdatePortIcon'})
MERGE (arg_UpdatePortIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdatePortIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdatePortIcon_icon)
WITH arg_UpdatePortIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdatePortIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdatePortIcon_icon)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateQualityDescription'})
MERGE (arg_UpdateQualityDescription_description:Data {name: 'description', soleOwnerKey: 'UpdateQualityDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateQualityDescription_description)
WITH arg_UpdateQualityDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateQualityDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateQualityDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateQualityIcon'})
MERGE (arg_UpdateQualityIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdateQualityIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateQualityIcon_icon)
WITH arg_UpdateQualityIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateQualityIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateQualityIcon_icon)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateTagDescription'})
MERGE (arg_UpdateTagDescription_description:Data {name: 'description', soleOwnerKey: 'UpdateTagDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateTagDescription_description)
WITH arg_UpdateTagDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateTagDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateTagDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateTagIcon'})
MERGE (arg_UpdateTagIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdateTagIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateTagIcon_icon)
WITH arg_UpdateTagIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateTagIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateTagIcon_icon)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateTypeDescription'})
MERGE (arg_UpdateTypeDescription_description:Data {name: 'description', soleOwnerKey: 'UpdateTypeDescription', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateTypeDescription_description)
WITH arg_UpdateTypeDescription_description
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateTypeDescription_description, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateTypeDescription_description)-[:IS]->(t));

MATCH (cmd:Command {name: 'UpdateTypeIcon'})
MERGE (arg_UpdateTypeIcon_icon:Data {name: 'icon', soleOwnerKey: 'UpdateTypeIcon', rank: '0', isList: false})
MERGE (cmd)-[:OWNS]->(arg_UpdateTypeIcon_icon)
WITH arg_UpdateTypeIcon_icon
OPTIONAL MATCH (t)
WHERE (t:Class OR t:Interface OR t:Scalar OR t:Command) AND t.name = 'String'
WITH arg_UpdateTypeIcon_icon, t ORDER BY CASE WHEN t:Class THEN 0 WHEN t:Interface THEN 1 WHEN t:Command THEN 2 ELSE 3 END
LIMIT 1
FOREACH (_ IN CASE WHEN t IS NULL THEN [] ELSE [1] END | MERGE (arg_UpdateTypeIcon_icon)-[:IS]->(t));
//#endregion MergeCommandInputSurfaces

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

//#region RetypeResidualModuleOwnsToHas
// Final sweep: any `Module-[:OWNS]->(Class|Interface|Scalar)` left from legacy imports becomes `HAS` (idempotent with MERGE).
MATCH (m:Module)-[r:OWNS]->(x)
WHERE x:Class OR x:Interface OR x:Scalar
MERGE (m)-[:HAS]->(x)
DELETE r;
//#endregion RetypeResidualModuleOwnsToHas

//#region DetachCommandKitInterfaceNode
OPTIONAL MATCH (ic:Interface {name: 'Command'})
DETACH DELETE ic;
//#endregion DetachCommandKitInterfaceNode
