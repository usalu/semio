//#region 🧲Header
// 2024-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/react: thin adapter from UI hosts to {@link Session} / {@link JsStore} of `@semio/js` (no kit authority).
//#endregion 🧲Header

//#region 🔌Adapters
import {
	Session,
	fetchSemioFileSystemChildren,
	type Kit as JsKit,
	type SemioFileSystemChildRef,
	type SemioFileSystemParentRef,
	type SetResult,
	type Store as JsStore,
	type Unsubscribe,
} from "@semio/js";
import { gunzipSync, unzipSync } from "fflate";
import { useSyncExternalStore } from "react";

/** @emoji 🗜️ Inflates a gzip stream (adapter over `fflate`). */
function inflateGzip(bytes: Uint8Array): Uint8Array {
	return gunzipSync(bytes);
}

/** @emoji 🗜️ Extracts every entry of a zip archive (adapter over `fflate`). */
function extractZip(bytes: Uint8Array): Readonly<Record<string, Uint8Array>> {
	return unzipSync(bytes);
}

export type {
	GraphqlVariables,
	KitOperation,
	KitOperationEnvelope,
	KitOperationExecutor,
	KitOperationListener,
	KitOperationMiddleware,
	KitOperationOrigin,
	SemioFileSystemChildRef,
	SemioFileSystemParentRef,
	SetError,
	SetResult,
	Unsubscribe,
} from "@semio/js";
export { newKitOperationId } from "@semio/js";

/** @emoji 🎛️ Kit command surface handed to {@link SemioKitClient.execute} (one method per GraphQL kit command). */
export type SemioKitCommands = JsKit;

/** @emoji 🧭 Live `@semio/js` session backing a {@link SemioKitClient} (hub replication hooks attach here). */
export type SemioKitSession = Session;
//#endregion 🔌Adapters

//#region 📦KitSnapshot
/** @emoji 🔗 Reference to another kit entity by id. */
export type EntityRef = Readonly<{ id: string }>;

/** @emoji 📍 3D point / vector. */
export type Point = Readonly<{ x: number; y: number; z: number }>;

/** @emoji ◳ Plane (origin + axes). */
export type Plane = Readonly<{ origin: Point; xAxis: Point; yAxis: Point }>;

/** @emoji 🧭 Piece placement (diagram center + scene plane). */
export type PiecePosition = Readonly<{ center?: Readonly<{ u: number; v: number }>; plane?: Plane }>;

/** @emoji ⭕ Piece row of a {@link Design} snapshot; exactly one of {@link type} / {@link design} is its blueprint. */
export type Piece = Readonly<{ id: string; name?: string; description?: string; type?: EntityRef; design?: EntityRef; position?: PiecePosition }>;

/** @emoji 🔗 One side of a {@link Connection}. */
export type ConnectionSide = Readonly<{ piece?: EntityRef; connector?: EntityRef }>;

/** @emoji 🔗 Connection row of a {@link Design} snapshot. */
export type Connection = Readonly<{ id: string; description?: string; parent?: ConnectionSide; child?: ConnectionSide }>;

/** @emoji 🏘 Design snapshot. */
export type Design = Readonly<{ id: string; name?: string; description?: string; unit?: string; pieces?: readonly Piece[]; connections?: readonly Connection[]; parent?: EntityRef }>;

/** @emoji 🔘 Port snapshot. */
export type Port = Readonly<{ id: string; label?: string; code?: string | null; name?: string; compatiblePorts?: readonly EntityRef[] }>;

/** @emoji ⚓ Connector snapshot. */
export type Connector = Readonly<{ id: string; name?: string; port?: Port }>;

/** @emoji 💾 Representation snapshot. */
export type Representation = Readonly<{ id: string; name?: string; file?: EntityRef; tags?: readonly EntityRef[] }>;

/** @emoji 🏠 Type snapshot. */
export type Type = Readonly<{
	id: string;
	name?: string;
	description?: string;
	unit?: string;
	representations?: readonly Representation[];
	ports?: readonly Port[];
	connectors?: readonly Connector[];
	parent?: EntityRef;
}>;

/** @emoji 📄 Kit file snapshot. */
export type KitFile = Readonly<{ id: string; name?: string; description?: string; url?: string; folder?: EntityRef }>;

/** @emoji 📁 Kit folder snapshot. */
export type Folder = Readonly<{ id: string; name?: string; path?: string; description?: string; parent?: EntityRef }>;

/** @emoji 🏛️ Typology snapshot (groups types and designs). */
export type Typology = Readonly<{ id: string; name: string; types: readonly Type[]; designs: readonly Design[] }>;

/** @emoji 👨‍👩‍👦 Family snapshot. */
export type Family = Readonly<{ id: string; name?: string; ports?: readonly Port[] }>;

/** @emoji 👤 Author snapshot. */
export type Author = Readonly<{ id: string; name?: string }>;

/** @emoji 🔢 Quality snapshot. */
export type Quality = Readonly<{ id: string; key?: string; value?: string }>;

/** @emoji 🏷️ Tag snapshot. */
export type Tag = Readonly<{ id: string; name?: string }>;

/** @emoji 📦 Read-model kit snapshot materialized from rs GraphQL (never authoritative). */
export type Kit = Readonly<{
	id: string;
	name?: string;
	description?: string;
	version?: string;
	createdAt?: string;
	updatedAt?: string;
	types?: readonly Type[];
	designs?: readonly Design[];
	typologies?: readonly Typology[];
	families?: readonly Family[];
	files?: readonly KitFile[];
	folders?: readonly Folder[];
	authors?: readonly Author[];
	qualities?: readonly Quality[];
	tags?: readonly Tag[];
}>;

/** @emoji 📡 Observable kit read model (implemented by {@link SemioKitClient} and fixed snapshots). */
export interface SemioKitSource {
	getSnapshot(): Kit;
	subscribe(listener: () => void): Unsubscribe;
}
//#endregion 📦KitSnapshot

//#region 🧾KitRead
type GraphqlNode = Readonly<Record<string, unknown>>;

const SEMIO_KIT_READ_SELECTION = `id name description version createdAt updatedAt
hasDesigns { edges { node { id name description unit
  hasPieces { edges { node { id name description blueprint { __typename id } position { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } } } } }
  hasConnections { edges { node { id description parent { referencesPiece { id } referencesConnector { id } } child { referencesPiece { id } referencesConnector { id } } } } }
} } }
hasTypes { edges { node { id name description unit
  hasConnectors { edges { node { id name port { id label code copatibleWith { edges { node { id } } } } } } }
  hasPorts { edges { node { id label code copatibleWith { edges { node { id } } } } } }
  hasRepresentations { edges { node { id name file { id } tags { edges { node { id } } } } } }
} } }
hasTypologies { edges { node { id name hasTypes { edges { node { id } } } hasDesigns { edges { node { id } } } } } }
hasFamilies { edges { node { id name } } }
qualities { edges { node { id key value } } }
tags { edges { node { id name } } }
authors { edges { node { id name } } }
hasFolders { edges { node { id name path description files { edges { node { id name url description folderId } } } } } }
hasFiles { edges { node { id name url description folderId } } }`;

function readNodes(parent: unknown, key: string): readonly GraphqlNode[] {
	const edges = (parent as Record<string, { edges?: readonly { node?: GraphqlNode | null }[] } | undefined> | null | undefined)?.[key]?.edges ?? [];
	return edges.map((edge) => edge.node).filter((node): node is GraphqlNode => node != null);
}

function readText(value: unknown): string | undefined {
	return value == null || value === "" ? undefined : String(value);
}

function readRef(value: unknown): EntityRef | undefined {
	const id = (value as { id?: unknown } | null | undefined)?.id;
	return id == null || id === "" ? undefined : { id: String(id) };
}

function readPort(node: GraphqlNode): Port {
	const compatiblePorts = readNodes(node, "copatibleWith").map((port) => ({ id: String(port["id"]) }));
	return {
		id: String(node["id"]),
		label: readText(node["label"]),
		code: node["code"] == null ? null : String(node["code"]),
		...(compatiblePorts.length > 0 ? { compatiblePorts } : {}),
	};
}

function readSide(value: unknown): ConnectionSide | undefined {
	if (value == null || typeof value !== "object") return undefined;
	const side = value as GraphqlNode;
	return { piece: readRef(side["referencesPiece"]), connector: readRef(side["referencesConnector"]) };
}

function readPiece(node: GraphqlNode): Piece {
	const blueprint = node["blueprint"] as GraphqlNode | null | undefined;
	const blueprintRef = readRef(blueprint);
	const position = node["position"] as PiecePosition | null | undefined;
	return {
		id: String(node["id"]),
		name: readText(node["name"]),
		description: readText(node["description"]),
		...(blueprintRef ? (blueprint?.["__typename"] === "Design" ? { design: blueprintRef } : { type: blueprintRef }) : {}),
		...(position ? { position } : {}),
	};
}

function readDesign(node: GraphqlNode): Design {
	return {
		id: String(node["id"]),
		name: readText(node["name"]),
		description: readText(node["description"]),
		unit: readText(node["unit"]),
		pieces: readNodes(node, "hasPieces").map(readPiece),
		connections: readNodes(node, "hasConnections").map((connection) => ({
			id: String(connection["id"]),
			description: readText(connection["description"]),
			parent: readSide(connection["parent"]),
			child: readSide(connection["child"]),
		})),
	};
}

function readType(node: GraphqlNode): Type {
	return {
		id: String(node["id"]),
		name: readText(node["name"]),
		description: readText(node["description"]),
		unit: readText(node["unit"]),
		representations: readNodes(node, "hasRepresentations").map((representation) => ({
			id: String(representation["id"]),
			name: readText(representation["name"]),
			file: readRef(representation["file"]),
			tags: readNodes(representation, "tags").map((tag) => ({ id: String(tag["id"]) })),
		})),
		ports: readNodes(node, "hasPorts").map(readPort),
		connectors: readNodes(node, "hasConnectors").map((connector) => {
			const port = connector["port"] as GraphqlNode | null | undefined;
			return { id: String(connector["id"]), name: readText(connector["name"]), ...(port ? { port: readPort(port) } : {}) };
		}),
	};
}

function readFile(node: GraphqlNode): KitFile {
	const folderId = readText(node["folderId"]);
	return {
		id: String(node["id"]),
		name: readText(node["name"]),
		description: readText(node["description"]),
		url: readText(node["url"]),
		...(folderId ? { folder: { id: folderId } } : {}),
	};
}

/** @emoji 📸 Materializes the {@link Kit} read model of a store's WIP kit from rs GraphQL. */
export async function readSemioKitSnapshot(store: JsStore): Promise<Kit> {
	const data = await store.readKitInner(SEMIO_KIT_READ_SELECTION);
	if (!data) return { id: "" };
	const designs = readNodes(data, "hasDesigns").map(readDesign);
	const types = readNodes(data, "hasTypes").map(readType);
	const designById = new Map(designs.map((design) => [design.id, design]));
	const typeById = new Map(types.map((type) => [type.id, type]));
	const folders = readNodes(data, "hasFolders");
	const filesById = new Map<string, KitFile>();
	for (const file of [...readNodes(data, "hasFiles"), ...folders.flatMap((folder) => readNodes(folder, "files"))].map(readFile)) filesById.set(file.id, file);
	return {
		id: String(data["id"] ?? ""),
		name: readText(data["name"]),
		description: readText(data["description"]),
		version: readText(data["version"]),
		createdAt: readText(data["createdAt"]),
		updatedAt: readText(data["updatedAt"]),
		designs,
		types,
		typologies: readNodes(data, "hasTypologies").map((typology) => ({
			id: String(typology["id"]),
			name: readText(typology["name"]) ?? String(typology["id"]),
			types: readNodes(typology, "hasTypes").flatMap((row) => typeById.get(String(row["id"])) ?? []),
			designs: readNodes(typology, "hasDesigns").flatMap((row) => designById.get(String(row["id"])) ?? []),
		})),
		families: readNodes(data, "hasFamilies").map((family) => ({ id: String(family["id"]), name: readText(family["name"]) })),
		files: [...filesById.values()],
		folders: folders.map((folder) => ({ id: String(folder["id"]), name: readText(folder["name"]), path: readText(folder["path"]), description: readText(folder["description"]) })),
		authors: readNodes(data, "authors").map((author) => ({ id: String(author["id"]), name: readText(author["name"]) })),
		qualities: readNodes(data, "qualities").map((quality) => ({ id: String(quality["id"]), key: readText(quality["key"]), value: readText(quality["value"]) })),
		tags: readNodes(data, "tags").map((tag) => ({ id: String(tag["id"]), name: readText(tag["name"]) })),
	};
}
//#endregion 🧾KitRead

//#region 🗜️KitArchive
type BundleRow = Record<string, unknown>;

/** @emoji 🧾 Recursively flattens `{ items: [...] }` and Relay `edges` of bundle JSON. */
function flattenBundleValue(value: unknown): unknown {
	if (value == null || typeof value !== "object") return value;
	if (Array.isArray(value)) return value.map(flattenBundleValue);
	const row = value as BundleRow;
	if (Array.isArray(row["items"])) return (row["items"] as unknown[]).map(flattenBundleValue);
	if (Array.isArray(row["edges"])) {
		return (row["edges"] as unknown[]).flatMap((edge) => (edge != null && typeof edge === "object" && "node" in edge ? [flattenBundleValue((edge as BundleRow)["node"])] : []));
	}
	return Object.fromEntries(Object.entries(row).map(([key, entry]) => [key, flattenBundleValue(entry)]));
}

/** @emoji 🧾 Lifts `*.kit.semio.json` envelopes (`initialKit` / `wip.initialKit`) and flattens bundle lists. */
export function decodeSemioKitEnvelope(value: unknown): unknown {
	let inner = value;
	if (inner && typeof inner === "object" && !Array.isArray(inner)) {
		const top = inner as BundleRow;
		const wip = top["wip"] as BundleRow | undefined;
		if (top["initialKit"] != null && typeof top["initialKit"] === "object") inner = top["initialKit"];
		else if (wip != null && typeof wip === "object" && wip["initialKit"] != null && typeof wip["initialKit"] === "object") inner = wip["initialKit"];
	}
	return flattenBundleValue(inner);
}

/** @emoji 🧾 Reads the kit root (`{ id, … }`) of a decoded semio bundle value. */
export function semioKitRootFromBundle(value: unknown): Readonly<BundleRow> | null {
	const root = decodeSemioKitEnvelope(value);
	return root != null && typeof root === "object" && !Array.isArray(root) && "id" in root ? (root as BundleRow) : null;
}

/** @emoji 📤 Wraps a kit snapshot in the `wip.initialKit` envelope accepted by {@link decodeSemioKitArchive}. */
export function semioKitToEnvelope(kit: Kit): { readonly wip: { readonly initialKit: Kit } } {
	return { wip: { initialKit: kit } };
}

/** @emoji 🗂️ Binary kit file contents keyed by kit file id (never kit data; blobs live outside rs). */
export class SemioKitFileUrls {
	private readonly created = new Map<string, string>();

	constructor(
		private readonly direct: ReadonlyMap<string, string>,
		private readonly entries: ReadonlyMap<string, Readonly<{ bytes: Uint8Array; mime: string }>>,
	) {}

	/** @emoji 🔗 Fetchable URL (data/blob/http/relative) for a kit file id, if its content is known. */
	url(fileId: string): string | undefined {
		const known = this.direct.get(fileId) ?? this.created.get(fileId);
		if (known) return known;
		const entry = this.entries.get(fileId);
		if (!entry || typeof URL === "undefined" || typeof URL.createObjectURL !== "function") return undefined;
		const url = URL.createObjectURL(new Blob([entry.bytes as Uint8Array<ArrayBuffer>], { type: entry.mime }));
		this.created.set(fileId, url);
		return url;
	}

	dispose(): void {
		for (const url of this.created.values()) URL.revokeObjectURL(url);
		this.created.clear();
	}
}

/** @emoji 📦 Decoded kit archive: rs projection JSON plus kit file contents. */
export type SemioKitArchive = Readonly<{ projection: string; kitId: string; fileUrls: SemioKitFileUrls }>;

const SEMIO_KIT_JSON_ENTRY_NAMES = ["kit.semio.json", "kit.json"] as const;

function fileMime(name: string): string {
	const lower = name.toLowerCase();
	if (lower.endsWith(".glb")) return "model/gltf-binary";
	if (lower.endsWith(".gltf")) return "model/gltf+json";
	if (lower.endsWith(".png")) return "image/png";
	if (lower.endsWith(".jpg") || lower.endsWith(".jpeg")) return "image/jpeg";
	if (lower.endsWith(".svg")) return "image/svg+xml";
	return "application/octet-stream";
}

function isFetchableUrl(value: unknown): value is string {
	return typeof value === "string" && /^(?:data:|blob:|https?:|\/)/i.test(value);
}

function kitJsonEntryName(names: readonly string[]): string | undefined {
	const depth = (name: string) => name.split("/").length;
	const sorted = [...names].filter((name) => !name.startsWith(".") && !name.includes("/.")).sort((left, right) => depth(left) - depth(right));
	for (const candidate of SEMIO_KIT_JSON_ENTRY_NAMES) {
		const match = sorted.find((name) => name === candidate || name.endsWith(`/${candidate}`));
		if (match) return match;
	}
	return sorted.find((name) => name.endsWith(".semio.json")) ?? sorted.find((name) => name.endsWith(".json"));
}

/** @emoji 📥 Decodes gzip / zip / JSON kit bytes into a rs projection and kit file contents (relative paths resolve against {@code baseUrl}). */
export function decodeSemioKitArchive(data: Uint8Array, baseUrl?: string): SemioKitArchive {
	let bytes = data;
	if (bytes.length >= 2 && bytes[0] === 0x1f && bytes[1] === 0x8b) bytes = inflateGzip(bytes);
	let entries: Readonly<Record<string, Uint8Array>> = {};
	let entryRoot = "";
	if (bytes.length >= 4 && bytes[0] === 0x50 && bytes[1] === 0x4b && bytes[2] === 0x03 && bytes[3] === 0x04) {
		entries = extractZip(bytes);
		const kitEntry = kitJsonEntryName(Object.keys(entries));
		if (!kitEntry) throw new Error("semio/react: kit archive contains no kit json");
		entryRoot = kitEntry.includes("/") ? kitEntry.slice(0, kitEntry.lastIndexOf("/") + 1) : "";
		bytes = entries[kitEntry]!;
	}
	const root = semioKitRootFromBundle(JSON.parse(new TextDecoder().decode(bytes)));
	if (!root) throw new Error("semio/react: kit json has no kit root");
	const folderPathById = new Map<string, string>();
	for (const folder of (root["folders"] as readonly BundleRow[] | undefined) ?? []) {
		const path = String(folder["path"] ?? folder["name"] ?? "").replace(/^\/+|\/+$/g, "");
		if (folder["id"] != null) folderPathById.set(String(folder["id"]), path);
	}
	const direct = new Map<string, string>();
	const archived = new Map<string, Readonly<{ bytes: Uint8Array; mime: string }>>();
	const files = ((root["files"] as readonly BundleRow[] | undefined) ?? []).map((file) => {
		const { blob, ...rest } = file;
		const id = String(file["id"] ?? "");
		const name = String(file["name"] ?? "");
		const folderId = readRef(file["folder"])?.id;
		const relativePath = [folderId ? folderPathById.get(folderId) : "", name].filter(Boolean).join("/");
		const entry = entries[`${entryRoot}${relativePath}`] ?? entries[relativePath];
		if (isFetchableUrl(blob)) direct.set(id, blob);
		else if (isFetchableUrl(file["url"])) direct.set(id, file["url"]);
		else if (entry) archived.set(id, { bytes: entry, mime: fileMime(name) });
		else if (baseUrl && relativePath) direct.set(id, new URL(relativePath, baseUrl).href);
		return rest;
	});
	return {
		projection: JSON.stringify({ ...root, files }),
		kitId: String(root["id"] ?? ""),
		fileUrls: new SemioKitFileUrls(direct, archived),
	};
}

async function readSourceBytes(data: ArrayBuffer | Uint8Array | Blob): Promise<Uint8Array> {
	if (data instanceof Uint8Array) return data;
	if (data instanceof ArrayBuffer) return new Uint8Array(data);
	return new Uint8Array(await data.arrayBuffer());
}
//#endregion 🗜️KitArchive

//#region 🧩KitClient
/** @emoji 📥 Where a {@link SemioKitClient} loads its kit from. */
export type SemioKitClientSource =
	| Readonly<{ kind: "bytes"; data: ArrayBuffer | Uint8Array | Blob; baseUrl?: string }>
	| Readonly<{ kind: "url"; url: string }>
	| Readonly<{ kind: "empty"; name: string }>
	| Readonly<{ kind: "http"; serverUrl: string }>;

/** @emoji 🧬 Blueprint of a piece (type or nested design). */
export type SemioPieceBlueprint = Readonly<{ kind: "Type" | "Design"; id: string }>;

/**
 * @emoji 🧩 One open kit: owns its `@semio/js` {@link Session}, mirrors the rs read model and is the single sketchpad-side command choke point ({@link execute}).
 * Every mutation still funnels through {@link Session.executeOperation} (hub middlewares attach to {@link session}).
 */
export class SemioKitClient implements SemioKitSource {
	private readonly listeners = new Set<() => void>();
	private readonly detach: readonly Unsubscribe[];
	private refreshing: Promise<Kit> | null = null;
	private stale = false;
	private disposed = false;

	private constructor(
		readonly session: Session,
		readonly store: JsStore,
		private readonly fileUrls: SemioKitFileUrls,
		private snapshot: Kit,
	) {
		this.detach = [session.onOperation(() => void this.refresh()), session.subscribe(() => void this.refresh())];
	}

	/** @emoji 📥 Opens a kit into a fresh in-memory rs session (bytes / URL / empty) or a `semio-store` HTTP session. */
	static async open(source: SemioKitClientSource): Promise<SemioKitClient> {
		if (source.kind === "http") {
			const session = await Session.openHttp(source.serverUrl);
			return SemioKitClient.attach(session, new SemioKitFileUrls(new Map(), new Map()));
		}
		const session = await Session.openInMemory();
		try {
			const store = await SemioKitClient.firstStore(session);
			if (source.kind === "empty") {
				const client = new SemioKitClient(session, store, new SemioKitFileUrls(new Map(), new Map()), await readSemioKitSnapshot(store));
				if (source.name.trim()) {
					const renamed = await client.execute((kit) => kit.rename(source.name.trim()));
					if (!renamed.ok) throw new Error(`semio/react: rename failed: ${renamed.error.message}`);
				}
				return client;
			}
			const archive =
				source.kind === "url"
					? decodeSemioKitArchive(new Uint8Array(await (await fetch(source.url)).arrayBuffer()), new URL(source.url, globalThis.location?.href ?? "http://localhost/").href)
					: decodeSemioKitArchive(await readSourceBytes(source.data), source.baseUrl);
			const installed = await store.installProjection(archive.projection);
			if (!installed.ok) throw new Error(`semio/react: installProjection failed: ${installed.error.message}`);
			return new SemioKitClient(session, store, archive.fileUrls, await readSemioKitSnapshot(store));
		} catch (error) {
			await session.dispose();
			throw error;
		}
	}

	private static async firstStore(session: Session): Promise<JsStore> {
		const store = (await session.stores())[0];
		if (!store) throw new Error("semio/react: session has no stores");
		return store;
	}

	private static async attach(session: Session, fileUrls: SemioKitFileUrls): Promise<SemioKitClient> {
		try {
			const store = await SemioKitClient.firstStore(session);
			return new SemioKitClient(session, store, fileUrls, await readSemioKitSnapshot(store));
		} catch (error) {
			await session.dispose();
			throw error;
		}
	}

	get kitId(): string {
		return this.snapshot.id;
	}

	getSnapshot(): Kit {
		return this.snapshot;
	}

	subscribe(listener: () => void): Unsubscribe {
		this.listeners.add(listener);
		return () => {
			this.listeners.delete(listener);
		};
	}

	/** @emoji 🔄 Re-materializes the read model from rs (coalesces concurrent requests) and notifies subscribers. */
	refresh(): Promise<Kit> {
		if (this.disposed) return Promise.resolve(this.snapshot);
		if (this.refreshing) {
			this.stale = true;
			return this.refreshing;
		}
		this.refreshing = (async () => {
			try {
				do {
					this.stale = false;
					this.snapshot = await readSemioKitSnapshot(this.store);
				} while (this.stale && !this.disposed);
			} finally {
				this.refreshing = null;
			}
			for (const listener of [...this.listeners]) listener();
			return this.snapshot;
		})();
		return this.refreshing;
	}

	/** @emoji ⚡ Runs kit commands on the WIP kit and resolves after the read model reflects them. */
	async execute(run: (kit: SemioKitCommands) => Promise<SetResult>): Promise<SetResult> {
		const result = await run(await this.store.wip().theKit().kit());
		await this.refresh();
		return result;
	}

	/** @emoji 🔗 Fetchable URL of a kit file's content (representation geometry, images, …). */
	fileUrl(fileId: string): string | undefined {
		return this.fileUrls.url(fileId);
	}

	/** @emoji 📁 Lazy virtual file system children of a kit node. */
	fileSystemChildren(parent: SemioFileSystemParentRef): Promise<readonly SemioFileSystemChildRef[]> {
		return fetchSemioFileSystemChildren(this.store, parent);
	}

	/** @emoji 🔗 Types and designs a design references transitively (rs computed). */
	async designReferences(designId: string): Promise<Readonly<{ types: readonly string[]; designs: readonly string[] }>> {
		const design = this.store.design(designId);
		const [types, designs] = await Promise.all([design.referencesTypesTransitive(), design.referencesDesignsTransitive()]);
		return { types: types.map((type) => type.id), designs: designs.map((row) => row.id).filter((id) => id !== designId) };
	}

	/** @emoji 🧬 Blueprint of a piece (rs computed). */
	async pieceBlueprint(designId: string, pieceId: string): Promise<SemioPieceBlueprint | null> {
		const blueprint = await this.store.design(designId).piece(pieceId).blueprint();
		return blueprint ? { kind: blueprint.blueprintKind, id: blueprint.id } : null;
	}

	async dispose(): Promise<void> {
		if (this.disposed) return;
		this.disposed = true;
		for (const off of this.detach) off();
		this.listeners.clear();
		this.fileUrls.dispose();
		await this.session.dispose();
	}
}

/** @emoji 📌 Read-only fixed {@link Kit} snapshot source (static hosts, previews, tests). */
export class SemioKitSnapshotSource implements SemioKitSource {
	constructor(private readonly kit: Kit) {}

	getSnapshot(): Kit {
		return this.kit;
	}

	subscribe(): Unsubscribe {
		return () => {};
	}
}
//#endregion 🧩KitClient

//#region 🪝Hooks
/** @emoji 🪝 Subscribes a React component to a {@link SemioKitSource} read model. */
export function useSemioKit(source: SemioKitSource): Kit {
	return useSyncExternalStore(
		(listener) => source.subscribe(listener),
		() => source.getSnapshot(),
		() => source.getSnapshot(),
	);
}

/** @emoji 🪝 Selects a slice of a {@link SemioKitSource} read model. */
export function useSemioKitSelector<T>(source: SemioKitSource, select: (kit: Kit) => T): T {
	return select(useSemioKit(source));
}
//#endregion 🪝Hooks
