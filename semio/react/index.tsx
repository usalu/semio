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
	id,
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
	id?: string;
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
	id?: string;
	path: Array<string | number>;
	value: any;
};

type IndexedSchemaState = {
	plain: any;
	kit: KitImpl;
	kitId?: string;
	byId: Map<string, IndexedSchemaReference[]>;
	byType: Map<string, IndexedSchemaReference[]>;
};

type SchemaScope = {
	typeName: string;
	id?: string;
	path: Array<string | number>;
};

export type KitRuntimeContextValue = {
	store: KitStore;
	snapshot: KitStoreSnapshot;
	state: IndexedSchemaState;
	recentEvents: SchemaPropertyEvent[];
	recentSetRejections: SetError[];
	pushSetRejection: (e: SetError) => void;
	canWrite: boolean;
	/** Active kit id: {@link KitProvider} `kitId` when set, otherwise `snapshot.kit.id`. */
	kitId?: string;
	kitClient: KitStoreClient | null;
	setFieldValue: (typeName: string, fieldName: string, next: SetStateAction<any>, id?: string, scope?: SchemaScope | null) => void;
	setObjectValue: (typeName: string, next: SetStateAction<any>, id?: string, scope?: SchemaScope | null) => void;
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

function kitIdFromRuntime(runtime: KitRuntimeContextValue): string | null {
	const g = runtime.kitId ?? (runtime.snapshot as { kit?: { id?: string } }).kit?.id;
	return g != null && g !== "" ? String(g) : null;
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
	if (fieldName === "id") return "id";
	if (typeName === "Kit" && fieldName === "release") return "version";
	return fieldName;
}

function getSchemaFieldName(typeName: string, dataKey: string): string {
	if (dataKey === "id") return "id";
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
	const byId = new Map<string, IndexedSchemaReference[]>();
	const byType = new Map<string, IndexedSchemaReference[]>();

	function push(ref: IndexedSchemaReference): void {
		if (ref.id) {
			const existing = byId.get(ref.id) ?? [];
			existing.push(ref);
			byId.set(ref.id, existing);
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
		const idValue = typeof value.id === "string" ? value.id : undefined;
		push({ typeName: resolvedTypeName, id: idValue, path, value });
		for (const [key, entry] of Object.entries(value)) {
			walk(entry, [...path, key], inferTypeName(resolvedTypeName, key));
		}
	}

	walk(root, [], "Kit");

	return {
		plain: root,
		kit: asKitInstance(root),
		kitId: root?.id,
		byId,
		byType,
	};
}

function collectIds(value: any, target: Set<string>): void {
	if (value == null) return;
	if (Array.isArray(value)) {
		for (const entry of value) collectIds(entry, target);
		return;
	}
	if (typeof value !== "object") return;
	if (typeof value.id === "string") target.add(value.id);
	for (const entry of Object.values(value)) collectIds(entry, target);
}

function resolveReference(index: IndexedSchemaState, typeName: string, id?: string, scope?: SchemaScope | null): IndexedSchemaReference | undefined {
	if (typeName === "Kit") return index.byType.get("Kit")?.[0];
	if (id) {
		const matches = index.byId.get(id) ?? [];
		return matches.find((entry) => entry.typeName === typeName) ?? matches[0];
	}
	if (scope && scope.typeName === typeName) {
		return { typeName, id: scope.id, path: scope.path, value: getByPath(index.plain, scope.path) };
	}
	const typeMatches = index.byType.get(typeName) ?? [];
	if (typeMatches.length === 1) return typeMatches[0];
	return undefined;
}

function findLivePiece(kit: KitImpl, pieceId: string): { piece: Piece; design: Design } | undefined {
	for (const design of kit.designs ?? []) {
		const piece = design.pieces?.find((entry) => entry.id === pieceId);
		if (piece) return { piece, design };
	}
	return undefined;
}

function findLiveConnection(kit: KitImpl, connectionId: string): { connection: any; design: Design } | undefined {
	for (const design of kit.designs ?? []) {
		const connection = design._connections?.find((entry) => entry.id === connectionId);
		if (connection) return { connection, design };
	}
	return undefined;
}

function findLiveEntity(kit: KitImpl, typeName: string, id?: string): any {
	if (typeName === "Kit") return kit;
	if (!id) return undefined;
	if (typeName === "Piece") return findLivePiece(kit, id)?.piece;
	if (typeName === "Connection") return findLiveConnection(kit, id)?.connection;
	if (typeName === "Type") return kit.findType(id);
	if (typeName === "Design") return kit.findDesign(id);
	if (typeName === "Port") return kit.ports?.find((entry) => entry.id === id);
	if (typeName === "Quality") return kit.qualities?.find((entry) => entry.id === id);
	if (typeName === "File") return kit.files?.find((entry) => entry.id === id);
	if (typeName === "Folder") return kit.folders?.find((entry) => entry.id === id);
	if (typeName === "Author") return kit.authors?.find((entry) => entry.id === id);
	if (typeName === "Tag") return kit.tags?.find((entry) => entry.id === id);
	if (typeName === "Concept") return kit.concepts?.find((entry) => entry.id === id);
	if (typeName === "Family") return kit.families?.find((entry) => entry.id === id);
	if (typeName === "Representation") {
		for (const entry of kit.types ?? []) {
			const match = entry.representations?.find((representation) => representation.id === id);
			if (match) return match;
		}
	}
	if (typeName === "Connector") {
		for (const entry of kit.types ?? []) {
			const match = entry.connectors?.find((connector) => connector.id === id);
			if (match) return match;
		}
	}
	if (typeName === "Benchmark") {
		for (const entry of kit.qualities ?? []) {
			const match = entry.benchmarks?.find((benchmark) => benchmark.id === id);
			if (match) return match;
		}
	}
	return undefined;
}

function readCustomFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string): any {
	if (typeName === "Kit" && fieldName === "release") return (state.kit as any).version;
	if (typeName === "Piece") {
		const found = id ? findLivePiece(state.kit, id) : undefined;
		if (!found) return undefined;
		const { piece, design } = found;
		if (fieldName === "kind") return piece.wireDesignAsPieceId() ? "DESIGN" : piece.wireTypeId() ? "TYPE" : undefined;
		if (fieldName === "flatPlane") return piece.flatPlane();
		if (fieldName === "flatCenter") return piece.flatCenter();
		if (fieldName === "parentPiece") {
			try {
				return state.kit.findParentPieceInDesign(design.id, piece.id);
			} catch {
				return undefined;
			}
		}
		if (fieldName === "parentConnection") {
			try {
				return state.kit.findParentConnectionForPieceInDesign(design.id, piece.id);
			} catch {
				return undefined;
			}
		}
		if (fieldName === "childPieces") {
			try {
				return state.kit.findChildrenPiecesInDesign(design.id, piece.id);
			} catch {
				return [];
			}
		}
		if (fieldName === "childConnections") {
			try {
				const metadata = state.kit.piecesMetadataFor(design.id);
				if (!metadata.ok || !metadata.diff) return [];
				return (design._connections ?? []).filter((connection) => {
					try {
						const connectedId = connection.connected.wirePieceId().id;
						const connectingId = connection.connecting.wirePieceId().id;
						if (connectedId === piece.id) return metadata.diff.get(connectingId)?.parentPieceId === piece.id;
						if (connectingId === piece.id) return metadata.diff.get(connectedId)?.parentPieceId === piece.id;
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
				return nestedDesign.getDesignFamily().filter((entry) => entry.id !== nestedDesign.id);
			} catch {
				return [];
			}
		}
		if (fieldName === "alternatives") {
			return [
				...((piece.alternativeTypes() ?? []).map((entry) => ({ type: entry, design: undefined }))),
				...((readCustomFieldValue(state, typeName, "alternativeDesigns", id) ?? []).map((entry: any) => ({ type: undefined, design: entry }))),
			];
		}
	}
	if (typeName === "Connection") {
		const found = id ? findLiveConnection(state.kit, id) : undefined;
		if (!found) return undefined;
		const { connection } = found;
		if (fieldName === "childPiece") return connection.connecting?.piece;
		if (fieldName === "parentPiece") return connection.connected?.piece;
		if (fieldName === "childConnector") return connection.connecting?.connector;
		if (fieldName === "parentConnector") return connection.connected?.connector;
	}
	if (typeName === "Type" && fieldName === "fixedPieces") {
		const liveType = id ? state.kit.findType(id) : undefined;
		if (!liveType) return [];
		const pieces: Piece[] = [];
		for (const design of state.kit.designs ?? []) {
			for (const piece of design.pieces ?? []) {
				if (piece.wireTypeId()?.id === liveType.id) pieces.push(piece);
			}
		}
		return pieces;
	}
	return undefined;
}

function readSchemaFieldValue(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string, scope?: SchemaScope | null): any {
	const custom = readCustomFieldValue(state, typeName, fieldName, id);
	if (custom !== undefined) return custom;
	const ref = resolveReference(state, typeName, id, scope);
	if (!ref) return undefined;
	const key = getFieldDataKey(typeName, fieldName);
	return ref.value?.[key];
}

function isWritableField(state: IndexedSchemaState, typeName: string, fieldName: string, id?: string, scope?: SchemaScope | null): boolean {
	if (NEVER_WRITABLE_FIELDS.has(fieldName)) return false;
	const ref = resolveReference(state, typeName, id, scope);
	if (!ref) return false;
	const key = getFieldDataKey(typeName, fieldName);
	if (fieldName === "hash") return false;
	return ref.value != null && (Object.prototype.hasOwnProperty.call(ref.value, key) || ref.value[key] !== undefined);
}

function normalizeNextValue(current: any, fieldName: string, next: any): any {
	if (typeof next === "string" && current && typeof current === "object" && "id" in current) {
		return { id: next };
	}
	if ((fieldName === "type" || fieldName === "design" || fieldName === "piece" || fieldName === "designPiece" || fieldName === "connector") && typeof next === "string") {
		return { id: next };
	}
	return next;
}

function nextValueFromAction<T>(current: T, next: SetStateAction<T>): T {
	return typeof next === "function" ? (next as (value: T) => T)(current) : next;
}

function normalizeStateInput(input: KitStoreSnapshot | KitLike | IndexedSchemaState): IndexedSchemaState {
	if ((input as IndexedSchemaState).byId instanceof Map) return input as IndexedSchemaState;
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
	const dirtyIds = new Set<string>();
	const allIds = new Set<string>([...(previous.byId.keys() ?? []), ...(next.byId.keys() ?? [])]);

	for (const idValue of allIds) {
		const previousRef = (previous.byId.get(idValue) ?? [])[0];
		const nextRef = (next.byId.get(idValue) ?? [])[0];
		if (!deepEqual(previousRef?.value, nextRef?.value)) {
			dirtyIds.add(idValue);
			collectIds(previousRef?.value, dirtyIds);
			collectIds(nextRef?.value, dirtyIds);
		}
	}

	const events: SchemaPropertyEvent[] = [];
	for (const idValue of dirtyIds) {
		const previousRef = (previous.byId.get(idValue) ?? [])[0];
		const nextRef = (next.byId.get(idValue) ?? [])[0];
		const typeName = nextRef?.typeName ?? previousRef?.typeName;
		if (!typeName) continue;
		for (const fieldName of collectChangedObjectFields(typeName, previousRef?.value, nextRef?.value)) {
			const previousValue = readSchemaFieldValue(previous, typeName, fieldName, idValue);
			const nextValue = readSchemaFieldValue(next, typeName, fieldName, idValue);
			if (!deepEqual(previousValue, nextValue)) {
				events.push({ key: `${typeName}.${fieldName}`, typeName, fieldName, id: idValue, previous: previousValue, current: nextValue });
			}
		}
	}

	if (!deepEqual(previous.plain, next.plain) && next.kitId) {
		for (const fieldName of collectChangedObjectFields("Kit", previous.plain, next.plain)) {
			const previousValue = readSchemaFieldValue(previous, "Kit", fieldName, previous.kitId);
			const nextValue = readSchemaFieldValue(next, "Kit", fieldName, next.kitId);
			if (!deepEqual(previousValue, nextValue)) {
				events.push({ key: `Kit.${fieldName}`, typeName: "Kit", fieldName, id: next.kitId, previous: previousValue, current: nextValue });
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
		const seed = resolvedBackbone.initialKit ?? initialKit ?? { id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() };
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
	open: (id: string, init: { backbone?: KitProviderBackbone; initialKit?: KitLike; store?: KitStore }) => Promise<void>;
	close: (id: string) => void;
	get: (id: string) => KitRegistryEntry | undefined;
	list: () => string[];
	status: (id: string) => "idle" | "loading" | "ready" | "error";
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
			async open(id, init) {
				const cur = rowsRef.current.get(id);
				if (cur) {
					cur.refs += 1;
					bump();
					return;
				}
				loadingRef.current.add(id);
				errRef.current.delete(id);
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
					rowsRef.current.set(id, { store, kitClient, refs: 1, unsub });
				} catch (e) {
					errRef.current.set(id, e instanceof Error ? e : new Error(String(e)));
				} finally {
					loadingRef.current.delete(id);
					bump();
				}
			},
			close(id) {
				const row = rowsRef.current.get(id);
				if (!row) return;
				row.refs -= 1;
				if (row.refs <= 0) {
					row.unsub();
					row.kitClient.dispose();
					rowsRef.current.delete(id);
				}
				bump();
			},
			get(id) {
				const row = rowsRef.current.get(id);
				if (!row) return undefined;
				return { store: row.store, kitClient: row.kitClient, refs: row.refs };
			},
			list() {
				return Array.from(rowsRef.current.keys());
			},
			status(id) {
				if (loadingRef.current.has(id)) return "loading";
				if (errRef.current.has(id)) return "error";
				if (rowsRef.current.has(id)) return "ready";
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

/** Like {@link useKitRuntime} but returns `null` outside {@link KitProvider} (no throw). */
export function useKitRuntimeSafe(): KitRuntimeContextValue | null {
	return React.useContext(KitRuntimeContext);
}

/** Returns the WASM worker {@link KitStoreClient} when inside {@link KitProvider}, or `null`. */
export function useKitStoreClient(): KitStoreClient | null {
	const runtime = useKitRuntime();
	return runtime.kitClient;
}

/** Active kit id from {@link KitProvider} runtime, or `undefined` outside a provider. */
export function useActiveKitId(): string | undefined {
	return React.useContext(KitRuntimeContext)?.kitId;
}

export type KitProviderProps = {
	store?: KitStore;
	/** When set with <KitRegistryProvider>, uses the registry entry for this kit (warm WASM worker). */
	kitId?: string;
	/** When provided (e.g. from registry), skips creating a new worker client. */
	kitClient?: KitStoreClient | null;
	backbone?: KitProviderBackbone;
	initialKit?: KitLike;
	children: ReactNode;
	fallback?: ReactNode;
};

export function KitProvider({
	store: externalStore,
	kitId: kitIdProp,
	kitClient: kitClientProp,
	backbone,
	initialKit,
	children,
	fallback = null,
}: KitProviderProps): React.ReactElement | null {
	const registry = React.useContext(KitRegistryContext);
	if (kitIdProp && !registry) {
		throw new Error("semio/react: <KitProvider kitId={...}> must be wrapped in <KitRegistryProvider>.");
	}
	const registryEntry = kitIdProp && registry ? registry.get(kitIdProp) : undefined;

	const [internalStore, setInternalStore] = React.useState<KitStore | null>(externalStore ?? null);
	const [kitClientState, setKitClientState] = React.useState<KitStoreClient | null>(kitClientProp ?? null);

	React.useEffect(() => {
		if (kitIdProp) return;
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
	}, [kitIdProp, externalStore, backbone, initialKit]);

	React.useEffect(() => {
		if (kitIdProp) return;
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
	}, [kitIdProp, externalStore, internalStore, kitClientProp]);

	const store = kitIdProp && registryEntry ? registryEntry.store : (externalStore ?? internalStore);
	const kitClient = kitIdProp && registryEntry ? registryEntry.kitClient : (kitClientProp ?? kitClientState);

	if (kitIdProp && registry && !registryEntry) return React.createElement(React.Fragment, null, fallback);
	if (!store) return React.createElement(React.Fragment, null, fallback);

	React.useEffect(() => {
		if (kitIdProp) return;
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
	}, [kitClient, store, kitIdProp]);

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

	const setFieldValue = React.useCallback((typeName: string, fieldName: string, next: SetStateAction<any>, idValue?: string, scope?: SchemaScope | null) => {
		const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
		if (!isWritableField(currentState, typeName, fieldName, idValue, scope)) return;
		const ref = resolveReference(currentState, typeName, idValue, scope);
		if (!ref) return;
		const key = getFieldDataKey(typeName, fieldName);
		const clone = deepClone(currentState.plain);
		const currentObject = getByPath(clone, ref.path);
		const currentValue = currentObject?.[key];
		currentObject[key] = normalizeNextValue(currentValue, fieldName, nextValueFromAction(currentValue, next));
		store.replace(asKitInstance(clone));
	}, [store]);

	const setObjectValue = React.useCallback((typeName: string, next: SetStateAction<any>, idValue?: string, scope?: SchemaScope | null) => {
		const currentState = scanSchemaState(store.getSnapshot().kit.toJSON());
		const ref = resolveReference(currentState, typeName, idValue, scope);
		if (!ref) return;
		const clone = deepClone(currentState.plain);
		const currentValue = getByPath(clone, ref.path);
		setByPath(clone, ref.path, nextValueFromAction(currentValue, next));
		store.replace(asKitInstance(clone));
	}, [store]);

	const activeKitId = kitIdProp ?? snapshot.kit?.id;

	const value = React.useMemo<KitRuntimeContextValue>(() => ({
		store,
		snapshot,
		state,
		recentEvents,
		recentSetRejections,
		pushSetRejection,
		canWrite: !snapshot.sync.readonly,
		kitId: activeKitId,
		kitClient,
		setFieldValue,
		setObjectValue,
	}), [store, snapshot, state, recentEvents, recentSetRejections, pushSetRejection, activeKitId, kitClient, setFieldValue, setObjectValue]);

	return React.createElement(KitRuntimeContext.Provider, { value }, children);
}

function useEntityScope(typeName: string, idValue?: string): SchemaScope {
	const runtime = useKitRuntime();
	const parentScope = React.useContext(SchemaScopeContext);
	const ref = resolveReference(runtime.state, typeName, idValue, parentScope);
	return ref ? { typeName, id: ref.id, path: ref.path } : { typeName, id: idValue, path: [] };
}

export function PieceProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Piece", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function TypeProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Type", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function DesignProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Design", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectionProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Connection", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PortProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Port", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function QualityProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Quality", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FileProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("File", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FolderProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Folder", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AuthorProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Author", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function TagProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Tag", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConceptProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Concept", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function FamilyProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Family", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function RepresentationProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Representation", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function ConnectorProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Connector", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function BenchmarkProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Benchmark", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function LayerProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Layer", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function GroupProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Group", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function StatProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Stat", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function PropProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Prop", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

export function AttributeProvider({ id: idValue, children }: { id?: string; children: ReactNode }): React.ReactElement {
	const scope = useEntityScope("Attribute", idValue);
	return React.createElement(SchemaScopeContext.Provider, { value: scope }, children);
}

// #endregion ⚛️Context

// #region ⚛️Core Hooks

function resolveRustFieldTarget(
	runtime: KitRuntimeContextValue,
	typeName: string,
	fieldName: string,
	idValue: string | undefined,
	scope: SchemaScope | null,
): { kind: string; id: string; field: string } | null {
	if (!runtime.kitClient) return null;
	if (typeName === "Piece" && (fieldName === "name" || fieldName === "color")) {
		const g = idValue ?? scope?.id;
		if (!g) return null;
		return { kind: "Piece", id: g, field: fieldName };
	}
	if (typeName === "Kit" && fieldName === "name") {
		return { kind: "Kit", id: runtime.snapshot.kit.id, field: "name" };
	}
	if (typeName === "Design" && fieldName === "name") {
		const g = idValue ?? scope?.id;
		if (!g) return null;
		return { kind: "Design", id: g, field: "name" };
	}
	if (typeName === "Type" && fieldName === "name") {
		const g = idValue ?? scope?.id;
		if (!g) return null;
		return { kind: "Type", id: g, field: "name" };
	}
	return null;
}

export function useSchemaEvents(filter?: Partial<Pick<SchemaPropertyEvent, "typeName" | "fieldName" | "id" | "key">>): SchemaPropertyEvent[] {
	const runtime = useKitRuntime();
	return React.useMemo(() => {
		if (!filter) return runtime.recentEvents;
		return runtime.recentEvents.filter((event) => {
			if (filter.typeName && event.typeName !== filter.typeName) return false;
			if (filter.fieldName && event.fieldName !== filter.fieldName) return false;
			if (filter.id && event.id !== filter.id) return false;
			if (filter.key && event.key !== filter.key) return false;
			return true;
		});
	}, [runtime.recentEvents, filter]);
}

export function useSetErrors(filter?: Partial<{ entityKind: string; id: string }>): SetError[] {
	const runtime = useKitRuntime();
	return React.useMemo(() => {
		if (!filter) return runtime.recentSetRejections;
		return runtime.recentSetRejections.filter((e) => {
			if (filter.entityKind && e.entity?.kind !== filter.entityKind) return false;
			if (filter.id && e.entity?.id !== filter.id) return false;
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

/**
 * Local draft over a {@link HookTriad}: mirror server value, edit locally, {@link commit} async-writes;
 * on rejection the draft is kept; {@link status} comes from the triad for {@link useWriteIndicator}.
 */
export function useDraft<T>(triad: HookTriad<T>): {
	value: T;
	setDraft: (next: SetStateAction<T>) => void;
	commit: () => Promise<SetResult>;
	reset: () => void;
	status: WriteStatus;
	error: SetError | undefined;
} {
	const [server, setServer, status] = triad;
	const [draft, setDraft] = React.useState<T | undefined>(undefined);
	const value = (draft !== undefined ? draft : server) as T;
	const setDraftFn = React.useCallback(
		(next: SetStateAction<T>) => {
			setDraft((d) => {
				const base = (d !== undefined ? d : server) as T;
				return typeof next === "function" ? (next as (p: T) => T)(base) : next;
			});
		},
		[server],
	);
	const commit = React.useCallback(async () => {
		if (draft === undefined) return { ok: true } as const;
		const r = await setServer(draft);
		if (r.ok) setDraft(undefined);
		return r;
	}, [draft, setServer]);
	const reset = React.useCallback(() => setDraft(undefined), []);
	const error = status.kind === "error" ? status.lastError : undefined;
	return { value, setDraft: setDraftFn, commit, reset, status, error };
}

// #region 🎛️KitStoreClient command hooks (WASM / worker RPCs)

export function useClusterPieces(): {
	run: (designId: string, pieceIds: string[], clusterName: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceIds: string[], clusterName: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.clusterPieces(designId, pieceIds, clusterName);
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
	run: (designId: string, pieceIds: string[], du: number, dv: number) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceIds: string[], du: number, dv: number) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.dragPieces(designId, pieceIds, du, dv);
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
	run: (designId: string, pieceIds: string[], gap: number, shift: number, rise: number) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceIds: string[], gap: number, shift: number, rise: number) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.movePieces(designId, pieceIds, gap, shift, rise);
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
	run: (designId: string, pieceIds: string[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceIds: string[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.fixPieces(designId, pieceIds);
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

export function useFlattenDesign(): { run: (designId: string) => Promise<SetResult>; status: WriteStatus } {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.flattenDesign(designId);
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
	run: (parentDesignId: string, nestedDesignId: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (parentDesignId: string, nestedDesignId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.expandDesign(parentDesignId, nestedDesignId);
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
	run: (designId: string, connectionId: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, connectionId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.deleteConnection(designId, connectionId);
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
	run: (designId: string, pieceId: string, newTypeId: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceId: string, newTypeId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.changePieceType(designId, pieceId, newTypeId);
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

export function useUndo(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(async () => {
		if (!runtime.kitClient || !runtime.canWrite) {
			const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
			setStatus({ kind: "error", pending: 0, lastError: e });
			return { ok: false, error: e } as const;
		}
		setStatus({ kind: "pending", pending: 1 });
		const r = await runtime.kitClient.undo();
		if (!r.ok) {
			runtime.pushSetRejection(r.error);
			setStatus({ kind: "error", pending: 0, lastError: r.error });
			return r;
		}
		setStatus({ kind: "idle", pending: 0 });
		return r;
	}, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
	return { run, status };
}

export function useRedo(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(async () => {
		if (!runtime.kitClient || !runtime.canWrite) {
			const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
			setStatus({ kind: "error", pending: 0, lastError: e });
			return { ok: false, error: e } as const;
		}
		setStatus({ kind: "pending", pending: 1 });
		const r = await runtime.kitClient.redo();
		if (!r.ok) {
			runtime.pushSetRejection(r.error);
			setStatus({ kind: "error", pending: 0, lastError: r.error });
			return r;
		}
		setStatus({ kind: "idle", pending: 0 });
		return r;
	}, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
	return { run, status };
}

export function useCanUndo(): SchemaHookTriad<boolean> {
	const runtime = useKitRuntime();
	const [v, setV] = React.useState(false);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient) {
			setV(false);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const b = await runtime.kitClient!.canUndo();
				if (!cancelled) setV(!!b);
			} catch {
				if (!cancelled) setV(false);
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
	const st: WriteStatus =
		!runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [v, noopAsyncSet, st] as const;
}

export function useCanRedo(): SchemaHookTriad<boolean> {
	const runtime = useKitRuntime();
	const [v, setV] = React.useState(false);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient) {
			setV(false);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const b = await runtime.kitClient!.canRedo();
				if (!cancelled) setV(!!b);
			} catch {
				if (!cancelled) setV(false);
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
	const st: WriteStatus =
		!runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [v, noopAsyncSet, st] as const;
}

export function useTransaction(): {
	begin: () => Promise<SetResult>;
	commit: () => Promise<SetResult>;
	abort: () => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const begin = React.useCallback(async () => {
		if (!runtime.kitClient || !runtime.canWrite) {
			const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
			setStatus({ kind: "error", pending: 0, lastError: e });
			return { ok: false, error: e } as const;
		}
		setStatus({ kind: "pending", pending: 1 });
		const r = await runtime.kitClient.beginTx();
		if (!r.ok) {
			runtime.pushSetRejection(r.error);
			setStatus({ kind: "error", pending: 0, lastError: r.error });
			return r;
		}
		setStatus({ kind: "idle", pending: 0 });
		return r;
	}, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
	const commit = React.useCallback(async () => {
		if (!runtime.kitClient || !runtime.canWrite) {
			const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
			setStatus({ kind: "error", pending: 0, lastError: e });
			return { ok: false, error: e } as const;
		}
		setStatus({ kind: "pending", pending: 1 });
		const r = await runtime.kitClient.commitTx();
		if (!r.ok) {
			runtime.pushSetRejection(r.error);
			setStatus({ kind: "error", pending: 0, lastError: r.error });
			return r;
		}
		setStatus({ kind: "idle", pending: 0 });
		return r;
	}, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
	const abort = React.useCallback(async () => {
		if (!runtime.kitClient || !runtime.canWrite) {
			const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
			setStatus({ kind: "error", pending: 0, lastError: e });
			return { ok: false, error: e } as const;
		}
		setStatus({ kind: "pending", pending: 1 });
		const r = await runtime.kitClient.abortTx();
		if (!r.ok) {
			runtime.pushSetRejection(r.error);
			setStatus({ kind: "error", pending: 0, lastError: r.error });
			return r;
		}
		setStatus({ kind: "idle", pending: 0 });
		return r;
	}, [runtime.kitClient, runtime.canWrite, runtime.pushSetRejection]);
	return { begin, commit, abort, status };
}

function useKitAddToKit(childKind: string): {
	run: (dto: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (dto: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			const kg = kitIdFromRuntime(runtime);
			if (!kg) {
				const e: SetError = { kind: "NotFound", message: "no active kit" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.addChild("Kit", kg, childKind, dto);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime, childKind],
	);
	return { run, status };
}

function useKitRemoveFromKit(childKind: string): {
	run: (childId: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (childId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			const kg = kitIdFromRuntime(runtime);
			if (!kg) {
				const e: SetError = { kind: "NotFound", message: "no active kit" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.removeChild("Kit", kg, childKind, childId);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime, childKind],
	);
	return { run, status };
}

export const useCreateAuthor = () => useKitAddToKit("Author");
export const useDeleteAuthor = () => useKitRemoveFromKit("Author");
export const useUpdateAuthor = (): {
	run: (authorId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (authorId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Author", authorId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateType = () => useKitAddToKit("Type");
export const useDeleteType = () => useKitRemoveFromKit("Type");
export const useUpdateType = (): {
	run: (typeId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (typeId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Type", typeId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateDesign = () => useKitAddToKit("Design");
export const useDeleteDesign = () => useKitRemoveFromKit("Design");
export const useUpdateDesign = (): {
	run: (designId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Design", designId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateQuality = () => useKitAddToKit("Quality");
export const useDeleteQuality = () => useKitRemoveFromKit("Quality");
export const useUpdateQuality = (): {
	run: (qualityId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (qualityId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Quality", qualityId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreatePort = () => useKitAddToKit("Port");
export const useDeletePort = () => useKitRemoveFromKit("Port");
export const useUpdatePort = (): {
	run: (portId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (portId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Port", portId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateTag = () => useKitAddToKit("Tag");
export const useDeleteTag = () => useKitRemoveFromKit("Tag");
export const useUpdateTag = (): {
	run: (tagId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (tagId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Tag", tagId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateConcept = () => useKitAddToKit("Concept");
export const useDeleteConcept = () => useKitRemoveFromKit("Concept");

export const useAddFile = () => useKitAddToKit("File");
export const useRemoveFile = () => useKitRemoveFromKit("File");
export const useUpdateFile = (): {
	run: (fileId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (fileId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("File", fileId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export const useCreateFolder = () => useKitAddToKit("Folder");
export const useDeleteFolder = () => useKitRemoveFromKit("Folder");
export const useUpdateFolder = (): {
	run: (folderId: string, patch: Record<string, unknown>) => Promise<SetResult>;
	status: WriteStatus;
} => {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (folderId: string, patch: Record<string, unknown>) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const [field, value] of Object.entries(patch)) {
				const r = await runtime.kitClient.setField("Folder", folderId, field, value);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
};

export function useMoveToFolder(): {
	run: (fileId: string, targetFolderId: string | null) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (fileId: string, targetFolderId: string | null) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.setField("File", fileId, "folder", targetFolderId);
			if (!r.ok) {
				runtime.pushSetRejection(r.error);
				setStatus({ kind: "error", pending: 0, lastError: r.error });
				return r;
			}
			setStatus({ kind: "idle", pending: 0 });
			return r;
		},
		[runtime],
	);
	return { run, status };
}

export type KitArtifactFolderKind = "type" | "design" | "quality" | "file" | "folder";

/**
 * Move a kit artifact into a folder (or to root) — mirrors legacy sketchpad `semio.kit.moveToFolder` / kitCommands behavior via {@link KitStoreClient.setField}.
 */
export function useMoveKitArtifactToFolder(): {
	run: (artifactKind: KitArtifactFolderKind, artifactId: string, folderId: string | null) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (artifactKind: KitArtifactFolderKind, artifactId: string, folderId: string | null) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			try {
				let r: SetResult;
				switch (artifactKind) {
					case "type":
						r = await runtime.kitClient.setField("Type", artifactId, "folder", folderId);
						break;
					case "design":
						r = await runtime.kitClient.setField("Design", artifactId, "folder", folderId);
						break;
					case "quality":
						r = await runtime.kitClient.setField("Quality", artifactId, "folder", folderId);
						break;
					case "file":
						r = await runtime.kitClient.setField("File", artifactId, "folder", folderId ? { id: folderId } : null);
						break;
					case "folder":
						r = await runtime.kitClient.setField("Folder", artifactId, "parent", folderId ? { id: folderId } : null);
						break;
					default: {
						const e: SetError = { kind: "InvalidValue", message: `unknown artifact kind: ${artifactKind}` };
						setStatus({ kind: "error", pending: 0, lastError: e });
						return { ok: false, error: e } as const;
					}
				}
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
				setStatus({ kind: "idle", pending: 0 });
				return r;
			} catch (e) {
				const err: SetError = { kind: "InvalidValue", message: e instanceof Error ? e.message : String(e) };
				setStatus({ kind: "error", pending: 0, lastError: err });
				return { ok: false, error: err } as const;
			}
		},
		[runtime],
	);
	return { run, status };
}

export function useImportKit(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
	const run = React.useCallback(async () => {
		return {
			ok: false,
			error: { kind: "InvalidValue", message: "useImportKit is wired from sketchpadMachine / host; not a KitStoreClient RPC" },
		} as const;
	}, []);
	return { run, status };
}

export function useExportKit(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
	const run = React.useCallback(async () => {
		return {
			ok: false,
			error: { kind: "InvalidValue", message: "useExportKit is wired from sketchpadMachine / host; not a KitStoreClient RPC" },
		} as const;
	}, []);
	return { run, status };
}

export function useAddConnections(): {
	run: (designId: string, connections: unknown[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const add = useAddConnection();
	const run = React.useCallback(
		async (designId: string, connections: unknown[]) => {
			for (const c of connections) {
				const r = await add.run(designId, c);
				if (!r.ok) return r;
			}
			return { ok: true } as const;
		},
		[add],
	);
	return { run, status: add.status };
}

export const useRemoveConnection = useDeleteConnection;

export function useRemoveConnections(): {
	run: (designId: string, connectionIds: string[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, connectionIds: string[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			for (const cg of connectionIds) {
				const r = await runtime.kitClient.deleteConnection(designId, cg);
				if (!r.ok) {
					runtime.pushSetRejection(r.error);
					setStatus({ kind: "error", pending: 0, lastError: r.error });
					return r;
				}
			}
			setStatus({ kind: "idle", pending: 0 });
			return { ok: true } as const;
		},
		[runtime],
	);
	return { run, status };
}

export function useDeleteSelected(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
	const run = React.useCallback(async () => {
		return { ok: false, error: { kind: "InvalidValue", message: "useDeleteSelected is UI/selection; use sketchpad actor" } } as const;
	}, []);
	return { run, status };
}

export function useDeselectAll(): { run: () => Promise<SetResult>; status: WriteStatus } {
	const [status] = React.useState<WriteStatus>({ kind: "readonly", pending: 0 });
	const run = React.useCallback(async () => {
		return { ok: false, error: { kind: "InvalidValue", message: "useDeselectAll is UI/selection; use sketchpad actor" } } as const;
	}, []);
	return { run, status };
}

export function usePasteDesignSelection(): {
	run: (designId: string, selection: unknown, plane?: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, selection: unknown, plane?: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.pasteDesignSelection(designId, selection, plane ?? null);
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
	run: (designId: string, typeIds: string[], plane: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, typeIds: string[], plane: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createHangingPieces(designId, typeIds, plane);
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
	run: (designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, parentPiece: string, parentPort: string, childType: string, childPort: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort);
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
	run: (designId: string, typeId: string, plane: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, typeId: string, plane: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.createFixedPiece(designId, typeId, plane);
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
	run: (designId: string, pieceId: string) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceId: string) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.removeChild("Design", designId, "Piece", pieceId);
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
	run: (designId: string, piece: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, piece: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.addChild("Design", designId, "Piece", piece);
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

/** @alias {@link useCreatePiece} */
export const useAddPiece = useCreatePiece;

export function useAddPieces(): {
	run: (designId: string, pieces: unknown[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const c = useCreatePiece();
	const run = React.useCallback(
		async (designId: string, pieces: unknown[]) => {
			for (const p of pieces) {
				const r = await c.run(designId, p);
				if (!r.ok) return r;
			}
			return { ok: true } as const;
		},
		[c],
	);
	return { run, status: c.status };
}

/** @alias {@link useDeletePiece} */
export const useRemovePiece = useDeletePiece;

export function useRemovePieces(): {
	run: (designId: string, pieceIds: string[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const del = useDeletePiece();
	const run = React.useCallback(
		async (designId: string, pieceIds: string[]) => {
			for (const g of pieceIds) {
				const r = await del.run(designId, g);
				if (!r.ok) return r;
			}
			return { ok: true } as const;
		},
		[del],
	);
	return { run, status: del.status };
}

export function useAddConnection(): {
	run: (designId: string, connection: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, connection: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const r = await runtime.kitClient.addChild("Design", designId, "Connection", connection);
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
	run: (designId: string, pieceId: string, patch: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, pieceId: string, patch: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = { pieces: { updated: [{ piece: { id: pieceId }, diff: patch }] } };
			const r = await runtime.kitClient.applyDesignDiff(designId, diff);
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
	run: (designId: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, updates: { id: string; diff: unknown }[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = {
				pieces: {
					updated: updates.map((u) => ({ piece: { id: u.id }, diff: u.diff })),
				},
			};
			const r = await runtime.kitClient.applyDesignDiff(designId, diff);
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
	run: (designId: string, connectionId: string, patch: unknown) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, connectionId: string, patch: unknown) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = { connections: { updated: [{ connection: { id: connectionId }, diff: patch }] } };
			const r = await runtime.kitClient.applyDesignDiff(designId, diff);
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
	run: (designId: string, updates: { id: string; diff: unknown }[]) => Promise<SetResult>;
	status: WriteStatus;
} {
	const runtime = useKitRuntime();
	const [status, setStatus] = React.useState<WriteStatus>({ kind: "idle", pending: 0 });
	const run = React.useCallback(
		async (designId: string, updates: { id: string; diff: unknown }[]) => {
			if (!runtime.kitClient || !runtime.canWrite) {
				const e: SetError = { kind: "Readonly", message: "read-only or no kit client" };
				setStatus({ kind: "error", pending: 0, lastError: e });
				return { ok: false, error: e } as const;
			}
			setStatus({ kind: "pending", pending: 1 });
			const diff = {
				connections: {
					updated: updates.map((u) => ({ connection: { id: u.id }, diff: u.diff })),
				},
			};
			const r = await runtime.kitClient.applyDesignDiff(designId, diff);
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
export function usePiecesMetadataMap(designId?: string): SchemaHookTriad<Record<string, any>> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<Record<string, any>>({});
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designId) {
			setValue({});
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getPiecesMetadata(designId);
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
	}, [runtime.kitClient, designId]);
	const status: WriteStatus =
		!designId || !runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcPieces(designId?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designId) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getPieces(designId);
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
	}, [runtime.kitClient, designId]);
	const status: WriteStatus =
		!designId || !runtime.kitClient
			? { kind: "readonly", pending: 0 }
			: pending > 0
				? { kind: "pending", pending }
				: { kind: "idle", pending: 0 };
	return [value, noopAsyncSet, status] as const;
}

export function useRpcConnections(designId?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const [value, setValue] = React.useState<any[]>([]);
	const [pending, setPending] = React.useState(0);
	React.useEffect(() => {
		if (!runtime.kitClient || !designId) {
			setValue([]);
			return;
		}
		let cancelled = false;
		const load = async () => {
			setPending((p) => p + 1);
			try {
				const m = await runtime.kitClient.getConnections(designId);
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
	}, [runtime.kitClient, designId]);
	const status: WriteStatus =
		!designId || !runtime.kitClient
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
export function usePieces(designId?: string): SchemaHookTriad<any[]> {
	return useRpcPieces(designId);
}

/** Alias for {@link useRpcConnections}. */
export function useConnections(designId?: string): SchemaHookTriad<any[]> {
	return useRpcConnections(designId);
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

export function usePieceMetadata(designId?: string, pieceId?: string): SchemaHookTriad<any> {
	const [map, , status] = usePiecesMetadataMap(designId);
	const value = React.useMemo(() => (pieceId ? map[pieceId] : undefined), [map, pieceId]);
	return [value, noopAsyncSet, status] as const;
}

export function useFlatPiecePlane(designId?: string, pieceId?: string): SchemaHookTriad<any> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => meta?.plane, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useFlatPieceCenter(designId?: string, pieceId?: string): SchemaHookTriad<any> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => meta?.center, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useIsConnectedPiece(designId?: string, pieceId?: string): SchemaHookTriad<boolean> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => !!(meta?.parentPieceId), [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function usePieceDepth(designId?: string, pieceId?: string): SchemaHookTriad<number> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => (typeof meta?.depth === "number" ? meta.depth : 0), [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useFixedPieceId(designId?: string, pieceId?: string): SchemaHookTriad<string | undefined> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => meta?.fixedPieceId, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function useParentPieceId(designId?: string, pieceId?: string): SchemaHookTriad<string | undefined> {
	const [meta, , status] = usePieceMetadata(designId, pieceId);
	const value = React.useMemo(() => meta?.parentPieceId ?? undefined, [meta]);
	return [value, noopAsyncSet, status] as const;
}

export function usePieceParentConnection(designId?: string, pieceId?: string): SchemaHookTriad<any | undefined> {
	const [conns, , st] = useRpcConnections(designId);
	const value = React.useMemo(() => {
		if (!pieceId || !Array.isArray(conns)) return undefined;
		return conns.find((c: any) => c?.connecting?.piece?.id === pieceId);
	}, [conns, pieceId]);
	return [value, noopAsyncSet, st] as const;
}

export function useIncludedDesigns(designId?: string): SchemaHookTriad<any[]> {
	const runtime = useKitRuntime();
	const value = React.useMemo(() => {
		if (!designId || !runtime.state?.kit) return [];
		const d = runtime.state.kit.designs?.find((x: any) => x.id === designId);
		return d ? getIncludedDesigns(d as Design) : [];
	}, [runtime.state.kit, designId]);
	return [value, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
}

export function useReplacableTypes(designId?: string, pieceIds?: string[]): SchemaHookTriad<string[]> {
	const runtime = useKitRuntime();
	const [, , metaStatus] = usePiecesMetadataMap(designId);
	const value = React.useMemo(() => {
		if (!designId || !pieceIds?.length || !runtime.state?.kit) return [];
		const kit = runtime.state.kit;
		const design = kit.designs?.find((d: any) => d.id === designId);
		if (!design) return [];
		const designs = kit.designs ?? [];
		const types = kit.types ?? [];
		const ports = kit.ports ?? [];
		return kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design as Design, designs as Design[], types as any, ports as any, { pieces: pieceIds }).types;
	}, [runtime.state.kit, designId, pieceIds]);
	return [value, noopAsyncSet, metaStatus] as const;
}

export function useReplacableDesigns(designId?: string, pieceIds?: string[]): SchemaHookTriad<string[]> {
	const runtime = useKitRuntime();
	const [, , metaStatus] = usePiecesMetadataMap(designId);
	const value = React.useMemo(() => {
		if (!designId || !pieceIds?.length || !runtime.state?.kit) return [];
		const kit = runtime.state.kit;
		const design = kit.designs?.find((d: any) => d.id === designId);
		if (!design) return [];
		const designs = kit.designs ?? [];
		const types = kit.types ?? [];
		const ports = kit.ports ?? [];
		return kit.findReplaceableTypesInDesignsForPiecesInDesignOp(design as Design, designs as Design[], types as any, ports as any, { pieces: pieceIds }).designs;
	}, [runtime.state.kit, designId, pieceIds]);
	return [value, noopAsyncSet, metaStatus] as const;
}

export function useExplodeableDesignNodes(designId?: string): SchemaHookTriad<string[]> {
	const [included, , st] = useIncludedDesigns(designId);
	const value = React.useMemo(() => (included ?? []).map((x: any) => x.id).filter(Boolean), [included]);
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

function useSchemaObjectState(typeName: string, idValue?: string): SchemaHookTriad<any> {
	const runtime = useKitRuntime();
	const scope = React.useContext(SchemaScopeContext);
	const ref = resolveReference(runtime.state, typeName, idValue, scope);
	const value = ref?.value;
	const canWrite = runtime.canWrite && !!ref;
	const setValue = React.useCallback(
		async (next: SetStateAction<any>) => {
			if (!canWrite) return { ok: false, error: { kind: "Readonly" as const, message: "read-only" } };
			runtime.setObjectValue(typeName, next, idValue, scope);
			return { ok: true } as const;
		},
		[runtime, typeName, idValue, scope, canWrite],
	);
	const status: WriteStatus = canWrite ? { kind: "idle", pending: 0 } : { kind: "readonly", pending: 0 };
	return [value, setValue, status] as const;
}

function useSchemaFieldState(typeName: string, fieldName: string, idValue?: string): SchemaHookTriad<any> {
	const runtime = useKitRuntime();
	const scope = React.useContext(SchemaScopeContext);
	const value = readSchemaFieldValue(runtime.state, typeName, fieldName, idValue, scope);
	const classicWritable = runtime.canWrite && isWritableField(runtime.state, typeName, fieldName, idValue, scope);
	const rustTarget = React.useMemo(
		() => resolveRustFieldTarget(runtime, typeName, fieldName, idValue, scope),
		[runtime.kitClient, runtime.snapshot.kit.id, runtime.canWrite, typeName, fieldName, idValue, scope],
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
				const r = await runtime.kitClient.setField(rustTarget.kind, rustTarget.id, rustTarget.field, resolved);
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
			runtime.setFieldValue(typeName, fieldName, resolved, idValue, scope);
			return { ok: true } as const;
		},
		[runtime, rustTarget, classicWritable, typeName, fieldName, idValue, scope, value],
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

export function useJSON(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("JSON", idValue);
}

export function useActorKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ActorKind", idValue);
}

export function useActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Actor", idValue);
}

export function useActorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "id", idValue);
}

export function useActorName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "name", idValue);
}

export function useActorEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "email", idValue);
}

export function useActorColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Actor", "color", idValue);
}

export function useUser(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("User", idValue);
}

export function useUserHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "hash", idValue);
}

export function useUserId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "id", idValue);
}

export function useUserName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "name", idValue);
}

export function useUserEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "email", idValue);
}

export function useUserColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("User", "color", idValue);
}

export function useAgent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Agent", idValue);
}

export function useAgentHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "hash", idValue);
}

export function useAgentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "id", idValue);
}

export function useAgentLlm(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "llm", idValue);
}

export function useAgentName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "name", idValue);
}

export function useAgentEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "email", idValue);
}

export function useAgentColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Agent", "color", idValue);
}

export function useSessionActorInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionActorInput", idValue);
}

export function useSessionActorInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "id", idValue);
}

export function useSessionActorInputKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "kind", idValue);
}

export function useSessionActorInputLlm(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "llm", idValue);
}

export function useSessionActorInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "name", idValue);
}

export function useSessionActorInputEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "email", idValue);
}

export function useSessionActorInputColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionActorInput", "color", idValue);
}

export function useCoordinate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Coordinate", idValue);
}

export function useCoordinateHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "hash", idValue);
}

export function useCoordinateU(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "u", idValue);
}

export function useCoordinateV(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Coordinate", "v", idValue);
}

export function useCoordinateInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CoordinateInput", idValue);
}

export function useCoordinateInputU(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CoordinateInput", "u", idValue);
}

export function useCoordinateInputV(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CoordinateInput", "v", idValue);
}

export function usePoint(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Point", idValue);
}

export function usePointHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "hash", idValue);
}

export function usePointX(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "x", idValue);
}

export function usePointY(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "y", idValue);
}

export function usePointZ(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Point", "z", idValue);
}

export function usePointInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PointInput", idValue);
}

export function usePointInputX(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "x", idValue);
}

export function usePointInputY(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "y", idValue);
}

export function usePointInputZ(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PointInput", "z", idValue);
}

export function useVector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Vector", idValue);
}

export function useVectorHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "hash", idValue);
}

export function useVectorX(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "x", idValue);
}

export function useVectorY(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "y", idValue);
}

export function useVectorZ(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Vector", "z", idValue);
}

export function useVectorInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VectorInput", idValue);
}

export function useVectorInputX(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "x", idValue);
}

export function useVectorInputY(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "y", idValue);
}

export function useVectorInputZ(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VectorInput", "z", idValue);
}

export function usePlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Plane", idValue);
}

export function usePlaneHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "hash", idValue);
}

export function usePlaneOrigin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "origin", idValue);
}

export function usePlaneXAxis(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "xAxis", idValue);
}

export function usePlaneYAxis(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Plane", "yAxis", idValue);
}

export function usePlaneInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PlaneInput", idValue);
}

export function usePlaneInputOrigin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "origin", idValue);
}

export function usePlaneInputXAxis(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "xAxis", idValue);
}

export function usePlaneInputYAxis(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PlaneInput", "yAxis", idValue);
}

export function useCamera(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Camera", idValue);
}

export function useCameraHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "hash", idValue);
}

export function useCameraPosition(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "position", idValue);
}

export function useCameraForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "forward", idValue);
}

export function useCameraUp(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Camera", "up", idValue);
}

export function useCameraInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CameraInput", idValue);
}

export function useCameraInputPosition(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "position", idValue);
}

export function useCameraInputForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "forward", idValue);
}

export function useCameraInputUp(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CameraInput", "up", idValue);
}

export function useAttribute(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Attribute", idValue);
}

export function useAttributeHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "hash", idValue);
}

export function useAttributeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "id", idValue);
}

export function useAttributeKey(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "key", idValue);
}

export function useAttributeValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "value", idValue);
}

export function useAttributeDefinition(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Attribute", "definition", idValue);
}

export function useAttributeInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AttributeInput", idValue);
}

export function useAttributeInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "id", idValue);
}

export function useAttributeInputKey(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "key", idValue);
}

export function useAttributeInputValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "value", idValue);
}

export function useAttributeInputDefinition(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AttributeInput", "definition", idValue);
}

export function useLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Location", idValue);
}

export function useLocationHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "hash", idValue);
}

export function useLocationLongitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "longitude", idValue);
}

export function useLocationLatitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "latitude", idValue);
}

export function useLocationAltitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "altitude", idValue);
}

export function useLocationAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Location", "attributes", idValue);
}

export function useLocationInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("LocationInput", idValue);
}

export function useLocationInputLongitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "longitude", idValue);
}

export function useLocationInputLatitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "latitude", idValue);
}

export function useLocationInputAltitude(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "altitude", idValue);
}

export function useLocationInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LocationInput", "attributes", idValue);
}

export function useAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Author", idValue);
}

export function useAuthorHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "hash", idValue);
}

export function useAuthorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "id", idValue);
}

export function useAuthorName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "name", idValue);
}

export function useAuthorEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "email", idValue);
}

export function useAuthorAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Author", "attributes", idValue);
}

export function useAuthorInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AuthorInput", idValue);
}

export function useAuthorInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "id", idValue);
}

export function useAuthorInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "name", idValue);
}

export function useAuthorInputEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "email", idValue);
}

export function useAuthorInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorInput", "attributes", idValue);
}

export function useAuthorPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AuthorPatchInput", idValue);
}

export function useAuthorPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "name", idValue);
}

export function useAuthorPatchInputEmail(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "email", idValue);
}

export function useAuthorPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AuthorPatchInput", "attributes", idValue);
}

export function useFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Folder", idValue);
}

export function useFolderHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "hash", idValue);
}

export function useFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "id", idValue);
}

export function useFolderKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "kit", idValue);
}

export function useFolderName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "name", idValue);
}

export function useFolderParent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "parent", idValue);
}

export function useFolderChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "children", idValue);
}

export function useFolderDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "description", idValue);
}

export function useFolderAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "attributes", idValue);
}

export function useFolderCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "createdAt", idValue);
}

export function useFolderCreatedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "createdBy", idValue);
}

export function useFolderUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "updatedAt", idValue);
}

export function useFolderUpdatedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Folder", "updatedBy", idValue);
}

export function useFolderInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FolderInput", idValue);
}

export function useFolderInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "id", idValue);
}

export function useFolderInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "name", idValue);
}

export function useFolderInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "parentId", idValue);
}

export function useFolderInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "description", idValue);
}

export function useFolderInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "attributes", idValue);
}

export function useFolderInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "createdAt", idValue);
}

export function useFolderInputCreatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "createdById", idValue);
}

export function useFolderInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "updatedAt", idValue);
}

export function useFolderInputUpdatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderInput", "updatedById", idValue);
}

export function useFolderPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FolderPatchInput", idValue);
}

export function useFolderPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "name", idValue);
}

export function useFolderPatchInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "parentId", idValue);
}

export function useFolderPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "description", idValue);
}

export function useFolderPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "attributes", idValue);
}

export function useFolderPatchInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "createdAt", idValue);
}

export function useFolderPatchInputCreatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "createdById", idValue);
}

export function useFolderPatchInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "updatedAt", idValue);
}

export function useFolderPatchInputUpdatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FolderPatchInput", "updatedById", idValue);
}

export function useFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("File", idValue);
}

export function useFileHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "hash", idValue);
}

export function useFileId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "id", idValue);
}

export function useFileKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "kit", idValue);
}

export function useFileName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "name", idValue);
}

export function useFileRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "remote", idValue);
}

export function useFileFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "folder", idValue);
}

export function useFileSize(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "size", idValue);
}

export function useFileContentHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "contentHash", idValue);
}

export function useFileBlob(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "blob", idValue);
}

export function useFileMime(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "mime", idValue);
}

export function useFileCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "createdAt", idValue);
}

export function useFileCreatedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "createdBy", idValue);
}

export function useFileUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "updatedAt", idValue);
}

export function useFileUpdatedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("File", "updatedBy", idValue);
}

export function useFileInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FileInput", idValue);
}

export function useFileInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "id", idValue);
}

export function useFileInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "name", idValue);
}

export function useFileInputRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "remote", idValue);
}

export function useFileInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "folderId", idValue);
}

export function useFileInputSize(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "size", idValue);
}

export function useFileInputContentHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "contentHash", idValue);
}

export function useFileInputBlob(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "blob", idValue);
}

export function useFileInputMime(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "mime", idValue);
}

export function useFileInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "createdAt", idValue);
}

export function useFileInputCreatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "createdById", idValue);
}

export function useFileInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "updatedAt", idValue);
}

export function useFileInputUpdatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FileInput", "updatedById", idValue);
}

export function useFilePatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FilePatchInput", idValue);
}

export function useFilePatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "name", idValue);
}

export function useFilePatchInputRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "remote", idValue);
}

export function useFilePatchInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "folderId", idValue);
}

export function useFilePatchInputSize(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "size", idValue);
}

export function useFilePatchInputContentHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "contentHash", idValue);
}

export function useFilePatchInputBlob(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "blob", idValue);
}

export function useFilePatchInputMime(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "mime", idValue);
}

export function useFilePatchInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "createdAt", idValue);
}

export function useFilePatchInputCreatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "createdById", idValue);
}

export function useFilePatchInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "updatedAt", idValue);
}

export function useFilePatchInputUpdatedById(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FilePatchInput", "updatedById", idValue);
}

export function useBenchmark(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Benchmark", idValue);
}

export function useBenchmarkHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "hash", idValue);
}

export function useBenchmarkId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "id", idValue);
}

export function useBenchmarkQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "quality", idValue);
}

export function useBenchmarkName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "name", idValue);
}

export function useBenchmarkIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "icon", idValue);
}

export function useBenchmarkMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "min", idValue);
}

export function useBenchmarkMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "minExcluded", idValue);
}

export function useBenchmarkMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "max", idValue);
}

export function useBenchmarkMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "maxExcluded", idValue);
}

export function useBenchmarkAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Benchmark", "attributes", idValue);
}

export function useBenchmarkInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BenchmarkInput", idValue);
}

export function useBenchmarkInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "id", idValue);
}

export function useBenchmarkInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "name", idValue);
}

export function useBenchmarkInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "icon", idValue);
}

export function useBenchmarkInputMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "min", idValue);
}

export function useBenchmarkInputMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "minExcluded", idValue);
}

export function useBenchmarkInputMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "max", idValue);
}

export function useBenchmarkInputMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "maxExcluded", idValue);
}

export function useBenchmarkInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BenchmarkInput", "attributes", idValue);
}

export function useQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Quality", idValue);
}

export function useQualityHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "hash", idValue);
}

export function useQualityId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "id", idValue);
}

export function useQualityKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "kit", idValue);
}

export function useQualityKey(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "key", idValue);
}

export function useQualityName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "name", idValue);
}

export function useQualityDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "description", idValue);
}

export function useQualityUri(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "uri", idValue);
}

export function useQualityKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "kind", idValue);
}

export function useQualityFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "folder", idValue);
}

export function useQualityCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "canScale", idValue);
}

export function useQualityDefaultSiUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultSiUnit", idValue);
}

export function useQualityDefaultImperialUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultImperialUnit", idValue);
}

export function useQualityMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "min", idValue);
}

export function useQualityIsMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "isMinExcluded", idValue);
}

export function useQualityMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "max", idValue);
}

export function useQualityIsMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "isMaxExcluded", idValue);
}

export function useQualityDefaultValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "defaultValue", idValue);
}

export function useQualityFormula(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "formula", idValue);
}

export function useQualityIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "icon", idValue);
}

export function useQualityImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "image", idValue);
}

export function useQualityUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "unit", idValue);
}

export function useQualityBenchmarks(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "benchmarks", idValue);
}

export function useQualityAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Quality", "attributes", idValue);
}

export function useQualityInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("QualityInput", idValue);
}

export function useQualityInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "id", idValue);
}

export function useQualityInputKey(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "key", idValue);
}

export function useQualityInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "name", idValue);
}

export function useQualityInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "description", idValue);
}

export function useQualityInputUri(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "uri", idValue);
}

export function useQualityInputKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "kind", idValue);
}

export function useQualityInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "folderId", idValue);
}

export function useQualityInputCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "canScale", idValue);
}

export function useQualityInputDefaultSiUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultSiUnit", idValue);
}

export function useQualityInputDefaultImperialUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultImperialUnit", idValue);
}

export function useQualityInputMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "min", idValue);
}

export function useQualityInputIsMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "isMinExcluded", idValue);
}

export function useQualityInputMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "max", idValue);
}

export function useQualityInputIsMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "isMaxExcluded", idValue);
}

export function useQualityInputDefaultValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "defaultValue", idValue);
}

export function useQualityInputFormula(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "formula", idValue);
}

export function useQualityInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "icon", idValue);
}

export function useQualityInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "image", idValue);
}

export function useQualityInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "unit", idValue);
}

export function useQualityInputBenchmarks(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "benchmarks", idValue);
}

export function useQualityInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityInput", "attributes", idValue);
}

export function useQualityPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("QualityPatchInput", idValue);
}

export function useQualityPatchInputKey(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "key", idValue);
}

export function useQualityPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "name", idValue);
}

export function useQualityPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "description", idValue);
}

export function useQualityPatchInputUri(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "uri", idValue);
}

export function useQualityPatchInputKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "kind", idValue);
}

export function useQualityPatchInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "folderId", idValue);
}

export function useQualityPatchInputCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "canScale", idValue);
}

export function useQualityPatchInputDefaultSiUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultSiUnit", idValue);
}

export function useQualityPatchInputDefaultImperialUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultImperialUnit", idValue);
}

export function useQualityPatchInputMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "min", idValue);
}

export function useQualityPatchInputIsMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "isMinExcluded", idValue);
}

export function useQualityPatchInputMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "max", idValue);
}

export function useQualityPatchInputIsMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "isMaxExcluded", idValue);
}

export function useQualityPatchInputDefaultValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "defaultValue", idValue);
}

export function useQualityPatchInputFormula(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "formula", idValue);
}

export function useQualityPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "icon", idValue);
}

export function useQualityPatchInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "image", idValue);
}

export function useQualityPatchInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "unit", idValue);
}

export function useQualityPatchInputBenchmarks(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "benchmarks", idValue);
}

export function useQualityPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("QualityPatchInput", "attributes", idValue);
}

export function usePort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Port", idValue);
}

export function usePortHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "hash", idValue);
}

export function usePortId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "id", idValue);
}

export function usePortKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "kit", idValue);
}

export function usePortName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "name", idValue);
}

export function usePortDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "description", idValue);
}

export function usePortIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "icon", idValue);
}

export function usePortMaxChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "maxChildren", idValue);
}

export function usePortCompatiblePorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "compatiblePorts", idValue);
}

export function usePortAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Port", "attributes", idValue);
}

export function usePortInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PortInput", idValue);
}

export function usePortInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "id", idValue);
}

export function usePortInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "name", idValue);
}

export function usePortInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "description", idValue);
}

export function usePortInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "icon", idValue);
}

export function usePortInputMaxChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "maxChildren", idValue);
}

export function usePortInputCompatiblePortIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "compatiblePortIds", idValue);
}

export function usePortInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortInput", "attributes", idValue);
}

export function usePortPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PortPatchInput", idValue);
}

export function usePortPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "name", idValue);
}

export function usePortPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "description", idValue);
}

export function usePortPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "icon", idValue);
}

export function usePortPatchInputMaxChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "maxChildren", idValue);
}

export function usePortPatchInputCompatiblePortIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "compatiblePortIds", idValue);
}

export function usePortPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PortPatchInput", "attributes", idValue);
}

export function useProp(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Prop", idValue);
}

export function usePropHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "hash", idValue);
}

export function usePropId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "id", idValue);
}

export function usePropKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "kit", idValue);
}

export function usePropQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "quality", idValue);
}

export function usePropValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "value", idValue);
}

export function usePropUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "unit", idValue);
}

export function usePropAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Prop", "attributes", idValue);
}

export function usePropInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PropInput", idValue);
}

export function usePropInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "id", idValue);
}

export function usePropInputQualityId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "qualityId", idValue);
}

export function usePropInputValue(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "value", idValue);
}

export function usePropInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "unit", idValue);
}

export function usePropInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PropInput", "attributes", idValue);
}

export function useTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Tag", idValue);
}

export function useTagHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "hash", idValue);
}

export function useTagId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "id", idValue);
}

export function useTagKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "kit", idValue);
}

export function useTagName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "name", idValue);
}

export function useTagDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "description", idValue);
}

export function useTagIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "icon", idValue);
}

export function useTagAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Tag", "attributes", idValue);
}

export function useTagInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TagInput", idValue);
}

export function useTagInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "id", idValue);
}

export function useTagInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "name", idValue);
}

export function useTagInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "description", idValue);
}

export function useTagInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "icon", idValue);
}

export function useTagInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagInput", "attributes", idValue);
}

export function useTagPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TagPatchInput", idValue);
}

export function useTagPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "name", idValue);
}

export function useTagPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "description", idValue);
}

export function useTagPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "icon", idValue);
}

export function useTagPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TagPatchInput", "attributes", idValue);
}

export function useConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Concept", idValue);
}

export function useConceptHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "hash", idValue);
}

export function useConceptId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "id", idValue);
}

export function useConceptKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "kit", idValue);
}

export function useConceptName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "name", idValue);
}

export function useConceptDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "description", idValue);
}

export function useConceptIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "icon", idValue);
}

export function useConceptAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Concept", "attributes", idValue);
}

export function useConceptInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConceptInput", idValue);
}

export function useConceptInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "id", idValue);
}

export function useConceptInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "name", idValue);
}

export function useConceptInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "description", idValue);
}

export function useConceptInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "icon", idValue);
}

export function useConceptInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptInput", "attributes", idValue);
}

export function useConceptPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConceptPatchInput", idValue);
}

export function useConceptPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "name", idValue);
}

export function useConceptPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "description", idValue);
}

export function useConceptPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "icon", idValue);
}

export function useConceptPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConceptPatchInput", "attributes", idValue);
}

export function useFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Family", idValue);
}

export function useFamilyHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "hash", idValue);
}

export function useFamilyId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "id", idValue);
}

export function useFamilyKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "kit", idValue);
}

export function useFamilyName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "name", idValue);
}

export function useFamilyDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "description", idValue);
}

export function useFamilyIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "icon", idValue);
}

export function useFamilyPorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "ports", idValue);
}

export function useFamilyAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Family", "attributes", idValue);
}

export function useFamilyInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FamilyInput", idValue);
}

export function useFamilyInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "id", idValue);
}

export function useFamilyInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "name", idValue);
}

export function useFamilyInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "description", idValue);
}

export function useFamilyInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "icon", idValue);
}

export function useFamilyInputPorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "ports", idValue);
}

export function useFamilyInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyInput", "attributes", idValue);
}

export function useFamilyPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FamilyPatchInput", idValue);
}

export function useFamilyPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "name", idValue);
}

export function useFamilyPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "description", idValue);
}

export function useFamilyPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "icon", idValue);
}

export function useFamilyPatchInputPorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "ports", idValue);
}

export function useFamilyPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FamilyPatchInput", "attributes", idValue);
}

export function useRepresentation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Representation", idValue);
}

export function useRepresentationHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "hash", idValue);
}

export function useRepresentationId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "id", idValue);
}

export function useRepresentationType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "type", idValue);
}

export function useRepresentationName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "name", idValue);
}

export function useRepresentationTags(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "tags", idValue);
}

export function useRepresentationFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "file", idValue);
}

export function useRepresentationDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "description", idValue);
}

export function useRepresentationAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Representation", "attributes", idValue);
}

export function useRepresentationInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("RepresentationInput", idValue);
}

export function useRepresentationInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "id", idValue);
}

export function useRepresentationInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "name", idValue);
}

export function useRepresentationInputTagIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "tagIds", idValue);
}

export function useRepresentationInputFileId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "fileId", idValue);
}

export function useRepresentationInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "description", idValue);
}

export function useRepresentationInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("RepresentationInput", "attributes", idValue);
}

export function useConnector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Connector", idValue);
}

export function useConnectorHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "hash", idValue);
}

export function useConnectorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "id", idValue);
}

export function useConnectorType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "type", idValue);
}

export function useConnectorName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "name", idValue);
}

export function useConnectorT(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "t", idValue);
}

export function useConnectorPoint(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "point", idValue);
}

export function useConnectorDirection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "direction", idValue);
}

export function useConnectorDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "description", idValue);
}

export function useConnectorPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "port", idValue);
}

export function useConnectorMandatory(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "mandatory", idValue);
}

export function useConnectorMaxChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "maxChildren", idValue);
}

export function useConnectorProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "props", idValue);
}

export function useConnectorAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "attributes", idValue);
}

export function useConnectorPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "pieces", idValue);
}

export function useConnectorCompatibleConnectors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connector", "compatibleConnectors", idValue);
}

export function useConnectorInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectorInput", idValue);
}

export function useConnectorInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "id", idValue);
}

export function useConnectorInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "name", idValue);
}

export function useConnectorInputT(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "t", idValue);
}

export function useConnectorInputPoint(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "point", idValue);
}

export function useConnectorInputDirection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "direction", idValue);
}

export function useConnectorInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "description", idValue);
}

export function useConnectorInputPortId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "portId", idValue);
}

export function useConnectorInputMandatory(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "mandatory", idValue);
}

export function useConnectorInputMaxChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "maxChildren", idValue);
}

export function useConnectorInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "props", idValue);
}

export function useConnectorInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectorInput", "attributes", idValue);
}

export function useType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Type", idValue);
}

export function useTypeHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "hash", idValue);
}

export function useTypeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "id", idValue);
}

export function useTypeKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "kit", idValue);
}

export function useTypeName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "name", idValue);
}

export function useTypeParent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "parent", idValue);
}

export function useTypeChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "children", idValue);
}

export function useTypeIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "isAbstract", idValue);
}

export function useTypeFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "folder", idValue);
}

export function useTypeRepresentations(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "representations", idValue);
}

export function useTypeConnectors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "connectors", idValue);
}

export function useTypeProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "props", idValue);
}

export function useTypeStock(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "stock", idValue);
}

export function useTypeVirtual(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "virtual", idValue);
}

export function useTypeUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "unit", idValue);
}

export function useTypeCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "createdAt", idValue);
}

export function useTypeUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "updatedAt", idValue);
}

export function useTypeLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "location", idValue);
}

export function useTypeAuthors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "authors", idValue);
}

export function useTypeConcepts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "concepts", idValue);
}

export function useTypeIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "icon", idValue);
}

export function useTypeImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "image", idValue);
}

export function useTypeDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "description", idValue);
}

export function useTypeAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "attributes", idValue);
}

export function useTypeFixedPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Type", "fixedPieces", idValue);
}

export function useTypeInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TypeInput", idValue);
}

export function useTypeInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "id", idValue);
}

export function useTypeInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "name", idValue);
}

export function useTypeInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "parentId", idValue);
}

export function useTypeInputIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "isAbstract", idValue);
}

export function useTypeInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "folderId", idValue);
}

export function useTypeInputRepresentations(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "representations", idValue);
}

export function useTypeInputConnectors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "connectors", idValue);
}

export function useTypeInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "props", idValue);
}

export function useTypeInputStock(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "stock", idValue);
}

export function useTypeInputVirtual(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "virtual", idValue);
}

export function useTypeInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "unit", idValue);
}

export function useTypeInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "createdAt", idValue);
}

export function useTypeInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "updatedAt", idValue);
}

export function useTypeInputLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "location", idValue);
}

export function useTypeInputAuthorIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "authorIds", idValue);
}

export function useTypeInputConceptIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "conceptIds", idValue);
}

export function useTypeInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "icon", idValue);
}

export function useTypeInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "image", idValue);
}

export function useTypeInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "description", idValue);
}

export function useTypeInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypeInput", "attributes", idValue);
}

export function useTypePatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TypePatchInput", idValue);
}

export function useTypePatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "name", idValue);
}

export function useTypePatchInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "parentId", idValue);
}

export function useTypePatchInputIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "isAbstract", idValue);
}

export function useTypePatchInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "folderId", idValue);
}

export function useTypePatchInputRepresentations(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "representations", idValue);
}

export function useTypePatchInputConnectors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "connectors", idValue);
}

export function useTypePatchInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "props", idValue);
}

export function useTypePatchInputStock(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "stock", idValue);
}

export function useTypePatchInputVirtual(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "virtual", idValue);
}

export function useTypePatchInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "unit", idValue);
}

export function useTypePatchInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "createdAt", idValue);
}

export function useTypePatchInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "updatedAt", idValue);
}

export function useTypePatchInputLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "location", idValue);
}

export function useTypePatchInputAuthorIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "authorIds", idValue);
}

export function useTypePatchInputConceptIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "conceptIds", idValue);
}

export function useTypePatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "icon", idValue);
}

export function useTypePatchInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "image", idValue);
}

export function useTypePatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "description", idValue);
}

export function useTypePatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TypePatchInput", "attributes", idValue);
}

export function useLayer(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Layer", idValue);
}

export function useLayerHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "hash", idValue);
}

export function useLayerId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "id", idValue);
}

export function useLayerDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "design", idValue);
}

export function useLayerPath(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "path", idValue);
}

export function useLayerIsHidden(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "isHidden", idValue);
}

export function useLayerIsLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "isLocked", idValue);
}

export function useLayerColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "color", idValue);
}

export function useLayerDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "description", idValue);
}

export function useLayerAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Layer", "attributes", idValue);
}

export function useLayerInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("LayerInput", idValue);
}

export function useLayerInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "id", idValue);
}

export function useLayerInputPath(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "path", idValue);
}

export function useLayerInputIsHidden(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "isHidden", idValue);
}

export function useLayerInputIsLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "isLocked", idValue);
}

export function useLayerInputColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "color", idValue);
}

export function useLayerInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "description", idValue);
}

export function useLayerInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("LayerInput", "attributes", idValue);
}

export function useSide(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Side", idValue);
}

export function useSideHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "hash", idValue);
}

export function useSideConnection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "connection", idValue);
}

export function useSidePiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "piece", idValue);
}

export function useSideDesignPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "designPiece", idValue);
}

export function useSideConnector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Side", "connector", idValue);
}

export function useSideInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SideInput", idValue);
}

export function useSideInputPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "pieceId", idValue);
}

export function useSideInputDesignPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "designPieceId", idValue);
}

export function useSideInputConnectorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SideInput", "connectorId", idValue);
}

export function useConnection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Connection", idValue);
}

export function useConnectionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "hash", idValue);
}

export function useConnectionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "id", idValue);
}

export function useConnectionDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "design", idValue);
}

export function useConnectionConnected(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "connected", idValue);
}

export function useConnectionConnecting(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "connecting", idValue);
}

export function useConnectionGap(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "gap", idValue);
}

export function useConnectionShift(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "shift", idValue);
}

export function useConnectionRise(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "rise", idValue);
}

export function useConnectionRotation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "rotation", idValue);
}

export function useConnectionTurn(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "turn", idValue);
}

export function useConnectionTilt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "tilt", idValue);
}

export function useConnectionU(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "u", idValue);
}

export function useConnectionV(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "v", idValue);
}

export function useConnectionDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "description", idValue);
}

export function useConnectionAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "attributes", idValue);
}

export function useConnectionChildPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "childPiece", idValue);
}

export function useConnectionChildConnector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "childConnector", idValue);
}

export function useConnectionParentPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "parentPiece", idValue);
}

export function useConnectionParentConnector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Connection", "parentConnector", idValue);
}

export function useConnectionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionInput", idValue);
}

export function useConnectionInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "id", idValue);
}

export function useConnectionInputConnected(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "connected", idValue);
}

export function useConnectionInputConnecting(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "connecting", idValue);
}

export function useConnectionInputGap(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "gap", idValue);
}

export function useConnectionInputShift(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "shift", idValue);
}

export function useConnectionInputRise(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "rise", idValue);
}

export function useConnectionInputRotation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "rotation", idValue);
}

export function useConnectionInputTurn(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "turn", idValue);
}

export function useConnectionInputTilt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "tilt", idValue);
}

export function useConnectionInputU(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "u", idValue);
}

export function useConnectionInputV(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "v", idValue);
}

export function useConnectionInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "description", idValue);
}

export function useConnectionInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionInput", "attributes", idValue);
}

export function useConnectionPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionPatchInput", idValue);
}

export function useConnectionPatchInputConnected(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "connected", idValue);
}

export function useConnectionPatchInputConnecting(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "connecting", idValue);
}

export function useConnectionPatchInputGap(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "gap", idValue);
}

export function useConnectionPatchInputShift(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "shift", idValue);
}

export function useConnectionPatchInputRise(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "rise", idValue);
}

export function useConnectionPatchInputRotation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "rotation", idValue);
}

export function useConnectionPatchInputTurn(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "turn", idValue);
}

export function useConnectionPatchInputTilt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "tilt", idValue);
}

export function useConnectionPatchInputU(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "u", idValue);
}

export function useConnectionPatchInputV(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "v", idValue);
}

export function useConnectionPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "description", idValue);
}

export function useConnectionPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionPatchInput", "attributes", idValue);
}

export function useStat(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Stat", idValue);
}

export function useStatHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "hash", idValue);
}

export function useStatId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "id", idValue);
}

export function useStatDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "design", idValue);
}

export function useStatQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "quality", idValue);
}

export function useStatUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "unit", idValue);
}

export function useStatMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "min", idValue);
}

export function useStatMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "minExcluded", idValue);
}

export function useStatMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "max", idValue);
}

export function useStatMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Stat", "maxExcluded", idValue);
}

export function useStatInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StatInput", idValue);
}

export function useStatInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "id", idValue);
}

export function useStatInputQualityId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "qualityId", idValue);
}

export function useStatInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "unit", idValue);
}

export function useStatInputMin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "min", idValue);
}

export function useStatInputMinExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "minExcluded", idValue);
}

export function useStatInputMax(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "max", idValue);
}

export function useStatInputMaxExcluded(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StatInput", "maxExcluded", idValue);
}

export function usePieceKindEnum(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceKind", idValue);
}

export function useBlueprint(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Blueprint", idValue);
}

export function useBlueprintType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Blueprint", "type", idValue);
}

export function useBlueprintDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Blueprint", "design", idValue);
}

export function usePiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Piece", idValue);
}

export function usePieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "id", idValue);
}

export function usePieceHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "hash", idValue);
}

export function usePieceName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "name", idValue);
}

export function usePiecePlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "plane", idValue);
}

export function usePieceCenter(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "center", idValue);
}

export function usePieceScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "scale", idValue);
}

export function usePieceMirrorPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "mirrorPlane", idValue);
}

export function usePieceIsHidden(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "isHidden", idValue);
}

export function usePieceIsLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "isLocked", idValue);
}

export function usePieceColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "color", idValue);
}

export function usePieceDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "description", idValue);
}

export function usePieceKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "kind", idValue);
}

export function usePieceType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "type", idValue);
}

export function usePieceDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "design", idValue);
}

export function usePieceProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "props", idValue);
}

export function usePieceAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "attributes", idValue);
}

export function usePieceFlatPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "flatPlane", idValue);
}

export function usePieceFlatCenter(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "flatCenter", idValue);
}

export function usePieceParentPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "parentPiece", idValue);
}

export function usePieceChildPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "childPieces", idValue);
}

export function usePieceChildConnections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "childConnections", idValue);
}

export function usePieceAlternatives(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternatives", idValue);
}

export function usePieceAlternativeTypes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternativeTypes", idValue);
}

export function usePieceAlternativeDesigns(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Piece", "alternativeDesigns", idValue);
}

export function usePieceInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceInput", idValue);
}

export function usePieceInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "id", idValue);
}

export function usePieceInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "name", idValue);
}

export function usePieceInputTypeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "typeId", idValue);
}

export function usePieceInputDesignReferenceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "designReferenceId", idValue);
}

export function usePieceInputPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "plane", idValue);
}

export function usePieceInputCenter(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "center", idValue);
}

export function usePieceInputScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "scale", idValue);
}

export function usePieceInputMirrorPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "mirrorPlane", idValue);
}

export function usePieceInputIsHidden(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "isHidden", idValue);
}

export function usePieceInputIsLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "isLocked", idValue);
}

export function usePieceInputColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "color", idValue);
}

export function usePieceInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "description", idValue);
}

export function usePieceInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "props", idValue);
}

export function usePieceInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceInput", "attributes", idValue);
}

export function usePiecePatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PiecePatchInput", idValue);
}

export function usePiecePatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "name", idValue);
}

export function usePiecePatchInputTypeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "typeId", idValue);
}

export function usePiecePatchInputDesignReferenceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "designReferenceId", idValue);
}

export function usePiecePatchInputPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "plane", idValue);
}

export function usePiecePatchInputCenter(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "center", idValue);
}

export function usePiecePatchInputScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "scale", idValue);
}

export function usePiecePatchInputMirrorPlane(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "mirrorPlane", idValue);
}

export function usePiecePatchInputIsHidden(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "isHidden", idValue);
}

export function usePiecePatchInputIsLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "isLocked", idValue);
}

export function usePiecePatchInputColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "color", idValue);
}

export function usePiecePatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "description", idValue);
}

export function usePiecePatchInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "props", idValue);
}

export function usePiecePatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PiecePatchInput", "attributes", idValue);
}

export function useGroup(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Group", idValue);
}

export function useGroupHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "hash", idValue);
}

export function useGroupId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "id", idValue);
}

export function useGroupDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "design", idValue);
}

export function useGroupPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "pieces", idValue);
}

export function useGroupColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "color", idValue);
}

export function useGroupName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "name", idValue);
}

export function useGroupDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "description", idValue);
}

export function useGroupAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Group", "attributes", idValue);
}

export function useGroupInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("GroupInput", idValue);
}

export function useGroupInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "id", idValue);
}

export function useGroupInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "pieceIds", idValue);
}

export function useGroupInputColor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "color", idValue);
}

export function useGroupInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "name", idValue);
}

export function useGroupInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "description", idValue);
}

export function useGroupInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("GroupInput", "attributes", idValue);
}

export function useDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Design", idValue);
}

export function useDesignHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "hash", idValue);
}

export function useDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "id", idValue);
}

export function useDesignKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "kit", idValue);
}

export function useDesignName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "name", idValue);
}

export function useDesignParent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "parent", idValue);
}

export function useDesignChildren(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "children", idValue);
}

export function useDesignIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "isAbstract", idValue);
}

export function useDesignFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "folder", idValue);
}

export function useDesignPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "pieces", idValue);
}

export function useDesignConnections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "connections", idValue);
}

export function useDesignStats(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "stats", idValue);
}

export function useDesignProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "props", idValue);
}

export function useDesignLayers(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "layers", idValue);
}

export function useDesignActiveLayer(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "activeLayer", idValue);
}

export function useDesignGroups(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "groups", idValue);
}

export function useDesignCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "canScale", idValue);
}

export function useDesignCanMirror(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "canMirror", idValue);
}

export function useDesignUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "unit", idValue);
}

export function useDesignLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "location", idValue);
}

export function useDesignAuthors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "authors", idValue);
}

export function useDesignConcepts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "concepts", idValue);
}

export function useDesignIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "icon", idValue);
}

export function useDesignImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "image", idValue);
}

export function useDesignDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "description", idValue);
}

export function useDesignAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "attributes", idValue);
}

export function useDesignCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "createdAt", idValue);
}

export function useDesignUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Design", "updatedAt", idValue);
}

export function useDesignInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DesignInput", idValue);
}

export function useDesignInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "id", idValue);
}

export function useDesignInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "name", idValue);
}

export function useDesignInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "parentId", idValue);
}

export function useDesignInputIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "isAbstract", idValue);
}

export function useDesignInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "folderId", idValue);
}

export function useDesignInputPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "pieces", idValue);
}

export function useDesignInputConnections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "connections", idValue);
}

export function useDesignInputStats(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "stats", idValue);
}

export function useDesignInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "props", idValue);
}

export function useDesignInputLayers(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "layers", idValue);
}

export function useDesignInputActiveLayerId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "activeLayerId", idValue);
}

export function useDesignInputGroups(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "groups", idValue);
}

export function useDesignInputCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "canScale", idValue);
}

export function useDesignInputCanMirror(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "canMirror", idValue);
}

export function useDesignInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "unit", idValue);
}

export function useDesignInputLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "location", idValue);
}

export function useDesignInputAuthorIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "authorIds", idValue);
}

export function useDesignInputConceptIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "conceptIds", idValue);
}

export function useDesignInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "icon", idValue);
}

export function useDesignInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "image", idValue);
}

export function useDesignInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "description", idValue);
}

export function useDesignInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "attributes", idValue);
}

export function useDesignInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "createdAt", idValue);
}

export function useDesignInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignInput", "updatedAt", idValue);
}

export function useDesignPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DesignPatchInput", idValue);
}

export function useDesignPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "name", idValue);
}

export function useDesignPatchInputParentId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "parentId", idValue);
}

export function useDesignPatchInputIsAbstract(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "isAbstract", idValue);
}

export function useDesignPatchInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "folderId", idValue);
}

export function useDesignPatchInputStats(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "stats", idValue);
}

export function useDesignPatchInputProps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "props", idValue);
}

export function useDesignPatchInputLayers(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "layers", idValue);
}

export function useDesignPatchInputActiveLayerId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "activeLayerId", idValue);
}

export function useDesignPatchInputGroups(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "groups", idValue);
}

export function useDesignPatchInputCanScale(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "canScale", idValue);
}

export function useDesignPatchInputCanMirror(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "canMirror", idValue);
}

export function useDesignPatchInputUnit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "unit", idValue);
}

export function useDesignPatchInputLocation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "location", idValue);
}

export function useDesignPatchInputAuthorIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "authorIds", idValue);
}

export function useDesignPatchInputConceptIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "conceptIds", idValue);
}

export function useDesignPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "icon", idValue);
}

export function useDesignPatchInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "image", idValue);
}

export function useDesignPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "description", idValue);
}

export function useDesignPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "attributes", idValue);
}

export function useDesignPatchInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "createdAt", idValue);
}

export function useDesignPatchInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DesignPatchInput", "updatedAt", idValue);
}

export function useKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Kit", idValue);
}

export function useKitHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "hash", idValue);
}

export function useKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "id", idValue);
}

export function useKitName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "name", idValue);
}

export function useKitRelease(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "release", idValue);
}

export function useKitTypes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "types", idValue);
}

export function useKitDesigns(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "designs", idValue);
}

export function useKitTags(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "tags", idValue);
}

export function useKitConcepts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "concepts", idValue);
}

export function useKitFamilies(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "families", idValue);
}

export function useKitPorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "ports", idValue);
}

export function useKitQualities(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "qualities", idValue);
}

export function useKitFiles(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "files", idValue);
}

export function useKitFolders(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "folders", idValue);
}

export function useKitAuthors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "authors", idValue);
}

export function useKitRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "remote", idValue);
}

export function useKitHomepage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "homepage", idValue);
}

export function useKitLicense(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "license", idValue);
}

export function useKitPreview(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "preview", idValue);
}

export function useKitIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "icon", idValue);
}

export function useKitImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "image", idValue);
}

export function useKitDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "description", idValue);
}

export function useKitAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "attributes", idValue);
}

export function useKitCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "createdAt", idValue);
}

export function useKitUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Kit", "updatedAt", idValue);
}

export function useKitInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInput", idValue);
}

export function useKitInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "id", idValue);
}

export function useKitInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "name", idValue);
}

export function useKitInputRelease(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "release", idValue);
}

export function useKitInputTypes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "types", idValue);
}

export function useKitInputDesigns(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "designs", idValue);
}

export function useKitInputTags(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "tags", idValue);
}

export function useKitInputConcepts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "concepts", idValue);
}

export function useKitInputFamilies(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "families", idValue);
}

export function useKitInputPorts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "ports", idValue);
}

export function useKitInputQualities(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "qualities", idValue);
}

export function useKitInputFiles(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "files", idValue);
}

export function useKitInputFolders(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "folders", idValue);
}

export function useKitInputAuthors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "authors", idValue);
}

export function useKitInputRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "remote", idValue);
}

export function useKitInputHomepage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "homepage", idValue);
}

export function useKitInputLicense(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "license", idValue);
}

export function useKitInputPreview(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "preview", idValue);
}

export function useKitInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "icon", idValue);
}

export function useKitInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "image", idValue);
}

export function useKitInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "description", idValue);
}

export function useKitInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "attributes", idValue);
}

export function useKitInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "createdAt", idValue);
}

export function useKitInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInput", "updatedAt", idValue);
}

export function useKitPatchInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitPatchInput", idValue);
}

export function useKitPatchInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "name", idValue);
}

export function useKitPatchInputRelease(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "release", idValue);
}

export function useKitPatchInputRemote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "remote", idValue);
}

export function useKitPatchInputHomepage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "homepage", idValue);
}

export function useKitPatchInputLicense(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "license", idValue);
}

export function useKitPatchInputPreview(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "preview", idValue);
}

export function useKitPatchInputIcon(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "icon", idValue);
}

export function useKitPatchInputImage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "image", idValue);
}

export function useKitPatchInputDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "description", idValue);
}

export function useKitPatchInputAttributes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "attributes", idValue);
}

export function useKitPatchInputCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "createdAt", idValue);
}

export function useKitPatchInputUpdatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitPatchInput", "updatedAt", idValue);
}

export function useBackboneKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BackboneKind", idValue);
}

export function useKitBackbone(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitBackbone", idValue);
}

export function useKitBackboneHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "hash", idValue);
}

export function useKitBackboneKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "kind", idValue);
}

export function useKitBackboneEndpoint(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "endpoint", idValue);
}

export function useKitBackboneAuthoritative(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "authoritative", idValue);
}

export function useKitBackboneLinearHistory(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "linearHistory", idValue);
}

export function useKitBackboneConnected(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "connected", idValue);
}

export function useKitBackboneTimeoutSeconds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "timeoutSeconds", idValue);
}

export function useKitBackboneCurrentHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "currentHash", idValue);
}

export function useKitBackboneLastInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "lastInteractionIndex", idValue);
}

export function useKitBackbonePendingCandidateCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitBackbone", "pendingCandidateCount", idValue);
}

export function useKitClientInfo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitClientInfo", idValue);
}

export function useKitClientInfoHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "hash", idValue);
}

export function useKitClientInfoId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "id", idValue);
}

export function useKitClientInfoName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "name", idValue);
}

export function useKitClientInfoVersion(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "version", idValue);
}

export function useKitClientInfoPlatform(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfo", "platform", idValue);
}

export function useKitClientInfoInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitClientInfoInput", idValue);
}

export function useKitClientInfoInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "id", idValue);
}

export function useKitClientInfoInputName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "name", idValue);
}

export function useKitClientInfoInputVersion(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "version", idValue);
}

export function useKitClientInfoInputPlatform(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitClientInfoInput", "platform", idValue);
}

export function useSessionState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionState", idValue);
}

export function useSessionWarningActionKindEnum(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionWarningActionKind", idValue);
}

export function useSessionWarningAction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionWarningAction", idValue);
}

export function useSessionWarningActionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "hash", idValue);
}

export function useSessionWarningActionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "kind", idValue);
}

export function useSessionWarningActionLabel(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionWarningAction", "label", idValue);
}

export function useKitSessionWarningEntity(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSessionWarning", idValue);
}

export function useKitSessionWarningHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "hash", idValue);
}

export function useKitSessionWarningCode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "code", idValue);
}

export function useKitSessionWarningMessage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "message", idValue);
}

export function useKitSessionWarningActions(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionWarning", "actions", idValue);
}

export function useSessionConnectorSelection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionConnectorSelection", idValue);
}

export function useSessionConnectorSelectionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "hash", idValue);
}

export function useSessionConnectorSelectionPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "piece", idValue);
}

export function useSessionConnectorSelectionDesignPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "designPiece", idValue);
}

export function useSessionConnectorSelectionConnector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelection", "connector", idValue);
}

export function useSessionConnectorSelectionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionConnectorSelectionInput", idValue);
}

export function useSessionConnectorSelectionInputPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "pieceId", idValue);
}

export function useSessionConnectorSelectionInputDesignPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "designPieceId", idValue);
}

export function useSessionConnectorSelectionInputConnectorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionConnectorSelectionInput", "connectorId", idValue);
}

export function useKitSessionSelectionEntity(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSessionSelection", idValue);
}

export function useKitSessionSelectionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "hash", idValue);
}

export function useKitSessionSelectionActiveDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "activeDesign", idValue);
}

export function useKitSessionSelectionPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "pieces", idValue);
}

export function useKitSessionSelectionConnections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "connections", idValue);
}

export function useKitSessionSelectionConnectors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "connectors", idValue);
}

export function useKitSessionSelectionRepresentations(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "representations", idValue);
}

export function useKitSessionSelectionDesigns(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "designs", idValue);
}

export function useKitSessionSelectionTypes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "types", idValue);
}

export function useKitSessionSelectionReplacementTypeCandidates(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "replacementTypeCandidates", idValue);
}

export function useKitSessionSelectionReplacementDesignCandidates(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "replacementDesignCandidates", idValue);
}

export function useKitSessionSelectionBoundaryConnectorCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSessionSelection", "boundaryConnectorCount", idValue);
}

export function useSessionSelectionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SessionSelectionInput", idValue);
}

export function useSessionSelectionInputActiveDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "activeDesignId", idValue);
}

export function useSessionSelectionInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "pieceIds", idValue);
}

export function useSessionSelectionInputConnectionIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "connectionIds", idValue);
}

export function useSessionSelectionInputConnectorSelections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "connectorSelections", idValue);
}

export function useSessionSelectionInputRepresentationIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "representationIds", idValue);
}

export function useSessionSelectionInputDesignIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "designIds", idValue);
}

export function useSessionSelectionInputTypeIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SessionSelectionInput", "typeIds", idValue);
}

export function useKitSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitSession", idValue);
}

export function useKitSessionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "hash", idValue);
}

export function useKitSessionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "id", idValue);
}

export function useKitSessionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "kit", idValue);
}

export function useKitSessionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "actor", idValue);
}

export function useKitSessionClient(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "client", idValue);
}

export function useKitSessionState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "state", idValue);
}

export function useKitSessionStrictMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "strictMode", idValue);
}

export function useKitSessionTimeoutSeconds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "timeoutSeconds", idValue);
}

export function useKitSessionStartedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "startedAt", idValue);
}

export function useKitSessionLastSeenAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "lastSeenAt", idValue);
}

export function useKitSessionExpiresAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "expiresAt", idValue);
}

export function useKitSessionDisconnectedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "disconnectedAt", idValue);
}

export function useKitSessionLocked(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "locked", idValue);
}

export function useKitSessionCanReconnect(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "canReconnect", idValue);
}

export function useKitSessionCanSaveLocalChanges(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "canSaveLocalChanges", idValue);
}

export function useKitSessionWarning(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "warning", idValue);
}

export function useKitSessionSelection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "selection", idValue);
}

export function useKitSessionActiveTransactions(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitSession", "activeTransactions", idValue);
}

export function useValidationSeverity(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ValidationSeverity", idValue);
}

export function useValidationNote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ValidationNote", idValue);
}

export function useValidationNoteHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "hash", idValue);
}

export function useValidationNoteSeverity(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "severity", idValue);
}

export function useValidationNoteCode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "code", idValue);
}

export function useValidationNotePath(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "path", idValue);
}

export function useValidationNoteEntityId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "entityId", idValue);
}

export function useValidationNoteMessage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ValidationNote", "message", idValue);
}

export function useKitValidationResult(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitValidationResult", idValue);
}

export function useKitValidationResultHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "hash", idValue);
}

export function useKitValidationResultOk(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "ok", idValue);
}

export function useKitValidationResultImmutable(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "immutable", idValue);
}

export function useKitValidationResultStrict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "strict", idValue);
}

export function useKitValidationResultErrors(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "errors", idValue);
}

export function useKitValidationResultWarnings(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "warnings", idValue);
}

export function useKitValidationResultInfos(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitValidationResult", "infos", idValue);
}

export function useKitConflictStatusEnum(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflictStatus", idValue);
}

export function useKitConflictKindEnum(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflictKind", idValue);
}

export function useConflictResolutionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConflictResolutionKind", idValue);
}

export function useConflictResolutionOption(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConflictResolutionOption", idValue);
}

export function useConflictResolutionOptionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "hash", idValue);
}

export function useConflictResolutionOptionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "id", idValue);
}

export function useConflictResolutionOptionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "kind", idValue);
}

export function useConflictResolutionOptionLabel(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "label", idValue);
}

export function useConflictResolutionOptionDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "description", idValue);
}

export function useConflictResolutionOptionPatchPreview(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConflictResolutionOption", "patchPreview", idValue);
}

export function useKitConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitConflict", idValue);
}

export function useKitConflictHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "hash", idValue);
}

export function useKitConflictId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "id", idValue);
}

export function useKitConflictKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "kit", idValue);
}

export function useKitConflictSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "session", idValue);
}

export function useKitConflictCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "candidate", idValue);
}

export function useKitConflictStatus(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "status", idValue);
}

export function useKitConflictKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "kind", idValue);
}

export function useKitConflictTitle(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "title", idValue);
}

export function useKitConflictMessage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "message", idValue);
}

export function useKitConflictBlocking(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "blocking", idValue);
}

export function useKitConflictStrict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "strict", idValue);
}

export function useKitConflictNotes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "notes", idValue);
}

export function useKitConflictOptions(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "options", idValue);
}

export function useKitConflictCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "createdAt", idValue);
}

export function useKitConflictResolvedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitConflict", "resolvedAt", idValue);
}

export function useKitCommandKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandKind", idValue);
}

export function useKitCommandDescriptor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandDescriptor", idValue);
}

export function useKitCommandDescriptorHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "hash", idValue);
}

export function useKitCommandDescriptorKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "kind", idValue);
}

export function useKitCommandDescriptorMutatesKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "mutatesKit", idValue);
}

export function useKitCommandDescriptorSessionScoped(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "sessionScoped", idValue);
}

export function useKitCommandDescriptorRequiresConsensus(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "requiresConsensus", idValue);
}

export function useKitCommandDescriptorDescription(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandDescriptor", "description", idValue);
}

export function useKitChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitChange", idValue);
}

export function useKitChangeHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "hash", idValue);
}

export function useKitChangeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "id", idValue);
}

export function useKitChangeKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "kind", idValue);
}

export function useKitChangeSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "summary", idValue);
}

export function useKitChangeOrigin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "origin", idValue);
}

export function useKitChangeActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "actor", idValue);
}

export function useKitChangeSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "session", idValue);
}

export function useKitChangeTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "transaction", idValue);
}

export function useKitChangeForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "forward", idValue);
}

export function useKitChangeBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "backward", idValue);
}

export function useKitChangeValidation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "validation", idValue);
}

export function useKitChangeCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "createdAt", idValue);
}

export function useKitChangeAppliedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChange", "appliedAt", idValue);
}

export function useKitCandidateStatus(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCandidateStatus", idValue);
}

export function useCandidateVoteState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CandidateVoteState", idValue);
}

export function useKitCandidateVote(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCandidateVote", idValue);
}

export function useKitCandidateVoteHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "hash", idValue);
}

export function useKitCandidateVoteSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "session", idValue);
}

export function useKitCandidateVoteState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "state", idValue);
}

export function useKitCandidateVoteReason(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "reason", idValue);
}

export function useKitCandidateVoteRespondedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "respondedAt", idValue);
}

export function useKitCandidateVoteResolutionOptionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCandidateVote", "resolutionOptionId", idValue);
}

export function useKitChangeCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitChangeCandidate", idValue);
}

export function useKitChangeCandidateHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "hash", idValue);
}

export function useKitChangeCandidateId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "id", idValue);
}

export function useKitChangeCandidateKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "kit", idValue);
}

export function useKitChangeCandidateKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "kind", idValue);
}

export function useKitChangeCandidateSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "summary", idValue);
}

export function useKitChangeCandidateProposedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "proposedBy", idValue);
}

export function useKitChangeCandidateActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "actor", idValue);
}

export function useKitChangeCandidateTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "transaction", idValue);
}

export function useKitChangeCandidateStatus(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "status", idValue);
}

export function useKitChangeCandidateRequestedFrom(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "requestedFrom", idValue);
}

export function useKitChangeCandidateVotes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "votes", idValue);
}

export function useKitChangeCandidateValidation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "validation", idValue);
}

export function useKitChangeCandidatePreview(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "preview", idValue);
}

export function useKitChangeCandidateProposedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "proposedAt", idValue);
}

export function useKitChangeCandidateExpiresAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "expiresAt", idValue);
}

export function useKitChangeCandidateDecidedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitChangeCandidate", "decidedAt", idValue);
}

export function useTransactionState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionState", idValue);
}

export function useKitTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitTransaction", idValue);
}

export function useKitTransactionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "hash", idValue);
}

export function useKitTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "id", idValue);
}

export function useKitTransactionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "kit", idValue);
}

export function useKitTransactionLabel(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "label", idValue);
}

export function useKitTransactionState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "state", idValue);
}

export function useKitTransactionStartedBy(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "startedBy", idValue);
}

export function useKitTransactionParent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "parent", idValue);
}

export function useKitTransactionStartedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "startedAt", idValue);
}

export function useKitTransactionFinalizedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "finalizedAt", idValue);
}

export function useKitTransactionAbortedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "abortedAt", idValue);
}

export function useKitTransactionChanges(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "changes", idValue);
}

export function useKitTransactionUndoStack(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "undoStack", idValue);
}

export function useKitTransactionRedoStack(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "redoStack", idValue);
}

export function useKitTransactionCanUndo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "canUndo", idValue);
}

export function useKitTransactionCanRedo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "canRedo", idValue);
}

export function useKitTransactionSquashedChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitTransaction", "squashedChange", idValue);
}

export function useKitHistoryEntry(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistoryEntry", idValue);
}

export function useKitHistoryEntryHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "hash", idValue);
}

export function useKitHistoryEntryId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "id", idValue);
}

export function useKitHistoryEntryIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "index", idValue);
}

export function useKitHistoryEntryTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "transaction", idValue);
}

export function useKitHistoryEntryCommandKinds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "commandKinds", idValue);
}

export function useKitHistoryEntrySummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "summary", idValue);
}

export function useKitHistoryEntrySquashedChangeCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "squashedChangeCount", idValue);
}

export function useKitHistoryEntryChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "change", idValue);
}

export function useKitHistoryEntryCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "createdAt", idValue);
}

export function useKitHistoryEntryFinalizedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "finalizedAt", idValue);
}

export function useKitHistoryEntryUndoneAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryEntry", "undoneAt", idValue);
}

export function useKitHistoryPage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistoryPage", idValue);
}

export function useKitHistoryPageHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "hash", idValue);
}

export function useKitHistoryPageNodes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "nodes", idValue);
}

export function useKitHistoryPagePageInfo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "pageInfo", idValue);
}

export function useKitHistoryPageTotalCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistoryPage", "totalCount", idValue);
}

export function useKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInteraction", idValue);
}

export function useKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "id", idValue);
}

export function useKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "hash", idValue);
}

export function useKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "index", idValue);
}

export function useKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "kit", idValue);
}

export function useKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "kind", idValue);
}

export function useKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "actor", idValue);
}

export function useKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "session", idValue);
}

export function useKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "transaction", idValue);
}

export function useKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "candidate", idValue);
}

export function useKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "change", idValue);
}

export function useKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "conflict", idValue);
}

export function useKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "summary", idValue);
}

export function useKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "metadata", idValue);
}

export function useKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteraction", "createdAt", idValue);
}

export function useChangeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangeKitInteraction", idValue);
}

export function useChangeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "id", idValue);
}

export function useChangeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "hash", idValue);
}

export function useChangeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "index", idValue);
}

export function useChangeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "kit", idValue);
}

export function useChangeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "kind", idValue);
}

export function useChangeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "actor", idValue);
}

export function useChangeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "session", idValue);
}

export function useChangeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "transaction", idValue);
}

export function useChangeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "candidate", idValue);
}

export function useChangeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "change", idValue);
}

export function useChangeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "conflict", idValue);
}

export function useChangeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "summary", idValue);
}

export function useChangeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "metadata", idValue);
}

export function useChangeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "createdAt", idValue);
}

export function useChangeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "forward", idValue);
}

export function useChangeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangeKitInteraction", "backward", idValue);
}

export function useSetSessionSelectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SetSessionSelectionKitInteraction", idValue);
}

export function useSetSessionSelectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "id", idValue);
}

export function useSetSessionSelectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "hash", idValue);
}

export function useSetSessionSelectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "index", idValue);
}

export function useSetSessionSelectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "kit", idValue);
}

export function useSetSessionSelectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "kind", idValue);
}

export function useSetSessionSelectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "actor", idValue);
}

export function useSetSessionSelectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "session", idValue);
}

export function useSetSessionSelectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "transaction", idValue);
}

export function useSetSessionSelectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "candidate", idValue);
}

export function useSetSessionSelectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "change", idValue);
}

export function useSetSessionSelectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "conflict", idValue);
}

export function useSetSessionSelectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "summary", idValue);
}

export function useSetSessionSelectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "metadata", idValue);
}

export function useSetSessionSelectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "createdAt", idValue);
}

export function useSetSessionSelectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "forward", idValue);
}

export function useSetSessionSelectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "backward", idValue);
}

export function useSetSessionSelectionKitInteractionMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "mode", idValue);
}

export function useSetSessionSelectionKitInteractionSelection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "selection", idValue);
}

export function useSetSessionSelectionKitInteractionPreviousSelection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionKitInteraction", "previousSelection", idValue);
}

export function useCreateAuthorKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateAuthorKitInteraction", idValue);
}

export function useCreateAuthorKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "id", idValue);
}

export function useCreateAuthorKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "hash", idValue);
}

export function useCreateAuthorKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "index", idValue);
}

export function useCreateAuthorKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "kit", idValue);
}

export function useCreateAuthorKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "kind", idValue);
}

export function useCreateAuthorKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "actor", idValue);
}

export function useCreateAuthorKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "session", idValue);
}

export function useCreateAuthorKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "transaction", idValue);
}

export function useCreateAuthorKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "candidate", idValue);
}

export function useCreateAuthorKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "change", idValue);
}

export function useCreateAuthorKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "conflict", idValue);
}

export function useCreateAuthorKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "summary", idValue);
}

export function useCreateAuthorKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "metadata", idValue);
}

export function useCreateAuthorKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "createdAt", idValue);
}

export function useCreateAuthorKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "forward", idValue);
}

export function useCreateAuthorKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "backward", idValue);
}

export function useCreateAuthorKitInteractionAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorKitInteraction", "author", idValue);
}

export function useUpdateAuthorKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateAuthorKitInteraction", idValue);
}

export function useUpdateAuthorKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "id", idValue);
}

export function useUpdateAuthorKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "hash", idValue);
}

export function useUpdateAuthorKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "index", idValue);
}

export function useUpdateAuthorKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "kit", idValue);
}

export function useUpdateAuthorKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "kind", idValue);
}

export function useUpdateAuthorKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "actor", idValue);
}

export function useUpdateAuthorKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "session", idValue);
}

export function useUpdateAuthorKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "transaction", idValue);
}

export function useUpdateAuthorKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "candidate", idValue);
}

export function useUpdateAuthorKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "change", idValue);
}

export function useUpdateAuthorKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "conflict", idValue);
}

export function useUpdateAuthorKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "summary", idValue);
}

export function useUpdateAuthorKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "metadata", idValue);
}

export function useUpdateAuthorKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "createdAt", idValue);
}

export function useUpdateAuthorKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "forward", idValue);
}

export function useUpdateAuthorKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "backward", idValue);
}

export function useUpdateAuthorKitInteractionAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "author", idValue);
}

export function useUpdateAuthorKitInteractionPreviousAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorKitInteraction", "previousAuthor", idValue);
}

export function useDeleteAuthorKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteAuthorKitInteraction", idValue);
}

export function useDeleteAuthorKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "id", idValue);
}

export function useDeleteAuthorKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "hash", idValue);
}

export function useDeleteAuthorKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "index", idValue);
}

export function useDeleteAuthorKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "kit", idValue);
}

export function useDeleteAuthorKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "kind", idValue);
}

export function useDeleteAuthorKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "actor", idValue);
}

export function useDeleteAuthorKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "session", idValue);
}

export function useDeleteAuthorKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "transaction", idValue);
}

export function useDeleteAuthorKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "candidate", idValue);
}

export function useDeleteAuthorKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "change", idValue);
}

export function useDeleteAuthorKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "conflict", idValue);
}

export function useDeleteAuthorKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "summary", idValue);
}

export function useDeleteAuthorKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "metadata", idValue);
}

export function useDeleteAuthorKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "createdAt", idValue);
}

export function useDeleteAuthorKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "forward", idValue);
}

export function useDeleteAuthorKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "backward", idValue);
}

export function useDeleteAuthorKitInteractionPreviousAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorKitInteraction", "previousAuthor", idValue);
}

export function useCreateTypeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTypeKitInteraction", idValue);
}

export function useCreateTypeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "id", idValue);
}

export function useCreateTypeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "hash", idValue);
}

export function useCreateTypeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "index", idValue);
}

export function useCreateTypeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "kit", idValue);
}

export function useCreateTypeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "kind", idValue);
}

export function useCreateTypeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "actor", idValue);
}

export function useCreateTypeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "session", idValue);
}

export function useCreateTypeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "transaction", idValue);
}

export function useCreateTypeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "candidate", idValue);
}

export function useCreateTypeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "change", idValue);
}

export function useCreateTypeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "conflict", idValue);
}

export function useCreateTypeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "summary", idValue);
}

export function useCreateTypeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "metadata", idValue);
}

export function useCreateTypeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "createdAt", idValue);
}

export function useCreateTypeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "forward", idValue);
}

export function useCreateTypeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "backward", idValue);
}

export function useCreateTypeKitInteractionType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeKitInteraction", "type", idValue);
}

export function useUpdateTypeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTypeKitInteraction", idValue);
}

export function useUpdateTypeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "id", idValue);
}

export function useUpdateTypeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "hash", idValue);
}

export function useUpdateTypeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "index", idValue);
}

export function useUpdateTypeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "kit", idValue);
}

export function useUpdateTypeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "kind", idValue);
}

export function useUpdateTypeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "actor", idValue);
}

export function useUpdateTypeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "session", idValue);
}

export function useUpdateTypeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "transaction", idValue);
}

export function useUpdateTypeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "candidate", idValue);
}

export function useUpdateTypeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "change", idValue);
}

export function useUpdateTypeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "conflict", idValue);
}

export function useUpdateTypeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "summary", idValue);
}

export function useUpdateTypeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "metadata", idValue);
}

export function useUpdateTypeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "createdAt", idValue);
}

export function useUpdateTypeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "forward", idValue);
}

export function useUpdateTypeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "backward", idValue);
}

export function useUpdateTypeKitInteractionType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "type", idValue);
}

export function useUpdateTypeKitInteractionPreviousType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeKitInteraction", "previousType", idValue);
}

export function useDeleteTypeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTypeKitInteraction", idValue);
}

export function useDeleteTypeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "id", idValue);
}

export function useDeleteTypeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "hash", idValue);
}

export function useDeleteTypeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "index", idValue);
}

export function useDeleteTypeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "kit", idValue);
}

export function useDeleteTypeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "kind", idValue);
}

export function useDeleteTypeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "actor", idValue);
}

export function useDeleteTypeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "session", idValue);
}

export function useDeleteTypeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "transaction", idValue);
}

export function useDeleteTypeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "candidate", idValue);
}

export function useDeleteTypeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "change", idValue);
}

export function useDeleteTypeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "conflict", idValue);
}

export function useDeleteTypeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "summary", idValue);
}

export function useDeleteTypeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "metadata", idValue);
}

export function useDeleteTypeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "createdAt", idValue);
}

export function useDeleteTypeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "forward", idValue);
}

export function useDeleteTypeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "backward", idValue);
}

export function useDeleteTypeKitInteractionPreviousType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeKitInteraction", "previousType", idValue);
}

export function useCreateDesignKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateDesignKitInteraction", idValue);
}

export function useCreateDesignKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "id", idValue);
}

export function useCreateDesignKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "hash", idValue);
}

export function useCreateDesignKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "index", idValue);
}

export function useCreateDesignKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "kit", idValue);
}

export function useCreateDesignKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "kind", idValue);
}

export function useCreateDesignKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "actor", idValue);
}

export function useCreateDesignKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "session", idValue);
}

export function useCreateDesignKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "transaction", idValue);
}

export function useCreateDesignKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "candidate", idValue);
}

export function useCreateDesignKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "change", idValue);
}

export function useCreateDesignKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "conflict", idValue);
}

export function useCreateDesignKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "summary", idValue);
}

export function useCreateDesignKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "metadata", idValue);
}

export function useCreateDesignKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "createdAt", idValue);
}

export function useCreateDesignKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "forward", idValue);
}

export function useCreateDesignKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "backward", idValue);
}

export function useCreateDesignKitInteractionDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignKitInteraction", "design", idValue);
}

export function useUpdateDesignKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateDesignKitInteraction", idValue);
}

export function useUpdateDesignKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "id", idValue);
}

export function useUpdateDesignKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "hash", idValue);
}

export function useUpdateDesignKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "index", idValue);
}

export function useUpdateDesignKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "kit", idValue);
}

export function useUpdateDesignKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "kind", idValue);
}

export function useUpdateDesignKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "actor", idValue);
}

export function useUpdateDesignKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "session", idValue);
}

export function useUpdateDesignKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "transaction", idValue);
}

export function useUpdateDesignKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "candidate", idValue);
}

export function useUpdateDesignKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "change", idValue);
}

export function useUpdateDesignKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "conflict", idValue);
}

export function useUpdateDesignKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "summary", idValue);
}

export function useUpdateDesignKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "metadata", idValue);
}

export function useUpdateDesignKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "createdAt", idValue);
}

export function useUpdateDesignKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "forward", idValue);
}

export function useUpdateDesignKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "backward", idValue);
}

export function useUpdateDesignKitInteractionDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "design", idValue);
}

export function useUpdateDesignKitInteractionPreviousDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignKitInteraction", "previousDesign", idValue);
}

export function useDeleteDesignKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteDesignKitInteraction", idValue);
}

export function useDeleteDesignKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "id", idValue);
}

export function useDeleteDesignKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "hash", idValue);
}

export function useDeleteDesignKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "index", idValue);
}

export function useDeleteDesignKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "kit", idValue);
}

export function useDeleteDesignKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "kind", idValue);
}

export function useDeleteDesignKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "actor", idValue);
}

export function useDeleteDesignKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "session", idValue);
}

export function useDeleteDesignKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "transaction", idValue);
}

export function useDeleteDesignKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "candidate", idValue);
}

export function useDeleteDesignKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "change", idValue);
}

export function useDeleteDesignKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "conflict", idValue);
}

export function useDeleteDesignKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "summary", idValue);
}

export function useDeleteDesignKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "metadata", idValue);
}

export function useDeleteDesignKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "createdAt", idValue);
}

export function useDeleteDesignKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "forward", idValue);
}

export function useDeleteDesignKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "backward", idValue);
}

export function useDeleteDesignKitInteractionPreviousDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignKitInteraction", "previousDesign", idValue);
}

export function useCreateQualityKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateQualityKitInteraction", idValue);
}

export function useCreateQualityKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "id", idValue);
}

export function useCreateQualityKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "hash", idValue);
}

export function useCreateQualityKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "index", idValue);
}

export function useCreateQualityKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "kit", idValue);
}

export function useCreateQualityKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "kind", idValue);
}

export function useCreateQualityKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "actor", idValue);
}

export function useCreateQualityKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "session", idValue);
}

export function useCreateQualityKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "transaction", idValue);
}

export function useCreateQualityKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "candidate", idValue);
}

export function useCreateQualityKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "change", idValue);
}

export function useCreateQualityKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "conflict", idValue);
}

export function useCreateQualityKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "summary", idValue);
}

export function useCreateQualityKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "metadata", idValue);
}

export function useCreateQualityKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "createdAt", idValue);
}

export function useCreateQualityKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "forward", idValue);
}

export function useCreateQualityKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "backward", idValue);
}

export function useCreateQualityKitInteractionQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityKitInteraction", "quality", idValue);
}

export function useUpdateQualityKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateQualityKitInteraction", idValue);
}

export function useUpdateQualityKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "id", idValue);
}

export function useUpdateQualityKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "hash", idValue);
}

export function useUpdateQualityKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "index", idValue);
}

export function useUpdateQualityKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "kit", idValue);
}

export function useUpdateQualityKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "kind", idValue);
}

export function useUpdateQualityKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "actor", idValue);
}

export function useUpdateQualityKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "session", idValue);
}

export function useUpdateQualityKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "transaction", idValue);
}

export function useUpdateQualityKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "candidate", idValue);
}

export function useUpdateQualityKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "change", idValue);
}

export function useUpdateQualityKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "conflict", idValue);
}

export function useUpdateQualityKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "summary", idValue);
}

export function useUpdateQualityKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "metadata", idValue);
}

export function useUpdateQualityKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "createdAt", idValue);
}

export function useUpdateQualityKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "forward", idValue);
}

export function useUpdateQualityKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "backward", idValue);
}

export function useUpdateQualityKitInteractionQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "quality", idValue);
}

export function useUpdateQualityKitInteractionPreviousQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityKitInteraction", "previousQuality", idValue);
}

export function useDeleteQualityKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteQualityKitInteraction", idValue);
}

export function useDeleteQualityKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "id", idValue);
}

export function useDeleteQualityKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "hash", idValue);
}

export function useDeleteQualityKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "index", idValue);
}

export function useDeleteQualityKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "kit", idValue);
}

export function useDeleteQualityKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "kind", idValue);
}

export function useDeleteQualityKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "actor", idValue);
}

export function useDeleteQualityKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "session", idValue);
}

export function useDeleteQualityKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "transaction", idValue);
}

export function useDeleteQualityKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "candidate", idValue);
}

export function useDeleteQualityKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "change", idValue);
}

export function useDeleteQualityKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "conflict", idValue);
}

export function useDeleteQualityKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "summary", idValue);
}

export function useDeleteQualityKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "metadata", idValue);
}

export function useDeleteQualityKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "createdAt", idValue);
}

export function useDeleteQualityKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "forward", idValue);
}

export function useDeleteQualityKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "backward", idValue);
}

export function useDeleteQualityKitInteractionPreviousQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityKitInteraction", "previousQuality", idValue);
}

export function useCreatePortKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePortKitInteraction", idValue);
}

export function useCreatePortKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "id", idValue);
}

export function useCreatePortKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "hash", idValue);
}

export function useCreatePortKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "index", idValue);
}

export function useCreatePortKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "kit", idValue);
}

export function useCreatePortKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "kind", idValue);
}

export function useCreatePortKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "actor", idValue);
}

export function useCreatePortKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "session", idValue);
}

export function useCreatePortKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "transaction", idValue);
}

export function useCreatePortKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "candidate", idValue);
}

export function useCreatePortKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "change", idValue);
}

export function useCreatePortKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "conflict", idValue);
}

export function useCreatePortKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "summary", idValue);
}

export function useCreatePortKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "metadata", idValue);
}

export function useCreatePortKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "createdAt", idValue);
}

export function useCreatePortKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "forward", idValue);
}

export function useCreatePortKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "backward", idValue);
}

export function useCreatePortKitInteractionPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortKitInteraction", "port", idValue);
}

export function useUpdatePortKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePortKitInteraction", idValue);
}

export function useUpdatePortKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "id", idValue);
}

export function useUpdatePortKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "hash", idValue);
}

export function useUpdatePortKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "index", idValue);
}

export function useUpdatePortKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "kit", idValue);
}

export function useUpdatePortKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "kind", idValue);
}

export function useUpdatePortKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "actor", idValue);
}

export function useUpdatePortKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "session", idValue);
}

export function useUpdatePortKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "transaction", idValue);
}

export function useUpdatePortKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "candidate", idValue);
}

export function useUpdatePortKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "change", idValue);
}

export function useUpdatePortKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "conflict", idValue);
}

export function useUpdatePortKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "summary", idValue);
}

export function useUpdatePortKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "metadata", idValue);
}

export function useUpdatePortKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "createdAt", idValue);
}

export function useUpdatePortKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "forward", idValue);
}

export function useUpdatePortKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "backward", idValue);
}

export function useUpdatePortKitInteractionPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "port", idValue);
}

export function useUpdatePortKitInteractionPreviousPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortKitInteraction", "previousPort", idValue);
}

export function useDeletePortKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePortKitInteraction", idValue);
}

export function useDeletePortKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "id", idValue);
}

export function useDeletePortKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "hash", idValue);
}

export function useDeletePortKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "index", idValue);
}

export function useDeletePortKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "kit", idValue);
}

export function useDeletePortKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "kind", idValue);
}

export function useDeletePortKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "actor", idValue);
}

export function useDeletePortKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "session", idValue);
}

export function useDeletePortKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "transaction", idValue);
}

export function useDeletePortKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "candidate", idValue);
}

export function useDeletePortKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "change", idValue);
}

export function useDeletePortKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "conflict", idValue);
}

export function useDeletePortKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "summary", idValue);
}

export function useDeletePortKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "metadata", idValue);
}

export function useDeletePortKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "createdAt", idValue);
}

export function useDeletePortKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "forward", idValue);
}

export function useDeletePortKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "backward", idValue);
}

export function useDeletePortKitInteractionPreviousPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortKitInteraction", "previousPort", idValue);
}

export function useCreateFamilyKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFamilyKitInteraction", idValue);
}

export function useCreateFamilyKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "id", idValue);
}

export function useCreateFamilyKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "hash", idValue);
}

export function useCreateFamilyKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "index", idValue);
}

export function useCreateFamilyKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "kit", idValue);
}

export function useCreateFamilyKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "kind", idValue);
}

export function useCreateFamilyKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "actor", idValue);
}

export function useCreateFamilyKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "session", idValue);
}

export function useCreateFamilyKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "transaction", idValue);
}

export function useCreateFamilyKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "candidate", idValue);
}

export function useCreateFamilyKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "change", idValue);
}

export function useCreateFamilyKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "conflict", idValue);
}

export function useCreateFamilyKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "summary", idValue);
}

export function useCreateFamilyKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "metadata", idValue);
}

export function useCreateFamilyKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "createdAt", idValue);
}

export function useCreateFamilyKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "forward", idValue);
}

export function useCreateFamilyKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "backward", idValue);
}

export function useCreateFamilyKitInteractionFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyKitInteraction", "family", idValue);
}

export function useUpdateFamilyKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFamilyKitInteraction", idValue);
}

export function useUpdateFamilyKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "id", idValue);
}

export function useUpdateFamilyKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "hash", idValue);
}

export function useUpdateFamilyKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "index", idValue);
}

export function useUpdateFamilyKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "kit", idValue);
}

export function useUpdateFamilyKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "kind", idValue);
}

export function useUpdateFamilyKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "actor", idValue);
}

export function useUpdateFamilyKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "session", idValue);
}

export function useUpdateFamilyKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "transaction", idValue);
}

export function useUpdateFamilyKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "candidate", idValue);
}

export function useUpdateFamilyKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "change", idValue);
}

export function useUpdateFamilyKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "conflict", idValue);
}

export function useUpdateFamilyKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "summary", idValue);
}

export function useUpdateFamilyKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "metadata", idValue);
}

export function useUpdateFamilyKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "createdAt", idValue);
}

export function useUpdateFamilyKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "forward", idValue);
}

export function useUpdateFamilyKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "backward", idValue);
}

export function useUpdateFamilyKitInteractionFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "family", idValue);
}

export function useUpdateFamilyKitInteractionPreviousFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyKitInteraction", "previousFamily", idValue);
}

export function useDeleteFamilyKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFamilyKitInteraction", idValue);
}

export function useDeleteFamilyKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "id", idValue);
}

export function useDeleteFamilyKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "hash", idValue);
}

export function useDeleteFamilyKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "index", idValue);
}

export function useDeleteFamilyKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "kit", idValue);
}

export function useDeleteFamilyKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "kind", idValue);
}

export function useDeleteFamilyKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "actor", idValue);
}

export function useDeleteFamilyKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "session", idValue);
}

export function useDeleteFamilyKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "transaction", idValue);
}

export function useDeleteFamilyKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "candidate", idValue);
}

export function useDeleteFamilyKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "change", idValue);
}

export function useDeleteFamilyKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "conflict", idValue);
}

export function useDeleteFamilyKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "summary", idValue);
}

export function useDeleteFamilyKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "metadata", idValue);
}

export function useDeleteFamilyKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "createdAt", idValue);
}

export function useDeleteFamilyKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "forward", idValue);
}

export function useDeleteFamilyKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "backward", idValue);
}

export function useDeleteFamilyKitInteractionPreviousFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyKitInteraction", "previousFamily", idValue);
}

export function useCreateTagKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTagKitInteraction", idValue);
}

export function useCreateTagKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "id", idValue);
}

export function useCreateTagKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "hash", idValue);
}

export function useCreateTagKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "index", idValue);
}

export function useCreateTagKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "kit", idValue);
}

export function useCreateTagKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "kind", idValue);
}

export function useCreateTagKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "actor", idValue);
}

export function useCreateTagKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "session", idValue);
}

export function useCreateTagKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "transaction", idValue);
}

export function useCreateTagKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "candidate", idValue);
}

export function useCreateTagKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "change", idValue);
}

export function useCreateTagKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "conflict", idValue);
}

export function useCreateTagKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "summary", idValue);
}

export function useCreateTagKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "metadata", idValue);
}

export function useCreateTagKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "createdAt", idValue);
}

export function useCreateTagKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "forward", idValue);
}

export function useCreateTagKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "backward", idValue);
}

export function useCreateTagKitInteractionTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagKitInteraction", "tag", idValue);
}

export function useUpdateTagKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTagKitInteraction", idValue);
}

export function useUpdateTagKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "id", idValue);
}

export function useUpdateTagKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "hash", idValue);
}

export function useUpdateTagKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "index", idValue);
}

export function useUpdateTagKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "kit", idValue);
}

export function useUpdateTagKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "kind", idValue);
}

export function useUpdateTagKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "actor", idValue);
}

export function useUpdateTagKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "session", idValue);
}

export function useUpdateTagKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "transaction", idValue);
}

export function useUpdateTagKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "candidate", idValue);
}

export function useUpdateTagKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "change", idValue);
}

export function useUpdateTagKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "conflict", idValue);
}

export function useUpdateTagKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "summary", idValue);
}

export function useUpdateTagKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "metadata", idValue);
}

export function useUpdateTagKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "createdAt", idValue);
}

export function useUpdateTagKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "forward", idValue);
}

export function useUpdateTagKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "backward", idValue);
}

export function useUpdateTagKitInteractionTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "tag", idValue);
}

export function useUpdateTagKitInteractionPreviousTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagKitInteraction", "previousTag", idValue);
}

export function useDeleteTagKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTagKitInteraction", idValue);
}

export function useDeleteTagKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "id", idValue);
}

export function useDeleteTagKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "hash", idValue);
}

export function useDeleteTagKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "index", idValue);
}

export function useDeleteTagKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "kit", idValue);
}

export function useDeleteTagKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "kind", idValue);
}

export function useDeleteTagKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "actor", idValue);
}

export function useDeleteTagKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "session", idValue);
}

export function useDeleteTagKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "transaction", idValue);
}

export function useDeleteTagKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "candidate", idValue);
}

export function useDeleteTagKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "change", idValue);
}

export function useDeleteTagKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "conflict", idValue);
}

export function useDeleteTagKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "summary", idValue);
}

export function useDeleteTagKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "metadata", idValue);
}

export function useDeleteTagKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "createdAt", idValue);
}

export function useDeleteTagKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "forward", idValue);
}

export function useDeleteTagKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "backward", idValue);
}

export function useDeleteTagKitInteractionPreviousTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagKitInteraction", "previousTag", idValue);
}

export function useCreateConceptKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConceptKitInteraction", idValue);
}

export function useCreateConceptKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "id", idValue);
}

export function useCreateConceptKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "hash", idValue);
}

export function useCreateConceptKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "index", idValue);
}

export function useCreateConceptKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "kit", idValue);
}

export function useCreateConceptKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "kind", idValue);
}

export function useCreateConceptKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "actor", idValue);
}

export function useCreateConceptKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "session", idValue);
}

export function useCreateConceptKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "transaction", idValue);
}

export function useCreateConceptKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "candidate", idValue);
}

export function useCreateConceptKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "change", idValue);
}

export function useCreateConceptKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "conflict", idValue);
}

export function useCreateConceptKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "summary", idValue);
}

export function useCreateConceptKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "metadata", idValue);
}

export function useCreateConceptKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "createdAt", idValue);
}

export function useCreateConceptKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "forward", idValue);
}

export function useCreateConceptKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "backward", idValue);
}

export function useCreateConceptKitInteractionConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptKitInteraction", "concept", idValue);
}

export function useUpdateConceptKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConceptKitInteraction", idValue);
}

export function useUpdateConceptKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "id", idValue);
}

export function useUpdateConceptKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "hash", idValue);
}

export function useUpdateConceptKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "index", idValue);
}

export function useUpdateConceptKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "kit", idValue);
}

export function useUpdateConceptKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "kind", idValue);
}

export function useUpdateConceptKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "actor", idValue);
}

export function useUpdateConceptKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "session", idValue);
}

export function useUpdateConceptKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "transaction", idValue);
}

export function useUpdateConceptKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "candidate", idValue);
}

export function useUpdateConceptKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "change", idValue);
}

export function useUpdateConceptKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "conflict", idValue);
}

export function useUpdateConceptKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "summary", idValue);
}

export function useUpdateConceptKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "metadata", idValue);
}

export function useUpdateConceptKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "createdAt", idValue);
}

export function useUpdateConceptKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "forward", idValue);
}

export function useUpdateConceptKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "backward", idValue);
}

export function useUpdateConceptKitInteractionConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "concept", idValue);
}

export function useUpdateConceptKitInteractionPreviousConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptKitInteraction", "previousConcept", idValue);
}

export function useDeleteConceptKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConceptKitInteraction", idValue);
}

export function useDeleteConceptKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "id", idValue);
}

export function useDeleteConceptKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "hash", idValue);
}

export function useDeleteConceptKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "index", idValue);
}

export function useDeleteConceptKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "kit", idValue);
}

export function useDeleteConceptKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "kind", idValue);
}

export function useDeleteConceptKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "actor", idValue);
}

export function useDeleteConceptKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "session", idValue);
}

export function useDeleteConceptKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "transaction", idValue);
}

export function useDeleteConceptKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "candidate", idValue);
}

export function useDeleteConceptKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "change", idValue);
}

export function useDeleteConceptKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "conflict", idValue);
}

export function useDeleteConceptKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "summary", idValue);
}

export function useDeleteConceptKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "metadata", idValue);
}

export function useDeleteConceptKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "createdAt", idValue);
}

export function useDeleteConceptKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "forward", idValue);
}

export function useDeleteConceptKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "backward", idValue);
}

export function useDeleteConceptKitInteractionPreviousConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptKitInteraction", "previousConcept", idValue);
}

export function useCreateFileKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFileKitInteraction", idValue);
}

export function useCreateFileKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "id", idValue);
}

export function useCreateFileKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "hash", idValue);
}

export function useCreateFileKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "index", idValue);
}

export function useCreateFileKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "kit", idValue);
}

export function useCreateFileKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "kind", idValue);
}

export function useCreateFileKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "actor", idValue);
}

export function useCreateFileKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "session", idValue);
}

export function useCreateFileKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "transaction", idValue);
}

export function useCreateFileKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "candidate", idValue);
}

export function useCreateFileKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "change", idValue);
}

export function useCreateFileKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "conflict", idValue);
}

export function useCreateFileKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "summary", idValue);
}

export function useCreateFileKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "metadata", idValue);
}

export function useCreateFileKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "createdAt", idValue);
}

export function useCreateFileKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "forward", idValue);
}

export function useCreateFileKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "backward", idValue);
}

export function useCreateFileKitInteractionFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileKitInteraction", "file", idValue);
}

export function useUpdateFileKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFileKitInteraction", idValue);
}

export function useUpdateFileKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "id", idValue);
}

export function useUpdateFileKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "hash", idValue);
}

export function useUpdateFileKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "index", idValue);
}

export function useUpdateFileKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "kit", idValue);
}

export function useUpdateFileKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "kind", idValue);
}

export function useUpdateFileKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "actor", idValue);
}

export function useUpdateFileKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "session", idValue);
}

export function useUpdateFileKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "transaction", idValue);
}

export function useUpdateFileKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "candidate", idValue);
}

export function useUpdateFileKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "change", idValue);
}

export function useUpdateFileKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "conflict", idValue);
}

export function useUpdateFileKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "summary", idValue);
}

export function useUpdateFileKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "metadata", idValue);
}

export function useUpdateFileKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "createdAt", idValue);
}

export function useUpdateFileKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "forward", idValue);
}

export function useUpdateFileKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "backward", idValue);
}

export function useUpdateFileKitInteractionFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "file", idValue);
}

export function useUpdateFileKitInteractionPreviousFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileKitInteraction", "previousFile", idValue);
}

export function useDeleteFileKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFileKitInteraction", idValue);
}

export function useDeleteFileKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "id", idValue);
}

export function useDeleteFileKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "hash", idValue);
}

export function useDeleteFileKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "index", idValue);
}

export function useDeleteFileKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "kit", idValue);
}

export function useDeleteFileKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "kind", idValue);
}

export function useDeleteFileKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "actor", idValue);
}

export function useDeleteFileKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "session", idValue);
}

export function useDeleteFileKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "transaction", idValue);
}

export function useDeleteFileKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "candidate", idValue);
}

export function useDeleteFileKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "change", idValue);
}

export function useDeleteFileKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "conflict", idValue);
}

export function useDeleteFileKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "summary", idValue);
}

export function useDeleteFileKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "metadata", idValue);
}

export function useDeleteFileKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "createdAt", idValue);
}

export function useDeleteFileKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "forward", idValue);
}

export function useDeleteFileKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "backward", idValue);
}

export function useDeleteFileKitInteractionPreviousFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileKitInteraction", "previousFile", idValue);
}

export function useCreateFolderKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFolderKitInteraction", idValue);
}

export function useCreateFolderKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "id", idValue);
}

export function useCreateFolderKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "hash", idValue);
}

export function useCreateFolderKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "index", idValue);
}

export function useCreateFolderKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "kit", idValue);
}

export function useCreateFolderKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "kind", idValue);
}

export function useCreateFolderKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "actor", idValue);
}

export function useCreateFolderKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "session", idValue);
}

export function useCreateFolderKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "transaction", idValue);
}

export function useCreateFolderKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "candidate", idValue);
}

export function useCreateFolderKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "change", idValue);
}

export function useCreateFolderKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "conflict", idValue);
}

export function useCreateFolderKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "summary", idValue);
}

export function useCreateFolderKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "metadata", idValue);
}

export function useCreateFolderKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "createdAt", idValue);
}

export function useCreateFolderKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "forward", idValue);
}

export function useCreateFolderKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "backward", idValue);
}

export function useCreateFolderKitInteractionFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderKitInteraction", "folder", idValue);
}

export function useUpdateFolderKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFolderKitInteraction", idValue);
}

export function useUpdateFolderKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "id", idValue);
}

export function useUpdateFolderKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "hash", idValue);
}

export function useUpdateFolderKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "index", idValue);
}

export function useUpdateFolderKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "kit", idValue);
}

export function useUpdateFolderKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "kind", idValue);
}

export function useUpdateFolderKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "actor", idValue);
}

export function useUpdateFolderKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "session", idValue);
}

export function useUpdateFolderKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "transaction", idValue);
}

export function useUpdateFolderKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "candidate", idValue);
}

export function useUpdateFolderKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "change", idValue);
}

export function useUpdateFolderKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "conflict", idValue);
}

export function useUpdateFolderKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "summary", idValue);
}

export function useUpdateFolderKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "metadata", idValue);
}

export function useUpdateFolderKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "createdAt", idValue);
}

export function useUpdateFolderKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "forward", idValue);
}

export function useUpdateFolderKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "backward", idValue);
}

export function useUpdateFolderKitInteractionFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "folder", idValue);
}

export function useUpdateFolderKitInteractionPreviousFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderKitInteraction", "previousFolder", idValue);
}

export function useDeleteFolderKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFolderKitInteraction", idValue);
}

export function useDeleteFolderKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "id", idValue);
}

export function useDeleteFolderKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "hash", idValue);
}

export function useDeleteFolderKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "index", idValue);
}

export function useDeleteFolderKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "kit", idValue);
}

export function useDeleteFolderKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "kind", idValue);
}

export function useDeleteFolderKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "actor", idValue);
}

export function useDeleteFolderKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "session", idValue);
}

export function useDeleteFolderKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "transaction", idValue);
}

export function useDeleteFolderKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "candidate", idValue);
}

export function useDeleteFolderKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "change", idValue);
}

export function useDeleteFolderKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "conflict", idValue);
}

export function useDeleteFolderKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "summary", idValue);
}

export function useDeleteFolderKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "metadata", idValue);
}

export function useDeleteFolderKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "createdAt", idValue);
}

export function useDeleteFolderKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "forward", idValue);
}

export function useDeleteFolderKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "backward", idValue);
}

export function useDeleteFolderKitInteractionPreviousFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderKitInteraction", "previousFolder", idValue);
}

export function useMoveArtifactToFolderKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MoveArtifactToFolderKitInteraction", idValue);
}

export function useMoveArtifactToFolderKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "id", idValue);
}

export function useMoveArtifactToFolderKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "hash", idValue);
}

export function useMoveArtifactToFolderKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "index", idValue);
}

export function useMoveArtifactToFolderKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kit", idValue);
}

export function useMoveArtifactToFolderKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "kind", idValue);
}

export function useMoveArtifactToFolderKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "actor", idValue);
}

export function useMoveArtifactToFolderKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "session", idValue);
}

export function useMoveArtifactToFolderKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "transaction", idValue);
}

export function useMoveArtifactToFolderKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "candidate", idValue);
}

export function useMoveArtifactToFolderKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "change", idValue);
}

export function useMoveArtifactToFolderKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "conflict", idValue);
}

export function useMoveArtifactToFolderKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "summary", idValue);
}

export function useMoveArtifactToFolderKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "metadata", idValue);
}

export function useMoveArtifactToFolderKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "createdAt", idValue);
}

export function useMoveArtifactToFolderKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "forward", idValue);
}

export function useMoveArtifactToFolderKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "backward", idValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactKind", idValue);
}

export function useMoveArtifactToFolderKitInteractionArtifactId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "artifactId", idValue);
}

export function useMoveArtifactToFolderKitInteractionFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "folder", idValue);
}

export function useMoveArtifactToFolderKitInteractionPreviousFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderKitInteraction", "previousFolder", idValue);
}

export function useCreatePieceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePieceKitInteraction", idValue);
}

export function useCreatePieceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "id", idValue);
}

export function useCreatePieceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "hash", idValue);
}

export function useCreatePieceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "index", idValue);
}

export function useCreatePieceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "kit", idValue);
}

export function useCreatePieceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "kind", idValue);
}

export function useCreatePieceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "actor", idValue);
}

export function useCreatePieceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "session", idValue);
}

export function useCreatePieceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "transaction", idValue);
}

export function useCreatePieceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "candidate", idValue);
}

export function useCreatePieceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "change", idValue);
}

export function useCreatePieceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "conflict", idValue);
}

export function useCreatePieceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "summary", idValue);
}

export function useCreatePieceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "metadata", idValue);
}

export function useCreatePieceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "createdAt", idValue);
}

export function useCreatePieceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "forward", idValue);
}

export function useCreatePieceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceKitInteraction", "backward", idValue);
}

export function useCreatePiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePiecesKitInteraction", idValue);
}

export function useCreatePiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "id", idValue);
}

export function useCreatePiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "hash", idValue);
}

export function useCreatePiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "index", idValue);
}

export function useCreatePiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "kit", idValue);
}

export function useCreatePiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "kind", idValue);
}

export function useCreatePiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "actor", idValue);
}

export function useCreatePiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "session", idValue);
}

export function useCreatePiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "transaction", idValue);
}

export function useCreatePiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "candidate", idValue);
}

export function useCreatePiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "change", idValue);
}

export function useCreatePiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "conflict", idValue);
}

export function useCreatePiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "summary", idValue);
}

export function useCreatePiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "metadata", idValue);
}

export function useCreatePiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "createdAt", idValue);
}

export function useCreatePiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "forward", idValue);
}

export function useCreatePiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesKitInteraction", "backward", idValue);
}

export function useUpdatePieceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePieceKitInteraction", idValue);
}

export function useUpdatePieceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "id", idValue);
}

export function useUpdatePieceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "hash", idValue);
}

export function useUpdatePieceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "index", idValue);
}

export function useUpdatePieceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "kit", idValue);
}

export function useUpdatePieceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "kind", idValue);
}

export function useUpdatePieceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "actor", idValue);
}

export function useUpdatePieceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "session", idValue);
}

export function useUpdatePieceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "transaction", idValue);
}

export function useUpdatePieceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "candidate", idValue);
}

export function useUpdatePieceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "change", idValue);
}

export function useUpdatePieceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "conflict", idValue);
}

export function useUpdatePieceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "summary", idValue);
}

export function useUpdatePieceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "metadata", idValue);
}

export function useUpdatePieceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "createdAt", idValue);
}

export function useUpdatePieceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "forward", idValue);
}

export function useUpdatePieceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceKitInteraction", "backward", idValue);
}

export function useUpdatePiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePiecesKitInteraction", idValue);
}

export function useUpdatePiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "id", idValue);
}

export function useUpdatePiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "hash", idValue);
}

export function useUpdatePiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "index", idValue);
}

export function useUpdatePiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "kit", idValue);
}

export function useUpdatePiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "kind", idValue);
}

export function useUpdatePiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "actor", idValue);
}

export function useUpdatePiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "session", idValue);
}

export function useUpdatePiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "transaction", idValue);
}

export function useUpdatePiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "candidate", idValue);
}

export function useUpdatePiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "change", idValue);
}

export function useUpdatePiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "conflict", idValue);
}

export function useUpdatePiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "summary", idValue);
}

export function useUpdatePiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "metadata", idValue);
}

export function useUpdatePiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "createdAt", idValue);
}

export function useUpdatePiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "forward", idValue);
}

export function useUpdatePiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesKitInteraction", "backward", idValue);
}

export function useDeletePieceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePieceKitInteraction", idValue);
}

export function useDeletePieceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "id", idValue);
}

export function useDeletePieceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "hash", idValue);
}

export function useDeletePieceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "index", idValue);
}

export function useDeletePieceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "kit", idValue);
}

export function useDeletePieceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "kind", idValue);
}

export function useDeletePieceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "actor", idValue);
}

export function useDeletePieceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "session", idValue);
}

export function useDeletePieceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "transaction", idValue);
}

export function useDeletePieceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "candidate", idValue);
}

export function useDeletePieceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "change", idValue);
}

export function useDeletePieceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "conflict", idValue);
}

export function useDeletePieceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "summary", idValue);
}

export function useDeletePieceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "metadata", idValue);
}

export function useDeletePieceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "createdAt", idValue);
}

export function useDeletePieceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "forward", idValue);
}

export function useDeletePieceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceKitInteraction", "backward", idValue);
}

export function useDeletePiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePiecesKitInteraction", idValue);
}

export function useDeletePiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "id", idValue);
}

export function useDeletePiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "hash", idValue);
}

export function useDeletePiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "index", idValue);
}

export function useDeletePiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "kit", idValue);
}

export function useDeletePiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "kind", idValue);
}

export function useDeletePiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "actor", idValue);
}

export function useDeletePiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "session", idValue);
}

export function useDeletePiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "transaction", idValue);
}

export function useDeletePiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "candidate", idValue);
}

export function useDeletePiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "change", idValue);
}

export function useDeletePiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "conflict", idValue);
}

export function useDeletePiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "summary", idValue);
}

export function useDeletePiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "metadata", idValue);
}

export function useDeletePiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "createdAt", idValue);
}

export function useDeletePiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "forward", idValue);
}

export function useDeletePiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesKitInteraction", "backward", idValue);
}

export function useCreateConnectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionKitInteraction", idValue);
}

export function useCreateConnectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "id", idValue);
}

export function useCreateConnectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "hash", idValue);
}

export function useCreateConnectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "index", idValue);
}

export function useCreateConnectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "kit", idValue);
}

export function useCreateConnectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "kind", idValue);
}

export function useCreateConnectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "actor", idValue);
}

export function useCreateConnectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "session", idValue);
}

export function useCreateConnectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "transaction", idValue);
}

export function useCreateConnectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "candidate", idValue);
}

export function useCreateConnectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "change", idValue);
}

export function useCreateConnectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "conflict", idValue);
}

export function useCreateConnectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "summary", idValue);
}

export function useCreateConnectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "metadata", idValue);
}

export function useCreateConnectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "createdAt", idValue);
}

export function useCreateConnectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "forward", idValue);
}

export function useCreateConnectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionKitInteraction", "backward", idValue);
}

export function useCreateConnectionsKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionsKitInteraction", idValue);
}

export function useCreateConnectionsKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "id", idValue);
}

export function useCreateConnectionsKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "hash", idValue);
}

export function useCreateConnectionsKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "index", idValue);
}

export function useCreateConnectionsKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "kit", idValue);
}

export function useCreateConnectionsKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "kind", idValue);
}

export function useCreateConnectionsKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "actor", idValue);
}

export function useCreateConnectionsKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "session", idValue);
}

export function useCreateConnectionsKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "transaction", idValue);
}

export function useCreateConnectionsKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "candidate", idValue);
}

export function useCreateConnectionsKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "change", idValue);
}

export function useCreateConnectionsKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "conflict", idValue);
}

export function useCreateConnectionsKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "summary", idValue);
}

export function useCreateConnectionsKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "metadata", idValue);
}

export function useCreateConnectionsKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "createdAt", idValue);
}

export function useCreateConnectionsKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "forward", idValue);
}

export function useCreateConnectionsKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsKitInteraction", "backward", idValue);
}

export function useUpdateConnectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionKitInteraction", idValue);
}

export function useUpdateConnectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "id", idValue);
}

export function useUpdateConnectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "hash", idValue);
}

export function useUpdateConnectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "index", idValue);
}

export function useUpdateConnectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "kit", idValue);
}

export function useUpdateConnectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "kind", idValue);
}

export function useUpdateConnectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "actor", idValue);
}

export function useUpdateConnectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "session", idValue);
}

export function useUpdateConnectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "transaction", idValue);
}

export function useUpdateConnectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "candidate", idValue);
}

export function useUpdateConnectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "change", idValue);
}

export function useUpdateConnectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "conflict", idValue);
}

export function useUpdateConnectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "summary", idValue);
}

export function useUpdateConnectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "metadata", idValue);
}

export function useUpdateConnectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "createdAt", idValue);
}

export function useUpdateConnectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "forward", idValue);
}

export function useUpdateConnectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionKitInteraction", "backward", idValue);
}

export function useUpdateConnectionsKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionsKitInteraction", idValue);
}

export function useUpdateConnectionsKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "id", idValue);
}

export function useUpdateConnectionsKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "hash", idValue);
}

export function useUpdateConnectionsKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "index", idValue);
}

export function useUpdateConnectionsKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "kit", idValue);
}

export function useUpdateConnectionsKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "kind", idValue);
}

export function useUpdateConnectionsKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "actor", idValue);
}

export function useUpdateConnectionsKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "session", idValue);
}

export function useUpdateConnectionsKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "transaction", idValue);
}

export function useUpdateConnectionsKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "candidate", idValue);
}

export function useUpdateConnectionsKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "change", idValue);
}

export function useUpdateConnectionsKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "conflict", idValue);
}

export function useUpdateConnectionsKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "summary", idValue);
}

export function useUpdateConnectionsKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "metadata", idValue);
}

export function useUpdateConnectionsKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "createdAt", idValue);
}

export function useUpdateConnectionsKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "forward", idValue);
}

export function useUpdateConnectionsKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsKitInteraction", "backward", idValue);
}

export function useDeleteConnectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionKitInteraction", idValue);
}

export function useDeleteConnectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "id", idValue);
}

export function useDeleteConnectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "hash", idValue);
}

export function useDeleteConnectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "index", idValue);
}

export function useDeleteConnectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "kit", idValue);
}

export function useDeleteConnectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "kind", idValue);
}

export function useDeleteConnectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "actor", idValue);
}

export function useDeleteConnectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "session", idValue);
}

export function useDeleteConnectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "transaction", idValue);
}

export function useDeleteConnectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "candidate", idValue);
}

export function useDeleteConnectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "change", idValue);
}

export function useDeleteConnectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "conflict", idValue);
}

export function useDeleteConnectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "summary", idValue);
}

export function useDeleteConnectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "metadata", idValue);
}

export function useDeleteConnectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "createdAt", idValue);
}

export function useDeleteConnectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "forward", idValue);
}

export function useDeleteConnectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionKitInteraction", "backward", idValue);
}

export function useDeleteConnectionsKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionsKitInteraction", idValue);
}

export function useDeleteConnectionsKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "id", idValue);
}

export function useDeleteConnectionsKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "hash", idValue);
}

export function useDeleteConnectionsKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "index", idValue);
}

export function useDeleteConnectionsKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "kit", idValue);
}

export function useDeleteConnectionsKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "kind", idValue);
}

export function useDeleteConnectionsKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "actor", idValue);
}

export function useDeleteConnectionsKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "session", idValue);
}

export function useDeleteConnectionsKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "transaction", idValue);
}

export function useDeleteConnectionsKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "candidate", idValue);
}

export function useDeleteConnectionsKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "change", idValue);
}

export function useDeleteConnectionsKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "conflict", idValue);
}

export function useDeleteConnectionsKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "summary", idValue);
}

export function useDeleteConnectionsKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "metadata", idValue);
}

export function useDeleteConnectionsKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "createdAt", idValue);
}

export function useDeleteConnectionsKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "forward", idValue);
}

export function useDeleteConnectionsKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsKitInteraction", "backward", idValue);
}

export function useDeleteSelectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteSelectionKitInteraction", idValue);
}

export function useDeleteSelectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "id", idValue);
}

export function useDeleteSelectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "hash", idValue);
}

export function useDeleteSelectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "index", idValue);
}

export function useDeleteSelectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "kit", idValue);
}

export function useDeleteSelectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "kind", idValue);
}

export function useDeleteSelectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "actor", idValue);
}

export function useDeleteSelectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "session", idValue);
}

export function useDeleteSelectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "transaction", idValue);
}

export function useDeleteSelectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "candidate", idValue);
}

export function useDeleteSelectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "change", idValue);
}

export function useDeleteSelectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "conflict", idValue);
}

export function useDeleteSelectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "summary", idValue);
}

export function useDeleteSelectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "metadata", idValue);
}

export function useDeleteSelectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "createdAt", idValue);
}

export function useDeleteSelectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "forward", idValue);
}

export function useDeleteSelectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionKitInteraction", "backward", idValue);
}

export function useFixPiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FixPiecesKitInteraction", idValue);
}

export function useFixPiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "id", idValue);
}

export function useFixPiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "hash", idValue);
}

export function useFixPiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "index", idValue);
}

export function useFixPiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "kit", idValue);
}

export function useFixPiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "kind", idValue);
}

export function useFixPiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "actor", idValue);
}

export function useFixPiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "session", idValue);
}

export function useFixPiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "transaction", idValue);
}

export function useFixPiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "candidate", idValue);
}

export function useFixPiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "change", idValue);
}

export function useFixPiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "conflict", idValue);
}

export function useFixPiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "summary", idValue);
}

export function useFixPiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "metadata", idValue);
}

export function useFixPiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "createdAt", idValue);
}

export function useFixPiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "forward", idValue);
}

export function useFixPiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesKitInteraction", "backward", idValue);
}

export function useClusterPiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ClusterPiecesKitInteraction", idValue);
}

export function useClusterPiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "id", idValue);
}

export function useClusterPiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "hash", idValue);
}

export function useClusterPiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "index", idValue);
}

export function useClusterPiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "kit", idValue);
}

export function useClusterPiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "kind", idValue);
}

export function useClusterPiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "actor", idValue);
}

export function useClusterPiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "session", idValue);
}

export function useClusterPiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "transaction", idValue);
}

export function useClusterPiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "candidate", idValue);
}

export function useClusterPiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "change", idValue);
}

export function useClusterPiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "conflict", idValue);
}

export function useClusterPiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "summary", idValue);
}

export function useClusterPiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "metadata", idValue);
}

export function useClusterPiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "createdAt", idValue);
}

export function useClusterPiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "forward", idValue);
}

export function useClusterPiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesKitInteraction", "backward", idValue);
}

export function useExpandDesignReferenceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExpandDesignReferenceKitInteraction", idValue);
}

export function useExpandDesignReferenceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "id", idValue);
}

export function useExpandDesignReferenceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "hash", idValue);
}

export function useExpandDesignReferenceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "index", idValue);
}

export function useExpandDesignReferenceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kit", idValue);
}

export function useExpandDesignReferenceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "kind", idValue);
}

export function useExpandDesignReferenceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "actor", idValue);
}

export function useExpandDesignReferenceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "session", idValue);
}

export function useExpandDesignReferenceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "transaction", idValue);
}

export function useExpandDesignReferenceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "candidate", idValue);
}

export function useExpandDesignReferenceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "change", idValue);
}

export function useExpandDesignReferenceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "conflict", idValue);
}

export function useExpandDesignReferenceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "summary", idValue);
}

export function useExpandDesignReferenceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "metadata", idValue);
}

export function useExpandDesignReferenceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "createdAt", idValue);
}

export function useExpandDesignReferenceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "forward", idValue);
}

export function useExpandDesignReferenceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceKitInteraction", "backward", idValue);
}

export function useFlattenDesignKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FlattenDesignKitInteraction", idValue);
}

export function useFlattenDesignKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "id", idValue);
}

export function useFlattenDesignKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "hash", idValue);
}

export function useFlattenDesignKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "index", idValue);
}

export function useFlattenDesignKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "kit", idValue);
}

export function useFlattenDesignKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "kind", idValue);
}

export function useFlattenDesignKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "actor", idValue);
}

export function useFlattenDesignKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "session", idValue);
}

export function useFlattenDesignKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "transaction", idValue);
}

export function useFlattenDesignKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "candidate", idValue);
}

export function useFlattenDesignKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "change", idValue);
}

export function useFlattenDesignKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "conflict", idValue);
}

export function useFlattenDesignKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "summary", idValue);
}

export function useFlattenDesignKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "metadata", idValue);
}

export function useFlattenDesignKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "createdAt", idValue);
}

export function useFlattenDesignKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "forward", idValue);
}

export function useFlattenDesignKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignKitInteraction", "backward", idValue);
}

export function useDragPiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DragPiecesKitInteraction", idValue);
}

export function useDragPiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "id", idValue);
}

export function useDragPiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "hash", idValue);
}

export function useDragPiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "index", idValue);
}

export function useDragPiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "kit", idValue);
}

export function useDragPiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "kind", idValue);
}

export function useDragPiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "actor", idValue);
}

export function useDragPiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "session", idValue);
}

export function useDragPiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "transaction", idValue);
}

export function useDragPiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "candidate", idValue);
}

export function useDragPiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "change", idValue);
}

export function useDragPiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "conflict", idValue);
}

export function useDragPiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "summary", idValue);
}

export function useDragPiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "metadata", idValue);
}

export function useDragPiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "createdAt", idValue);
}

export function useDragPiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "forward", idValue);
}

export function useDragPiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesKitInteraction", "backward", idValue);
}

export function useMovePiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesKitInteraction", idValue);
}

export function useMovePiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "id", idValue);
}

export function useMovePiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "hash", idValue);
}

export function useMovePiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "index", idValue);
}

export function useMovePiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "kit", idValue);
}

export function useMovePiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "kind", idValue);
}

export function useMovePiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "actor", idValue);
}

export function useMovePiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "session", idValue);
}

export function useMovePiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "transaction", idValue);
}

export function useMovePiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "candidate", idValue);
}

export function useMovePiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "change", idValue);
}

export function useMovePiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "conflict", idValue);
}

export function useMovePiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "summary", idValue);
}

export function useMovePiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "metadata", idValue);
}

export function useMovePiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "createdAt", idValue);
}

export function useMovePiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "forward", idValue);
}

export function useMovePiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesKitInteraction", "backward", idValue);
}

export function useCreateFixedPieceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFixedPieceKitInteraction", idValue);
}

export function useCreateFixedPieceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "id", idValue);
}

export function useCreateFixedPieceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "hash", idValue);
}

export function useCreateFixedPieceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "index", idValue);
}

export function useCreateFixedPieceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "kit", idValue);
}

export function useCreateFixedPieceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "kind", idValue);
}

export function useCreateFixedPieceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "actor", idValue);
}

export function useCreateFixedPieceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "session", idValue);
}

export function useCreateFixedPieceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "transaction", idValue);
}

export function useCreateFixedPieceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "candidate", idValue);
}

export function useCreateFixedPieceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "change", idValue);
}

export function useCreateFixedPieceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "conflict", idValue);
}

export function useCreateFixedPieceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "summary", idValue);
}

export function useCreateFixedPieceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "metadata", idValue);
}

export function useCreateFixedPieceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "createdAt", idValue);
}

export function useCreateFixedPieceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "forward", idValue);
}

export function useCreateFixedPieceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceKitInteraction", "backward", idValue);
}

export function useCreateConnectedPieceKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectedPieceKitInteraction", idValue);
}

export function useCreateConnectedPieceKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "id", idValue);
}

export function useCreateConnectedPieceKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "hash", idValue);
}

export function useCreateConnectedPieceKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "index", idValue);
}

export function useCreateConnectedPieceKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kit", idValue);
}

export function useCreateConnectedPieceKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "kind", idValue);
}

export function useCreateConnectedPieceKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "actor", idValue);
}

export function useCreateConnectedPieceKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "session", idValue);
}

export function useCreateConnectedPieceKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "transaction", idValue);
}

export function useCreateConnectedPieceKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "candidate", idValue);
}

export function useCreateConnectedPieceKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "change", idValue);
}

export function useCreateConnectedPieceKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "conflict", idValue);
}

export function useCreateConnectedPieceKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "summary", idValue);
}

export function useCreateConnectedPieceKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "metadata", idValue);
}

export function useCreateConnectedPieceKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "createdAt", idValue);
}

export function useCreateConnectedPieceKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "forward", idValue);
}

export function useCreateConnectedPieceKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceKitInteraction", "backward", idValue);
}

export function useCreateHangingPiecesKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateHangingPiecesKitInteraction", idValue);
}

export function useCreateHangingPiecesKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "id", idValue);
}

export function useCreateHangingPiecesKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "hash", idValue);
}

export function useCreateHangingPiecesKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "index", idValue);
}

export function useCreateHangingPiecesKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kit", idValue);
}

export function useCreateHangingPiecesKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "kind", idValue);
}

export function useCreateHangingPiecesKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "actor", idValue);
}

export function useCreateHangingPiecesKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "session", idValue);
}

export function useCreateHangingPiecesKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "transaction", idValue);
}

export function useCreateHangingPiecesKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "candidate", idValue);
}

export function useCreateHangingPiecesKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "change", idValue);
}

export function useCreateHangingPiecesKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "conflict", idValue);
}

export function useCreateHangingPiecesKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "summary", idValue);
}

export function useCreateHangingPiecesKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "metadata", idValue);
}

export function useCreateHangingPiecesKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "createdAt", idValue);
}

export function useCreateHangingPiecesKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "forward", idValue);
}

export function useCreateHangingPiecesKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesKitInteraction", "backward", idValue);
}

export function useChangePieceTypeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePieceTypeKitInteraction", idValue);
}

export function useChangePieceTypeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "id", idValue);
}

export function useChangePieceTypeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "hash", idValue);
}

export function useChangePieceTypeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "index", idValue);
}

export function useChangePieceTypeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "kit", idValue);
}

export function useChangePieceTypeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "kind", idValue);
}

export function useChangePieceTypeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "actor", idValue);
}

export function useChangePieceTypeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "session", idValue);
}

export function useChangePieceTypeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "transaction", idValue);
}

export function useChangePieceTypeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "candidate", idValue);
}

export function useChangePieceTypeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "change", idValue);
}

export function useChangePieceTypeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "conflict", idValue);
}

export function useChangePieceTypeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "summary", idValue);
}

export function useChangePieceTypeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "metadata", idValue);
}

export function useChangePieceTypeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "createdAt", idValue);
}

export function useChangePieceTypeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "forward", idValue);
}

export function useChangePieceTypeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeKitInteraction", "backward", idValue);
}

export function useChangePiecesTypeKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePiecesTypeKitInteraction", idValue);
}

export function useChangePiecesTypeKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "id", idValue);
}

export function useChangePiecesTypeKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "hash", idValue);
}

export function useChangePiecesTypeKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "index", idValue);
}

export function useChangePiecesTypeKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kit", idValue);
}

export function useChangePiecesTypeKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "kind", idValue);
}

export function useChangePiecesTypeKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "actor", idValue);
}

export function useChangePiecesTypeKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "session", idValue);
}

export function useChangePiecesTypeKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "transaction", idValue);
}

export function useChangePiecesTypeKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "candidate", idValue);
}

export function useChangePiecesTypeKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "change", idValue);
}

export function useChangePiecesTypeKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "conflict", idValue);
}

export function useChangePiecesTypeKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "summary", idValue);
}

export function useChangePiecesTypeKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "metadata", idValue);
}

export function useChangePiecesTypeKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "createdAt", idValue);
}

export function useChangePiecesTypeKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "forward", idValue);
}

export function useChangePiecesTypeKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeKitInteraction", "backward", idValue);
}

export function usePasteDesignSelectionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PasteDesignSelectionKitInteraction", idValue);
}

export function usePasteDesignSelectionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "id", idValue);
}

export function usePasteDesignSelectionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "hash", idValue);
}

export function usePasteDesignSelectionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "index", idValue);
}

export function usePasteDesignSelectionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kit", idValue);
}

export function usePasteDesignSelectionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "kind", idValue);
}

export function usePasteDesignSelectionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "actor", idValue);
}

export function usePasteDesignSelectionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "session", idValue);
}

export function usePasteDesignSelectionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "transaction", idValue);
}

export function usePasteDesignSelectionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "candidate", idValue);
}

export function usePasteDesignSelectionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "change", idValue);
}

export function usePasteDesignSelectionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "conflict", idValue);
}

export function usePasteDesignSelectionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "summary", idValue);
}

export function usePasteDesignSelectionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "metadata", idValue);
}

export function usePasteDesignSelectionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "createdAt", idValue);
}

export function usePasteDesignSelectionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "forward", idValue);
}

export function usePasteDesignSelectionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionKitInteraction", "backward", idValue);
}

export function useImportKitKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ImportKitKitInteraction", idValue);
}

export function useImportKitKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "id", idValue);
}

export function useImportKitKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "hash", idValue);
}

export function useImportKitKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "index", idValue);
}

export function useImportKitKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "kit", idValue);
}

export function useImportKitKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "kind", idValue);
}

export function useImportKitKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "actor", idValue);
}

export function useImportKitKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "session", idValue);
}

export function useImportKitKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "transaction", idValue);
}

export function useImportKitKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "candidate", idValue);
}

export function useImportKitKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "change", idValue);
}

export function useImportKitKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "conflict", idValue);
}

export function useImportKitKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "summary", idValue);
}

export function useImportKitKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "metadata", idValue);
}

export function useImportKitKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "createdAt", idValue);
}

export function useImportKitKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "forward", idValue);
}

export function useImportKitKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitKitInteraction", "backward", idValue);
}

export function useResetKitKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResetKitKitInteraction", idValue);
}

export function useResetKitKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "id", idValue);
}

export function useResetKitKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "hash", idValue);
}

export function useResetKitKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "index", idValue);
}

export function useResetKitKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "kit", idValue);
}

export function useResetKitKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "kind", idValue);
}

export function useResetKitKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "actor", idValue);
}

export function useResetKitKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "session", idValue);
}

export function useResetKitKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "transaction", idValue);
}

export function useResetKitKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "candidate", idValue);
}

export function useResetKitKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "change", idValue);
}

export function useResetKitKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "conflict", idValue);
}

export function useResetKitKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "summary", idValue);
}

export function useResetKitKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "metadata", idValue);
}

export function useResetKitKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "createdAt", idValue);
}

export function useResetKitKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "forward", idValue);
}

export function useResetKitKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitKitInteraction", "backward", idValue);
}

export function useExportKitKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExportKitKitInteraction", idValue);
}

export function useExportKitKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "id", idValue);
}

export function useExportKitKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "hash", idValue);
}

export function useExportKitKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "index", idValue);
}

export function useExportKitKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "kit", idValue);
}

export function useExportKitKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "kind", idValue);
}

export function useExportKitKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "actor", idValue);
}

export function useExportKitKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "session", idValue);
}

export function useExportKitKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "transaction", idValue);
}

export function useExportKitKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "candidate", idValue);
}

export function useExportKitKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "change", idValue);
}

export function useExportKitKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "conflict", idValue);
}

export function useExportKitKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "summary", idValue);
}

export function useExportKitKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "metadata", idValue);
}

export function useExportKitKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "createdAt", idValue);
}

export function useExportKitKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "forward", idValue);
}

export function useExportKitKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitKitInteraction", "backward", idValue);
}

export function useStartKitSessionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StartKitSessionKitInteraction", idValue);
}

export function useStartKitSessionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "id", idValue);
}

export function useStartKitSessionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "hash", idValue);
}

export function useStartKitSessionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "index", idValue);
}

export function useStartKitSessionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "kit", idValue);
}

export function useStartKitSessionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "kind", idValue);
}

export function useStartKitSessionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "actor", idValue);
}

export function useStartKitSessionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "session", idValue);
}

export function useStartKitSessionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "transaction", idValue);
}

export function useStartKitSessionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "candidate", idValue);
}

export function useStartKitSessionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "change", idValue);
}

export function useStartKitSessionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "conflict", idValue);
}

export function useStartKitSessionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "summary", idValue);
}

export function useStartKitSessionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "metadata", idValue);
}

export function useStartKitSessionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "createdAt", idValue);
}

export function useStartKitSessionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "forward", idValue);
}

export function useStartKitSessionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionKitInteraction", "backward", idValue);
}

export function useHeartbeatKitSessionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HeartbeatKitSessionKitInteraction", idValue);
}

export function useHeartbeatKitSessionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "id", idValue);
}

export function useHeartbeatKitSessionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "hash", idValue);
}

export function useHeartbeatKitSessionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "index", idValue);
}

export function useHeartbeatKitSessionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kit", idValue);
}

export function useHeartbeatKitSessionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "kind", idValue);
}

export function useHeartbeatKitSessionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "actor", idValue);
}

export function useHeartbeatKitSessionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "session", idValue);
}

export function useHeartbeatKitSessionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "transaction", idValue);
}

export function useHeartbeatKitSessionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "candidate", idValue);
}

export function useHeartbeatKitSessionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "change", idValue);
}

export function useHeartbeatKitSessionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "conflict", idValue);
}

export function useHeartbeatKitSessionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "summary", idValue);
}

export function useHeartbeatKitSessionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "metadata", idValue);
}

export function useHeartbeatKitSessionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "createdAt", idValue);
}

export function useHeartbeatKitSessionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "forward", idValue);
}

export function useHeartbeatKitSessionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionKitInteraction", "backward", idValue);
}

export function useEndKitSessionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("EndKitSessionKitInteraction", idValue);
}

export function useEndKitSessionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "id", idValue);
}

export function useEndKitSessionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "hash", idValue);
}

export function useEndKitSessionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "index", idValue);
}

export function useEndKitSessionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "kit", idValue);
}

export function useEndKitSessionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "kind", idValue);
}

export function useEndKitSessionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "actor", idValue);
}

export function useEndKitSessionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "session", idValue);
}

export function useEndKitSessionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "transaction", idValue);
}

export function useEndKitSessionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "candidate", idValue);
}

export function useEndKitSessionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "change", idValue);
}

export function useEndKitSessionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "conflict", idValue);
}

export function useEndKitSessionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "summary", idValue);
}

export function useEndKitSessionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "metadata", idValue);
}

export function useEndKitSessionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "createdAt", idValue);
}

export function useEndKitSessionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "forward", idValue);
}

export function useEndKitSessionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionKitInteraction", "backward", idValue);
}

export function useReconnectKitSessionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ReconnectKitSessionKitInteraction", idValue);
}

export function useReconnectKitSessionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "id", idValue);
}

export function useReconnectKitSessionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "hash", idValue);
}

export function useReconnectKitSessionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "index", idValue);
}

export function useReconnectKitSessionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kit", idValue);
}

export function useReconnectKitSessionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "kind", idValue);
}

export function useReconnectKitSessionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "actor", idValue);
}

export function useReconnectKitSessionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "session", idValue);
}

export function useReconnectKitSessionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "transaction", idValue);
}

export function useReconnectKitSessionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "candidate", idValue);
}

export function useReconnectKitSessionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "change", idValue);
}

export function useReconnectKitSessionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "conflict", idValue);
}

export function useReconnectKitSessionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "summary", idValue);
}

export function useReconnectKitSessionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "metadata", idValue);
}

export function useReconnectKitSessionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "createdAt", idValue);
}

export function useReconnectKitSessionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "forward", idValue);
}

export function useReconnectKitSessionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionKitInteraction", "backward", idValue);
}

export function useBeginKitTransactionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BeginKitTransactionKitInteraction", idValue);
}

export function useBeginKitTransactionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "id", idValue);
}

export function useBeginKitTransactionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "hash", idValue);
}

export function useBeginKitTransactionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "index", idValue);
}

export function useBeginKitTransactionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "kit", idValue);
}

export function useBeginKitTransactionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "kind", idValue);
}

export function useBeginKitTransactionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "actor", idValue);
}

export function useBeginKitTransactionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "session", idValue);
}

export function useBeginKitTransactionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "transaction", idValue);
}

export function useBeginKitTransactionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "candidate", idValue);
}

export function useBeginKitTransactionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "change", idValue);
}

export function useBeginKitTransactionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "conflict", idValue);
}

export function useBeginKitTransactionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "summary", idValue);
}

export function useBeginKitTransactionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "metadata", idValue);
}

export function useBeginKitTransactionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "createdAt", idValue);
}

export function useBeginKitTransactionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "forward", idValue);
}

export function useBeginKitTransactionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionKitInteraction", "backward", idValue);
}

export function useFinalizeKitTransactionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FinalizeKitTransactionKitInteraction", idValue);
}

export function useFinalizeKitTransactionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "id", idValue);
}

export function useFinalizeKitTransactionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "hash", idValue);
}

export function useFinalizeKitTransactionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "index", idValue);
}

export function useFinalizeKitTransactionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kit", idValue);
}

export function useFinalizeKitTransactionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "kind", idValue);
}

export function useFinalizeKitTransactionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "actor", idValue);
}

export function useFinalizeKitTransactionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "session", idValue);
}

export function useFinalizeKitTransactionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "transaction", idValue);
}

export function useFinalizeKitTransactionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "candidate", idValue);
}

export function useFinalizeKitTransactionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "change", idValue);
}

export function useFinalizeKitTransactionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "conflict", idValue);
}

export function useFinalizeKitTransactionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "summary", idValue);
}

export function useFinalizeKitTransactionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "metadata", idValue);
}

export function useFinalizeKitTransactionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "createdAt", idValue);
}

export function useFinalizeKitTransactionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "forward", idValue);
}

export function useFinalizeKitTransactionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionKitInteraction", "backward", idValue);
}

export function useAbortKitTransactionKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AbortKitTransactionKitInteraction", idValue);
}

export function useAbortKitTransactionKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "id", idValue);
}

export function useAbortKitTransactionKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "hash", idValue);
}

export function useAbortKitTransactionKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "index", idValue);
}

export function useAbortKitTransactionKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "kit", idValue);
}

export function useAbortKitTransactionKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "kind", idValue);
}

export function useAbortKitTransactionKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "actor", idValue);
}

export function useAbortKitTransactionKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "session", idValue);
}

export function useAbortKitTransactionKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "transaction", idValue);
}

export function useAbortKitTransactionKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "candidate", idValue);
}

export function useAbortKitTransactionKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "change", idValue);
}

export function useAbortKitTransactionKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "conflict", idValue);
}

export function useAbortKitTransactionKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "summary", idValue);
}

export function useAbortKitTransactionKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "metadata", idValue);
}

export function useAbortKitTransactionKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "createdAt", idValue);
}

export function useAbortKitTransactionKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "forward", idValue);
}

export function useAbortKitTransactionKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionKitInteraction", "backward", idValue);
}

export function useTransactionStepKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionStepKitInteraction", idValue);
}

export function useTransactionStepKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "id", idValue);
}

export function useTransactionStepKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "hash", idValue);
}

export function useTransactionStepKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "index", idValue);
}

export function useTransactionStepKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "kit", idValue);
}

export function useTransactionStepKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "kind", idValue);
}

export function useTransactionStepKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "actor", idValue);
}

export function useTransactionStepKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "session", idValue);
}

export function useTransactionStepKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "transaction", idValue);
}

export function useTransactionStepKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "candidate", idValue);
}

export function useTransactionStepKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "change", idValue);
}

export function useTransactionStepKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "conflict", idValue);
}

export function useTransactionStepKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "summary", idValue);
}

export function useTransactionStepKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "metadata", idValue);
}

export function useTransactionStepKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "createdAt", idValue);
}

export function useTransactionStepKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "forward", idValue);
}

export function useTransactionStepKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepKitInteraction", "backward", idValue);
}

export function useHistoryStepKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HistoryStepKitInteraction", idValue);
}

export function useHistoryStepKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "id", idValue);
}

export function useHistoryStepKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "hash", idValue);
}

export function useHistoryStepKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "index", idValue);
}

export function useHistoryStepKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "kit", idValue);
}

export function useHistoryStepKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "kind", idValue);
}

export function useHistoryStepKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "actor", idValue);
}

export function useHistoryStepKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "session", idValue);
}

export function useHistoryStepKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "transaction", idValue);
}

export function useHistoryStepKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "candidate", idValue);
}

export function useHistoryStepKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "change", idValue);
}

export function useHistoryStepKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "conflict", idValue);
}

export function useHistoryStepKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "summary", idValue);
}

export function useHistoryStepKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "metadata", idValue);
}

export function useHistoryStepKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "createdAt", idValue);
}

export function useHistoryStepKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "forward", idValue);
}

export function useHistoryStepKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepKitInteraction", "backward", idValue);
}

export function useVoteOnKitChangeCandidateKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VoteOnKitChangeCandidateKitInteraction", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "id", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "hash", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "index", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kit", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "kind", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "actor", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "session", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "transaction", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "candidate", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "change", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "conflict", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "summary", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "metadata", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "createdAt", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "forward", idValue);
}

export function useVoteOnKitChangeCandidateKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateKitInteraction", "backward", idValue);
}

export function useResolveKitConflictKitInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResolveKitConflictKitInteraction", idValue);
}

export function useResolveKitConflictKitInteractionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "id", idValue);
}

export function useResolveKitConflictKitInteractionHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "hash", idValue);
}

export function useResolveKitConflictKitInteractionIndex(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "index", idValue);
}

export function useResolveKitConflictKitInteractionKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "kit", idValue);
}

export function useResolveKitConflictKitInteractionKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "kind", idValue);
}

export function useResolveKitConflictKitInteractionActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "actor", idValue);
}

export function useResolveKitConflictKitInteractionSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "session", idValue);
}

export function useResolveKitConflictKitInteractionTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "transaction", idValue);
}

export function useResolveKitConflictKitInteractionCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "candidate", idValue);
}

export function useResolveKitConflictKitInteractionChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "change", idValue);
}

export function useResolveKitConflictKitInteractionConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "conflict", idValue);
}

export function useResolveKitConflictKitInteractionSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "summary", idValue);
}

export function useResolveKitConflictKitInteractionMetadata(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "metadata", idValue);
}

export function useResolveKitConflictKitInteractionCreatedAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "createdAt", idValue);
}

export function useResolveKitConflictKitInteractionForward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "forward", idValue);
}

export function useResolveKitConflictKitInteractionBackward(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictKitInteraction", "backward", idValue);
}

export function useKitInteractionPage(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitInteractionPage", idValue);
}

export function useKitInteractionPageHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "hash", idValue);
}

export function useKitInteractionPageNodes(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "nodes", idValue);
}

export function useKitInteractionPagePageInfo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "pageInfo", idValue);
}

export function useKitInteractionPageTotalCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitInteractionPage", "totalCount", idValue);
}

export function useKitHistory(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitHistory", idValue);
}

export function useKitHistoryHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "hash", idValue);
}

export function useKitHistoryCanUndo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "canUndo", idValue);
}

export function useKitHistoryCanRedo(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "canRedo", idValue);
}

export function useKitHistoryTotalCount(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "totalCount", idValue);
}

export function useKitHistoryHead(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitHistory", "head", idValue);
}

export function useKitStoreEntity(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStore", idValue);
}

export function useKitStoreHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "hash", idValue);
}

export function useKitStoreKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "kit", idValue);
}

export function useKitStoreBackbone(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "backbone", idValue);
}

export function useKitStoreSessions(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "sessions", idValue);
}

export function useKitStoreTransactions(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "transactions", idValue);
}

export function useKitStorePendingCandidates(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "pendingCandidates", idValue);
}

export function useKitStoreActiveConflicts(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "activeConflicts", idValue);
}

export function useKitStoreValidation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "validation", idValue);
}

export function useKitStoreHistory(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "history", idValue);
}

export function useKitStoreBlockedByConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "blockedByConflict", idValue);
}

export function useKitStoreStrictMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStore", "strictMode", idValue);
}

export function useArtifactKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ArtifactKind", idValue);
}

export function useSelectionMutationMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SelectionMutationMode", idValue);
}

export function useKitArchiveExport(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitArchiveExport", idValue);
}

export function useKitArchiveExportHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "hash", idValue);
}

export function useKitArchiveExportFileName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "fileName", idValue);
}

export function useKitArchiveExportUrl(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "url", idValue);
}

export function useKitArchiveExportExpiresAt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitArchiveExport", "expiresAt", idValue);
}

export function useKitMutationResult(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitMutationResult", idValue);
}

export function useKitMutationResultHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "hash", idValue);
}

export function useKitMutationResultAccepted(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "accepted", idValue);
}

export function useKitMutationResultKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "kind", idValue);
}

export function useKitMutationResultSummary(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "summary", idValue);
}

export function useKitMutationResultStore(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "store", idValue);
}

export function useKitMutationResultKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "kit", idValue);
}

export function useKitMutationResultSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "session", idValue);
}

export function useKitMutationResultTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "transaction", idValue);
}

export function useKitMutationResultCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "candidate", idValue);
}

export function useKitMutationResultChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "change", idValue);
}

export function useKitMutationResultHistoryEntry(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "historyEntry", idValue);
}

export function useKitMutationResultConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "conflict", idValue);
}

export function useKitMutationResultValidation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "validation", idValue);
}

export function useKitMutationResultExport(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitMutationResult", "export", idValue);
}

export function useKitCommandContextInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitCommandContextInput", idValue);
}

export function useKitCommandContextInputKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "kitId", idValue);
}

export function useKitCommandContextInputSessionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "sessionId", idValue);
}

export function useKitCommandContextInputTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "transactionId", idValue);
}

export function useKitCommandContextInputOrigin(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "origin", idValue);
}

export function useKitCommandContextInputExpectedHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "expectedHash", idValue);
}

export function useKitCommandContextInputStrictMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitCommandContextInput", "strictMode", idValue);
}

export function useStartKitSessionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("StartKitSessionInput", idValue);
}

export function useStartKitSessionInputKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "kitId", idValue);
}

export function useStartKitSessionInputActor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "actor", idValue);
}

export function useStartKitSessionInputClient(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "client", idValue);
}

export function useStartKitSessionInputStrictMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("StartKitSessionInput", "strictMode", idValue);
}

export function useHeartbeatKitSessionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HeartbeatKitSessionInput", idValue);
}

export function useHeartbeatKitSessionInputKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "kitId", idValue);
}

export function useHeartbeatKitSessionInputSessionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "sessionId", idValue);
}

export function useHeartbeatKitSessionInputLastKnownHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HeartbeatKitSessionInput", "lastKnownHash", idValue);
}

export function useEndKitSessionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("EndKitSessionInput", idValue);
}

export function useEndKitSessionInputKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionInput", "kitId", idValue);
}

export function useEndKitSessionInputSessionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("EndKitSessionInput", "sessionId", idValue);
}

export function useReconnectKitSessionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ReconnectKitSessionInput", idValue);
}

export function useReconnectKitSessionInputKitId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "kitId", idValue);
}

export function useReconnectKitSessionInputSessionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "sessionId", idValue);
}

export function useReconnectKitSessionInputClient(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "client", idValue);
}

export function useReconnectKitSessionInputLastKnownHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ReconnectKitSessionInput", "lastKnownHash", idValue);
}

export function useSetSessionSelectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("SetSessionSelectionCommandInput", idValue);
}

export function useSetSessionSelectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "context", idValue);
}

export function useSetSessionSelectionCommandInputMode(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "mode", idValue);
}

export function useSetSessionSelectionCommandInputSelection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("SetSessionSelectionCommandInput", "selection", idValue);
}

export function useBeginKitTransactionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("BeginKitTransactionInput", idValue);
}

export function useBeginKitTransactionInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "context", idValue);
}

export function useBeginKitTransactionInputLabel(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "label", idValue);
}

export function useBeginKitTransactionInputParentTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("BeginKitTransactionInput", "parentTransactionId", idValue);
}

export function useFinalizeKitTransactionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FinalizeKitTransactionInput", idValue);
}

export function useFinalizeKitTransactionInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionInput", "context", idValue);
}

export function useFinalizeKitTransactionInputTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FinalizeKitTransactionInput", "transactionId", idValue);
}

export function useAbortKitTransactionInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("AbortKitTransactionInput", idValue);
}

export function useAbortKitTransactionInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionInput", "context", idValue);
}

export function useAbortKitTransactionInputTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("AbortKitTransactionInput", "transactionId", idValue);
}

export function useTransactionStepInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("TransactionStepInput", idValue);
}

export function useTransactionStepInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepInput", "context", idValue);
}

export function useTransactionStepInputTransactionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("TransactionStepInput", "transactionId", idValue);
}

export function useHistoryStepInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("HistoryStepInput", idValue);
}

export function useHistoryStepInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepInput", "context", idValue);
}

export function useHistoryStepInputSteps(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("HistoryStepInput", "steps", idValue);
}

export function useVoteOnKitChangeCandidateInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("VoteOnKitChangeCandidateInput", idValue);
}

export function useVoteOnKitChangeCandidateInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "context", idValue);
}

export function useVoteOnKitChangeCandidateInputCandidateId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "candidateId", idValue);
}

export function useVoteOnKitChangeCandidateInputState(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "state", idValue);
}

export function useVoteOnKitChangeCandidateInputReason(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "reason", idValue);
}

export function useVoteOnKitChangeCandidateInputResolutionOptionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("VoteOnKitChangeCandidateInput", "resolutionOptionId", idValue);
}

export function useResolveKitConflictInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResolveKitConflictInput", idValue);
}

export function useResolveKitConflictInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "context", idValue);
}

export function useResolveKitConflictInputConflictId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "conflictId", idValue);
}

export function useResolveKitConflictInputOptionId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "optionId", idValue);
}

export function useResolveKitConflictInputPayload(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResolveKitConflictInput", "payload", idValue);
}

export function useCreateAuthorCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateAuthorCommandInput", idValue);
}

export function useCreateAuthorCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorCommandInput", "context", idValue);
}

export function useCreateAuthorCommandInputAuthor(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateAuthorCommandInput", "author", idValue);
}

export function useUpdateAuthorCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateAuthorCommandInput", idValue);
}

export function useUpdateAuthorCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "context", idValue);
}

export function useUpdateAuthorCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "id", idValue);
}

export function useUpdateAuthorCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateAuthorCommandInput", "patch", idValue);
}

export function useDeleteAuthorCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteAuthorCommandInput", idValue);
}

export function useDeleteAuthorCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorCommandInput", "context", idValue);
}

export function useDeleteAuthorCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteAuthorCommandInput", "id", idValue);
}

export function useCreateTypeCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTypeCommandInput", idValue);
}

export function useCreateTypeCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeCommandInput", "context", idValue);
}

export function useCreateTypeCommandInputType(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTypeCommandInput", "type", idValue);
}

export function useUpdateTypeCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTypeCommandInput", idValue);
}

export function useUpdateTypeCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "context", idValue);
}

export function useUpdateTypeCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "id", idValue);
}

export function useUpdateTypeCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTypeCommandInput", "patch", idValue);
}

export function useDeleteTypeCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTypeCommandInput", idValue);
}

export function useDeleteTypeCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeCommandInput", "context", idValue);
}

export function useDeleteTypeCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTypeCommandInput", "id", idValue);
}

export function useCreateDesignCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateDesignCommandInput", idValue);
}

export function useCreateDesignCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignCommandInput", "context", idValue);
}

export function useCreateDesignCommandInputDesign(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateDesignCommandInput", "design", idValue);
}

export function useUpdateDesignCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateDesignCommandInput", idValue);
}

export function useUpdateDesignCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "context", idValue);
}

export function useUpdateDesignCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "id", idValue);
}

export function useUpdateDesignCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateDesignCommandInput", "patch", idValue);
}

export function useDeleteDesignCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteDesignCommandInput", idValue);
}

export function useDeleteDesignCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignCommandInput", "context", idValue);
}

export function useDeleteDesignCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteDesignCommandInput", "id", idValue);
}

export function useCreateQualityCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateQualityCommandInput", idValue);
}

export function useCreateQualityCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityCommandInput", "context", idValue);
}

export function useCreateQualityCommandInputQuality(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateQualityCommandInput", "quality", idValue);
}

export function useUpdateQualityCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateQualityCommandInput", idValue);
}

export function useUpdateQualityCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "context", idValue);
}

export function useUpdateQualityCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "id", idValue);
}

export function useUpdateQualityCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateQualityCommandInput", "patch", idValue);
}

export function useDeleteQualityCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteQualityCommandInput", idValue);
}

export function useDeleteQualityCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityCommandInput", "context", idValue);
}

export function useDeleteQualityCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteQualityCommandInput", "id", idValue);
}

export function useCreatePortCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePortCommandInput", idValue);
}

export function useCreatePortCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortCommandInput", "context", idValue);
}

export function useCreatePortCommandInputPort(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePortCommandInput", "port", idValue);
}

export function useUpdatePortCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePortCommandInput", idValue);
}

export function useUpdatePortCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "context", idValue);
}

export function useUpdatePortCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "id", idValue);
}

export function useUpdatePortCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePortCommandInput", "patch", idValue);
}

export function useDeletePortCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePortCommandInput", idValue);
}

export function useDeletePortCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortCommandInput", "context", idValue);
}

export function useDeletePortCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePortCommandInput", "id", idValue);
}

export function useCreateFamilyCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFamilyCommandInput", idValue);
}

export function useCreateFamilyCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyCommandInput", "context", idValue);
}

export function useCreateFamilyCommandInputFamily(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFamilyCommandInput", "family", idValue);
}

export function useUpdateFamilyCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFamilyCommandInput", idValue);
}

export function useUpdateFamilyCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "context", idValue);
}

export function useUpdateFamilyCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "id", idValue);
}

export function useUpdateFamilyCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFamilyCommandInput", "patch", idValue);
}

export function useDeleteFamilyCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFamilyCommandInput", idValue);
}

export function useDeleteFamilyCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyCommandInput", "context", idValue);
}

export function useDeleteFamilyCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFamilyCommandInput", "id", idValue);
}

export function useCreateTagCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateTagCommandInput", idValue);
}

export function useCreateTagCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagCommandInput", "context", idValue);
}

export function useCreateTagCommandInputTag(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateTagCommandInput", "tag", idValue);
}

export function useUpdateTagCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateTagCommandInput", idValue);
}

export function useUpdateTagCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "context", idValue);
}

export function useUpdateTagCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "id", idValue);
}

export function useUpdateTagCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateTagCommandInput", "patch", idValue);
}

export function useDeleteTagCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteTagCommandInput", idValue);
}

export function useDeleteTagCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagCommandInput", "context", idValue);
}

export function useDeleteTagCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteTagCommandInput", "id", idValue);
}

export function useCreateConceptCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConceptCommandInput", idValue);
}

export function useCreateConceptCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptCommandInput", "context", idValue);
}

export function useCreateConceptCommandInputConcept(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConceptCommandInput", "concept", idValue);
}

export function useUpdateConceptCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConceptCommandInput", idValue);
}

export function useUpdateConceptCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "context", idValue);
}

export function useUpdateConceptCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "id", idValue);
}

export function useUpdateConceptCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConceptCommandInput", "patch", idValue);
}

export function useDeleteConceptCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConceptCommandInput", idValue);
}

export function useDeleteConceptCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptCommandInput", "context", idValue);
}

export function useDeleteConceptCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConceptCommandInput", "id", idValue);
}

export function useCreateFileCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFileCommandInput", idValue);
}

export function useCreateFileCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileCommandInput", "context", idValue);
}

export function useCreateFileCommandInputFile(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFileCommandInput", "file", idValue);
}

export function useUpdateFileCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFileCommandInput", idValue);
}

export function useUpdateFileCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "context", idValue);
}

export function useUpdateFileCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "id", idValue);
}

export function useUpdateFileCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFileCommandInput", "patch", idValue);
}

export function useDeleteFileCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFileCommandInput", idValue);
}

export function useDeleteFileCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileCommandInput", "context", idValue);
}

export function useDeleteFileCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFileCommandInput", "id", idValue);
}

export function useCreateFolderCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFolderCommandInput", idValue);
}

export function useCreateFolderCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderCommandInput", "context", idValue);
}

export function useCreateFolderCommandInputFolder(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFolderCommandInput", "folder", idValue);
}

export function useUpdateFolderCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateFolderCommandInput", idValue);
}

export function useUpdateFolderCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "context", idValue);
}

export function useUpdateFolderCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "id", idValue);
}

export function useUpdateFolderCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateFolderCommandInput", "patch", idValue);
}

export function useDeleteFolderCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteFolderCommandInput", idValue);
}

export function useDeleteFolderCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderCommandInput", "context", idValue);
}

export function useDeleteFolderCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteFolderCommandInput", "id", idValue);
}

export function useMoveArtifactToFolderCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MoveArtifactToFolderCommandInput", idValue);
}

export function useMoveArtifactToFolderCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "context", idValue);
}

export function useMoveArtifactToFolderCommandInputArtifactKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactKind", idValue);
}

export function useMoveArtifactToFolderCommandInputArtifactId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "artifactId", idValue);
}

export function useMoveArtifactToFolderCommandInputFolderId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MoveArtifactToFolderCommandInput", "folderId", idValue);
}

export function useCreatePieceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePieceCommandInput", idValue);
}

export function useCreatePieceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "context", idValue);
}

export function useCreatePieceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "designId", idValue);
}

export function useCreatePieceCommandInputPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePieceCommandInput", "piece", idValue);
}

export function useCreatePiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreatePiecesCommandInput", idValue);
}

export function useCreatePiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "context", idValue);
}

export function useCreatePiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "designId", idValue);
}

export function useCreatePiecesCommandInputPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreatePiecesCommandInput", "pieces", idValue);
}

export function usePieceUpdateInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PieceUpdateInput", idValue);
}

export function usePieceUpdateInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceUpdateInput", "id", idValue);
}

export function usePieceUpdateInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PieceUpdateInput", "patch", idValue);
}

export function useUpdatePieceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePieceCommandInput", idValue);
}

export function useUpdatePieceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "context", idValue);
}

export function useUpdatePieceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "designId", idValue);
}

export function useUpdatePieceCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "id", idValue);
}

export function useUpdatePieceCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePieceCommandInput", "patch", idValue);
}

export function useUpdatePiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdatePiecesCommandInput", idValue);
}

export function useUpdatePiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "context", idValue);
}

export function useUpdatePiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "designId", idValue);
}

export function useUpdatePiecesCommandInputUpdates(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdatePiecesCommandInput", "updates", idValue);
}

export function useDeletePieceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePieceCommandInput", idValue);
}

export function useDeletePieceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "context", idValue);
}

export function useDeletePieceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "designId", idValue);
}

export function useDeletePieceCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePieceCommandInput", "id", idValue);
}

export function useDeletePiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeletePiecesCommandInput", idValue);
}

export function useDeletePiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "context", idValue);
}

export function useDeletePiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "designId", idValue);
}

export function useDeletePiecesCommandInputIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeletePiecesCommandInput", "ids", idValue);
}

export function useCreateConnectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionCommandInput", idValue);
}

export function useCreateConnectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "context", idValue);
}

export function useCreateConnectionCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "designId", idValue);
}

export function useCreateConnectionCommandInputConnection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionCommandInput", "connection", idValue);
}

export function useCreateConnectionsCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectionsCommandInput", idValue);
}

export function useCreateConnectionsCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "context", idValue);
}

export function useCreateConnectionsCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "designId", idValue);
}

export function useCreateConnectionsCommandInputConnections(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectionsCommandInput", "connections", idValue);
}

export function useConnectionUpdateInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ConnectionUpdateInput", idValue);
}

export function useConnectionUpdateInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionUpdateInput", "id", idValue);
}

export function useConnectionUpdateInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ConnectionUpdateInput", "patch", idValue);
}

export function useUpdateConnectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionCommandInput", idValue);
}

export function useUpdateConnectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "context", idValue);
}

export function useUpdateConnectionCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "designId", idValue);
}

export function useUpdateConnectionCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "id", idValue);
}

export function useUpdateConnectionCommandInputPatch(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionCommandInput", "patch", idValue);
}

export function useUpdateConnectionsCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("UpdateConnectionsCommandInput", idValue);
}

export function useUpdateConnectionsCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "context", idValue);
}

export function useUpdateConnectionsCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "designId", idValue);
}

export function useUpdateConnectionsCommandInputUpdates(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("UpdateConnectionsCommandInput", "updates", idValue);
}

export function useDeleteConnectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionCommandInput", idValue);
}

export function useDeleteConnectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "context", idValue);
}

export function useDeleteConnectionCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "designId", idValue);
}

export function useDeleteConnectionCommandInputId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionCommandInput", "id", idValue);
}

export function useDeleteConnectionsCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteConnectionsCommandInput", idValue);
}

export function useDeleteConnectionsCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "context", idValue);
}

export function useDeleteConnectionsCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "designId", idValue);
}

export function useDeleteConnectionsCommandInputIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteConnectionsCommandInput", "ids", idValue);
}

export function useDeleteSelectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DeleteSelectionCommandInput", idValue);
}

export function useDeleteSelectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "context", idValue);
}

export function useDeleteSelectionCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "designId", idValue);
}

export function useDeleteSelectionCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "pieceIds", idValue);
}

export function useDeleteSelectionCommandInputConnectionIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DeleteSelectionCommandInput", "connectionIds", idValue);
}

export function useFixPiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FixPiecesCommandInput", idValue);
}

export function useFixPiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "context", idValue);
}

export function useFixPiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "designId", idValue);
}

export function useFixPiecesCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FixPiecesCommandInput", "pieceIds", idValue);
}

export function useClusterPiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ClusterPiecesCommandInput", idValue);
}

export function useClusterPiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "context", idValue);
}

export function useClusterPiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "designId", idValue);
}

export function useClusterPiecesCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "pieceIds", idValue);
}

export function useClusterPiecesCommandInputNewDesignName(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ClusterPiecesCommandInput", "newDesignName", idValue);
}

export function useExpandDesignReferenceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExpandDesignReferenceCommandInput", idValue);
}

export function useExpandDesignReferenceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "context", idValue);
}

export function useExpandDesignReferenceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "designId", idValue);
}

export function useExpandDesignReferenceCommandInputReferencedDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExpandDesignReferenceCommandInput", "referencedDesignId", idValue);
}

export function useFlattenDesignCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("FlattenDesignCommandInput", idValue);
}

export function useFlattenDesignCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignCommandInput", "context", idValue);
}

export function useFlattenDesignCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("FlattenDesignCommandInput", "designId", idValue);
}

export function useDragPiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("DragPiecesCommandInput", idValue);
}

export function useDragPiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "context", idValue);
}

export function useDragPiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "designId", idValue);
}

export function useDragPiecesCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "pieceIds", idValue);
}

export function useDragPiecesCommandInputOffset(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("DragPiecesCommandInput", "offset", idValue);
}

export function useMovePiecesVectorInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesVectorInput", idValue);
}

export function useMovePiecesVectorInputShift(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "shift", idValue);
}

export function useMovePiecesVectorInputGap(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "gap", idValue);
}

export function useMovePiecesVectorInputRise(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "rise", idValue);
}

export function useMovePiecesVectorInputRotation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "rotation", idValue);
}

export function useMovePiecesVectorInputTurn(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "turn", idValue);
}

export function useMovePiecesVectorInputTilt(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesVectorInput", "tilt", idValue);
}

export function useMovePiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("MovePiecesCommandInput", idValue);
}

export function useMovePiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "context", idValue);
}

export function useMovePiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "designId", idValue);
}

export function useMovePiecesCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "pieceIds", idValue);
}

export function useMovePiecesCommandInputVector(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("MovePiecesCommandInput", "vector", idValue);
}

export function useCreateFixedPieceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateFixedPieceCommandInput", idValue);
}

export function useCreateFixedPieceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "context", idValue);
}

export function useCreateFixedPieceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "designId", idValue);
}

export function useCreateFixedPieceCommandInputPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateFixedPieceCommandInput", "piece", idValue);
}

export function useCreateConnectedPieceCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateConnectedPieceCommandInput", idValue);
}

export function useCreateConnectedPieceCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "context", idValue);
}

export function useCreateConnectedPieceCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "designId", idValue);
}

export function useCreateConnectedPieceCommandInputPiece(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "piece", idValue);
}

export function useCreateConnectedPieceCommandInputConnection(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateConnectedPieceCommandInput", "connection", idValue);
}

export function useCreateHangingPiecesCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("CreateHangingPiecesCommandInput", idValue);
}

export function useCreateHangingPiecesCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "context", idValue);
}

export function useCreateHangingPiecesCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "designId", idValue);
}

export function useCreateHangingPiecesCommandInputPieces(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "pieces", idValue);
}

export function useCreateHangingPiecesCommandInputParentPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentPieceId", idValue);
}

export function useCreateHangingPiecesCommandInputParentDesignPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentDesignPieceId", idValue);
}

export function useCreateHangingPiecesCommandInputParentConnectorId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "parentConnectorId", idValue);
}

export function useCreateHangingPiecesCommandInputConnectionTemplate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("CreateHangingPiecesCommandInput", "connectionTemplate", idValue);
}

export function useChangePieceTypeCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePieceTypeCommandInput", idValue);
}

export function useChangePieceTypeCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "context", idValue);
}

export function useChangePieceTypeCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "designId", idValue);
}

export function useChangePieceTypeCommandInputPieceId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "pieceId", idValue);
}

export function useChangePieceTypeCommandInputTypeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePieceTypeCommandInput", "typeId", idValue);
}

export function useChangePiecesTypeCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ChangePiecesTypeCommandInput", idValue);
}

export function useChangePiecesTypeCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "context", idValue);
}

export function useChangePiecesTypeCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "designId", idValue);
}

export function useChangePiecesTypeCommandInputPieceIds(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "pieceIds", idValue);
}

export function useChangePiecesTypeCommandInputTypeId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ChangePiecesTypeCommandInput", "typeId", idValue);
}

export function usePasteDesignSelectionCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("PasteDesignSelectionCommandInput", idValue);
}

export function usePasteDesignSelectionCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "context", idValue);
}

export function usePasteDesignSelectionCommandInputDesignId(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "designId", idValue);
}

export function usePasteDesignSelectionCommandInputPayload(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "payload", idValue);
}

export function usePasteDesignSelectionCommandInputOffset(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("PasteDesignSelectionCommandInput", "offset", idValue);
}

export function useImportKitCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ImportKitCommandInput", idValue);
}

export function useImportKitCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "context", idValue);
}

export function useImportKitCommandInputSourceUrl(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "sourceUrl", idValue);
}

export function useImportKitCommandInputArchiveBase64(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ImportKitCommandInput", "archiveBase64", idValue);
}

export function useResetKitCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ResetKitCommandInput", idValue);
}

export function useResetKitCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "context", idValue);
}

export function useResetKitCommandInputSourceUrl(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "sourceUrl", idValue);
}

export function useResetKitCommandInputArchiveBase64(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "archiveBase64", idValue);
}

export function useResetKitCommandInputKit(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ResetKitCommandInput", "kit", idValue);
}

export function useExportKitCommandInput(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("ExportKitCommandInput", idValue);
}

export function useExportKitCommandInputContext(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("ExportKitCommandInput", "context", idValue);
}

export function useQuery(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Query", idValue);
}

export function useQueryKitCommandCatalog(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("Query", "kitCommandCatalog", idValue);
}

export function useMutation(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("Mutation", idValue);
}

export function useKitStoreEventKindEnum(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStoreEventKind", idValue);
}

export function useKitStoreEvent(idValue?: string): SchemaHookTriad<any> {
	return useSchemaObjectState("KitStoreEvent", idValue);
}

export function useKitStoreEventHash(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "hash", idValue);
}

export function useKitStoreEventKind(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "kind", idValue);
}

export function useKitStoreEventStore(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "store", idValue);
}

export function useKitStoreEventInteraction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "interaction", idValue);
}

export function useKitStoreEventChange(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "change", idValue);
}

export function useKitStoreEventCandidate(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "candidate", idValue);
}

export function useKitStoreEventConflict(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "conflict", idValue);
}

export function useKitStoreEventSession(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "session", idValue);
}

export function useKitStoreEventTransaction(idValue?: string): SchemaHookTriad<any> {
	return useSchemaFieldState("KitStoreEvent", "transaction", idValue);
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
	useReconnectKitSessionKitInteraction,
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

export function useSchemaHook(hookName: string, idValue?: string): SchemaHookTriad<any> {
	const hook = (schemaHooks)[hookName];
	if (typeof hook !== "function") {
		return [undefined, noopAsyncSet, { kind: "readonly", pending: 0 }] as const;
	}
	return hook(idValue);
}

// #endregion ⚛️Direct Domain Exports

// #region ⚛️Embedded tests
const shouldRunReactEmbeddedTests =
	(typeof process !== "undefined" && process.env.SEMIO_REACT_RUN_EMBEDDED_TESTS === "1") ||
	(typeof (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ !== "undefined" &&
		(globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ === true);

if (shouldRunReactEmbeddedTests) {
	const { describe, expect, it } = await import("vitest");
	const { act, render, waitFor } = await import("@testing-library/react");
	const { InMemoryKitStore, asKitInstance } = await import("@semio/js");

	describe("pipeline hooks", () => {
		it("useKitName rejects empty required name via kit client", async () => {
			const kit = asKitInstance({
				id: "k1",
				name: "K",
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				designs: [
					{
						id: "d1",
						name: "D",
						createdAt: new Date().toISOString(),
						updatedAt: new Date().toISOString(),
						pieces: [{ id: "p1", name: "N" }],
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
				id: "k1",
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
				id: "k1",
				name: "K",
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				designs: [
					{
						id: "d1",
						name: "D",
						createdAt: new Date().toISOString(),
						updatedAt: new Date().toISOString(),
						pieces: [{ id: "p1", name: "P" }],
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

	describe("useDraft", () => {
		it("keeps local draft and does not clear it when commit rejects", async () => {
			const triad: HookTriad<string> = [
				"server",
				async (next) => {
					const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
					if (v === "reject")
						return { ok: false, error: { kind: "InvalidValue", message: "rejected" } } as const;
					return { ok: true } as const;
				},
				{ kind: "idle", pending: 0 },
			];
			let snap: ReturnType<typeof useDraft<string>> | null = null;
			function P() {
				snap = useDraft(triad);
				return null;
			}
			render(React.createElement(P));
			await waitFor(() => expect(snap).not.toBeNull());
			await act(async () => {
				snap!.setDraft("reject");
			});
			const r = await act(async () => snap!.commit());
			expect(r.ok).toBe(false);
			expect(snap!.value).toBe("reject");
		});

		it("clears draft when commit succeeds", async () => {
			const triad: HookTriad<string> = [
				"server",
				async (next) => {
					const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
					return { ok: true } as const;
				},
				{ kind: "idle", pending: 0 },
			];
			let snap: ReturnType<typeof useDraft<string>> | null = null;
			function P() {
				snap = useDraft(triad);
				return null;
			}
			render(React.createElement(P));
			await waitFor(() => expect(snap).not.toBeNull());
			await act(async () => {
				snap!.setDraft("edited");
			});
			expect(snap!.value).toBe("edited");
			const r = await act(async () => snap!.commit());
			expect(r.ok).toBe(true);
			expect(snap!.value).toBe("server");
		});

		it("two useDraft instances do not share draft state", async () => {
			const triadA: HookTriad<string> = [
				"a",
				async () => ({ ok: true } as const),
				{ kind: "idle", pending: 0 },
			];
			const triadB: HookTriad<string> = [
				"b",
				async () => ({ ok: true } as const),
				{ kind: "idle", pending: 0 },
			];
			let sa: ReturnType<typeof useDraft<string>> | null = null;
			let sb: ReturnType<typeof useDraft<string>> | null = null;
			function P() {
				sa = useDraft(triadA);
				sb = useDraft(triadB);
				return null;
			}
			render(React.createElement(P));
			await waitFor(() => expect(sa && sb).toBeTruthy());
			await act(async () => {
				sa!.setDraft("only-a");
				sb!.setDraft("only-b");
			});
			expect(sa!.value).toBe("only-a");
			expect(sb!.value).toBe("only-b");
		});
	});
}
// #endregion ⚛️Embedded tests
