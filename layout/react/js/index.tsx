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
	private readonly session: LayoutSession;
	private readonly chromeBlueprint: boolean;
	private onHitCallback: ((objectId: string | null) => void) | null = null;

	constructor(chromeMode: LayoutChromeMode = "blueprint", onHit?: (objectId: string | null) => void) {
		this.session = new LayoutSession();
		this.chromeBlueprint = chromeMode === "blueprint";
		this.session.setChromeMode(this.chromeBlueprint);
		this.onHitCallback = onHit ?? null;
	}

	async attachCanvas(canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number): Promise<unknown> {
		await ensureLayoutWasm();
		return this.session.attachCanvas(canvas, logicalW, logicalH, dpr);
	}

	setSize(width: number, height: number, dpr: number): void {
		this.session.setSize(width, height, dpr);
	}

	renderFrame(): void {
		this.session.renderFrame();
	}

	setDocumentJson(json: string): void {
		this.session.setDocumentJson(json);
	}

	setPageId(pageId: string): void {
		this.session.setPageId(pageId);
	}

	setSelectedIds(ids: readonly string[]): void {
		this.session.setSelectedIdsJson(JSON.stringify(ids));
	}

	pointerDown(x: number, y: number, extend: boolean): void {
		const hit = this.hitTest(x, y);
		if (!extend || !hit) {
			this.onHitCallback?.(hit);
		}
	}

	pointerMove(_x: number, _y: number): void {}

	pointerUp(): void {}

	hitTest(x: number, y: number): string | null {
		const hit = this.session.hitTest(x, y);
		return typeof hit === "string" ? hit : null;
	}

	exportPng(pageId: string): Uint8Array {
		return this.session.exportPng(pageId);
	}

	exportSvg(pageId: string): string {
		return this.session.exportSvg(pageId);
	}

	exportPdf(pageId: string): Uint8Array {
		return this.session.exportPdf(pageId);
	}

	exportPackage(preflightJson: string): Uint8Array {
		return this.session.exportPackage(preflightJson);
	}
}

export interface LayoutCanvasProps {
	readonly className?: string;
	readonly chromeMode?: LayoutChromeMode;
	readonly documentJson: string;
	readonly pageId: string;
	readonly selectedIds?: readonly string[];
	readonly onHit?: (objectId: string | null) => void;
}

export function LayoutCanvas({
	className,
	chromeMode = "blueprint",
	documentJson,
	pageId,
	selectedIds = [],
	onHit,
}: LayoutCanvasProps): React.JSX.Element {
	const sessionRef = React.useRef<LayoutEngineSession | null>(null);

	React.useEffect(() => {
		const session = sessionRef.current;
		if (!session) return;
		session.setDocumentJson(documentJson);
		session.setPageId(pageId);
		session.setSelectedIds(selectedIds);
		session.renderFrame();
	}, [documentJson, pageId, selectedIds]);

	return (
		<GraphWasmCanvas
			className={className}
			enablePointer={chromeMode === "blueprint"}
			sessionFactory={() => {
				const session = new LayoutEngineSession(chromeMode, onHit);
				sessionRef.current = session;
				void ensureLayoutWasm().then(() => {
					session.setDocumentJson(documentJson);
					session.setPageId(pageId);
					session.setSelectedIds(selectedIds);
				});
				return session;
			}}
			onSessionReady={(session) => {
				const layoutSession = session as LayoutEngineSession;
				sessionRef.current = layoutSession;
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
	});
}
