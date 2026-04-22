// @ts-nocheck
// #region ⚛️Header

// Standalone React hooks bundle for semio.

// #endregion ⚛️Header

// #region ⚛️Imports

import * as React from "react";
import type { ReactNode, SetStateAction } from "react";
import {
	asKitInstance,
	Coordinate,
	createFolderKitStore,
	createJsonFileKitStore,
	createKitStoreClient,
	createSessionKitStore,
	Design,
	getIncludedDesigns,
	guid,
	InMemoryKitStore,
	KitImpl,
	type KitLike,
	type KitStore,
	type KitStoreClient,
	type KitStoreSnapshot,
	type SetError,
	type SetResult,
	type WriteStatus,
	Plane,
	Piece,
} from "@semio/js";

// #endregion ⚛️Imports

// #region ⚛️Types

/** @deprecated use HookTriad */
export type SchemaHookTriad<T> = readonly [T, (next: SetStateAction<T>) => Promise<SetResult>, WriteStatus];

export type { SetError, SetResult, WriteStatus };
export type HookTriad<T> = readonly [T, (next: SetStateAction<T>) => Promise<SetResult>, WriteStatus];

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
	recentSetRejections: SetError[];
	pushSetRejection: (e: SetError) => void;
	canWrite: boolean;
	kitClient: KitStoreClient | null;
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

async function noopAsyncSet(_next?: unknown): Promise<SetResult> {
	return { ok: true } as const;
}

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

function getFieldDataKey(typeName: string, fieldName: string): string {
	if (fieldName === "id") return "guid";
	if (typeName === "Kit" && fieldName === "release") return "version";
	return fieldName;
}

function getSchemaFieldName(typeName: string, dataKey: string): string {
	if (dataKey === "guid") return "id";
	if (typeName === "Kit" && dataKey === "version") return "release";
	return dataKey;
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

function collectChangedObjectFields(typeName: string, previousValue: any, nextValue: any): string[] {
	const dataKeys = new Set<string>();
	if (previousValue && typeof previousValue === "object") {
		for (const dataKey of Object.keys(previousValue)) dataKeys.add(dataKey);
	}
	if (nextValue && typeof nextValue === "object") {
		for (const dataKey of Object.keys(nextValue)) dataKeys.add(dataKey);
	}
	const fieldNames: string[] = [];
	for (const dataKey of dataKeys) {
		if (!deepEqual(previousValue?.[dataKey], nextValue?.[dataKey])) {
			fieldNames.push(getSchemaFieldName(typeName, dataKey));
		}
	}
	return fieldNames;
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
		for (const fieldName of collectChangedObjectFields(typeName, previousRef?.value, nextRef?.value)) {
			const previousValue = readSchemaFieldValue(previous, typeName, fieldName, guidValue);
			const nextValue = readSchemaFieldValue(next, typeName, fieldName, guidValue);
			if (!deepEqual(previousValue, nextValue)) {
				events.push({ key: `${typeName}.${fieldName}`, typeName, fieldName, guid: guidValue, previous: previousValue, current: nextValue });
			}
		}
	}

	if (!deepEqual(previous.plain, next.plain) && next.kitGuid) {
		for (const fieldName of collectChangedObjectFields("Kit", previous.plain, next.plain)) {
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

async function createStoreFromBackbone(backbone: KitProviderBackbone | undefined, initialKit?: KitLike): Promise<KitStore> {
	const resolvedBackbone = backbone?.kind ? backbone : ({ kind: "memory", initialKit } as MemoryBackboneConfig);
	if (resolvedBackbone.kind === "memory") {
		const seed = resolvedBackbone.initialKit ?? initialKit ?? { guid: guid(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() };
		return new InMemoryKitStore(asKitInstance(seed));
	}
	if (resolvedBackbone.kind === "dev") {
		return createJsonFileKitStore(await createNodeJsonFileAdapter(resolvedBackbone.filePath));
	}
	if (resolvedBackbone.kind === "local") {
		return createFolderKitStore(await createNodeFolderAdapter(resolvedBackbone.folderPath), initialKit ? asKitInstance(initialKit).toJSON() as any : undefined);
	}
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

// #region KitRegistry

export type KitRegistryEntry = {
	store: KitStore;
	kitClient: KitStoreClient;
	refs: number;
};

export type KitRegistryValue = {
	open: (guid: string, init: { backbone?: KitProviderBackbone; initialKit?: KitLike; store?: KitStore }) => Promise<void>;
	close: (guid: string) => void;
	get: (guid: string) => KitRegistryEntry | undefined;
	list: () => string[];
	status: (guid: string) => "idle" | "loading" | "ready" | "error";
};

type RegistryRow = {
	store: KitStore;
	kitClient: KitStoreClient;
	refs: number;
	unsub: () => void;
};

const KitRegistryContext = React.createContext<KitRegistryValue | null>(null);

export function KitRegistryProvider({ children }: { children: ReactNode }): React.ReactElement {
	const rowsRef = React.useRef(new Map<string, RegistryRow>());
	const loadingRef = React.useRef(new Set<string>());
	const errRef = React.useRef(new Map<string, Error>());
	const [, bump] = React.useReducer((x: number) => x + 1, 0);

	const value = React.useMemo<KitRegistryValue>(
		() => ({
			async open(guid, init) {
				const cur = rowsRef.current.get(guid);
				if (cur) {
					cur.refs += 1;
					bump();
					return;
				}
				loadingRef.current.add(guid);
				errRef.current.delete(guid);
				bump();
				try {
					const store = init.store ?? (await createStoreFromBackbone(init.backbone, init.initialKit));
					const kitClient = await createKitStoreClient({ initialKit: store.getSnapshot().kit });
					const unsub = kitClient.subscribe(() => {
						try {
							const incoming = kitClient.getDto();
							const curJson = store.getSnapshot().kit.toJSON();
							if (JSON.stringify(incoming) === JSON.stringify(curJson)) return;
							store.replace(asKitInstance(incoming));
						} catch {
							store.replace(asKitInstance(kitClient.getDto()));
						}
					});
					rowsRef.current.set(guid, { store, kitClient, refs: 1, unsub });
				} catch (e) {
					errRef.current.set(guid, e instanceof Error ? e : new Error(String(e)));
				} finally {
					loadingRef.current.delete(guid);
					bump();
				}
			},
			close(guid) {
				const row = rowsRef.current.get(guid);
				if (!row) return;
				row.refs -= 1;
				if (row.refs <= 0) {
					row.unsub();
					row.kitClient.dispose();
					rowsRef.current.delete(guid);
				}
				bump();
			},
			get(guid) {
				const row = rowsRef.current.get(guid);
				if (!row) return undefined;
				return { store: row.store, kitClient: row.kitClient, refs: row.refs };
			},
			list() {
				return Array.from(rowsRef.current.keys());
			},
			status(guid) {
				if (loadingRef.current.has(guid)) return "loading";
				if (errRef.current.has(guid)) return "error";
				if (rowsRef.current.has(guid)) return "ready";
				return "idle";
			},
		}),
		[],
	);

	return React.createElement(KitRegistryContext.Provider, { value }, children);
}

export function useKitRegistry(): KitRegistryValue {
	const v = React.useContext(KitRegistryContext);
	if (!v) throw new Error("useKitRegistry must be used within <KitRegistryProvider>.");
	return v;
}

/** Like {@link useKitRegistry} but returns `null` when no provider is mounted. */
export function useKitRegistrySafe(): KitRegistryValue | null {
	return React.useContext(KitRegistryContext);
}

// #endregion KitRegistry

function useKitRuntime(): KitRuntimeContextValue {
	const runtime = React.useContext(KitRuntimeContext);
	if (!runtime) throw new Error("semio/react hooks must be used inside <KitProvider>.");
	return runtime;
}

/** Returns the WASM worker {@link KitStoreClient} when inside {@link KitProvider}, or `null`. */
export function useKitStoreClient(): KitStoreClient | null {
	const runtime = useKitRuntime();
	return runtime.kitClient;
}

export type KitProviderProps = {
	store?: KitStore;
	/** When set with <KitRegistryProvider>, uses the registry entry for this kit (warm WASM worker). */
	kitGuid?: string;
	/** When provided (e.g. from registry), skips creating a new worker client. */
	kitClient?: KitStoreClient | null;
	backbone?: KitProviderBackbone;
	initialKit?: KitLike;
	children: ReactNode;
	fallback?: ReactNode;
};

export function KitProvider({
	store: externalStore,
	kitGuid,
	kitClient: kitClientProp,
	backbone,
	initialKit,
	children,
	fallback = null,
}: KitProviderProps): React.ReactElement | null {
	const registry = React.useContext(KitRegistryContext);
	if (kitGuid && !registry) {
		throw new Error("semio/react: <KitProvider kitGuid={...}> must be wrapped in <KitRegistryProvider>.");
	}
	const registryEntry = kitGuid && registry ? registry.get(kitGuid) : undefined;

	const [internalStore, setInternalStore] = React.useState<KitStore | null>(externalStore ?? null);
	const [kitClientState, setKitClientState] = React.useState<KitStoreClient | null>(kitClientProp ?? null);

	React.useEffect(() => {
		if (kitGuid) return;
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
	}, [kitGuid, externalStore, backbone, initialKit]);

	React.useEffect(() => {
		if (kitGuid) return;
		if (kitClientProp !== undefined) {
			setKitClientState(kitClientProp);
			return;
		}
		const st = externalStore ?? internalStore;
		if (!st) return;
		let cancelled = false;
		let client: KitStoreClient | null = null;
		void createKitStoreClient({ initialKit: st.getSnapshot().kit }).then((c) => {
			if (cancelled) {
				c.dispose();
				return;
			}
			client = c;
			setKitClientState(c);
		});
		return () => {
			cancelled = true;
			client?.dispose();
			setKitClientState(null);
		};
	}, [kitGuid, externalStore, internalStore, kitClientProp]);

	const store = kitGuid && registryEntry ? registryEntry.store : (externalStore ?? internalStore);
	const kitClient = kitGuid && registryEntry ? registryEntry.kitClient : (kitClientProp ?? kitClientState);

	if (kitGuid && registry && !registryEntry) return React.createElement(React.Fragment, null, fallback);
	if (!store) return React.createElement(React.Fragment, null, fallback);

	React.useEffect(() => {
		if (kitGuid) return;
		if (!kitClient) return;
		return kitClient.subscribe(() => {
			try {
				const incoming = kitClient.getDto();
				const cur = store.getSnapshot().kit.toJSON();
				if (JSON.stringify(incoming) === JSON.stringify(cur)) return;
				store.replace(asKitInstance(incoming));
			} catch {
				store.replace(asKitInstance(kitClient.getDto()));
			}
		});
	}, [kitClient, store, kitGuid]);

	const snapshotRef = React.useRef<KitStoreSnapshot | null>(null);
	const getSnapshot = React.useCallback(() => {
		const snap = store.getSnapshot();
		const prev = snapshotRef.current;
		if (
			prev &&
			prev.kit === snap.kit &&
			prev.sync.status === snap.sync.status &&
			prev.sync.dirty === snap.sync.dirty &&
			prev.sync.readonly === snap.sync.readonly &&
			prev.sync.lastSyncedAt === snap.sync.lastSyncedAt &&
			prev.sync.error === snap.sync.error
		) {
			return prev;
		}
		snapshotRef.current = snap;
		return snap;
	}, [store]);

	const snapshot = React.useSyncExternalStore(
		React.useCallback((listener) => store.subscribe(listener), [store]),
		getSnapshot,
		getSnapshot,
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

	const [recentSetRejections, setRecentSetRejections] = React.useState<SetError[]>([]);
	const pushSetRejection = React.useCallback((e: SetError) => {
		setRecentSetRejections((r) => [...r.slice(-99), e]);
	}, []);

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
		recentSetRejections,
		pushSetRejection,
		canWrite: !snapshot.sync.readonly,
		kitClient,
		setFieldValue,
		setObjectValue,
	}), [store, snapshot, state, recentEvents, recentSetRejections, pushSetRejection, kitClient, setFieldValue, setObjectValue]);

	return React.createElement(KitRuntimeContext.Provider, { value }, children);
}

function useEntityScope(typeName: string, guidValue?: string): SchemaScope {
	const runtime = useKitRuntime();
	const parentScope = React.useContext(SchemaScopeContext);
	const ref = resolveReference(runtime.state, typeName, guidValue, parentScope);
	return ref ? { typeName, guid: ref.guid, path: ref.path } : { typeName, guid: guidValue, path: [] };
}

export function PieceProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Piece", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function TypeProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Type", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function DesignProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Design", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectionProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Connection", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PortProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Port", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function QualityProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Quality", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FileProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("File", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FolderProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Folder", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AuthorProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Author", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function TagProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Tag", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConceptProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Concept", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FamilyProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Family", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function RepresentationProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Representation", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectorProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Connector", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function BenchmarkProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Benchmark", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function LayerProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Layer", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function GroupProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Group", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function StatProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Stat", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PropProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Prop", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AttributeProvider({ guid: guidValue, children }: { guid?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Attribute", guidValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

// #endregion ⚛️Context

// #region ⚛️Core Hooks

function resolveRustFieldTarget(
	runtime: KitRuntimeContextValue,
	typeName: string,
	fieldName: string,
	guidValue: string | undefined,
	scope: SchemaScope | null,
): { kind: string; guid: string; field: string } | null {
	if (!runtime.kitClient) return null;
	if (typeName === "Piece" && (fieldName === "name" || fieldName === "color")) {
		const g = guidValue ?? scope?.guid;
		if (!g) return null;
		return { kind: "Piece", guid: g, field: fieldName };
	}
	if (typeName === "Kit" && fieldName === "name") {
		return { kind: "Kit", guid: runtime.snapshot.kit.guid, field: "name" };
	}
	if (typeName === "Design" && fieldName === "name") {
		const g = guidValue ?? scope?.guid;
		if (!g) return null;
		return { kind: "Design", guid: g, field: "name" };
	}
	if (typeName === "Type" && fieldName === "name") {
		const g = guidValue ?? scope?.guid;
		if (!g) return null;
		return { kind: "Type", guid: g, field: "name" };
	}
	return null;
}

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

export function useSetErrors(filter?: Partial<{ entityKind: string; guid: string }>): SetError[] {
	const runtime = useKitRuntime();
	return React.useMemo(() => {
		if (!filter) return runtime.recentSetRejections;
		return runtime.recentSetRejections.filter((e) => {
			if (filter.entityKind && e.entity?.kind !== filter.entityKind) return false;
			if (filter.guid && e.entity?.guid !== filter.guid) return false;
			return true;
		});
	}, [runtime.recentSetRejections, filter]);
}

export function useWriteQueue(): { pending: number; byEntity: Record<string, number> } {
	const runtime = useKitRuntime();
	return React.useMemo(() => ({ pending: 0, byEntity: {} }), [runtime.snapshot.sync.status]);
}

export function useKitSync(): { status: "idle" | "loading" | "saving" | "error"; lastError?: SetError } {
	const runtime = useKitRuntime();
	const s = runtime.snapshot.sync;
	if (s.status === "loading") return { status: "loading" };
	if (s.status === "saving") return { status: "saving" };
	if (s.status === "error")
		return {
			status: "error",
			lastError: { kind: "Internal", message: s.error instanceof Error ? s.error.message : String(s.error ?? "") },
		};
	return { status: "idle" };
}

export function useWriteIndicator(status: WriteStatus): {
	disabled: boolean;
	spinning: boolean;
	error?: SetError;
	warning?: SetError;
} {
	if (status.kind === "readonly") return { disabled: true, spinning: false };
	if (status.kind === "pending") return { disabled: true, spinning: true, error: status.lastError, warning: undefined };
	if (status.kind === "error") return { disabled: false, spinning: false, error: status.lastError };
	return { disabled: false, spinning: false };
}

export function useOptimistic<T>(triad: HookTriad<T>): {
	display: T;
	draft: T;
	setDraft: (next: SetStateAction<T>) => void;
	commit: () => Promise<SetResult>;
	reset: () => void;
	status: WriteStatus;
	dirty: boolean;
} {
	const [value, setValue, status] = triad;
	const [draft, setDraft] = React.useState<T | undefined>(undefined);
	const dirty = draft !== undefined;
	const display = (dirty ? draft : value) as T;
	const commit = React.useCallback(async () => {
		if (draft === undefined) return { ok: true } as const;
		const r = await setValue(draft);
		if (r.ok) setDraft(undefined);
		return r;
	}, [draft, setValue]);
	const reset = React.useCallback(() => setDraft(undefined), []);
	const setDraftFn = React.useCallback(
		(next: SetStateAction<T>) => {
			setDraft((d) => {
				const base = (d !== undefined ? d : value) as T;
				return typeof next === "function" ? (next as (p: T) => T)(base) : next;
			});
		},
		[value],
	);
	return {
		display,
		draft: (draft !== undefined ? draft : value) as T,
		setDraft: setDraftFn,
		commit,
		reset,
		status,
		dirty,
	};
}

// #region 🎛️KitStoreClient command hooks (WASM / worker RPCs)

export function useClusterPieces(): {
	run: (designGuid: string, pieceGuids: string[], clusterName: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuids: string[], clusterName: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.clusterPieces(designGuid, pieceGuids, clusterName);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useDragPieces(): {
	run: (designGuid: string, pieceGuids: string[], du: number, dv: number) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuids: string[], du: number, dv: number) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.dragPieces(designGuid, pieceGuids, du, dv);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useMovePieces(): {
	run: (designGuid: string, pieceGuids: string[], gap: number, shift: number, rise: number) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuids: string[], gap: number, shift: number, rise: number) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.movePieces(designGuid, pieceGuids, gap, shift, rise);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useFixPieces(): {
	run: (designGuid: string, pieceGuids: string[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuids: string[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.fixPieces(designGuid, pieceGuids);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useFlattenDesign(): { run: (designGuid: string) => Promise<SetResult>; status: WriteStatus } {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.flattenDesign(designGuid);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useExpandDesign(): {
	run: (parentDesignGuid: string, nestedDesignGuid: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (parentDesignGuid: string, nestedDesignGuid: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.expandDesign(parentDesignGuid, nestedDesignGuid);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useDeleteConnection(): {
	run: (designGuid: string, connectionGuid: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, connectionGuid: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.deleteConnection(designGuid, connectionGuid);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useChangePieceType(): {
	run: (designGuid: string, pieceGuid: string, newTypeGuid: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuid: string, newTypeGuid: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.changePieceType(designGuid, pieceGuid, newTypeGuid);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function usePasteDesignSelection(): {
	run: (designGuid: string, selection: unknown, plane?: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, selection: unknown, plane?: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.pasteDesignSelection(designGuid, selection, plane ?? null);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useCreateHangingPieces(): {
	run: (designGuid: string, typeGuids: string[], plane: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, typeGuids: string[], plane: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createHangingPieces(designGuid, typeGuids, plane);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useCreateConnectedPiece(): {
	run: (designGuid: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createConnectedPiece(designGuid, parentPiece, parentPort, childType, childPort);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useCreateFixedPiece(): {
	run: (designGuid: string, typeGuid: string, plane: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, typeGuid: string, plane: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createFixedPiece(designGuid, typeGuid, plane);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useDeletePiece(): {
	run: (designGuid: string, pieceGuid: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuid: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.removeChild("Design", designGuid, "Piece", pieceGuid);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useCreatePiece(): {
	run: (designGuid: string, piece: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, piece: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.addChild("Design", designGuid, "Piece", piece);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useAddConnection(): {
	run: (designGuid: string, connection: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, connection: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.addChild("Design", designGuid, "Connection", connection);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useUpdatePiece(): {
	run: (designGuid: string, pieceGuid: string, patch: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, pieceGuid: string, patch: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = { pieces: { updated: [{ piece: { guid: pieceGuid }, diff: patch }] } };
			const r = await runtime.kitClient.applyDesignDiff(designGuid, diff);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useUpdatePieces(): {
	run: (designGuid: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, updates: { id: string; diff: unknown }[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = {
				pieces: {
					updated: updates.map((u) => ({ piece: { guid: u.id }, diff: u.diff })),
				},
			};
			const r = await runtime.kitClient.applyDesignDiff(designGuid, diff);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useUpdateConnection(): {
	run: (designGuid: string, connectionGuid: string, patch: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, connectionGuid: string, patch: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = { connections: { updated: [{ connection: { guid: connectionGuid }, diff: patch }] } };
			const r = await runtime.kitClient.applyDesignDiff(designGuid, diff);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

export function useUpdateConnections(): {
	run: (designGuid: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designGuid: string, updates: { id: string; diff: unknown }[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = {
				connections: {
					updated: updates.map((u) => ({ connection: { guid: u.id }, diff: u.diff })),
				},
			};
			const r = await runtime.kitClient.applyDesignDiff(designGuid, diff);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime.kitClient, runtime.canWrite, runtime.pushSetRejection],
	);
	return { run, status };
}

/** Flatten-derived placement map from the Rust worker (`getPiecesMetadata`). */
export function usePiecesMetadataMap(designGuid?: string): SchemaHookTriad<Record<string, any>> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<Record<string, any>>({});
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designGuid) {
			setValue({});
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getPiecesMetadata(designGuid);
				if (!cancelled && m && typeof m === "object") setValue(m as Record<string, any>);
			} catch {
				if (!cancelled) setValue({});
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => {
			void load();
		});
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient, designGuid]);
	const status: WriteStatus =
		!designGuid || !runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcPieces(designGuid?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designGuid) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getPieces(designGuid);
				if (!cancelled) setValue(Array.isArray(m) ? m : []);
			} catch {
				if (!cancelled) setValue([]);
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => void load());
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient, designGuid]);
	const status: WriteStatus =
		!designGuid || !runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcConnections(designGuid?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designGuid) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getConnections(designGuid);
				if (!cancelled) setValue(Array.isArray(m) ? m : []);
			} catch {
				if (!cancelled) setValue([]);
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => void load());
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient, designGuid]);
	const status: WriteStatus =
		!designGuid || !runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcDesigns(): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getDesigns();
				if (!cancelled) setValue(Array.isArray(m) ? m : []);
			} catch {
				if (!cancelled) setValue([]);
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => void load());
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient]);
	const status: WriteStatus =
		!runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcTypes(): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getTypes();
				if (!cancelled) setValue(Array.isArray(m) ? m : []);
			} catch {
				if (!cancelled) setValue([]);
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => void load());
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient]);
	const status: WriteStatus =
		!runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcAuthors(): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getAuthors();
				if (!cancelled) setValue(Array.isArray(m) ? m : []);
			} catch {
				if (!cancelled) setValue([]);
			} finally {
				if (!cancelled) setPending((p) => Math.max(0, p - 1));
			}
		};
		void load();
		const unsub = runtime.kitClient.subscribe(() => void load());
		return () => {
			cancelled = true;
			unsub();
		};
	}, [runtime.kitClient]);
	const status: WriteStatus =
		!runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

/** Alias for {@link useRpcPieces}. */
export function usePieces(designGuid?: string): SchemaHookTriad<any[]> {
	return useRpcPieces(designGuid);
}

/** Alias for {@link useRpcConnections}. */
export function useConnections(designGuid?: string): SchemaHookTriad<any[]> {
	return useRpcConnections(designGuid);
}

/** Alias for {@link useRpcDesigns}. */
export function useDesigns(): SchemaHookTriad<any[]> {
	return useRpcDesigns();
}

/** Alias for {@link useRpcTypes}. */
export function useTypes(): SchemaHookTriad<any[]> {
	return useRpcTypes();
}

/** Alias for {@link useRpcAuthors}. */
export function useAuthors(): SchemaHookTriad<any[]> {
	return useRpcAuthors();
}

export function usePieceMetadata(designGuid?: string, pieceGuid?: string): SchemaHookTriad<any> {
	const [map, , status] = usePiecesMetadataMap(designGuid);
	const value = React.useMemo(() => (pieceGuid ? map[pieceGuid] : undefined), [map, pieceGuid]);
	return [value, noopAsyncSet, status] as const;
}

export function useFlatPiecePlane(designGuid?: string, pieceGuid?: string): SchemaHookTriad<any> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => meta?.plane, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useFlatPieceCenter(designGuid?: string, pieceGuid?: string): SchemaHookTriad<any> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => meta?.center, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useIsConnectedPiece(designGuid?: string, pieceGuid?: string): SchemaHookTriad<boolean> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => !!(meta?.parentPieceId), [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function usePieceDepth(designGuid?: string, pieceGuid?: string): SchemaHookTriad<number> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => (typeof meta?.depth === "number" ? meta.depth : 0), [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useFixedPieceId(designGuid?: string, pieceGuid?: string): SchemaHookTriad<string | undefined> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => meta?.fixedPieceId, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useParentPieceId(designGuid?: string, pieceGuid?: string): SchemaHookTriad<string | undefined> {
	const [meta, , status] = usePieceMetadata(designGuid, pieceGuid);
	const value = React.useMemo(() => meta?.parentPieceId ?? undefined, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function usePieceParentConnection(designGuid?: string, pieceGuid?: string): SchemaHookTriad<any | undefined> {
	const [conns, , st] = useRpcConnections(designGuid);
	const value = React.useMemo(() => {
		if (!pieceGuid || !Array.isArray(conns)) return undefined;
		return conns.find((c: any) => c?.connecting?.piece?.guid === pieceGuid);
	}, [conns, pieceGuid]);
	return [value, noopAsyncSet, st] as const;
}

export function useIncludedDesigns(designGuid?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const value = React.useMemo(() => {
		if (!designGuid || !runtime.state?.kit) return [];
		const d = runtime.state.kit.designs?.find((x: any) => x.guid === designGuid);
		return d ? getIncludedDesigns(d as Design) : [];
	}, [runtime.state.kit, designGuid]);
	return [value, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

export function useReplacableTypes(designGuid?: string, pieceGuids?: string[]): SchemaHookTriad<string[]> {
	const runtime = useKitRuntime();
	const [, , metaStatus] = usePiecesMetadataMap(designGuid);
	const value = React.useMemo(() => {
		if (!designGuid || !pieceGuids?.length || !runtime.state?.kit) return [];
		const kit = runtime.state.kit;
		const design = kit.designs?.find((d: any) => d.guid === designGuid);
		if (!design) return [];
		const designs = kit.designs ?? [];
		const types = kit.types ?? [];
		const ports = kit.ports ?? [];
		return kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design as Design, designs as Design[], types as any, ports as any, { pieces: pieceGuids }).types;
	}, [runtime.state.kit, designGuid, pieceGuids]);
	return [value, noopAsyncSet, metaStatus] as const;
}

export function useReplacableDesigns(designGuid?: string, pieceGuids?: string[]): SchemaHookTriad<string[]> {
	const runtime = useKitRuntime();
	const [, , metaStatus] = usePiecesMetadataMap(designGuid);
	const value = React.useMemo(() => {
		if (!designGuid || !pieceGuids?.length || !runtime.state?.kit) return [];
		const kit = runtime.state.kit;
		const design = kit.designs?.find((d: any) => d.guid === designGuid);
		if (!design) return [];
		const designs = kit.designs ?? [];
		const types = kit.types ?? [];
		const ports = kit.ports ?? [];
		return kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design as Design, designs as Design[], types as any, ports as any, { pieces: pieceGuids }).designs;
	}, [runtime.state.kit, designGuid, pieceGuids]);
	return [value, noopAsyncSet, metaStatus] as const;
}

export function useExplodeableDesignNodes(designGuid?: string): SchemaHookTriad<string[]> {
	const [included, , st] = useIncludedDesigns(designGuid);
	const value = React.useMemo(() => (included ?? []).map((x: any) => x.guid).filter(Boolean), [included]);
	return [value, noopAsyncSet, st] as const;
}

// #endregion 🎛️KitStoreClient command hooks

export function useKitStore(): SchemaHookTriad<KitStore> {
	const runtime = useKitRuntime();
	return [runtime.store, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

export function useKitSnapshot(): SchemaHookTriad<KitStoreSnapshot> {
	const runtime = useKitRuntime();
	return [runtime.snapshot, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

function useSchemaObjectState(typeName: string, guidValue?: string): SchemaHookTriad<any> {
	const runtime = useKitRuntime();
	const scope = React.useContext(SchemaScopeContext);
	const ref = resolveReference(runtime.state, typeName, guidValue, scope);
	const value = ref?.value;
	const canWrite = runtime.canWrite && !!ref;
	const setValue = React.useCallback(
		async (next: SetStateAction<any>) => {
			if (!canWrite) return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
			runtime.setObjectValue(typeName, next, guidValue, scope);
			return { ok: true } as const;
		},
		[runtime, typeName, guidValue, scope, canWrite],
	);
	const status: WriteStatus = canWrite ? { kind: "idle", pending: 0 } : { kind: "readonly", pending: 0 };
	return [value, setValue, status] as const;
}

function useSchemaFieldState(typeName: string, fieldName: string, guidValue?: string): SchemaHookTriad<any> {
	const runtime = useKitRuntime();
	const scope = React.useContext(SchemaScopeContext);
	const value = readSchemaFieldValue(runtime.state, typeName, fieldName, guidValue, scope);
	const classicWritable = runtime.canWrite && isWritableField(runtime.state, typeName, fieldName, guidValue, scope);
	const rustTarget = React.useMemo(
		() => resolveRustFieldTarget(runtime, typeName, fieldName, guidValue, scope),
		[runtime.kitClient, runtime.snapshot.kit.guid, runtime.canWrite, typeName, fieldName, guidValue, scope],
	);
	const [pending, setPending] = React.useState(0);
	const [lastErr, setLastErr] = React.useState<SetError | undefined>(undefined);

	const setValue = React.useCallback(
		async (next: SetStateAction<any>) => {
			const resolved = typeof next === "function" ? (next as (p: any) => any)(value) : next;
			if (rustTarget && runtime.kitClient) {
				if (!runtime.canWrite) {
					return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
				}
				setPending((p) => p + 1);
				setLastErr(undefined);
				const r = await runtime.kitClient.setField(rustTarget.kind, rustTarget.guid, rustTarget.field, resolved);
				setPending((p) => p - 1);
				if (!r.ok) {
					setLastErr(r.error);
					runtime.pushSetRejection(r.error);
					return r;
				}
				return r;
			}
			if (!classicWritable) {
				return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
			}
			runtime.setFieldValue(typeName, fieldName, resolved, guidValue, scope);
			return { ok: true } as const;
		},
		[runtime, rustTarget, classicWritable, typeName, fieldName, guidValue, scope, value],
	);

	let status: WriteStatus;
	if (rustTarget && runtime.kitClient) {
		if (!runtime.canWrite) {
			status = { kind: "readonly", pending: 0 };
		} else if (pending > 0) {
			status = { kind: "pending", pending, lastError: lastErr };
		} else if (lastErr) {
			status = { kind: "error", pending: 0, lastError: lastErr };
		} else {
			status = { kind: "idle", pending: 0 };
		}
	} else {
		status = classicWritable ? { kind: "idle", pending: 0 } : { kind: "readonly", pending: 0 };
	}

	return [value, setValue, status] as const;
}

// #endregion ⚛️Core Hooks

// #region ⚛️Direct Domain Exports

export function useJSON(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("JSON", guidValue);
}

export function useActorKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ActorKind", guidValue);
}

export function useActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Actor", guidValue);
}

export function useActorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "id", guidValue);
}

export function useActorName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "name", guidValue);
}

export function useActorEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "email", guidValue);
}

export function useActorColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "color", guidValue);
}

export function useUser(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("User", guidValue);
}

export function useUserHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "hash", guidValue);
}

export function useUserId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "id", guidValue);
}

export function useUserName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "name", guidValue);
}

export function useUserEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "email", guidValue);
}

export function useUserColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "color", guidValue);
}

export function useAgent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Agent", guidValue);
}

export function useAgentHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "hash", guidValue);
}

export function useAgentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "id", guidValue);
}

export function useAgentLlm(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "llm", guidValue);
}

export function useAgentName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "name", guidValue);
}

export function useAgentEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "email", guidValue);
}

export function useAgentColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "color", guidValue);
}

export function useSessionActorInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionActorInput", guidValue);
}

export function useSessionActorInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "id", guidValue);
}

export function useSessionActorInputKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "kind", guidValue);
}

export function useSessionActorInputLlm(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "llm", guidValue);
}

export function useSessionActorInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "name", guidValue);
}

export function useSessionActorInputEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "email", guidValue);
}

export function useSessionActorInputColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "color", guidValue);
}

export function useCoordinate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Coordinate", guidValue);
}

export function useCoordinateHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "hash", guidValue);
}

export function useCoordinateU(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "u", guidValue);
}

export function useCoordinateV(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "v", guidValue);
}

export function useCoordinateInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CoordinateInput", guidValue);
}

export function useCoordinateInputU(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CoordinateInput", "u", guidValue);
}

export function useCoordinateInputV(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CoordinateInput", "v", guidValue);
}

export function usePoint(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Point", guidValue);
}

export function usePointHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "hash", guidValue);
}

export function usePointX(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "x", guidValue);
}

export function usePointY(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "y", guidValue);
}

export function usePointZ(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "z", guidValue);
}

export function usePointInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PointInput", guidValue);
}

export function usePointInputX(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "x", guidValue);
}

export function usePointInputY(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "y", guidValue);
}

export function usePointInputZ(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "z", guidValue);
}

export function useVector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Vector", guidValue);
}

export function useVectorHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "hash", guidValue);
}

export function useVectorX(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "x", guidValue);
}

export function useVectorY(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "y", guidValue);
}

export function useVectorZ(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "z", guidValue);
}

export function useVectorInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VectorInput", guidValue);
}

export function useVectorInputX(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "x", guidValue);
}

export function useVectorInputY(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "y", guidValue);
}

export function useVectorInputZ(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "z", guidValue);
}

export function usePlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Plane", guidValue);
}

export function usePlaneHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "hash", guidValue);
}

export function usePlaneOrigin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "origin", guidValue);
}

export function usePlaneXAxis(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "xAxis", guidValue);
}

export function usePlaneYAxis(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "yAxis", guidValue);
}

export function usePlaneInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PlaneInput", guidValue);
}

export function usePlaneInputOrigin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "origin", guidValue);
}

export function usePlaneInputXAxis(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "xAxis", guidValue);
}

export function usePlaneInputYAxis(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "yAxis", guidValue);
}

export function useCamera(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Camera", guidValue);
}

export function useCameraHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "hash", guidValue);
}

export function useCameraPosition(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "position", guidValue);
}

export function useCameraForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "forward", guidValue);
}

export function useCameraUp(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "up", guidValue);
}

export function useCameraInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CameraInput", guidValue);
}

export function useCameraInputPosition(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "position", guidValue);
}

export function useCameraInputForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "forward", guidValue);
}

export function useCameraInputUp(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "up", guidValue);
}

export function useAttribute(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Attribute", guidValue);
}

export function useAttributeHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "hash", guidValue);
}

export function useAttributeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "id", guidValue);
}

export function useAttributeKey(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "key", guidValue);
}

export function useAttributeValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "value", guidValue);
}

export function useAttributeDefinition(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "definition", guidValue);
}

export function useAttributeInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AttributeInput", guidValue);
}

export function useAttributeInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "id", guidValue);
}

export function useAttributeInputKey(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "key", guidValue);
}

export function useAttributeInputValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "value", guidValue);
}

export function useAttributeInputDefinition(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "definition", guidValue);
}

export function useLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Location", guidValue);
}

export function useLocationHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "hash", guidValue);
}

export function useLocationLongitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "longitude", guidValue);
}

export function useLocationLatitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "latitude", guidValue);
}

export function useLocationAltitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "altitude", guidValue);
}

export function useLocationAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "attributes", guidValue);
}

export function useLocationInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("LocationInput", guidValue);
}

export function useLocationInputLongitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "longitude", guidValue);
}

export function useLocationInputLatitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "latitude", guidValue);
}

export function useLocationInputAltitude(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "altitude", guidValue);
}

export function useLocationInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "attributes", guidValue);
}

export function useAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Author", guidValue);
}

export function useAuthorHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "hash", guidValue);
}

export function useAuthorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "id", guidValue);
}

export function useAuthorName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "name", guidValue);
}

export function useAuthorEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "email", guidValue);
}

export function useAuthorAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "attributes", guidValue);
}

export function useAuthorInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AuthorInput", guidValue);
}

export function useAuthorInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "id", guidValue);
}

export function useAuthorInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "name", guidValue);
}

export function useAuthorInputEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "email", guidValue);
}

export function useAuthorInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "attributes", guidValue);
}

export function useAuthorPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AuthorPatchInput", guidValue);
}

export function useAuthorPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "name", guidValue);
}

export function useAuthorPatchInputEmail(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "email", guidValue);
}

export function useAuthorPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "attributes", guidValue);
}

export function useFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Folder", guidValue);
}

export function useFolderHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "hash", guidValue);
}

export function useFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "id", guidValue);
}

export function useFolderKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "kit", guidValue);
}

export function useFolderName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "name", guidValue);
}

export function useFolderParent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "parent", guidValue);
}

export function useFolderChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "children", guidValue);
}

export function useFolderDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "description", guidValue);
}

export function useFolderAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "attributes", guidValue);
}

export function useFolderCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "createdAt", guidValue);
}

export function useFolderCreatedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "createdBy", guidValue);
}

export function useFolderUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "updatedAt", guidValue);
}

export function useFolderUpdatedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "updatedBy", guidValue);
}

export function useFolderInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FolderInput", guidValue);
}

export function useFolderInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "id", guidValue);
}

export function useFolderInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "name", guidValue);
}

export function useFolderInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "parentId", guidValue);
}

export function useFolderInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "description", guidValue);
}

export function useFolderInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "attributes", guidValue);
}

export function useFolderInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "createdAt", guidValue);
}

export function useFolderInputCreatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "createdById", guidValue);
}

export function useFolderInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "updatedAt", guidValue);
}

export function useFolderInputUpdatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "updatedById", guidValue);
}

export function useFolderPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FolderPatchInput", guidValue);
}

export function useFolderPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "name", guidValue);
}

export function useFolderPatchInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "parentId", guidValue);
}

export function useFolderPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "description", guidValue);
}

export function useFolderPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "attributes", guidValue);
}

export function useFolderPatchInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "createdAt", guidValue);
}

export function useFolderPatchInputCreatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "createdById", guidValue);
}

export function useFolderPatchInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "updatedAt", guidValue);
}

export function useFolderPatchInputUpdatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "updatedById", guidValue);
}

export function useFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("File", guidValue);
}

export function useFileHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "hash", guidValue);
}

export function useFileId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "id", guidValue);
}

export function useFileKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "kit", guidValue);
}

export function useFileName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "name", guidValue);
}

export function useFileRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "remote", guidValue);
}

export function useFileFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "folder", guidValue);
}

export function useFileSize(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "size", guidValue);
}

export function useFileContentHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "contentHash", guidValue);
}

export function useFileBlob(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "blob", guidValue);
}

export function useFileMime(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "mime", guidValue);
}

export function useFileCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "createdAt", guidValue);
}

export function useFileCreatedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "createdBy", guidValue);
}

export function useFileUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "updatedAt", guidValue);
}

export function useFileUpdatedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "updatedBy", guidValue);
}

export function useFileInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FileInput", guidValue);
}

export function useFileInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "id", guidValue);
}

export function useFileInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "name", guidValue);
}

export function useFileInputRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "remote", guidValue);
}

export function useFileInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "folderId", guidValue);
}

export function useFileInputSize(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "size", guidValue);
}

export function useFileInputContentHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "contentHash", guidValue);
}

export function useFileInputBlob(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "blob", guidValue);
}

export function useFileInputMime(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "mime", guidValue);
}

export function useFileInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "createdAt", guidValue);
}

export function useFileInputCreatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "createdById", guidValue);
}

export function useFileInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "updatedAt", guidValue);
}

export function useFileInputUpdatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "updatedById", guidValue);
}

export function useFilePatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FilePatchInput", guidValue);
}

export function useFilePatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "name", guidValue);
}

export function useFilePatchInputRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "remote", guidValue);
}

export function useFilePatchInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "folderId", guidValue);
}

export function useFilePatchInputSize(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "size", guidValue);
}

export function useFilePatchInputContentHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "contentHash", guidValue);
}

export function useFilePatchInputBlob(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "blob", guidValue);
}

export function useFilePatchInputMime(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "mime", guidValue);
}

export function useFilePatchInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "createdAt", guidValue);
}

export function useFilePatchInputCreatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "createdById", guidValue);
}

export function useFilePatchInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "updatedAt", guidValue);
}

export function useFilePatchInputUpdatedById(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "updatedById", guidValue);
}

export function useBenchmark(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Benchmark", guidValue);
}

export function useBenchmarkHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "hash", guidValue);
}

export function useBenchmarkId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "id", guidValue);
}

export function useBenchmarkQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "quality", guidValue);
}

export function useBenchmarkName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "name", guidValue);
}

export function useBenchmarkIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "icon", guidValue);
}

export function useBenchmarkMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "min", guidValue);
}

export function useBenchmarkMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "minExcluded", guidValue);
}

export function useBenchmarkMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "max", guidValue);
}

export function useBenchmarkMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "maxExcluded", guidValue);
}

export function useBenchmarkAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "attributes", guidValue);
}

export function useBenchmarkInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BenchmarkInput", guidValue);
}

export function useBenchmarkInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "id", guidValue);
}

export function useBenchmarkInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "name", guidValue);
}

export function useBenchmarkInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "icon", guidValue);
}

export function useBenchmarkInputMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "min", guidValue);
}

export function useBenchmarkInputMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "minExcluded", guidValue);
}

export function useBenchmarkInputMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "max", guidValue);
}

export function useBenchmarkInputMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "maxExcluded", guidValue);
}

export function useBenchmarkInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "attributes", guidValue);
}

export function useQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Quality", guidValue);
}

export function useQualityHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "hash", guidValue);
}

export function useQualityId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "id", guidValue);
}

export function useQualityKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "kit", guidValue);
}

export function useQualityKey(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "key", guidValue);
}

export function useQualityName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "name", guidValue);
}

export function useQualityDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "description", guidValue);
}

export function useQualityUri(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "uri", guidValue);
}

export function useQualityKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "kind", guidValue);
}

export function useQualityFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "folder", guidValue);
}

export function useQualityCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "canScale", guidValue);
}

export function useQualityDefaultSiUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultSiUnit", guidValue);
}

export function useQualityDefaultImperialUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultImperialUnit", guidValue);
}

export function useQualityMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "min", guidValue);
}

export function useQualityIsMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "isMinExcluded", guidValue);
}

export function useQualityMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "max", guidValue);
}

export function useQualityIsMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "isMaxExcluded", guidValue);
}

export function useQualityDefaultValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultValue", guidValue);
}

export function useQualityFormula(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "formula", guidValue);
}

export function useQualityIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "icon", guidValue);
}

export function useQualityImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "image", guidValue);
}

export function useQualityUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "unit", guidValue);
}

export function useQualityBenchmarks(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "benchmarks", guidValue);
}

export function useQualityAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "attributes", guidValue);
}

export function useQualityInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("QualityInput", guidValue);
}

export function useQualityInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "id", guidValue);
}

export function useQualityInputKey(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "key", guidValue);
}

export function useQualityInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "name", guidValue);
}

export function useQualityInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "description", guidValue);
}

export function useQualityInputUri(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "uri", guidValue);
}

export function useQualityInputKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "kind", guidValue);
}

export function useQualityInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "folderId", guidValue);
}

export function useQualityInputCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "canScale", guidValue);
}

export function useQualityInputDefaultSiUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultSiUnit", guidValue);
}

export function useQualityInputDefaultImperialUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultImperialUnit", guidValue);
}

export function useQualityInputMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "min", guidValue);
}

export function useQualityInputIsMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "isMinExcluded", guidValue);
}

export function useQualityInputMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "max", guidValue);
}

export function useQualityInputIsMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "isMaxExcluded", guidValue);
}

export function useQualityInputDefaultValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultValue", guidValue);
}

export function useQualityInputFormula(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "formula", guidValue);
}

export function useQualityInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "icon", guidValue);
}

export function useQualityInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "image", guidValue);
}

export function useQualityInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "unit", guidValue);
}

export function useQualityInputBenchmarks(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "benchmarks", guidValue);
}

export function useQualityInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "attributes", guidValue);
}

export function useQualityPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("QualityPatchInput", guidValue);
}

export function useQualityPatchInputKey(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "key", guidValue);
}

export function useQualityPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "name", guidValue);
}

export function useQualityPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "description", guidValue);
}

export function useQualityPatchInputUri(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "uri", guidValue);
}

export function useQualityPatchInputKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "kind", guidValue);
}

export function useQualityPatchInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "folderId", guidValue);
}

export function useQualityPatchInputCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "canScale", guidValue);
}

export function useQualityPatchInputDefaultSiUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultSiUnit", guidValue);
}

export function useQualityPatchInputDefaultImperialUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultImperialUnit", guidValue);
}

export function useQualityPatchInputMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "min", guidValue);
}

export function useQualityPatchInputIsMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "isMinExcluded", guidValue);
}

export function useQualityPatchInputMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "max", guidValue);
}

export function useQualityPatchInputIsMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "isMaxExcluded", guidValue);
}

export function useQualityPatchInputDefaultValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultValue", guidValue);
}

export function useQualityPatchInputFormula(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "formula", guidValue);
}

export function useQualityPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "icon", guidValue);
}

export function useQualityPatchInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "image", guidValue);
}

export function useQualityPatchInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "unit", guidValue);
}

export function useQualityPatchInputBenchmarks(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "benchmarks", guidValue);
}

export function useQualityPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "attributes", guidValue);
}

export function usePort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Port", guidValue);
}

export function usePortHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "hash", guidValue);
}

export function usePortId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "id", guidValue);
}

export function usePortKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "kit", guidValue);
}

export function usePortName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "name", guidValue);
}

export function usePortDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "description", guidValue);
}

export function usePortIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "icon", guidValue);
}

export function usePortMaxChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "maxChildren", guidValue);
}

export function usePortCompatiblePorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "compatiblePorts", guidValue);
}

export function usePortAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "attributes", guidValue);
}

export function usePortInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PortInput", guidValue);
}

export function usePortInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "id", guidValue);
}

export function usePortInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "name", guidValue);
}

export function usePortInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "description", guidValue);
}

export function usePortInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "icon", guidValue);
}

export function usePortInputMaxChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "maxChildren", guidValue);
}

export function usePortInputCompatiblePortIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "compatiblePortIds", guidValue);
}

export function usePortInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "attributes", guidValue);
}

export function usePortPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PortPatchInput", guidValue);
}

export function usePortPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "name", guidValue);
}

export function usePortPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "description", guidValue);
}

export function usePortPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "icon", guidValue);
}

export function usePortPatchInputMaxChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "maxChildren", guidValue);
}

export function usePortPatchInputCompatiblePortIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "compatiblePortIds", guidValue);
}

export function usePortPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "attributes", guidValue);
}

export function useProp(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Prop", guidValue);
}

export function usePropHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "hash", guidValue);
}

export function usePropId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "id", guidValue);
}

export function usePropKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "kit", guidValue);
}

export function usePropQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "quality", guidValue);
}

export function usePropValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "value", guidValue);
}

export function usePropUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "unit", guidValue);
}

export function usePropAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "attributes", guidValue);
}

export function usePropInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PropInput", guidValue);
}

export function usePropInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "id", guidValue);
}

export function usePropInputQualityId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "qualityId", guidValue);
}

export function usePropInputValue(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "value", guidValue);
}

export function usePropInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "unit", guidValue);
}

export function usePropInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "attributes", guidValue);
}

export function useTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Tag", guidValue);
}

export function useTagHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "hash", guidValue);
}

export function useTagId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "id", guidValue);
}

export function useTagKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "kit", guidValue);
}

export function useTagName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "name", guidValue);
}

export function useTagDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "description", guidValue);
}

export function useTagIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "icon", guidValue);
}

export function useTagAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "attributes", guidValue);
}

export function useTagInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TagInput", guidValue);
}

export function useTagInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "id", guidValue);
}

export function useTagInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "name", guidValue);
}

export function useTagInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "description", guidValue);
}

export function useTagInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "icon", guidValue);
}

export function useTagInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "attributes", guidValue);
}

export function useTagPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TagPatchInput", guidValue);
}

export function useTagPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "name", guidValue);
}

export function useTagPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "description", guidValue);
}

export function useTagPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "icon", guidValue);
}

export function useTagPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "attributes", guidValue);
}

export function useConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Concept", guidValue);
}

export function useConceptHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "hash", guidValue);
}

export function useConceptId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "id", guidValue);
}

export function useConceptKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "kit", guidValue);
}

export function useConceptName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "name", guidValue);
}

export function useConceptDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "description", guidValue);
}

export function useConceptIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "icon", guidValue);
}

export function useConceptAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "attributes", guidValue);
}

export function useConceptInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConceptInput", guidValue);
}

export function useConceptInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "id", guidValue);
}

export function useConceptInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "name", guidValue);
}

export function useConceptInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "description", guidValue);
}

export function useConceptInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "icon", guidValue);
}

export function useConceptInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "attributes", guidValue);
}

export function useConceptPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConceptPatchInput", guidValue);
}

export function useConceptPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "name", guidValue);
}

export function useConceptPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "description", guidValue);
}

export function useConceptPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "icon", guidValue);
}

export function useConceptPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "attributes", guidValue);
}

export function useFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Family", guidValue);
}

export function useFamilyHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "hash", guidValue);
}

export function useFamilyId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "id", guidValue);
}

export function useFamilyKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "kit", guidValue);
}

export function useFamilyName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "name", guidValue);
}

export function useFamilyDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "description", guidValue);
}

export function useFamilyIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "icon", guidValue);
}

export function useFamilyPorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "ports", guidValue);
}

export function useFamilyAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "attributes", guidValue);
}

export function useFamilyInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FamilyInput", guidValue);
}

export function useFamilyInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "id", guidValue);
}

export function useFamilyInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "name", guidValue);
}

export function useFamilyInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "description", guidValue);
}

export function useFamilyInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "icon", guidValue);
}

export function useFamilyInputPorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "ports", guidValue);
}

export function useFamilyInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "attributes", guidValue);
}

export function useFamilyPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FamilyPatchInput", guidValue);
}

export function useFamilyPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "name", guidValue);
}

export function useFamilyPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "description", guidValue);
}

export function useFamilyPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "icon", guidValue);
}

export function useFamilyPatchInputPorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "ports", guidValue);
}

export function useFamilyPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "attributes", guidValue);
}

export function useRepresentation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Representation", guidValue);
}

export function useRepresentationHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "hash", guidValue);
}

export function useRepresentationId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "id", guidValue);
}

export function useRepresentationType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "type", guidValue);
}

export function useRepresentationName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "name", guidValue);
}

export function useRepresentationTags(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "tags", guidValue);
}

export function useRepresentationFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "file", guidValue);
}

export function useRepresentationDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "description", guidValue);
}

export function useRepresentationAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "attributes", guidValue);
}

export function useRepresentationInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("RepresentationInput", guidValue);
}

export function useRepresentationInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "id", guidValue);
}

export function useRepresentationInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "name", guidValue);
}

export function useRepresentationInputTagIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "tagIds", guidValue);
}

export function useRepresentationInputFileId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "fileId", guidValue);
}

export function useRepresentationInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "description", guidValue);
}

export function useRepresentationInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "attributes", guidValue);
}

export function useConnector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Connector", guidValue);
}

export function useConnectorHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "hash", guidValue);
}

export function useConnectorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "id", guidValue);
}

export function useConnectorType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "type", guidValue);
}

export function useConnectorName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "name", guidValue);
}

export function useConnectorT(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "t", guidValue);
}

export function useConnectorPoint(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "point", guidValue);
}

export function useConnectorDirection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "direction", guidValue);
}

export function useConnectorDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "description", guidValue);
}

export function useConnectorPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "port", guidValue);
}

export function useConnectorMandatory(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "mandatory", guidValue);
}

export function useConnectorMaxChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "maxChildren", guidValue);
}

export function useConnectorProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "props", guidValue);
}

export function useConnectorAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "attributes", guidValue);
}

export function useConnectorPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "pieces", guidValue);
}

export function useConnectorCompatibleConnectors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "compatibleConnectors", guidValue);
}

export function useConnectorInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectorInput", guidValue);
}

export function useConnectorInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "id", guidValue);
}

export function useConnectorInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "name", guidValue);
}

export function useConnectorInputT(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "t", guidValue);
}

export function useConnectorInputPoint(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "point", guidValue);
}

export function useConnectorInputDirection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "direction", guidValue);
}

export function useConnectorInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "description", guidValue);
}

export function useConnectorInputPortId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "portId", guidValue);
}

export function useConnectorInputMandatory(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "mandatory", guidValue);
}

export function useConnectorInputMaxChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "maxChildren", guidValue);
}

export function useConnectorInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "props", guidValue);
}

export function useConnectorInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "attributes", guidValue);
}

export function useType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Type", guidValue);
}

export function useTypeHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "hash", guidValue);
}

export function useTypeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "id", guidValue);
}

export function useTypeKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "kit", guidValue);
}

export function useTypeName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "name", guidValue);
}

export function useTypeParent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "parent", guidValue);
}

export function useTypeChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "children", guidValue);
}

export function useTypeIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "isAbstract", guidValue);
}

export function useTypeFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "folder", guidValue);
}

export function useTypeRepresentations(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "representations", guidValue);
}

export function useTypeConnectors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "connectors", guidValue);
}

export function useTypeProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "props", guidValue);
}

export function useTypeStock(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "stock", guidValue);
}

export function useTypeVirtual(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "virtual", guidValue);
}

export function useTypeUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "unit", guidValue);
}

export function useTypeCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "createdAt", guidValue);
}

export function useTypeUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "updatedAt", guidValue);
}

export function useTypeLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "location", guidValue);
}

export function useTypeAuthors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "authors", guidValue);
}

export function useTypeConcepts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "concepts", guidValue);
}

export function useTypeIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "icon", guidValue);
}

export function useTypeImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "image", guidValue);
}

export function useTypeDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "description", guidValue);
}

export function useTypeAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "attributes", guidValue);
}

export function useTypeFixedPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "fixedPieces", guidValue);
}

export function useTypeInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TypeInput", guidValue);
}

export function useTypeInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "id", guidValue);
}

export function useTypeInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "name", guidValue);
}

export function useTypeInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "parentId", guidValue);
}

export function useTypeInputIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "isAbstract", guidValue);
}

export function useTypeInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "folderId", guidValue);
}

export function useTypeInputRepresentations(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "representations", guidValue);
}

export function useTypeInputConnectors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "connectors", guidValue);
}

export function useTypeInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "props", guidValue);
}

export function useTypeInputStock(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "stock", guidValue);
}

export function useTypeInputVirtual(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "virtual", guidValue);
}

export function useTypeInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "unit", guidValue);
}

export function useTypeInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "createdAt", guidValue);
}

export function useTypeInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "updatedAt", guidValue);
}

export function useTypeInputLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "location", guidValue);
}

export function useTypeInputAuthorIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "authorIds", guidValue);
}

export function useTypeInputConceptIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "conceptIds", guidValue);
}

export function useTypeInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "icon", guidValue);
}

export function useTypeInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "image", guidValue);
}

export function useTypeInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "description", guidValue);
}

export function useTypeInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "attributes", guidValue);
}

export function useTypePatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TypePatchInput", guidValue);
}

export function useTypePatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "name", guidValue);
}

export function useTypePatchInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "parentId", guidValue);
}

export function useTypePatchInputIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "isAbstract", guidValue);
}

export function useTypePatchInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "folderId", guidValue);
}

export function useTypePatchInputRepresentations(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "representations", guidValue);
}

export function useTypePatchInputConnectors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "connectors", guidValue);
}

export function useTypePatchInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "props", guidValue);
}

export function useTypePatchInputStock(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "stock", guidValue);
}

export function useTypePatchInputVirtual(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "virtual", guidValue);
}

export function useTypePatchInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "unit", guidValue);
}

export function useTypePatchInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "createdAt", guidValue);
}

export function useTypePatchInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "updatedAt", guidValue);
}

export function useTypePatchInputLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "location", guidValue);
}

export function useTypePatchInputAuthorIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "authorIds", guidValue);
}

export function useTypePatchInputConceptIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "conceptIds", guidValue);
}

export function useTypePatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "icon", guidValue);
}

export function useTypePatchInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "image", guidValue);
}

export function useTypePatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "description", guidValue);
}

export function useTypePatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "attributes", guidValue);
}

export function useLayer(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Layer", guidValue);
}

export function useLayerHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "hash", guidValue);
}

export function useLayerId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "id", guidValue);
}

export function useLayerDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "design", guidValue);
}

export function useLayerPath(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "path", guidValue);
}

export function useLayerIsHidden(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "isHidden", guidValue);
}

export function useLayerIsLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "isLocked", guidValue);
}

export function useLayerColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "color", guidValue);
}

export function useLayerDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "description", guidValue);
}

export function useLayerAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "attributes", guidValue);
}

export function useLayerInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("LayerInput", guidValue);
}

export function useLayerInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "id", guidValue);
}

export function useLayerInputPath(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "path", guidValue);
}

export function useLayerInputIsHidden(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "isHidden", guidValue);
}

export function useLayerInputIsLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "isLocked", guidValue);
}

export function useLayerInputColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "color", guidValue);
}

export function useLayerInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "description", guidValue);
}

export function useLayerInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "attributes", guidValue);
}

export function useSide(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Side", guidValue);
}

export function useSideHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "hash", guidValue);
}

export function useSideConnection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "connection", guidValue);
}

export function useSidePiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "piece", guidValue);
}

export function useSideDesignPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "designPiece", guidValue);
}

export function useSideConnector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "connector", guidValue);
}

export function useSideInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SideInput", guidValue);
}

export function useSideInputPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "pieceId", guidValue);
}

export function useSideInputDesignPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "designPieceId", guidValue);
}

export function useSideInputConnectorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "connectorId", guidValue);
}

export function useConnection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Connection", guidValue);
}

export function useConnectionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "hash", guidValue);
}

export function useConnectionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "id", guidValue);
}

export function useConnectionDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "design", guidValue);
}

export function useConnectionConnected(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "connected", guidValue);
}

export function useConnectionConnecting(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "connecting", guidValue);
}

export function useConnectionGap(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "gap", guidValue);
}

export function useConnectionShift(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "shift", guidValue);
}

export function useConnectionRise(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "rise", guidValue);
}

export function useConnectionRotation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "rotation", guidValue);
}

export function useConnectionTurn(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "turn", guidValue);
}

export function useConnectionTilt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "tilt", guidValue);
}

export function useConnectionU(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "u", guidValue);
}

export function useConnectionV(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "v", guidValue);
}

export function useConnectionDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "description", guidValue);
}

export function useConnectionAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "attributes", guidValue);
}

export function useConnectionChildPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "childPiece", guidValue);
}

export function useConnectionChildConnector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "childConnector", guidValue);
}

export function useConnectionParentPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "parentPiece", guidValue);
}

export function useConnectionParentConnector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "parentConnector", guidValue);
}

export function useConnectionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionInput", guidValue);
}

export function useConnectionInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "id", guidValue);
}

export function useConnectionInputConnected(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "connected", guidValue);
}

export function useConnectionInputConnecting(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "connecting", guidValue);
}

export function useConnectionInputGap(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "gap", guidValue);
}

export function useConnectionInputShift(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "shift", guidValue);
}

export function useConnectionInputRise(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "rise", guidValue);
}

export function useConnectionInputRotation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "rotation", guidValue);
}

export function useConnectionInputTurn(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "turn", guidValue);
}

export function useConnectionInputTilt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "tilt", guidValue);
}

export function useConnectionInputU(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "u", guidValue);
}

export function useConnectionInputV(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "v", guidValue);
}

export function useConnectionInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "description", guidValue);
}

export function useConnectionInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "attributes", guidValue);
}

export function useConnectionPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionPatchInput", guidValue);
}

export function useConnectionPatchInputConnected(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "connected", guidValue);
}

export function useConnectionPatchInputConnecting(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "connecting", guidValue);
}

export function useConnectionPatchInputGap(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "gap", guidValue);
}

export function useConnectionPatchInputShift(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "shift", guidValue);
}

export function useConnectionPatchInputRise(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "rise", guidValue);
}

export function useConnectionPatchInputRotation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "rotation", guidValue);
}

export function useConnectionPatchInputTurn(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "turn", guidValue);
}

export function useConnectionPatchInputTilt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "tilt", guidValue);
}

export function useConnectionPatchInputU(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "u", guidValue);
}

export function useConnectionPatchInputV(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "v", guidValue);
}

export function useConnectionPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "description", guidValue);
}

export function useConnectionPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "attributes", guidValue);
}

export function useStat(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Stat", guidValue);
}

export function useStatHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "hash", guidValue);
}

export function useStatId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "id", guidValue);
}

export function useStatDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "design", guidValue);
}

export function useStatQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "quality", guidValue);
}

export function useStatUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "unit", guidValue);
}

export function useStatMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "min", guidValue);
}

export function useStatMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "minExcluded", guidValue);
}

export function useStatMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "max", guidValue);
}

export function useStatMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "maxExcluded", guidValue);
}

export function useStatInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StatInput", guidValue);
}

export function useStatInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "id", guidValue);
}

export function useStatInputQualityId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "qualityId", guidValue);
}

export function useStatInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "unit", guidValue);
}

export function useStatInputMin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "min", guidValue);
}

export function useStatInputMinExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "minExcluded", guidValue);
}

export function useStatInputMax(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "max", guidValue);
}

export function useStatInputMaxExcluded(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "maxExcluded", guidValue);
}

export function usePieceKindEnum(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceKind", guidValue);
}

export function useBlueprint(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Blueprint", guidValue);
}

export function useBlueprintType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Blueprint", "type", guidValue);
}

export function useBlueprintDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Blueprint", "design", guidValue);
}

export function usePiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Piece", guidValue);
}

export function usePieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "id", guidValue);
}

export function usePieceHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "hash", guidValue);
}

export function usePieceName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "name", guidValue);
}

export function usePiecePlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "plane", guidValue);
}

export function usePieceCenter(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "center", guidValue);
}

export function usePieceScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "scale", guidValue);
}

export function usePieceMirrorPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "mirrorPlane", guidValue);
}

export function usePieceIsHidden(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "isHidden", guidValue);
}

export function usePieceIsLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "isLocked", guidValue);
}

export function usePieceColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "color", guidValue);
}

export function usePieceDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "description", guidValue);
}

export function usePieceKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "kind", guidValue);
}

export function usePieceType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "type", guidValue);
}

export function usePieceDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "design", guidValue);
}

export function usePieceProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "props", guidValue);
}

export function usePieceAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "attributes", guidValue);
}

export function usePieceFlatPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "flatPlane", guidValue);
}

export function usePieceFlatCenter(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "flatCenter", guidValue);
}

export function usePieceParentPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "parentPiece", guidValue);
}

export function usePieceParentConnection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "parentConnection", guidValue);
}

export function usePieceChildPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "childPieces", guidValue);
}

export function usePieceChildConnections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "childConnections", guidValue);
}

export function usePieceAlternatives(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternatives", guidValue);
}

export function usePieceAlternativeTypes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternativeTypes", guidValue);
}

export function usePieceAlternativeDesigns(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternativeDesigns", guidValue);
}

export function usePieceInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceInput", guidValue);
}

export function usePieceInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "id", guidValue);
}

export function usePieceInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "name", guidValue);
}

export function usePieceInputTypeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "typeId", guidValue);
}

export function usePieceInputDesignReferenceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "designReferenceId", guidValue);
}

export function usePieceInputPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "plane", guidValue);
}

export function usePieceInputCenter(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "center", guidValue);
}

export function usePieceInputScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "scale", guidValue);
}

export function usePieceInputMirrorPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "mirrorPlane", guidValue);
}

export function usePieceInputIsHidden(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "isHidden", guidValue);
}

export function usePieceInputIsLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "isLocked", guidValue);
}

export function usePieceInputColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "color", guidValue);
}

export function usePieceInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "description", guidValue);
}

export function usePieceInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "props", guidValue);
}

export function usePieceInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "attributes", guidValue);
}

export function usePiecePatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PiecePatchInput", guidValue);
}

export function usePiecePatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "name", guidValue);
}

export function usePiecePatchInputTypeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "typeId", guidValue);
}

export function usePiecePatchInputDesignReferenceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "designReferenceId", guidValue);
}

export function usePiecePatchInputPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "plane", guidValue);
}

export function usePiecePatchInputCenter(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "center", guidValue);
}

export function usePiecePatchInputScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "scale", guidValue);
}

export function usePiecePatchInputMirrorPlane(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "mirrorPlane", guidValue);
}

export function usePiecePatchInputIsHidden(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "isHidden", guidValue);
}

export function usePiecePatchInputIsLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "isLocked", guidValue);
}

export function usePiecePatchInputColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "color", guidValue);
}

export function usePiecePatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "description", guidValue);
}

export function usePiecePatchInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "props", guidValue);
}

export function usePiecePatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "attributes", guidValue);
}

export function useGroup(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Group", guidValue);
}

export function useGroupHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "hash", guidValue);
}

export function useGroupId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "id", guidValue);
}

export function useGroupDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "design", guidValue);
}

export function useGroupPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "pieces", guidValue);
}

export function useGroupColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "color", guidValue);
}

export function useGroupName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "name", guidValue);
}

export function useGroupDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "description", guidValue);
}

export function useGroupAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "attributes", guidValue);
}

export function useGroupInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("GroupInput", guidValue);
}

export function useGroupInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "id", guidValue);
}

export function useGroupInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "pieceIds", guidValue);
}

export function useGroupInputColor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "color", guidValue);
}

export function useGroupInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "name", guidValue);
}

export function useGroupInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "description", guidValue);
}

export function useGroupInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "attributes", guidValue);
}

export function useDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Design", guidValue);
}

export function useDesignHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "hash", guidValue);
}

export function useDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "id", guidValue);
}

export function useDesignKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "kit", guidValue);
}

export function useDesignName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "name", guidValue);
}

export function useDesignParent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "parent", guidValue);
}

export function useDesignChildren(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "children", guidValue);
}

export function useDesignIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "isAbstract", guidValue);
}

export function useDesignFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "folder", guidValue);
}

export function useDesignPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "pieces", guidValue);
}

export function useDesignConnections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "connections", guidValue);
}

export function useDesignStats(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "stats", guidValue);
}

export function useDesignProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "props", guidValue);
}

export function useDesignLayers(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "layers", guidValue);
}

export function useDesignActiveLayer(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "activeLayer", guidValue);
}

export function useDesignGroups(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "groups", guidValue);
}

export function useDesignCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "canScale", guidValue);
}

export function useDesignCanMirror(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "canMirror", guidValue);
}

export function useDesignUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "unit", guidValue);
}

export function useDesignLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "location", guidValue);
}

export function useDesignAuthors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "authors", guidValue);
}

export function useDesignConcepts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "concepts", guidValue);
}

export function useDesignIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "icon", guidValue);
}

export function useDesignImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "image", guidValue);
}

export function useDesignDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "description", guidValue);
}

export function useDesignAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "attributes", guidValue);
}

export function useDesignCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "createdAt", guidValue);
}

export function useDesignUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "updatedAt", guidValue);
}

export function useDesignInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DesignInput", guidValue);
}

export function useDesignInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "id", guidValue);
}

export function useDesignInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "name", guidValue);
}

export function useDesignInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "parentId", guidValue);
}

export function useDesignInputIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "isAbstract", guidValue);
}

export function useDesignInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "folderId", guidValue);
}

export function useDesignInputPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "pieces", guidValue);
}

export function useDesignInputConnections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "connections", guidValue);
}

export function useDesignInputStats(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "stats", guidValue);
}

export function useDesignInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "props", guidValue);
}

export function useDesignInputLayers(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "layers", guidValue);
}

export function useDesignInputActiveLayerId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "activeLayerId", guidValue);
}

export function useDesignInputGroups(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "groups", guidValue);
}

export function useDesignInputCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "canScale", guidValue);
}

export function useDesignInputCanMirror(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "canMirror", guidValue);
}

export function useDesignInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "unit", guidValue);
}

export function useDesignInputLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "location", guidValue);
}

export function useDesignInputAuthorIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "authorIds", guidValue);
}

export function useDesignInputConceptIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "conceptIds", guidValue);
}

export function useDesignInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "icon", guidValue);
}

export function useDesignInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "image", guidValue);
}

export function useDesignInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "description", guidValue);
}

export function useDesignInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "attributes", guidValue);
}

export function useDesignInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "createdAt", guidValue);
}

export function useDesignInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "updatedAt", guidValue);
}

export function useDesignPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DesignPatchInput", guidValue);
}

export function useDesignPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "name", guidValue);
}

export function useDesignPatchInputParentId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "parentId", guidValue);
}

export function useDesignPatchInputIsAbstract(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "isAbstract", guidValue);
}

export function useDesignPatchInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "folderId", guidValue);
}

export function useDesignPatchInputStats(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "stats", guidValue);
}

export function useDesignPatchInputProps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "props", guidValue);
}

export function useDesignPatchInputLayers(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "layers", guidValue);
}

export function useDesignPatchInputActiveLayerId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "activeLayerId", guidValue);
}

export function useDesignPatchInputGroups(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "groups", guidValue);
}

export function useDesignPatchInputCanScale(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "canScale", guidValue);
}

export function useDesignPatchInputCanMirror(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "canMirror", guidValue);
}

export function useDesignPatchInputUnit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "unit", guidValue);
}

export function useDesignPatchInputLocation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "location", guidValue);
}

export function useDesignPatchInputAuthorIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "authorIds", guidValue);
}

export function useDesignPatchInputConceptIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "conceptIds", guidValue);
}

export function useDesignPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "icon", guidValue);
}

export function useDesignPatchInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "image", guidValue);
}

export function useDesignPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "description", guidValue);
}

export function useDesignPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "attributes", guidValue);
}

export function useDesignPatchInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "createdAt", guidValue);
}

export function useDesignPatchInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "updatedAt", guidValue);
}

export function useKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Kit", guidValue);
}

export function useKitHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "hash", guidValue);
}

export function useKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "id", guidValue);
}

export function useKitName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "name", guidValue);
}

export function useKitRelease(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "release", guidValue);
}

export function useKitTypes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "types", guidValue);
}

export function useKitDesigns(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "designs", guidValue);
}

export function useKitTags(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "tags", guidValue);
}

export function useKitConcepts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "concepts", guidValue);
}

export function useKitFamilies(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "families", guidValue);
}

export function useKitPorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "ports", guidValue);
}

export function useKitQualities(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "qualities", guidValue);
}

export function useKitFiles(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "files", guidValue);
}

export function useKitFolders(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "folders", guidValue);
}

export function useKitAuthors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "authors", guidValue);
}

export function useKitRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "remote", guidValue);
}

export function useKitHomepage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "homepage", guidValue);
}

export function useKitLicense(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "license", guidValue);
}

export function useKitPreview(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "preview", guidValue);
}

export function useKitIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "icon", guidValue);
}

export function useKitImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "image", guidValue);
}

export function useKitDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "description", guidValue);
}

export function useKitAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "attributes", guidValue);
}

export function useKitCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "createdAt", guidValue);
}

export function useKitUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "updatedAt", guidValue);
}

export function useKitInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInput", guidValue);
}

export function useKitInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "id", guidValue);
}

export function useKitInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "name", guidValue);
}

export function useKitInputRelease(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "release", guidValue);
}

export function useKitInputTypes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "types", guidValue);
}

export function useKitInputDesigns(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "designs", guidValue);
}

export function useKitInputTags(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "tags", guidValue);
}

export function useKitInputConcepts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "concepts", guidValue);
}

export function useKitInputFamilies(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "families", guidValue);
}

export function useKitInputPorts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "ports", guidValue);
}

export function useKitInputQualities(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "qualities", guidValue);
}

export function useKitInputFiles(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "files", guidValue);
}

export function useKitInputFolders(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "folders", guidValue);
}

export function useKitInputAuthors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "authors", guidValue);
}

export function useKitInputRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "remote", guidValue);
}

export function useKitInputHomepage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "homepage", guidValue);
}

export function useKitInputLicense(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "license", guidValue);
}

export function useKitInputPreview(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "preview", guidValue);
}

export function useKitInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "icon", guidValue);
}

export function useKitInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "image", guidValue);
}

export function useKitInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "description", guidValue);
}

export function useKitInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "attributes", guidValue);
}

export function useKitInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "createdAt", guidValue);
}

export function useKitInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "updatedAt", guidValue);
}

export function useKitPatchInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitPatchInput", guidValue);
}

export function useKitPatchInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "name", guidValue);
}

export function useKitPatchInputRelease(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "release", guidValue);
}

export function useKitPatchInputRemote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "remote", guidValue);
}

export function useKitPatchInputHomepage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "homepage", guidValue);
}

export function useKitPatchInputLicense(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "license", guidValue);
}

export function useKitPatchInputPreview(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "preview", guidValue);
}

export function useKitPatchInputIcon(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "icon", guidValue);
}

export function useKitPatchInputImage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "image", guidValue);
}

export function useKitPatchInputDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "description", guidValue);
}

export function useKitPatchInputAttributes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "attributes", guidValue);
}

export function useKitPatchInputCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "createdAt", guidValue);
}

export function useKitPatchInputUpdatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "updatedAt", guidValue);
}

export function useBackboneKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BackboneKind", guidValue);
}

export function useKitBackbone(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitBackbone", guidValue);
}

export function useKitBackboneHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "hash", guidValue);
}

export function useKitBackboneKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "kind", guidValue);
}

export function useKitBackboneEndpoint(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "endpoint", guidValue);
}

export function useKitBackboneAuthoritative(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "authoritative", guidValue);
}

export function useKitBackboneLinearHistory(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "linearHistory", guidValue);
}

export function useKitBackboneConnected(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "connected", guidValue);
}

export function useKitBackboneTimeoutSeconds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "timeoutSeconds", guidValue);
}

export function useKitBackboneCurrentHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "currentHash", guidValue);
}

export function useKitBackboneLastInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "lastInteractionIndex", guidValue);
}

export function useKitBackbonePendingCandidateCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "pendingCandidateCount", guidValue);
}

export function useKitClientInfo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitClientInfo", guidValue);
}

export function useKitClientInfoHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "hash", guidValue);
}

export function useKitClientInfoId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "id", guidValue);
}

export function useKitClientInfoName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "name", guidValue);
}

export function useKitClientInfoVersion(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "version", guidValue);
}

export function useKitClientInfoPlatform(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "platform", guidValue);
}

export function useKitClientInfoInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitClientInfoInput", guidValue);
}

export function useKitClientInfoInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "id", guidValue);
}

export function useKitClientInfoInputName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "name", guidValue);
}

export function useKitClientInfoInputVersion(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "version", guidValue);
}

export function useKitClientInfoInputPlatform(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "platform", guidValue);
}

export function useSessionState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionState", guidValue);
}

export function useSessionWarningActionKindEnum(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionWarningActionKind", guidValue);
}

export function useSessionWarningAction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionWarningAction", guidValue);
}

export function useSessionWarningActionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "hash", guidValue);
}

export function useSessionWarningActionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "kind", guidValue);
}

export function useSessionWarningActionLabel(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "label", guidValue);
}

export function useKitSessionWarningEntity(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSessionWarning", guidValue);
}

export function useKitSessionWarningHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "hash", guidValue);
}

export function useKitSessionWarningCode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "code", guidValue);
}

export function useKitSessionWarningMessage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "message", guidValue);
}

export function useKitSessionWarningActions(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "actions", guidValue);
}

export function useSessionConnectorSelection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionConnectorSelection", guidValue);
}

export function useSessionConnectorSelectionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "hash", guidValue);
}

export function useSessionConnectorSelectionPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "piece", guidValue);
}

export function useSessionConnectorSelectionDesignPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "designPiece", guidValue);
}

export function useSessionConnectorSelectionConnector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "connector", guidValue);
}

export function useSessionConnectorSelectionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionConnectorSelectionInput", guidValue);
}

export function useSessionConnectorSelectionInputPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "pieceId", guidValue);
}

export function useSessionConnectorSelectionInputDesignPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "designPieceId", guidValue);
}

export function useSessionConnectorSelectionInputConnectorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "connectorId", guidValue);
}

export function useKitSessionSelectionEntity(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSessionSelection", guidValue);
}

export function useKitSessionSelectionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "hash", guidValue);
}

export function useKitSessionSelectionActiveDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "activeDesign", guidValue);
}

export function useKitSessionSelectionPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "pieces", guidValue);
}

export function useKitSessionSelectionConnections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "connections", guidValue);
}

export function useKitSessionSelectionConnectors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "connectors", guidValue);
}

export function useKitSessionSelectionRepresentations(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "representations", guidValue);
}

export function useKitSessionSelectionDesigns(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "designs", guidValue);
}

export function useKitSessionSelectionTypes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "types", guidValue);
}

export function useKitSessionSelectionReplacementTypeCandidates(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "replacementTypeCandidates", guidValue);
}

export function useKitSessionSelectionReplacementDesignCandidates(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "replacementDesignCandidates", guidValue);
}

export function useKitSessionSelectionBoundaryConnectorCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "boundaryConnectorCount", guidValue);
}

export function useSessionSelectionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionSelectionInput", guidValue);
}

export function useSessionSelectionInputActiveDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "activeDesignId", guidValue);
}

export function useSessionSelectionInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "pieceIds", guidValue);
}

export function useSessionSelectionInputConnectionIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "connectionIds", guidValue);
}

export function useSessionSelectionInputConnectorSelections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "connectorSelections", guidValue);
}

export function useSessionSelectionInputRepresentationIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "representationIds", guidValue);
}

export function useSessionSelectionInputDesignIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "designIds", guidValue);
}

export function useSessionSelectionInputTypeIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "typeIds", guidValue);
}

export function useKitSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSession", guidValue);
}

export function useKitSessionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "hash", guidValue);
}

export function useKitSessionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "id", guidValue);
}

export function useKitSessionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "kit", guidValue);
}

export function useKitSessionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "actor", guidValue);
}

export function useKitSessionClient(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "client", guidValue);
}

export function useKitSessionState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "state", guidValue);
}

export function useKitSessionStrictMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "strictMode", guidValue);
}

export function useKitSessionTimeoutSeconds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "timeoutSeconds", guidValue);
}

export function useKitSessionStartedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "startedAt", guidValue);
}

export function useKitSessionLastSeenAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "lastSeenAt", guidValue);
}

export function useKitSessionExpiresAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "expiresAt", guidValue);
}

export function useKitSessionDisconnectedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "disconnectedAt", guidValue);
}

export function useKitSessionLocked(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "locked", guidValue);
}

export function useKitSessionCanReconnect(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "canReconnect", guidValue);
}

export function useKitSessionCanSaveLocalChanges(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "canSaveLocalChanges", guidValue);
}

export function useKitSessionWarning(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "warning", guidValue);
}

export function useKitSessionSelection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "selection", guidValue);
}

export function useKitSessionActiveTransactions(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "activeTransactions", guidValue);
}

export function useValidationSeverity(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ValidationSeverity", guidValue);
}

export function useValidationNote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ValidationNote", guidValue);
}

export function useValidationNoteHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "hash", guidValue);
}

export function useValidationNoteSeverity(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "severity", guidValue);
}

export function useValidationNoteCode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "code", guidValue);
}

export function useValidationNotePath(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "path", guidValue);
}

export function useValidationNoteEntityId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "entityId", guidValue);
}

export function useValidationNoteMessage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "message", guidValue);
}

export function useKitValidationResult(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitValidationResult", guidValue);
}

export function useKitValidationResultHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "hash", guidValue);
}

export function useKitValidationResultOk(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "ok", guidValue);
}

export function useKitValidationResultImmutable(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "immutable", guidValue);
}

export function useKitValidationResultStrict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "strict", guidValue);
}

export function useKitValidationResultErrors(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "errors", guidValue);
}

export function useKitValidationResultWarnings(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "warnings", guidValue);
}

export function useKitValidationResultInfos(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "infos", guidValue);
}

export function useKitConflictStatusEnum(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflictStatus", guidValue);
}

export function useKitConflictKindEnum(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflictKind", guidValue);
}

export function useConflictResolutionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConflictResolutionKind", guidValue);
}

export function useConflictResolutionOption(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConflictResolutionOption", guidValue);
}

export function useConflictResolutionOptionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "hash", guidValue);
}

export function useConflictResolutionOptionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "id", guidValue);
}

export function useConflictResolutionOptionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "kind", guidValue);
}

export function useConflictResolutionOptionLabel(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "label", guidValue);
}

export function useConflictResolutionOptionDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "description", guidValue);
}

export function useConflictResolutionOptionPatchPreview(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "patchPreview", guidValue);
}

export function useKitConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflict", guidValue);
}

export function useKitConflictHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "hash", guidValue);
}

export function useKitConflictId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "id", guidValue);
}

export function useKitConflictKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "kit", guidValue);
}

export function useKitConflictSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "session", guidValue);
}

export function useKitConflictCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "candidate", guidValue);
}

export function useKitConflictStatus(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "status", guidValue);
}

export function useKitConflictKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "kind", guidValue);
}

export function useKitConflictTitle(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "title", guidValue);
}

export function useKitConflictMessage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "message", guidValue);
}

export function useKitConflictBlocking(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "blocking", guidValue);
}

export function useKitConflictStrict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "strict", guidValue);
}

export function useKitConflictNotes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "notes", guidValue);
}

export function useKitConflictOptions(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "options", guidValue);
}

export function useKitConflictCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "createdAt", guidValue);
}

export function useKitConflictResolvedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "resolvedAt", guidValue);
}

export function useKitCommandKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandKind", guidValue);
}

export function useKitCommandDescriptor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandDescriptor", guidValue);
}

export function useKitCommandDescriptorHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "hash", guidValue);
}

export function useKitCommandDescriptorKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "kind", guidValue);
}

export function useKitCommandDescriptorMutatesKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "mutatesKit", guidValue);
}

export function useKitCommandDescriptorSessionScoped(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "sessionScoped", guidValue);
}

export function useKitCommandDescriptorRequiresConsensus(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "requiresConsensus", guidValue);
}

export function useKitCommandDescriptorDescription(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "description", guidValue);
}

export function useKitChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitChange", guidValue);
}

export function useKitChangeHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "hash", guidValue);
}

export function useKitChangeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "id", guidValue);
}

export function useKitChangeKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "kind", guidValue);
}

export function useKitChangeSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "summary", guidValue);
}

export function useKitChangeOrigin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "origin", guidValue);
}

export function useKitChangeActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "actor", guidValue);
}

export function useKitChangeSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "session", guidValue);
}

export function useKitChangeTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "transaction", guidValue);
}

export function useKitChangeForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "forward", guidValue);
}

export function useKitChangeBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "backward", guidValue);
}

export function useKitChangeValidation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "validation", guidValue);
}

export function useKitChangeCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "createdAt", guidValue);
}

export function useKitChangeAppliedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "appliedAt", guidValue);
}

export function useKitCandidateStatus(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCandidateStatus", guidValue);
}

export function useCandidateVoteState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CandidateVoteState", guidValue);
}

export function useKitCandidateVote(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCandidateVote", guidValue);
}

export function useKitCandidateVoteHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "hash", guidValue);
}

export function useKitCandidateVoteSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "session", guidValue);
}

export function useKitCandidateVoteState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "state", guidValue);
}

export function useKitCandidateVoteReason(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "reason", guidValue);
}

export function useKitCandidateVoteRespondedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "respondedAt", guidValue);
}

export function useKitCandidateVoteResolutionOptionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "resolutionOptionId", guidValue);
}

export function useKitChangeCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitChangeCandidate", guidValue);
}

export function useKitChangeCandidateHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "hash", guidValue);
}

export function useKitChangeCandidateId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "id", guidValue);
}

export function useKitChangeCandidateKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "kit", guidValue);
}

export function useKitChangeCandidateKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "kind", guidValue);
}

export function useKitChangeCandidateSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "summary", guidValue);
}

export function useKitChangeCandidateProposedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "proposedBy", guidValue);
}

export function useKitChangeCandidateActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "actor", guidValue);
}

export function useKitChangeCandidateTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "transaction", guidValue);
}

export function useKitChangeCandidateStatus(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "status", guidValue);
}

export function useKitChangeCandidateRequestedFrom(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "requestedFrom", guidValue);
}

export function useKitChangeCandidateVotes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "votes", guidValue);
}

export function useKitChangeCandidateValidation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "validation", guidValue);
}

export function useKitChangeCandidatePreview(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "preview", guidValue);
}

export function useKitChangeCandidateProposedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "proposedAt", guidValue);
}

export function useKitChangeCandidateExpiresAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "expiresAt", guidValue);
}

export function useKitChangeCandidateDecidedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "decidedAt", guidValue);
}

export function useTransactionState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionState", guidValue);
}

export function useKitTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitTransaction", guidValue);
}

export function useKitTransactionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "hash", guidValue);
}

export function useKitTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "id", guidValue);
}

export function useKitTransactionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "kit", guidValue);
}

export function useKitTransactionLabel(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "label", guidValue);
}

export function useKitTransactionState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "state", guidValue);
}

export function useKitTransactionStartedBy(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "startedBy", guidValue);
}

export function useKitTransactionParent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "parent", guidValue);
}

export function useKitTransactionStartedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "startedAt", guidValue);
}

export function useKitTransactionFinalizedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "finalizedAt", guidValue);
}

export function useKitTransactionAbortedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "abortedAt", guidValue);
}

export function useKitTransactionChanges(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "changes", guidValue);
}

export function useKitTransactionUndoStack(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "undoStack", guidValue);
}

export function useKitTransactionRedoStack(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "redoStack", guidValue);
}

export function useKitTransactionCanUndo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "canUndo", guidValue);
}

export function useKitTransactionCanRedo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "canRedo", guidValue);
}

export function useKitTransactionSquashedChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "squashedChange", guidValue);
}

export function useKitHistoryEntry(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistoryEntry", guidValue);
}

export function useKitHistoryEntryHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "hash", guidValue);
}

export function useKitHistoryEntryId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "id", guidValue);
}

export function useKitHistoryEntryIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "index", guidValue);
}

export function useKitHistoryEntryTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "transaction", guidValue);
}

export function useKitHistoryEntryCommandKinds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "commandKinds", guidValue);
}

export function useKitHistoryEntrySummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "summary", guidValue);
}

export function useKitHistoryEntrySquashedChangeCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "squashedChangeCount", guidValue);
}

export function useKitHistoryEntryChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "change", guidValue);
}

export function useKitHistoryEntryCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "createdAt", guidValue);
}

export function useKitHistoryEntryFinalizedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "finalizedAt", guidValue);
}

export function useKitHistoryEntryUndoneAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "undoneAt", guidValue);
}

export function useKitHistoryPage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistoryPage", guidValue);
}

export function useKitHistoryPageHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "hash", guidValue);
}

export function useKitHistoryPageNodes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "nodes", guidValue);
}

export function useKitHistoryPagePageInfo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "pageInfo", guidValue);
}

export function useKitHistoryPageTotalCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "totalCount", guidValue);
}

export function useKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInteraction", guidValue);
}

export function useKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "id", guidValue);
}

export function useKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "hash", guidValue);
}

export function useKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "index", guidValue);
}

export function useKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "kit", guidValue);
}

export function useKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "kind", guidValue);
}

export function useKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "actor", guidValue);
}

export function useKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "session", guidValue);
}

export function useKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "transaction", guidValue);
}

export function useKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "candidate", guidValue);
}

export function useKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "change", guidValue);
}

export function useKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "conflict", guidValue);
}

export function useKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "summary", guidValue);
}

export function useKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "metadata", guidValue);
}

export function useKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "createdAt", guidValue);
}

export function useChangeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangeKitInteraction", guidValue);
}

export function useChangeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "id", guidValue);
}

export function useChangeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "hash", guidValue);
}

export function useChangeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "index", guidValue);
}

export function useChangeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "kit", guidValue);
}

export function useChangeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "kind", guidValue);
}

export function useChangeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "actor", guidValue);
}

export function useChangeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "session", guidValue);
}

export function useChangeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "transaction", guidValue);
}

export function useChangeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "candidate", guidValue);
}

export function useChangeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "change", guidValue);
}

export function useChangeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "conflict", guidValue);
}

export function useChangeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "summary", guidValue);
}

export function useChangeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "metadata", guidValue);
}

export function useChangeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "createdAt", guidValue);
}

export function useChangeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "forward", guidValue);
}

export function useChangeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "backward", guidValue);
}

export function useSetSessionSelectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SetSessionSelectionKitInteraction", guidValue);
}

export function useSetSessionSelectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "id", guidValue);
}

export function useSetSessionSelectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "hash", guidValue);
}

export function useSetSessionSelectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "index", guidValue);
}

export function useSetSessionSelectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "kit", guidValue);
}

export function useSetSessionSelectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "kind", guidValue);
}

export function useSetSessionSelectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "actor", guidValue);
}

export function useSetSessionSelectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "session", guidValue);
}

export function useSetSessionSelectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "transaction", guidValue);
}

export function useSetSessionSelectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "candidate", guidValue);
}

export function useSetSessionSelectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "change", guidValue);
}

export function useSetSessionSelectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "conflict", guidValue);
}

export function useSetSessionSelectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "summary", guidValue);
}

export function useSetSessionSelectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "metadata", guidValue);
}

export function useSetSessionSelectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "createdAt", guidValue);
}

export function useSetSessionSelectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "forward", guidValue);
}

export function useSetSessionSelectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "backward", guidValue);
}

export function useSetSessionSelectionKitInteractionMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "mode", guidValue);
}

export function useSetSessionSelectionKitInteractionSelection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "selection", guidValue);
}

export function useSetSessionSelectionKitInteractionPreviousSelection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "previousSelection", guidValue);
}

export function useCreateAuthorKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateAuthorKitInteraction", guidValue);
}

export function useCreateAuthorKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "id", guidValue);
}

export function useCreateAuthorKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "hash", guidValue);
}

export function useCreateAuthorKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "index", guidValue);
}

export function useCreateAuthorKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "kit", guidValue);
}

export function useCreateAuthorKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "kind", guidValue);
}

export function useCreateAuthorKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "actor", guidValue);
}

export function useCreateAuthorKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "session", guidValue);
}

export function useCreateAuthorKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "transaction", guidValue);
}

export function useCreateAuthorKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "candidate", guidValue);
}

export function useCreateAuthorKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "change", guidValue);
}

export function useCreateAuthorKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "conflict", guidValue);
}

export function useCreateAuthorKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "summary", guidValue);
}

export function useCreateAuthorKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "metadata", guidValue);
}

export function useCreateAuthorKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "createdAt", guidValue);
}

export function useCreateAuthorKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "forward", guidValue);
}

export function useCreateAuthorKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "backward", guidValue);
}

export function useCreateAuthorKitInteractionAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "author", guidValue);
}

export function useUpdateAuthorKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateAuthorKitInteraction", guidValue);
}

export function useUpdateAuthorKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "id", guidValue);
}

export function useUpdateAuthorKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "hash", guidValue);
}

export function useUpdateAuthorKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "index", guidValue);
}

export function useUpdateAuthorKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "kit", guidValue);
}

export function useUpdateAuthorKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "kind", guidValue);
}

export function useUpdateAuthorKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "actor", guidValue);
}

export function useUpdateAuthorKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "session", guidValue);
}

export function useUpdateAuthorKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "transaction", guidValue);
}

export function useUpdateAuthorKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "candidate", guidValue);
}

export function useUpdateAuthorKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "change", guidValue);
}

export function useUpdateAuthorKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "conflict", guidValue);
}

export function useUpdateAuthorKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "summary", guidValue);
}

export function useUpdateAuthorKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "metadata", guidValue);
}

export function useUpdateAuthorKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "createdAt", guidValue);
}

export function useUpdateAuthorKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "forward", guidValue);
}

export function useUpdateAuthorKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "backward", guidValue);
}

export function useUpdateAuthorKitInteractionAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "author", guidValue);
}

export function useUpdateAuthorKitInteractionPreviousAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "previousAuthor", guidValue);
}

export function useDeleteAuthorKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteAuthorKitInteraction", guidValue);
}

export function useDeleteAuthorKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "id", guidValue);
}

export function useDeleteAuthorKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "hash", guidValue);
}

export function useDeleteAuthorKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "index", guidValue);
}

export function useDeleteAuthorKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "kit", guidValue);
}

export function useDeleteAuthorKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "kind", guidValue);
}

export function useDeleteAuthorKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "actor", guidValue);
}

export function useDeleteAuthorKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "session", guidValue);
}

export function useDeleteAuthorKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "transaction", guidValue);
}

export function useDeleteAuthorKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "candidate", guidValue);
}

export function useDeleteAuthorKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "change", guidValue);
}

export function useDeleteAuthorKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "conflict", guidValue);
}

export function useDeleteAuthorKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "summary", guidValue);
}

export function useDeleteAuthorKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "metadata", guidValue);
}

export function useDeleteAuthorKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "createdAt", guidValue);
}

export function useDeleteAuthorKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "forward", guidValue);
}

export function useDeleteAuthorKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "backward", guidValue);
}

export function useDeleteAuthorKitInteractionPreviousAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "previousAuthor", guidValue);
}

export function useCreateTypeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTypeKitInteraction", guidValue);
}

export function useCreateTypeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "id", guidValue);
}

export function useCreateTypeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "hash", guidValue);
}

export function useCreateTypeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "index", guidValue);
}

export function useCreateTypeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "kit", guidValue);
}

export function useCreateTypeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "kind", guidValue);
}

export function useCreateTypeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "actor", guidValue);
}

export function useCreateTypeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "session", guidValue);
}

export function useCreateTypeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "transaction", guidValue);
}

export function useCreateTypeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "candidate", guidValue);
}

export function useCreateTypeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "change", guidValue);
}

export function useCreateTypeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "conflict", guidValue);
}

export function useCreateTypeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "summary", guidValue);
}

export function useCreateTypeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "metadata", guidValue);
}

export function useCreateTypeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "createdAt", guidValue);
}

export function useCreateTypeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "forward", guidValue);
}

export function useCreateTypeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "backward", guidValue);
}

export function useCreateTypeKitInteractionType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "type", guidValue);
}

export function useUpdateTypeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTypeKitInteraction", guidValue);
}

export function useUpdateTypeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "id", guidValue);
}

export function useUpdateTypeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "hash", guidValue);
}

export function useUpdateTypeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "index", guidValue);
}

export function useUpdateTypeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "kit", guidValue);
}

export function useUpdateTypeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "kind", guidValue);
}

export function useUpdateTypeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "actor", guidValue);
}

export function useUpdateTypeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "session", guidValue);
}

export function useUpdateTypeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "transaction", guidValue);
}

export function useUpdateTypeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "candidate", guidValue);
}

export function useUpdateTypeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "change", guidValue);
}

export function useUpdateTypeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "conflict", guidValue);
}

export function useUpdateTypeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "summary", guidValue);
}

export function useUpdateTypeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "metadata", guidValue);
}

export function useUpdateTypeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "createdAt", guidValue);
}

export function useUpdateTypeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "forward", guidValue);
}

export function useUpdateTypeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "backward", guidValue);
}

export function useUpdateTypeKitInteractionType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "type", guidValue);
}

export function useUpdateTypeKitInteractionPreviousType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "previousType", guidValue);
}

export function useDeleteTypeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTypeKitInteraction", guidValue);
}

export function useDeleteTypeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "id", guidValue);
}

export function useDeleteTypeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "hash", guidValue);
}

export function useDeleteTypeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "index", guidValue);
}

export function useDeleteTypeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "kit", guidValue);
}

export function useDeleteTypeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "kind", guidValue);
}

export function useDeleteTypeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "actor", guidValue);
}

export function useDeleteTypeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "session", guidValue);
}

export function useDeleteTypeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "transaction", guidValue);
}

export function useDeleteTypeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "candidate", guidValue);
}

export function useDeleteTypeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "change", guidValue);
}

export function useDeleteTypeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "conflict", guidValue);
}

export function useDeleteTypeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "summary", guidValue);
}

export function useDeleteTypeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "metadata", guidValue);
}

export function useDeleteTypeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "createdAt", guidValue);
}

export function useDeleteTypeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "forward", guidValue);
}

export function useDeleteTypeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "backward", guidValue);
}

export function useDeleteTypeKitInteractionPreviousType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "previousType", guidValue);
}

export function useCreateDesignKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateDesignKitInteraction", guidValue);
}

export function useCreateDesignKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "id", guidValue);
}

export function useCreateDesignKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "hash", guidValue);
}

export function useCreateDesignKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "index", guidValue);
}

export function useCreateDesignKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "kit", guidValue);
}

export function useCreateDesignKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "kind", guidValue);
}

export function useCreateDesignKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "actor", guidValue);
}

export function useCreateDesignKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "session", guidValue);
}

export function useCreateDesignKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "transaction", guidValue);
}

export function useCreateDesignKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "candidate", guidValue);
}

export function useCreateDesignKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "change", guidValue);
}

export function useCreateDesignKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "conflict", guidValue);
}

export function useCreateDesignKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "summary", guidValue);
}

export function useCreateDesignKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "metadata", guidValue);
}

export function useCreateDesignKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "createdAt", guidValue);
}

export function useCreateDesignKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "forward", guidValue);
}

export function useCreateDesignKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "backward", guidValue);
}

export function useCreateDesignKitInteractionDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "design", guidValue);
}

export function useUpdateDesignKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateDesignKitInteraction", guidValue);
}

export function useUpdateDesignKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "id", guidValue);
}

export function useUpdateDesignKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "hash", guidValue);
}

export function useUpdateDesignKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "index", guidValue);
}

export function useUpdateDesignKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "kit", guidValue);
}

export function useUpdateDesignKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "kind", guidValue);
}

export function useUpdateDesignKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "actor", guidValue);
}

export function useUpdateDesignKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "session", guidValue);
}

export function useUpdateDesignKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "transaction", guidValue);
}

export function useUpdateDesignKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "candidate", guidValue);
}

export function useUpdateDesignKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "change", guidValue);
}

export function useUpdateDesignKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "conflict", guidValue);
}

export function useUpdateDesignKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "summary", guidValue);
}

export function useUpdateDesignKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "metadata", guidValue);
}

export function useUpdateDesignKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "createdAt", guidValue);
}

export function useUpdateDesignKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "forward", guidValue);
}

export function useUpdateDesignKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "backward", guidValue);
}

export function useUpdateDesignKitInteractionDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "design", guidValue);
}

export function useUpdateDesignKitInteractionPreviousDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "previousDesign", guidValue);
}

export function useDeleteDesignKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteDesignKitInteraction", guidValue);
}

export function useDeleteDesignKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "id", guidValue);
}

export function useDeleteDesignKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "hash", guidValue);
}

export function useDeleteDesignKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "index", guidValue);
}

export function useDeleteDesignKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "kit", guidValue);
}

export function useDeleteDesignKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "kind", guidValue);
}

export function useDeleteDesignKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "actor", guidValue);
}

export function useDeleteDesignKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "session", guidValue);
}

export function useDeleteDesignKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "transaction", guidValue);
}

export function useDeleteDesignKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "candidate", guidValue);
}

export function useDeleteDesignKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "change", guidValue);
}

export function useDeleteDesignKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "conflict", guidValue);
}

export function useDeleteDesignKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "summary", guidValue);
}

export function useDeleteDesignKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "metadata", guidValue);
}

export function useDeleteDesignKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "createdAt", guidValue);
}

export function useDeleteDesignKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "forward", guidValue);
}

export function useDeleteDesignKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "backward", guidValue);
}

export function useDeleteDesignKitInteractionPreviousDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "previousDesign", guidValue);
}

export function useCreateQualityKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateQualityKitInteraction", guidValue);
}

export function useCreateQualityKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "id", guidValue);
}

export function useCreateQualityKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "hash", guidValue);
}

export function useCreateQualityKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "index", guidValue);
}

export function useCreateQualityKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "kit", guidValue);
}

export function useCreateQualityKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "kind", guidValue);
}

export function useCreateQualityKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "actor", guidValue);
}

export function useCreateQualityKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "session", guidValue);
}

export function useCreateQualityKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "transaction", guidValue);
}

export function useCreateQualityKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "candidate", guidValue);
}

export function useCreateQualityKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "change", guidValue);
}

export function useCreateQualityKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "conflict", guidValue);
}

export function useCreateQualityKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "summary", guidValue);
}

export function useCreateQualityKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "metadata", guidValue);
}

export function useCreateQualityKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "createdAt", guidValue);
}

export function useCreateQualityKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "forward", guidValue);
}

export function useCreateQualityKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "backward", guidValue);
}

export function useCreateQualityKitInteractionQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "quality", guidValue);
}

export function useUpdateQualityKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateQualityKitInteraction", guidValue);
}

export function useUpdateQualityKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "id", guidValue);
}

export function useUpdateQualityKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "hash", guidValue);
}

export function useUpdateQualityKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "index", guidValue);
}

export function useUpdateQualityKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "kit", guidValue);
}

export function useUpdateQualityKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "kind", guidValue);
}

export function useUpdateQualityKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "actor", guidValue);
}

export function useUpdateQualityKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "session", guidValue);
}

export function useUpdateQualityKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "transaction", guidValue);
}

export function useUpdateQualityKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "candidate", guidValue);
}

export function useUpdateQualityKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "change", guidValue);
}

export function useUpdateQualityKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "conflict", guidValue);
}

export function useUpdateQualityKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "summary", guidValue);
}

export function useUpdateQualityKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "metadata", guidValue);
}

export function useUpdateQualityKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "createdAt", guidValue);
}

export function useUpdateQualityKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "forward", guidValue);
}

export function useUpdateQualityKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "backward", guidValue);
}

export function useUpdateQualityKitInteractionQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "quality", guidValue);
}

export function useUpdateQualityKitInteractionPreviousQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "previousQuality", guidValue);
}

export function useDeleteQualityKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteQualityKitInteraction", guidValue);
}

export function useDeleteQualityKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "id", guidValue);
}

export function useDeleteQualityKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "hash", guidValue);
}

export function useDeleteQualityKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "index", guidValue);
}

export function useDeleteQualityKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "kit", guidValue);
}

export function useDeleteQualityKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "kind", guidValue);
}

export function useDeleteQualityKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "actor", guidValue);
}

export function useDeleteQualityKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "session", guidValue);
}

export function useDeleteQualityKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "transaction", guidValue);
}

export function useDeleteQualityKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "candidate", guidValue);
}

export function useDeleteQualityKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "change", guidValue);
}

export function useDeleteQualityKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "conflict", guidValue);
}

export function useDeleteQualityKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "summary", guidValue);
}

export function useDeleteQualityKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "metadata", guidValue);
}

export function useDeleteQualityKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "createdAt", guidValue);
}

export function useDeleteQualityKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "forward", guidValue);
}

export function useDeleteQualityKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "backward", guidValue);
}

export function useDeleteQualityKitInteractionPreviousQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "previousQuality", guidValue);
}

export function useCreatePortKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePortKitInteraction", guidValue);
}

export function useCreatePortKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "id", guidValue);
}

export function useCreatePortKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "hash", guidValue);
}

export function useCreatePortKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "index", guidValue);
}

export function useCreatePortKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "kit", guidValue);
}

export function useCreatePortKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "kind", guidValue);
}

export function useCreatePortKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "actor", guidValue);
}

export function useCreatePortKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "session", guidValue);
}

export function useCreatePortKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "transaction", guidValue);
}

export function useCreatePortKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "candidate", guidValue);
}

export function useCreatePortKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "change", guidValue);
}

export function useCreatePortKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "conflict", guidValue);
}

export function useCreatePortKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "summary", guidValue);
}

export function useCreatePortKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "metadata", guidValue);
}

export function useCreatePortKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "createdAt", guidValue);
}

export function useCreatePortKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "forward", guidValue);
}

export function useCreatePortKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "backward", guidValue);
}

export function useCreatePortKitInteractionPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "port", guidValue);
}

export function useUpdatePortKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePortKitInteraction", guidValue);
}

export function useUpdatePortKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "id", guidValue);
}

export function useUpdatePortKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "hash", guidValue);
}

export function useUpdatePortKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "index", guidValue);
}

export function useUpdatePortKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "kit", guidValue);
}

export function useUpdatePortKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "kind", guidValue);
}

export function useUpdatePortKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "actor", guidValue);
}

export function useUpdatePortKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "session", guidValue);
}

export function useUpdatePortKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "transaction", guidValue);
}

export function useUpdatePortKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "candidate", guidValue);
}

export function useUpdatePortKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "change", guidValue);
}

export function useUpdatePortKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "conflict", guidValue);
}

export function useUpdatePortKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "summary", guidValue);
}

export function useUpdatePortKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "metadata", guidValue);
}

export function useUpdatePortKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "createdAt", guidValue);
}

export function useUpdatePortKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "forward", guidValue);
}

export function useUpdatePortKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "backward", guidValue);
}

export function useUpdatePortKitInteractionPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "port", guidValue);
}

export function useUpdatePortKitInteractionPreviousPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "previousPort", guidValue);
}

export function useDeletePortKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePortKitInteraction", guidValue);
}

export function useDeletePortKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "id", guidValue);
}

export function useDeletePortKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "hash", guidValue);
}

export function useDeletePortKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "index", guidValue);
}

export function useDeletePortKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "kit", guidValue);
}

export function useDeletePortKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "kind", guidValue);
}

export function useDeletePortKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "actor", guidValue);
}

export function useDeletePortKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "session", guidValue);
}

export function useDeletePortKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "transaction", guidValue);
}

export function useDeletePortKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "candidate", guidValue);
}

export function useDeletePortKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "change", guidValue);
}

export function useDeletePortKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "conflict", guidValue);
}

export function useDeletePortKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "summary", guidValue);
}

export function useDeletePortKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "metadata", guidValue);
}

export function useDeletePortKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "createdAt", guidValue);
}

export function useDeletePortKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "forward", guidValue);
}

export function useDeletePortKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "backward", guidValue);
}

export function useDeletePortKitInteractionPreviousPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "previousPort", guidValue);
}

export function useCreateFamilyKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFamilyKitInteraction", guidValue);
}

export function useCreateFamilyKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "id", guidValue);
}

export function useCreateFamilyKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "hash", guidValue);
}

export function useCreateFamilyKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "index", guidValue);
}

export function useCreateFamilyKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "kit", guidValue);
}

export function useCreateFamilyKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "kind", guidValue);
}

export function useCreateFamilyKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "actor", guidValue);
}

export function useCreateFamilyKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "session", guidValue);
}

export function useCreateFamilyKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "transaction", guidValue);
}

export function useCreateFamilyKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "candidate", guidValue);
}

export function useCreateFamilyKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "change", guidValue);
}

export function useCreateFamilyKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "conflict", guidValue);
}

export function useCreateFamilyKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "summary", guidValue);
}

export function useCreateFamilyKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "metadata", guidValue);
}

export function useCreateFamilyKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "createdAt", guidValue);
}

export function useCreateFamilyKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "forward", guidValue);
}

export function useCreateFamilyKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "backward", guidValue);
}

export function useCreateFamilyKitInteractionFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "family", guidValue);
}

export function useUpdateFamilyKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFamilyKitInteraction", guidValue);
}

export function useUpdateFamilyKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "id", guidValue);
}

export function useUpdateFamilyKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "hash", guidValue);
}

export function useUpdateFamilyKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "index", guidValue);
}

export function useUpdateFamilyKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "kit", guidValue);
}

export function useUpdateFamilyKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "kind", guidValue);
}

export function useUpdateFamilyKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "actor", guidValue);
}

export function useUpdateFamilyKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "session", guidValue);
}

export function useUpdateFamilyKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "transaction", guidValue);
}

export function useUpdateFamilyKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "candidate", guidValue);
}

export function useUpdateFamilyKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "change", guidValue);
}

export function useUpdateFamilyKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "conflict", guidValue);
}

export function useUpdateFamilyKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "summary", guidValue);
}

export function useUpdateFamilyKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "metadata", guidValue);
}

export function useUpdateFamilyKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "createdAt", guidValue);
}

export function useUpdateFamilyKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "forward", guidValue);
}

export function useUpdateFamilyKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "backward", guidValue);
}

export function useUpdateFamilyKitInteractionFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "family", guidValue);
}

export function useUpdateFamilyKitInteractionPreviousFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "previousFamily", guidValue);
}

export function useDeleteFamilyKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFamilyKitInteraction", guidValue);
}

export function useDeleteFamilyKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "id", guidValue);
}

export function useDeleteFamilyKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "hash", guidValue);
}

export function useDeleteFamilyKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "index", guidValue);
}

export function useDeleteFamilyKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "kit", guidValue);
}

export function useDeleteFamilyKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "kind", guidValue);
}

export function useDeleteFamilyKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "actor", guidValue);
}

export function useDeleteFamilyKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "session", guidValue);
}

export function useDeleteFamilyKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "transaction", guidValue);
}

export function useDeleteFamilyKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "candidate", guidValue);
}

export function useDeleteFamilyKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "change", guidValue);
}

export function useDeleteFamilyKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "conflict", guidValue);
}

export function useDeleteFamilyKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "summary", guidValue);
}

export function useDeleteFamilyKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "metadata", guidValue);
}

export function useDeleteFamilyKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "createdAt", guidValue);
}

export function useDeleteFamilyKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "forward", guidValue);
}

export function useDeleteFamilyKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "backward", guidValue);
}

export function useDeleteFamilyKitInteractionPreviousFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "previousFamily", guidValue);
}

export function useCreateTagKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTagKitInteraction", guidValue);
}

export function useCreateTagKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "id", guidValue);
}

export function useCreateTagKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "hash", guidValue);
}

export function useCreateTagKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "index", guidValue);
}

export function useCreateTagKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "kit", guidValue);
}

export function useCreateTagKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "kind", guidValue);
}

export function useCreateTagKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "actor", guidValue);
}

export function useCreateTagKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "session", guidValue);
}

export function useCreateTagKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "transaction", guidValue);
}

export function useCreateTagKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "candidate", guidValue);
}

export function useCreateTagKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "change", guidValue);
}

export function useCreateTagKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "conflict", guidValue);
}

export function useCreateTagKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "summary", guidValue);
}

export function useCreateTagKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "metadata", guidValue);
}

export function useCreateTagKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "createdAt", guidValue);
}

export function useCreateTagKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "forward", guidValue);
}

export function useCreateTagKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "backward", guidValue);
}

export function useCreateTagKitInteractionTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "tag", guidValue);
}

export function useUpdateTagKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTagKitInteraction", guidValue);
}

export function useUpdateTagKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "id", guidValue);
}

export function useUpdateTagKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "hash", guidValue);
}

export function useUpdateTagKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "index", guidValue);
}

export function useUpdateTagKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "kit", guidValue);
}

export function useUpdateTagKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "kind", guidValue);
}

export function useUpdateTagKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "actor", guidValue);
}

export function useUpdateTagKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "session", guidValue);
}

export function useUpdateTagKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "transaction", guidValue);
}

export function useUpdateTagKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "candidate", guidValue);
}

export function useUpdateTagKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "change", guidValue);
}

export function useUpdateTagKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "conflict", guidValue);
}

export function useUpdateTagKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "summary", guidValue);
}

export function useUpdateTagKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "metadata", guidValue);
}

export function useUpdateTagKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "createdAt", guidValue);
}

export function useUpdateTagKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "forward", guidValue);
}

export function useUpdateTagKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "backward", guidValue);
}

export function useUpdateTagKitInteractionTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "tag", guidValue);
}

export function useUpdateTagKitInteractionPreviousTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "previousTag", guidValue);
}

export function useDeleteTagKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTagKitInteraction", guidValue);
}

export function useDeleteTagKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "id", guidValue);
}

export function useDeleteTagKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "hash", guidValue);
}

export function useDeleteTagKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "index", guidValue);
}

export function useDeleteTagKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "kit", guidValue);
}

export function useDeleteTagKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "kind", guidValue);
}

export function useDeleteTagKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "actor", guidValue);
}

export function useDeleteTagKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "session", guidValue);
}

export function useDeleteTagKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "transaction", guidValue);
}

export function useDeleteTagKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "candidate", guidValue);
}

export function useDeleteTagKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "change", guidValue);
}

export function useDeleteTagKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "conflict", guidValue);
}

export function useDeleteTagKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "summary", guidValue);
}

export function useDeleteTagKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "metadata", guidValue);
}

export function useDeleteTagKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "createdAt", guidValue);
}

export function useDeleteTagKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "forward", guidValue);
}

export function useDeleteTagKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "backward", guidValue);
}

export function useDeleteTagKitInteractionPreviousTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "previousTag", guidValue);
}

export function useCreateConceptKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConceptKitInteraction", guidValue);
}

export function useCreateConceptKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "id", guidValue);
}

export function useCreateConceptKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "hash", guidValue);
}

export function useCreateConceptKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "index", guidValue);
}

export function useCreateConceptKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "kit", guidValue);
}

export function useCreateConceptKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "kind", guidValue);
}

export function useCreateConceptKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "actor", guidValue);
}

export function useCreateConceptKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "session", guidValue);
}

export function useCreateConceptKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "transaction", guidValue);
}

export function useCreateConceptKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "candidate", guidValue);
}

export function useCreateConceptKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "change", guidValue);
}

export function useCreateConceptKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "conflict", guidValue);
}

export function useCreateConceptKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "summary", guidValue);
}

export function useCreateConceptKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "metadata", guidValue);
}

export function useCreateConceptKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "createdAt", guidValue);
}

export function useCreateConceptKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "forward", guidValue);
}

export function useCreateConceptKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "backward", guidValue);
}

export function useCreateConceptKitInteractionConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "concept", guidValue);
}

export function useUpdateConceptKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConceptKitInteraction", guidValue);
}

export function useUpdateConceptKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "id", guidValue);
}

export function useUpdateConceptKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "hash", guidValue);
}

export function useUpdateConceptKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "index", guidValue);
}

export function useUpdateConceptKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "kit", guidValue);
}

export function useUpdateConceptKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "kind", guidValue);
}

export function useUpdateConceptKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "actor", guidValue);
}

export function useUpdateConceptKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "session", guidValue);
}

export function useUpdateConceptKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "transaction", guidValue);
}

export function useUpdateConceptKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "candidate", guidValue);
}

export function useUpdateConceptKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "change", guidValue);
}

export function useUpdateConceptKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "conflict", guidValue);
}

export function useUpdateConceptKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "summary", guidValue);
}

export function useUpdateConceptKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "metadata", guidValue);
}

export function useUpdateConceptKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "createdAt", guidValue);
}

export function useUpdateConceptKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "forward", guidValue);
}

export function useUpdateConceptKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "backward", guidValue);
}

export function useUpdateConceptKitInteractionConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "concept", guidValue);
}

export function useUpdateConceptKitInteractionPreviousConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "previousConcept", guidValue);
}

export function useDeleteConceptKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConceptKitInteraction", guidValue);
}

export function useDeleteConceptKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "id", guidValue);
}

export function useDeleteConceptKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "hash", guidValue);
}

export function useDeleteConceptKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "index", guidValue);
}

export function useDeleteConceptKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "kit", guidValue);
}

export function useDeleteConceptKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "kind", guidValue);
}

export function useDeleteConceptKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "actor", guidValue);
}

export function useDeleteConceptKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "session", guidValue);
}

export function useDeleteConceptKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "transaction", guidValue);
}

export function useDeleteConceptKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "candidate", guidValue);
}

export function useDeleteConceptKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "change", guidValue);
}

export function useDeleteConceptKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "conflict", guidValue);
}

export function useDeleteConceptKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "summary", guidValue);
}

export function useDeleteConceptKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "metadata", guidValue);
}

export function useDeleteConceptKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "createdAt", guidValue);
}

export function useDeleteConceptKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "forward", guidValue);
}

export function useDeleteConceptKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "backward", guidValue);
}

export function useDeleteConceptKitInteractionPreviousConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "previousConcept", guidValue);
}

export function useCreateFileKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFileKitInteraction", guidValue);
}

export function useCreateFileKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "id", guidValue);
}

export function useCreateFileKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "hash", guidValue);
}

export function useCreateFileKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "index", guidValue);
}

export function useCreateFileKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "kit", guidValue);
}

export function useCreateFileKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "kind", guidValue);
}

export function useCreateFileKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "actor", guidValue);
}

export function useCreateFileKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "session", guidValue);
}

export function useCreateFileKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "transaction", guidValue);
}

export function useCreateFileKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "candidate", guidValue);
}

export function useCreateFileKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "change", guidValue);
}

export function useCreateFileKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "conflict", guidValue);
}

export function useCreateFileKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "summary", guidValue);
}

export function useCreateFileKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "metadata", guidValue);
}

export function useCreateFileKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "createdAt", guidValue);
}

export function useCreateFileKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "forward", guidValue);
}

export function useCreateFileKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "backward", guidValue);
}

export function useCreateFileKitInteractionFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "file", guidValue);
}

export function useUpdateFileKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFileKitInteraction", guidValue);
}

export function useUpdateFileKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "id", guidValue);
}

export function useUpdateFileKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "hash", guidValue);
}

export function useUpdateFileKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "index", guidValue);
}

export function useUpdateFileKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "kit", guidValue);
}

export function useUpdateFileKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "kind", guidValue);
}

export function useUpdateFileKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "actor", guidValue);
}

export function useUpdateFileKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "session", guidValue);
}

export function useUpdateFileKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "transaction", guidValue);
}

export function useUpdateFileKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "candidate", guidValue);
}

export function useUpdateFileKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "change", guidValue);
}

export function useUpdateFileKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "conflict", guidValue);
}

export function useUpdateFileKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "summary", guidValue);
}

export function useUpdateFileKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "metadata", guidValue);
}

export function useUpdateFileKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "createdAt", guidValue);
}

export function useUpdateFileKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "forward", guidValue);
}

export function useUpdateFileKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "backward", guidValue);
}

export function useUpdateFileKitInteractionFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "file", guidValue);
}

export function useUpdateFileKitInteractionPreviousFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "previousFile", guidValue);
}

export function useDeleteFileKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFileKitInteraction", guidValue);
}

export function useDeleteFileKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "id", guidValue);
}

export function useDeleteFileKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "hash", guidValue);
}

export function useDeleteFileKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "index", guidValue);
}

export function useDeleteFileKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "kit", guidValue);
}

export function useDeleteFileKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "kind", guidValue);
}

export function useDeleteFileKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "actor", guidValue);
}

export function useDeleteFileKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "session", guidValue);
}

export function useDeleteFileKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "transaction", guidValue);
}

export function useDeleteFileKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "candidate", guidValue);
}

export function useDeleteFileKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "change", guidValue);
}

export function useDeleteFileKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "conflict", guidValue);
}

export function useDeleteFileKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "summary", guidValue);
}

export function useDeleteFileKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "metadata", guidValue);
}

export function useDeleteFileKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "createdAt", guidValue);
}

export function useDeleteFileKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "forward", guidValue);
}

export function useDeleteFileKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "backward", guidValue);
}

export function useDeleteFileKitInteractionPreviousFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "previousFile", guidValue);
}

export function useCreateFolderKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFolderKitInteraction", guidValue);
}

export function useCreateFolderKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "id", guidValue);
}

export function useCreateFolderKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "hash", guidValue);
}

export function useCreateFolderKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "index", guidValue);
}

export function useCreateFolderKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "kit", guidValue);
}

export function useCreateFolderKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "kind", guidValue);
}

export function useCreateFolderKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "actor", guidValue);
}

export function useCreateFolderKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "session", guidValue);
}

export function useCreateFolderKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "transaction", guidValue);
}

export function useCreateFolderKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "candidate", guidValue);
}

export function useCreateFolderKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "change", guidValue);
}

export function useCreateFolderKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "conflict", guidValue);
}

export function useCreateFolderKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "summary", guidValue);
}

export function useCreateFolderKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "metadata", guidValue);
}

export function useCreateFolderKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "createdAt", guidValue);
}

export function useCreateFolderKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "forward", guidValue);
}

export function useCreateFolderKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "backward", guidValue);
}

export function useCreateFolderKitInteractionFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "folder", guidValue);
}

export function useUpdateFolderKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFolderKitInteraction", guidValue);
}

export function useUpdateFolderKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "id", guidValue);
}

export function useUpdateFolderKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "hash", guidValue);
}

export function useUpdateFolderKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "index", guidValue);
}

export function useUpdateFolderKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "kit", guidValue);
}

export function useUpdateFolderKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "kind", guidValue);
}

export function useUpdateFolderKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "actor", guidValue);
}

export function useUpdateFolderKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "session", guidValue);
}

export function useUpdateFolderKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "transaction", guidValue);
}

export function useUpdateFolderKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "candidate", guidValue);
}

export function useUpdateFolderKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "change", guidValue);
}

export function useUpdateFolderKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "conflict", guidValue);
}

export function useUpdateFolderKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "summary", guidValue);
}

export function useUpdateFolderKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "metadata", guidValue);
}

export function useUpdateFolderKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "createdAt", guidValue);
}

export function useUpdateFolderKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "forward", guidValue);
}

export function useUpdateFolderKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "backward", guidValue);
}

export function useUpdateFolderKitInteractionFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "folder", guidValue);
}

export function useUpdateFolderKitInteractionPreviousFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "previousFolder", guidValue);
}

export function useDeleteFolderKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFolderKitInteraction", guidValue);
}

export function useDeleteFolderKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "id", guidValue);
}

export function useDeleteFolderKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "hash", guidValue);
}

export function useDeleteFolderKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "index", guidValue);
}

export function useDeleteFolderKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "kit", guidValue);
}

export function useDeleteFolderKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "kind", guidValue);
}

export function useDeleteFolderKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "actor", guidValue);
}

export function useDeleteFolderKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "session", guidValue);
}

export function useDeleteFolderKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "transaction", guidValue);
}

export function useDeleteFolderKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "candidate", guidValue);
}

export function useDeleteFolderKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "change", guidValue);
}

export function useDeleteFolderKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "conflict", guidValue);
}

export function useDeleteFolderKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "summary", guidValue);
}

export function useDeleteFolderKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "metadata", guidValue);
}

export function useDeleteFolderKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "createdAt", guidValue);
}

export function useDeleteFolderKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "forward", guidValue);
}

export function useDeleteFolderKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "backward", guidValue);
}

export function useDeleteFolderKitInteractionPreviousFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "previousFolder", guidValue);
}

export function useMoveArtifactToFolderKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MoveArtifactToFolderKitInteraction", guidValue);
}

export function useMoveArtifactToFolderKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "id", guidValue);
}

export function useMoveArtifactToFolderKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "hash", guidValue);
}

export function useMoveArtifactToFolderKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "index", guidValue);
}

export function useMoveArtifactToFolderKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kit", guidValue);
}

export function useMoveArtifactToFolderKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kind", guidValue);
}

export function useMoveArtifactToFolderKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "actor", guidValue);
}

export function useMoveArtifactToFolderKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "session", guidValue);
}

export function useMoveArtifactToFolderKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "transaction", guidValue);
}

export function useMoveArtifactToFolderKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "candidate", guidValue);
}

export function useMoveArtifactToFolderKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "change", guidValue);
}

export function useMoveArtifactToFolderKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "conflict", guidValue);
}

export function useMoveArtifactToFolderKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "summary", guidValue);
}

export function useMoveArtifactToFolderKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "metadata", guidValue);
}

export function useMoveArtifactToFolderKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "createdAt", guidValue);
}

export function useMoveArtifactToFolderKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "forward", guidValue);
}

export function useMoveArtifactToFolderKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "backward", guidValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactKind", guidValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactId", guidValue);
}

export function useMoveArtifactToFolderKitInteractionFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "folder", guidValue);
}

export function useMoveArtifactToFolderKitInteractionPreviousFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "previousFolder", guidValue);
}

export function useCreatePieceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePieceKitInteraction", guidValue);
}

export function useCreatePieceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "id", guidValue);
}

export function useCreatePieceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "hash", guidValue);
}

export function useCreatePieceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "index", guidValue);
}

export function useCreatePieceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "kit", guidValue);
}

export function useCreatePieceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "kind", guidValue);
}

export function useCreatePieceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "actor", guidValue);
}

export function useCreatePieceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "session", guidValue);
}

export function useCreatePieceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "transaction", guidValue);
}

export function useCreatePieceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "candidate", guidValue);
}

export function useCreatePieceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "change", guidValue);
}

export function useCreatePieceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "conflict", guidValue);
}

export function useCreatePieceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "summary", guidValue);
}

export function useCreatePieceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "metadata", guidValue);
}

export function useCreatePieceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "createdAt", guidValue);
}

export function useCreatePieceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "forward", guidValue);
}

export function useCreatePieceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "backward", guidValue);
}

export function useCreatePiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePiecesKitInteraction", guidValue);
}

export function useCreatePiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "id", guidValue);
}

export function useCreatePiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "hash", guidValue);
}

export function useCreatePiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "index", guidValue);
}

export function useCreatePiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "kit", guidValue);
}

export function useCreatePiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "kind", guidValue);
}

export function useCreatePiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "actor", guidValue);
}

export function useCreatePiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "session", guidValue);
}

export function useCreatePiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "transaction", guidValue);
}

export function useCreatePiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "candidate", guidValue);
}

export function useCreatePiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "change", guidValue);
}

export function useCreatePiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "conflict", guidValue);
}

export function useCreatePiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "summary", guidValue);
}

export function useCreatePiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "metadata", guidValue);
}

export function useCreatePiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "createdAt", guidValue);
}

export function useCreatePiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "forward", guidValue);
}

export function useCreatePiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "backward", guidValue);
}

export function useUpdatePieceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePieceKitInteraction", guidValue);
}

export function useUpdatePieceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "id", guidValue);
}

export function useUpdatePieceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "hash", guidValue);
}

export function useUpdatePieceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "index", guidValue);
}

export function useUpdatePieceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "kit", guidValue);
}

export function useUpdatePieceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "kind", guidValue);
}

export function useUpdatePieceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "actor", guidValue);
}

export function useUpdatePieceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "session", guidValue);
}

export function useUpdatePieceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "transaction", guidValue);
}

export function useUpdatePieceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "candidate", guidValue);
}

export function useUpdatePieceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "change", guidValue);
}

export function useUpdatePieceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "conflict", guidValue);
}

export function useUpdatePieceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "summary", guidValue);
}

export function useUpdatePieceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "metadata", guidValue);
}

export function useUpdatePieceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "createdAt", guidValue);
}

export function useUpdatePieceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "forward", guidValue);
}

export function useUpdatePieceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "backward", guidValue);
}

export function useUpdatePiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePiecesKitInteraction", guidValue);
}

export function useUpdatePiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "id", guidValue);
}

export function useUpdatePiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "hash", guidValue);
}

export function useUpdatePiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "index", guidValue);
}

export function useUpdatePiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "kit", guidValue);
}

export function useUpdatePiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "kind", guidValue);
}

export function useUpdatePiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "actor", guidValue);
}

export function useUpdatePiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "session", guidValue);
}

export function useUpdatePiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "transaction", guidValue);
}

export function useUpdatePiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "candidate", guidValue);
}

export function useUpdatePiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "change", guidValue);
}

export function useUpdatePiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "conflict", guidValue);
}

export function useUpdatePiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "summary", guidValue);
}

export function useUpdatePiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "metadata", guidValue);
}

export function useUpdatePiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "createdAt", guidValue);
}

export function useUpdatePiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "forward", guidValue);
}

export function useUpdatePiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "backward", guidValue);
}

export function useDeletePieceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePieceKitInteraction", guidValue);
}

export function useDeletePieceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "id", guidValue);
}

export function useDeletePieceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "hash", guidValue);
}

export function useDeletePieceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "index", guidValue);
}

export function useDeletePieceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "kit", guidValue);
}

export function useDeletePieceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "kind", guidValue);
}

export function useDeletePieceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "actor", guidValue);
}

export function useDeletePieceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "session", guidValue);
}

export function useDeletePieceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "transaction", guidValue);
}

export function useDeletePieceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "candidate", guidValue);
}

export function useDeletePieceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "change", guidValue);
}

export function useDeletePieceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "conflict", guidValue);
}

export function useDeletePieceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "summary", guidValue);
}

export function useDeletePieceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "metadata", guidValue);
}

export function useDeletePieceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "createdAt", guidValue);
}

export function useDeletePieceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "forward", guidValue);
}

export function useDeletePieceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "backward", guidValue);
}

export function useDeletePiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePiecesKitInteraction", guidValue);
}

export function useDeletePiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "id", guidValue);
}

export function useDeletePiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "hash", guidValue);
}

export function useDeletePiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "index", guidValue);
}

export function useDeletePiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "kit", guidValue);
}

export function useDeletePiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "kind", guidValue);
}

export function useDeletePiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "actor", guidValue);
}

export function useDeletePiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "session", guidValue);
}

export function useDeletePiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "transaction", guidValue);
}

export function useDeletePiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "candidate", guidValue);
}

export function useDeletePiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "change", guidValue);
}

export function useDeletePiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "conflict", guidValue);
}

export function useDeletePiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "summary", guidValue);
}

export function useDeletePiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "metadata", guidValue);
}

export function useDeletePiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "createdAt", guidValue);
}

export function useDeletePiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "forward", guidValue);
}

export function useDeletePiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "backward", guidValue);
}

export function useCreateConnectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionKitInteraction", guidValue);
}

export function useCreateConnectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "id", guidValue);
}

export function useCreateConnectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "hash", guidValue);
}

export function useCreateConnectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "index", guidValue);
}

export function useCreateConnectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "kit", guidValue);
}

export function useCreateConnectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "kind", guidValue);
}

export function useCreateConnectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "actor", guidValue);
}

export function useCreateConnectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "session", guidValue);
}

export function useCreateConnectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "transaction", guidValue);
}

export function useCreateConnectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "candidate", guidValue);
}

export function useCreateConnectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "change", guidValue);
}

export function useCreateConnectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "conflict", guidValue);
}

export function useCreateConnectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "summary", guidValue);
}

export function useCreateConnectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "metadata", guidValue);
}

export function useCreateConnectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "createdAt", guidValue);
}

export function useCreateConnectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "forward", guidValue);
}

export function useCreateConnectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "backward", guidValue);
}

export function useCreateConnectionsKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionsKitInteraction", guidValue);
}

export function useCreateConnectionsKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "id", guidValue);
}

export function useCreateConnectionsKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "hash", guidValue);
}

export function useCreateConnectionsKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "index", guidValue);
}

export function useCreateConnectionsKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "kit", guidValue);
}

export function useCreateConnectionsKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "kind", guidValue);
}

export function useCreateConnectionsKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "actor", guidValue);
}

export function useCreateConnectionsKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "session", guidValue);
}

export function useCreateConnectionsKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "transaction", guidValue);
}

export function useCreateConnectionsKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "candidate", guidValue);
}

export function useCreateConnectionsKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "change", guidValue);
}

export function useCreateConnectionsKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "conflict", guidValue);
}

export function useCreateConnectionsKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "summary", guidValue);
}

export function useCreateConnectionsKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "metadata", guidValue);
}

export function useCreateConnectionsKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "createdAt", guidValue);
}

export function useCreateConnectionsKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "forward", guidValue);
}

export function useCreateConnectionsKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "backward", guidValue);
}

export function useUpdateConnectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionKitInteraction", guidValue);
}

export function useUpdateConnectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "id", guidValue);
}

export function useUpdateConnectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "hash", guidValue);
}

export function useUpdateConnectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "index", guidValue);
}

export function useUpdateConnectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "kit", guidValue);
}

export function useUpdateConnectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "kind", guidValue);
}

export function useUpdateConnectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "actor", guidValue);
}

export function useUpdateConnectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "session", guidValue);
}

export function useUpdateConnectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "transaction", guidValue);
}

export function useUpdateConnectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "candidate", guidValue);
}

export function useUpdateConnectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "change", guidValue);
}

export function useUpdateConnectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "conflict", guidValue);
}

export function useUpdateConnectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "summary", guidValue);
}

export function useUpdateConnectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "metadata", guidValue);
}

export function useUpdateConnectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "createdAt", guidValue);
}

export function useUpdateConnectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "forward", guidValue);
}

export function useUpdateConnectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "backward", guidValue);
}

export function useUpdateConnectionsKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionsKitInteraction", guidValue);
}

export function useUpdateConnectionsKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "id", guidValue);
}

export function useUpdateConnectionsKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "hash", guidValue);
}

export function useUpdateConnectionsKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "index", guidValue);
}

export function useUpdateConnectionsKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "kit", guidValue);
}

export function useUpdateConnectionsKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "kind", guidValue);
}

export function useUpdateConnectionsKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "actor", guidValue);
}

export function useUpdateConnectionsKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "session", guidValue);
}

export function useUpdateConnectionsKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "transaction", guidValue);
}

export function useUpdateConnectionsKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "candidate", guidValue);
}

export function useUpdateConnectionsKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "change", guidValue);
}

export function useUpdateConnectionsKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "conflict", guidValue);
}

export function useUpdateConnectionsKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "summary", guidValue);
}

export function useUpdateConnectionsKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "metadata", guidValue);
}

export function useUpdateConnectionsKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "createdAt", guidValue);
}

export function useUpdateConnectionsKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "forward", guidValue);
}

export function useUpdateConnectionsKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "backward", guidValue);
}

export function useDeleteConnectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionKitInteraction", guidValue);
}

export function useDeleteConnectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "id", guidValue);
}

export function useDeleteConnectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "hash", guidValue);
}

export function useDeleteConnectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "index", guidValue);
}

export function useDeleteConnectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "kit", guidValue);
}

export function useDeleteConnectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "kind", guidValue);
}

export function useDeleteConnectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "actor", guidValue);
}

export function useDeleteConnectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "session", guidValue);
}

export function useDeleteConnectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "transaction", guidValue);
}

export function useDeleteConnectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "candidate", guidValue);
}

export function useDeleteConnectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "change", guidValue);
}

export function useDeleteConnectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "conflict", guidValue);
}

export function useDeleteConnectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "summary", guidValue);
}

export function useDeleteConnectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "metadata", guidValue);
}

export function useDeleteConnectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "createdAt", guidValue);
}

export function useDeleteConnectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "forward", guidValue);
}

export function useDeleteConnectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "backward", guidValue);
}

export function useDeleteConnectionsKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionsKitInteraction", guidValue);
}

export function useDeleteConnectionsKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "id", guidValue);
}

export function useDeleteConnectionsKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "hash", guidValue);
}

export function useDeleteConnectionsKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "index", guidValue);
}

export function useDeleteConnectionsKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "kit", guidValue);
}

export function useDeleteConnectionsKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "kind", guidValue);
}

export function useDeleteConnectionsKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "actor", guidValue);
}

export function useDeleteConnectionsKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "session", guidValue);
}

export function useDeleteConnectionsKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "transaction", guidValue);
}

export function useDeleteConnectionsKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "candidate", guidValue);
}

export function useDeleteConnectionsKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "change", guidValue);
}

export function useDeleteConnectionsKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "conflict", guidValue);
}

export function useDeleteConnectionsKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "summary", guidValue);
}

export function useDeleteConnectionsKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "metadata", guidValue);
}

export function useDeleteConnectionsKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "createdAt", guidValue);
}

export function useDeleteConnectionsKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "forward", guidValue);
}

export function useDeleteConnectionsKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "backward", guidValue);
}

export function useDeleteSelectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteSelectionKitInteraction", guidValue);
}

export function useDeleteSelectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "id", guidValue);
}

export function useDeleteSelectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "hash", guidValue);
}

export function useDeleteSelectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "index", guidValue);
}

export function useDeleteSelectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "kit", guidValue);
}

export function useDeleteSelectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "kind", guidValue);
}

export function useDeleteSelectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "actor", guidValue);
}

export function useDeleteSelectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "session", guidValue);
}

export function useDeleteSelectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "transaction", guidValue);
}

export function useDeleteSelectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "candidate", guidValue);
}

export function useDeleteSelectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "change", guidValue);
}

export function useDeleteSelectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "conflict", guidValue);
}

export function useDeleteSelectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "summary", guidValue);
}

export function useDeleteSelectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "metadata", guidValue);
}

export function useDeleteSelectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "createdAt", guidValue);
}

export function useDeleteSelectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "forward", guidValue);
}

export function useDeleteSelectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "backward", guidValue);
}

export function useFixPiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FixPiecesKitInteraction", guidValue);
}

export function useFixPiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "id", guidValue);
}

export function useFixPiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "hash", guidValue);
}

export function useFixPiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "index", guidValue);
}

export function useFixPiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "kit", guidValue);
}

export function useFixPiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "kind", guidValue);
}

export function useFixPiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "actor", guidValue);
}

export function useFixPiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "session", guidValue);
}

export function useFixPiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "transaction", guidValue);
}

export function useFixPiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "candidate", guidValue);
}

export function useFixPiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "change", guidValue);
}

export function useFixPiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "conflict", guidValue);
}

export function useFixPiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "summary", guidValue);
}

export function useFixPiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "metadata", guidValue);
}

export function useFixPiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "createdAt", guidValue);
}

export function useFixPiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "forward", guidValue);
}

export function useFixPiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "backward", guidValue);
}

export function useClusterPiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ClusterPiecesKitInteraction", guidValue);
}

export function useClusterPiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "id", guidValue);
}

export function useClusterPiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "hash", guidValue);
}

export function useClusterPiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "index", guidValue);
}

export function useClusterPiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "kit", guidValue);
}

export function useClusterPiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "kind", guidValue);
}

export function useClusterPiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "actor", guidValue);
}

export function useClusterPiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "session", guidValue);
}

export function useClusterPiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "transaction", guidValue);
}

export function useClusterPiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "candidate", guidValue);
}

export function useClusterPiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "change", guidValue);
}

export function useClusterPiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "conflict", guidValue);
}

export function useClusterPiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "summary", guidValue);
}

export function useClusterPiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "metadata", guidValue);
}

export function useClusterPiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "createdAt", guidValue);
}

export function useClusterPiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "forward", guidValue);
}

export function useClusterPiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "backward", guidValue);
}

export function useExpandDesignReferenceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExpandDesignReferenceKitInteraction", guidValue);
}

export function useExpandDesignReferenceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "id", guidValue);
}

export function useExpandDesignReferenceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "hash", guidValue);
}

export function useExpandDesignReferenceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "index", guidValue);
}

export function useExpandDesignReferenceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kit", guidValue);
}

export function useExpandDesignReferenceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kind", guidValue);
}

export function useExpandDesignReferenceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "actor", guidValue);
}

export function useExpandDesignReferenceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "session", guidValue);
}

export function useExpandDesignReferenceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "transaction", guidValue);
}

export function useExpandDesignReferenceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "candidate", guidValue);
}

export function useExpandDesignReferenceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "change", guidValue);
}

export function useExpandDesignReferenceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "conflict", guidValue);
}

export function useExpandDesignReferenceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "summary", guidValue);
}

export function useExpandDesignReferenceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "metadata", guidValue);
}

export function useExpandDesignReferenceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "createdAt", guidValue);
}

export function useExpandDesignReferenceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "forward", guidValue);
}

export function useExpandDesignReferenceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "backward", guidValue);
}

export function useFlattenDesignKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FlattenDesignKitInteraction", guidValue);
}

export function useFlattenDesignKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "id", guidValue);
}

export function useFlattenDesignKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "hash", guidValue);
}

export function useFlattenDesignKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "index", guidValue);
}

export function useFlattenDesignKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "kit", guidValue);
}

export function useFlattenDesignKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "kind", guidValue);
}

export function useFlattenDesignKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "actor", guidValue);
}

export function useFlattenDesignKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "session", guidValue);
}

export function useFlattenDesignKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "transaction", guidValue);
}

export function useFlattenDesignKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "candidate", guidValue);
}

export function useFlattenDesignKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "change", guidValue);
}

export function useFlattenDesignKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "conflict", guidValue);
}

export function useFlattenDesignKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "summary", guidValue);
}

export function useFlattenDesignKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "metadata", guidValue);
}

export function useFlattenDesignKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "createdAt", guidValue);
}

export function useFlattenDesignKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "forward", guidValue);
}

export function useFlattenDesignKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "backward", guidValue);
}

export function useDragPiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DragPiecesKitInteraction", guidValue);
}

export function useDragPiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "id", guidValue);
}

export function useDragPiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "hash", guidValue);
}

export function useDragPiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "index", guidValue);
}

export function useDragPiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "kit", guidValue);
}

export function useDragPiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "kind", guidValue);
}

export function useDragPiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "actor", guidValue);
}

export function useDragPiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "session", guidValue);
}

export function useDragPiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "transaction", guidValue);
}

export function useDragPiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "candidate", guidValue);
}

export function useDragPiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "change", guidValue);
}

export function useDragPiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "conflict", guidValue);
}

export function useDragPiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "summary", guidValue);
}

export function useDragPiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "metadata", guidValue);
}

export function useDragPiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "createdAt", guidValue);
}

export function useDragPiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "forward", guidValue);
}

export function useDragPiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "backward", guidValue);
}

export function useMovePiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesKitInteraction", guidValue);
}

export function useMovePiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "id", guidValue);
}

export function useMovePiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "hash", guidValue);
}

export function useMovePiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "index", guidValue);
}

export function useMovePiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "kit", guidValue);
}

export function useMovePiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "kind", guidValue);
}

export function useMovePiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "actor", guidValue);
}

export function useMovePiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "session", guidValue);
}

export function useMovePiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "transaction", guidValue);
}

export function useMovePiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "candidate", guidValue);
}

export function useMovePiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "change", guidValue);
}

export function useMovePiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "conflict", guidValue);
}

export function useMovePiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "summary", guidValue);
}

export function useMovePiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "metadata", guidValue);
}

export function useMovePiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "createdAt", guidValue);
}

export function useMovePiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "forward", guidValue);
}

export function useMovePiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "backward", guidValue);
}

export function useCreateFixedPieceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFixedPieceKitInteraction", guidValue);
}

export function useCreateFixedPieceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "id", guidValue);
}

export function useCreateFixedPieceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "hash", guidValue);
}

export function useCreateFixedPieceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "index", guidValue);
}

export function useCreateFixedPieceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "kit", guidValue);
}

export function useCreateFixedPieceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "kind", guidValue);
}

export function useCreateFixedPieceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "actor", guidValue);
}

export function useCreateFixedPieceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "session", guidValue);
}

export function useCreateFixedPieceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "transaction", guidValue);
}

export function useCreateFixedPieceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "candidate", guidValue);
}

export function useCreateFixedPieceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "change", guidValue);
}

export function useCreateFixedPieceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "conflict", guidValue);
}

export function useCreateFixedPieceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "summary", guidValue);
}

export function useCreateFixedPieceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "metadata", guidValue);
}

export function useCreateFixedPieceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "createdAt", guidValue);
}

export function useCreateFixedPieceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "forward", guidValue);
}

export function useCreateFixedPieceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "backward", guidValue);
}

export function useCreateConnectedPieceKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectedPieceKitInteraction", guidValue);
}

export function useCreateConnectedPieceKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "id", guidValue);
}

export function useCreateConnectedPieceKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "hash", guidValue);
}

export function useCreateConnectedPieceKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "index", guidValue);
}

export function useCreateConnectedPieceKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kit", guidValue);
}

export function useCreateConnectedPieceKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kind", guidValue);
}

export function useCreateConnectedPieceKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "actor", guidValue);
}

export function useCreateConnectedPieceKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "session", guidValue);
}

export function useCreateConnectedPieceKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "transaction", guidValue);
}

export function useCreateConnectedPieceKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "candidate", guidValue);
}

export function useCreateConnectedPieceKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "change", guidValue);
}

export function useCreateConnectedPieceKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "conflict", guidValue);
}

export function useCreateConnectedPieceKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "summary", guidValue);
}

export function useCreateConnectedPieceKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "metadata", guidValue);
}

export function useCreateConnectedPieceKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "createdAt", guidValue);
}

export function useCreateConnectedPieceKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "forward", guidValue);
}

export function useCreateConnectedPieceKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "backward", guidValue);
}

export function useCreateHangingPiecesKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateHangingPiecesKitInteraction", guidValue);
}

export function useCreateHangingPiecesKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "id", guidValue);
}

export function useCreateHangingPiecesKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "hash", guidValue);
}

export function useCreateHangingPiecesKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "index", guidValue);
}

export function useCreateHangingPiecesKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kit", guidValue);
}

export function useCreateHangingPiecesKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kind", guidValue);
}

export function useCreateHangingPiecesKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "actor", guidValue);
}

export function useCreateHangingPiecesKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "session", guidValue);
}

export function useCreateHangingPiecesKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "transaction", guidValue);
}

export function useCreateHangingPiecesKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "candidate", guidValue);
}

export function useCreateHangingPiecesKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "change", guidValue);
}

export function useCreateHangingPiecesKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "conflict", guidValue);
}

export function useCreateHangingPiecesKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "summary", guidValue);
}

export function useCreateHangingPiecesKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "metadata", guidValue);
}

export function useCreateHangingPiecesKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "createdAt", guidValue);
}

export function useCreateHangingPiecesKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "forward", guidValue);
}

export function useCreateHangingPiecesKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "backward", guidValue);
}

export function useChangePieceTypeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePieceTypeKitInteraction", guidValue);
}

export function useChangePieceTypeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "id", guidValue);
}

export function useChangePieceTypeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "hash", guidValue);
}

export function useChangePieceTypeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "index", guidValue);
}

export function useChangePieceTypeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "kit", guidValue);
}

export function useChangePieceTypeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "kind", guidValue);
}

export function useChangePieceTypeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "actor", guidValue);
}

export function useChangePieceTypeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "session", guidValue);
}

export function useChangePieceTypeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "transaction", guidValue);
}

export function useChangePieceTypeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "candidate", guidValue);
}

export function useChangePieceTypeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "change", guidValue);
}

export function useChangePieceTypeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "conflict", guidValue);
}

export function useChangePieceTypeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "summary", guidValue);
}

export function useChangePieceTypeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "metadata", guidValue);
}

export function useChangePieceTypeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "createdAt", guidValue);
}

export function useChangePieceTypeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "forward", guidValue);
}

export function useChangePieceTypeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "backward", guidValue);
}

export function useChangePiecesTypeKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePiecesTypeKitInteraction", guidValue);
}

export function useChangePiecesTypeKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "id", guidValue);
}

export function useChangePiecesTypeKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "hash", guidValue);
}

export function useChangePiecesTypeKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "index", guidValue);
}

export function useChangePiecesTypeKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kit", guidValue);
}

export function useChangePiecesTypeKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kind", guidValue);
}

export function useChangePiecesTypeKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "actor", guidValue);
}

export function useChangePiecesTypeKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "session", guidValue);
}

export function useChangePiecesTypeKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "transaction", guidValue);
}

export function useChangePiecesTypeKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "candidate", guidValue);
}

export function useChangePiecesTypeKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "change", guidValue);
}

export function useChangePiecesTypeKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "conflict", guidValue);
}

export function useChangePiecesTypeKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "summary", guidValue);
}

export function useChangePiecesTypeKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "metadata", guidValue);
}

export function useChangePiecesTypeKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "createdAt", guidValue);
}

export function useChangePiecesTypeKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "forward", guidValue);
}

export function useChangePiecesTypeKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "backward", guidValue);
}

export function usePasteDesignSelectionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PasteDesignSelectionKitInteraction", guidValue);
}

export function usePasteDesignSelectionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "id", guidValue);
}

export function usePasteDesignSelectionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "hash", guidValue);
}

export function usePasteDesignSelectionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "index", guidValue);
}

export function usePasteDesignSelectionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kit", guidValue);
}

export function usePasteDesignSelectionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kind", guidValue);
}

export function usePasteDesignSelectionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "actor", guidValue);
}

export function usePasteDesignSelectionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "session", guidValue);
}

export function usePasteDesignSelectionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "transaction", guidValue);
}

export function usePasteDesignSelectionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "candidate", guidValue);
}

export function usePasteDesignSelectionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "change", guidValue);
}

export function usePasteDesignSelectionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "conflict", guidValue);
}

export function usePasteDesignSelectionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "summary", guidValue);
}

export function usePasteDesignSelectionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "metadata", guidValue);
}

export function usePasteDesignSelectionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "createdAt", guidValue);
}

export function usePasteDesignSelectionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "forward", guidValue);
}

export function usePasteDesignSelectionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "backward", guidValue);
}

export function useImportKitKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ImportKitKitInteraction", guidValue);
}

export function useImportKitKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "id", guidValue);
}

export function useImportKitKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "hash", guidValue);
}

export function useImportKitKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "index", guidValue);
}

export function useImportKitKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "kit", guidValue);
}

export function useImportKitKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "kind", guidValue);
}

export function useImportKitKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "actor", guidValue);
}

export function useImportKitKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "session", guidValue);
}

export function useImportKitKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "transaction", guidValue);
}

export function useImportKitKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "candidate", guidValue);
}

export function useImportKitKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "change", guidValue);
}

export function useImportKitKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "conflict", guidValue);
}

export function useImportKitKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "summary", guidValue);
}

export function useImportKitKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "metadata", guidValue);
}

export function useImportKitKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "createdAt", guidValue);
}

export function useImportKitKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "forward", guidValue);
}

export function useImportKitKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "backward", guidValue);
}

export function useResetKitKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResetKitKitInteraction", guidValue);
}

export function useResetKitKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "id", guidValue);
}

export function useResetKitKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "hash", guidValue);
}

export function useResetKitKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "index", guidValue);
}

export function useResetKitKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "kit", guidValue);
}

export function useResetKitKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "kind", guidValue);
}

export function useResetKitKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "actor", guidValue);
}

export function useResetKitKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "session", guidValue);
}

export function useResetKitKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "transaction", guidValue);
}

export function useResetKitKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "candidate", guidValue);
}

export function useResetKitKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "change", guidValue);
}

export function useResetKitKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "conflict", guidValue);
}

export function useResetKitKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "summary", guidValue);
}

export function useResetKitKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "metadata", guidValue);
}

export function useResetKitKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "createdAt", guidValue);
}

export function useResetKitKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "forward", guidValue);
}

export function useResetKitKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "backward", guidValue);
}

export function useExportKitKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExportKitKitInteraction", guidValue);
}

export function useExportKitKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "id", guidValue);
}

export function useExportKitKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "hash", guidValue);
}

export function useExportKitKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "index", guidValue);
}

export function useExportKitKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "kit", guidValue);
}

export function useExportKitKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "kind", guidValue);
}

export function useExportKitKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "actor", guidValue);
}

export function useExportKitKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "session", guidValue);
}

export function useExportKitKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "transaction", guidValue);
}

export function useExportKitKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "candidate", guidValue);
}

export function useExportKitKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "change", guidValue);
}

export function useExportKitKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "conflict", guidValue);
}

export function useExportKitKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "summary", guidValue);
}

export function useExportKitKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "metadata", guidValue);
}

export function useExportKitKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "createdAt", guidValue);
}

export function useExportKitKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "forward", guidValue);
}

export function useExportKitKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "backward", guidValue);
}

export function useStartKitSessionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StartKitSessionKitInteraction", guidValue);
}

export function useStartKitSessionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "id", guidValue);
}

export function useStartKitSessionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "hash", guidValue);
}

export function useStartKitSessionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "index", guidValue);
}

export function useStartKitSessionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "kit", guidValue);
}

export function useStartKitSessionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "kind", guidValue);
}

export function useStartKitSessionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "actor", guidValue);
}

export function useStartKitSessionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "session", guidValue);
}

export function useStartKitSessionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "transaction", guidValue);
}

export function useStartKitSessionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "candidate", guidValue);
}

export function useStartKitSessionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "change", guidValue);
}

export function useStartKitSessionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "conflict", guidValue);
}

export function useStartKitSessionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "summary", guidValue);
}

export function useStartKitSessionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "metadata", guidValue);
}

export function useStartKitSessionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "createdAt", guidValue);
}

export function useStartKitSessionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "forward", guidValue);
}

export function useStartKitSessionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "backward", guidValue);
}

export function useHeartbeatKitSessionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HeartbeatKitSessionKitInteraction", guidValue);
}

export function useHeartbeatKitSessionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "id", guidValue);
}

export function useHeartbeatKitSessionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "hash", guidValue);
}

export function useHeartbeatKitSessionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "index", guidValue);
}

export function useHeartbeatKitSessionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kit", guidValue);
}

export function useHeartbeatKitSessionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kind", guidValue);
}

export function useHeartbeatKitSessionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "actor", guidValue);
}

export function useHeartbeatKitSessionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "session", guidValue);
}

export function useHeartbeatKitSessionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "transaction", guidValue);
}

export function useHeartbeatKitSessionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "candidate", guidValue);
}

export function useHeartbeatKitSessionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "change", guidValue);
}

export function useHeartbeatKitSessionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "conflict", guidValue);
}

export function useHeartbeatKitSessionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "summary", guidValue);
}

export function useHeartbeatKitSessionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "metadata", guidValue);
}

export function useHeartbeatKitSessionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "createdAt", guidValue);
}

export function useHeartbeatKitSessionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "forward", guidValue);
}

export function useHeartbeatKitSessionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "backward", guidValue);
}

export function useEndKitSessionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("EndKitSessionKitInteraction", guidValue);
}

export function useEndKitSessionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "id", guidValue);
}

export function useEndKitSessionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "hash", guidValue);
}

export function useEndKitSessionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "index", guidValue);
}

export function useEndKitSessionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "kit", guidValue);
}

export function useEndKitSessionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "kind", guidValue);
}

export function useEndKitSessionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "actor", guidValue);
}

export function useEndKitSessionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "session", guidValue);
}

export function useEndKitSessionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "transaction", guidValue);
}

export function useEndKitSessionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "candidate", guidValue);
}

export function useEndKitSessionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "change", guidValue);
}

export function useEndKitSessionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "conflict", guidValue);
}

export function useEndKitSessionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "summary", guidValue);
}

export function useEndKitSessionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "metadata", guidValue);
}

export function useEndKitSessionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "createdAt", guidValue);
}

export function useEndKitSessionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "forward", guidValue);
}

export function useEndKitSessionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "backward", guidValue);
}

export function useReconnectKitSessionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ReconnectKitSessionKitInteraction", guidValue);
}

export function useReconnectKitSessionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "id", guidValue);
}

export function useReconnectKitSessionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "hash", guidValue);
}

export function useReconnectKitSessionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "index", guidValue);
}

export function useReconnectKitSessionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kit", guidValue);
}

export function useReconnectKitSessionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kind", guidValue);
}

export function useReconnectKitSessionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "actor", guidValue);
}

export function useReconnectKitSessionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "session", guidValue);
}

export function useReconnectKitSessionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "transaction", guidValue);
}

export function useReconnectKitSessionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "candidate", guidValue);
}

export function useReconnectKitSessionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "change", guidValue);
}

export function useReconnectKitSessionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "conflict", guidValue);
}

export function useReconnectKitSessionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "summary", guidValue);
}

export function useReconnectKitSessionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "metadata", guidValue);
}

export function useReconnectKitSessionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "createdAt", guidValue);
}

export function useReconnectKitSessionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "forward", guidValue);
}

export function useReconnectKitSessionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "backward", guidValue);
}

export function useBeginKitTransactionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BeginKitTransactionKitInteraction", guidValue);
}

export function useBeginKitTransactionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "id", guidValue);
}

export function useBeginKitTransactionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "hash", guidValue);
}

export function useBeginKitTransactionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "index", guidValue);
}

export function useBeginKitTransactionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "kit", guidValue);
}

export function useBeginKitTransactionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "kind", guidValue);
}

export function useBeginKitTransactionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "actor", guidValue);
}

export function useBeginKitTransactionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "session", guidValue);
}

export function useBeginKitTransactionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "transaction", guidValue);
}

export function useBeginKitTransactionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "candidate", guidValue);
}

export function useBeginKitTransactionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "change", guidValue);
}

export function useBeginKitTransactionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "conflict", guidValue);
}

export function useBeginKitTransactionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "summary", guidValue);
}

export function useBeginKitTransactionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "metadata", guidValue);
}

export function useBeginKitTransactionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "createdAt", guidValue);
}

export function useBeginKitTransactionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "forward", guidValue);
}

export function useBeginKitTransactionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "backward", guidValue);
}

export function useFinalizeKitTransactionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FinalizeKitTransactionKitInteraction", guidValue);
}

export function useFinalizeKitTransactionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "id", guidValue);
}

export function useFinalizeKitTransactionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "hash", guidValue);
}

export function useFinalizeKitTransactionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "index", guidValue);
}

export function useFinalizeKitTransactionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kit", guidValue);
}

export function useFinalizeKitTransactionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kind", guidValue);
}

export function useFinalizeKitTransactionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "actor", guidValue);
}

export function useFinalizeKitTransactionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "session", guidValue);
}

export function useFinalizeKitTransactionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "transaction", guidValue);
}

export function useFinalizeKitTransactionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "candidate", guidValue);
}

export function useFinalizeKitTransactionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "change", guidValue);
}

export function useFinalizeKitTransactionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "conflict", guidValue);
}

export function useFinalizeKitTransactionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "summary", guidValue);
}

export function useFinalizeKitTransactionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "metadata", guidValue);
}

export function useFinalizeKitTransactionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "createdAt", guidValue);
}

export function useFinalizeKitTransactionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "forward", guidValue);
}

export function useFinalizeKitTransactionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "backward", guidValue);
}

export function useAbortKitTransactionKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AbortKitTransactionKitInteraction", guidValue);
}

export function useAbortKitTransactionKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "id", guidValue);
}

export function useAbortKitTransactionKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "hash", guidValue);
}

export function useAbortKitTransactionKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "index", guidValue);
}

export function useAbortKitTransactionKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "kit", guidValue);
}

export function useAbortKitTransactionKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "kind", guidValue);
}

export function useAbortKitTransactionKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "actor", guidValue);
}

export function useAbortKitTransactionKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "session", guidValue);
}

export function useAbortKitTransactionKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "transaction", guidValue);
}

export function useAbortKitTransactionKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "candidate", guidValue);
}

export function useAbortKitTransactionKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "change", guidValue);
}

export function useAbortKitTransactionKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "conflict", guidValue);
}

export function useAbortKitTransactionKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "summary", guidValue);
}

export function useAbortKitTransactionKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "metadata", guidValue);
}

export function useAbortKitTransactionKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "createdAt", guidValue);
}

export function useAbortKitTransactionKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "forward", guidValue);
}

export function useAbortKitTransactionKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "backward", guidValue);
}

export function useTransactionStepKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionStepKitInteraction", guidValue);
}

export function useTransactionStepKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "id", guidValue);
}

export function useTransactionStepKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "hash", guidValue);
}

export function useTransactionStepKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "index", guidValue);
}

export function useTransactionStepKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "kit", guidValue);
}

export function useTransactionStepKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "kind", guidValue);
}

export function useTransactionStepKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "actor", guidValue);
}

export function useTransactionStepKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "session", guidValue);
}

export function useTransactionStepKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "transaction", guidValue);
}

export function useTransactionStepKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "candidate", guidValue);
}

export function useTransactionStepKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "change", guidValue);
}

export function useTransactionStepKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "conflict", guidValue);
}

export function useTransactionStepKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "summary", guidValue);
}

export function useTransactionStepKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "metadata", guidValue);
}

export function useTransactionStepKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "createdAt", guidValue);
}

export function useTransactionStepKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "forward", guidValue);
}

export function useTransactionStepKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "backward", guidValue);
}

export function useHistoryStepKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HistoryStepKitInteraction", guidValue);
}

export function useHistoryStepKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "id", guidValue);
}

export function useHistoryStepKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "hash", guidValue);
}

export function useHistoryStepKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "index", guidValue);
}

export function useHistoryStepKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "kit", guidValue);
}

export function useHistoryStepKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "kind", guidValue);
}

export function useHistoryStepKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "actor", guidValue);
}

export function useHistoryStepKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "session", guidValue);
}

export function useHistoryStepKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "transaction", guidValue);
}

export function useHistoryStepKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "candidate", guidValue);
}

export function useHistoryStepKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "change", guidValue);
}

export function useHistoryStepKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "conflict", guidValue);
}

export function useHistoryStepKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "summary", guidValue);
}

export function useHistoryStepKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "metadata", guidValue);
}

export function useHistoryStepKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "createdAt", guidValue);
}

export function useHistoryStepKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "forward", guidValue);
}

export function useHistoryStepKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "backward", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VoteOnKitChangeCandidateKitInteraction", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "id", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "hash", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "index", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kit", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kind", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "actor", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "session", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "transaction", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "candidate", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "change", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "conflict", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "summary", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "metadata", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "createdAt", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "forward", guidValue);
}

export function useVoteOnKitChangeCandidateKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "backward", guidValue);
}

export function useResolveKitConflictKitInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResolveKitConflictKitInteraction", guidValue);
}

export function useResolveKitConflictKitInteractionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "id", guidValue);
}

export function useResolveKitConflictKitInteractionHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "hash", guidValue);
}

export function useResolveKitConflictKitInteractionIndex(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "index", guidValue);
}

export function useResolveKitConflictKitInteractionKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "kit", guidValue);
}

export function useResolveKitConflictKitInteractionKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "kind", guidValue);
}

export function useResolveKitConflictKitInteractionActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "actor", guidValue);
}

export function useResolveKitConflictKitInteractionSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "session", guidValue);
}

export function useResolveKitConflictKitInteractionTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "transaction", guidValue);
}

export function useResolveKitConflictKitInteractionCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "candidate", guidValue);
}

export function useResolveKitConflictKitInteractionChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "change", guidValue);
}

export function useResolveKitConflictKitInteractionConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "conflict", guidValue);
}

export function useResolveKitConflictKitInteractionSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "summary", guidValue);
}

export function useResolveKitConflictKitInteractionMetadata(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "metadata", guidValue);
}

export function useResolveKitConflictKitInteractionCreatedAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "createdAt", guidValue);
}

export function useResolveKitConflictKitInteractionForward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "forward", guidValue);
}

export function useResolveKitConflictKitInteractionBackward(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "backward", guidValue);
}

export function useKitInteractionPage(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInteractionPage", guidValue);
}

export function useKitInteractionPageHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "hash", guidValue);
}

export function useKitInteractionPageNodes(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "nodes", guidValue);
}

export function useKitInteractionPagePageInfo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "pageInfo", guidValue);
}

export function useKitInteractionPageTotalCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "totalCount", guidValue);
}

export function useKitHistory(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistory", guidValue);
}

export function useKitHistoryHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "hash", guidValue);
}

export function useKitHistoryCanUndo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "canUndo", guidValue);
}

export function useKitHistoryCanRedo(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "canRedo", guidValue);
}

export function useKitHistoryTotalCount(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "totalCount", guidValue);
}

export function useKitHistoryHead(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "head", guidValue);
}

export function useKitStoreEntity(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStore", guidValue);
}

export function useKitStoreHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "hash", guidValue);
}

export function useKitStoreKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "kit", guidValue);
}

export function useKitStoreBackbone(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "backbone", guidValue);
}

export function useKitStoreSessions(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "sessions", guidValue);
}

export function useKitStoreTransactions(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "transactions", guidValue);
}

export function useKitStorePendingCandidates(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "pendingCandidates", guidValue);
}

export function useKitStoreActiveConflicts(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "activeConflicts", guidValue);
}

export function useKitStoreValidation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "validation", guidValue);
}

export function useKitStoreHistory(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "history", guidValue);
}

export function useKitStoreBlockedByConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "blockedByConflict", guidValue);
}

export function useKitStoreStrictMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "strictMode", guidValue);
}

export function useArtifactKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ArtifactKind", guidValue);
}

export function useSelectionMutationMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SelectionMutationMode", guidValue);
}

export function useKitArchiveExport(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitArchiveExport", guidValue);
}

export function useKitArchiveExportHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "hash", guidValue);
}

export function useKitArchiveExportFileName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "fileName", guidValue);
}

export function useKitArchiveExportUrl(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "url", guidValue);
}

export function useKitArchiveExportExpiresAt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "expiresAt", guidValue);
}

export function useKitMutationResult(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitMutationResult", guidValue);
}

export function useKitMutationResultHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "hash", guidValue);
}

export function useKitMutationResultAccepted(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "accepted", guidValue);
}

export function useKitMutationResultKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "kind", guidValue);
}

export function useKitMutationResultSummary(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "summary", guidValue);
}

export function useKitMutationResultStore(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "store", guidValue);
}

export function useKitMutationResultKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "kit", guidValue);
}

export function useKitMutationResultSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "session", guidValue);
}

export function useKitMutationResultTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "transaction", guidValue);
}

export function useKitMutationResultCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "candidate", guidValue);
}

export function useKitMutationResultChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "change", guidValue);
}

export function useKitMutationResultHistoryEntry(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "historyEntry", guidValue);
}

export function useKitMutationResultConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "conflict", guidValue);
}

export function useKitMutationResultValidation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "validation", guidValue);
}

export function useKitMutationResultExport(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "export", guidValue);
}

export function useKitCommandContextInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandContextInput", guidValue);
}

export function useKitCommandContextInputKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "kitId", guidValue);
}

export function useKitCommandContextInputSessionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "sessionId", guidValue);
}

export function useKitCommandContextInputTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "transactionId", guidValue);
}

export function useKitCommandContextInputOrigin(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "origin", guidValue);
}

export function useKitCommandContextInputExpectedHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "expectedHash", guidValue);
}

export function useKitCommandContextInputStrictMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "strictMode", guidValue);
}

export function useStartKitSessionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StartKitSessionInput", guidValue);
}

export function useStartKitSessionInputKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "kitId", guidValue);
}

export function useStartKitSessionInputActor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "actor", guidValue);
}

export function useStartKitSessionInputClient(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "client", guidValue);
}

export function useStartKitSessionInputStrictMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "strictMode", guidValue);
}

export function useHeartbeatKitSessionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HeartbeatKitSessionInput", guidValue);
}

export function useHeartbeatKitSessionInputKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "kitId", guidValue);
}

export function useHeartbeatKitSessionInputSessionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "sessionId", guidValue);
}

export function useHeartbeatKitSessionInputLastKnownHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "lastKnownHash", guidValue);
}

export function useEndKitSessionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("EndKitSessionInput", guidValue);
}

export function useEndKitSessionInputKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionInput", "kitId", guidValue);
}

export function useEndKitSessionInputSessionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionInput", "sessionId", guidValue);
}

export function useReconnectKitSessionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ReconnectKitSessionInput", guidValue);
}

export function useReconnectKitSessionInputKitId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "kitId", guidValue);
}

export function useReconnectKitSessionInputSessionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "sessionId", guidValue);
}

export function useReconnectKitSessionInputClient(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "client", guidValue);
}

export function useReconnectKitSessionInputLastKnownHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "lastKnownHash", guidValue);
}

export function useSetSessionSelectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SetSessionSelectionCommandInput", guidValue);
}

export function useSetSessionSelectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "context", guidValue);
}

export function useSetSessionSelectionCommandInputMode(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "mode", guidValue);
}

export function useSetSessionSelectionCommandInputSelection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "selection", guidValue);
}

export function useBeginKitTransactionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BeginKitTransactionInput", guidValue);
}

export function useBeginKitTransactionInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "context", guidValue);
}

export function useBeginKitTransactionInputLabel(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "label", guidValue);
}

export function useBeginKitTransactionInputParentTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "parentTransactionId", guidValue);
}

export function useFinalizeKitTransactionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FinalizeKitTransactionInput", guidValue);
}

export function useFinalizeKitTransactionInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionInput", "context", guidValue);
}

export function useFinalizeKitTransactionInputTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionInput", "transactionId", guidValue);
}

export function useAbortKitTransactionInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AbortKitTransactionInput", guidValue);
}

export function useAbortKitTransactionInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionInput", "context", guidValue);
}

export function useAbortKitTransactionInputTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionInput", "transactionId", guidValue);
}

export function useTransactionStepInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionStepInput", guidValue);
}

export function useTransactionStepInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepInput", "context", guidValue);
}

export function useTransactionStepInputTransactionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepInput", "transactionId", guidValue);
}

export function useHistoryStepInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HistoryStepInput", guidValue);
}

export function useHistoryStepInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepInput", "context", guidValue);
}

export function useHistoryStepInputSteps(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepInput", "steps", guidValue);
}

export function useVoteOnKitChangeCandidateInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VoteOnKitChangeCandidateInput", guidValue);
}

export function useVoteOnKitChangeCandidateInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "context", guidValue);
}

export function useVoteOnKitChangeCandidateInputCandidateId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "candidateId", guidValue);
}

export function useVoteOnKitChangeCandidateInputState(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "state", guidValue);
}

export function useVoteOnKitChangeCandidateInputReason(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "reason", guidValue);
}

export function useVoteOnKitChangeCandidateInputResolutionOptionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "resolutionOptionId", guidValue);
}

export function useResolveKitConflictInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResolveKitConflictInput", guidValue);
}

export function useResolveKitConflictInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "context", guidValue);
}

export function useResolveKitConflictInputConflictId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "conflictId", guidValue);
}

export function useResolveKitConflictInputOptionId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "optionId", guidValue);
}

export function useResolveKitConflictInputPayload(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "payload", guidValue);
}

export function useCreateAuthorCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateAuthorCommandInput", guidValue);
}

export function useCreateAuthorCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorCommandInput", "context", guidValue);
}

export function useCreateAuthorCommandInputAuthor(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorCommandInput", "author", guidValue);
}

export function useUpdateAuthorCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateAuthorCommandInput", guidValue);
}

export function useUpdateAuthorCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "context", guidValue);
}

export function useUpdateAuthorCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "id", guidValue);
}

export function useUpdateAuthorCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "patch", guidValue);
}

export function useDeleteAuthorCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteAuthorCommandInput", guidValue);
}

export function useDeleteAuthorCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorCommandInput", "context", guidValue);
}

export function useDeleteAuthorCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorCommandInput", "id", guidValue);
}

export function useCreateTypeCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTypeCommandInput", guidValue);
}

export function useCreateTypeCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeCommandInput", "context", guidValue);
}

export function useCreateTypeCommandInputType(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeCommandInput", "type", guidValue);
}

export function useUpdateTypeCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTypeCommandInput", guidValue);
}

export function useUpdateTypeCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "context", guidValue);
}

export function useUpdateTypeCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "id", guidValue);
}

export function useUpdateTypeCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "patch", guidValue);
}

export function useDeleteTypeCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTypeCommandInput", guidValue);
}

export function useDeleteTypeCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeCommandInput", "context", guidValue);
}

export function useDeleteTypeCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeCommandInput", "id", guidValue);
}

export function useCreateDesignCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateDesignCommandInput", guidValue);
}

export function useCreateDesignCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignCommandInput", "context", guidValue);
}

export function useCreateDesignCommandInputDesign(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignCommandInput", "design", guidValue);
}

export function useUpdateDesignCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateDesignCommandInput", guidValue);
}

export function useUpdateDesignCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "context", guidValue);
}

export function useUpdateDesignCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "id", guidValue);
}

export function useUpdateDesignCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "patch", guidValue);
}

export function useDeleteDesignCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteDesignCommandInput", guidValue);
}

export function useDeleteDesignCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignCommandInput", "context", guidValue);
}

export function useDeleteDesignCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignCommandInput", "id", guidValue);
}

export function useCreateQualityCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateQualityCommandInput", guidValue);
}

export function useCreateQualityCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityCommandInput", "context", guidValue);
}

export function useCreateQualityCommandInputQuality(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityCommandInput", "quality", guidValue);
}

export function useUpdateQualityCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateQualityCommandInput", guidValue);
}

export function useUpdateQualityCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "context", guidValue);
}

export function useUpdateQualityCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "id", guidValue);
}

export function useUpdateQualityCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "patch", guidValue);
}

export function useDeleteQualityCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteQualityCommandInput", guidValue);
}

export function useDeleteQualityCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityCommandInput", "context", guidValue);
}

export function useDeleteQualityCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityCommandInput", "id", guidValue);
}

export function useCreatePortCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePortCommandInput", guidValue);
}

export function useCreatePortCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortCommandInput", "context", guidValue);
}

export function useCreatePortCommandInputPort(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortCommandInput", "port", guidValue);
}

export function useUpdatePortCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePortCommandInput", guidValue);
}

export function useUpdatePortCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "context", guidValue);
}

export function useUpdatePortCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "id", guidValue);
}

export function useUpdatePortCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "patch", guidValue);
}

export function useDeletePortCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePortCommandInput", guidValue);
}

export function useDeletePortCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortCommandInput", "context", guidValue);
}

export function useDeletePortCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortCommandInput", "id", guidValue);
}

export function useCreateFamilyCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFamilyCommandInput", guidValue);
}

export function useCreateFamilyCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyCommandInput", "context", guidValue);
}

export function useCreateFamilyCommandInputFamily(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyCommandInput", "family", guidValue);
}

export function useUpdateFamilyCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFamilyCommandInput", guidValue);
}

export function useUpdateFamilyCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "context", guidValue);
}

export function useUpdateFamilyCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "id", guidValue);
}

export function useUpdateFamilyCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "patch", guidValue);
}

export function useDeleteFamilyCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFamilyCommandInput", guidValue);
}

export function useDeleteFamilyCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyCommandInput", "context", guidValue);
}

export function useDeleteFamilyCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyCommandInput", "id", guidValue);
}

export function useCreateTagCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTagCommandInput", guidValue);
}

export function useCreateTagCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagCommandInput", "context", guidValue);
}

export function useCreateTagCommandInputTag(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagCommandInput", "tag", guidValue);
}

export function useUpdateTagCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTagCommandInput", guidValue);
}

export function useUpdateTagCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "context", guidValue);
}

export function useUpdateTagCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "id", guidValue);
}

export function useUpdateTagCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "patch", guidValue);
}

export function useDeleteTagCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTagCommandInput", guidValue);
}

export function useDeleteTagCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagCommandInput", "context", guidValue);
}

export function useDeleteTagCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagCommandInput", "id", guidValue);
}

export function useCreateConceptCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConceptCommandInput", guidValue);
}

export function useCreateConceptCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptCommandInput", "context", guidValue);
}

export function useCreateConceptCommandInputConcept(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptCommandInput", "concept", guidValue);
}

export function useUpdateConceptCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConceptCommandInput", guidValue);
}

export function useUpdateConceptCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "context", guidValue);
}

export function useUpdateConceptCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "id", guidValue);
}

export function useUpdateConceptCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "patch", guidValue);
}

export function useDeleteConceptCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConceptCommandInput", guidValue);
}

export function useDeleteConceptCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptCommandInput", "context", guidValue);
}

export function useDeleteConceptCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptCommandInput", "id", guidValue);
}

export function useCreateFileCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFileCommandInput", guidValue);
}

export function useCreateFileCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileCommandInput", "context", guidValue);
}

export function useCreateFileCommandInputFile(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileCommandInput", "file", guidValue);
}

export function useUpdateFileCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFileCommandInput", guidValue);
}

export function useUpdateFileCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "context", guidValue);
}

export function useUpdateFileCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "id", guidValue);
}

export function useUpdateFileCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "patch", guidValue);
}

export function useDeleteFileCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFileCommandInput", guidValue);
}

export function useDeleteFileCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileCommandInput", "context", guidValue);
}

export function useDeleteFileCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileCommandInput", "id", guidValue);
}

export function useCreateFolderCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFolderCommandInput", guidValue);
}

export function useCreateFolderCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderCommandInput", "context", guidValue);
}

export function useCreateFolderCommandInputFolder(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderCommandInput", "folder", guidValue);
}

export function useUpdateFolderCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFolderCommandInput", guidValue);
}

export function useUpdateFolderCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "context", guidValue);
}

export function useUpdateFolderCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "id", guidValue);
}

export function useUpdateFolderCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "patch", guidValue);
}

export function useDeleteFolderCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFolderCommandInput", guidValue);
}

export function useDeleteFolderCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderCommandInput", "context", guidValue);
}

export function useDeleteFolderCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderCommandInput", "id", guidValue);
}

export function useMoveArtifactToFolderCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MoveArtifactToFolderCommandInput", guidValue);
}

export function useMoveArtifactToFolderCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "context", guidValue);
}

export function useMoveArtifactToFolderCommandInputArtifactKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactKind", guidValue);
}

export function useMoveArtifactToFolderCommandInputArtifactId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactId", guidValue);
}

export function useMoveArtifactToFolderCommandInputFolderId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "folderId", guidValue);
}

export function useCreatePieceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePieceCommandInput", guidValue);
}

export function useCreatePieceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "context", guidValue);
}

export function useCreatePieceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "designId", guidValue);
}

export function useCreatePieceCommandInputPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "piece", guidValue);
}

export function useCreatePiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePiecesCommandInput", guidValue);
}

export function useCreatePiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "context", guidValue);
}

export function useCreatePiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "designId", guidValue);
}

export function useCreatePiecesCommandInputPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "pieces", guidValue);
}

export function usePieceUpdateInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceUpdateInput", guidValue);
}

export function usePieceUpdateInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceUpdateInput", "id", guidValue);
}

export function usePieceUpdateInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceUpdateInput", "patch", guidValue);
}

export function useUpdatePieceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePieceCommandInput", guidValue);
}

export function useUpdatePieceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "context", guidValue);
}

export function useUpdatePieceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "designId", guidValue);
}

export function useUpdatePieceCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "id", guidValue);
}

export function useUpdatePieceCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "patch", guidValue);
}

export function useUpdatePiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePiecesCommandInput", guidValue);
}

export function useUpdatePiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "context", guidValue);
}

export function useUpdatePiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "designId", guidValue);
}

export function useUpdatePiecesCommandInputUpdates(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "updates", guidValue);
}

export function useDeletePieceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePieceCommandInput", guidValue);
}

export function useDeletePieceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "context", guidValue);
}

export function useDeletePieceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "designId", guidValue);
}

export function useDeletePieceCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "id", guidValue);
}

export function useDeletePiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePiecesCommandInput", guidValue);
}

export function useDeletePiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "context", guidValue);
}

export function useDeletePiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "designId", guidValue);
}

export function useDeletePiecesCommandInputIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "ids", guidValue);
}

export function useCreateConnectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionCommandInput", guidValue);
}

export function useCreateConnectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "context", guidValue);
}

export function useCreateConnectionCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "designId", guidValue);
}

export function useCreateConnectionCommandInputConnection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "connection", guidValue);
}

export function useCreateConnectionsCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionsCommandInput", guidValue);
}

export function useCreateConnectionsCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "context", guidValue);
}

export function useCreateConnectionsCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "designId", guidValue);
}

export function useCreateConnectionsCommandInputConnections(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "connections", guidValue);
}

export function useConnectionUpdateInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionUpdateInput", guidValue);
}

export function useConnectionUpdateInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionUpdateInput", "id", guidValue);
}

export function useConnectionUpdateInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionUpdateInput", "patch", guidValue);
}

export function useUpdateConnectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionCommandInput", guidValue);
}

export function useUpdateConnectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "context", guidValue);
}

export function useUpdateConnectionCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "designId", guidValue);
}

export function useUpdateConnectionCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "id", guidValue);
}

export function useUpdateConnectionCommandInputPatch(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "patch", guidValue);
}

export function useUpdateConnectionsCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionsCommandInput", guidValue);
}

export function useUpdateConnectionsCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "context", guidValue);
}

export function useUpdateConnectionsCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "designId", guidValue);
}

export function useUpdateConnectionsCommandInputUpdates(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "updates", guidValue);
}

export function useDeleteConnectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionCommandInput", guidValue);
}

export function useDeleteConnectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "context", guidValue);
}

export function useDeleteConnectionCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "designId", guidValue);
}

export function useDeleteConnectionCommandInputId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "id", guidValue);
}

export function useDeleteConnectionsCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionsCommandInput", guidValue);
}

export function useDeleteConnectionsCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "context", guidValue);
}

export function useDeleteConnectionsCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "designId", guidValue);
}

export function useDeleteConnectionsCommandInputIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "ids", guidValue);
}

export function useDeleteSelectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteSelectionCommandInput", guidValue);
}

export function useDeleteSelectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "context", guidValue);
}

export function useDeleteSelectionCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "designId", guidValue);
}

export function useDeleteSelectionCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "pieceIds", guidValue);
}

export function useDeleteSelectionCommandInputConnectionIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "connectionIds", guidValue);
}

export function useFixPiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FixPiecesCommandInput", guidValue);
}

export function useFixPiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "context", guidValue);
}

export function useFixPiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "designId", guidValue);
}

export function useFixPiecesCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "pieceIds", guidValue);
}

export function useClusterPiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ClusterPiecesCommandInput", guidValue);
}

export function useClusterPiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "context", guidValue);
}

export function useClusterPiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "designId", guidValue);
}

export function useClusterPiecesCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "pieceIds", guidValue);
}

export function useClusterPiecesCommandInputNewDesignName(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "newDesignName", guidValue);
}

export function useExpandDesignReferenceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExpandDesignReferenceCommandInput", guidValue);
}

export function useExpandDesignReferenceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "context", guidValue);
}

export function useExpandDesignReferenceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "designId", guidValue);
}

export function useExpandDesignReferenceCommandInputReferencedDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "referencedDesignId", guidValue);
}

export function useFlattenDesignCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FlattenDesignCommandInput", guidValue);
}

export function useFlattenDesignCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignCommandInput", "context", guidValue);
}

export function useFlattenDesignCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignCommandInput", "designId", guidValue);
}

export function useDragPiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DragPiecesCommandInput", guidValue);
}

export function useDragPiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "context", guidValue);
}

export function useDragPiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "designId", guidValue);
}

export function useDragPiecesCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "pieceIds", guidValue);
}

export function useDragPiecesCommandInputOffset(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "offset", guidValue);
}

export function useMovePiecesVectorInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesVectorInput", guidValue);
}

export function useMovePiecesVectorInputShift(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "shift", guidValue);
}

export function useMovePiecesVectorInputGap(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "gap", guidValue);
}

export function useMovePiecesVectorInputRise(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "rise", guidValue);
}

export function useMovePiecesVectorInputRotation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "rotation", guidValue);
}

export function useMovePiecesVectorInputTurn(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "turn", guidValue);
}

export function useMovePiecesVectorInputTilt(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "tilt", guidValue);
}

export function useMovePiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesCommandInput", guidValue);
}

export function useMovePiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "context", guidValue);
}

export function useMovePiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "designId", guidValue);
}

export function useMovePiecesCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "pieceIds", guidValue);
}

export function useMovePiecesCommandInputVector(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "vector", guidValue);
}

export function useCreateFixedPieceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFixedPieceCommandInput", guidValue);
}

export function useCreateFixedPieceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "context", guidValue);
}

export function useCreateFixedPieceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "designId", guidValue);
}

export function useCreateFixedPieceCommandInputPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "piece", guidValue);
}

export function useCreateConnectedPieceCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectedPieceCommandInput", guidValue);
}

export function useCreateConnectedPieceCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "context", guidValue);
}

export function useCreateConnectedPieceCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "designId", guidValue);
}

export function useCreateConnectedPieceCommandInputPiece(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "piece", guidValue);
}

export function useCreateConnectedPieceCommandInputConnection(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "connection", guidValue);
}

export function useCreateHangingPiecesCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateHangingPiecesCommandInput", guidValue);
}

export function useCreateHangingPiecesCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "context", guidValue);
}

export function useCreateHangingPiecesCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "designId", guidValue);
}

export function useCreateHangingPiecesCommandInputPieces(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "pieces", guidValue);
}

export function useCreateHangingPiecesCommandInputParentPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentPieceId", guidValue);
}

export function useCreateHangingPiecesCommandInputParentDesignPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentDesignPieceId", guidValue);
}

export function useCreateHangingPiecesCommandInputParentConnectorId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentConnectorId", guidValue);
}

export function useCreateHangingPiecesCommandInputConnectionTemplate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "connectionTemplate", guidValue);
}

export function useChangePieceTypeCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePieceTypeCommandInput", guidValue);
}

export function useChangePieceTypeCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "context", guidValue);
}

export function useChangePieceTypeCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "designId", guidValue);
}

export function useChangePieceTypeCommandInputPieceId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "pieceId", guidValue);
}

export function useChangePieceTypeCommandInputTypeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "typeId", guidValue);
}

export function useChangePiecesTypeCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePiecesTypeCommandInput", guidValue);
}

export function useChangePiecesTypeCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "context", guidValue);
}

export function useChangePiecesTypeCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "designId", guidValue);
}

export function useChangePiecesTypeCommandInputPieceIds(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "pieceIds", guidValue);
}

export function useChangePiecesTypeCommandInputTypeId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "typeId", guidValue);
}

export function usePasteDesignSelectionCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PasteDesignSelectionCommandInput", guidValue);
}

export function usePasteDesignSelectionCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "context", guidValue);
}

export function usePasteDesignSelectionCommandInputDesignId(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "designId", guidValue);
}

export function usePasteDesignSelectionCommandInputPayload(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "payload", guidValue);
}

export function usePasteDesignSelectionCommandInputOffset(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "offset", guidValue);
}

export function useImportKitCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ImportKitCommandInput", guidValue);
}

export function useImportKitCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "context", guidValue);
}

export function useImportKitCommandInputSourceUrl(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "sourceUrl", guidValue);
}

export function useImportKitCommandInputArchiveBase64(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "archiveBase64", guidValue);
}

export function useResetKitCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResetKitCommandInput", guidValue);
}

export function useResetKitCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "context", guidValue);
}

export function useResetKitCommandInputSourceUrl(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "sourceUrl", guidValue);
}

export function useResetKitCommandInputArchiveBase64(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "archiveBase64", guidValue);
}

export function useResetKitCommandInputKit(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "kit", guidValue);
}

export function useExportKitCommandInput(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExportKitCommandInput", guidValue);
}

export function useExportKitCommandInputContext(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitCommandInput", "context", guidValue);
}

export function useQuery(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Query", guidValue);
}

export function useQueryKitCommandCatalog(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Query", "kitCommandCatalog", guidValue);
}

export function useMutation(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Mutation", guidValue);
}

export function useKitStoreEventKindEnum(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStoreEventKind", guidValue);
}

export function useKitStoreEvent(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStoreEvent", guidValue);
}

export function useKitStoreEventHash(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "hash", guidValue);
}

export function useKitStoreEventKind(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "kind", guidValue);
}

export function useKitStoreEventStore(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "store", guidValue);
}

export function useKitStoreEventInteraction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "interaction", guidValue);
}

export function useKitStoreEventChange(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "change", guidValue);
}

export function useKitStoreEventCandidate(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "candidate", guidValue);
}

export function useKitStoreEventConflict(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "conflict", guidValue);
}

export function useKitStoreEventSession(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "session", guidValue);
}

export function useKitStoreEventTransaction(guidValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "transaction", guidValue);
}

export const schemaHooks = Object.freeze({
	useJSON,
	useActorKind,
	useActor,
	useActorId,
	useActorName,
	useActorEmail,
	useActorColor,
	useUser,
	useUserHash,
	useUserId,
	useUserName,
	useUserEmail,
	useUserColor,
	useEndKitSessionKitInteractionBackward,
	useReconnectKitSessionKitInteraction,
	useReconnectKitSessionKitInteractionId,
	useDragPiecesKitInteractionIndex,
	useDragPiecesKitInteractionKit,
	useDragPiecesKitInteractionKind,
	useDragPiecesKitInteractionActor,
	useDragPiecesKitInteractionSession,
	useDragPiecesKitInteractionTransaction,
	useDragPiecesKitInteractionForward,
	useDragPiecesKitInteractionBackward,
	usePasteDesignSelectionKitInteractionChange,
	usePasteDesignSelectionKitInteractionConflict,
	usePasteDesignSelectionKitInteractionSummary,
	usePasteDesignSelectionKitInteractionMetadata,
	usePasteDesignSelectionKitInteractionCreatedAt,
	usePasteDesignSelectionKitInteractionForward,
	usePasteDesignSelectionKitInteractionBackward,
	useImportKitKitInteractionId,
	useImportKitKitInteractionHash,
	useImportKitKitInteractionIndex,
	useImportKitKitInteractionKit,
	useImportKitKitInteractionKind,
	useImportKitKitInteractionActor,
	useImportKitKitInteractionSession,
	useImportKitKitInteractionTransaction,
	useImportKitKitInteractionCandidate,
	useImportKitKitInteractionChange,
	useImportKitKitInteractionConflict,
	useImportKitKitInteractionSummary,
	useImportKitKitInteractionMetadata,
	useImportKitKitInteractionCreatedAt,
	useImportKitKitInteractionForward,
	useImportKitKitInteractionBackward,
	useResetKitKitInteractionId,
	useResetKitKitInteractionHash,
	useResetKitKitInteractionIndex,
	useResetKitKitInteractionKit,
	useResetKitKitInteractionKind,
	useResetKitKitInteractionActor,
	useResetKitKitInteractionSession,
	useResetKitKitInteractionTransaction,
	useResetKitKitInteractionCandidate,
	useResetKitKitInteractionChange,
	useResetKitKitInteractionConflict,
	useResetKitKitInteractionSummary,
	useResetKitKitInteractionMetadata,
	useResetKitKitInteractionCreatedAt,
	useResetKitKitInteractionForward,
	useResetKitKitInteractionBackward,
	useExportKitKitInteractionId,
	useExportKitKitInteractionHash,
	useExportKitKitInteractionIndex,
	useExportKitKitInteractionKit,
	useExportKitKitInteractionKind,
	useExportKitKitInteractionActor,
	useExportKitKitInteractionSession,
	useExportKitKitInteractionTransaction,
	useExportKitKitInteractionCandidate,
	useExportKitKitInteractionChange,
	useExportKitKitInteractionConflict,
	useExportKitKitInteractionSummary,
	useExportKitKitInteractionMetadata,
	useExportKitKitInteractionCreatedAt,
	useExportKitKitInteractionForward,
	useExportKitKitInteractionBackward,
	useStartKitSessionKitInteractionId,
	useStartKitSessionKitInteractionHash,
	useStartKitSessionKitInteractionIndex,
	useStartKitSessionKitInteractionKit,
	useStartKitSessionKitInteractionKind,
	useStartKitSessionKitInteractionActor,
	useStartKitSessionKitInteractionSession,
	useStartKitSessionKitInteractionTransaction,
	useStartKitSessionKitInteractionCandidate,
	useStartKitSessionKitInteractionChange,
	useStartKitSessionKitInteractionConflict,
	useStartKitSessionKitInteractionSummary,
	useStartKitSessionKitInteractionMetadata,
	useStartKitSessionKitInteractionCreatedAt,
	useStartKitSessionKitInteractionForward,
	useStartKitSessionKitInteractionBackward,
	useHeartbeatKitSessionKitInteractionId,
	useHeartbeatKitSessionKitInteractionHash,
	useHeartbeatKitSessionKitInteractionIndex,
	useHeartbeatKitSessionKitInteractionKit,
	useHeartbeatKitSessionKitInteractionKind,
	useHeartbeatKitSessionKitInteractionActor,
	useHeartbeatKitSessionKitInteractionSession,
	useHeartbeatKitSessionKitInteractionTransaction,
	useHeartbeatKitSessionKitInteractionCandidate,
	useHeartbeatKitSessionKitInteractionChange,
	useHeartbeatKitSessionKitInteractionConflict,
	useHeartbeatKitSessionKitInteractionSummary,
	useHeartbeatKitSessionKitInteractionMetadata,
	useHeartbeatKitSessionKitInteractionCreatedAt,
	useHeartbeatKitSessionKitInteractionForward,
	useHeartbeatKitSessionKitInteractionBackward,
	useEndKitSessionKitInteractionId,
	useEndKitSessionKitInteractionHash,
	useEndKitSessionKitInteractionIndex,
	useEndKitSessionKitInteractionKit,
	useEndKitSessionKitInteractionKind,
	useEndKitSessionKitInteractionActor,
	useEndKitSessionKitInteractionSession,
	useEndKitSessionKitInteractionTransaction,
	useEndKitSessionKitInteractionCandidate,
	useEndKitSessionKitInteractionChange,
	useEndKitSessionKitInteractionConflict,
	useEndKitSessionKitInteractionSummary,
	useEndKitSessionKitInteractionMetadata,
	useEndKitSessionKitInteractionCreatedAt,
	useEndKitSessionKitInteractionForward,
	useEndKitSessionKitInteractionBackward,
	useReconnectKitSessionKitInteractionId,
	useReconnectKitSessionKitInteractionHash,
	useReconnectKitSessionKitInteractionIndex,
	useReconnectKitSessionKitInteractionKit,
	useReconnectKitSessionKitInteractionKind,
	useReconnectKitSessionKitInteractionActor,
	useReconnectKitSessionKitInteractionSession,
	useReconnectKitSessionKitInteractionTransaction,
	useReconnectKitSessionKitInteractionCandidate,
	useReconnectKitSessionKitInteractionChange,
	useReconnectKitSessionKitInteractionConflict,
	useReconnectKitSessionKitInteractionSummary,
	useReconnectKitSessionKitInteractionMetadata,
	useReconnectKitSessionKitInteractionCreatedAt,
	useReconnectKitSessionKitInteractionForward,
	useReconnectKitSessionKitInteractionBackward,
	useBeginKitTransactionKitInteractionId,
	useBeginKitTransactionKitInteractionHash,
	useBeginKitTransactionKitInteractionIndex,
	useBeginKitTransactionKitInteractionKit,
	useBeginKitTransactionKitInteractionKind,
	useBeginKitTransactionKitInteractionActor,
	useBeginKitTransactionKitInteractionSession,
	useBeginKitTransactionKitInteractionTransaction,
	useBeginKitTransactionKitInteractionCandidate,
	useBeginKitTransactionKitInteractionChange,
	useBeginKitTransactionKitInteractionConflict,
	useBeginKitTransactionKitInteractionSummary,
	useBeginKitTransactionKitInteractionMetadata,
	useBeginKitTransactionKitInteractionCreatedAt,
	useBeginKitTransactionKitInteractionForward,
	useBeginKitTransactionKitInteractionBackward,
	useFinalizeKitTransactionKitInteractionId,
	useFinalizeKitTransactionKitInteractionHash,
	useFinalizeKitTransactionKitInteractionIndex,
	useFinalizeKitTransactionKitInteractionKit,
	useFinalizeKitTransactionKitInteractionKind,
	useFinalizeKitTransactionKitInteractionActor,
	useFinalizeKitTransactionKitInteractionSession,
	useFinalizeKitTransactionKitInteractionTransaction,
	useFinalizeKitTransactionKitInteractionCandidate,
	useFinalizeKitTransactionKitInteractionChange,
	useFinalizeKitTransactionKitInteractionConflict,
	useFinalizeKitTransactionKitInteractionSummary,
	useFinalizeKitTransactionKitInteractionMetadata,
	useFinalizeKitTransactionKitInteractionCreatedAt,
	useFinalizeKitTransactionKitInteractionForward,
	useFinalizeKitTransactionKitInteractionBackward,
	useAbortKitTransactionKitInteractionId,
	useAbortKitTransactionKitInteractionHash,
	useAbortKitTransactionKitInteractionIndex,
	useAbortKitTransactionKitInteractionKit,
	useAbortKitTransactionKitInteractionKind,
	useAbortKitTransactionKitInteractionActor,
	useAbortKitTransactionKitInteractionSession,
	useAbortKitTransactionKitInteractionTransaction,
	useAbortKitTransactionKitInteractionCandidate,
	useAbortKitTransactionKitInteractionChange,
	useAbortKitTransactionKitInteractionConflict,
	useAbortKitTransactionKitInteractionSummary,
	useAbortKitTransactionKitInteractionMetadata,
	useAbortKitTransactionKitInteractionCreatedAt,
	useAbortKitTransactionKitInteractionForward,
	useAbortKitTransactionKitInteractionBackward,
	useTransactionStepKitInteractionId,
	useTransactionStepKitInteractionHash,
	useTransactionStepKitInteractionIndex,
	useTransactionStepKitInteractionKit,
	useTransactionStepKitInteractionKind,
	useTransactionStepKitInteractionActor,
	useTransactionStepKitInteractionSession,
	useTransactionStepKitInteractionTransaction,
	useTransactionStepKitInteractionCandidate,
	useTransactionStepKitInteractionChange,
	useTransactionStepKitInteractionConflict,
	useTransactionStepKitInteractionSummary,
	useTransactionStepKitInteractionMetadata,
	useTransactionStepKitInteractionCreatedAt,
	useTransactionStepKitInteractionForward,
	useTransactionStepKitInteractionBackward,
	useHistoryStepKitInteractionId,
	useHistoryStepKitInteractionHash,
	useHistoryStepKitInteractionIndex,
	useHistoryStepKitInteractionKit,
	useHistoryStepKitInteractionKind,
	useHistoryStepKitInteractionActor,
	useHistoryStepKitInteractionSession,
	useHistoryStepKitInteractionTransaction,
	useHistoryStepKitInteractionCandidate,
	useHistoryStepKitInteractionChange,
	useHistoryStepKitInteractionConflict,
	useHistoryStepKitInteractionSummary,
	useHistoryStepKitInteractionMetadata,
	useHistoryStepKitInteractionCreatedAt,
	useHistoryStepKitInteractionForward,
	useHistoryStepKitInteractionBackward,
	useVoteOnKitChangeCandidateKitInteractionId,
	useVoteOnKitChangeCandidateKitInteractionHash,
	useVoteOnKitChangeCandidateKitInteractionIndex,
	useVoteOnKitChangeCandidateKitInteractionKit,
	useVoteOnKitChangeCandidateKitInteractionKind,
	useVoteOnKitChangeCandidateKitInteractionActor,
	useVoteOnKitChangeCandidateKitInteractionSession,
	useVoteOnKitChangeCandidateKitInteractionTransaction,
	useVoteOnKitChangeCandidateKitInteractionCandidate,
	useVoteOnKitChangeCandidateKitInteractionChange,
	useVoteOnKitChangeCandidateKitInteractionConflict,
	useVoteOnKitChangeCandidateKitInteractionSummary,
	useVoteOnKitChangeCandidateKitInteractionMetadata,
	useVoteOnKitChangeCandidateKitInteractionCreatedAt,
	useVoteOnKitChangeCandidateKitInteractionForward,
	useVoteOnKitChangeCandidateKitInteractionBackward,
	useResolveKitConflictKitInteractionId,
	useResolveKitConflictKitInteractionHash,
	useResolveKitConflictKitInteractionIndex,
	useResolveKitConflictKitInteractionKit,
	useResolveKitConflictKitInteractionKind,
	useResolveKitConflictKitInteractionActor,
	useResolveKitConflictKitInteractionSession,
	useResolveKitConflictKitInteractionTransaction,
	useResolveKitConflictKitInteractionCandidate,
	useResolveKitConflictKitInteractionChange,
	useResolveKitConflictKitInteractionConflict,
	useResolveKitConflictKitInteractionSummary,
	useResolveKitConflictKitInteractionMetadata,
	useResolveKitConflictKitInteractionCreatedAt,
	useResolveKitConflictKitInteractionForward,
	useResolveKitConflictKitInteractionBackward,
	useKitInteractionPageHash,
	useKitInteractionPageNodes,
	useKitInteractionPagePageInfo,
	useKitInteractionPageTotalCount,
	useKitHistoryHash,
	useKitHistoryCanUndo,
	useKitHistoryCanRedo,
	useKitHistoryTotalCount,
	useKitHistoryHead,
	useKitStoreHash,
	useKitStoreKit,
	useKitStoreBackbone,
	useKitStoreSessions,
	useKitStoreTransactions,
	useKitStorePendingCandidates,
	useKitStoreActiveConflicts,
	useKitStoreValidation,
	useKitStoreHistory,
	useKitStoreBlockedByConflict,
	useKitStoreStrictMode,
	useKitArchiveExportHash,
	useKitArchiveExportFileName,
	useKitArchiveExportUrl,
	useKitArchiveExportExpiresAt,
	useKitMutationResultHash,
	useKitMutationResultAccepted,
	useKitMutationResultKind,
	useKitMutationResultSummary,
	useKitMutationResultStore,
	useKitMutationResultKit,
	useKitMutationResultSession,
	useKitMutationResultTransaction,
	useKitMutationResultCandidate,
	useKitMutationResultChange,
	useKitMutationResultHistoryEntry,
	useKitMutationResultConflict,
	useKitMutationResultValidation,
	useKitMutationResultExport,
	useKitCommandContextInputKitId,
	useKitCommandContextInputSessionId,
	useKitCommandContextInputTransactionId,
	useKitCommandContextInputOrigin,
	useKitCommandContextInputExpectedHash,
	useKitCommandContextInputStrictMode,
	useStartKitSessionInputKitId,
	useStartKitSessionInputActor,
	useStartKitSessionInputClient,
	useStartKitSessionInputStrictMode,
	useHeartbeatKitSessionInputKitId,
	useHeartbeatKitSessionInputSessionId,
	useHeartbeatKitSessionInputLastKnownHash,
	useEndKitSessionInputKitId,
	useEndKitSessionInputSessionId,
	useReconnectKitSessionInputKitId,
	useReconnectKitSessionInputSessionId,
	useReconnectKitSessionInputClient,
	useReconnectKitSessionInputLastKnownHash,
	useSetSessionSelectionCommandInputContext,
	useSetSessionSelectionCommandInputMode,
	useSetSessionSelectionCommandInputSelection,
	useBeginKitTransactionInputContext,
	useBeginKitTransactionInputLabel,
	useBeginKitTransactionInputParentTransactionId,
	useFinalizeKitTransactionInputContext,
	useFinalizeKitTransactionInputTransactionId,
	useAbortKitTransactionInputContext,
	useAbortKitTransactionInputTransactionId,
	useTransactionStepInputContext,
	useTransactionStepInputTransactionId,
	useHistoryStepInputContext,
	useHistoryStepInputSteps,
	useVoteOnKitChangeCandidateInputContext,
	useVoteOnKitChangeCandidateInputCandidateId,
	useVoteOnKitChangeCandidateInputState,
	useVoteOnKitChangeCandidateInputReason,
	useVoteOnKitChangeCandidateInputResolutionOptionId,
	useResolveKitConflictInputContext,
	useResolveKitConflictInputConflictId,
	useResolveKitConflictInputOptionId,
	useResolveKitConflictInputPayload,
	useCreateAuthorCommandInputContext,
	useCreateAuthorCommandInputAuthor,
	useUpdateAuthorCommandInputContext,
	useUpdateAuthorCommandInputId,
	useUpdateAuthorCommandInputPatch,
	useDeleteAuthorCommandInputContext,
	useDeleteAuthorCommandInputId,
	useCreateTypeCommandInputContext,
	useCreateTypeCommandInputType,
	useUpdateTypeCommandInputContext,
	useUpdateTypeCommandInputId,
	useUpdateTypeCommandInputPatch,
	useDeleteTypeCommandInputContext,
	useDeleteTypeCommandInputId,
	useCreateDesignCommandInputContext,
	useCreateDesignCommandInputDesign,
	useUpdateDesignCommandInputContext,
	useUpdateDesignCommandInputId,
	useUpdateDesignCommandInputPatch,
	useDeleteDesignCommandInputContext,
	useDeleteDesignCommandInputId,
	useCreateQualityCommandInputContext,
	useCreateQualityCommandInputQuality,
	useUpdateQualityCommandInputContext,
	useUpdateQualityCommandInputId,
	useUpdateQualityCommandInputPatch,
	useDeleteQualityCommandInputContext,
	useDeleteQualityCommandInputId,
	useCreatePortCommandInputContext,
	useCreatePortCommandInputPort,
	useUpdatePortCommandInputContext,
	useUpdatePortCommandInputId,
	useUpdatePortCommandInputPatch,
	useDeletePortCommandInputContext,
	useDeletePortCommandInputId,
	useCreateFamilyCommandInputContext,
	useCreateFamilyCommandInputFamily,
	useUpdateFamilyCommandInputContext,
	useUpdateFamilyCommandInputId,
	useUpdateFamilyCommandInputPatch,
	useDeleteFamilyCommandInputContext,
	useDeleteFamilyCommandInputId,
	useCreateTagCommandInputContext,
	useCreateTagCommandInputTag,
	useUpdateTagCommandInputContext,
	useUpdateTagCommandInputId,
	useUpdateTagCommandInputPatch,
	useDeleteTagCommandInputContext,
	useDeleteTagCommandInputId,
	useCreateConceptCommandInputContext,
	useCreateConceptCommandInputConcept,
	useUpdateConceptCommandInputContext,
	useUpdateConceptCommandInputId,
	useUpdateConceptCommandInputPatch,
	useDeleteConceptCommandInputContext,
	useDeleteConceptCommandInputId,
	useCreateFileCommandInputContext,
	useCreateFileCommandInputFile,
	useUpdateFileCommandInputContext,
	useUpdateFileCommandInputId,
	useUpdateFileCommandInputPatch,
	useDeleteFileCommandInputContext,
	useDeleteFileCommandInputId,
	useCreateFolderCommandInputContext,
	useCreateFolderCommandInputFolder,
	useUpdateFolderCommandInputContext,
	useUpdateFolderCommandInputId,
	useUpdateFolderCommandInputPatch,
	useDeleteFolderCommandInputContext,
	useDeleteFolderCommandInputId,
	useMoveArtifactToFolderCommandInputContext,
	useMoveArtifactToFolderCommandInputArtifactKind,
	useMoveArtifactToFolderCommandInputArtifactId,
	useMoveArtifactToFolderCommandInputFolderId,
	useCreatePieceCommandInputContext,
	useCreatePieceCommandInputDesignId,
	useCreatePieceCommandInputPiece,
	useCreatePiecesCommandInputContext,
	useCreatePiecesCommandInputDesignId,
	useCreatePiecesCommandInputPieces,
	usePieceUpdateInputId,
	usePieceUpdateInputPatch,
	useUpdatePieceCommandInputContext,
	useUpdatePieceCommandInputDesignId,
	useUpdatePieceCommandInputId,
	useUpdatePieceCommandInputPatch,
	useUpdatePiecesCommandInputContext,
	useUpdatePiecesCommandInputDesignId,
	useUpdatePiecesCommandInputUpdates,
	useDeletePieceCommandInputContext,
	useDeletePieceCommandInputDesignId,
	useDeletePieceCommandInputId,
	useDeletePiecesCommandInputContext,
	useDeletePiecesCommandInputDesignId,
	useDeletePiecesCommandInputIds,
	useCreateConnectionCommandInputContext,
	useCreateConnectionCommandInputDesignId,
	useCreateConnectionCommandInputConnection,
	useCreateConnectionsCommandInputContext,
	useCreateConnectionsCommandInputDesignId,
	useCreateConnectionsCommandInputConnections,
	useConnectionUpdateInputId,
	useConnectionUpdateInputPatch,
	useUpdateConnectionCommandInputContext,
	useUpdateConnectionCommandInputDesignId,
	useUpdateConnectionCommandInputId,
	useUpdateConnectionCommandInputPatch,
	useUpdateConnectionsCommandInputContext,
	useUpdateConnectionsCommandInputDesignId,
	useUpdateConnectionsCommandInputUpdates,
	useDeleteConnectionCommandInputContext,
	useDeleteConnectionCommandInputDesignId,
	useDeleteConnectionCommandInputId,
	useDeleteConnectionsCommandInputContext,
	useDeleteConnectionsCommandInputDesignId,
	useDeleteConnectionsCommandInputIds,
	useDeleteSelectionCommandInputContext,
	useDeleteSelectionCommandInputDesignId,
	useDeleteSelectionCommandInputPieceIds,
	useDeleteSelectionCommandInputConnectionIds,
	useFixPiecesCommandInputContext,
	useFixPiecesCommandInputDesignId,
	useFixPiecesCommandInputPieceIds,
	useClusterPiecesCommandInputContext,
	useClusterPiecesCommandInputDesignId,
	useClusterPiecesCommandInputPieceIds,
	useClusterPiecesCommandInputNewDesignName,
	useExpandDesignReferenceCommandInputContext,
	useExpandDesignReferenceCommandInputDesignId,
	useExpandDesignReferenceCommandInputReferencedDesignId,
	useFlattenDesignCommandInputContext,
	useFlattenDesignCommandInputDesignId,
	useDragPiecesCommandInputContext,
	useDragPiecesCommandInputDesignId,
	useDragPiecesCommandInputPieceIds,
	useDragPiecesCommandInputOffset,
	useMovePiecesVectorInputShift,
	useMovePiecesVectorInputGap,
	useMovePiecesVectorInputRise,
	useMovePiecesVectorInputRotation,
	useMovePiecesVectorInputTurn,
	useMovePiecesVectorInputTilt,
	useMovePiecesCommandInputContext,
	useMovePiecesCommandInputDesignId,
	useMovePiecesCommandInputPieceIds,
	useMovePiecesCommandInputVector,
	useCreateFixedPieceCommandInputContext,
	useCreateFixedPieceCommandInputDesignId,
	useCreateFixedPieceCommandInputPiece,
	useCreateConnectedPieceCommandInputContext,
	useCreateConnectedPieceCommandInputDesignId,
	useCreateConnectedPieceCommandInputPiece,
	useCreateConnectedPieceCommandInputConnection,
	useCreateHangingPiecesCommandInputContext,
	useCreateHangingPiecesCommandInputDesignId,
	useCreateHangingPiecesCommandInputPieces,
	useCreateHangingPiecesCommandInputParentPieceId,
	useCreateHangingPiecesCommandInputParentDesignPieceId,
	useCreateHangingPiecesCommandInputParentConnectorId,
	useCreateHangingPiecesCommandInputConnectionTemplate,
	useChangePieceTypeCommandInputContext,
	useChangePieceTypeCommandInputDesignId,
	useChangePieceTypeCommandInputPieceId,
	useChangePieceTypeCommandInputTypeId,
	useChangePiecesTypeCommandInputContext,
	useChangePiecesTypeCommandInputDesignId,
	useChangePiecesTypeCommandInputPieceIds,
	useChangePiecesTypeCommandInputTypeId,
	usePasteDesignSelectionCommandInputContext,
	usePasteDesignSelectionCommandInputDesignId,
	usePasteDesignSelectionCommandInputPayload,
	usePasteDesignSelectionCommandInputOffset,
	useImportKitCommandInputContext,
	useImportKitCommandInputSourceUrl,
	useImportKitCommandInputArchiveBase64,
	useResetKitCommandInputContext,
	useResetKitCommandInputSourceUrl,
	useResetKitCommandInputArchiveBase64,
	useResetKitCommandInputKit,
	useExportKitCommandInputContext,
	useQueryKitCommandCatalog,
	useKitStoreEventHash,
	useKitStoreEventKind,
	useKitStoreEventStore,
	useKitStoreEventInteraction,
	useKitStoreEventChange,
	useKitStoreEventCandidate,
	useKitStoreEventConflict,
	useKitStoreEventSession,
	useKitStoreEventTransaction,
});

export function useSchemaHook(hookName: string, guidValue?: string): SchemaHookTriad<any> {
	const hook = (schemaHooks)[hookName];
	if (typeof hook !== "function") {
		return [undefined, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
	}
	return hook(guidValue);
}

// #endregion ⚛️Direct Domain Exports

// #region ⚛️Embedded tests
const shouldRunReactEmbeddedTests =
	(typeof process !== "undefined" && process.env.SEMIO_REACT_RUN_EMBEDDED_TESTS === "1") ||
	(typeof (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ !== "undefined" &&
		(globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ === true);

if (shouldRunReactEmbeddedTests) {
	const { describe, expect, it } = await import("vitest");
	const { render, waitFor } = await import("@testing-library/react");
	const { InMemoryKitStore, asKitInstance } = await import("@semio/js");

	describe("pipeline hooks", () => {
		it("useKitName rejects empty required name via kit client", async () => {
			const kit = asKitInstance({
				guid: "k1",
				name: "K",
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				designs: [
					{
						guid: "d1",
						name: "D",
						createdAt: new Date().toISOString(),
						updatedAt: new Date().toISOString(),
						pieces: [{ guid: "p1", name: "N" }],
					},
				],
			});
			const store = new InMemoryKitStore(kit);
			let setName: ((v: any) => Promise<any>) | undefined;
			let lastStatus: WriteStatus | undefined;

			function Probe() {
				const triad = useKitName();
				setName = triad[1];
				lastStatus = triad[2];
				return null;
			}

			render(React.createElement(KitProvider, { store }, React.createElement(Probe)));

			await waitFor(() => expect(setName).toBeDefined());
			const r = await setName!("");
			expect(r.ok).toBe(false);
			await waitFor(() => expect(lastStatus?.kind).toBe("error"));
		});
	});

	describe("KitRegistry + useOptimistic", () => {
		it("registry open/close refcounts and useOptimistic keeps draft until commit", async () => {
			const kit = asKitInstance({
				guid: "k1",
				name: "K",
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
			});
			const store = new InMemoryKitStore(kit);
			let reg: ReturnType<typeof useKitRegistry> | null = null;
			function RegProbe() {
				reg = useKitRegistry();
				return null;
			}
			render(
				React.createElement(
					KitRegistryProvider,
					null,
					React.createElement(RegProbe),
				),
			);
			await waitFor(() => expect(reg).not.toBeNull());
			await reg!.open("k1", { store });
			expect(reg!.get("k1")?.refs).toBe(1);
			await reg!.open("k1", { store });
			expect(reg!.get("k1")?.refs).toBe(2);
			reg!.close("k1");
			expect(reg!.get("k1")?.refs).toBe(1);
			reg!.close("k1");
			expect(reg!.get("k1")).toBeUndefined();

			const triad: HookTriad<string> = [
				"hello",
				async () => ({ ok: true } as const),
				{ kind: "idle", pending: 0 },
			];
			let opt: ReturnType<typeof useOptimistic<string>> | null = null;
			function OptProbe() {
				opt = useOptimistic(triad);
				return null;
			}
			render(React.createElement(OptProbe));
			await waitFor(() => expect(opt).not.toBeNull());
			expect(opt!.dirty).toBe(false);
		});
	});

	describe("KitStoreClient stub RPC hooks", () => {
		it("useClusterPieces forwards failures to useSetErrors", async () => {
			const kit = asKitInstance({
				guid: "k1",
				name: "K",
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				designs: [
					{
						guid: "d1",
						name: "D",
						createdAt: new Date().toISOString(),
						updatedAt: new Date().toISOString(),
						pieces: [{ guid: "p1", name: "P" }],
						connections: [],
					},
				],
			});
			const store = new InMemoryKitStore(kit);
			const stub: import("@semio/js").KitStoreClient = {
				getDto: () => store.getSnapshot().kit.toJSON(),
				getSnapshot: async () => store.getSnapshot().kit.toJSON(),
				setField: async () => ({ ok: true } as const),
				addChild: async () => ({ ok: true } as const),
				removeChild: async () => ({ ok: true } as const),
				applyDesignDiff: async () => ({ ok: true } as const),
				clusterPieces: async () => ({ ok: false, error: { kind: "InvalidValue", message: "stub-cluster" } }),
				dragPieces: async () => ({ ok: true } as const),
				movePieces: async () => ({ ok: true } as const),
				fixPieces: async () => ({ ok: true } as const),
				flattenDesign: async () => ({ ok: true } as const),
				expandDesign: async () => ({ ok: true } as const),
				deleteConnection: async () => ({ ok: true } as const),
				changePieceType: async () => ({ ok: true } as const),
				pasteDesignSelection: async () => ({ ok: true } as const),
				createHangingPieces: async () => ({ ok: true } as const),
				createConnectedPiece: async () => ({ ok: true } as const),
				createFixedPiece: async () => ({ ok: true } as const),
				getPiecesMetadata: async () => ({}),
				getPieces: async () => [],
				getConnections: async () => [],
				getDesigns: async () => [],
				getTypes: async () => [],
				getAuthors: async () => [],
				getKitMetadata: async () => ({}),
				subscribe: () => () => {},
				dispose: () => {},
			};
			let seen: SetError[] = [];
			function Probe() {
				const { run } = useClusterPieces();
				seen = useSetErrors();
				const ran = React.useRef(false);
				React.useEffect(() => {
					if (ran.current) return;
					ran.current = true;
					void run("d1", ["p1"], "C");
				}, [run]);
				return null;
			}
			render(React.createElement(KitProvider, { store, kitClient: stub }, React.createElement(Probe)));
			await waitFor(() => expect(seen.length).toBeGreaterThan(0));
			expect(seen[0]?.message).toContain("stub-cluster");
		});
	});
}
// #endregion ⚛️Embedded tests
