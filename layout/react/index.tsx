/** @emoji 📄 Layout React — WebGPU canvas bindings for the layout engine. */

import { GraphWasmCanvas, type GraphWasmSession, React } from "@semio-tech/infinite-cavas-react-renderer";
import init, { LayoutSession } from "@semio-tech/layout-rs";

export type LayoutChromeMode = "blueprint" | "preview";

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
			if (this.pendingDocumentJson !== null) session.setDocumentJson(this.pendingDocumentJson);
			if (this.pendingPageId !== null) session.setPageId(this.pendingPageId);
			if (this.pendingSelectedIds !== null) session.setSelectedIdsJson(JSON.stringify(this.pendingSelectedIds));
			if (this.pendingHoveredId !== undefined) session.setHoveredId(this.pendingHoveredId);
			this.session = session;
		}
		return this.session;
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
		if (this.session) this.session.setDocumentJson(json);
		else this.pendingDocumentJson = json;
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

	pointerDown(x: number, y: number, extend: boolean): void {
		const hit = this.hitTest(x, y);
		if (!extend || !hit) {
			this.onHitCallback?.(hit);
		}
	}

	pointerMove(x: number, y: number): void {
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

	pointerUp(): void {}

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

export interface LayoutCanvasProps {
	readonly className?: string;
	readonly chromeMode?: LayoutChromeMode;
	readonly documentJson: string;
	readonly pageId: string;
	readonly selectedIds?: readonly string[];
	readonly hoveredId?: string | null;
	readonly onHit?: (objectId: string | null) => void;
	readonly onHover?: (objectId: string | null) => void;
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
}: LayoutCanvasProps): React.JSX.Element {
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
		<GraphWasmCanvas
			className={className}
			enablePointer={chromeMode === "blueprint"}
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
	});
}
