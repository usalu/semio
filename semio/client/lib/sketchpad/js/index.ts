// #region 🧲Header
// 2024-2026 Ueli Saluz <ueli@semio-tech.com>
// Render-agnostic sketchpad product: {@link Platform} apps, {@link Component} snapshots, controller-owned {@link Store}s.
// #endregion 🧲Header

//#region 🔌Adapters
import type { Design, Kit, Session, SetResult, Type } from "@semio/js";
import { Kit as JsKitEntity, Session as SemioSession } from "@semio/js";
import { gunzipSync } from "fflate";
import type { Store as JsKitStore } from "@semio/js";
import {
	CommandBus,
	Component,
	Controller,
	ObservableCell,
	Panel,
	Platform,
	PluginHost,
	Store,
	PlatformTopologyStore,
	PlatformTopologyPayload,
	platformTopologyStoreId,
	PLATFORM_TOPOLOGY_STORE_PREFIX,
	Table,
	buildPanelWindowBody,
	buildPuzzle2dWindowBody,
	buildPuzzle5dWindowBody,
	buildTableWindowBody,
	createDefaultLayout,
	createTabStackLayout,
	registerPlatformComponent,
	registerSidePanelBody,
	registerWindowBody,
	type ComponentKind,
	type PanelModel,
	type PlatformSpec,
	type PluginManifest,
	type PluginModule,
	type Puzzle2dModel,
	type Puzzle5dModel,
	type TableModel,
	type SideTabSpec,
	type UiNode,
	type WindowBodyViewContext,
	getPlatformControllerById,
} from "@framework/platform/core";
import type { PlatformBreadcrumbItem, SearchItemSpec } from "@framework/core";
//#endregion 🔌Adapters

//#region 🔖KitImport
type SemioBundleJson = Record<string, unknown>;

/** @emoji 🧾 Recursively flattens `{ items: [...] }` and Relay `edges` for GraphQL install payloads. */
function semioDenormalizeBundleValue(v: unknown): unknown {
	if (v == null || typeof v !== "object") return v;
	if (Array.isArray(v)) return v.map(semioDenormalizeBundleValue);
	const o = v as SemioBundleJson;
	if (Array.isArray(o["items"])) return (o["items"] as unknown[]).map(semioDenormalizeBundleValue);
	if (Array.isArray(o["edges"])) {
		const out: unknown[] = [];
		for (const e of o["edges"] as unknown[]) {
			if (e != null && typeof e === "object" && !Array.isArray(e) && "node" in (e as SemioBundleJson)) {
				out.push(semioDenormalizeBundleValue((e as SemioBundleJson)["node"]));
			}
		}
		return out;
	}
	const flat: SemioBundleJson = {};
	for (const [k, val] of Object.entries(o)) flat[k] = semioDenormalizeBundleValue(val) as never;
	return flat;
}

/** @emoji 🧾 Lifts `*.kit.semio.json` (`initialKit` / `wip.initialKit`) then flattens bundle lists. */
export function decodeKitSemioEnvelopeToFullFromValue(v: unknown): unknown {
	let inner: unknown = v;
	if (inner && typeof inner === "object" && !Array.isArray(inner)) {
		const top = inner as SemioBundleJson;
		if (top["initialKit"] != null && typeof top["initialKit"] === "object" && !Array.isArray(top["initialKit"])) {
			inner = top["initialKit"];
		} else if (top["wip"] != null && typeof top["wip"] === "object" && !Array.isArray(top["wip"])) {
			const wr = (top["wip"] as SemioBundleJson)["initialKit"];
			if (wr != null && typeof wr === "object" && !Array.isArray(wr)) inner = wr;
		}
	}
	return semioDenormalizeBundleValue(inner);
}

/** @emoji 🧾 Reads a kit DTO root from a decoded semio bundle value. */
export function sketchpadKitFromDecodedBundle(value: unknown): Kit | null {
	const denorm = decodeKitSemioEnvelopeToFullFromValue(value);
	if (denorm == null || typeof denorm !== "object" || Array.isArray(denorm)) return null;
	if ("id" in denorm) return denorm as Kit;
	return null;
}

/** @emoji 📦 Decode gzip-or-JSON kit bytes into a live {@link Kit} via {@link Session.openInMemory}. */
export async function importKit(
	data: ArrayBuffer | Uint8Array | Blob | File | string,
): Promise<{ readonly kit: Kit; readonly session: Session; readonly portCompatSource: Kit }> {
	let bytes: Uint8Array;
	if (typeof data === "string") {
		const res = await fetch(data);
		bytes = new Uint8Array(await res.arrayBuffer());
	} else if (data instanceof Uint8Array) {
		bytes = data;
	} else if (data instanceof ArrayBuffer) {
		bytes = new Uint8Array(data);
	} else {
		bytes = new Uint8Array(await data.arrayBuffer());
	}
	if (bytes.length >= 2 && bytes[0] === 0x1f && bytes[1] === 0x8b) {
		bytes = gunzipSync(bytes);
	}
	const text = new TextDecoder().decode(bytes);
	const plainUnknown = decodeKitSemioEnvelopeToFullFromValue(JSON.parse(text));
	const payload = typeof plainUnknown === "object" && plainUnknown != null ? JSON.stringify(plainUnknown) : String(plainUnknown);
	const bundleKit = sketchpadKitFromDecodedBundle(plainUnknown);
	const session = await SemioSession.openInMemory();
	const stores = await session.stores();
	if (stores.length === 0) throw new Error("semio/sketchpad: importKit found zero stores after openInMemory");
	const store = stores[0]!;
	const installed = await store.installProjection(payload);
	if (!installed.ok) throw new Error(`semio/sketchpad: importKit installProjection failed: ${installed.error?.message ?? "unknown"}`);
	const kitDto = await sketchpadKitDtoFromJsStore(store);
	const portCompatSource = (bundleKit ?? kitDto) as Kit;
	const compat = sketchpadMergePortCompatMaps(sketchpadExtractPortCompatById(portCompatSource), sketchpadExtractPortCompatById(kitDto));
	const kit = sketchpadApplyPortCompatById(kitDto, compat);
	return { kit, session, portCompatSource };
}

/** @emoji 📤 Wraps a kit DTO in the `wip.initialKit` envelope used by {@link importKit}. */
export function sketchpadKitToSemioEnvelope(kit: Kit): { readonly wip: { readonly initialKit: Kit } } {
	return { wip: { initialKit: kit } };
}

/** @emoji 💾 Triggers a browser download of kit JSON (semio envelope). */
export function sketchpadDownloadKitJson(kit: Kit, filename?: string): void {
	if (typeof document === "undefined") return;
	const json = JSON.stringify(sketchpadKitToSemioEnvelope(kit), null, 2);
	const blob = new Blob([json], { type: "application/json" });
	const url = URL.createObjectURL(blob);
	const anchor = document.createElement("a");
	const safeName = (kit.name ?? kit.id ?? "kit").replace(/[^\w.-]+/g, "-");
	anchor.href = url;
	anchor.download = filename ?? `${safeName}.kit.semio.json`;
	anchor.click();
	URL.revokeObjectURL(url);
}

/** @emoji 📋 Copies kit JSON (semio envelope) to the clipboard when available. */
export async function sketchpadCopyKitJsonToClipboard(kit: Kit): Promise<boolean> {
	if (typeof navigator === "undefined" || !navigator.clipboard?.writeText) return false;
	await navigator.clipboard.writeText(JSON.stringify(sketchpadKitToSemioEnvelope(kit), null, 2));
	return true;
}
//#endregion 🔖KitImport

//#region 🔖KitHost
export type SketchpadKitPersistenceKind = "temporary" | "file" | "folder" | "remote" | "fixture";

/** @emoji 🏭 Host-provided kit open factory (Electron, VS Code, browser file picker, …). */
export type SketchpadKitBackendFactory = () => Promise<SemioKitStore>;

let sketchpadKitBackendFactories: Partial<Record<SketchpadKitPersistenceKind, SketchpadKitBackendFactory>> = {};

function sketchpadKitStoreFromFactory(result: SemioKitStore | SemioKitStoreBackend): SemioKitStore {
	return result instanceof SemioKitStore ? result : new SemioKitStore(result);
}

function sketchpadPromptServerUrl(preset?: string): string | null {
	if (typeof window === "undefined" || typeof window.prompt !== "function") return preset ?? null;
	return window.prompt("Semio store URL", preset ?? "http://localhost:8080");
}

/** @emoji 🌐 Default browser remote kit factory ({@link Session.openHttp}). */
export async function sketchpadDefaultRemoteKitFactory(): Promise<SemioKitStore> {
	const serverUrl = sketchpadPromptServerUrl()?.trim();
	if (!serverUrl) throw new Error("semio/sketchpad: remote kit open cancelled");
	return sketchpadOpenRemoteKitStore(serverUrl);
}

/** @emoji 🌐 Opens an HTTP {@link Session} kit and returns a {@link SemioJsKitStore}. */
export async function sketchpadOpenRemoteKitStore(serverUrl: string): Promise<SemioJsKitStore> {
	const session = await SemioSession.openHttp(serverUrl);
	const stores = await session.stores();
	const jsStore = stores[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("semio/sketchpad: remote session has no stores");
	}
	return createSemioKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
}

/** @emoji 🔧 Registers host kit open factories used by {@link SketchpadShellController} `openKit` commands. */
export function configureSketchpadKitFactories(factories: Partial<Record<SketchpadKitPersistenceKind, SketchpadKitBackendFactory>>): void {
	sketchpadKitBackendFactories = { remote: sketchpadDefaultRemoteKitFactory, ...sketchpadKitBackendFactories, ...factories };
}

configureSketchpadKitFactories({});

/** @emoji 📂 Picks a kit archive or JSON file in the browser (File System Access API or hidden input). */
export async function sketchpadPickKitImportFile(): Promise<File | null> {
	if (typeof window === "undefined") return null;
	const accept = {
		"application/json": [".json", ".semio.json"],
		"application/zip": [".zip", ".semio.zip"],
		"application/gzip": [".gz"],
		"application/x-gzip": [".gz"],
	};
	if ("showOpenFilePicker" in window) {
		try {
			const handles = await (
				window as Window & { showOpenFilePicker: (o: unknown) => Promise<FileSystemFileHandle[]> }
			).showOpenFilePicker({
				multiple: false,
				types: [{ description: "Semio kit", accept }],
			});
			const handle = handles[0];
			return handle ? await handle.getFile() : null;
		} catch {
			return null;
		}
	}
	return new Promise((resolve) => {
		const input = document.createElement("input");
		input.type = "file";
		input.accept = ".json,.semio.json,.zip,.semio.zip,.gz,application/json,application/zip";
		input.onchange = () => resolve(input.files?.[0] ?? null);
		input.click();
	});
}

/** @emoji 📂 Opens a user-selected kit file via {@link importKit} and returns a {@link SemioJsKitStore}. */
export async function sketchpadBrowserFileKitFactory(): Promise<SemioJsKitStore> {
	const file = await sketchpadPickKitImportFile();
	if (!file) throw new Error("semio/sketchpad: file kit open cancelled");
	const { session, portCompatSource } = await importKit(file);
	const jsStore = (await session.stores())[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("semio/sketchpad: file kit open found no stores");
	}
	return createSemioKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose(), portCompatSource });
}

/** @emoji 📁 Opens a folder kit when {@link showDirectoryPicker} is available (kit.semio.json at folder root). */
export async function sketchpadBrowserFolderKitFactory(): Promise<SemioJsKitStore> {
	if (typeof window === "undefined" || !("showDirectoryPicker" in window)) {
		throw new Error("semio/sketchpad: folder kit open requires showDirectoryPicker");
	}
	const dir = await (
		window as Window & { showDirectoryPicker: () => Promise<FileSystemDirectoryHandle> }
	).showDirectoryPicker();
	const kitFile =
		(await dir.getFileHandle("kit.semio.json", { create: false }).then((h) => h.getFile()).catch(() => null)) ??
		(await dir.getFileHandle("wip/initialKit/kit.semio.json", { create: false }).then((h) => h.getFile()).catch(() => null));
	if (!kitFile) throw new Error("semio/sketchpad: no kit.semio.json in selected folder");
	const { session, portCompatSource } = await importKit(kitFile);
	const jsStore = (await session.stores())[0];
	if (!jsStore) {
		await session.dispose();
		throw new Error("semio/sketchpad: folder kit open found no stores");
	}
	return createSemioKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose(), portCompatSource });
}

/** @emoji 🌐 Registers browser file/folder/remote kit factories for {@link SketchpadShellController}. */
export function sketchpadConfigureBrowserKitFactories(): void {
	if (typeof window === "undefined") return;
	configureSketchpadKitFactories({
		file: sketchpadBrowserFileKitFactory,
		folder: sketchpadBrowserFolderKitFactory,
		remote: sketchpadDefaultRemoteKitFactory,
	});
}

let sketchpadHomeDropzoneInstalled = false;
let sketchpadHomeDropzoneDragDepth = 0;

const SKETCHPAD_HOME_DROPZONE_OVERLAY_ID = "semio-sketchpad-home-dropzone-overlay";
const SKETCHPAD_HOME_KIT_FILE_INPUT_ID = "semio-sketchpad-home-kit-file-input";

/** @emoji 🏠 Home table surface id (dropzone host binds in {@link boot.ts}). */
export const SKETCHPAD_SURFACE_HOME_TABLE = "semio.sketchpad.surface.home.table/v1";

function sketchpadHomeRouteActive(): boolean {
	return (getSketchpadPlatform()?.uri.split("?")[0] ?? "/") === "/";
}

function sketchpadTransferHasKitArchive(transfer: DataTransfer | null): boolean {
	if (!transfer) return false;
	if (transfer.types.includes("Files")) return true;
	const file = transfer.files?.[0];
	if (!file) return false;
	return /\.(semio\.)?zip$/i.test(file.name) || file.type.includes("zip");
}

/** @emoji 🖼️ Toggles the full-screen home kit import drop overlay. */
export function sketchpadSetHomeDropzoneOverlayVisible(visible: boolean): void {
	if (typeof document === "undefined") return;
	let overlay = document.getElementById(SKETCHPAD_HOME_DROPZONE_OVERLAY_ID);
	if (!overlay && visible) {
		overlay = document.createElement("div");
		overlay.id = SKETCHPAD_HOME_DROPZONE_OVERLAY_ID;
		overlay.setAttribute("data-testid", "sketchpad-home-dropzone-overlay");
		overlay.className =
			"pointer-events-none fixed inset-0 z-50 flex items-center justify-center bg-base/80 backdrop-blur-sm";
		const inner = document.createElement("div");
		inner.className = "flex flex-col items-center gap-2 px-6 text-center";
		const title = document.createElement("p");
		title.className = "text-lg font-medium";
		title.textContent = "Drop kit archive";
		const hint = document.createElement("p");
		hint.className = "text-sm text-muted-foreground";
		hint.textContent = "Release a .zip or .semio.zip file to import";
		inner.append(title, hint);
		overlay.append(inner);
		document.body.appendChild(overlay);
	}
	if (overlay) overlay.classList.toggle("hidden", !visible);
}

function sketchpadEnsureHomeKitFileInput(): HTMLInputElement {
	let input = document.getElementById(SKETCHPAD_HOME_KIT_FILE_INPUT_ID) as HTMLInputElement | null;
	if (!input) {
		input = document.createElement("input");
		input.type = "file";
		input.id = SKETCHPAD_HOME_KIT_FILE_INPUT_ID;
		input.accept = ".zip,.semio.zip,application/zip,application/x-zip-compressed";
		input.className = "hidden";
		input.setAttribute("data-testid", SKETCHPAD_HOME_KIT_FILE_INPUT_ID);
		document.body.appendChild(input);
	}
	return input;
}

/** @emoji 📂 Opens the hidden home kit archive file picker (`.zip` / `.semio.zip`). */
export function sketchpadPromptHomeKitArchiveFile(): void {
	if (typeof document === "undefined") return;
	sketchpadEnsureHomeKitFileInput().click();
}

/** @emoji 📥 Installs document-level home drag/drop (overlay + kit import on `/`). */
export function sketchpadInstallHomeDropzone(): void {
	if (typeof window === "undefined" || sketchpadHomeDropzoneInstalled) return;
	sketchpadHomeDropzoneInstalled = true;
	const fileInput = sketchpadEnsureHomeKitFileInput();
	fileInput.addEventListener("change", () => {
		const file = fileInput.files?.[0];
		fileInput.value = "";
		if (!file) return;
		const ctrl = getSketchpadShellController();
		if (!ctrl) return;
		ctrl.run("importKitFromDrop", { file });
	});
	const onDragEnter = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (!sketchpadTransferHasKitArchive(event.dataTransfer)) return;
		event.preventDefault();
		sketchpadHomeDropzoneDragDepth += 1;
		sketchpadSetHomeDropzoneOverlayVisible(true);
	};
	const onDragOver = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (!sketchpadTransferHasKitArchive(event.dataTransfer)) return;
		event.preventDefault();
	};
	const onDragLeave = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		if (sketchpadHomeDropzoneDragDepth <= 0) return;
		sketchpadHomeDropzoneDragDepth -= 1;
		if (sketchpadHomeDropzoneDragDepth === 0) sketchpadSetHomeDropzoneOverlayVisible(false);
	};
	const onDrop = (event: DragEvent) => {
		if (!sketchpadHomeRouteActive()) return;
		event.preventDefault();
		sketchpadHomeDropzoneDragDepth = 0;
		sketchpadSetHomeDropzoneOverlayVisible(false);
		const file = event.dataTransfer?.files?.[0];
		if (!file) return;
		const ctrl = getSketchpadShellController();
		if (!ctrl) return;
		ctrl.run("importKitFromDrop", { file });
	};
	window.addEventListener("dragenter", onDragEnter);
	window.addEventListener("dragover", onDragOver);
	window.addEventListener("dragleave", onDragLeave);
	window.addEventListener("drop", onDrop);
}

/** @emoji 📎 Registers a {@link SemioKitStore} on the shell controller. */
export function attachSketchpadKitStore(
	kitId: string,
	store: SemioKitStore,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean },
): void {
	const ctrl = getSketchpadShellController();
	if (!ctrl) throw new Error("semio/sketchpad: platform not initialized — call ensureSketchpadPlatform first");
	ctrl.registerKitStore(kitId, store, { kind: options?.kind });
	if (options?.navigate !== false) {
		navigateSketchpadTo(`/kits/${kitId}`);
	}
}

/** @emoji 📎 Attaches a kit backend to the shell controller and optionally navigates to it. */
export function attachSketchpadKit(
	kitId: string,
	backend: SemioKitStoreBackend,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean },
): void {
	attachSketchpadKitStore(kitId, new SemioKitStore(backend), options);
}

/** @emoji 🧭 Navigates the sketchpad {@link Platform} (updates history when in a browser). */
export function navigateSketchpadTo(uri: string): void {
	const platform = getSketchpadPlatform();
	if (!platform) throw new Error("semio/sketchpad: platform not initialized — call ensureSketchpadPlatform first");
	if (platform.onNavigate) {
		platform.onNavigate(uri);
		return;
	}
	applySketchpadUri(platform, uri);
}

/** @emoji 📦 Imports kit bytes/URL and registers them on the active platform. */
export async function openSketchpadKitFromImport(
	data: ArrayBuffer | Blob | File | string,
	options?: { readonly kind?: SketchpadKitPersistenceKind; readonly navigate?: boolean },
): Promise<string> {
	const { kit, session, portCompatSource } = await importKit(data);
	const jsStores = await session.stores();
	const jsStore = jsStores[0];
	const store = jsStore
		? await createSemioKitStoreFromJsStore(jsStore, {
				onDispose: () => void session.dispose(),
				portCompatSource,
			})
		: new InMemorySemioKitStore(kit);
	attachSketchpadKitStore(kit.id, store, { kind: options?.kind ?? "fixture", navigate: options?.navigate });
	return kit.id;
}

const SKETCHPAD_DEV_FIXTURE_KIT_URL = "/assets/semio/metabolism/wip/initialKit/kit.semio.json";

/** @emoji 🧪 Dev-only Nakagin-filtered kit URL (served from `/fixtures/` in sketchpad Vite). */
export const SKETCHPAD_DEV_FIXTURE_NAKAGIN_FILTERED_URL = "/fixtures/nakagin-capsule-tower.filtered.kit.semio.json";

/** @emoji 🧪 Loads the metabolism fixture when no kits are open (dev browser only). */
export async function seedSketchpadDevFixtureKitIfEmpty(): Promise<string | null> {
	const ctrl = getSketchpadShellController();
	if (!ctrl || ctrl.listOpenKitIds().length > 0) return null;
	try {
		return await openSketchpadKitFromImport(SKETCHPAD_DEV_FIXTURE_KIT_URL, { kind: "fixture", navigate: true });
	} catch (error) {
		console.warn("[semio.sketchpad] dev fixture kit failed to load:", error);
		return null;
	}
}
//#endregion 🔖KitHost

//#region 🔖KitStore
export const SKETCHPAD_SHELL_STORE_SHELL = "shell";
export const SKETCHPAD_KIT_STORE_PREFIX = "kit:";

/** @emoji 📸 Kit row snapshot for {@link SemioKitStore}. */
export type SketchpadKitSnapshot = { readonly kit: Kit };

/** @emoji 🎯 Selection within the active kit/design route (diagrams). */
export interface SketchpadRouteSelection {
	readonly pieceIds: readonly string[];
	readonly connectionIds: readonly string[];
	readonly kitDiagramNodeIds: readonly string[];
}

/** @emoji 📥 Home kit import progress surfaced in workbench chrome. */
export interface SketchpadImportStatus {
	readonly phase: "idle" | "importing" | "success" | "error";
	readonly label?: string;
	readonly error?: string;
}

/** @emoji 🏠 Home table UI state (expand, selection, URL-synced filters). */
export interface SketchpadHomeUiState {
	readonly expandedRowIds: readonly string[];
	readonly selectedKitIds: readonly string[];
	readonly kindFilter: string | null;
	readonly searchQuery: string;
	readonly nameFilter: string | null;
	readonly versionFilter: string | null;
	readonly sortColumnId: string | null;
	readonly sortDescending: boolean;
}

/** @emoji 🧭 Shell chrome snapshot (navigation, panels, open kits). */
export interface SketchpadShellSnapshot {
	readonly navigationPath: string;
	readonly panelVisibility: { readonly leftSidePanel: boolean; readonly rightSidePanel: boolean };
	readonly openKitIds: readonly string[];
	readonly routeSelection: SketchpadRouteSelection;
	readonly home: SketchpadHomeUiState;
	readonly importStatus: SketchpadImportStatus;
}

function sketchpadEmptyRouteSelection(): SketchpadRouteSelection {
	return { pieceIds: [], connectionIds: [], kitDiagramNodeIds: [] };
}

function sketchpadEmptyHomeUiState(): SketchpadHomeUiState {
	return {
		expandedRowIds: [],
		selectedKitIds: [],
		kindFilter: null,
		searchQuery: "",
		nameFilter: null,
		versionFilter: null,
		sortColumnId: null,
		sortDescending: false,
	};
}

function sketchpadEmptyImportStatus(): SketchpadImportStatus {
	return { phase: "idle" };
}

function sketchpadPathSupportsRouteSelectionQuery(pathOnly: string): boolean {
	return pathOnly.startsWith("/kits/");
}

/** @emoji 🔎 Parses kit/design diagram selection query params from a platform URI. */
export function parseSketchpadRouteSelectionQuery(uri: string): SketchpadRouteSelection {
	const query = uri.includes("?") ? uri.slice(uri.indexOf("?") + 1) : "";
	const params = new URLSearchParams(query);
	return {
		pieceIds: params.getAll("piece"),
		connectionIds: params.getAll("conn"),
		kitDiagramNodeIds: params.getAll("diag"),
	};
}

/** @emoji 🔗 Serializes {@link SketchpadRouteSelection} into kit-route query params. */
export function sketchpadRouteSelectionUriFilters(selection: SketchpadRouteSelection): string {
	const params = new URLSearchParams();
	for (const id of selection.pieceIds) params.append("piece", id);
	for (const id of selection.connectionIds) params.append("conn", id);
	for (const id of selection.kitDiagramNodeIds) params.append("diag", id);
	const serialized = params.toString();
	return serialized.length > 0 ? `?${serialized}` : "";
}

/** @emoji 🔎 Parses home filter query params from a platform URI. */
export function parseSketchpadHomeQuery(uri: string): SketchpadHomeUiState {
	const query = uri.includes("?") ? uri.slice(uri.indexOf("?") + 1) : "";
	const params = new URLSearchParams(query);
	return {
		expandedRowIds: params.getAll("e"),
		selectedKitIds: params.getAll("sel"),
		kindFilter: params.get("kind"),
		searchQuery: params.get("q") ?? "",
		nameFilter: params.get("name"),
		versionFilter: params.get("version"),
		sortColumnId: params.get("sort"),
		sortDescending: params.get("dir") === "desc",
	};
}

function sketchpadHomeUriFilters(home: SketchpadHomeUiState): string {
	const params = new URLSearchParams();
	if (home.kindFilter) params.set("kind", home.kindFilter);
	if (home.searchQuery) params.set("q", home.searchQuery);
	if (home.nameFilter) params.set("name", home.nameFilter);
	if (home.versionFilter) params.set("version", home.versionFilter);
	for (const id of home.expandedRowIds) params.append("e", id);
	for (const id of home.selectedKitIds) params.append("sel", id);
	if (home.sortColumnId) params.set("sort", home.sortColumnId);
	if (home.sortDescending) params.set("dir", "desc");
	const serialized = params.toString();
	return serialized.length > 0 ? `?${serialized}` : "";
}

function sketchpadTitleFromDocPath(relativePath: string): string {
	const segment = relativePath.replace(/\/index$/, "").split("/").pop() ?? relativePath;
	return segment
		.split(/[-_]/)
		.filter((part) => part.length > 0)
		.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
		.join(" ");
}

type SketchpadDocPage = { readonly path: string; readonly title: string };
type SketchpadDocSection = { readonly id: string; readonly label: string; readonly pages: readonly SketchpadDocPage[] };

/** @emoji 📄 Lazy-loaded MDX module shape from the sketchpad pages bundle. */
export type SketchpadMdxModule = {
	readonly default: unknown;
	readonly frontmatter?: Readonly<Record<string, unknown>>;
};

const SKETCHPAD_MDX_MODULE_LOADERS = import.meta.glob<SketchpadMdxModule>("./pages/**/*.mdx");
const SKETCHPAD_MDX_MODULE_PATHS = Object.keys(SKETCHPAD_MDX_MODULE_LOADERS);

/** @emoji 🔍 Resolves a docs route path to a Vite MDX module key. */
export function sketchpadResolveMdxModuleKey(docsPath: string): string | null {
	const clean = docsPath.replace(/^\/+/, "").replace(/\.mdx$/, "");
	const matches = SKETCHPAD_MDX_MODULE_PATHS.filter((key) => {
		const keyPath = key.replace(/^\.\/pages\//, "").replace(/\.mdx$/, "");
		return keyPath === clean || keyPath === `${clean}/index`;
	});
	return matches[0] ?? null;
}

/** @emoji 📥 Loads an MDX page module for a docs route (`getting-started/index`, …). */
export async function sketchpadLoadMdxModule(docsPath: string): Promise<SketchpadMdxModule | null> {
	const moduleKey = sketchpadResolveMdxModuleKey(docsPath);
	if (!moduleKey) return null;
	try {
		return await SKETCHPAD_MDX_MODULE_LOADERS[moduleKey]!();
	} catch {
		return null;
	}
}

/** @emoji 🏷️ Reads a display title from MDX frontmatter or route path. */
export function sketchpadMdxTitle(module: SketchpadMdxModule | null, docsPath: string): string {
	const frontmatter = module?.frontmatter;
	if (frontmatter && typeof frontmatter["title"] === "string" && frontmatter["title"].length > 0) {
		return frontmatter["title"];
	}
	return sketchpadTitleFromDocPath(docsPath);
}

/** @emoji 📚 Builds the sketchpad docs tree from bundled MDX pages (Vite glob). */
export function sketchpadBuildDocsRegistry(): readonly SketchpadDocSection[] {
	const sectionMap = new Map<string, SketchpadDocPage[]>();
	for (const modulePath of SKETCHPAD_MDX_MODULE_PATHS) {
		const relative = modulePath.replace(/^\.\/pages\//, "").replace(/\.mdx$/, "");
		const sectionId = relative.split("/")[0] ?? "root";
		const pages = sectionMap.get(sectionId) ?? [];
		pages.push({ path: relative, title: sketchpadTitleFromDocPath(relative) });
		sectionMap.set(sectionId, pages);
	}
	if (sectionMap.size === 0) {
		return [
			{
				id: "getting-started",
				label: "Getting started",
				pages: [
					{ path: "getting-started/index", title: "Getting started" },
					{ path: "getting-started/installation", title: "Installation" },
				],
			},
		];
	}
	return [...sectionMap.entries()]
		.map(([id, pages]) => ({
			id,
			label: sketchpadTitleFromDocPath(id),
			pages: pages.sort((left, right) => left.path.localeCompare(right.path)),
		}))
		.sort((left, right) => left.label.localeCompare(right.label));
}

/** @emoji 🏠 Builds the home table model (docs tree + grouped kits). */
export function sketchpadBuildHomeTableModel(input: {
	readonly openKitIds: readonly string[];
	readonly kitById: (kitId: string) => Kit | undefined;
	readonly kitKind: (kitId: string) => string;
	readonly home: SketchpadHomeUiState;
	readonly docs?: readonly SketchpadDocSection[];
}): TableModel {
	const docs = input.docs ?? sketchpadBuildDocsRegistry();
	const expanded = new Set(input.home.expandedRowIds);
	const rows: TableModel["rows"][number][] = [];
	const expandToggle = (rowId: string) => ({
		command: "toggleHomeRowExpand",
		args: { rowId },
	});
	const docsRootId = "docs-root";
	rows.push({
		id: docsRootId,
		depth: 0,
		hasChildren: true,
		expanded: expanded.has(docsRootId),
		expandToggle: expandToggle(docsRootId),
		cells: { name: "Documentation", version: "", kind: "docs", updated: "" },
	});
	if (expanded.has(docsRootId)) {
		for (const section of docs) {
			const sectionId = `docs-section-${section.id}`;
			rows.push({
				id: sectionId,
				depth: 1,
				hasChildren: section.pages.length > 0,
				expanded: expanded.has(sectionId),
				expandToggle: expandToggle(sectionId),
				cells: { name: section.label, version: "", kind: "docs", updated: "" },
			});
			if (expanded.has(sectionId)) {
				for (const page of section.pages) {
					rows.push({
						id: `docs-page-${page.path}`,
						depth: 2,
						cells: { name: page.title, version: "", kind: "docs", updated: "" },
						navigateUri: `/docs/${page.path}`,
					});
				}
			}
		}
	}
	const kitGroups = new Map<string, { readonly kitId: string; readonly kit: Kit; readonly kind: string }[]>();
	for (const kitId of input.openKitIds) {
		const kit = input.kitById(kitId);
		if (!kit) continue;
		const kind = input.kitKind(kitId) || "temporary";
		if (input.home.kindFilter && input.home.kindFilter !== kind) continue;
		const name = kit.name ?? kitId;
		if (input.home.searchQuery && !name.toLowerCase().includes(input.home.searchQuery.toLowerCase())) continue;
		if (input.home.nameFilter && name !== input.home.nameFilter) continue;
		const version = kit.version ?? "";
		if (input.home.versionFilter && version !== input.home.versionFilter) continue;
		const group = kitGroups.get(name) ?? [];
		group.push({ kitId, kit, kind });
		kitGroups.set(name, group);
	}
	const sortedKitGroups = [...kitGroups.entries()].sort((left, right) => {
		const column = input.home.sortColumnId;
		if (!column) return left[0].localeCompare(right[0]);
		const leftKit = (left[1].find((entry) => !entry.kit.version) ?? left[1][0]!).kit;
		const rightKit = (right[1].find((entry) => !entry.kit.version) ?? right[1][0]!).kit;
		const leftKind = left[1][0]?.kind ?? "";
		const rightKind = right[1][0]?.kind ?? "";
		let comparison = 0;
		switch (column) {
			case "name":
				comparison = left[0].localeCompare(right[0]);
				break;
			case "version":
				comparison = (leftKit.version ?? "").localeCompare(rightKit.version ?? "");
				break;
			case "kind":
				comparison = leftKind.localeCompare(rightKind);
				break;
			case "updated":
				comparison = sketchpadFormatKitTimestamp(leftKit.updatedAt ?? leftKit.createdAt).localeCompare(
					sketchpadFormatKitTimestamp(rightKit.updatedAt ?? rightKit.createdAt),
				);
				break;
			default:
				comparison = left[0].localeCompare(right[0]);
		}
		return input.home.sortDescending ? -comparison : comparison;
	});
	for (const [name, group] of sortedKitGroups) {
		const parentId = `kit-group-${name}`;
		const defaultKit = group.find((entry) => !entry.kit.version) ?? group[0]!;
		const hasChildren = group.length > 1;
		rows.push({
			id: parentId,
			depth: 0,
			hasChildren,
			expanded: expanded.has(parentId),
			expandToggle: hasChildren ? expandToggle(parentId) : undefined,
			cells: {
				name,
				version: defaultKit.kit.version ?? "",
				kind: defaultKit.kind,
				updated: sketchpadFormatKitTimestamp(defaultKit.kit.updatedAt ?? defaultKit.kit.createdAt),
			},
			navigateUri: hasChildren ? undefined : `/kits/${defaultKit.kitId}`,
		});
		if (expanded.has(parentId) && hasChildren) {
			for (const entry of group) {
				if (entry.kitId === defaultKit.kitId) continue;
				const versionLabel = entry.kit.version ?? "(default)";
				rows.push({
					id: entry.kitId,
					depth: 1,
					cells: {
						name: versionLabel,
						version: entry.kit.version ?? "",
						kind: entry.kind,
						updated: sketchpadFormatKitTimestamp(entry.kit.updatedAt ?? entry.kit.createdAt),
					},
					navigateUri: `/kits/${entry.kitId}`,
				});
			}
		}
	}
	return {
		columns: [
			{ id: "name", label: "Name", sortable: true },
			{ id: "version", label: "Version", sortable: true },
			{ id: "kind", label: "Kind", sortable: true },
			{ id: "updated", label: "Updated", sortable: true },
		],
		rows,
		selectedRowIds: input.home.selectedKitIds,
		sortColumnId: input.home.sortColumnId,
		sortDescending: input.home.sortDescending,
			emptyMessage:
				rows.length === 0
					? "No kits open — drop a .zip here, use Workbench → Import kit archive, or the command palette"
					: undefined,
	};
}

/** @emoji 🔌 Backend contract for {@link SemioKitStore} (memory, WASM worker, HTTP, …). */
export type SemioKitStoreBackend = {
	getSnapshot(): SketchpadKitSnapshot;
	subscribe?(listener: () => void): () => void;
	replace?(next: Kit): void;
};

/** @emoji 🗄️ Kit authority store; adapts any {@link SemioKitStoreBackend} to {@link Store}. */
export class SemioKitStore extends Store<SketchpadKitSnapshot> {
	private detach?: () => void;

	constructor(private readonly backend: SemioKitStoreBackend) {
		super();
		if (backend.subscribe) {
			this.detach = backend.subscribe(() => this.notify());
		}
	}

	override getSnapshot(): SketchpadKitSnapshot {
		return this.backend.getSnapshot();
	}

	replaceKit(next: Kit): void {
		this.backend.replace?.(next);
		this.notify();
	}

	override dispose(): void {
		this.detach?.();
		super.dispose();
	}
}

/** @emoji 💾 In-memory kit store for hosts without a live {@link @semio/js} session yet. */
export class InMemorySemioKitStore extends SemioKitStore {
	constructor(kit: Kit) {
		let current = kit;
		super({
			getSnapshot: () => ({ kit: current }),
			replace: (next) => {
				current = next;
			},
		});
	}
}

/** @emoji 🌐 {@link SemioKitStore} backed by {@link @semio/js} with live kit mutations. */
export class SemioJsKitStore extends SemioKitStore {
	constructor(
		backend: SemioKitStoreBackend,
		readonly jsStore: JsKitStore,
		private readonly onSessionDispose: (() => void | Promise<void>) | undefined,
		private readonly portCompatById: ReadonlyMap<string, readonly { readonly id: string }[]>,
	) {
		super(backend);
	}

	/** @emoji 🏛 WIP {@link JsKitEntity} handle for GraphQL kit commands. */
	async jsKitEntity(): Promise<JsKitEntity> {
		return this.jsStore.wip().theKit().kit();
	}

	/** @emoji 🔄 Re-reads kit DTO from rs and notifies subscribers. */
	async refreshFromJs(): Promise<void> {
		const kit = await sketchpadKitDtoFromJsStore(this.jsStore);
		const fromGraphql = sketchpadExtractPortCompatById(kit);
		const compat = sketchpadMergePortCompatMaps(this.portCompatById, fromGraphql);
		this.replaceKit(sketchpadApplyPortCompatById(kit, compat));
	}

	override dispose(): void {
		super.dispose();
		void this.onSessionDispose?.();
	}
}

const SKETCHPAD_KIT_READ_INNER = `id name description version createdAt updatedAt
designs {
  edges {
    node {
      id name description unit
      pieces {
        edges {
          node {
            id name
            blueprint { id }
            position { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } }
          }
        }
      }
      connections {
        edges {
          node {
            id
            parent { piece { id } connector { id } }
            child { piece { id } connector { id } }
          }
        }
      }
    }
  }
}
types {
  edges {
    node {
      id name description
      connectors { edges { node { id name port { id label code copatibleWith { edges { node { id } } } } } } }
      ports { edges { node { id label code copatibleWith { edges { node { id } } } } } }
      representations { edges { node { id name file { id } } } }
    }
  }
}
qualities { edges { node { id key value } } }
folders { edges { node { id path description } } }
authors { edges { node { id name } } }
files { edges { node { id url description } } }`;

function sketchpadFormatKitTimestamp(value: unknown): string {
	if (value == null || value === "") return "";
	const date = typeof value === "string" || typeof value === "number" ? new Date(value) : value instanceof Date ? value : null;
	if (!date || Number.isNaN(date.getTime())) return "";
	return date.toLocaleString();
}

/** @emoji 🔌 Maps GraphQL {@code copatibleWith} relay edges onto {@code compatiblePorts} DTO refs. */
export function sketchpadPortDtoFromGraphqlNode(node: Record<string, unknown>): Record<string, unknown> {
	const compatEdges =
		(node["copatibleWith"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
	const compatiblePorts = compatEdges
		.map((edge) => edge.node)
		.filter((port): port is Record<string, unknown> => port != null)
		.map((port) => ({ id: port["id"] }));
	if (compatiblePorts.length === 0) return node;
	return { ...node, compatiblePorts };
}

/** @emoji 📸 Materializes a kit DTO from rs GraphQL for platform snapshots. */
export async function sketchpadKitDtoFromJsStore(jsStore: JsKitStore): Promise<Kit> {
	const data = await jsStore.readKitInner(SKETCHPAD_KIT_READ_INNER);
	if (!data) return { id: "", name: "" } as Kit;
	const nodes = (key: string): readonly Record<string, unknown>[] => {
		const edges = (data[key] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges.map((edge) => edge.node).filter((node): node is Record<string, unknown> => node != null);
	};
	const parseDesigns = (): Design[] => {
		const edges = (data["designs"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges
			.map((edge) => edge.node)
			.filter((node): node is Record<string, unknown> => node != null)
			.map((node) => {
				const pieceEdges = (node["pieces"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const pieces = pieceEdges.map((pe) => pe.node).filter((n): n is Record<string, unknown> => n != null) as Design["pieces"];
				const connectionEdges = (node["connections"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const connections = connectionEdges.map((ce) => ce.node).filter((n): n is Record<string, unknown> => n != null);
				return { ...node, pieces, connections } as Design;
			});
	};
	const parseTypes = (): Type[] => {
		const edges = (data["types"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
		return edges
			.map((edge) => edge.node)
			.filter((node): node is Record<string, unknown> => node != null)
			.map((node) => {
				const repEdges = (node["representations"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const representations = repEdges.map((re) => re.node).filter((n): n is Record<string, unknown> => n != null);
				const portEdges = (node["ports"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const ports = portEdges
					.map((pe) => pe.node)
					.filter((n): n is Record<string, unknown> => n != null)
					.map((port) => sketchpadPortDtoFromGraphqlNode(port));
				const conEdges = (node["connectors"] as { edges?: readonly { node?: Record<string, unknown> }[] } | undefined)?.edges ?? [];
				const connectors = conEdges
					.map((ce) => ce.node)
					.filter((n): n is Record<string, unknown> => n != null)
					.map((connector) => {
						const port = connector["port"];
						if (port == null || typeof port !== "object") return connector;
						return { ...connector, port: sketchpadPortDtoFromGraphqlNode(port as Record<string, unknown>) };
					});
				return { ...node, representations, ports, connectors } as Type;
			});
	};
	return {
		id: String(data["id"] ?? ""),
		name: String(data["name"] ?? ""),
		description: data["description"] != null ? String(data["description"]) : undefined,
		version: data["version"] != null ? String(data["version"]) : undefined,
		createdAt: data["createdAt"] != null ? String(data["createdAt"]) : undefined,
		updatedAt: data["updatedAt"] != null ? String(data["updatedAt"]) : undefined,
		files: nodes("files") as Kit["files"],
		folders: nodes("folders") as Kit["folders"],
		authors: nodes("authors") as Kit["authors"],
		qualities: nodes("qualities") as Kit["qualities"],
		designs: parseDesigns(),
		types: parseTypes(),
	} as Kit;
}

/** @emoji 🌐 Builds a {@link SemioJsKitStore} from a live {@link @semio/js} store. */
export async function createSemioKitStoreFromJsStore(
	jsStore: JsKitStore,
	options?: { readonly onDispose?: () => void | Promise<void>; readonly portCompatSource?: Kit },
): Promise<SemioJsKitStore> {
	const portCompatById = sketchpadExtractPortCompatById(
		options?.portCompatSource ?? ({ id: "", name: "" } as Kit),
	);
	const materializeKit = async (): Promise<Kit> => {
		const dto = await sketchpadKitDtoFromJsStore(jsStore);
		const compat = sketchpadMergePortCompatMaps(portCompatById, sketchpadExtractPortCompatById(dto));
		return sketchpadApplyPortCompatById(dto, compat);
	};
	let kit = await materializeKit();
	const refresh = async (): Promise<void> => {
		kit = await materializeKit();
	};
	await refresh();
	return new SemioJsKitStore(
		{
			getSnapshot: () => ({ kit }),
			replace: (next) => {
				kit = next;
			},
			subscribe: (listener) =>
				jsStore.session.subscribe(() => {
					void refresh().then(listener);
				}),
		},
		jsStore,
		options?.onDispose,
		portCompatById,
	);
}

/** @emoji ⚡ Runs a {@link JsKitEntity} mutation on the active js-backed kit store. */
export async function executeSketchpadJsKitMutation(
	kitId: string,
	run: (kit: JsKitEntity) => Promise<SetResult>,
	storeOverride?: SemioKitStore,
): Promise<SetResult> {
	const store = storeOverride ?? getSketchpadShellController()?.getKitStore(kitId);
	if (!(store instanceof SemioJsKitStore)) {
		return { ok: false, error: { kind: "NotSupported", message: "semio/sketchpad: kit is not backed by @semio/js" } };
	}
	const result = await run(await store.jsKitEntity());
	await store.refreshFromJs();
	return result;
}

function sketchpadActiveKitIdFromPath(path: string): string | null {
	return path.split("?")[0]?.match(/^\/kits\/([^/]+)/)?.[1] ?? null;
}

export function sketchpadKitStoreId(kitId: string): string {
	return `${SKETCHPAD_KIT_STORE_PREFIX}${kitId}`;
}

let sketchpadShellControllerSingleton: SketchpadShellController | null = null;

/** @emoji 🎛 Active sketchpad shell controller after {@link buildSketchpadPlatform}. */
export function getSketchpadShellController(): SketchpadShellController | null {
	return sketchpadShellControllerSingleton;
}
//#endregion 🔖KitStore

//#region 🔖SketchpadRouteScope
/** @emoji 🧭 Kit/design/type ids parsed from a sketchpad URL path (render-agnostic). */
export function parseSketchpadRouteScopeFromPath(path: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly docsPath: string;
} {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") {
		const docsPath = pathParts.slice(1).join("/") || "index";
		return { kitId: null, designId: null, typeId: null, docsPath };
	}
	if (pathParts[0] !== "kits") {
		return { kitId: null, designId: null, typeId: null, docsPath: "index" };
	}
	const kitId = pathParts[1] && isUuidPattern(pathParts[1]) ? pathParts[1] : null;
	const designId = pathParts[2] === "designs" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	const typeId = pathParts[2] === "types" && pathParts[3] && isUuidPattern(pathParts[3]) ? pathParts[3] : null;
	return { kitId, designId, typeId, docsPath: "index" };
}

/** @emoji 🧭 Maps a location path to the sketchpad {@link Platform} active app id. */
export function sketchpadAppIdFromPath(path: string): string {
	const pathParts = path.split("/").filter((part) => part.length > 0);
	const isUuidPattern = (value: string) => /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
	if (pathParts[0] === "docs") return SKETCHPAD_DOCS_APP_ID;
	if (pathParts[0] === "feedback") return SKETCHPAD_FEEDBACK_APP_ID;
	if (pathParts[0] !== "kits") return SKETCHPAD_HOME_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "designs" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_DESIGN_APP_ID;
	if (pathParts.length >= 4 && pathParts[2] === "types" && isUuidPattern(pathParts[3] ?? "")) return SKETCHPAD_TYPE_APP_ID;
	if (pathParts.length >= 2 && isUuidPattern(pathParts[1] ?? "")) return SKETCHPAD_KIT_APP_ID;
	return SKETCHPAD_HOME_APP_ID;
}
//#endregion 🔖SketchpadRouteScope

//#region 🔖KitHelpers
/** @emoji 🔍 Finds a type row on a kit snapshot. */
export function findTypeInKit(kit: Kit, typeId: string | null | undefined): Type | undefined {
	if (!typeId) return undefined;
	return kit.types?.find((t) => t.id === typeId);
}

/** @emoji 🔍 Finds a design row on a kit snapshot. */
export function findDesignInKit(kit: Kit, designId: string | null | undefined): Design | undefined {
	if (!designId) return undefined;
	return kit.designs?.find((d) => d.id === designId);
}

/** @emoji 🔍 Finds a piece row on a design snapshot. */
export function findPieceInDesign(design: Design, pieceId: string | null | undefined) {
	if (!pieceId) return undefined;
	return design.pieces?.find((p) => p.id === pieceId);
}

function sketchpadReadEntityId(ref: unknown): string | null {
	if (ref == null) return null;
	if (typeof ref === "string") return ref;
	if (typeof ref === "object" && "id" in ref) return String((ref as { id: unknown }).id);
	return null;
}

const SKETCHPAD_METABOLISM_KIT_ASSET_ROOT = "/assets/semio/metabolism/wip/initialKit";

/** @emoji 🗂️ Resolves kit file ids to fetchable mesh URLs (http, absolute, or metabolism assets). */
export function sketchpadKitFileUrlById(kit: Kit): ReadonlyMap<string, string> {
	const map = new Map<string, string>();
	for (const file of kit.files ?? []) {
		const row = file as { id: string; url?: string; uri?: string; path?: string; name?: string };
		const direct = row.url ?? row.uri;
		if (direct) {
			map.set(row.id, direct);
			continue;
		}
		if (row.path) {
			const path = row.path.replace(/^\.\//, "");
			map.set(row.id, path.startsWith("/") ? path : `${SKETCHPAD_METABOLISM_KIT_ASSET_ROOT}/${path}`);
			continue;
		}
		if (row.name && row.name.endsWith(".glb")) {
			map.set(row.id, `${SKETCHPAD_METABOLISM_KIT_ASSET_ROOT}/files/${row.name}`);
		}
	}
	return map;
}

const SKETCHPAD_PLACEHOLDER_MESH_URL = "puzzle.3d.placeholder://box";

/** @emoji 🧊 Picks a representation mesh URL for a design piece (placeholder when unresolved). */
export function sketchpadResolvePieceMeshUrl(
	piece: { readonly type?: unknown; readonly blueprint?: unknown },
	kit: Kit,
	fileUrls: ReadonlyMap<string, string> = sketchpadKitFileUrlById(kit),
): string {
	const typeId = sketchpadReadEntityId(piece.type ?? piece.blueprint);
	if (!typeId) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const type = findTypeInKit(kit, typeId);
	const reps = (type?.representations ?? []) as readonly { readonly file?: unknown; readonly tags?: unknown }[];
	if (reps.length === 0) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const untagged =
		reps.find((rep) => {
			const tags = rep.tags as { items?: readonly unknown[] } | readonly unknown[] | undefined;
			if (Array.isArray(tags)) return tags.length === 0;
			return !tags?.items?.length;
		}) ?? reps[0];
	const fileId = sketchpadReadEntityId(untagged?.file);
	if (!fileId) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	return fileUrls.get(fileId) ?? SKETCHPAD_PLACEHOLDER_MESH_URL;
}

/** @emoji 🧊 Picks the primary representation mesh URL for a kit kind. */
export function sketchpadResolveTypeMeshUrl(
	type: Type,
	kit: Kit,
	fileUrls: ReadonlyMap<string, string> = sketchpadKitFileUrlById(kit),
): string {
	const reps = (type.representations ?? []) as readonly { readonly file?: unknown; readonly tags?: unknown }[];
	if (reps.length === 0) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	const untagged =
		reps.find((rep) => {
			const tags = rep.tags as { items?: readonly unknown[] } | readonly unknown[] | undefined;
			if (Array.isArray(tags)) return tags.length === 0;
			return !tags?.items?.length;
		}) ?? reps[0];
	const fileId = sketchpadReadEntityId(untagged?.file);
	if (!fileId) return SKETCHPAD_PLACEHOLDER_MESH_URL;
	return fileUrls.get(fileId) ?? SKETCHPAD_PLACEHOLDER_MESH_URL;
}

function sketchpadNewKitId(): string {
	if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
		return crypto.randomUUID();
	}
	return `kit-${Date.now()}`;
}

function sketchpadPanelTextStack(lines: readonly { readonly text: string; readonly emphasize?: boolean }[]): PanelModel {
	return {
		body: {
			type: "stack",
			direction: "vertical",
			padding: "standard",
			children: lines.map((line) => ({ type: "text", value: line.text, emphasize: line.emphasize })),
		},
	};
}

function sketchpadPanelCommandButton(
	label: string,
	command: string,
	args?: unknown,
): { readonly type: "button"; readonly label: string; readonly command: { readonly controllerId: string; readonly command: string; readonly args?: unknown } } {
	return {
		type: "button",
		label,
		command: { controllerId: "semio.sketchpad.shell", command, ...(args !== undefined ? { args } : {}) },
	};
}
//#endregion 🔖KitHelpers

//#region 🔖Topology
const SKETCHPAD_FLAT_HANDLE_SEPARATOR = "::";

/** @emoji 🔗 Re-exports {@link PLATFORM_TOPOLOGY_STORE_PREFIX} for sketchpad topology stores. */
export const SKETCHPAD_TOPOLOGY_STORE_PREFIX = PLATFORM_TOPOLOGY_STORE_PREFIX;

type SketchpadPuzzle2dFixtureV1 = {
	readonly schema: "puzzle.2d.fixture/v1";
	readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
	readonly nodes: readonly Record<string, unknown>[];
	readonly edges: readonly Record<string, unknown>[];
};

type SketchpadVolumeFixtureV1 = {
	readonly schema: "puzzle.3d.fixture/v1";
	readonly domain: string;
	readonly camera: {
		readonly position: readonly [number, number, number];
		readonly target: readonly [number, number, number];
		readonly zoom: number;
	};
	readonly objects: readonly Record<string, unknown>[];
	readonly attractions: readonly Record<string, unknown>[];
};

function sketchpadFlatPartCenterFromTopLeft(
	position: { readonly x: number; readonly y: number },
	frame: { readonly width: number; readonly height: number },
): { x: number; y: number } {
	return { x: position.x + frame.width / 2, y: position.y + frame.height / 2 };
}

function sketchpadFlatCameraFromPartCenters(centers: readonly { x: number; y: number }[]): SketchpadPuzzle2dFixtureV1["camera"] {
	if (centers.length === 0) return { x: 0, y: 0, zoom: 1 };
	const avgX = centers.reduce((sum, point) => sum + point.x, 0) / centers.length;
	const avgY = centers.reduce((sum, point) => sum + point.y, 0) / centers.length;
	return { x: -avgX, y: -avgY, zoom: 1 };
}

function sketchpadFlatHandleCompoundId(left: string, right: string): string {
	return `${left}${SKETCHPAD_FLAT_HANDLE_SEPARATOR}${right}`;
}

function sketchpadTopologyAnchorFullId(partId: string, anchorId: string): string {
	return `${partId}:${anchorId}`;
}

/** @emoji 🧩 Stable FiveD instance id for kit diagram surfaces. */
export function sketchpadKitDiagramInstanceId(kitId: string): string {
	return `${kitId}:kit:diagram`;
}

/** @emoji 🧩 Stable FiveD instance id for a design scene (volume). */
export function sketchpadDesignSceneInstanceId(kitId: string, designId: string): string {
	return `${kitId}:${designId}:scene`;
}

/** @emoji 🧩 Stable FiveD instance id for a design diagram (flat). */
export function sketchpadDesignDiagramInstanceId(kitId: string, designId: string): string {
	return `${kitId}:${designId}:diagram`;
}

/** @emoji 🧩 Stable FiveD instance id for a type CAD scene (volume). */
export function sketchpadTypeSceneInstanceId(kitId: string, typeId: string): string {
	return `${kitId}:type:${typeId}:scene`;
}

/** @emoji 🔍 Parses sketchpad FiveD {@link Puzzle5dModel.instanceId} segments. */
export function parseSketchpadPuzzleInstanceId(instanceId: string): {
	readonly kitId: string | null;
	readonly designId: string | null;
	readonly typeId: string | null;
	readonly pane: "kit-diagram" | "scene" | "diagram" | "type-scene" | null;
} {
	const parts = instanceId.split(":");
	if (parts.length === 3 && parts[1] === "kit" && parts[2] === "diagram") {
		return { kitId: parts[0] ?? null, designId: null, typeId: null, pane: "kit-diagram" };
	}
	if (parts.length === 4 && parts[1] === "type" && parts[3] === "scene") {
		return { kitId: parts[0] ?? null, designId: null, typeId: parts[2] ?? null, pane: "type-scene" };
	}
	if (parts.length === 3 && parts[2] === "scene") {
		return { kitId: parts[0] ?? null, designId: parts[1] ?? null, typeId: null, pane: "scene" };
	}
	if (parts.length === 3 && parts[2] === "diagram") {
		return { kitId: parts[0] ?? null, designId: parts[1] ?? null, typeId: null, pane: "diagram" };
	}
	return { kitId: null, designId: null, typeId: null, pane: null };
}

/** @emoji 🔑 Delegates to {@link platformTopologyStoreId}. */
export function sketchpadTopologyStoreId(instanceId: string): string {
	return platformTopologyStoreId(instanceId);
}

function sketchpadEmptyVolumeFixture(): SketchpadVolumeFixtureV1 {
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera: { position: [12, 12, 12], target: [0, 0, 0], zoom: 1 },
		objects: [],
		attractions: [],
	};
}

type SketchpadKitDiagramNodeKind = "type" | "design" | "quality" | "port" | "file" | "folder" | "author";

function sketchpadKitDiagramNodeFrame(kind: SketchpadKitDiagramNodeKind): {
	readonly width: number;
	readonly height: number;
	readonly shape: "circle" | "rectangle";
} {
	switch (kind) {
		case "design":
			return { width: 48, height: 48, shape: "circle" };
		case "type":
			return { width: 120, height: 48, shape: "rectangle" };
		case "file":
			return { width: 100, height: 48, shape: "rectangle" };
		default:
			return { width: 140, height: 36, shape: "rectangle" };
	}
}

function sketchpadKitDiagramPortLabel(port: Record<string, unknown>): string {
	const label = port["label"];
	if (typeof label === "string" && label.length > 0) return label;
	const code = port["code"];
	if (typeof code === "string" && code.length > 0) return code;
	const name = port["name"];
	if (typeof name === "string" && name.length > 0) return name;
	return String(port["id"] ?? "");
}

/** @emoji 👨‍👩‍👦 Reads kit-level {@code families} rows from a denormalized bundle or projection DTO. */
export function sketchpadReadKitFamilyRows(kit: Kit): readonly Record<string, unknown>[] {
	const raw = (kit as { families?: unknown }).families;
	if (raw == null) return [];
	const asRow = (entry: unknown): entry is Record<string, unknown> =>
		entry != null && typeof entry === "object" && !Array.isArray(entry);
	if (Array.isArray(raw)) return raw.filter(asRow);
	if (typeof raw === "object") {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) return items.filter(asRow);
	}
	return [];
}

function sketchpadReadFamilyPortRows(family: Record<string, unknown>): readonly Record<string, unknown>[] {
	const raw = family["ports"];
	if (raw == null) return [];
	const asRow = (entry: unknown): entry is Record<string, unknown> =>
		entry != null && typeof entry === "object" && !Array.isArray(entry);
	if (Array.isArray(raw)) return raw.filter(asRow);
	if (typeof raw === "object") {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) return items.filter(asRow);
	}
	return [];
}

function sketchpadForEachKitPortRecord(kit: Kit, visit: (port: Record<string, unknown>) => void): void {
	for (const type of kit.types ?? []) {
		for (const port of (type as { ports?: readonly unknown[] }).ports ?? []) {
			if (port != null && typeof port === "object" && !Array.isArray(port)) visit(port as Record<string, unknown>);
		}
		for (const connector of type.connectors ?? []) {
			const port = (connector as { port?: unknown }).port;
			if (port != null && typeof port === "object" && !Array.isArray(port)) visit(port as Record<string, unknown>);
		}
	}
	for (const family of sketchpadReadKitFamilyRows(kit)) {
		for (const port of sketchpadReadFamilyPortRows(family)) visit(port);
	}
}

/** @emoji 🔌 Collects unique ports on kit kinds, connectors, and kit-level families (metabolism). */
export function sketchpadCollectKitPorts(kit: Kit): readonly { readonly id: string; readonly name: string }[] {
	const byId = new Map<string, { id: string; name: string }>();
	const remember = (port: Record<string, unknown>) => {
		const id = sketchpadReadEntityId(port);
		if (!id || byId.has(id)) return;
		byId.set(id, { id, name: sketchpadKitDiagramPortLabel(port) });
	};
	sketchpadForEachKitPortRecord(kit, remember);
	return [...byId.values()];
}

function sketchpadCollectKitPortRecords(kit: Kit): readonly Record<string, unknown>[] {
	const byId = new Map<string, Record<string, unknown>>();
	const remember = (port: Record<string, unknown>) => {
		const id = sketchpadReadEntityId(port);
		if (!id) return;
		const prev = byId.get(id);
		if (!prev) {
			byId.set(id, { ...port });
			return;
		}
		const mergedCompat = new Set<string>();
		for (const ref of sketchpadReadCompatiblePortIds(prev)) mergedCompat.add(ref);
		for (const ref of sketchpadReadCompatiblePortIds(port)) mergedCompat.add(ref);
		byId.set(id, {
			...prev,
			...port,
			compatiblePorts: [...mergedCompat].map((compatId) => ({ id: compatId })),
		});
	};
	sketchpadForEachKitPortRecord(kit, remember);
	return [...byId.values()];
}

/** @emoji 🔀 Merges port compat maps; later map entries override earlier ones for the same port id. */
export function sketchpadMergePortCompatMaps(
	primary: ReadonlyMap<string, readonly { readonly id: string }[]>,
	overlay: ReadonlyMap<string, readonly { readonly id: string }[]>,
): Map<string, readonly { readonly id: string }[]> {
	const merged = new Map(primary);
	for (const [portId, refs] of overlay) merged.set(portId, refs);
	return merged;
}

/** @emoji 🗺️ Collects port {@code compatiblePorts} refs from a kit snapshot (bundle or DTO). */
export function sketchpadExtractPortCompatById(kit: Kit): Map<string, readonly { readonly id: string }[]> {
	const map = new Map<string, readonly { readonly id: string }[]>();
	for (const port of sketchpadCollectKitPortRecords(kit)) {
		const id = sketchpadReadEntityId(port);
		const compatIds = sketchpadReadCompatiblePortIds(port);
		if (id && compatIds.length > 0) map.set(id, compatIds.map((compatId) => ({ id: compatId })));
	}
	return map;
}

/** @emoji 🔗 Re-applies stored {@code compatiblePorts} onto a GraphQL-shaped kit DTO. */
export function sketchpadApplyPortCompatById(
	kit: Kit,
	compatById: ReadonlyMap<string, readonly { readonly id: string }[]>,
): Kit {
	if (compatById.size === 0) return kit;
	const enrichPort = (port: unknown): unknown => {
		if (port == null || typeof port !== "object") return port;
		const row = { ...(port as Record<string, unknown>) };
		const id = sketchpadReadEntityId(row);
		const compat = id ? compatById.get(id) : undefined;
		if (compat?.length) row.compatiblePorts = compat;
		return row;
	};
	const types = (kit.types ?? []).map((type) => ({
		...type,
		ports: ((type as { ports?: readonly unknown[] }).ports ?? []).map(enrichPort),
		connectors: (type.connectors ?? []).map((connector) => ({
			...connector,
			port: enrichPort((connector as { port?: unknown }).port),
		})),
	}));
	const familyRows = sketchpadReadKitFamilyRows(kit);
	const families =
		familyRows.length === 0
			? undefined
			: familyRows.map((family) => ({
					...family,
					ports: sketchpadReadFamilyPortRows(family).map(enrichPort),
				}));
	return { ...kit, types, ...(families != null ? { families } : {}) } as Kit;
}

function sketchpadReadCompatiblePortIds(port: Record<string, unknown>): readonly string[] {
	const raw = port["compatiblePorts"];
	if (raw == null) return [];
	const ids: string[] = [];
	const visit = (entry: unknown) => {
		const id = sketchpadReadEntityId(entry);
		if (id) ids.push(id);
	};
	if (Array.isArray(raw)) {
		for (const entry of raw) visit(entry);
		return ids;
	}
	if (typeof raw === "object" && raw !== null) {
		const items = (raw as { items?: readonly unknown[] }).items;
		if (Array.isArray(items)) {
			for (const entry of items) visit(entry);
			return ids;
		}
	}
	return ids;
}

/** @emoji 🔗 Union-find map grouping kit ports by {@code compatiblePorts} and shared {@code code}. */
export function sketchpadCreatePortGroupMap(
	ports: readonly { readonly id: string; readonly code?: string | null; readonly compatiblePorts?: readonly unknown[] }[],
): Map<string, string> {
	const parent = new Map<string, string>();
	const register = (id: string) => {
		if (!parent.has(id)) parent.set(id, id);
	};
	for (const port of ports) {
		const id = sketchpadReadEntityId(port);
		if (id) register(id);
	}
	const find = (id: string): string => {
		const direct = parent.get(id);
		if (!direct) return id;
		if (direct === id) return direct;
		const root = find(direct);
		parent.set(id, root);
		return root;
	};
	const union = (left: string, right: string) => {
		const leftRoot = find(left);
		const rightRoot = find(right);
		if (leftRoot === rightRoot) return;
		parent.set(rightRoot, leftRoot);
	};
	for (const port of ports) {
		const id = sketchpadReadEntityId(port);
		if (!id) continue;
		for (const relatedId of sketchpadReadCompatiblePortIds(port as Record<string, unknown>)) {
			register(relatedId);
			union(id, relatedId);
		}
		const code = typeof port.code === "string" ? port.code.trim() : "";
		if (code.length > 0) {
			for (const other of ports) {
				const otherId = sketchpadReadEntityId(other);
				const otherCode = typeof other.code === "string" ? other.code.trim() : "";
				if (otherId && otherId !== id && otherCode === code) union(id, otherId);
			}
		}
	}
	const groups = new Map<string, string>();
	for (const id of parent.keys()) groups.set(id, find(id));
	return groups;
}

/** @emoji ↔️ Adds dashed type adjacency edges for types that share compatible port groups. */
export function sketchpadKitDiagramPushTypeCompatEdges(
	kit: Kit,
	edges: SketchpadPuzzle2dFixtureV1["edges"],
	edgeIds: Set<string>,
): void {
	const ports = sketchpadCollectKitPortRecords(kit);
	if (ports.length === 0) return;
	const groups = sketchpadCreatePortGroupMap(
		ports.map((port) => ({
			id: String(port["id"] ?? ""),
			code: typeof port["code"] === "string" ? port["code"] : null,
			compatiblePorts: sketchpadReadCompatiblePortIds(port).map((compatId) => ({ id: compatId })),
		})),
	);
	const portToTypes = new Map<string, Set<string>>();
	for (const type of kit.types ?? []) {
		for (const connector of type.connectors ?? []) {
			const portId = sketchpadReadEntityId((connector as { port?: unknown }).port);
			if (!portId) continue;
			const typeIds = portToTypes.get(portId) ?? new Set<string>();
			typeIds.add(type.id);
			portToTypes.set(portId, typeIds);
		}
	}
	const rootToTypes = new Map<string, Set<string>>();
	for (const [portId, typeIds] of portToTypes) {
		const root = groups.get(portId) ?? portId;
		const merged = rootToTypes.get(root) ?? new Set<string>();
		for (const typeId of typeIds) merged.add(typeId);
		rootToTypes.set(root, merged);
	}
	for (const typeIds of rootToTypes.values()) {
		if (typeIds.size < 2) continue;
		const sorted = [...typeIds].sort();
		for (let i = 0; i < sorted.length; i++) {
			for (let j = i + 1; j < sorted.length; j++) {
				const left = sorted[i]!;
				const right = sorted[j]!;
				sketchpadKitDiagramPushEdge(edges, edgeIds, `compat-type:${left}-type:${right}`, `type:${left}`, `type:${right}`);
			}
		}
	}
}

function sketchpadKitDiagramFileLabel(file: Record<string, unknown>): string {
	const description = file["description"];
	if (typeof description === "string" && description.length > 0) return description;
	const url = file["url"];
	if (typeof url === "string" && url.length > 0) {
		const slash = url.lastIndexOf("/");
		return slash >= 0 ? url.slice(slash + 1) : url;
	}
	const path = file["path"];
	if (typeof path === "string" && path.length > 0) {
		const slash = path.lastIndexOf("/");
		return slash >= 0 ? path.slice(slash + 1) : path;
	}
	return String(file["id"] ?? "");
}

function sketchpadKitDiagramPushEdge(
	edges: SketchpadPuzzle2dFixtureV1["edges"],
	edgeIds: Set<string>,
	id: string,
	source: string,
	target: string,
): void {
	if (edgeIds.has(id)) return;
	edgeIds.add(id);
	edges.push({ id, source, target });
}

function sketchpadKitDiagramNode(
	kind: SketchpadKitDiagramNodeKind,
	entityId: string,
	label: string,
	root: boolean,
): { node: SketchpadPuzzle2dFixtureV1["nodes"][number]; center: { x: number; y: number } } {
	const nodeId = `${kind}:${entityId}`;
	const frame = sketchpadKitDiagramNodeFrame(kind);
	const center = sketchpadFlatPartCenterFromTopLeft({ x: 0, y: 0 }, frame);
	const base = {
		id: nodeId,
		x: center.x,
		y: center.y,
		text: label,
		nodeKind: `semio.kit.${kind}`,
		root,
		handles: [] as readonly Record<string, unknown>[],
	};
	if (frame.shape === "circle") {
		return {
			node: { ...base, shape: "circle", radius: frame.width / 2 },
			center,
		};
	}
	return {
		node: { ...base, shape: "rectangle", width: frame.width, height: frame.height },
		center,
	};
}

function sketchpadTopologyPayload(flat: SketchpadPuzzle2dFixtureV1, volume: SketchpadVolumeFixtureV1): PlatformTopologyPayload {
	return { flat: flat as unknown as Record<string, unknown>, volume: volume as unknown as Record<string, unknown> };
}

/** @emoji 🗺️ Builds a flat kit topology diagram from kit entities (types, designs, ports, files, …). */
export function sketchpadKitPuzzle2dFixtureFromKit(kit: Kit): SketchpadPuzzle2dFixtureV1 {
	const nodes: SketchpadPuzzle2dFixtureV1["nodes"] = [];
	const edges: SketchpadPuzzle2dFixtureV1["edges"] = [];
	const edgeIds = new Set<string>();
	const centers: { x: number; y: number }[] = [];
	const kindGroups: readonly SketchpadKitDiagramNodeKind[] = ["type", "design", "quality", "port", "file", "folder", "author"];
	for (const kind of kindGroups) {
		let items: readonly { readonly id: string; readonly name: string; readonly parentId?: string }[] = [];
		switch (kind) {
			case "type":
				items = (kit.types ?? []).map((t) => ({
					id: t.id,
					name: t.name ?? t.id,
					parentId: sketchpadReadEntityId((t as { parent?: unknown }).parent) ?? undefined,
				}));
				break;
			case "design":
				items = (kit.designs ?? []).map((d) => ({
					id: d.id,
					name: d.name ?? d.id,
					parentId: sketchpadReadEntityId((d as { parent?: unknown }).parent) ?? undefined,
				}));
				break;
			case "quality":
				items = (kit.qualities ?? []).map((q) => {
					const row = q as { id: string; key?: string; value?: string };
					const key = row.key ?? row.id;
					const label = row.value != null && row.value !== "" ? `${key} · ${row.value}` : key;
					return { id: row.id, name: label };
				});
				break;
			case "port":
				items = sketchpadCollectKitPorts(kit);
				break;
			case "file":
				items = (kit.files ?? []).map((f) => {
					const row = f as Record<string, unknown>;
					return {
						id: String(row["id"] ?? ""),
						name: sketchpadKitDiagramFileLabel(row),
						parentId: sketchpadReadEntityId(row["folder"]) ?? undefined,
					};
				});
				break;
			case "folder":
				items = (kit.folders ?? []).map((f) => {
					const row = f as Record<string, unknown>;
					const path = typeof row["path"] === "string" ? row["path"] : "";
					const slash = path.lastIndexOf("/");
					const name = slash >= 0 ? path.slice(slash + 1) : path || String(row["id"] ?? "");
					return {
						id: String(row["id"] ?? ""),
						name,
						parentId: sketchpadReadEntityId(row["parent"]) ?? undefined,
					};
				});
				break;
			case "author":
				items = (kit.authors ?? []).map((a) => ({
					id: String((a as { id: string }).id),
					name: String((a as { name?: string }).name ?? (a as { id: string }).id),
				}));
				break;
		}
		for (const item of items) {
			if (!item.id) continue;
			const { node, center } = sketchpadKitDiagramNode(kind, item.id, item.name, !item.parentId);
			nodes.push(node);
			centers.push(center);
			if (item.parentId) {
				const parentKind = kind === "file" ? "folder" : kind;
				sketchpadKitDiagramPushEdge(
					edges,
					edgeIds,
					`${kind}-${item.parentId}-${item.id}`,
					`${parentKind}:${item.parentId}`,
					`${kind}:${item.id}`,
				);
			}
		}
	}
	for (const design of kit.designs ?? []) {
		for (const piece of design.pieces ?? []) {
			const typeId = sketchpadReadEntityId((piece as { type?: unknown; blueprint?: unknown }).type ?? (piece as { blueprint?: unknown }).blueprint);
			if (typeId) {
				sketchpadKitDiagramPushEdge(
					edges,
					edgeIds,
					`ref-type:${typeId}-design:${design.id}`,
					`type:${typeId}`,
					`design:${design.id}`,
				);
			}
		}
	}
	for (const type of kit.types ?? []) {
		for (const connector of type.connectors ?? []) {
			const portId = sketchpadReadEntityId((connector as { port?: unknown }).port);
			if (!portId) continue;
			sketchpadKitDiagramPushEdge(
				edges,
				edgeIds,
				`ref-port:${portId}-type:${type.id}`,
				`port:${portId}`,
				`type:${type.id}`,
			);
		}
	}
	sketchpadKitDiagramPushTypeCompatEdges(kit, edges, edgeIds);
	return {
		schema: "puzzle.2d.fixture/v1",
		camera: sketchpadFlatCameraFromPartCenters(centers.length > 0 ? centers : [{ x: 0, y: 0 }]),
		nodes,
		edges,
	};
}

const SKETCHPAD_TOPOLOGY_ICON_WIDTH = 48;
const SKETCHPAD_DESIGN_DIAGRAM_NODE = { width: 80, height: 40 } as const;

type SketchpadKitConnection = {
	readonly id?: string;
	readonly connecting?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly connected?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly parent?: { readonly piece?: unknown; readonly connector?: unknown };
	readonly child?: { readonly piece?: unknown; readonly connector?: unknown };
};

function sketchpadConnectionEndpoints(connection: SketchpadKitConnection): {
	readonly sourcePieceId: string | null;
	readonly targetPieceId: string | null;
	readonly sourceConnectorId: string | null;
	readonly targetConnectorId: string | null;
} {
	const sourcePieceId =
		sketchpadReadEntityId(connection.connecting?.piece) ?? sketchpadReadEntityId(connection.parent?.piece);
	const targetPieceId =
		sketchpadReadEntityId(connection.connected?.piece) ?? sketchpadReadEntityId(connection.child?.piece);
	const sourceConnectorId =
		sketchpadReadEntityId(connection.connecting?.connector) ?? sketchpadReadEntityId(connection.parent?.connector);
	const targetConnectorId =
		sketchpadReadEntityId(connection.connected?.connector) ?? sketchpadReadEntityId(connection.child?.connector);
	return { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId };
}

function sketchpadPieceLabel(piece: { readonly id: string; readonly name?: string | null }, kit?: Kit): string {
	const typeId = sketchpadReadEntityId((piece as { type?: unknown }).type);
	const type = typeId && kit ? findTypeInKit(kit, typeId) : undefined;
	return piece.name ?? type?.name ?? piece.id;
}

function sketchpadPieceDiagramUv(piece: { readonly id: string }, index: number): { readonly u: number; readonly v: number } {
	const row = piece as {
		readonly center?: { readonly u?: number; readonly v?: number };
		readonly position?: { readonly center?: { readonly u?: number; readonly v?: number }; readonly plane?: { readonly origin?: { readonly x?: number; readonly y?: number } } };
		readonly plane?: { readonly origin?: { readonly x?: number; readonly y?: number } };
	};
	const center = row.center ?? row.position?.center;
	if (center && typeof center.u === "number") {
		return { u: center.u, v: typeof center.v === "number" ? center.v : 0 };
	}
	const planeOrigin = row.plane?.origin ?? row.position?.plane?.origin;
	if (planeOrigin) {
		return { u: planeOrigin.x ?? index, v: planeOrigin.y ?? 0 };
	}
	return { u: (index % 8) * 1.2, v: Math.floor(index / 8) * 1.2 };
}

function sketchpadPieceSceneOrigin(piece: { readonly id: string }, index: number): [number, number, number] {
	const row = piece as {
		readonly position?: { readonly plane?: { readonly origin?: { readonly x?: number; readonly y?: number; readonly z?: number } } };
		readonly plane?: { readonly origin?: { readonly x?: number; readonly y?: number; readonly z?: number } };
	};
	const o = row.plane?.origin ?? row.position?.plane?.origin;
	if (o) return [o.x ?? 0, o.y ?? 0, o.z ?? 0];
	return [index * 2, 0, 0];
}

/** @emoji 🧭 Maps kit diagram node ids to sketchpad routes. */
export function sketchpadPathFromDiagramNodeId(kitId: string, diagramNodeId: string): string | null {
	const sep = diagramNodeId.indexOf(":");
	if (sep <= 0) return null;
	const kind = diagramNodeId.slice(0, sep);
	const id = diagramNodeId.slice(sep + 1);
	if (kind === "type") return `/kits/${kitId}/types/${id}`;
	if (kind === "design") return `/kits/${kitId}/designs/${id}`;
	if (kind === "quality" || kind === "port" || kind === "file" || kind === "folder" || kind === "author") {
		return `/kits/${kitId}?${kind}=${encodeURIComponent(id)}`;
	}
	return null;
}

/** @emoji 🧭 Navigates from the first recognized kit diagram selection entry. */
export function sketchpadNavigateFromDiagramSelection(instanceId: string, puzzle2dIds: readonly string[]): void {
	const { kitId, pane } = parseSketchpadPuzzleInstanceId(instanceId);
	if (!kitId || pane !== "kit-diagram") return;
	const ctrl = getSketchpadShellController();
	if (!ctrl) return;
	for (const diagramId of puzzle2dIds) {
		const path = sketchpadPathFromDiagramNodeId(kitId, diagramId);
		if (path) {
			ctrl.navigateTo(path);
			return;
		}
	}
}

/** @emoji 🎯 Applies FiveD puzzle2d/volume selection (kit navigation or design piece/connection selection). */
export function sketchpadApplyPuzzle2dSelection(
	instanceId: string,
	puzzle2dIds: readonly string[],
	controller?: SketchpadShellController,
): void {
	const scope = parseSketchpadPuzzleInstanceId(instanceId);
	const ctrl = controller ?? getSketchpadShellController();
	if (!ctrl || !scope.kitId) return;
	if (scope.pane === "kit-diagram") {
		if (puzzle2dIds.length === 1) {
			const path = sketchpadPathFromDiagramNodeId(scope.kitId, puzzle2dIds[0]!);
			if (path) {
				ctrl.navigateTo(path);
				return;
			}
		}
		ctrl.setRouteSelection({ ...ctrl.routeSelection, kitDiagramNodeIds: [...puzzle2dIds] });
		return;
	}
	if (scope.pane === "diagram" || scope.pane === "scene") {
		const kit = ctrl.getKitStore(scope.kitId)?.getSnapshot().kit;
		const design = scope.designId && kit ? findDesignInKit(kit, scope.designId) : undefined;
		const pieceIdSet = new Set((design?.pieces ?? []).map((piece) => piece.id).filter((id): id is string => Boolean(id)));
		const connectionIdSet = new Set(
			(((design as { connections?: readonly SketchpadKitConnection[] } | undefined)?.connections ?? []) as readonly SketchpadKitConnection[])
				.map((connection) => connection.id)
				.filter((id): id is string => Boolean(id)),
		);
		const pieceIds: string[] = [];
		const connectionIds: string[] = [];
		for (const id of puzzle2dIds) {
			if (!id || id.includes(":")) continue;
			if (id.includes("semio.connection") || id.startsWith("connection:") || connectionIdSet.has(id)) {
				connectionIds.push(id);
			} else if (pieceIdSet.has(id) || !design) {
				pieceIds.push(id);
			}
		}
		ctrl.setRouteSelection({ pieceIds, connectionIds, kitDiagramNodeIds: [] });
	}
}

/** @emoji 🔍 Parses sketchpad CAD {@link CadModel.instanceId}. */
export function parseSketchpadCadInstanceId(instanceId: string): { readonly kitId: string | null; readonly typeId: string | null } {
	const parts = instanceId.split(":");
	if (parts.length === 2) return { kitId: parts[0] ?? null, typeId: parts[1] ?? null };
	return { kitId: null, typeId: null };
}

/** @emoji 🗺️ Builds a flat design diagram from design pieces and connections. */
export function sketchpadDesignPuzzle2dFixtureFromDesign(design: Design, kit?: Kit): SketchpadPuzzle2dFixtureV1 {
	const pieces = design.pieces ?? [];
	const connections = ((design as { connections?: readonly SketchpadKitConnection[] }).connections ?? []) as readonly SketchpadKitConnection[];
	const centers = pieces.map((piece, index) => {
		const uv = sketchpadPieceDiagramUv(piece, index);
		return { x: uv.u * SKETCHPAD_TOPOLOGY_ICON_WIDTH, y: -uv.v * SKETCHPAD_TOPOLOGY_ICON_WIDTH };
	});
	const edges = connections
		.map((connection) => {
			const { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId } = sketchpadConnectionEndpoints(connection);
			if (!sourcePieceId || !targetPieceId || !sourceConnectorId || !targetConnectorId) return null;
			return {
				id: connection.id ?? `${sourcePieceId}-${targetPieceId}`,
				source: sketchpadFlatHandleCompoundId(sourcePieceId, sourceConnectorId),
				target: sketchpadFlatHandleCompoundId(targetPieceId, targetConnectorId),
				edgeKind: "semio.connection",
			};
		})
		.filter((edge): edge is NonNullable<typeof edge> => edge !== null);
	return {
		schema: "puzzle.2d.fixture/v1",
		camera: sketchpadFlatCameraFromPartCenters(centers.length > 0 ? centers : [{ x: 0, y: 0 }]),
		nodes: pieces.map((piece, index) => {
			const uv = sketchpadPieceDiagramUv(piece, index);
			return {
				id: piece.id,
				shape: "rectangle",
				width: SKETCHPAD_DESIGN_DIAGRAM_NODE.width,
				height: SKETCHPAD_DESIGN_DIAGRAM_NODE.height,
				x: uv.u * SKETCHPAD_TOPOLOGY_ICON_WIDTH,
				y: -uv.v * SKETCHPAD_TOPOLOGY_ICON_WIDTH,
				text: sketchpadPieceLabel(piece, kit),
				nodeKind: "semio.design.piece",
				root: true,
				handles: [],
			};
		}),
		edges,
	};
}

/** @emoji 🌐 Builds a 3D design scene volume from design pieces (placeholder meshes until file URLs are wired). */
export function sketchpadDesignVolumeFixtureFromDesign(design: Design, kit?: Kit): SketchpadVolumeFixtureV1 {
	const pieces = design.pieces ?? [];
	const connections = ((design as { connections?: readonly SketchpadKitConnection[] }).connections ?? []) as readonly SketchpadKitConnection[];
	const fileUrls = kit ? sketchpadKitFileUrlById(kit) : new Map<string, string>();
	const objects = pieces.map((piece, index) => ({
		id: piece.id,
		objectKind: "semio.design.piece",
		meshUrl: kit ? sketchpadResolvePieceMeshUrl(piece, kit, fileUrls) : SKETCHPAD_PLACEHOLDER_MESH_URL,
		origin: sketchpadPieceSceneOrigin(piece, index),
		orientation: [0, 0, 0, 1] as [number, number, number, number],
		scale: [1, 1, 1] as [number, number, number],
		label: sketchpadPieceLabel(piece, kit),
		vortices: [],
	}));
	const attractions = connections
		.map((connection) => {
			const { sourcePieceId, targetPieceId, sourceConnectorId, targetConnectorId } = sketchpadConnectionEndpoints(connection);
			if (!sourcePieceId || !targetPieceId || !sourceConnectorId || !targetConnectorId) return null;
			return {
				id: connection.id ?? `${sourcePieceId}-${targetPieceId}`,
				attracting: sketchpadTopologyAnchorFullId(sourcePieceId, sourceConnectorId),
				attracted: sketchpadTopologyAnchorFullId(targetPieceId, targetConnectorId),
				attractionKind: "semio.connection",
			};
		})
		.filter((attraction): attraction is NonNullable<typeof attraction> => attraction !== null);
	const camera = sketchpadSceneCameraFromDesign(design);
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera,
		objects,
		attractions,
	};
}

function sketchpadSceneCameraFromDesign(design: Design): SketchpadVolumeFixtureV1["camera"] {
	const pieces = design.pieces ?? [];
	if (pieces.length === 0) {
		return { position: [8, 8, 8], target: [0, 0, 0], zoom: 1 };
	}
	let sx = 0;
	let sy = 0;
	let sz = 0;
	let count = 0;
	for (const piece of pieces) {
		const [x, y, z] = sketchpadPieceSceneOrigin(piece, count);
		sx += x;
		sy += y;
		sz += z;
		count += 1;
	}
	const target: [number, number, number] = [sx / count, sy / count, sz / count];
	return { position: [target[0] + 8, target[1] + 8, target[2] + 8], target, zoom: 1 };
}

function sketchpadTopologyPayloadForKitDiagram(kit: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(sketchpadKitPuzzle2dFixtureFromKit(kit), sketchpadEmptyVolumeFixture());
}

function sketchpadTopologyPayloadForDesignScene(design: Design, kit?: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(
		sketchpadDesignPuzzle2dFixtureFromDesign(design, kit),
		sketchpadDesignVolumeFixtureFromDesign(design, kit),
	);
}

function sketchpadTopologyPayloadForDesignDiagram(design: Design, kit?: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(sketchpadDesignPuzzle2dFixtureFromDesign(design, kit), sketchpadEmptyVolumeFixture());
}

/** @emoji 🌐 Builds a single-mesh 3D volume for a kit kind (type CAD scene). */
export function sketchpadTypeVolumeFixtureFromType(type: Type, kit: Kit): SketchpadVolumeFixtureV1 {
	const fileUrls = sketchpadKitFileUrlById(kit);
	return {
		schema: "puzzle.3d.fixture/v1",
		domain: "architecture",
		camera: { position: [4, 4, 4], target: [0, 0, 0], zoom: 1 },
		objects: [
			{
				id: type.id,
				objectKind: "semio.type",
				meshUrl: sketchpadResolveTypeMeshUrl(type, kit, fileUrls),
				origin: [0, 0, 0] as [number, number, number],
				orientation: [0, 0, 0, 1] as [number, number, number, number],
				scale: [1, 1, 1] as [number, number, number, number],
				label: type.name ?? type.id,
				vortices: [],
			},
		],
		attractions: [],
	};
}

function sketchpadTopologyPayloadForTypeScene(type: Type, kit: Kit): PlatformTopologyPayload {
	return sketchpadTopologyPayload(sketchpadEmptyPuzzle2dFixture(), sketchpadTypeVolumeFixtureFromType(type, kit));
}

function sketchpadEmptyPuzzle2dFixture(): SketchpadPuzzle2dFixtureV1 {
	return { schema: "puzzle.2d.fixture/v1", camera: { x: 0, y: 0, zoom: 1 }, nodes: [], edges: [] };
}
//#endregion 🔖Topology

export const SKETCHPAD_SHELL_CONTROLLER_ID = "semio.sketchpad.shell";
const SKETCHPAD_EXTENSION_ID = "semio.sketchpad.builtin";
export const SKETCHPAD_HOME_APP_ID = "home";
export const SKETCHPAD_KIT_APP_ID = "kit";
export const SKETCHPAD_DESIGN_APP_ID = "design";
export const SKETCHPAD_TYPE_APP_ID = "type";
export const SKETCHPAD_DOCS_APP_ID = "docs";
export const SKETCHPAD_FEEDBACK_APP_ID = "feedback";
const SKETCHPAD_BODY_HOME = "semio.sketchpad.window.home";
const SKETCHPAD_BODY_KIT_TABLE = "semio.sketchpad.window.kit.table";
const SKETCHPAD_BODY_KIT_DIAGRAM = "semio.sketchpad.window.kit.diagram";
const SKETCHPAD_BODY_DESIGN_SCENE = "semio.sketchpad.window.design.scene";
const SKETCHPAD_BODY_DESIGN_DIAGRAM = "semio.sketchpad.window.design.diagram";
const SKETCHPAD_BODY_TYPE = "semio.sketchpad.window.type";
const SKETCHPAD_BODY_DOCS = "semio.sketchpad.window.docs";
const SKETCHPAD_BODY_FEEDBACK = "semio.sketchpad.window.feedback";
const SKETCHPAD_SURFACE_KIT_TABLE = "semio.sketchpad.surface.kit.table/v1";
const SKETCHPAD_SURFACE_KIT_DIAGRAM = "semio.sketchpad.surface.kit.diagram/v1";
const SKETCHPAD_SURFACE_DESIGN_SCENE = "semio.sketchpad.surface.design.scene/v1";
const SKETCHPAD_SURFACE_DESIGN_DIAGRAM = "semio.sketchpad.surface.design.diagram/v1";
const SKETCHPAD_SURFACE_WORKBENCH = "semio.sketchpad.surface.workbench/v1";
const SKETCHPAD_SURFACE_DETAILS = "semio.sketchpad.surface.details/v1";
const SKETCHPAD_SURFACE_TYPE_SCENE = "semio.sketchpad.surface.type.scene/v1";
export const SKETCHPAD_SURFACE_DOCS_PAGE = "semio.sketchpad.surface.docs.page/v1";
const SKETCHPAD_SURFACE_FEEDBACK_FORM = "semio.sketchpad.surface.feedback.form/v1";
const SKETCHPAD_PANEL_WINDOWS_BODY = "semio.sketchpad.panel.windows";
const SKETCHPAD_PANEL_WORKBENCH_BODY = "semio.sketchpad.panel.workbench";
const SKETCHPAD_PANEL_DETAILS_BODY = "semio.sketchpad.panel.details";

//#region 🔖SketchpadPlatformComponents
abstract class SketchpadRoutedComponent<TSnapshot> extends Component<TSnapshot> {
	protected route = parseSketchpadRouteScopeFromPath("/");
	private readonly detachRoute: () => void;
	private readonly detachShellStore?: () => void;
	private detachKitStore?: () => void;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialSnapshot: TSnapshot, platform: Platform) {
		super(componentKind, surfaceId, controllerId, initialSnapshot);
		this.route = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
		this.detachRoute = platform.subscribe(() => {
			const nextRoute = parseSketchpadRouteScopeFromPath(platform.uri.split("?")[0] ?? "/");
			if (
				nextRoute.kitId !== this.route.kitId ||
				nextRoute.designId !== this.route.designId ||
				nextRoute.typeId !== this.route.typeId ||
				nextRoute.docsPath !== this.route.docsPath
			) {
				this.route = nextRoute;
				this.attachActiveKitStore();
				this.refresh();
			}
		});
		const shellStore = getSketchpadShellController()?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (shellStore) {
			this.detachShellStore = shellStore.subscribe(() => this.refresh());
		}
		this.attachActiveKitStore();
	}

	protected attachActiveKitStore(): void {
		this.detachKitStore?.();
		this.detachKitStore = undefined;
		const { kitId } = this.route;
		if (!kitId) return;
		const store = getSketchpadShellController()?.getKitStore(kitId);
		if (store) {
			this.detachKitStore = store.subscribe(() => {
				this.syncTopologyForSurface();
				this.refresh();
			});
			this.syncTopologyForSurface();
		}
	}

	/** @emoji 🔄 Pushes kit/design data into controller-owned topology stores for FiveD surfaces. */
	protected syncTopologyForSurface(): void {
		getSketchpadShellController()?.syncTopologyForSurface(this.surfaceId, this.route);
	}

	dispose(): void {
		this.detachRoute();
		this.detachShellStore?.();
		this.detachKitStore?.();
		super.dispose();
	}
}

/** @emoji 🏠 Home kits table backed by the kit registry bridge. */
export class SketchpadHomeTable extends Table {
	constructor(platform: Platform) {
		super(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID);
		platform.subscribe(() => this.refresh());
		const shellStore = getSketchpadShellController()?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL);
		if (shellStore) {
			shellStore.subscribe(() => this.refresh());
		}
	}

	override buildSnapshot(): TableModel {
		const ctrl = getSketchpadShellController();
		if (!ctrl) {
			return { columns: [], rows: [], emptyMessage: "Platform loading…" };
		}
		const shell = ctrl.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
		return sketchpadBuildHomeTableModel({
			openKitIds: ctrl.listOpenKitIds(),
			kitById: (kitId) => ctrl.getKitStore(kitId)?.getSnapshot().kit,
			kitKind: (kitId) => ctrl.getKitPersistenceKind(kitId) ?? "",
			home: shell?.home ?? sketchpadEmptyHomeUiState(),
		});
	}
}

/** @emoji 📊 Active kit table surface. */
export class SketchpadKitTable extends SketchpadRoutedComponent<TableModel> {
	constructor(platform: Platform) {
		super("table", SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, { columns: [], rows: [] }, platform);
	}

	override buildSnapshot(): TableModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { columns: [], rows: [], emptyMessage: "Open a kit to view the table" };
		}
		const store = getSketchpadShellController()?.getKitStore(kitId);
		if (!store) {
			return { columns: [], rows: [], emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit;
		const types = kit.types ?? [];
		const designs = kit.designs ?? [];
		const qualities = kit.qualities ?? [];
		const ports = sketchpadCollectKitPorts(kit);
		const files = kit.files ?? [];
		const folders = kit.folders ?? [];
		const authors = kit.authors ?? [];
		return {
			columns: [
				{ id: "name", label: "Name" },
				{ id: "kind", label: "Kind" },
				{ id: "pieces", label: "Count" },
			],
			rows: [
				...types
					.filter((t): t is Type => typeof t === "object" && t !== null && "id" in t)
					.map((t) => ({
						id: `type:${t.id}`,
						cells: {
							name: t.name ?? t.id,
							kind: "type",
							pieces: String(t.connectors?.length ?? 0),
						},
						navigateUri: `/kits/${kitId}/types/${t.id}`,
					})),
				...designs
					.filter((d): d is Design => typeof d === "object" && d !== null && "id" in d)
					.map((d) => ({
						id: `design:${d.id}`,
						cells: {
							name: d.name ?? d.id,
							kind: "design",
							pieces: String(d.pieces?.length ?? 0),
						},
						navigateUri: `/kits/${kitId}/designs/${d.id}`,
					})),
				...qualities.map((q) => {
					const row = q as { id: string; key?: string; value?: string };
					const key = row.key ?? row.id;
					const label = row.value != null && row.value !== "" ? `${key} · ${row.value}` : key;
					return {
						id: `quality:${row.id}`,
						cells: { name: label, kind: "quality", pieces: "—" },
						navigateUri: `/kits/${kitId}?quality=${encodeURIComponent(row.id)}`,
					};
				}),
				...ports.map((p) => ({
					id: `port:${p.id}`,
					cells: { name: p.name, kind: "port", pieces: "—" },
					navigateUri: `/kits/${kitId}?port=${encodeURIComponent(p.id)}`,
				})),
				...files.map((f) => {
					const row = f as Record<string, unknown>;
					const id = String(row["id"] ?? "");
					return {
						id: `file:${id}`,
						cells: { name: sketchpadKitDiagramFileLabel(row), kind: "file", pieces: "—" },
						navigateUri: `/kits/${kitId}?file=${encodeURIComponent(id)}`,
					};
				}),
				...folders.map((f) => {
					const row = f as Record<string, unknown>;
					const id = String(row["id"] ?? "");
					const path = typeof row["path"] === "string" ? row["path"] : id;
					const slash = path.lastIndexOf("/");
					const name = slash >= 0 ? path.slice(slash + 1) : path;
					return {
						id: `folder:${id}`,
						cells: { name, kind: "folder", pieces: "—" },
						navigateUri: `/kits/${kitId}?folder=${encodeURIComponent(id)}`,
					};
				}),
				...authors.map((a) => ({
					id: `author:${(a as { id: string }).id}`,
					cells: { name: String((a as { name?: string }).name ?? (a as { id: string }).id), kind: "author", pieces: "—" },
					navigateUri: `/kits/${kitId}?author=${encodeURIComponent((a as { id: string }).id)}`,
				})),
			],
			emptyMessage: "No kit entities in this kit",
		};
	}
}

/** @emoji 📋 Kit diagram surface (FiveD flat topology). */
export class SketchpadKitDiagram extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_KIT_DIAGRAM,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_DIAGRAM },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId } = this.route;
		if (!kitId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_DIAGRAM, emptyMessage: "Open a kit to view the diagram" };
		}
		const store = getSketchpadShellController()?.getKitStore(kitId);
		if (!store) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_KIT_DIAGRAM, emptyMessage: "Kit loading…" };
		}
		const kit = store.getSnapshot().kit;
		const hasContent =
			(kit.types?.length ?? 0) +
				(kit.designs?.length ?? 0) +
				(kit.qualities?.length ?? 0) +
				sketchpadCollectKitPorts(kit).length +
				(kit.files?.length ?? 0) +
				(kit.folders?.length ?? 0) +
				(kit.authors?.length ?? 0) >
			0;
		return {
			presentation: "flat",
			instanceId: sketchpadKitDiagramInstanceId(kitId),
			emptyMessage: hasContent ? undefined : "No kit entities to diagram",
		};
	}
}

/** @emoji 🎬 Design scene (5D volume). */
export class SketchpadDesignScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_SCENE,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "volume", instanceId: SKETCHPAD_SURFACE_DESIGN_SCENE, emptyMessage: "Open a design to view the scene" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		const design = kit ? findDesignInKit(kit, designId) : undefined;
		return {
			presentation: "volume",
			instanceId: sketchpadDesignSceneInstanceId(kitId, designId),
			emptyMessage: design ? undefined : `Design ${designId} not found`,
		};
	}
}

/** @emoji 📐 Design diagram (5D flat). */
export class SketchpadDesignDiagram extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_DESIGN_DIAGRAM,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, designId } = this.route;
		if (!kitId || !designId) {
			return { presentation: "flat", instanceId: SKETCHPAD_SURFACE_DESIGN_DIAGRAM, emptyMessage: "Open a design to view the diagram" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		const row = kit ? findDesignInKit(kit, designId) : undefined;
		return {
			presentation: "flat",
			instanceId: sketchpadDesignDiagramInstanceId(kitId, designId),
			emptyMessage: row ? undefined : `Design ${designId} not found`,
		};
	}
}

/** @emoji 📐 Type CAD scene (5D volume with representation mesh). */
export class SketchpadTypeScene extends SketchpadRoutedComponent<Puzzle5dModel> {
	constructor(platform: Platform) {
		super(
			"puzzle5d",
			SKETCHPAD_SURFACE_TYPE_SCENE,
			SKETCHPAD_SHELL_CONTROLLER_ID,
			{ presentation: "volume", instanceId: SKETCHPAD_SURFACE_TYPE_SCENE },
			platform,
		);
	}

	override buildSnapshot(): Puzzle5dModel {
		const { kitId, typeId } = this.route;
		if (!kitId || !typeId) {
			return { presentation: "volume", instanceId: SKETCHPAD_SURFACE_TYPE_SCENE, emptyMessage: "Open a type to view the CAD scene" };
		}
		const kit = getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit;
		const type = kit ? findTypeInKit(kit, typeId) : undefined;
		return {
			presentation: "volume",
			instanceId: sketchpadTypeSceneInstanceId(kitId, typeId),
			emptyMessage: type ? undefined : `Type ${typeId} not found`,
		};
	}
}

/** @emoji 📄 Docs panel surface. */
export class SketchpadDocsPanel extends SketchpadRoutedComponent<PanelModel> {
	constructor(platform: Platform) {
		super("panel", SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID, { body: { type: "text", value: "Docs" } }, platform);
	}

	override buildSnapshot(): PanelModel {
		const docsPath = this.route.docsPath;
		return {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: sketchpadTitleFromDocPath(docsPath), emphasize: true },
					{ type: "text", value: `/docs/${docsPath}` },
					{ type: "text", value: "MDX content renders in the docs window." },
				],
			},
		};
	}
}

/** @emoji 💬 Feedback panel surface. */
export class SketchpadFeedbackPanel extends Panel {
	constructor(_platform: Platform) {
		super(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID, {
			body: {
				type: "stack",
				direction: "vertical",
				padding: "standard",
				children: [
					{ type: "text", value: "Feedback", emphasize: true },
					{ type: "text", value: "Send feedback from the footer or command palette." },
				],
			},
		});
	}
}

/** @emoji 🧩 Workbench side panel for the active route. */
class SketchpadWorkbenchPanel extends SketchpadRoutedComponent<PanelModel> {
	constructor(platform: Platform) {
		super("panel", SKETCHPAD_SURFACE_WORKBENCH, SKETCHPAD_SHELL_CONTROLLER_ID, { body: { type: "text", value: "" } }, platform);
	}

	override buildSnapshot(): PanelModel {
		const ctrl = getSketchpadShellController();
		const { kitId, designId, typeId } = this.route;
		if (!kitId) {
			const path = getSketchpadPlatform()?.uri.split("?")[0] ?? "/";
			if (path.startsWith("/docs")) {
				const docsPath = parseSketchpadRouteScopeFromPath(path).docsPath;
				const children: UiNode[] = [
					{ type: "text", value: "Documentation", emphasize: true },
					{ type: "text", value: sketchpadTitleFromDocPath(docsPath) },
				];
				for (const section of sketchpadBuildDocsRegistry()) {
					const inSection = section.pages.some((entry) => entry.path === docsPath);
					if (!inSection) continue;
					children.push({ type: "text", value: `Section · ${section.label}`, emphasize: true });
					for (const page of section.pages) {
						children.push({
							type: "button",
							label: page.title,
							command: {
								controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
								command: "navigate",
								args: { path: `/docs/${page.path}` },
							},
							style: page.path === docsPath ? { variant: "success" } : { variant: "subtle" },
						});
					}
					break;
				}
				return { body: { type: "stack", direction: "vertical", padding: "standard", gap: "tight", children } };
			}
			const open = ctrl?.listOpenKitIds() ?? [];
			const shell = ctrl?.getStore<SketchpadShellSnapshot>(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot();
			const importStatus = shell?.importStatus ?? sketchpadEmptyImportStatus();
			if (importStatus.phase === "importing") {
				return sketchpadPanelTextStack([
					{ text: "Importing kit", emphasize: true },
					{ text: importStatus.label ?? "…" },
				]);
			}
			if (importStatus.phase === "error") {
				return sketchpadPanelTextStack([
					{ text: "Import failed", emphasize: true },
					{ text: importStatus.error ?? "Unknown error" },
				]);
			}
			if (importStatus.phase === "success") {
				return sketchpadPanelTextStack([
					{ text: "Import complete", emphasize: true },
					{ text: importStatus.label ?? "Kit ready" },
				]);
			}
			const selected = shell?.home.selectedKitIds ?? [];
			if (selected.length > 0) {
				const lines: { text: string; emphasize?: boolean }[] = [
					{ text: "Home", emphasize: true },
					{ text: `${selected.length} kit(s) selected` },
				];
				for (const id of selected.slice(0, 5)) {
					const kit = ctrl?.getKitStore(id)?.getSnapshot().kit;
					lines.push({ text: kit?.name ?? id });
				}
				if (selected.length > 5) lines.push({ text: "…" });
				return sketchpadPanelTextStack(lines);
			}
			return {
				body: {
					type: "stack",
					direction: "vertical",
					padding: "standard",
					gap: "tight",
					children: [
						{ type: "text", value: "Workbench", emphasize: true },
						{ type: "text", value: `${open.length} kit(s) open` },
						sketchpadPanelCommandButton("Import kit archive…", "importKitFromFile"),
						sketchpadPanelCommandButton("Create empty kit", "createTemporaryKit", { name: "Untitled Kit" }),
						sketchpadPanelCommandButton("Open metabolism fixture", "importFixtureKit"),
						sketchpadPanelCommandButton("Open Nakagin filtered fixture", "importNakaginFilteredKit"),
						{ type: "text", value: "Drag a .zip onto Home or use the command palette." },
					],
				},
			};
		}
		const kitStore = ctrl?.getKitStore(kitId);
		const kit = kitStore?.getSnapshot().kit;
		const kind = ctrl?.getKitPersistenceKind(kitId) ?? "";
		if (designId && kit) {
			const design = findDesignInKit(kit, designId);
			const pieceCount = design?.pieces?.length ?? 0;
			const selected = ctrl?.routeSelection.pieceIds ?? [];
			const lines: { text: string; emphasize?: boolean }[] = [
				{ text: "Design", emphasize: true },
				{ text: design?.name ?? designId },
				{ text: `${pieceCount} piece(s) · ${selected.length} selected` },
				{ text: `Kit · ${kit.name ?? kitId} (${kind})` },
			];
			if (selected.length > 0) {
				const names = selected
					.map((id) => findPieceInDesign(design!, id)?.name ?? id)
					.slice(0, 4)
					.join(", ");
				lines.push({ text: `Selection · ${names}${selected.length > 4 ? "…" : ""}` });
			}
			return sketchpadPanelTextStack(lines);
		}
		if (typeId && kit) {
			const type = findTypeInKit(kit, typeId);
			return sketchpadPanelTextStack([
				{ text: "Type", emphasize: true },
				{ text: type?.name ?? typeId },
				{ text: `Kit · ${kit.name ?? kitId} (${kind})` },
			]);
		}
		const diagramSelected = ctrl?.routeSelection.kitDiagramNodeIds ?? [];
		if (diagramSelected.length > 0 && kit) {
			const lines: { text: string; emphasize?: boolean }[] = [
				{ text: "Kit diagram", emphasize: true },
				{ text: `${diagramSelected.length} node(s) selected` },
			];
			for (const diagramId of diagramSelected.slice(0, 6)) {
				lines.push({ text: diagramId });
			}
			if (diagramSelected.length > 6) lines.push({ text: "…" });
			return sketchpadPanelTextStack(lines);
		}
		if (kit) {
			const types = kit.types?.length ?? 0;
			const designs = kit.designs?.length ?? 0;
			const lines: { text: string; emphasize?: boolean }[] = [
				{ text: "Kit", emphasize: true },
				{ text: kit.name ?? kitId },
				{ text: `${types} type(s) · ${designs} design(s)` },
				{ text: kind ? `Persistence · ${kind}` : "Persistence · unknown" },
			];
			if (kit.version) lines.push({ text: `Version · ${kit.version}` });
			const updated = sketchpadFormatKitTimestamp(kit.updatedAt ?? kit.createdAt);
			if (updated) lines.push({ text: `Updated · ${updated}` });
			if (kit.description) lines.push({ text: kit.description });
			return sketchpadPanelTextStack(lines);
		}
		return sketchpadPanelTextStack([{ text: "Kit loading…" }]);
	}
}

/** @emoji 🔎 Details side panel for the active route. */
class SketchpadDetailsPanel extends SketchpadRoutedComponent<PanelModel> {
	constructor(platform: Platform) {
		super("panel", SKETCHPAD_SURFACE_DETAILS, SKETCHPAD_SHELL_CONTROLLER_ID, { body: { type: "text", value: "" } }, platform);
	}

	override buildSnapshot(): PanelModel {
		const ctrl = getSketchpadShellController();
		const { kitId, designId, typeId } = this.route;
		if (!kitId) {
			return sketchpadPanelTextStack([
				{ text: "Details", emphasize: true },
				{ text: "No kit in scope." },
			]);
		}
		const kit = ctrl?.getKitStore(kitId)?.getSnapshot().kit;
		if (!kit) {
			return sketchpadPanelTextStack([{ text: "Details", emphasize: true }, { text: "Kit loading…" }]);
		}
		if (designId) {
			const design = findDesignInKit(kit, designId);
			const lines: { text: string; emphasize?: boolean }[] = [
				{ text: "Design details", emphasize: true },
				{ text: `Id · ${designId}` },
			];
			if (design?.description) lines.push({ text: design.description });
			if (design?.unit) lines.push({ text: `Unit · ${design.unit}` });
			lines.push({ text: `Pieces · ${design?.pieces?.length ?? 0}` });
			const selected = ctrl?.routeSelection.pieceIds ?? [];
			if (selected.length === 1) {
				const piece = findPieceInDesign(design!, selected[0]!);
				if (piece) {
					lines.push({ text: `Selected · ${piece.name ?? piece.id}` });
					const typeId = sketchpadReadEntityId((piece as { type?: unknown }).type);
					if (typeId) lines.push({ text: `Type · ${findTypeInKit(kit, typeId)?.name ?? typeId}` });
				}
			} else if (selected.length > 1) {
				lines.push({ text: `${selected.length} pieces selected` });
			}
			return sketchpadPanelTextStack(lines);
		}
		if (typeId) {
			const type = findTypeInKit(kit, typeId);
			const lines: { text: string; emphasize?: boolean }[] = [
				{ text: "Type details", emphasize: true },
				{ text: `Id · ${typeId}` },
			];
			if (type?.description) lines.push({ text: type.description });
			if (type?.unit) lines.push({ text: `Unit · ${type.unit}` });
			const reps = type?.representations?.length ?? 0;
			const connectors = type?.connectors?.length ?? 0;
			lines.push({ text: `Representations · ${reps} · Connectors · ${connectors}` });
			return sketchpadPanelTextStack(lines);
		}
		return sketchpadPanelTextStack([
			{ text: "Kit details", emphasize: true },
			{ text: `Id · ${kitId}` },
			{ text: kit.description ?? kit.name ?? kitId },
			{ text: `Authors · ${kit.authors?.length ?? 0} · Tags · ${kit.tags?.length ?? 0}` },
		]);
	}
}

class SketchpadPlatformComponents {
	readonly components: readonly Component<unknown>[];

	constructor(platform: Platform) {
		this.components = [
			new SketchpadHomeTable(platform),
			new SketchpadKitTable(platform),
			new SketchpadKitDiagram(platform),
			new SketchpadDesignScene(platform),
			new SketchpadDesignDiagram(platform),
			new SketchpadTypeScene(platform),
			new SketchpadDocsPanel(platform),
			new SketchpadFeedbackPanel(platform),
			new SketchpadWorkbenchPanel(platform),
			new SketchpadDetailsPanel(platform),
		];
		for (const component of this.components) {
			registerPlatformComponent(platform, component);
			component.refresh();
		}
		platform.subscribe(() => {
			for (const component of this.components) {
				component.refresh();
			}
		});
	}
}
//#endregion 🔖SketchpadPlatformComponents

/** @emoji 🧭 Routes sketchpad navigation and panel chrome through {@link CommandBus}. */
export class SketchpadShellController extends Controller {
	private readonly shellStore: ObservableCell<SketchpadShellSnapshot>;
	private readonly kitKinds = new Map<string, string>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SKETCHPAD_SHELL_CONTROLLER_ID, commandBus, hostNotify);
		this.shellStore = new ObservableCell<SketchpadShellSnapshot>({
			navigationPath: "/",
			panelVisibility: { leftSidePanel: false, rightSidePanel: false },
			openKitIds: [],
			routeSelection: sketchpadEmptyRouteSelection(),
			home: sketchpadEmptyHomeUiState(),
			importStatus: sketchpadEmptyImportStatus(),
		});
		this.provideStore(SKETCHPAD_SHELL_STORE_SHELL, this.shellStore);
	}

	get navigationPath(): string {
		return this.shellStore.get().navigationPath;
	}

	get panelVisibility(): SketchpadShellSnapshot["panelVisibility"] {
		return this.shellStore.get().panelVisibility;
	}

	get routeSelection(): SketchpadRouteSelection {
		return this.shellStore.get().routeSelection;
	}

	/** @emoji 📥 Updates home kit import status for workbench feedback. */
	setImportStatus(status: SketchpadImportStatus): void {
		this.shellStore.set({ ...this.shellStore.get(), importStatus: status });
		this.emit();
	}

	/** @emoji 🎯 Updates diagram/scene selection and syncs `/kits/...` query params when applicable. */
	setRouteSelection(selection: SketchpadRouteSelection): void {
		const shell = this.shellStore.get();
		const pathOnly = shell.navigationPath.split("?")[0] ?? "/";
		if (!sketchpadPathSupportsRouteSelectionQuery(pathOnly)) {
			this.shellStore.set({ ...shell, routeSelection: selection });
			this.emit();
			return;
		}
		const navigationPath = `${pathOnly}${sketchpadRouteSelectionUriFilters(selection)}`;
		this.shellStore.set({ ...shell, routeSelection: selection, navigationPath });
		const platform = getSketchpadPlatform();
		if (platform?.onNavigate) platform.onNavigate(navigationPath);
		else if (platform) applySketchpadUri(platform, navigationPath);
		this.emit();
	}

	/** @emoji 📋 Open kit ids from the shell store snapshot. */
	listOpenKitIds(): readonly string[] {
		return this.shellStore.get().openKitIds;
	}

	/** @emoji 🗄️ Registers a kit store on this controller (`kit:<id>`). */
	registerKitStore(kitId: string, store: SemioKitStore, options?: { readonly kind?: string }): void {
		this.provideStore(sketchpadKitStoreId(kitId), store);
		if (options?.kind) this.kitKinds.set(kitId, options.kind);
		const openKitIds = this.shellStore.get().openKitIds;
		if (!openKitIds.includes(kitId)) {
			this.shellStore.set({ ...this.shellStore.get(), openKitIds: [...openKitIds, kitId] });
		}
		this.emit();
	}

	/** @emoji 🔍 Resolves a controller-owned kit store. */
	getKitStore(kitId: string): SemioKitStore | undefined {
		return this.getStore<SketchpadKitSnapshot>(sketchpadKitStoreId(kitId)) as SemioKitStore | undefined;
	}

	/** @emoji 🏷️ Persistence kind recorded when the kit was opened. */
	getKitPersistenceKind(kitId: string): string | undefined {
		return this.kitKinds.get(kitId);
	}

	/** @emoji 🗺️ Refreshes topology stores for routed FiveD surfaces (kit diagram, design/type scene/diagram). */
	syncTopologyForSurface(
		surfaceId: string,
		route: { readonly kitId: string | null; readonly designId: string | null; readonly typeId: string | null },
	): void {
		const { kitId, designId, typeId } = route;
		if (!kitId) return;
		const kit = this.getKitStore(kitId)?.getSnapshot().kit;
		if (!kit) return;
		if (surfaceId === SKETCHPAD_SURFACE_KIT_DIAGRAM) {
			this.upsertTopologyStore(sketchpadKitDiagramInstanceId(kitId), sketchpadTopologyPayloadForKitDiagram(kit));
			return;
		}
		if (surfaceId === SKETCHPAD_SURFACE_TYPE_SCENE && typeId) {
			const type = findTypeInKit(kit, typeId);
			if (type) {
				this.upsertTopologyStore(sketchpadTypeSceneInstanceId(kitId, typeId), sketchpadTopologyPayloadForTypeScene(type, kit));
			}
			return;
		}
		if (!designId) return;
		const design = findDesignInKit(kit, designId);
		if (!design) return;
		if (surfaceId === SKETCHPAD_SURFACE_DESIGN_SCENE) {
			this.upsertTopologyStore(sketchpadDesignSceneInstanceId(kitId, designId), sketchpadTopologyPayloadForDesignScene(design, kit));
			return;
		}
		if (surfaceId === SKETCHPAD_SURFACE_DESIGN_DIAGRAM) {
			this.upsertTopologyStore(sketchpadDesignDiagramInstanceId(kitId, designId), sketchpadTopologyPayloadForDesignDiagram(design, kit));
		}
	}

	private upsertTopologyStore(instanceId: string, payload: PlatformTopologyPayload): void {
		const storeId = platformTopologyStoreId(instanceId);
		const existing = this.getStore(storeId) as PlatformTopologyStore | undefined;
		if (existing) {
			existing.replacePayload(payload);
			this.emit();
			return;
		}
		this.provideStore(storeId, new PlatformTopologyStore(payload));
		this.emit();
	}

	/** @emoji 📂 Opens a kit via host factories or in-memory import and navigates to it. */
	async openKit(kind: SketchpadKitPersistenceKind, options?: { readonly serverUrl?: string; readonly importUrl?: string }): Promise<string> {
		if (options?.importUrl) {
			return openSketchpadKitFromImport(options.importUrl, { kind, navigate: true });
		}
		if (kind === "remote" && options?.serverUrl?.trim()) {
			const store = await sketchpadOpenRemoteKitStore(options.serverUrl.trim());
			const kitId = store.getSnapshot().kit.id;
			this.registerKitStore(kitId, store, { kind });
			this.navigateTo(`/kits/${kitId}`);
			return kitId;
		}
		const factory = sketchpadKitBackendFactories[kind];
		if (!factory) {
			throw new Error(`semio/sketchpad: no kit factory registered for kind "${kind}"`);
		}
		const store = sketchpadKitStoreFromFactory(await factory());
		const kitId = store.getSnapshot().kit.id;
		this.registerKitStore(kitId, store, { kind });
		this.navigateTo(`/kits/${kitId}`);
		return kitId;
	}

	/** @emoji 🆕 Creates an empty in-memory kit backed by {@link @semio/js} and opens it. */
	async createTemporaryKit(name = "Untitled Kit"): Promise<string> {
		const session = await SemioSession.openInMemory();
		const jsStore = (await session.stores())[0];
		if (!jsStore) {
			await session.dispose();
			throw new Error("semio/sketchpad: createTemporaryKit found no stores");
		}
		const store = await createSemioKitStoreFromJsStore(jsStore, { onDispose: () => void session.dispose() });
		if (name.trim()) {
			const renamed = await (await store.jsKitEntity()).rename(name.trim());
			if (!renamed.ok) throw new Error(`semio/sketchpad: rename failed: ${renamed.error?.message ?? "unknown"}`);
			await store.refreshFromJs();
		}
		const kitId = store.getSnapshot().kit.id;
		this.registerKitStore(kitId, store, { kind: "temporary" });
		this.navigateTo(`/kits/${kitId}`);
		return kitId;
	}

	/** @emoji 🗑️ Closes a kit store and navigates home when it was active. */
	closeKit(kitId: string): void {
		const shell = this.shellStore.get();
		const openKitIds = shell.openKitIds.filter((id) => id !== kitId);
		this.shellStore.set({ ...shell, openKitIds });
		this.revokeStore(sketchpadKitStoreId(kitId));
		for (const storeId of [...this.stores.keys()]) {
			if (storeId.startsWith(PLATFORM_TOPOLOGY_STORE_PREFIX) && storeId.includes(kitId)) {
				this.revokeStore(storeId);
			}
		}
		this.kitKinds.delete(kitId);
		const platform = getSketchpadPlatform();
		const activePath = platform?.uri.split("?")[0] ?? shell.navigationPath;
		if (activePath.startsWith(`/kits/${kitId}`)) {
			this.navigateTo(openKitIds.length > 0 ? `/kits/${openKitIds[openKitIds.length - 1]}` : "/");
		}
		this.emit();
	}

	/** @emoji 🏠 Merges home UI state and syncs `/` query params when on the home route. */
	updateHome(home: SketchpadHomeUiState): void {
		const shell = this.shellStore.get();
		const pathOnly = shell.navigationPath.split("?")[0] ?? "/";
		const navigationPath = pathOnly === "/" ? `/${sketchpadHomeUriFilters(home)}` : shell.navigationPath;
		this.shellStore.set({ ...shell, home, navigationPath });
		if (pathOnly === "/") {
			const platform = getSketchpadPlatform();
			if (platform?.onNavigate) platform.onNavigate(navigationPath);
			else if (platform) applySketchpadUri(platform, navigationPath);
		}
		this.emit();
	}

	/** @emoji 🧭 Navigates to a path (updates shell snapshot; drives platform when mounted). */
	navigateTo(path: string): void {
		const pathOnly = path.split("?")[0] ?? "/";
		const shell = this.shellStore.get();
		const home = pathOnly === "/" ? parseSketchpadHomeQuery(path) : shell.home;
		const routeSelection = sketchpadPathSupportsRouteSelectionQuery(pathOnly)
			? parseSketchpadRouteSelectionQuery(path)
			: sketchpadEmptyRouteSelection();
		this.shellStore.set({ ...this.shellStore.get(), navigationPath: path, routeSelection, home });
		const platform = getSketchpadPlatform();
		if (!platform) return;
		if (platform.onNavigate) {
			platform.onNavigate(path);
			return;
		}
		applySketchpadUri(platform, path);
	}

	override run(command: string, args?: unknown): void {
		const shell = this.shellStore.get();
		switch (command) {
			case "setNavigation": {
				const path = (args as { path: string }).path;
				const pathOnly = path.split("?")[0] ?? "/";
				const home = pathOnly === "/" ? parseSketchpadHomeQuery(path) : shell.home;
				const routeSelection =
					pathOnly === "/"
						? sketchpadEmptyRouteSelection()
						: sketchpadPathSupportsRouteSelectionQuery(pathOnly)
							? parseSketchpadRouteSelectionQuery(path)
							: sketchpadEmptyRouteSelection();
				this.shellStore.set({ ...shell, navigationPath: path, home, routeSelection });
				break;
			}
			case "toggleHomeRowExpand": {
				const rowId = (args as { rowId: string }).rowId;
				const expanded = new Set(shell.home.expandedRowIds);
				if (expanded.has(rowId)) expanded.delete(rowId);
				else expanded.add(rowId);
				this.updateHome({ ...shell.home, expandedRowIds: [...expanded] });
				break;
			}
			case "toggleTableRowSelection": {
				const rowId = (args as { rowId: string }).rowId;
				if (!shell.openKitIds.includes(rowId)) break;
				const selected = new Set(shell.home.selectedKitIds);
				if (selected.has(rowId)) selected.delete(rowId);
				else selected.add(rowId);
				this.updateHome({ ...shell.home, selectedKitIds: [...selected] });
				break;
			}
			case "setHomeFilters": {
				const payload = args as {
					kind?: string | null;
					q?: string;
					name?: string | null;
					version?: string | null;
				};
				this.updateHome({
					...shell.home,
					kindFilter: payload.kind === undefined ? shell.home.kindFilter : payload.kind,
					searchQuery: payload.q === undefined ? shell.home.searchQuery : payload.q,
					nameFilter: payload.name === undefined ? shell.home.nameFilter : payload.name,
					versionFilter: payload.version === undefined ? shell.home.versionFilter : payload.version,
				});
				break;
			}
			case "setHomeSort": {
				const payload = args as { columnId?: string | null; descending?: boolean };
				this.updateHome({
					...shell.home,
					sortColumnId: payload.columnId === undefined ? shell.home.sortColumnId : payload.columnId,
					sortDescending: payload.descending === undefined ? shell.home.sortDescending : payload.descending,
				});
				break;
			}
			case "cycleTableSort": {
				const payload = args as { columnId: string; surfaceId: string };
				if (payload.surfaceId !== SKETCHPAD_SURFACE_HOME_TABLE) break;
				const home = shell.home;
				const same = home.sortColumnId === payload.columnId;
				this.updateHome({
					...home,
					sortColumnId: payload.columnId,
					sortDescending: same ? !home.sortDescending : false,
				});
				break;
			}
			case "exportActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				if (!kitId) break;
				const kit = this.getKitStore(kitId)?.getSnapshot().kit;
				if (kit) sketchpadDownloadKitJson(kit);
				break;
			}
			case "copyActiveKitJson": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				if (!kitId) break;
				const kit = this.getKitStore(kitId)?.getSnapshot().kit;
				if (!kit) break;
				void sketchpadCopyKitJsonToClipboard(kit).catch((error) => {
					console.error("[semio/sketchpad] copyActiveKitJson failed:", error);
				});
				break;
			}
			case "importKitFromDrop": {
				const file = (args as { file?: File }).file;
				if (!file) break;
				this.setImportStatus({ phase: "importing", label: file.name });
				void (async () => {
					const { kit, session, portCompatSource } = await importKit(file);
					const jsStore = (await session.stores())[0];
					if (!jsStore) throw new Error("semio/sketchpad: importKitFromDrop found no stores");
					const store = await createSemioKitStoreFromJsStore(jsStore, {
						onDispose: () => void session.dispose(),
						portCompatSource,
					});
					const kitId = kit.id;
					this.registerKitStore(kitId, store, { kind: "file" });
					this.setImportStatus({ phase: "success", label: kit.name ?? kitId });
					this.navigateTo(`/kits/${kitId}`);
				})().catch((error) => {
					const message = error instanceof Error ? error.message : String(error);
					console.error("[semio/sketchpad] importKitFromDrop failed:", error);
					this.setImportStatus({ phase: "error", error: message });
				});
				break;
			}
			case "togglePanel": {
				const panel = (args as { panel: "leftSidePanel" | "rightSidePanel" }).panel;
				this.shellStore.set({
					...shell,
					panelVisibility: { ...shell.panelVisibility, [panel]: !shell.panelVisibility[panel] },
				});
				break;
			}
			case "openKit": {
				const payload = args as { kind: SketchpadKitPersistenceKind; serverUrl?: string; importUrl?: string };
				void this.openKit(payload.kind, { serverUrl: payload.serverUrl, importUrl: payload.importUrl }).catch((error) => {
					console.error("[semio.sketchpad] openKit failed:", error);
				});
				break;
			}
			case "importFixtureKit": {
				void seedSketchpadDevFixtureKitIfEmpty().catch((error) => {
					console.warn("[semio.sketchpad] importFixtureKit failed:", error);
				});
				break;
			}
			case "importNakaginFilteredKit": {
				void openSketchpadKitFromImport(SKETCHPAD_DEV_FIXTURE_NAKAGIN_FILTERED_URL, { kind: "fixture", navigate: true }).catch((error) => {
					console.warn("[semio.sketchpad] importNakaginFilteredKit failed:", error);
				});
				break;
			}
			case "importKitFromFile": {
				sketchpadPromptHomeKitArchiveFile();
				break;
			}
			case "navigate": {
				this.navigateTo((args as { path: string }).path);
				break;
			}
			case "setRouteSelection": {
				this.setRouteSelection(args as SketchpadRouteSelection);
				break;
			}
			case "applyPuzzle2dSelection": {
				const payload = args as { instanceId: string; puzzle2dIds: readonly string[] };
				sketchpadApplyPuzzle2dSelection(payload.instanceId, payload.puzzle2dIds);
				break;
			}
			case "createTemporaryKit": {
				const name = (args as { name?: string }).name;
				void this.createTemporaryKit(name).catch((error) => {
					console.error("[semio.sketchpad] createTemporaryKit failed:", error);
				});
				break;
			}
			case "renameActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				const name = (args as { name?: string }).name?.trim();
				if (!kitId || !name) break;
				void executeSketchpadJsKitMutation(kitId, (kit) => kit.rename(name))
					.then((result) => {
						if (!result.ok) console.error("[semio.sketchpad] renameActiveKit failed:", result.error?.message);
					})
					.catch((error) => console.error("[semio.sketchpad] renameActiveKit failed:", error));
				break;
			}
			case "createDesignInActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(shell.navigationPath);
				const designName = (args as { name?: string }).name?.trim() ?? "New design";
				if (!kitId) break;
				void executeSketchpadJsKitMutation(kitId, (kit) => kit.createDesign(designName))
					.then((result) => {
						if (!result.ok) console.error("[semio.sketchpad] createDesignInActiveKit failed:", result.error?.message);
					})
					.catch((error) => console.error("[semio.sketchpad] createDesignInActiveKit failed:", error));
				break;
			}
			case "closeKit": {
				this.closeKit((args as { kitId: string }).kitId);
				break;
			}
			case "closeActiveKit": {
				const kitId = sketchpadActiveKitIdFromPath(this.shellStore.get().navigationPath);
				if (kitId) this.closeKit(kitId);
				break;
			}
			default:
				break;
		}
		this.emit();
	}
}

let sketchpadPlatformSingleton: Platform | null = null;
let sketchpadPluginHostSingleton: PluginHost | null = null;
let sketchpadPlatformReady: Promise<Platform> | null = null;
let sketchpadBodiesRegistered = false;

function sketchpadShellCommand(
	id: string,
	label: string,
	command: string,
	args?: unknown,
	category = "Sketchpad",
): SearchItemSpec {
	return { id, label, category, controllerId: SKETCHPAD_SHELL_CONTROLLER_ID, command, args };
}

function sketchpadHomeCommands(): readonly SearchItemSpec[] {
	return [
		sketchpadShellCommand("semio.sketchpad.home.openFixture", "Open metabolism fixture", "importFixtureKit"),
		sketchpadShellCommand("semio.sketchpad.home.openNakaginFiltered", "Open Nakagin filtered fixture", "importNakaginFilteredKit"),
		sketchpadShellCommand("semio.sketchpad.home.createKit", "Create empty kit", "createTemporaryKit", { name: "Untitled Kit" }),
		sketchpadShellCommand("semio.sketchpad.home.importFile", "Import kit from file", "importKitFromFile"),
		sketchpadShellCommand("semio.sketchpad.home.openFolder", "Open folder kit", "openKit", { kind: "folder" }),
		sketchpadShellCommand("semio.sketchpad.home.openFile", "Open file kit", "openKit", { kind: "file" }),
		sketchpadShellCommand("semio.sketchpad.home.openRemote", "Open remote kit", "openKit", { kind: "remote" }),
		sketchpadShellCommand("semio.sketchpad.home.filterTemporary", "Filter · temporary kits", "setHomeFilters", { kind: "temporary" }),
		sketchpadShellCommand("semio.sketchpad.home.filterFile", "Filter · file kits", "setHomeFilters", { kind: "file" }),
		sketchpadShellCommand("semio.sketchpad.home.clearFilters", "Clear home filters", "setHomeFilters", {
			kind: null,
			q: "",
			name: null,
			version: null,
		}),
		sketchpadShellCommand("semio.sketchpad.home.sortUpdated", "Sort home by updated", "setHomeSort", {
			columnId: "updated",
			descending: true,
		}),
		sketchpadShellCommand("semio.sketchpad.home.sortName", "Sort home by name", "setHomeSort", { columnId: "name", descending: false }),
	];
}

function sketchpadKitAppCommands(): readonly SearchItemSpec[] {
	return [
		sketchpadShellCommand("semio.sketchpad.kit.goHome", "Go to Home", "navigate", { path: "/" }),
		sketchpadShellCommand("semio.sketchpad.kit.close", "Close active kit", "closeActiveKit"),
		sketchpadShellCommand("semio.sketchpad.kit.rename", "Rename kit", "renameActiveKit", { name: "Renamed kit" }),
		sketchpadShellCommand("semio.sketchpad.kit.createDesign", "Create design", "createDesignInActiveKit", { name: "New design" }),
		sketchpadShellCommand("semio.sketchpad.kit.export", "Export active kit JSON", "exportActiveKit"),
		sketchpadShellCommand("semio.sketchpad.kit.copyJson", "Copy active kit JSON", "copyActiveKitJson"),
	];
}

function sketchpadHomePanelTabs(): readonly SideTabSpec[] {
	return [
		{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", panel: "workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY },
		{ id: "details", iconId: "semio.sketchpad.icon.details", panel: "details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY },
	];
}

function sketchpadKitPanelTabs(): readonly SideTabSpec[] {
	return [
		{ id: "windows", iconId: "semio.sketchpad.icon.windows", panel: "windows", bodyKey: SKETCHPAD_PANEL_WINDOWS_BODY },
		{ id: "workbench", iconId: "semio.sketchpad.icon.workbench", panel: "workbench", bodyKey: SKETCHPAD_PANEL_WORKBENCH_BODY },
		{ id: "details", iconId: "semio.sketchpad.icon.details", panel: "details", bodyKey: SKETCHPAD_PANEL_DETAILS_BODY },
	];
}

function buildSketchpadExtensionManifest(): PluginManifest {
	return {
		id: SKETCHPAD_EXTENSION_ID,
		label: "Semio Sketchpad",
		contributes: {
			apps: [
				{
					id: SKETCHPAD_HOME_APP_ID,
					label: "Home",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "home-main", label: "Home", bodyKey: SKETCHPAD_BODY_HOME }],
					defaultLayout: createTabStackLayout(["home-main"], ["Home"]),
					commands: sketchpadHomeCommands(),
					panelTabs: sketchpadHomePanelTabs(),
				},
				{
					id: SKETCHPAD_KIT_APP_ID,
					label: "Kit",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "table", label: "Table", bodyKey: SKETCHPAD_BODY_KIT_TABLE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_KIT_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["table", "diagram"], "row", [50, 50], ["Table", "Diagram"]),
					commands: sketchpadKitAppCommands(),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_DESIGN_APP_ID,
					label: "Design",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [
						{ id: "scene", label: "Scene", bodyKey: SKETCHPAD_BODY_DESIGN_SCENE },
						{ id: "diagram", label: "Diagram", bodyKey: SKETCHPAD_BODY_DESIGN_DIAGRAM },
					],
					defaultLayout: createDefaultLayout(["scene", "diagram"], "row", [60, 40], ["Scene", "Diagram"]),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_TYPE_APP_ID,
					label: "Type",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "type-main", label: "Type", bodyKey: SKETCHPAD_BODY_TYPE }],
					defaultLayout: createTabStackLayout(["type-main"], ["Type"]),
					panelTabs: sketchpadKitPanelTabs(),
				},
				{
					id: SKETCHPAD_DOCS_APP_ID,
					label: "Docs",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "docs-main", label: "Docs", bodyKey: SKETCHPAD_BODY_DOCS }],
					defaultLayout: createTabStackLayout(["docs-main"], ["Docs"]),
				},
				{
					id: SKETCHPAD_FEEDBACK_APP_ID,
					label: "Feedback",
					controllerId: SKETCHPAD_SHELL_CONTROLLER_ID,
					windowKinds: [{ id: "feedback-main", label: "Feedback", bodyKey: SKETCHPAD_BODY_FEEDBACK }],
					defaultLayout: createTabStackLayout(["feedback-main"], ["Feedback"]),
				},
			],
		},
	};
}

function registerSketchpadWindowBodies(): void {
	if (sketchpadBodiesRegistered) return;
	sketchpadBodiesRegistered = true;
	registerWindowBody(SKETCHPAD_BODY_HOME, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_HOME_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "home-main"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_TABLE, () =>
		buildTableWindowBody(SKETCHPAD_SURFACE_KIT_TABLE, SKETCHPAD_SHELL_CONTROLLER_ID, "table"),
	);
	registerWindowBody(SKETCHPAD_BODY_KIT_DIAGRAM, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_KIT_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_SCENE, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, "scene"),
	);
	registerWindowBody(SKETCHPAD_BODY_DESIGN_DIAGRAM, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_DESIGN_DIAGRAM, SKETCHPAD_SHELL_CONTROLLER_ID, "diagram"),
	);
	registerWindowBody(SKETCHPAD_BODY_TYPE, () =>
		buildPuzzle5dWindowBody(SKETCHPAD_SURFACE_TYPE_SCENE, SKETCHPAD_SHELL_CONTROLLER_ID, "scene"),
	);
	registerWindowBody(SKETCHPAD_BODY_DOCS, () => buildPanelWindowBody(SKETCHPAD_SURFACE_DOCS_PAGE, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerWindowBody(SKETCHPAD_BODY_FEEDBACK, () => buildPanelWindowBody(SKETCHPAD_SURFACE_FEEDBACK_FORM, SKETCHPAD_SHELL_CONTROLLER_ID));
	registerSidePanelBody(SKETCHPAD_PANEL_WINDOWS_BODY, (ctx) => {
		const app = ctx.platform.getActiveApp();
		const windowKinds = app?.windowKinds ?? [];
		return {
			type: "stack",
			direction: "vertical",
			gap: "tight",
			children: windowKinds.map((windowKind) => ({
				type: "text",
				value: windowKind.label,
				dataAttributes: { "data-window-kind-id": windowKind.id },
			})),
		};
	});
	registerSidePanelBody(SKETCHPAD_PANEL_WORKBENCH_BODY, () =>
		buildPanelWindowBody(SKETCHPAD_SURFACE_WORKBENCH, SKETCHPAD_SHELL_CONTROLLER_ID, "workbench"),
	);
	registerSidePanelBody(SKETCHPAD_PANEL_DETAILS_BODY, () =>
		buildPanelWindowBody(SKETCHPAD_SURFACE_DETAILS, SKETCHPAD_SHELL_CONTROLLER_ID, "details"),
	);
}

function applySketchpadUri(platform: Platform, uri: string): void {
	const path = uri.split("?")[0] ?? "/";
	platform.uri = uri;
	platform.activeAppId = sketchpadAppIdFromPath(path);
	platform.commandBus.dispatch(SKETCHPAD_SHELL_CONTROLLER_ID, "setNavigation", { path });
	platform.notify();
}

function isSketchpadUuid(value: string): boolean {
	return /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(value);
}

/** @emoji 🍞 Friendly breadcrumb labels for kit routes. */
function sketchpadBreadcrumb(platform: Platform, uri: string): PlatformBreadcrumbItem[] {
	const path = uri.split("?")[0] ?? "/";
	const segments = path.split("/").filter(Boolean);
	const items: PlatformBreadcrumbItem[] = [{ id: "root", content: "Home" }];
	let cumulative = "";
	for (let index = 0; index < segments.length; index++) {
		const segment = segments[index] ?? "";
		cumulative += `/${segment}`;
		const href = cumulative;
		let label: string = segment;
		if (segment === "kits") {
			label = "Kits";
		} else if (segment === "designs") {
			label = "Designs";
		} else if (segment === "types") {
			label = "Types";
		} else if (isSketchpadUuid(segment)) {
			const scope = parseSketchpadRouteScopeFromPath(path);
			const controller = getPlatformControllerById(platform, SKETCHPAD_SHELL_CONTROLLER_ID) as SketchpadShellController | undefined;
			const kitStore =
				scope.kitId && controller ? (controller.getStore<SketchpadKitSnapshot>(`${SKETCHPAD_KIT_STORE_PREFIX}${scope.kitId}`) as SemioKitStore | undefined) : undefined;
			const kit = kitStore?.getSnapshot().kit;
			if (scope.designId && kit) {
				label = findDesignInKit(kit, scope.designId)?.name ?? segment;
			} else if (scope.typeId && kit) {
				label = findTypeInKit(kit, scope.typeId)?.name ?? segment;
			} else if (scope.kitId && kit) {
				label = kit.name ?? segment;
			}
		}
		items.push({ id: href, content: label });
	}
	return items;
}

const SKETCHPAD_PLATFORM_SPEC: PlatformSpec = {
	id: "semio.sketchpad",
	name: "Semio Sketchpad",
	defaultActiveAppId: SKETCHPAD_HOME_APP_ID,
};

/** @emoji 🧱 Builds the sketchpad {@link Platform} (apps, window bodies, {@link Component} registry). */
export async function buildSketchpadPlatform(): Promise<Platform> {
	sketchpadConfigureBrowserKitFactories();
	registerSketchpadWindowBodies();
	const platform = new Platform(SKETCHPAD_PLATFORM_SPEC);
	const controller = new SketchpadShellController(platform.commandBus, () => platform.notify());
	sketchpadShellControllerSingleton = controller;
	const host = new PluginHost(platform);
	host.register(buildSketchpadExtensionManifest(), {
		id: SKETCHPAD_EXTENSION_ID,
		activate() {},
	} satisfies PluginModule);
	await host.activateAll((controllerId) => (controllerId === SKETCHPAD_SHELL_CONTROLLER_ID ? controller : undefined));
	new SketchpadPlatformComponents(platform);
	platform.applyUri = (uri) => applySketchpadUri(platform, uri);
	platform.breadcrumb = (uri) => sketchpadBreadcrumb(platform, uri);
	if (typeof window === "undefined") {
		platform.activeAppId = SKETCHPAD_HOME_APP_ID;
		platform.notify();
	}
	sketchpadPlatformSingleton = platform;
	sketchpadPluginHostSingleton = host;
	if (typeof window !== "undefined") {
		sketchpadInstallHomeDropzone();
	}
	if (typeof import.meta !== "undefined" && (import.meta as { env?: { DEV?: boolean } }).env?.DEV) {
		void seedSketchpadDevFixtureKitIfEmpty();
	}
	return platform;
}

/** @emoji 🚀 Ensures the sketchpad {@link Platform} is initialized once per session. */
export async function ensureSketchpadPlatform(): Promise<Platform> {
	if (sketchpadPlatformSingleton) return sketchpadPlatformSingleton;
	if (!sketchpadPlatformReady) {
		sketchpadPlatformReady = buildSketchpadPlatform();
	}
	return sketchpadPlatformReady;
}

/** @emoji 🔍 Returns the live sketchpad {@link Platform}, if built. */
export function getSketchpadPlatform(): Platform | null {
	return sketchpadPlatformSingleton;
}

/** @emoji 🚀 @deprecated Use {@link ensureSketchpadPlatform}. */
export const ensureSketchpadDeclarativeShell = ensureSketchpadPlatform;

/** @emoji 🔍 @deprecated Use {@link getSketchpadPlatform}. */
export const getSketchpadProductRuntime = getSketchpadPlatform;

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("SemioKitStore", () => {
		it("InMemorySemioKitStore exposes kit snapshot", () => {
			const store = new InMemorySemioKitStore({ id: "k1", name: "Demo" } as Kit);
			expect(store.getSnapshot().kit.name).toBe("Demo");
		});
	});

	describe("SketchpadShellController stores", () => {
		it("provideStore registers shell and kit stores", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitStore = new InMemorySemioKitStore({ id: "k1", name: "A" } as Kit);
			ctrl.registerKitStore("k1", kitStore, { kind: "temporary" });
			expect(ctrl.getStore(SKETCHPAD_SHELL_STORE_SHELL)?.getSnapshot().openKitIds).toEqual(["k1"]);
			expect(ctrl.routeSelection.pieceIds).toEqual([]);
			expect(ctrl.getKitStore("k1")?.getSnapshot().kit.name).toBe("A");
			expect(ctrl.getKitPersistenceKind("k1")).toBe("temporary");
			ctrl.dispose();
		});
	});

	describe("decodeKitSemioEnvelopeToFullFromValue", () => {
		it("unwraps wip.initialKit envelope", () => {
			const inner = decodeKitSemioEnvelopeToFullFromValue({ wip: { initialKit: { id: "k", name: "N" } } });
			expect((inner as { id: string }).id).toBe("k");
		});
	});

	describe("sketchpadKitFromDecodedBundle", () => {
		it("reads metabolism.kit.light.semio.json fixture file", async () => {
			const { readFileSync } = await import("node:fs");
			const { dirname, join } = await import("node:path");
			const { fileURLToPath } = await import("node:url");
			const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../../fixtures/metabolism.kit.light.semio.json");
			const kit = sketchpadKitFromDecodedBundle(JSON.parse(readFileSync(fixturePath, "utf8")));
			expect(kit?.name).toBe("Metabolism");
			expect(sketchpadExtractPortCompatById(kit!).size).toBeGreaterThan(0);
			expect(sketchpadCollectKitPorts(kit!).length).toBeGreaterThan(0);
			expect(sketchpadReadKitFamilyRows(kit!).some((f) => f["name"] === "Nakagin Capsule Tower")).toBe(true);
		});

		it("reads metabolism-shaped wip.initialKit bundle", () => {
			const raw = {
				schema: "test",
				wip: {
					initialKit: {
						id: "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc",
						name: "Metabolism",
						types: { items: [{ id: "t1", name: "Base" }] },
						designs: { items: [] },
						families: {
							items: [
								{
									id: "fam-nakagin",
									name: "Nakagin Capsule Tower",
									ports: {
										items: [
											{ id: "p1", name: "bottom", compatiblePorts: { items: [{ id: "p2" }] } },
											{ id: "p2", name: "top", compatiblePorts: { items: [{ id: "p1" }] } },
										],
									},
								},
							],
						},
					},
				},
			};
			const kit = sketchpadKitFromDecodedBundle(raw);
			expect(kit?.name).toBe("Metabolism");
			expect(sketchpadExtractPortCompatById(kit!).size).toBe(2);
		});
	});

	describe("sketchpadAppIdFromPath", () => {
		it("resolves design app from kit route", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			expect(sketchpadAppIdFromPath(`/kits/${kitId}/designs/${designId}`)).toBe(SKETCHPAD_DESIGN_APP_ID);
		});
	});

	describe("SketchpadShellController navigation", () => {
		it("closeKit removes store and open id", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			ctrl.registerKitStore("k1", new InMemorySemioKitStore({ id: "k1", name: "A" } as Kit));
			ctrl.closeKit("k1");
			expect(ctrl.listOpenKitIds()).toEqual([]);
			expect(ctrl.getKitStore("k1")).toBeUndefined();
			ctrl.dispose();
		});

		it("createTemporaryKit registers navigable kit", async () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const id = await ctrl.createTemporaryKit("Test");
			expect(ctrl.listOpenKitIds()).toContain(id);
			expect(ctrl.getKitStore(id)?.getSnapshot().kit.name).toBe("Test");
			expect(ctrl.navigationPath).toBe(`/kits/${id}`);
			expect(ctrl.getKitStore(id)).toBeInstanceOf(SemioJsKitStore);
			ctrl.dispose();
		});
	});

	describe("importKit", () => {
		it("hydrates family ports into live kit and diagram compat edges", async () => {
			const payload = JSON.stringify({
				id: "kit-import-test",
				name: "Import Test",
				families: [
					{
						id: "fam1",
						name: "Tower",
						ports: [
							{ id: "p1", name: "bottom", compatiblePorts: [{ id: "p2" }] },
							{ id: "p2", name: "top", compatiblePorts: [{ id: "p1" }] },
						],
					},
				],
				types: [
					{ id: "t1", name: "A", connectors: [{ id: "c1", name: "c1", port: { id: "p1" } }] },
					{ id: "t2", name: "B", connectors: [{ id: "c2", name: "c2", port: { id: "p2" } }] },
				],
				designs: [],
			});
			const { kit, session } = await importKit(new TextEncoder().encode(payload));
			try {
				expect(kit.name).toBe("Import Test");
				expect(Array.isArray(kit.types)).toBe(true);
				expect(sketchpadExtractPortCompatById(kit).size).toBe(2);
				const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
				expect(fixture.edges.some((edge) => edge.id === "compat-type:t1-type:t2")).toBe(true);
			} finally {
				await session.dispose();
			}
		}, 120_000);
	});

	describe("executeSketchpadJsKitMutation", () => {
		it("createDesign updates kit snapshot", async () => {
			const session = await SemioSession.openInMemory({ timeoutMs: 120_000 });
			try {
				const jsStore = (await session.stores())[0]!;
				const store = await createSemioKitStoreFromJsStore(jsStore);
				const bus = new CommandBus();
				const ctrl = new SketchpadShellController(bus, () => {});
				const kitId = store.getSnapshot().kit.id;
				ctrl.registerKitStore(kitId, store);
				const created = await executeSketchpadJsKitMutation(kitId, (kit) => kit.createDesign("Layout A"), store);
				expect(created.ok).toBe(true);
				expect(store.getSnapshot().kit.designs?.some((d) => d.name === "Layout A")).toBe(true);
				ctrl.dispose();
			} finally {
				await session.dispose();
			}
		});
	});

	describe("findDesignInKit", () => {
		it("returns design by id", () => {
			const kit = { id: "k", designs: [{ id: "d1", name: "D" }] } as Kit;
			expect(findDesignInKit(kit, "d1")?.name).toBe("D");
		});
	});

	describe("parseSketchpadPuzzleInstanceId", () => {
		it("parses kit diagram and design panes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			expect(parseSketchpadPuzzleInstanceId(sketchpadKitDiagramInstanceId(kitId))).toEqual({
				kitId,
				designId: null,
				typeId: null,
				pane: "kit-diagram",
			});
			expect(parseSketchpadPuzzleInstanceId(sketchpadDesignSceneInstanceId(kitId, designId))).toEqual({
				kitId,
				designId,
				typeId: null,
				pane: "scene",
			});
			const typeId = "22222222-3333-4444-5555-666666666666";
			expect(parseSketchpadPuzzleInstanceId(sketchpadTypeSceneInstanceId(kitId, typeId))).toEqual({
				kitId,
				designId: null,
				typeId,
				pane: "type-scene",
			});
		});
	});

	describe("sketchpadTypeVolumeFixtureFromType", () => {
		it("places one mesh object at the origin", () => {
			const kit = {
				id: "k",
				types: [{ id: "t1", name: "Chair", representations: [{ file: { id: "f1" } }] }],
				files: [{ id: "f1", path: "files/chair.glb" }],
			} as Kit;
			const volume = sketchpadTypeVolumeFixtureFromType(kit.types![0]!, kit);
			expect(volume.objects).toHaveLength(1);
			expect(volume.objects[0]?.id).toBe("t1");
			expect(volume.objects[0]?.meshUrl).toContain("chair.glb");
		});
	});

	describe("sketchpadPortDtoFromGraphqlNode", () => {
		it("maps copatibleWith edges to compatiblePorts", () => {
			const port = sketchpadPortDtoFromGraphqlNode({
				id: "p1",
				copatibleWith: { edges: [{ node: { id: "p2" } }] },
			});
			expect(sketchpadReadCompatiblePortIds(port)).toEqual(["p2"]);
		});
	});

	describe("sketchpadMergePortCompatMaps", () => {
		it("overlays graphql compat onto bundle-derived compat", () => {
			const base = new Map<string, readonly { readonly id: string }[]>([["p1", [{ id: "p-old" }]]]);
			const overlay = new Map<string, readonly { readonly id: string }[]>([["p1", [{ id: "p-new" }]]]);
			const merged = sketchpadMergePortCompatMaps(base, overlay);
			expect(merged.get("p1")).toEqual([{ id: "p-new" }]);
		});
	});

	describe("sketchpadApplyPortCompatById", () => {
		it("restores compatiblePorts stripped by GraphQL-shaped reads", () => {
			const bundle = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", compatiblePorts: [{ id: "p2" }] } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", compatiblePorts: [{ id: "p1" }] } }] },
				],
			} as Kit;
			const graphqlKit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", label: "A" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", label: "B" } }] },
				],
			} as Kit;
			const compat = sketchpadExtractPortCompatById(bundle);
			const merged = sketchpadApplyPortCompatById(graphqlKit, compat);
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(merged);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});

		it("reads port compat from kit families and wires type adjacency via connectors", () => {
			const bundle = {
				id: "k",
				families: [
					{
						id: "fam1",
						name: "Tower",
						ports: [
							{ id: "p1", name: "core bottom", compatiblePorts: [{ id: "p2" }] },
							{ id: "p2", name: "core top", compatiblePorts: [{ id: "p1" }] },
						],
					},
				],
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2" } }] },
				],
			} as Kit;
			const compat = sketchpadExtractPortCompatById(bundle);
			expect(compat.size).toBe(2);
			const graphqlKit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1" } }] },
					{ id: "t2", connectors: [{ port: { id: "p2" } }] },
				],
			} as Kit;
			const merged = sketchpadApplyPortCompatById(graphqlKit, compat);
			expect(sketchpadCollectKitPorts(merged).map((p) => p.id).sort()).toEqual(["p1", "p2"]);
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(merged);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});
	});

	describe("sketchpadReadKitFamilyRows", () => {
		it("accepts denormalized arrays and block items", () => {
			const fromArray = sketchpadReadKitFamilyRows({ id: "k", families: [{ id: "f1" }] } as Kit);
			expect(fromArray).toHaveLength(1);
			const fromBlock = sketchpadReadKitFamilyRows({
				id: "k",
				families: { items: [{ id: "f2" }] },
			} as Kit);
			expect(fromBlock[0]?.["id"]).toBe("f2");
		});
	});

	describe("sketchpadEnsureHomeKitFileInput", () => {
		it("creates a hidden file input once", () => {
			if (typeof document === "undefined") return;
			sketchpadInstallHomeDropzone();
			const input = document.getElementById(SKETCHPAD_HOME_KIT_FILE_INPUT_ID);
			expect(input?.getAttribute("type")).toBe("file");
		});
	});

	describe("sketchpadSetHomeDropzoneOverlayVisible", () => {
		it("creates and toggles the overlay element", () => {
			if (typeof document === "undefined") return;
			sketchpadSetHomeDropzoneOverlayVisible(true);
			const overlay = document.getElementById(SKETCHPAD_HOME_DROPZONE_OVERLAY_ID);
			expect(overlay).toBeTruthy();
			expect(overlay?.classList.contains("hidden")).toBe(false);
			sketchpadSetHomeDropzoneOverlayVisible(false);
			expect(overlay?.classList.contains("hidden")).toBe(true);
		});
	});

	describe("parseSketchpadRouteSelectionQuery", () => {
		it("reads piece connection and diagram ids from query params", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const selection = parseSketchpadRouteSelectionQuery(
				`/kits/${kitId}/designs/${designId}?piece=p1&piece=p2&conn=c1&diag=type:t1`,
			);
			expect(selection.pieceIds).toEqual(["p1", "p2"]);
			expect(selection.connectionIds).toEqual(["c1"]);
			expect(selection.kitDiagramNodeIds).toEqual(["type:t1"]);
		});
	});

	describe("sketchpadRouteSelectionUriFilters", () => {
		it("round-trips selection through query serialization", () => {
			const selection = { pieceIds: ["a"], connectionIds: ["b"], kitDiagramNodeIds: ["type:x"] };
			const uri = `/kits/k/designs/d${sketchpadRouteSelectionUriFilters(selection)}`;
			expect(parseSketchpadRouteSelectionQuery(uri)).toEqual(selection);
		});
	});

	describe("SketchpadShellController route selection URL", () => {
		it("syncs navigation path when selection changes on a design route", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/designs/${designId}`);
			ctrl.setRouteSelection({ pieceIds: ["piece-a"], connectionIds: [], kitDiagramNodeIds: [] });
			expect(ctrl.navigationPath).toBe(`/kits/${kitId}/designs/${designId}?piece=piece-a`);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a"]);
			ctrl.dispose();
		});
	});

	describe("parseSketchpadHomeQuery", () => {
		it("reads kind and search filters from the URI", () => {
			const home = parseSketchpadHomeQuery("/?kind=file&q=metab&e=docs-root&sel=k1&sort=updated&dir=desc");
			expect(home.kindFilter).toBe("file");
			expect(home.searchQuery).toBe("metab");
			expect(home.expandedRowIds).toEqual(["docs-root"]);
			expect(home.selectedKitIds).toEqual(["k1"]);
			expect(home.sortColumnId).toBe("updated");
			expect(home.sortDescending).toBe(true);
		});
	});

	describe("sketchpadKitToSemioEnvelope", () => {
		it("wraps kit in wip.initialKit", () => {
			const envelope = sketchpadKitToSemioEnvelope({ id: "k1", name: "Demo" } as Kit);
			expect((envelope.wip.initialKit as Kit).name).toBe("Demo");
		});
	});

	describe("sketchpadResolveMdxModuleKey", () => {
		it("resolves index and leaf docs paths", () => {
			expect(sketchpadResolveMdxModuleKey("getting-started/index")).toMatch(/getting-started\/index\.mdx$/);
			expect(sketchpadResolveMdxModuleKey("getting-started/installation")).toMatch(/installation\.mdx$/);
		});
	});

	describe("sketchpadBuildHomeTableModel", () => {
		it("includes documentation root and grouped kit rows", () => {
			const model = sketchpadBuildHomeTableModel({
				openKitIds: ["k1", "k2"],
				kitById: (id) =>
					id === "k1"
						? ({ id: "k1", name: "Alpha", version: "r1" } as Kit)
						: ({ id: "k2", name: "Alpha", version: "r2" } as Kit),
				kitKind: () => "temporary",
				home: {
					...sketchpadEmptyHomeUiState(),
					expandedRowIds: ["docs-root", "docs-section-intro", "kit-group-Alpha"],
				},
				docs: [{ id: "intro", label: "Intro", pages: [{ path: "intro/index", title: "Overview" }] }],
			});
			expect(model.rows.some((row) => row.id === "docs-root")).toBe(true);
			expect(model.rows.some((row) => row.id === "docs-page-intro/index")).toBe(true);
			expect(model.rows.some((row) => row.id === "k2")).toBe(true);
		});
	});

	describe("SketchpadHomeTable snapshot", () => {
		it("lists open kits with version and kind columns", async () => {
			const platform = await buildSketchpadPlatform();
			const ctrl = getSketchpadShellController()!;
			ctrl.registerKitStore(
				"k-home",
				new InMemorySemioKitStore({ id: "k-home", name: "Demo Kit", version: "r1", updatedAt: "2025-06-01T12:00:00.000Z" } as Kit),
				{ kind: "fixture" },
			);
			const table = new SketchpadHomeTable(platform);
			const snap = table.buildSnapshot();
			expect(snap.columns?.map((column) => column.id)).toEqual(expect.arrayContaining(["name", "version", "kind", "updated"]));
			expect(snap.rows.some((row) => row.id === "kit-group-Demo Kit")).toBe(true);
			expect(snap.rows.find((row) => row.id === "kit-group-Demo Kit")?.cells.version).toBe("r1");
			expect(snap.rows.find((row) => row.id === "kit-group-Demo Kit")?.cells.kind).toBe("fixture");
			ctrl.dispose();
		});
	});

	describe("sketchpadKitPuzzle2dFixtureFromKit", () => {
		it("materializes type and design nodes", () => {
			const kit = {
				id: "k",
				types: [{ id: "t1", name: "Window" }],
				designs: [{ id: "d1", name: "Plan", pieces: [{ id: "p1", type: { id: "t1" } }] }],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.nodes.some((n) => n.id === "type:t1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "design:d1")).toBe(true);
			expect(fixture.edges.length).toBeGreaterThan(0);
		});

		it("materializes ports qualities files folders authors", () => {
			const kit = {
				id: "k",
				types: [
					{
						id: "t1",
						name: "Window",
						connectors: [{ id: "c1", port: { id: "p1", label: "Frame" } }],
						ports: [{ id: "p2", label: "Glass" }],
					},
				],
				qualities: [{ id: "q1", key: "Thermal", value: "1.2" }],
				files: [{ id: "f1", url: "files/mesh.glb" }],
				folders: [{ id: "fo1", path: "assets/models" }],
				authors: [{ id: "a1", name: "Ada" }],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.nodes.some((n) => n.id === "port:p1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "port:p2")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "quality:q1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "file:f1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "folder:fo1")).toBe(true);
			expect(fixture.nodes.some((n) => n.id === "author:a1")).toBe(true);
			expect(fixture.edges.some((e) => e.id === "ref-port:p1-type:t1")).toBe(true);
		});
	});

	describe("sketchpadCreatePortGroupMap", () => {
		it("unions ports linked by compatiblePorts", () => {
			const groups = sketchpadCreatePortGroupMap([
				{ id: "p1", compatiblePorts: [{ id: "p2" }] },
				{ id: "p2", compatiblePorts: [{ id: "p1" }] },
				{ id: "p3" },
			]);
			expect(groups.get("p1")).toBe(groups.get("p2"));
			expect(groups.get("p3")).toBe("p3");
		});
	});

	describe("sketchpadKitPuzzle2dFixtureFromKit type compat", () => {
		it("draws type adjacency edges for compatible ports", () => {
			const kit = {
				id: "k",
				types: [
					{ id: "t1", connectors: [{ port: { id: "p1", compatiblePorts: [{ id: "p2" }] } }] },
					{ id: "t2", connectors: [{ port: { id: "p2", compatiblePorts: [{ id: "p1" }] } }] },
				],
			} as Kit;
			const fixture = sketchpadKitPuzzle2dFixtureFromKit(kit);
			expect(fixture.edges.some((e) => e.id === "compat-type:t1-type:t2")).toBe(true);
		});
	});

	describe("sketchpadCollectKitPorts", () => {
		it("deduplicates ports from connectors and type ports", () => {
			const kit = {
				id: "k",
				types: [
					{
						id: "t1",
						connectors: [{ port: { id: "p1", label: "A" } }],
						ports: [{ id: "p1", label: "A" }, { id: "p2", code: "B" }],
					},
				],
			} as Kit;
			const ports = sketchpadCollectKitPorts(kit);
			expect(ports).toHaveLength(2);
			expect(ports.map((p) => p.id).sort()).toEqual(["p1", "p2"]);
		});
	});

	describe("sketchpadDesignVolumeFixtureFromDesign", () => {
		it("creates placeholder mesh objects per piece", () => {
			const design = {
				id: "d",
				pieces: [{ id: "p1", name: "A", plane: { origin: { x: 1, y: 2, z: 3 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } } }],
			} as Design;
			const volume = sketchpadDesignVolumeFixtureFromDesign(design);
			expect(volume.objects).toHaveLength(1);
			expect(volume.objects[0]?.origin).toEqual([1, 2, 3]);
		});
	});

	describe("sketchpadKitFileUrlById", () => {
		it("resolves metabolism-relative file paths", () => {
			const kit = {
				id: "k",
				files: [{ id: "f1", path: "files/mesh.glb" }],
			} as Kit;
			expect(sketchpadKitFileUrlById(kit).get("f1")).toBe("/assets/semio/metabolism/wip/initialKit/files/mesh.glb");
		});
	});

	describe("sketchpadApplyPuzzle2dSelection", () => {
		it("stores design piece selection on shell", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/designs/${designId}`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignDiagramInstanceId(kitId, designId), ["piece-a", "piece-b"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a", "piece-b"]);
			ctrl.dispose();
		});

		it("stores design piece selection from scene volume object ids", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			ctrl.navigateTo(`/kits/${kitId}/designs/${designId}/scene`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignSceneInstanceId(kitId, designId), ["piece-x"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-x"]);
			ctrl.dispose();
		});

		it("maps volume attraction ids to connection selection", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			const designId = "11111111-2222-3333-4444-555555555555";
			const kit = {
				id: kitId,
				name: "K",
				designs: [
					{
						id: designId,
						name: "D",
						pieces: [{ id: "piece-a", name: "A" }],
						connections: [{ id: "conn-1", parent: { piece: { id: "piece-a" }, connector: { id: "c1" } }, child: { piece: { id: "piece-a" }, connector: { id: "c2" } } }],
					},
				],
			} as Kit;
			ctrl.registerKitStore(kitId, new InMemorySemioKitStore(kit));
			ctrl.navigateTo(`/kits/${kitId}/designs/${designId}`);
			sketchpadApplyPuzzle2dSelection(sketchpadDesignSceneInstanceId(kitId, designId), ["piece-a", "conn-1"], ctrl);
			expect(ctrl.routeSelection.pieceIds).toEqual(["piece-a"]);
			expect(ctrl.routeSelection.connectionIds).toEqual(["conn-1"]);
			ctrl.dispose();
		});

		it("stores multi-select on kit diagram without navigating", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			ctrl.registerKitStore(kitId, new InMemorySemioKitStore({ id: kitId, name: "K", types: [], designs: [] } as Kit));
			ctrl.navigateTo(`/kits/${kitId}`);
			sketchpadApplyPuzzle2dSelection(sketchpadKitDiagramInstanceId(kitId), ["type:a", "design:b"], ctrl);
			expect(ctrl.routeSelection.kitDiagramNodeIds).toEqual(["type:a", "design:b"]);
			expect(ctrl.navigationPath).toBe(
				`/kits/${kitId}${sketchpadRouteSelectionUriFilters({ pieceIds: [], connectionIds: [], kitDiagramNodeIds: ["type:a", "design:b"] })}`,
			);
			ctrl.dispose();
		});
	});

	describe("sketchpadPathFromDiagramNodeId", () => {
		it("maps kit diagram nodes to routes", () => {
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			expect(sketchpadPathFromDiagramNodeId(kitId, "type:11111111-2222-3333-4444-555555555555")).toBe(
				`/kits/${kitId}/types/11111111-2222-3333-4444-555555555555`,
			);
		});
	});

	describe("SketchpadShellController topology", () => {
		it("upserts topology store for kit diagram surface", () => {
			const bus = new CommandBus();
			const ctrl = new SketchpadShellController(bus, () => {});
			const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
			ctrl.registerKitStore(
				kitId,
				new InMemorySemioKitStore({
					id: kitId,
					name: "K",
					types: [{ id: "t1", name: "T" }],
					designs: [],
				} as Kit),
			);
			ctrl.syncTopologyForSurface(SKETCHPAD_SURFACE_KIT_DIAGRAM, { kitId, designId: null, typeId: null });
			const topo = ctrl.getStore(platformTopologyStoreId(sketchpadKitDiagramInstanceId(kitId))) as PlatformTopologyStore;
			expect(topo).toBeDefined();
			expect(topo!.getSnapshot().flat.schema).toBe("puzzle.2d.fixture/v1");
			ctrl.dispose();
		});
	});
}
//#endregion 🧪Tests

//#region 🧪E2E
if (typeof __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__ !== "undefined" && __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__) {
	const { test, expect } = await import("@playwright/test");
	test.describe("sketchpad platform", () => {
		test("home table mounts on root", async ({ page }) => {
			await page.goto("/");
			await expect(page.getByText(/No kits open/)).toBeVisible({ timeout: 120_000 });
		});

		test("workbench panel is present when platform loads", async ({ page }) => {
			await page.goto("/");
			await expect(page.getByTestId("app-panel.workbench")).toBeVisible({ timeout: 120_000 });
		});
	});
}
//#endregion 🧪E2E
