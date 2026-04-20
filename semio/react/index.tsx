// @ts-nocheck
// #region ⚛️Header

// Standalone React hooks bundle for semio.

// #endregion ⚛️Header

// #region ⚛️Imports

import * as React from "react";
import type { ReactNode, SetStateAction } from "react";
import {
	asKitInstance,
	Coord,
	Design,
	guid,
	InMemoryKitStore,
	KitImpl,
	type KitLike,
	type KitStore,
	type KitStoreSnapshot,
	Plane,
	Piece,
} from "@semio/js";

// #endregion ⚛️Imports

// #region ⚛️Manifest

const SCHEMA_MANIFEST = `scalar JSON
enum ActorKind
interface Actor:id,name,email,color
type User:hash,id,name,email,color
type Agent:hash,id,llm,name,email,color
input SessionActorInput:id,kind,llm,name,email,color
type Coordinate:hash,u,v
input CoordinateInput:u,v
type Point:hash,x,y,z
input PointInput:x,y,z
type Vector:hash,x,y,z
input VectorInput:x,y,z
type Plane:hash,origin,xAxis,yAxis
input PlaneInput:origin,xAxis,yAxis
type Camera:hash,position,forward,up
input CameraInput:position,forward,up
type Attribute:hash,id,key,value,definition
input AttributeInput:id,key,value,definition
type Location:hash,longitude,latitude,altitude,attributes
input LocationInput:longitude,latitude,altitude,attributes
type Author:hash,id,name,email,attributes
input AuthorInput:id,name,email,attributes
input AuthorPatchInput:name,email,attributes
type Folder:hash,id,kit,name,parent,children,description,attributes,createdAt,createdBy,updatedAt,updatedBy
input FolderInput:id,name,parentId,description,attributes,createdAt,createdById,updatedAt,updatedById
input FolderPatchInput:name,parentId,description,attributes,createdAt,createdById,updatedAt,updatedById
type File:hash,id,kit,name,remote,folder,size,contentHash,blob,mime,createdAt,createdBy,updatedAt,updatedBy
input FileInput:id,name,remote,folderId,size,contentHash,blob,mime,createdAt,createdById,updatedAt,updatedById
input FilePatchInput:name,remote,folderId,size,contentHash,blob,mime,createdAt,createdById,updatedAt,updatedById
type Benchmark:hash,id,quality,name,icon,min,minExcluded,max,maxExcluded,attributes
input BenchmarkInput:id,name,icon,min,minExcluded,max,maxExcluded,attributes
type Quality:hash,id,kit,key,name,description,uri,kind,folder,canScale,defaultSiUnit,defaultImperialUnit,min,isMinExcluded,max,isMaxExcluded,defaultValue,formula,icon,image,unit,benchmarks,attributes
input QualityInput:id,key,name,description,uri,kind,folderId,canScale,defaultSiUnit,defaultImperialUnit,min,isMinExcluded,max,isMaxExcluded,defaultValue,formula,icon,image,unit,benchmarks,attributes
input QualityPatchInput:key,name,description,uri,kind,folderId,canScale,defaultSiUnit,defaultImperialUnit,min,isMinExcluded,max,isMaxExcluded,defaultValue,formula,icon,image,unit,benchmarks,attributes
type Port:hash,id,kit,name,description,icon,maxChildren,compatiblePorts,attributes
input PortInput:id,name,description,icon,maxChildren,compatiblePortIds,attributes
input PortPatchInput:name,description,icon,maxChildren,compatiblePortIds,attributes
type Prop:hash,id,kit,quality,value,unit,attributes
input PropInput:id,qualityId,value,unit,attributes
type Tag:hash,id,kit,name,description,icon,attributes
input TagInput:id,name,description,icon,attributes
input TagPatchInput:name,description,icon,attributes
type Concept:hash,id,kit,name,description,icon,attributes
input ConceptInput:id,name,description,icon,attributes
input ConceptPatchInput:name,description,icon,attributes
type Family:hash,id,kit,name,description,icon,ports,attributes
input FamilyInput:id,name,description,icon,ports,attributes
input FamilyPatchInput:name,description,icon,ports,attributes
type Representation:hash,id,type,name,tags,file,description,attributes
input RepresentationInput:id,name,tagIds,fileId,description,attributes
type Connector:hash,id,type,name,t,point,direction,description,port,mandatory,maxChildren,props,attributes,pieces,compatibleConnectors
input ConnectorInput:id,name,t,point,direction,description,portId,mandatory,maxChildren,props,attributes
type Type:hash,id,kit,name,parent,children,isAbstract,folder,representations,connectors,props,stock,virtual,unit,createdAt,updatedAt,location,authors,concepts,icon,image,description,attributes,fixedPieces
input TypeInput:id,name,parentId,isAbstract,folderId,representations,connectors,props,stock,virtual,unit,createdAt,updatedAt,location,authorIds,conceptIds,icon,image,description,attributes
input TypePatchInput:name,parentId,isAbstract,folderId,representations,connectors,props,stock,virtual,unit,createdAt,updatedAt,location,authorIds,conceptIds,icon,image,description,attributes
type Layer:hash,id,design,path,isHidden,isLocked,color,description,attributes
input LayerInput:id,path,isHidden,isLocked,color,description,attributes
type Side:hash,connection,piece,designPiece,connector
input SideInput:pieceId,designPieceId,connectorId
type Connection:hash,id,design,connected,connecting,gap,shift,rise,rotation,turn,tilt,u,v,description,attributes,childPiece,childConnector,parentPiece,parentConnector
input ConnectionInput:id,connected,connecting,gap,shift,rise,rotation,turn,tilt,u,v,description,attributes
input ConnectionPatchInput:connected,connecting,gap,shift,rise,rotation,turn,tilt,u,v,description,attributes
type Stat:hash,id,design,quality,unit,min,minExcluded,max,maxExcluded
input StatInput:id,qualityId,unit,min,minExcluded,max,maxExcluded
enum PieceKind
type Blueprint:type,design
type Piece:id,hash,name,plane,center,scale,mirrorPlane,isHidden,isLocked,color,description,kind,type,design,props,attributes,flatPlane,flatCenter,parentPiece,parentConnection,childPieces,childConnections,alternatives,alternativeTypes,alternativeDesigns
input PieceInput:id,name,typeId,designReferenceId,plane,center,scale,mirrorPlane,isHidden,isLocked,color,description,props,attributes
input PiecePatchInput:name,typeId,designReferenceId,plane,center,scale,mirrorPlane,isHidden,isLocked,color,description,props,attributes
type Group:hash,id,design,pieces,color,name,description,attributes
input GroupInput:id,pieceIds,color,name,description,attributes
type Design:hash,id,kit,name,parent,children,isAbstract,folder,pieces,connections,stats,props,layers,activeLayer,groups,canScale,canMirror,unit,location,authors,concepts,icon,image,description,attributes,createdAt,updatedAt
input DesignInput:id,name,parentId,isAbstract,folderId,pieces,connections,stats,props,layers,activeLayerId,groups,canScale,canMirror,unit,location,authorIds,conceptIds,icon,image,description,attributes,createdAt,updatedAt
input DesignPatchInput:name,parentId,isAbstract,folderId,stats,props,layers,activeLayerId,groups,canScale,canMirror,unit,location,authorIds,conceptIds,icon,image,description,attributes,createdAt,updatedAt
type Kit:hash,id,name,release,types,designs,tags,concepts,families,ports,qualities,files,folders,authors,remote,homepage,license,preview,icon,image,description,attributes,createdAt,updatedAt
input KitInput:id,name,release,types,designs,tags,concepts,families,ports,qualities,files,folders,authors,remote,homepage,license,preview,icon,image,description,attributes,createdAt,updatedAt
input KitPatchInput:name,release,remote,homepage,license,preview,icon,image,description,attributes,createdAt,updatedAt
enum BackboneKind
type KitBackbone:hash,kind,endpoint,authoritative,linearHistory,connected,timeoutSeconds,currentHash,lastInteractionIndex,pendingCandidateCount
type KitClientInfo:hash,id,name,version,platform
input KitClientInfoInput:id,name,version,platform
enum SessionState
enum SessionWarningActionKind
type SessionWarningAction:hash,kind,label
type KitSessionWarning:hash,code,message,actions
type SessionConnectorSelection:hash,piece,designPiece,connector
input SessionConnectorSelectionInput:pieceId,designPieceId,connectorId
type KitSessionSelection:hash,activeDesign,pieces,connections,connectors,representations,designs,types,replacementTypeCandidates,replacementDesignCandidates,boundaryConnectorCount
input SessionSelectionInput:activeDesignId,pieceIds,connectionIds,connectorSelections,representationIds,designIds,typeIds
type KitSession:hash,id,kit,actor,client,state,strictMode,timeoutSeconds,startedAt,lastSeenAt,expiresAt,disconnectedAt,locked,canReconnect,canSaveLocalChanges,warning,selection,activeTransactions
enum ValidationSeverity
type ValidationNote:hash,severity,code,path,entityId,message
type KitValidationResult:hash,ok,immutable,strict,errors,warnings,infos
enum KitConflictStatus
enum KitConflictKind
enum ConflictResolutionKind
type ConflictResolutionOption:hash,id,kind,label,description,patchPreview
type KitConflict:hash,id,kit,session,candidate,status,kind,title,message,blocking,strict,notes,options,createdAt,resolvedAt
enum KitCommandKind
type KitCommandDescriptor:hash,kind,mutatesKit,sessionScoped,requiresConsensus,description
type KitChange:hash,id,kind,summary,origin,actor,session,transaction,forward,backward,validation,createdAt,appliedAt
enum KitCandidateStatus
enum CandidateVoteState
type KitCandidateVote:hash,session,state,reason,respondedAt,resolutionOptionId
type KitChangeCandidate:hash,id,kit,kind,summary,proposedBy,actor,transaction,status,requestedFrom,votes,validation,preview,proposedAt,expiresAt,decidedAt
enum TransactionState
type KitTransaction:hash,id,kit,label,state,startedBy,parent,startedAt,finalizedAt,abortedAt,changes,undoStack,redoStack,canUndo,canRedo,squashedChange
type KitHistoryEntry:hash,id,index,transaction,commandKinds,summary,squashedChangeCount,change,createdAt,finalizedAt,undoneAt
type KitHistoryPage:hash,nodes,pageInfo,totalCount
interface KitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt
interface ChangeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type SetSessionSelectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,mode,selection,previousSelection
type CreateAuthorKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,author
type UpdateAuthorKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,author,previousAuthor
type DeleteAuthorKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousAuthor
type CreateTypeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,type
type UpdateTypeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,type,previousType
type DeleteTypeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousType
type CreateDesignKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,design
type UpdateDesignKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,design,previousDesign
type DeleteDesignKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousDesign
type CreateQualityKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,quality
type UpdateQualityKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,quality,previousQuality
type DeleteQualityKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousQuality
type CreatePortKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,port
type UpdatePortKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,port,previousPort
type DeletePortKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousPort
type CreateFamilyKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,family
type UpdateFamilyKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,family,previousFamily
type DeleteFamilyKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousFamily
type CreateTagKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,tag
type UpdateTagKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,tag,previousTag
type DeleteTagKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousTag
type CreateConceptKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,concept
type UpdateConceptKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,concept,previousConcept
type DeleteConceptKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousConcept
type CreateFileKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,file
type UpdateFileKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,file,previousFile
type DeleteFileKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousFile
type CreateFolderKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,folder
type UpdateFolderKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,folder,previousFolder
type DeleteFolderKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,previousFolder
type MoveArtifactToFolderKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward,artifactKind,artifactId,folder,previousFolder
type CreatePieceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreatePiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type UpdatePieceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type UpdatePiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DeletePieceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DeletePiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreateConnectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreateConnectionsKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type UpdateConnectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type UpdateConnectionsKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DeleteConnectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DeleteConnectionsKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DeleteSelectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type FixPiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ClusterPiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ExpandDesignReferenceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type FlattenDesignKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type DragPiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type MovePiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreateFixedPieceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreateConnectedPieceKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type CreateHangingPiecesKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ChangePieceTypeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ChangePiecesTypeKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type PasteDesignSelectionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ImportKitKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ResetKitKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ExportKitKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type StartKitSessionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type HeartbeatKitSessionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type EndKitSessionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ReconnectKitSessionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type BeginKitTransactionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type FinalizeKitTransactionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type AbortKitTransactionKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type TransactionStepKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type HistoryStepKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type VoteOnKitChangeCandidateKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type ResolveKitConflictKitInteraction:id,hash,index,kit,kind,actor,session,transaction,candidate,change,conflict,summary,metadata,createdAt,forward,backward
type KitInteractionPage:hash,nodes,pageInfo,totalCount
type KitHistory:hash,canUndo,canRedo,totalCount,head
type KitStore:hash,kit,backbone,sessions,transactions,pendingCandidates,activeConflicts,validation,history,blockedByConflict,strictMode
enum ArtifactKind
enum SelectionMutationMode
type KitArchiveExport:hash,fileName,url,expiresAt
type KitMutationResult:hash,accepted,kind,summary,store,kit,session,transaction,candidate,change,historyEntry,conflict,validation,export
input KitCommandContextInput:kitId,sessionId,transactionId,origin,expectedHash,strictMode
input StartKitSessionInput:kitId,actor,client,strictMode
input HeartbeatKitSessionInput:kitId,sessionId,lastKnownHash
input EndKitSessionInput:kitId,sessionId
input ReconnectKitSessionInput:kitId,sessionId,client,lastKnownHash
input SetSessionSelectionCommandInput:context,mode,selection
input BeginKitTransactionInput:context,label,parentTransactionId
input FinalizeKitTransactionInput:context,transactionId
input AbortKitTransactionInput:context,transactionId
input TransactionStepInput:context,transactionId
input HistoryStepInput:context,steps
input VoteOnKitChangeCandidateInput:context,candidateId,state,reason,resolutionOptionId
input ResolveKitConflictInput:context,conflictId,optionId,payload
input CreateAuthorCommandInput:context,author
input UpdateAuthorCommandInput:context,id,patch
input DeleteAuthorCommandInput:context,id
input CreateTypeCommandInput:context,type
input UpdateTypeCommandInput:context,id,patch
input DeleteTypeCommandInput:context,id
input CreateDesignCommandInput:context,design
input UpdateDesignCommandInput:context,id,patch
input DeleteDesignCommandInput:context,id
input CreateQualityCommandInput:context,quality
input UpdateQualityCommandInput:context,id,patch
input DeleteQualityCommandInput:context,id
input CreatePortCommandInput:context,port
input UpdatePortCommandInput:context,id,patch
input DeletePortCommandInput:context,id
input CreateFamilyCommandInput:context,family
input UpdateFamilyCommandInput:context,id,patch
input DeleteFamilyCommandInput:context,id
input CreateTagCommandInput:context,tag
input UpdateTagCommandInput:context,id,patch
input DeleteTagCommandInput:context,id
input CreateConceptCommandInput:context,concept
input UpdateConceptCommandInput:context,id,patch
input DeleteConceptCommandInput:context,id
input CreateFileCommandInput:context,file
input UpdateFileCommandInput:context,id,patch
input DeleteFileCommandInput:context,id
input CreateFolderCommandInput:context,folder
input UpdateFolderCommandInput:context,id,patch
input DeleteFolderCommandInput:context,id
input MoveArtifactToFolderCommandInput:context,artifactKind,artifactId,folderId
input CreatePieceCommandInput:context,designId,piece
input CreatePiecesCommandInput:context,designId,pieces
input PieceUpdateInput:id,patch
input UpdatePieceCommandInput:context,designId,id,patch
input UpdatePiecesCommandInput:context,designId,updates
input DeletePieceCommandInput:context,designId,id
input DeletePiecesCommandInput:context,designId,ids
input CreateConnectionCommandInput:context,designId,connection
input CreateConnectionsCommandInput:context,designId,connections
input ConnectionUpdateInput:id,patch
input UpdateConnectionCommandInput:context,designId,id,patch
input UpdateConnectionsCommandInput:context,designId,updates
input DeleteConnectionCommandInput:context,designId,id
input DeleteConnectionsCommandInput:context,designId,ids
input DeleteSelectionCommandInput:context,designId,pieceIds,connectionIds
input FixPiecesCommandInput:context,designId,pieceIds
input ClusterPiecesCommandInput:context,designId,pieceIds,newDesignName
input ExpandDesignReferenceCommandInput:context,designId,referencedDesignId
input FlattenDesignCommandInput:context,designId
input DragPiecesCommandInput:context,designId,pieceIds,offset
input MovePiecesVectorInput:shift,gap,rise,rotation,turn,tilt
input MovePiecesCommandInput:context,designId,pieceIds,vector
input CreateFixedPieceCommandInput:context,designId,piece
input CreateConnectedPieceCommandInput:context,designId,piece,connection
input CreateHangingPiecesCommandInput:context,designId,pieces,parentPieceId,parentDesignPieceId,parentConnectorId,connectionTemplate
input ChangePieceTypeCommandInput:context,designId,pieceId,typeId
input ChangePiecesTypeCommandInput:context,designId,pieceIds,typeId
input PasteDesignSelectionCommandInput:context,designId,payload,offset
input ImportKitCommandInput:context,sourceUrl,archiveBase64
input ResetKitCommandInput:context,sourceUrl,archiveBase64,kit
input ExportKitCommandInput:context
type Query:kitCommandCatalog
type Mutation
enum KitStoreEventKind
type KitStoreEvent:hash,kind,store,interaction,change,candidate,conflict,session,transaction`;

type SchemaDefinition = { kind: string; name: string; fields: string[] };

function parseSchemaManifest(input: string): SchemaDefinition[] {
	return input
		.split(/\r?\n/)
		.map((line) => line.trim())
		.filter(Boolean)
		.map((line) => {
			const [head, tail] = line.split(":");
			const [kind, name] = head.split(/\s+/);
			return { kind, name, fields: tail ? tail.split(",").filter(Boolean) : [] };
		});
}

export const SCHEMA_DEFINITIONS = Object.freeze(parseSchemaManifest(SCHEMA_MANIFEST));
export const SCHEMA_DEFINITION_MAP = Object.freeze(Object.fromEntries(SCHEMA_DEFINITIONS.map((definition) => [definition.name, definition])));

// #endregion ⚛️Manifest

// #region ⚛️Types

export type SchemaHookTriad<T> = readonly [T, (next: SetStateAction<T>) => void, boolean];

export type SchemaPropertyEvent = {
	key: string;
	typeName: string;
	fieldName: string;
	guid?: string;
	previous: unknown;
	current: unknown;
};

export type MemoryBackboneConfig = {
	kind?: "memory";
	initialKit?: KitLike;
};

export type DevBackboneConfig = {
	kind: "dev";
	filePath: string;
};

export type LocalBackboneConfig = {
	kind: "local";
	folderPath: string;
};

export type RemoteBackboneConfig = {
	kind: "remote";
	serverUrl: string;
	sessionId?: string;
	kitName?: string;
	personId?: string;
	clientId?: string;
	authToken?: string;
	readOnly?: boolean;
};

export type KitProviderBackbone = MemoryBackboneConfig | DevBackboneConfig | LocalBackboneConfig | RemoteBackboneConfig;

type IndexedSchemaReference = {
	typeName: string;
	guid?: string;
	path: Array<string | number>;
	value: any;
};

type IndexedSchemaState = {
	plain: any;
	kit: KitImpl;
	kitGuid?: string;
	byGuid: Map<string, IndexedSchemaReference[]>;
	byType: Map<string, IndexedSchemaReference[]>;
};

type SchemaScope = {
	typeName: string;
	guid?: string;
	path: Array<string | number>;
};

type KitRuntimeContextValue = {
	store: KitStore;
	snapshot: KitStoreSnapshot;
	state: IndexedSchemaState;
	recentEvents: SchemaPropertyEvent[];
	canWrite: boolean;
	setFieldValue: (typeName: string, fieldName: string, next: SetStateAction<any>, guid?: string, scope?: SchemaScope | null) => void;
	setObjectValue: (typeName: string, next: SetStateAction<any>, guid?: string, scope?: SchemaScope | null) => void;
};

// #endregion ⚛️Types

// #region ⚛️Constants

const ROOT_COLLECTION_TYPE_BY_KEY: Record<string, string> = {
	types: "Type",
	designs: "Design",
	tags: "Tag",
	concepts: "Concept",
	families: "Family",
	ports: "Port",
	qualities: "Quality",
	files: "File",
	folders: "Folder",
	authors: "Author",
	pieces: "Piece",
	connections: "Connection",
	benchmarks: "Benchmark",
	representations: "Representation",
	connectors: "Connector",
	stats: "Stat",
	props: "Prop",
	layers: "Layer",
	groups: "Group",
	attributes: "Attribute",
	sessions: "KitSession",
	transactions: "KitTransaction",
	pendingCandidates: "KitChangeCandidate",
	activeConflicts: "KitConflict",
	activeTransactions: "KitTransaction",
	changes: "KitChange",
	undoStack: "KitChange",
	redoStack: "KitChange",
	votes: "KitCandidateVote",
	requestedFrom: "KitSession",
	actions: "SessionWarningAction",
	nodes: "KitHistoryEntry",
};

const NESTED_TYPE_BY_KEY: Record<string, string> = {
	plane: "Plane",
	mirrorPlane: "Plane",
	flatPlane: "Plane",
	center: "Coordinate",
	flatCenter: "Coordinate",
	offset: "Coordinate",
	origin: "Point",
	point: "Point",
	position: "Point",
	xAxis: "Vector",
	yAxis: "Vector",
	forward: "Vector",
	up: "Vector",
	direction: "Vector",
	connected: "Side",
	connecting: "Side",
	piece: "Piece",
	designPiece: "Piece",
	parentPiece: "Piece",
	childPiece: "Piece",
	activeDesign: "Design",
	type: "Type",
	design: "Design",
	quality: "Quality",
	folder: "Folder",
	createdBy: "Author",
	updatedBy: "Author",
	port: "Port",
	connector: "Connector",
	childConnector: "Connector",
	parentConnector: "Connector",
	actor: "Actor",
	session: "KitSession",
	client: "KitClientInfo",
	warning: "KitSessionWarning",
	selection: "KitSessionSelection",
	validation: "KitValidationResult",
	candidate: "KitChangeCandidate",
	conflict: "KitConflict",
	change: "KitChange",
	transaction: "KitTransaction",
	store: "KitStore",
	history: "KitHistory",
	backbone: "KitBackbone",
	historyEntry: "KitHistoryEntry",
	export: "KitArchiveExport",
	pageInfo: "PageInfo",
};

const NEVER_WRITABLE_FIELDS = new Set([
	"hash",
	"kind",
	"flatPlane",
	"flatCenter",
	"parentPiece",
	"parentConnection",
	"childPieces",
	"childConnections",
	"alternatives",
	"alternativeTypes",
	"alternativeDesigns",
	"childPiece",
	"childConnector",
	"parentPiece",
	"parentConnector",
	"fixedPieces",
]);

// #endregion ⚛️Constants

// #region ⚛️Utilities

function noop(): void {}

function deepClone<T>(value: T): T {
	return JSON.parse(JSON.stringify(value));
}

function deepEqual(a: any, b: any): boolean {
	if (a === b) return true;
	if (a == null || b == null) return a == null && b == null;
	if (typeof a !== typeof b) return false;
	if (Array.isArray(a)) {
		if (!Array.isArray(b) || a.length !== b.length) return false;
		for (let index = 0; index < a.length; index += 1) {
			if (!deepEqual(a[index], b[index])) return false;
		}
		return true;
	}
	if (typeof a === "object") {
		const keysA = Object.keys(a);
		const keysB = Object.keys(b);
		if (keysA.length !== keysB.length) return false;
		for (const key of keysA) {
			if (!deepEqual(a[key], b[key])) return false;
		}
		return true;
	}
	return false;
}

function pascalCase(value: string): string {
	return value.length === 0 ? value : value[0].toUpperCase() + value.slice(1);
}

function getFieldDataKey(typeName: string, fieldName: string): string {
	if (fieldName === "id") return "guid";
	if (typeName === "Kit" && fieldName === "release") return "version";
	return fieldName;
}

function getByPath(root: any, path: Array<string | number>): any {
	let current = root;
	for (const segment of path) {
		if (current == null) return undefined;
		current = current[segment as any];
	}
	return current;
}

function setByPath(root: any, path: Array<string | number>, value: any): void {
	if (path.length === 0) return;
	const parent = getByPath(root, path.slice(0, -1));
	if (parent == null) return;
	parent[path[path.length - 1] as any] = value;
}

function inferTypeName(parentTypeName: string | undefined, key: string | undefined): string | undefined {
	if (!key) return parentTypeName;
	if (ROOT_COLLECTION_TYPE_BY_KEY[key]) return ROOT_COLLECTION_TYPE_BY_KEY[key];
	if (NESTED_TYPE_BY_KEY[key]) return NESTED_TYPE_BY_KEY[key];
	return parentTypeName;
}

function scanSchemaState(root: any): IndexedSchemaState {
	const byGuid = new Map<string, IndexedSchemaReference[]>();
	const byType = new Map<string, IndexedSchemaReference[]>();

	function push(ref: IndexedSchemaReference): void {
		if (ref.guid) {
			const existing = byGuid.get(ref.guid) ?? [];
			existing.push(ref);
			byGuid.set(ref.guid, existing);
		}
		const existing = byType.get(ref.typeName) ?? [];
		existing.push(ref);
		byType.set(ref.typeName, existing);
	}

	function walk(value: any, path: Array<string | number>, typeName: string | undefined): void {
		if (value == null) return;
		if (Array.isArray(value)) {
			const collectionName = typeof path[path.length - 1] === "string" ? (path[path.length - 1] as string) : undefined;
			const childTypeName = inferTypeName(typeName, collectionName);
			value.forEach((entry, index) => walk(entry, [...path, index], childTypeName));
			return;
		}
		if (typeof value !== "object") return;
		const resolvedTypeName = typeName ?? "Kit";
		const guidValue = typeof value.guid === "string" ? value.guid : undefined;
		push({ typeName: resolvedTypeName, guid: guidValue, path, value });
		for (const [key, entry] of Object.entries(value)) {
			walk(entry, [...path, key], inferTypeName(resolvedTypeName, key));
		}
	}

	walk(root, [], "Kit");

	return {
		plain: root,
		kit: asKitInstance(root),
		kitGuid: root?.guid,
		byGuid,
		byType,
	};
}

function collectGuids(value: any, target: Set<string>): void {
	if (value == null) return;
	if (Array.isArray(value)) {
		for (const entry of value) collectGuids(entry, target);
		return;
	}
	if (typeof value !== "object") return;
	if (typeof value.guid === "string") target.add(value.guid);
	for (const entry of Object.values(value)) collectGuids(entry, target);
}

function resolveReference(index: IndexedSchemaState, typeName: string, guid?: string, scope?: SchemaScope | null): IndexedSchemaReference | undefined {
	if (typeName === "Kit") return index.byType.get("Kit")?.[0];
	if (guid) {
		const matches = index.byGuid.get(guid) ?? [];
		return matches.find((entry) => entry.typeName === typeName) ?? matches[0];
	}
	if (scope && scope.typeName === typeName) {
		return { typeName, guid: scope.guid, path: scope.path, value: getByPath(index.plain, scope.path) };
	}
	const typeMatches = index.byType.get(typeName) ?? [];
	if (typeMatches.length === 1) return typeMatches[0];
	return undefined;
}

function findLivePiece(kit: KitImpl, pieceGuid: string): { piece: Piece; design: Design } | undefined {
	for (const design of kit.designs ?? []) {
		const piece = design.pieces?.find((entry) => entry.guid === pieceGuid);
		if (piece) return { piece, design };
	}
	return undefined;
}

function findLiveConnection(kit: KitImpl, connectionGuid: string): { connection: any; design: Design } | undefined {
	for (const design of kit.designs ?? []) {
		const connection = design._connections?.find((entry) => entry.guid === connectionGuid);
		if (connection) return { connection, design };
	}
	return undefined;
}

function findLiveEntity(kit: KitImpl, typeName: string, guid?: string): any {
	if (typeName === "Kit") return kit;
	if (!guid) return undefined;
	if (typeName === "Piece") return findLivePiece(kit, guid)?.piece;
	if (typeName === "Connection") return findLiveConnection(kit, guid)?.connection;
	if (typeName === "Type") return kit.findType(guid);
	if (typeName === "Design") return kit.findDesign(guid);
	if (typeName === "Port") return kit.ports?.find((entry) => entry.guid === guid);
	if (typeName === "Quality") return kit.qualities?.find((entry) => entry.guid === guid);
	if (typeName === "File") return kit.files?.find((entry) => entry.guid === guid);
	if (typeName === "Folder") return kit.folders?.find((entry) => entry.guid === guid);
	if (typeName === "Author") return kit.authors?.find((entry) => entry.guid === guid);
	if (typeName === "Tag") return kit.tags?.find((entry) => entry.guid === guid);
	if (typeName === "Concept") return kit.concepts?.find((entry) => entry.guid === guid);
	if (typeName === "Family") return kit.families?.find((entry) => entry.guid === guid);
	if (typeName === "Representation") {
		for (const entry of kit.types ?? []) {
			const match = entry.representations?.find((representation) => representation.guid === guid);
			if (match) return match;
		}
	}
	if (typeName === "Connector") {
		for (const entry of kit.types ?? []) {
			const match = entry.connectors?.find((connector) => connector.guid === guid);
			if (match) return match;
		}
	}
	if (typeName === "Benchmark") {
		for (const entry of kit.qualities ?? []) {
			const match = entry.benchmarks?.find((benchmark) => benchmark.guid === guid);
			if (match) return match;
		}
	}
	return undefined;
}

function readCustomFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, guid?: string): any {
	if (typeName === "Kit" && fieldName === "release") return (state.kit as any).version;
	if (typeName === "Piece") {
		const found = guid ? findLivePiece(state.kit, guid) : undefined;
		if (!found) return undefined;
		const { piece, design } = found;
		if (fieldName === "kind") return piece.wireDesignAsPieceId() ? "DESIGN" : piece.wireTypeId() ? "TYPE" : undefined;
		if (fieldName === "flatPlane") return piece.flatPlane();
		if (fieldName === "flatCenter") return piece.flatCenter();
		if (fieldName === "parentPiece") {
			try {
				return state.kit.findParentPieceInDesign(design.guid, piece.guid);
			} catch {
				return undefined;
			}
		}
		if (fieldName === "parentConnection") {
			try {
				return state.kit.findParentConnectionForPieceInDesign(design.guid, piece.guid);
			} catch {
				return undefined;
			}
		}
		if (fieldName === "childPieces") {
			try {
				return state.kit.findChildrenPiecesInDesign(design.guid, piece.guid);
			} catch {
				return [];
			}
		}
		if (fieldName === "childConnections") {
			try {
				const metadata = state.kit.piecesMetadataFor(design.guid);
				if (!metadata.ok || !metadata.diff) return [];
				return (design._connections ?? []).filter((connection) => {
					try {
						const connectedGuid = connection.connected.wirePieceId().guid;
						const connectingGuid = connection.connecting.wirePieceId().guid;
						if (connectedGuid === piece.guid) return metadata.diff.get(connectingGuid)?.parentPieceId === piece.guid;
						if (connectingGuid === piece.guid) return metadata.diff.get(connectedGuid)?.parentPieceId === piece.guid;
						return false;
					} catch {
						return false;
					}
				});
			} catch {
				return [];
			}
		}
		if (fieldName === "alternativeTypes") return piece.alternativeTypes();
		if (fieldName === "alternativeDesigns") {
			const nestedDesign = piece.design;
			if (!nestedDesign || typeof nestedDesign.getDesignFamily !== "function") return [];
			try {
				return nestedDesign.getDesignFamily().filter((entry) => entry.guid !== nestedDesign.guid);
			} catch {
				return [];
			}
		}
		if (fieldName === "alternatives") {
			return [
				...((piece.alternativeTypes() ?? []).map((entry) => ({ type: entry, design: undefined }))),
				...((readCustomFieldValue(state, typeName, "alternativeDesigns", guid) ?? []).map((entry: any) => ({ type: undefined, design: entry }))),
			];
		}
	}
	if (typeName === "Connection") {
		const found = guid ? findLiveConnection(state.kit, guid) : undefined;
		if (!found) return undefined;
		const { connection } = found;
		if (fieldName === "childPiece") return connection.connecting?.piece;
		if (fieldName === "parentPiece") return connection.connected?.piece;
		if (fieldName === "childConnector") return connection.connecting?.connector;
		if (fieldName === "parentConnector") return connection.connected?.connector;
	}
	if (typeName === "Type" && fieldName === "fixedPieces") {
		const liveType = guid ? state.kit.findType(guid) : undefined;
		if (!liveType) return [];
		const pieces: Piece[] = [];
		for (const design of state.kit.designs ?? []) {
			for (const piece of design.pieces ?? []) {
				if (piece.wireTypeId()?.guid === liveType.guid) pieces.push(piece);
			}
		}
		return pieces;
	}
	return undefined;
}

function readSchemaFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, guid?: string, scope?: SchemaScope | null): any {
	const custom = readCustomFieldValue(state, typeName, fieldName, guid);
	if (custom !== undefined) return custom;
	const ref = resolveReference(state, typeName, guid, scope);
	if (!ref) return undefined;
	const key = getFieldDataKey(typeName, fieldName);
	return ref.value?.[key];
}

function isWritableField(state: IndexedSchemaState, typeName: string, fieldName: string, guid?: string, scope?: SchemaScope | null): boolean {
	if (NEVER_WRITABLE_FIELDS.has(fieldName)) return false;
	const ref = resolveReference(state, typeName, guid, scope);
	if (!ref) return false;
	const key = getFieldDataKey(typeName, fieldName);
	if (fieldName === "hash") return false;
	return ref.value != null && (Object.prototype.hasOwnProperty.call(ref.value, key) || ref.value[key] !== undefined);
}

function normalizeNextValue(current: any, fieldName: string, next: any): any {
	if (typeof next === "string" && current && typeof current === "object" && "guid" in current) {
		return { guid: next };
	}
	if ((fieldName === "type" || fieldName === "design" || fieldName === "piece" || fieldName === "designPiece" || fieldName === "connector") && typeof next === "string") {
		return { guid: next };
	}
	return next;
}

function nextValueFromAction<T>(current: T, next: SetStateAction<T>): T {
	return typeof next === "function" ? (next as (value: T) => T)(current) : next;
}

function normalizeStateInput(input: KitStoreSnapshot | KitLike | IndexedSchemaState): IndexedSchemaState {
	if ((input as IndexedSchemaState).byGuid instanceof Map) return input as IndexedSchemaState;
	if ((input as KitStoreSnapshot).kit) {
		const snapshot = input as KitStoreSnapshot;
		return scanSchemaState(snapshot.kit.toJSON());
	}
	const kit = asKitInstance(input as KitLike);
	return scanSchemaState(kit.toJSON());
}

export function diffSchemaPropertyEvents(previousInput: KitStoreSnapshot | KitLike | IndexedSchemaState, nextInput: KitStoreSnapshot | KitLike | IndexedSchemaState): SchemaPropertyEvent[] {
	const previous = normalizeStateInput(previousInput);
	const next = normalizeStateInput(nextInput);
	const dirtyGuids = new Set<string>();
	const allGuids = new Set<string>([...(previous.byGuid.keys() ?? []), ...(next.byGuid.keys() ?? [])]);

	for (const guidValue of allGuids) {
		const previousRef = (previous.byGuid.get(guidValue) ?? [])[0];
		const nextRef = (next.byGuid.get(guidValue) ?? [])[0];
		if (!deepEqual(previousRef?.value, nextRef?.value)) {
			dirtyGuids.add(guidValue);
			collectGuids(previousRef?.value, dirtyGuids);
			collectGuids(nextRef?.value, dirtyGuids);
		}
	}

	const events: SchemaPropertyEvent[] = [];
	for (const guidValue of dirtyGuids) {
		const previousRef = (previous.byGuid.get(guidValue) ?? [])[0];
		const nextRef = (next.byGuid.get(guidValue) ?? [])[0];
		const typeName = nextRef?.typeName ?? previousRef?.typeName;
		if (!typeName) continue;
		const definition = SCHEMA_DEFINITION_MAP[typeName];
		if (!definition) continue;
		for (const fieldName of definition.fields) {
			const previousValue = readSchemaFieldValue(previous, typeName, fieldName, guidValue);
			const nextValue = readSchemaFieldValue(next, typeName, fieldName, guidValue);
			if (!deepEqual(previousValue, nextValue)) {
				events.push({ key: `${typeName}.${fieldName}`, typeName, fieldName, guid: guidValue, previous: previousValue, current: nextValue });
			}
		}
	}

	if (!deepEqual(previous.plain, next.plain) && next.kitGuid) {
		for (const fieldName of SCHEMA_DEFINITION_MAP.Kit?.fields ?? []) {
			const previousValue = readSchemaFieldValue(previous, "Kit", fieldName, previous.kitGuid);
			const nextValue = readSchemaFieldValue(next, "Kit", fieldName, next.kitGuid);
			if (!deepEqual(previousValue, nextValue)) {
				events.push({ key: `Kit.${fieldName}`, typeName: "Kit", fieldName, guid: next.kitGuid, previous: previousValue, current: nextValue });
			}
		}
	}

	return events;
}

async function createNodeJsonFileAdapter(filePath: string) {
	const fs = await import("node:fs/promises");
	const path = await import("node:path");
	return {
		async read() {
			try {
				return await fs.readFile(filePath, "utf8");
			} catch {
				return null;
			}
		},
		async write(json: string) {
			await fs.mkdir(path.dirname(filePath), { recursive: true });
			await fs.writeFile(filePath, json, "utf8");
		},
	};
}

async function createNodeFolderAdapter(folderPath: string) {
	const fs = await import("node:fs/promises");
	const syncFs = await import("node:fs");
	const path = await import("node:path");
	const kitDbPath = path.join(folderPath, ".semio", "kit.db");

	async function listRecursive(currentPath: string, prefix: string = ""): Promise<string[]> {
		try {
			const entries = await fs.readdir(currentPath, { withFileTypes: true });
			const files: string[] = [];
			for (const entry of entries) {
				const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
				const absolute = path.join(currentPath, entry.name);
				if (entry.isDirectory()) {
					files.push(...(await listRecursive(absolute, relative)));
				} else {
					if (relative !== ".semio/kit.db") files.push(relative.replace(/\\/g, "/"));
				}
			}
			return files;
		} catch {
			return [];
		}
	}

	return {
		async readKit() {
			try {
				return new Uint8Array(await fs.readFile(kitDbPath));
			} catch {
				return null;
			}
		},
		async writeKit(data: Uint8Array) {
			await fs.mkdir(path.dirname(kitDbPath), { recursive: true });
			await fs.writeFile(kitDbPath, data);
		},
		async readFile(relativePath: string) {
			try {
				const data = await fs.readFile(path.join(folderPath, relativePath));
				return new Blob([data]);
			} catch {
				return null;
			}
		},
		async writeFile(relativePath: string, blob: Blob) {
			const absolutePath = path.join(folderPath, relativePath);
			await fs.mkdir(path.dirname(absolutePath), { recursive: true });
			await fs.writeFile(absolutePath, new Uint8Array(await blob.arrayBuffer()));
		},
		async deleteFile(relativePath: string) {
			await fs.rm(path.join(folderPath, relativePath), { force: true });
		},
		async createDirectory(relativePath: string) {
			await fs.mkdir(path.join(folderPath, relativePath), { recursive: true });
		},
		async moveEntry(fromPath: string, toPath: string) {
			await fs.mkdir(path.dirname(path.join(folderPath, toPath)), { recursive: true });
			await fs.rename(path.join(folderPath, fromPath), path.join(folderPath, toPath));
		},
		async listFiles() {
			await fs.mkdir(folderPath, { recursive: true });
			return listRecursive(folderPath);
		},
		watch(callback: () => void) {
			const watcher = syncFs.watch(folderPath, { recursive: true }, () => callback());
			return () => watcher.close();
		},
	};
}

async function loadSketchpadModule() {
	const sketchpadModuleId = "@semio" + "/sketchpad";
	return import(/* @vite-ignore */ sketchpadModuleId);
}

async function createStoreFromBackbone(backbone: KitProviderBackbone | undefined, initialKit?: KitLike): Promise<KitStore> {
	const resolvedBackbone = backbone?.kind ? backbone : ({ kind: "memory", initialKit } as MemoryBackboneConfig);
	if (resolvedBackbone.kind === "memory") {
		const seed = resolvedBackbone.initialKit ?? initialKit ?? { guid: guid(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() };
		return new InMemoryKitStore(asKitInstance(seed));
	}
	if (resolvedBackbone.kind === "dev") {
		const { createJsonFileKitStore } = await loadSketchpadModule();
		return createJsonFileKitStore(await createNodeJsonFileAdapter(resolvedBackbone.filePath));
	}
	if (resolvedBackbone.kind === "local") {
		const { createFolderKitStore } = await loadSketchpadModule();
		return createFolderKitStore(await createNodeFolderAdapter(resolvedBackbone.folderPath), initialKit ? asKitInstance(initialKit).toJSON() as any : undefined);
	}
	const { createSessionKitStore } = await loadSketchpadModule();
	return createSessionKitStore({
		serverUrl: resolvedBackbone.serverUrl,
		sessionId: resolvedBackbone.sessionId,
		kitName: resolvedBackbone.kitName,
		personId: resolvedBackbone.personId,
		clientId: resolvedBackbone.clientId,
		authToken: resolvedBackbone.authToken,
		readOnly: resolvedBackbone.readOnly,
	});
}

// #endregion ⚛️Utilities

// #region ⚛️Context

const KitRuntimeContext = React.createContext<KitRuntimeContextValue | null>(null);
const SchemaScopeContext = React.createContext<SchemaScope | null>(null);

function useKitRuntime(): KitRuntimeContextValue {
	const runtime = React.useContext(KitRuntimeContext);
	if (!runtime) throw new Error("semio/react hooks must be used inside <KitProvider>.");
	return runtime;
}

export type KitProviderProps = {
	store?: KitStore;
	backbone?: KitProviderBackbone;
	initialKit?: KitLike;
	children: ReactNode;
	fallback?: ReactNode;
};

export function KitProvider({ store: externalStore, backbone, initialKit, children, fallback = null }: KitProviderProps): React.ReactElement | null {
	const [internalStore, setInternalStore] = React.useState<KitStore | null>(externalStore ?? null);

	React.useEffect(() => {
		if (externalStore) {
			setInternalStore(externalStore);
			return;
		}
		let disposed = false;
		createStoreFromBackbone(backbone, initialKit).then((store) => {
			if (!disposed) setInternalStore(store);
		});
		return () => {
			disposed = true;
		};
	}, [externalStore, backbone, initialKit]);

	const store = externalStore ?? internalStore;
	if (!store) return React.createElement(React.Fragment, null, fallback);

	const snapshot = React.useSyncExternalStore(
		React.useCallback((listener) => store.subscribe(listener), [store]),
		React.useCallback(() => store.getSnapshot(), [store]),
		React.useCallback(() => store.getSnapshot(), [store]),
	);

	const state = React.useMemo(() => scanSchemaState(snapshot.kit.toJSON()), [snapshot]);
	const previousStateRef = React.useRef<IndexedSchemaState | null>(null);
	const [recentEvents, setRecentEvents] = React.useState<SchemaPropertyEvent[]>([]);

	React.useEffect(() => {
		const previous = previousStateRef.current;
		if (previous) {
			const nextEvents = diffSchemaPropertyEvents(previous, state);
			if (nextEvents.length > 0) {
				setRecentEvents((existing) => [...existing, ...nextEvents].slice(-500));
			}
		}
		previousStateRef.current = state;
	}, [state]);

	const setFieldValue = React.useCallback((typeName: string, fieldName: string, next: SetStateAction<any>, guidValue?: string, scope?: SchemaScope | null) => {
		const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
		if (!isWritableField(currentState, typeName, fieldName, guidValue, scope)) return;
		const ref = resolveReference(currentState, typeName, guidValue, scope);
		if (!ref) return;
		const key = getFieldDataKey(typeName, fieldName);
		const clone = deepClone(currentState.plain);
		const currentObject = getByPath(clone, ref.path);
		const currentValue = currentObject?.[key];
		currentObject[key] = normalizeNextValue(currentValue, fieldName, nextValueFromAction(currentValue, next));
		store.replace(asKitInstance(clone));
	}, [store]);

	const setObjectValue = React.useCallback((typeName: string, next: SetStateAction<any>, guidValue?: string, scope?: SchemaScope | null) => {
		const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
		const ref = resolveReference(currentState, typeName, guidValue, scope);
		if (!ref) return;
		const clone = deepClone(currentState.plain);
		const currentValue = getByPath(clone, ref.path);
		setByPath(clone, ref.path, nextValueFromAction(currentValue, next));
		store.replace(asKitInstance(clone));
	}, [store]);

	const value = React.useMemo<KitRuntimeContextValue>(() => ({
		store,
		snapshot,
		state,
		recentEvents,
		canWrite: !snapshot.sync.readonly,
		setFieldValue,
		setObjectValue,
	}), [store, snapshot, state, recentEvents, setFieldValue, setObjectValue]);

	return React.createElement(KitRuntimeContext.Provider, { value }, children);
}

export function createEntityProvider(typeName: string) {
	return function EntityProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
		const runtime = useKitRuntime();
		const parentScope = React.useContext(SchemaScopeContext);
		const ref = resolveReference(runtime.state, typeName, guidValue, parentScope);
		const scope: SchemaScope = ref ? { typeName, guid: ref.guid, path: ref.path } : { typeName, guid: guidValue, path: [] };
		return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
	};
}

export const PieceProvider = createEntityProvider("Piece");
export const TypeProvider = createEntityProvider("Type");
export const DesignProvider = createEntityProvider("Design");
export const ConnectionProvider = createEntityProvider("Connection");
export const PortProvider = createEntityProvider("Port");
export const QualityProvider = createEntityProvider("Quality");
export const FileProvider = createEntityProvider("File");
export const FolderProvider = createEntityProvider("Folder");
export const AuthorProvider = createEntityProvider("Author");
export const TagProvider = createEntityProvider("Tag");
export const ConceptProvider = createEntityProvider("Concept");
export const FamilyProvider = createEntityProvider("Family");
export const RepresentationProvider = createEntityProvider("Representation");
export const ConnectorProvider = createEntityProvider("Connector");
export const BenchmarkProvider = createEntityProvider("Benchmark");
export const LayerProvider = createEntityProvider("Layer");
export const GroupProvider = createEntityProvider("Group");
export const StatProvider = createEntityProvider("Stat");
export const PropProvider = createEntityProvider("Prop");
export const AttributeProvider = createEntityProvider("Attribute");

// #endregion ⚛️Context

// #region ⚛️Core Hooks

export function useSchemaEvents(filter?: Partial<Pick<SchemaPropertyEvent, "typeName" | "fieldName" | "guid" | "key">>): SchemaPropertyEvent[] {
	const runtime = useKitRuntime();
	return React.useMemo(() => {
		if (!filter) return runtime.recentEvents;
		return runtime.recentEvents.filter((event) => {
			if (filter.typeName && event.typeName !== filter.typeName) return false;
			if (filter.fieldName && event.fieldName !== filter.fieldName) return false;
			if (filter.guid && event.guid !== filter.guid) return false;
			if (filter.key && event.key !== filter.key) return false;
			return true;
		});
	}, [runtime.recentEvents, filter]);
}

export function useKitStore(): SchemaHookTriad<KitStore> {
	const runtime = useKitRuntime();
	return [runtime.store, noop, false] as const;
}

export function useKitSnapshot(): SchemaHookTriad<KitStoreSnapshot> {
	const runtime = useKitRuntime();
	return [runtime.snapshot, noop, false] as const;
}

export function createSchemaObjectHook(typeName: string) {
	return function useSchemaObject(guidValue?: string): SchemaHookTriad<any> {
		const runtime = useKitRuntime();
		const scope = React.useContext(SchemaScopeContext);
		const ref = resolveReference(runtime.state, typeName, guidValue, scope);
		const value = ref?.value;
		const canSet = runtime.canWrite && !!ref;
		const setValue = React.useCallback((next: SetStateAction<any>) => runtime.setObjectValue(typeName, next, guidValue, scope), [runtime, typeName, guidValue, scope]);
		return [value, setValue, canSet] as const;
	};
}

export function createSchemaFieldHook(typeName: string, fieldName: string) {
	return function useSchemaField(guidValue?: string): SchemaHookTriad<any> {
		const runtime = useKitRuntime();
		const scope = React.useContext(SchemaScopeContext);
		const value = readSchemaFieldValue(runtime.state, typeName, fieldName, guidValue, scope);
		const canSet = runtime.canWrite && isWritableField(runtime.state, typeName, fieldName, guidValue, scope);
		const setValue = React.useCallback((next: SetStateAction<any>) => runtime.setFieldValue(typeName, fieldName, next, guidValue, scope), [runtime, typeName, fieldName, guidValue, scope]);
		return [value, setValue, canSet] as const;
	};
}

const schemaHookCache = new Map<string, any>();

function createHookNameLookup(): Map<string, { typeName: string; fieldName?: string }> {
	const map = new Map<string, { typeName: string; fieldName?: string }>();
	for (const definition of SCHEMA_DEFINITIONS) {
		map.set(`use${definition.name}`, { typeName: definition.name });
		for (const fieldName of definition.fields) {
			map.set(`use${definition.name}${pascalCase(fieldName)}`, { typeName: definition.name, fieldName });
		}
	}
	return map;
}

const schemaHookNameLookup = createHookNameLookup();

export const schemaHooks = new Proxy({} as Record<string, any>, {
	get(_target, property) {
		const name = String(property);
		if (schemaHookCache.has(name)) return schemaHookCache.get(name);
		const entry = schemaHookNameLookup.get(name);
		if (!entry) return undefined;
		const hook = entry.fieldName ? createSchemaFieldHook(entry.typeName, entry.fieldName) : createSchemaObjectHook(entry.typeName);
		schemaHookCache.set(name, hook);
		return hook;
	},
});

export function useSchemaHook(hookName: string, guidValue?: string): SchemaHookTriad<any> {
	const hook = (schemaHooks as any)[hookName];
	if (typeof hook !== "function") return [undefined, noop, false] as const;
	return hook(guidValue);
}

export const SCHEMA_EVENT_KEYS = Object.freeze(
	Object.fromEntries(
		SCHEMA_DEFINITIONS.flatMap((definition) => definition.fields.map((fieldName) => [`${definition.name}.${fieldName}`, `${definition.name}.${fieldName}`])),
	),
);

// #endregion ⚛️Core Hooks

// #region ⚛️Direct Domain Exports

export const useActor = createSchemaObjectHook("Actor"), useActorId = createSchemaFieldHook("Actor", "id"), useActorName = createSchemaFieldHook("Actor", "name"), useActorEmail = createSchemaFieldHook("Actor", "email"), useActorColor = createSchemaFieldHook("Actor", "color");
export const useUser = createSchemaObjectHook("User"), useUserHash = createSchemaFieldHook("User", "hash"), useUserId = createSchemaFieldHook("User", "id"), useUserName = createSchemaFieldHook("User", "name"), useUserEmail = createSchemaFieldHook("User", "email"), useUserColor = createSchemaFieldHook("User", "color");
export const useAgent = createSchemaObjectHook("Agent"), useAgentHash = createSchemaFieldHook("Agent", "hash"), useAgentId = createSchemaFieldHook("Agent", "id"), useAgentLlm = createSchemaFieldHook("Agent", "llm"), useAgentName = createSchemaFieldHook("Agent", "name"), useAgentEmail = createSchemaFieldHook("Agent", "email"), useAgentColor = createSchemaFieldHook("Agent", "color");
export const useCoordinate = createSchemaObjectHook("Coordinate"), useCoordinateHash = createSchemaFieldHook("Coordinate", "hash"), useCoordinateU = createSchemaFieldHook("Coordinate", "u"), useCoordinateV = createSchemaFieldHook("Coordinate", "v");
export const usePoint = createSchemaObjectHook("Point"), usePointHash = createSchemaFieldHook("Point", "hash"), usePointX = createSchemaFieldHook("Point", "x"), usePointY = createSchemaFieldHook("Point", "y"), usePointZ = createSchemaFieldHook("Point", "z");
export const useVector = createSchemaObjectHook("Vector"), useVectorHash = createSchemaFieldHook("Vector", "hash"), useVectorX = createSchemaFieldHook("Vector", "x"), useVectorY = createSchemaFieldHook("Vector", "y"), useVectorZ = createSchemaFieldHook("Vector", "z");
export const usePlane = createSchemaObjectHook("Plane"), usePlaneHash = createSchemaFieldHook("Plane", "hash"), usePlaneOrigin = createSchemaFieldHook("Plane", "origin"), usePlaneXAxis = createSchemaFieldHook("Plane", "xAxis"), usePlaneYAxis = createSchemaFieldHook("Plane", "yAxis");
export const useCamera = createSchemaObjectHook("Camera"), useCameraHash = createSchemaFieldHook("Camera", "hash"), useCameraPosition = createSchemaFieldHook("Camera", "position"), useCameraForward = createSchemaFieldHook("Camera", "forward"), useCameraUp = createSchemaFieldHook("Camera", "up");
export const useAttribute = createSchemaObjectHook("Attribute"), useAttributeHash = createSchemaFieldHook("Attribute", "hash"), useAttributeId = createSchemaFieldHook("Attribute", "id"), useAttributeKey = createSchemaFieldHook("Attribute", "key"), useAttributeValue = createSchemaFieldHook("Attribute", "value"), useAttributeDefinition = createSchemaFieldHook("Attribute", "definition");
export const useLocation = createSchemaObjectHook("Location"), useLocationHash = createSchemaFieldHook("Location", "hash"), useLocationLongitude = createSchemaFieldHook("Location", "longitude"), useLocationLatitude = createSchemaFieldHook("Location", "latitude"), useLocationAltitude = createSchemaFieldHook("Location", "altitude"), useLocationAttributes = createSchemaFieldHook("Location", "attributes");
export const useAuthor = createSchemaObjectHook("Author"), useAuthorHash = createSchemaFieldHook("Author", "hash"), useAuthorId = createSchemaFieldHook("Author", "id"), useAuthorName = createSchemaFieldHook("Author", "name"), useAuthorEmail = createSchemaFieldHook("Author", "email"), useAuthorAttributes = createSchemaFieldHook("Author", "attributes");
export const useFolder = createSchemaObjectHook("Folder"), useFolderHash = createSchemaFieldHook("Folder", "hash"), useFolderId = createSchemaFieldHook("Folder", "id"), useFolderKit = createSchemaFieldHook("Folder", "kit"), useFolderName = createSchemaFieldHook("Folder", "name"), useFolderParent = createSchemaFieldHook("Folder", "parent"), useFolderChildren = createSchemaFieldHook("Folder", "children"), useFolderDescription = createSchemaFieldHook("Folder", "description"), useFolderAttributes = createSchemaFieldHook("Folder", "attributes"), useFolderCreatedAt = createSchemaFieldHook("Folder", "createdAt"), useFolderCreatedBy = createSchemaFieldHook("Folder", "createdBy"), useFolderUpdatedAt = createSchemaFieldHook("Folder", "updatedAt"), useFolderUpdatedBy = createSchemaFieldHook("Folder", "updatedBy");
export const useFile = createSchemaObjectHook("File"), useFileHash = createSchemaFieldHook("File", "hash"), useFileId = createSchemaFieldHook("File", "id"), useFileKit = createSchemaFieldHook("File", "kit"), useFileName = createSchemaFieldHook("File", "name"), useFileRemote = createSchemaFieldHook("File", "remote"), useFileFolder = createSchemaFieldHook("File", "folder"), useFileSize = createSchemaFieldHook("File", "size"), useFileContentHash = createSchemaFieldHook("File", "contentHash"), useFileBlob = createSchemaFieldHook("File", "blob"), useFileMime = createSchemaFieldHook("File", "mime"), useFileCreatedAt = createSchemaFieldHook("File", "createdAt"), useFileCreatedBy = createSchemaFieldHook("File", "createdBy"), useFileUpdatedAt = createSchemaFieldHook("File", "updatedAt"), useFileUpdatedBy = createSchemaFieldHook("File", "updatedBy");
export const useBenchmark = createSchemaObjectHook("Benchmark"), useBenchmarkHash = createSchemaFieldHook("Benchmark", "hash"), useBenchmarkId = createSchemaFieldHook("Benchmark", "id"), useBenchmarkQuality = createSchemaFieldHook("Benchmark", "quality"), useBenchmarkName = createSchemaFieldHook("Benchmark", "name"), useBenchmarkIcon = createSchemaFieldHook("Benchmark", "icon"), useBenchmarkMin = createSchemaFieldHook("Benchmark", "min"), useBenchmarkMinExcluded = createSchemaFieldHook("Benchmark", "minExcluded"), useBenchmarkMax = createSchemaFieldHook("Benchmark", "max"), useBenchmarkMaxExcluded = createSchemaFieldHook("Benchmark", "maxExcluded"), useBenchmarkAttributes = createSchemaFieldHook("Benchmark", "attributes");
export const useQuality = createSchemaObjectHook("Quality"), useQualityHash = createSchemaFieldHook("Quality", "hash"), useQualityId = createSchemaFieldHook("Quality", "id"), useQualityKit = createSchemaFieldHook("Quality", "kit"), useQualityKey = createSchemaFieldHook("Quality", "key"), useQualityName = createSchemaFieldHook("Quality", "name"), useQualityDescription = createSchemaFieldHook("Quality", "description"), useQualityUri = createSchemaFieldHook("Quality", "uri"), useQualityKind = createSchemaFieldHook("Quality", "kind"), useQualityFolder = createSchemaFieldHook("Quality", "folder"), useQualityCanScale = createSchemaFieldHook("Quality", "canScale"), useQualityDefaultSiUnit = createSchemaFieldHook("Quality", "defaultSiUnit"), useQualityDefaultImperialUnit = createSchemaFieldHook("Quality", "defaultImperialUnit"), useQualityMin = createSchemaFieldHook("Quality", "min"), useQualityIsMinExcluded = createSchemaFieldHook("Quality", "isMinExcluded"), useQualityMax = createSchemaFieldHook("Quality", "max"), useQualityIsMaxExcluded = createSchemaFieldHook("Quality", "isMaxExcluded"), useQualityDefaultValue = createSchemaFieldHook("Quality", "defaultValue"), useQualityFormula = createSchemaFieldHook("Quality", "formula"), useQualityIcon = createSchemaFieldHook("Quality", "icon"), useQualityImage = createSchemaFieldHook("Quality", "image"), useQualityUnit = createSchemaFieldHook("Quality", "unit"), useQualityBenchmarks = createSchemaFieldHook("Quality", "benchmarks"), useQualityAttributes = createSchemaFieldHook("Quality", "attributes");
export const usePort = createSchemaObjectHook("Port"), usePortHash = createSchemaFieldHook("Port", "hash"), usePortId = createSchemaFieldHook("Port", "id"), usePortKit = createSchemaFieldHook("Port", "kit"), usePortName = createSchemaFieldHook("Port", "name"), usePortDescription = createSchemaFieldHook("Port", "description"), usePortIcon = createSchemaFieldHook("Port", "icon"), usePortMaxChildren = createSchemaFieldHook("Port", "maxChildren"), usePortCompatiblePorts = createSchemaFieldHook("Port", "compatiblePorts"), usePortAttributes = createSchemaFieldHook("Port", "attributes");
export const useProp = createSchemaObjectHook("Prop"), usePropHash = createSchemaFieldHook("Prop", "hash"), usePropId = createSchemaFieldHook("Prop", "id"), usePropKit = createSchemaFieldHook("Prop", "kit"), usePropQuality = createSchemaFieldHook("Prop", "quality"), usePropValue = createSchemaFieldHook("Prop", "value"), usePropUnit = createSchemaFieldHook("Prop", "unit"), usePropAttributes = createSchemaFieldHook("Prop", "attributes");
export const useTag = createSchemaObjectHook("Tag"), useTagHash = createSchemaFieldHook("Tag", "hash"), useTagId = createSchemaFieldHook("Tag", "id"), useTagKit = createSchemaFieldHook("Tag", "kit"), useTagName = createSchemaFieldHook("Tag", "name"), useTagDescription = createSchemaFieldHook("Tag", "description"), useTagIcon = createSchemaFieldHook("Tag", "icon"), useTagAttributes = createSchemaFieldHook("Tag", "attributes");
export const useConcept = createSchemaObjectHook("Concept"), useConceptHash = createSchemaFieldHook("Concept", "hash"), useConceptId = createSchemaFieldHook("Concept", "id"), useConceptKit = createSchemaFieldHook("Concept", "kit"), useConceptName = createSchemaFieldHook("Concept", "name"), useConceptDescription = createSchemaFieldHook("Concept", "description"), useConceptIcon = createSchemaFieldHook("Concept", "icon"), useConceptAttributes = createSchemaFieldHook("Concept", "attributes");
export const useFamily = createSchemaObjectHook("Family"), useFamilyHash = createSchemaFieldHook("Family", "hash"), useFamilyId = createSchemaFieldHook("Family", "id"), useFamilyKit = createSchemaFieldHook("Family", "kit"), useFamilyName = createSchemaFieldHook("Family", "name"), useFamilyDescription = createSchemaFieldHook("Family", "description"), useFamilyIcon = createSchemaFieldHook("Family", "icon"), useFamilyPorts = createSchemaFieldHook("Family", "ports"), useFamilyAttributes = createSchemaFieldHook("Family", "attributes");
export const useRepresentation = createSchemaObjectHook("Representation"), useRepresentationHash = createSchemaFieldHook("Representation", "hash"), useRepresentationId = createSchemaFieldHook("Representation", "id"), useRepresentationType = createSchemaFieldHook("Representation", "type"), useRepresentationName = createSchemaFieldHook("Representation", "name"), useRepresentationTags = createSchemaFieldHook("Representation", "tags"), useRepresentationFile = createSchemaFieldHook("Representation", "file"), useRepresentationDescription = createSchemaFieldHook("Representation", "description"), useRepresentationAttributes = createSchemaFieldHook("Representation", "attributes");
export const useConnector = createSchemaObjectHook("Connector"), useConnectorHash = createSchemaFieldHook("Connector", "hash"), useConnectorId = createSchemaFieldHook("Connector", "id"), useConnectorType = createSchemaFieldHook("Connector", "type"), useConnectorName = createSchemaFieldHook("Connector", "name"), useConnectorT = createSchemaFieldHook("Connector", "t"), useConnectorPoint = createSchemaFieldHook("Connector", "point"), useConnectorDirection = createSchemaFieldHook("Connector", "direction"), useConnectorDescription = createSchemaFieldHook("Connector", "description"), useConnectorPort = createSchemaFieldHook("Connector", "port"), useConnectorMandatory = createSchemaFieldHook("Connector", "mandatory"), useConnectorMaxChildren = createSchemaFieldHook("Connector", "maxChildren"), useConnectorProps = createSchemaFieldHook("Connector", "props"), useConnectorAttributes = createSchemaFieldHook("Connector", "attributes"), useConnectorPieces = createSchemaFieldHook("Connector", "pieces"), useConnectorCompatibleConnectors = createSchemaFieldHook("Connector", "compatibleConnectors");
export const useType = createSchemaObjectHook("Type"), useTypeHash = createSchemaFieldHook("Type", "hash"), useTypeId = createSchemaFieldHook("Type", "id"), useTypeKit = createSchemaFieldHook("Type", "kit"), useTypeName = createSchemaFieldHook("Type", "name"), useTypeParent = createSchemaFieldHook("Type", "parent"), useTypeChildren = createSchemaFieldHook("Type", "children"), useTypeIsAbstract = createSchemaFieldHook("Type", "isAbstract"), useTypeFolder = createSchemaFieldHook("Type", "folder"), useTypeRepresentations = createSchemaFieldHook("Type", "representations"), useTypeConnectors = createSchemaFieldHook("Type", "connectors"), useTypeProps = createSchemaFieldHook("Type", "props"), useTypeStock = createSchemaFieldHook("Type", "stock"), useTypeVirtual = createSchemaFieldHook("Type", "virtual"), useTypeUnit = createSchemaFieldHook("Type", "unit"), useTypeCreatedAt = createSchemaFieldHook("Type", "createdAt"), useTypeUpdatedAt = createSchemaFieldHook("Type", "updatedAt"), useTypeLocation = createSchemaFieldHook("Type", "location"), useTypeAuthors = createSchemaFieldHook("Type", "authors"), useTypeConcepts = createSchemaFieldHook("Type", "concepts"), useTypeIcon = createSchemaFieldHook("Type", "icon"), useTypeImage = createSchemaFieldHook("Type", "image"), useTypeDescription = createSchemaFieldHook("Type", "description"), useTypeAttributes = createSchemaFieldHook("Type", "attributes"), useTypeFixedPieces = createSchemaFieldHook("Type", "fixedPieces");
export const useLayer = createSchemaObjectHook("Layer"), useLayerHash = createSchemaFieldHook("Layer", "hash"), useLayerId = createSchemaFieldHook("Layer", "id"), useLayerDesign = createSchemaFieldHook("Layer", "design"), useLayerPath = createSchemaFieldHook("Layer", "path"), useLayerIsHidden = createSchemaFieldHook("Layer", "isHidden"), useLayerIsLocked = createSchemaFieldHook("Layer", "isLocked"), useLayerColor = createSchemaFieldHook("Layer", "color"), useLayerDescription = createSchemaFieldHook("Layer", "description"), useLayerAttributes = createSchemaFieldHook("Layer", "attributes");
export const useSide = createSchemaObjectHook("Side"), useSideHash = createSchemaFieldHook("Side", "hash"), useSideConnection = createSchemaFieldHook("Side", "connection"), useSidePiece = createSchemaFieldHook("Side", "piece"), useSideDesignPiece = createSchemaFieldHook("Side", "designPiece"), useSideConnector = createSchemaFieldHook("Side", "connector");
export const useConnection = createSchemaObjectHook("Connection"), useConnectionHash = createSchemaFieldHook("Connection", "hash"), useConnectionId = createSchemaFieldHook("Connection", "id"), useConnectionDesign = createSchemaFieldHook("Connection", "design"), useConnectionConnected = createSchemaFieldHook("Connection", "connected"), useConnectionConnecting = createSchemaFieldHook("Connection", "connecting"), useConnectionGap = createSchemaFieldHook("Connection", "gap"), useConnectionShift = createSchemaFieldHook("Connection", "shift"), useConnectionRise = createSchemaFieldHook("Connection", "rise"), useConnectionRotation = createSchemaFieldHook("Connection", "rotation"), useConnectionTurn = createSchemaFieldHook("Connection", "turn"), useConnectionTilt = createSchemaFieldHook("Connection", "tilt"), useConnectionU = createSchemaFieldHook("Connection", "u"), useConnectionV = createSchemaFieldHook("Connection", "v"), useConnectionDescription = createSchemaFieldHook("Connection", "description"), useConnectionAttributes = createSchemaFieldHook("Connection", "attributes"), useConnectionChildPiece = createSchemaFieldHook("Connection", "childPiece"), useConnectionChildConnector = createSchemaFieldHook("Connection", "childConnector"), useConnectionParentPiece = createSchemaFieldHook("Connection", "parentPiece"), useConnectionParentConnector = createSchemaFieldHook("Connection", "parentConnector");
export const useStat = createSchemaObjectHook("Stat"), useStatHash = createSchemaFieldHook("Stat", "hash"), useStatId = createSchemaFieldHook("Stat", "id"), useStatDesign = createSchemaFieldHook("Stat", "design"), useStatQuality = createSchemaFieldHook("Stat", "quality"), useStatUnit = createSchemaFieldHook("Stat", "unit"), useStatMin = createSchemaFieldHook("Stat", "min"), useStatMinExcluded = createSchemaFieldHook("Stat", "minExcluded"), useStatMax = createSchemaFieldHook("Stat", "max"), useStatMaxExcluded = createSchemaFieldHook("Stat", "maxExcluded");
export const useBlueprint = createSchemaObjectHook("Blueprint"), useBlueprintType = createSchemaFieldHook("Blueprint", "type"), useBlueprintDesign = createSchemaFieldHook("Blueprint", "design");
export const usePiece = createSchemaObjectHook("Piece"), usePieceId = createSchemaFieldHook("Piece", "id"), usePieceHash = createSchemaFieldHook("Piece", "hash"), usePieceName = createSchemaFieldHook("Piece", "name"), usePiecePlane = createSchemaFieldHook("Piece", "plane"), usePieceCenter = createSchemaFieldHook("Piece", "center"), usePieceScale = createSchemaFieldHook("Piece", "scale"), usePieceMirrorPlane = createSchemaFieldHook("Piece", "mirrorPlane"), usePieceIsHidden = createSchemaFieldHook("Piece", "isHidden"), usePieceIsLocked = createSchemaFieldHook("Piece", "isLocked"), usePieceColor = createSchemaFieldHook("Piece", "color"), usePieceDescription = createSchemaFieldHook("Piece", "description"), usePieceKind = createSchemaFieldHook("Piece", "kind"), usePieceType = createSchemaFieldHook("Piece", "type"), usePieceDesign = createSchemaFieldHook("Piece", "design"), usePieceProps = createSchemaFieldHook("Piece", "props"), usePieceAttributes = createSchemaFieldHook("Piece", "attributes"), usePieceFlatPlane = createSchemaFieldHook("Piece", "flatPlane"), usePieceFlatCenter = createSchemaFieldHook("Piece", "flatCenter"), usePieceParentPiece = createSchemaFieldHook("Piece", "parentPiece"), usePieceParentConnection = createSchemaFieldHook("Piece", "parentConnection"), usePieceChildPieces = createSchemaFieldHook("Piece", "childPieces"), usePieceChildConnections = createSchemaFieldHook("Piece", "childConnections"), usePieceAlternatives = createSchemaFieldHook("Piece", "alternatives"), usePieceAlternativeTypes = createSchemaFieldHook("Piece", "alternativeTypes"), usePieceAlternativeDesigns = createSchemaFieldHook("Piece", "alternativeDesigns");
export const useGroup = createSchemaObjectHook("Group"), useGroupHash = createSchemaFieldHook("Group", "hash"), useGroupId = createSchemaFieldHook("Group", "id"), useGroupDesign = createSchemaFieldHook("Group", "design"), useGroupPieces = createSchemaFieldHook("Group", "pieces"), useGroupColor = createSchemaFieldHook("Group", "color"), useGroupName = createSchemaFieldHook("Group", "name"), useGroupDescription = createSchemaFieldHook("Group", "description"), useGroupAttributes = createSchemaFieldHook("Group", "attributes");
export const useDesign = createSchemaObjectHook("Design"), useDesignHash = createSchemaFieldHook("Design", "hash"), useDesignId = createSchemaFieldHook("Design", "id"), useDesignKit = createSchemaFieldHook("Design", "kit"), useDesignName = createSchemaFieldHook("Design", "name"), useDesignParent = createSchemaFieldHook("Design", "parent"), useDesignChildren = createSchemaFieldHook("Design", "children"), useDesignIsAbstract = createSchemaFieldHook("Design", "isAbstract"), useDesignFolder = createSchemaFieldHook("Design", "folder"), useDesignPieces = createSchemaFieldHook("Design", "pieces"), useDesignConnections = createSchemaFieldHook("Design", "connections"), useDesignStats = createSchemaFieldHook("Design", "stats"), useDesignProps = createSchemaFieldHook("Design", "props"), useDesignLayers = createSchemaFieldHook("Design", "layers"), useDesignActiveLayer = createSchemaFieldHook("Design", "activeLayer"), useDesignGroups = createSchemaFieldHook("Design", "groups"), useDesignCanScale = createSchemaFieldHook("Design", "canScale"), useDesignCanMirror = createSchemaFieldHook("Design", "canMirror"), useDesignUnit = createSchemaFieldHook("Design", "unit"), useDesignLocation = createSchemaFieldHook("Design", "location"), useDesignAuthors = createSchemaFieldHook("Design", "authors"), useDesignConcepts = createSchemaFieldHook("Design", "concepts"), useDesignIcon = createSchemaFieldHook("Design", "icon"), useDesignImage = createSchemaFieldHook("Design", "image"), useDesignDescription = createSchemaFieldHook("Design", "description"), useDesignAttributes = createSchemaFieldHook("Design", "attributes"), useDesignCreatedAt = createSchemaFieldHook("Design", "createdAt"), useDesignUpdatedAt = createSchemaFieldHook("Design", "updatedAt");
export const useKit = createSchemaObjectHook("Kit"), useKitHash = createSchemaFieldHook("Kit", "hash"), useKitId = createSchemaFieldHook("Kit", "id"), useKitName = createSchemaFieldHook("Kit", "name"), useKitRelease = createSchemaFieldHook("Kit", "release"), useKitTypes = createSchemaFieldHook("Kit", "types"), useKitDesigns = createSchemaFieldHook("Kit", "designs"), useKitTags = createSchemaFieldHook("Kit", "tags"), useKitConcepts = createSchemaFieldHook("Kit", "concepts"), useKitFamilies = createSchemaFieldHook("Kit", "families"), useKitPorts = createSchemaFieldHook("Kit", "ports"), useKitQualities = createSchemaFieldHook("Kit", "qualities"), useKitFiles = createSchemaFieldHook("Kit", "files"), useKitFolders = createSchemaFieldHook("Kit", "folders"), useKitAuthors = createSchemaFieldHook("Kit", "authors"), useKitRemote = createSchemaFieldHook("Kit", "remote"), useKitHomepage = createSchemaFieldHook("Kit", "homepage"), useKitLicense = createSchemaFieldHook("Kit", "license"), useKitPreview = createSchemaFieldHook("Kit", "preview"), useKitIcon = createSchemaFieldHook("Kit", "icon"), useKitImage = createSchemaFieldHook("Kit", "image"), useKitDescription = createSchemaFieldHook("Kit", "description"), useKitAttributes = createSchemaFieldHook("Kit", "attributes"), useKitCreatedAt = createSchemaFieldHook("Kit", "createdAt"), useKitUpdatedAt = createSchemaFieldHook("Kit", "updatedAt");
export const useKitBackbone = createSchemaObjectHook("KitBackbone"), useKitBackboneHash = createSchemaFieldHook("KitBackbone", "hash"), useKitBackboneKind = createSchemaFieldHook("KitBackbone", "kind"), useKitBackboneEndpoint = createSchemaFieldHook("KitBackbone", "endpoint"), useKitBackboneAuthoritative = createSchemaFieldHook("KitBackbone", "authoritative"), useKitBackboneLinearHistory = createSchemaFieldHook("KitBackbone", "linearHistory"), useKitBackboneConnected = createSchemaFieldHook("KitBackbone", "connected"), useKitBackboneTimeoutSeconds = createSchemaFieldHook("KitBackbone", "timeoutSeconds"), useKitBackboneCurrentHash = createSchemaFieldHook("KitBackbone", "currentHash"), useKitBackboneLastInteractionIndex = createSchemaFieldHook("KitBackbone", "lastInteractionIndex"), useKitBackbonePendingCandidateCount = createSchemaFieldHook("KitBackbone", "pendingCandidateCount");
export const useKitClientInfo = createSchemaObjectHook("KitClientInfo"), useKitClientInfoHash = createSchemaFieldHook("KitClientInfo", "hash"), useKitClientInfoId = createSchemaFieldHook("KitClientInfo", "id"), useKitClientInfoName = createSchemaFieldHook("KitClientInfo", "name"), useKitClientInfoVersion = createSchemaFieldHook("KitClientInfo", "version"), useKitClientInfoPlatform = createSchemaFieldHook("KitClientInfo", "platform");
export const useKitSession = createSchemaObjectHook("KitSession"), useKitSessionHash = createSchemaFieldHook("KitSession", "hash"), useKitSessionId = createSchemaFieldHook("KitSession", "id"), useKitSessionKit = createSchemaFieldHook("KitSession", "kit"), useKitSessionActor = createSchemaFieldHook("KitSession", "actor"), useKitSessionClient = createSchemaFieldHook("KitSession", "client"), useKitSessionState = createSchemaFieldHook("KitSession", "state"), useKitSessionStrictMode = createSchemaFieldHook("KitSession", "strictMode"), useKitSessionTimeoutSeconds = createSchemaFieldHook("KitSession", "timeoutSeconds"), useKitSessionStartedAt = createSchemaFieldHook("KitSession", "startedAt"), useKitSessionLastSeenAt = createSchemaFieldHook("KitSession", "lastSeenAt"), useKitSessionExpiresAt = createSchemaFieldHook("KitSession", "expiresAt"), useKitSessionDisconnectedAt = createSchemaFieldHook("KitSession", "disconnectedAt"), useKitSessionLocked = createSchemaFieldHook("KitSession", "locked"), useKitSessionCanReconnect = createSchemaFieldHook("KitSession", "canReconnect"), useKitSessionCanSaveLocalChanges = createSchemaFieldHook("KitSession", "canSaveLocalChanges"), useKitSessionWarning = createSchemaFieldHook("KitSession", "warning"), useKitSessionSelection = createSchemaFieldHook("KitSession", "selection"), useKitSessionActiveTransactions = createSchemaFieldHook("KitSession", "activeTransactions");
export const useKitChange = createSchemaObjectHook("KitChange"), useKitChangeHash = createSchemaFieldHook("KitChange", "hash"), useKitChangeId = createSchemaFieldHook("KitChange", "id"), useKitChangeKind = createSchemaFieldHook("KitChange", "kind"), useKitChangeSummary = createSchemaFieldHook("KitChange", "summary"), useKitChangeOrigin = createSchemaFieldHook("KitChange", "origin"), useKitChangeActor = createSchemaFieldHook("KitChange", "actor"), useKitChangeSession = createSchemaFieldHook("KitChange", "session"), useKitChangeTransaction = createSchemaFieldHook("KitChange", "transaction"), useKitChangeForward = createSchemaFieldHook("KitChange", "forward"), useKitChangeBackward = createSchemaFieldHook("KitChange", "backward"), useKitChangeValidation = createSchemaFieldHook("KitChange", "validation"), useKitChangeCreatedAt = createSchemaFieldHook("KitChange", "createdAt"), useKitChangeAppliedAt = createSchemaFieldHook("KitChange", "appliedAt");
export const useKitChangeCandidate = createSchemaObjectHook("KitChangeCandidate"), useKitChangeCandidateHash = createSchemaFieldHook("KitChangeCandidate", "hash"), useKitChangeCandidateId = createSchemaFieldHook("KitChangeCandidate", "id"), useKitChangeCandidateKit = createSchemaFieldHook("KitChangeCandidate", "kit"), useKitChangeCandidateKind = createSchemaFieldHook("KitChangeCandidate", "kind"), useKitChangeCandidateSummary = createSchemaFieldHook("KitChangeCandidate", "summary"), useKitChangeCandidateProposedBy = createSchemaFieldHook("KitChangeCandidate", "proposedBy"), useKitChangeCandidateActor = createSchemaFieldHook("KitChangeCandidate", "actor"), useKitChangeCandidateTransaction = createSchemaFieldHook("KitChangeCandidate", "transaction"), useKitChangeCandidateStatus = createSchemaFieldHook("KitChangeCandidate", "status"), useKitChangeCandidateRequestedFrom = createSchemaFieldHook("KitChangeCandidate", "requestedFrom"), useKitChangeCandidateVotes = createSchemaFieldHook("KitChangeCandidate", "votes"), useKitChangeCandidateValidation = createSchemaFieldHook("KitChangeCandidate", "validation"), useKitChangeCandidatePreview = createSchemaFieldHook("KitChangeCandidate", "preview"), useKitChangeCandidateProposedAt = createSchemaFieldHook("KitChangeCandidate", "proposedAt"), useKitChangeCandidateExpiresAt = createSchemaFieldHook("KitChangeCandidate", "expiresAt"), useKitChangeCandidateDecidedAt = createSchemaFieldHook("KitChangeCandidate", "decidedAt");
export const useKitTransaction = createSchemaObjectHook("KitTransaction"), useKitTransactionHash = createSchemaFieldHook("KitTransaction", "hash"), useKitTransactionId = createSchemaFieldHook("KitTransaction", "id"), useKitTransactionKit = createSchemaFieldHook("KitTransaction", "kit"), useKitTransactionLabel = createSchemaFieldHook("KitTransaction", "label"), useKitTransactionState = createSchemaFieldHook("KitTransaction", "state"), useKitTransactionStartedBy = createSchemaFieldHook("KitTransaction", "startedBy"), useKitTransactionParent = createSchemaFieldHook("KitTransaction", "parent"), useKitTransactionStartedAt = createSchemaFieldHook("KitTransaction", "startedAt"), useKitTransactionFinalizedAt = createSchemaFieldHook("KitTransaction", "finalizedAt"), useKitTransactionAbortedAt = createSchemaFieldHook("KitTransaction", "abortedAt"), useKitTransactionChanges = createSchemaFieldHook("KitTransaction", "changes"), useKitTransactionUndoStack = createSchemaFieldHook("KitTransaction", "undoStack"), useKitTransactionRedoStack = createSchemaFieldHook("KitTransaction", "redoStack"), useKitTransactionCanUndo = createSchemaFieldHook("KitTransaction", "canUndo"), useKitTransactionCanRedo = createSchemaFieldHook("KitTransaction", "canRedo"), useKitTransactionSquashedChange = createSchemaFieldHook("KitTransaction", "squashedChange");
export const useKitStoreEntity = createSchemaObjectHook("KitStore"), useKitStoreEntityHash = createSchemaFieldHook("KitStore", "hash"), useKitStoreEntityKit = createSchemaFieldHook("KitStore", "kit"), useKitStoreEntityBackbone = createSchemaFieldHook("KitStore", "backbone"), useKitStoreEntitySessions = createSchemaFieldHook("KitStore", "sessions"), useKitStoreEntityTransactions = createSchemaFieldHook("KitStore", "transactions"), useKitStoreEntityPendingCandidates = createSchemaFieldHook("KitStore", "pendingCandidates"), useKitStoreEntityActiveConflicts = createSchemaFieldHook("KitStore", "activeConflicts"), useKitStoreEntityValidation = createSchemaFieldHook("KitStore", "validation"), useKitStoreEntityHistory = createSchemaFieldHook("KitStore", "history"), useKitStoreEntityBlockedByConflict = createSchemaFieldHook("KitStore", "blockedByConflict"), useKitStoreEntityStrictMode = createSchemaFieldHook("KitStore", "strictMode");
export const useKitStoreEvent = createSchemaObjectHook("KitStoreEvent"), useKitStoreEventHash = createSchemaFieldHook("KitStoreEvent", "hash"), useKitStoreEventKind = createSchemaFieldHook("KitStoreEvent", "kind"), useKitStoreEventStore = createSchemaFieldHook("KitStoreEvent", "store"), useKitStoreEventInteraction = createSchemaFieldHook("KitStoreEvent", "interaction"), useKitStoreEventChange = createSchemaFieldHook("KitStoreEvent", "change"), useKitStoreEventCandidate = createSchemaFieldHook("KitStoreEvent", "candidate"), useKitStoreEventConflict = createSchemaFieldHook("KitStoreEvent", "conflict"), useKitStoreEventSession = createSchemaFieldHook("KitStoreEvent", "session"), useKitStoreEventTransaction = createSchemaFieldHook("KitStoreEvent", "transaction");

// #endregion ⚛️Direct Domain Exports

// #region ⚛️Embedded Tests

declare const __SEMIO_REACT_RUN_EMBEDDED_TESTS__: boolean | undefined;

const shouldRunEmbeddedTests =
	(typeof __SEMIO_REACT_RUN_EMBEDDED_TESTS__ !== "undefined" && __SEMIO_REACT_RUN_EMBEDDED_TESTS__) ||
	(typeof __SEMIO_REACT_RUN_EMBEDDED_TESTS__ === "undefined" && typeof (globalThis as any).__vitest_worker__ !== "undefined" && typeof process !== "undefined" && process.env.SEMIO_REACT_RUN_EMBEDDED_TESTS === "1");

if (shouldRunEmbeddedTests) {
	const { beforeEach, describe, expect, it } = await import("vitest");
	const { createElement } = await import("react");
	const { renderToStaticMarkup } = await import("react-dom/server");
	const { MetabolismKit } = await import("@semio/assets");

	let store: InMemoryKitStore;
	let pieceGuid: string;

	beforeEach(() => {
		const kit = asKitInstance(MetabolismKit as any);
		store = new InMemoryKitStore(kit);
		pieceGuid = kit.designs?.[0]?.pieces?.[0]?.guid;
	});

	const renderHook = <T,>(hook: () => SchemaHookTriad<T>): SchemaHookTriad<T> => {
		let triad: SchemaHookTriad<T> | undefined;
		const Component = () => {
			triad = hook();
			return createElement("div", null, "x");
		};
		renderToStaticMarkup(createElement(KitProvider, { store }, createElement(Component, null)));
		return triad!;
	};

	describe("@semio/react", () => {
		it("exposes triad hooks for kit fields", () => {
			const [name, setName, canSetName] = renderHook(() => useKitName());
			expect(typeof name).toBe("string");
			expect(canSetName).toBe(true);
			setName("Renamed Kit");
			const [nextName] = renderHook(() => useKitName());
			expect(nextName).toBe("Renamed Kit");
		});

		it("updates piece fields through direct hooks", () => {
			const [name, setName, canSetName] = renderHook(() => usePieceName(pieceGuid));
			expect(name).toBeDefined();
			expect(canSetName).toBe(true);
			setName("Updated Piece Name");
			const [nextName] = renderHook(() => usePieceName(pieceGuid));
			expect(nextName).toBe("Updated Piece Name");
		});

		it("generates schema property events from snapshots", () => {
			const before = { guid: "kit-event", name: "Initial Kit", release: "0.1.0", types: [], designs: [] } as any;
			const after = { ...before, name: "Evented Kit" } as any;
			const events = diffSchemaPropertyEvents(before, after);
			expect(events.some((event) => event.key === "Kit.name")).toBe(true);
		});

		it("exposes dynamic schema hook namespace", () => {
			const [name] = renderHook(() => (schemaHooks as any).usePortName());
			expect(name === undefined || typeof name === "string").toBe(true);
		});
	});
}

// #endregion ⚛️Embedded Tests
