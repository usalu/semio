/** @emoji 📄 Layout React — WebGPU canvas bindings for the layout engine. */

import {
	LAYOUT_CATALOGUE_KIND_DRAG_MIME,
	parseLayoutDocumentJson,
	type LayoutCatalogueKind,
} from "@semio-tech/layout-core";
import { GraphWasmCanvas, type GraphWasmSession, React } from "@semio-tech/infinite-cavas-react-renderer";
import init, { LayoutSession } from "@semio-tech/layout-rs";
import { type TreeDragAndDropController } from "@semio-tech/ui-react";

export type LayoutChromeMode = "blueprint" | "preview";

export const LAYOUT_CATALOGUE_DRAG_SESSION_EVENT = "layout-catalogue-drag-session";

export const layoutCatalogueDragSessionRef = { active: false, kind: null as LayoutCatalogueKind | null };

/** @emoji 🖱️ Begins a pointer-driven catalogue drag session. */
export function beginLayoutCatalogueDrag(kind: LayoutCatalogueKind): void {
	layoutCatalogueDragSessionRef.active = true;
	layoutCatalogueDragSessionRef.kind = kind;
	globalThis.dispatchEvent?.(new CustomEvent(LAYOUT_CATALOGUE_DRAG_SESSION_EVENT, { detail: { kind } }));
}

/** @emoji 🖱️ Ends the active catalogue drag session. */
export function endLayoutCatalogueDrag(): void {
	layoutCatalogueDragSessionRef.active = false;
	layoutCatalogueDragSessionRef.kind = null;
	globalThis.dispatchEvent?.(new CustomEvent(LAYOUT_CATALOGUE_DRAG_SESSION_EVENT, { detail: null }));
}

/** @emoji 🖱️ {@link TreeDragAndDropController} for catalogue rows carrying layout palette drag payloads. */
export function createLayoutPlayCatalogueTreeDragController(): TreeDragAndDropController {
	const readEncoded = (dragData: Record<string, string> | undefined): string | undefined => {
		const payload = dragData?.[LAYOUT_CATALOGUE_KIND_DRAG_MIME];
		return payload?.trim() ? payload : undefined;
	};
	return {
		pointerPaletteDrag: {
			readEncodedDragPayload: readEncoded,
			begin: (encoded) => {
				const parsed = JSON.parse(encoded) as { kind?: LayoutCatalogueKind };
				if (parsed.kind) beginLayoutCatalogueDrag(parsed.kind);
			},
			cancel: endLayoutCatalogueDrag,
		},
		onDragEnd: endLayoutCatalogueDrag,
	};
}

let wasmReady: Promise<void> | null = null;

export async function ensureLayoutWasm(): Promise<void> {
	if (!wasmReady) {
		wasmReady = init().then(() => undefined);
	}
	await wasmReady;
}

export class LayoutEngineSession implements GraphWasmSession {
	private session: LayoutSession | null = null;
	private pendingDocumentJson: string | null = null;
	private pendingPageId: string | null = null;
	private pendingSelectedIds: readonly string[] | null = null;
	private pendingHoveredId: string | null | undefined;
	private readonly chromeBlueprint: boolean;
	private onHitCallback: ((objectId: string | null) => void) | null = null;
	private onHoverCallback: ((objectId: string | null) => void) | null = null;
	private lastHoverId: string | null | undefined = undefined;
	private cameraSeeded = false;
	private isPanning = false;

	constructor(chromeMode: LayoutChromeMode = "blueprint", onHit?: (objectId: string | null) => void, onHover?: (objectId: string | null) => void) {
		this.chromeBlueprint = chromeMode === "blueprint";
		this.onHitCallback = onHit ?? null;
		this.onHoverCallback = onHover ?? null;
	}

	async ensureReady(): Promise<void> {
		await this.ensureSession();
	}

	private async ensureSession(): Promise<LayoutSession> {
		await ensureLayoutWasm();
		if (!this.session) {
			const session = new LayoutSession();
			session.setChromeMode(this.chromeBlueprint);
			if (this.pendingDocumentJson !== null) {
				session.setDocumentJson(this.pendingDocumentJson);
				this.seedCameraFromDocumentJson(this.pendingDocumentJson, session);
			}
			if (this.pendingPageId !== null) session.setPageId(this.pendingPageId);
			if (this.pendingSelectedIds !== null) session.setSelectedIdsJson(JSON.stringify(this.pendingSelectedIds));
			if (this.pendingHoveredId !== undefined) session.setHoveredId(this.pendingHoveredId);
			this.session = session;
		}
		return this.session;
	}

	private seedCameraFromDocumentJson(json: string, session: LayoutSession): void {
		if (this.cameraSeeded) return;
		const doc = parseLayoutDocumentJson(json);
		if (!doc) return;
		const cam = this.chromeBlueprint ? doc.camera : doc.previewCamera;
		session.setCamera(cam.x, cam.y, cam.zoom);
		this.cameraSeeded = true;
	}

	async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
		const session = await this.ensureSession();
		return session.attachCanvas(canvas, logicalW, logicalH, dpr);
	}

	setSize(width: number, height: number, dpr: number): void {
		this.session?.setSize(width, height, dpr);
	}

	renderFrame(): void {
		this.session?.renderFrame();
	}

	setDocumentJson(json: string): void {
		if (this.session) {
			this.session.setDocumentJson(json);
		} else {
			this.pendingDocumentJson = json;
		}
	}

	setPageId(pageId: string): void {
		if (this.session) this.session.setPageId(pageId);
		else this.pendingPageId = pageId;
	}

	setSelectedIds(ids: readonly string[]): void {
		if (this.session) this.session.setSelectedIdsJson(JSON.stringify(ids));
		else this.pendingSelectedIds = ids;
	}

	setHoveredId(id: string | null): void {
		if (this.session) this.session.setHoveredId(id);
		else this.pendingHoveredId = id;
	}

	setDropPreview(kind: LayoutCatalogueKind, worldX: number, worldY: number): void {
		this.session?.setDropPreview(kind, worldX, worldY);
		this.session?.renderFrame();
	}

	clearDropPreview(): void {
		this.session?.clearDropPreview();
		this.session?.renderFrame();
	}

	screenToWorld(x: number, y: number): { readonly x: number; readonly y: number } | null {
		if (!this.session) return null;
		const json = this.session.screenToWorld(x, y);
		const parsed = JSON.parse(json) as { x?: number; y?: number };
		if (typeof parsed.x !== "number" || typeof parsed.y !== "number") return null;
		return { x: parsed.x, y: parsed.y };
	}

	pointerDown(x: number, y: number, button: number, extend: boolean): void {
		if (button === 1) {
			this.isPanning = true;
			this.session?.pointerDownScreen(x, y, 1);
			return;
		}
		if (button !== 0 || !this.chromeBlueprint) return;
		const hit = this.hitTest(x, y);
		if (!extend || !hit) {
			this.onHitCallback?.(hit);
		}
	}

	pointerMove(x: number, y: number): void {
		if (this.isPanning) {
			this.session?.pointerMoveScreen(x, y);
			return;
		}
		if (!this.chromeBlueprint) return;
		const hit = this.hitTest(x, y);
		if (this.lastHoverId === undefined) {
			this.lastHoverId = hit;
			this.onHoverCallback?.(hit);
			return;
		}
		if (hit === this.lastHoverId) return;
		this.lastHoverId = hit;
		this.onHoverCallback?.(hit);
	}

	pointerUp(x: number, y: number): void {
		if (this.isPanning) {
			this.session?.pointerUpScreen(x, y);
			this.isPanning = false;
		}
	}

	wheel(x: number, y: number, deltaY: number): void {
		this.session?.wheelScreen(x, y, deltaY);
	}

	hitTest(x: number, y: number): string | null {
		const hit = this.session?.hitTest(x, y);
		return typeof hit === "string" ? hit : null;
	}

	exportPng(pageId: string): Uint8Array {
		return this.session!.exportPng(pageId);
	}

	exportSvg(pageId: string): string {
		return this.session!.exportSvg(pageId);
	}

	exportPdf(pageId: string): Uint8Array {
		return this.session!.exportPdf(pageId);
	}

	exportPackage(preflightJson: string): Uint8Array {
		return this.session!.exportPackage(preflightJson);
	}
}

function LayoutCatalogueDropBridge({
	containerRef,
	sessionRef,
	enabled,
	onCatalogueDrop,
}: {
	readonly containerRef: React.RefObject<HTMLDivElement | null>;
	readonly sessionRef: React.RefObject<LayoutEngineSession | null>;
	readonly enabled: boolean;
	readonly onCatalogueDrop?: (kind: LayoutCatalogueKind, worldX: number, worldY: number) => void;
}): null {
	React.useEffect(() => {
		if (!enabled) return;

		const isOverHost = (clientX: number, clientY: number): boolean => {
			const container = containerRef.current;
			if (!container) return false;
			const rect = container.getBoundingClientRect();
			return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
		};

		const localFromClient = (clientX: number, clientY: number): { readonly x: number; readonly y: number } | null => {
			const container = containerRef.current;
			if (!container) return null;
			const rect = container.getBoundingClientRect();
			return { x: clientX - rect.left, y: clientY - rect.top };
		};

		const onPointerMove = (event: PointerEvent): void => {
			if (!layoutCatalogueDragSessionRef.active || !layoutCatalogueDragSessionRef.kind) return;
			const session = sessionRef.current;
			if (!session) return;
			if (!isOverHost(event.clientX, event.clientY)) {
				session.clearDropPreview();
				return;
			}
			const local = localFromClient(event.clientX, event.clientY);
			if (!local) return;
			const world = session.screenToWorld(local.x, local.y);
			if (!world) return;
			session.setDropPreview(layoutCatalogueDragSessionRef.kind, world.x, world.y);
		};

		const onPointerUp = (event: PointerEvent): void => {
			if (!layoutCatalogueDragSessionRef.active || !layoutCatalogueDragSessionRef.kind) return;
			const kind = layoutCatalogueDragSessionRef.kind;
			const session = sessionRef.current;
			session?.clearDropPreview();
			if (!isOverHost(event.clientX, event.clientY) || !onCatalogueDrop) return;
			const local = localFromClient(event.clientX, event.clientY);
			if (!local) return;
			const world = session?.screenToWorld(local.x, local.y);
			if (!world) return;
			onCatalogueDrop(kind, world.x, world.y);
		};

		globalThis.addEventListener?.("pointermove", onPointerMove);
		globalThis.addEventListener?.("pointerup", onPointerUp, true);
		return () => {
			globalThis.removeEventListener?.("pointermove", onPointerMove);
			globalThis.removeEventListener?.("pointerup", onPointerUp, true);
		};
	}, [containerRef, enabled, onCatalogueDrop, sessionRef]);

	return null;
}

export interface LayoutCanvasProps {
	readonly className?: string;
	readonly chromeMode?: LayoutChromeMode;
	readonly documentJson: string;
	readonly pageId: string;
	readonly selectedIds?: readonly string[];
	readonly hoveredId?: string | null;
	readonly onHit?: (objectId: string | null) => void;
	readonly onHover?: (objectId: string | null) => void;
	readonly onCatalogueDrop?: (kind: LayoutCatalogueKind, worldX: number, worldY: number) => void;
}

export function LayoutCanvas({
	className,
	chromeMode = "blueprint",
	documentJson,
	pageId,
	selectedIds = [],
	hoveredId = null,
	onHit,
	onHover,
	onCatalogueDrop,
}: LayoutCanvasProps): React.JSX.Element {
	const containerRef = React.useRef<HTMLDivElement>(null);
	const sessionRef = React.useRef<LayoutEngineSession | null>(null);

	React.useEffect(() => {
		const session = sessionRef.current;
		if (!session) return;
		session.setDocumentJson(documentJson);
		session.setPageId(pageId);
		session.setSelectedIds(selectedIds);
		session.setHoveredId(hoveredId);
		session.renderFrame();
	}, [documentJson, pageId, selectedIds, hoveredId]);

	return (
		<div ref={containerRef} className={className ?? "relative h-full min-h-0 w-full"}>
			<GraphWasmCanvas
				className="h-full w-full"
				sessionFactory={() => {
					const session = new LayoutEngineSession(chromeMode, onHit, onHover);
					sessionRef.current = session;
					session.setDocumentJson(documentJson);
					session.setPageId(pageId);
					session.setSelectedIds(selectedIds);
					session.setHoveredId(hoveredId);
					return session;
				}}
				onSessionReady={(session) => {
					sessionRef.current = session as LayoutEngineSession;
				}}
			/>
			<LayoutCatalogueDropBridge
				containerRef={containerRef}
				sessionRef={sessionRef}
				enabled={chromeMode === "blueprint"}
				onCatalogueDrop={onCatalogueDrop}
			/>
		</div>
	);
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("LayoutEngineSession", () => {
		it("exposes wasm bootstrap", () => {
			expect(typeof ensureLayoutWasm).toBe("function");
		});
		it("pointerMove invokes onHover when hit changes", () => {
			const hits: Array<string | null> = [];
			const session = new LayoutEngineSession("blueprint", undefined, (id) => hits.push(id));
			session.pointerMove(10, 10);
			expect(hits).toEqual([null]);
		});
		it("does not construct wasm session synchronously", () => {
			expect(() => new LayoutEngineSession("blueprint")).not.toThrow();
		});
	});
	describe("createLayoutPlayCatalogueTreeDragController", () => {
		it("toggles catalogue drag session", () => {
			const controller = createLayoutPlayCatalogueTreeDragController();
			controller.pointerPaletteDrag?.begin(JSON.stringify({ kind: "rect" }));
			expect(layoutCatalogueDragSessionRef.active).toBe(true);
			expect(layoutCatalogueDragSessionRef.kind).toBe("rect");
			controller.onDragEnd?.({ items: [], sourceItem: { id: "x", label: "x" }, section: { id: "s", label: "s" } });
			expect(layoutCatalogueDragSessionRef.active).toBe(false);
		});
	});
}
