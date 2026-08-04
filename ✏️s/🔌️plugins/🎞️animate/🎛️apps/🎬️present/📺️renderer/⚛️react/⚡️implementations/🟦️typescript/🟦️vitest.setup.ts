/** @emoji 🧪️ jsdom polyfills for presentation renderer tests. */
import { createElement, useLayoutEffect, type ReactNode } from "react";
import { vi } from "vitest";

Object.defineProperty(globalThis, "IS_REACT_ACT_ENVIRONMENT", { value: true, writable: true });

Object.defineProperty(window, "matchMedia", {
	writable: true,
	value: (query: string) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: () => undefined,
		removeListener: () => undefined,
		addEventListener: () => undefined,
		removeEventListener: () => undefined,
		dispatchEvent: () => false,
	}),
});

if (typeof globalThis.DOMMatrix === "undefined") {
	(globalThis as { DOMMatrix?: typeof DOMMatrix }).DOMMatrix = class DOMMatrix {
		a = 1;
		b = 0;
		c = 0;
		d = 1;
		e = 0;
		f = 0;
	} as unknown as typeof DOMMatrix;
}

if (typeof globalThis.Path2D === "undefined") {
	(globalThis as { Path2D?: typeof Path2D }).Path2D = class Path2D {} as unknown as typeof Path2D;
}

function stubFetchBody(url: string): { body: string; contentType: string } {
	const path = url.split("?")[0]?.split("#")[0] ?? url;
	if (path.endsWith(".ops") || path.endsWith(".dsl") || path.endsWith(".spk")) {
		return { body: "", contentType: "application/octet-stream" };
	}
	if (path.endsWith(".md") || path.endsWith(".markdown")) {
		return { body: "# stub", contentType: "text/markdown" };
	}
	return { body: "", contentType: "text/plain" };
}

function stubRelativeFetchResponse(url: string): Response {
	const { body, contentType } = stubFetchBody(url);
	return new Response(body, { status: 200, headers: { "content-type": contentType } });
}

function isRelativeFetchUrl(url: string): boolean {
	return url.startsWith("./") || (url.startsWith("/") && !url.startsWith("//"));
}

if (typeof globalThis.fetch !== "function") {
	(globalThis as { fetch?: typeof fetch }).fetch = async (input: RequestInfo | URL) => {
		const url = typeof input === "string" ? input : input instanceof URL ? input.href : input.url;
		if (isRelativeFetchUrl(url)) {
			return stubRelativeFetchResponse(url);
		}
		throw new TypeError(`fetch not available for ${url}`);
	};
} else {
	const originalFetch = globalThis.fetch.bind(globalThis);
	globalThis.fetch = async (input: RequestInfo | URL, init?: RequestInit) => {
		const url = typeof input === "string" ? input : input instanceof URL ? input.href : input.url;
		if (isRelativeFetchUrl(url)) {
			return stubRelativeFetchResponse(url);
		}
		return originalFetch(input, init);
	};
}

if (typeof Promise.withResolvers !== "function") {
	Promise.withResolvers = <T>() => {
		let resolve!: (value: T | PromiseLike<T>) => void;
		let reject!: (reason?: unknown) => void;
		const promise = new Promise<T>((res, rej) => {
			resolve = res;
			reject = rej;
		});
		return { promise, resolve, reject };
	};
}

const originalGetContext = HTMLCanvasElement.prototype.getContext;
HTMLCanvasElement.prototype.getContext = function getContext(
	type: string,
	options?: CanvasRenderingContext2DSettings,
): RenderingContext | null {
	if (type === "2d") {
		return {
			fillRect: () => undefined,
			clearRect: () => undefined,
			getImageData: () => ({ data: new Uint8ClampedArray(4) }),
			putImageData: () => undefined,
			createImageData: () => ({ data: new Uint8ClampedArray(4) }),
			setTransform: () => undefined,
			drawImage: () => undefined,
			save: () => undefined,
			restore: () => undefined,
			beginPath: () => undefined,
			moveTo: () => undefined,
			lineTo: () => undefined,
			closePath: () => undefined,
			stroke: () => undefined,
			translate: () => undefined,
			scale: () => undefined,
			rotate: () => undefined,
			arc: () => undefined,
			fill: () => undefined,
			measureText: () => ({ width: 0 }),
			transform: () => undefined,
			rect: () => undefined,
			clip: () => undefined,
			canvas: this,
		} as unknown as CanvasRenderingContext2D;
	}
	return originalGetContext.call(this, type, options);
};

class ResizeObserverStub {
	observe(): void {
		undefined;
	}
	disconnect(): void {
		undefined;
	}
}

if (typeof globalThis.ResizeObserver === "undefined") {
	(globalThis as { ResizeObserver?: typeof ResizeObserver }).ResizeObserver =
		ResizeObserverStub as unknown as typeof ResizeObserver;
}

if (typeof globalThis.PointerEvent === "undefined") {
	(globalThis as { PointerEvent?: typeof PointerEvent }).PointerEvent = class PointerEvent extends MouseEvent {
		readonly pointerId: number;
		constructor(type: string, params: PointerEventInit = {}) {
			super(type, params);
			this.pointerId = params.pointerId ?? 1;
		}
	} as unknown as typeof PointerEvent;
}

if (typeof Range !== "undefined" && typeof Range.prototype.getClientRects !== "function") {
	Range.prototype.getClientRects = function getClientRectsPolyfill(this: Range): DOMRectList {
		const element =
			this.commonAncestorContainer.nodeType === Node.ELEMENT_NODE
				? (this.commonAncestorContainer as HTMLElement)
				: this.commonAncestorContainer.parentElement;
		if (!element) {
			return [] as unknown as DOMRectList;
		}
		const box = element.getBoundingClientRect();
		const text = element.textContent?.trim() ?? "";
		const width = Math.min(Math.max(text.length * 9, 24), box.width);
		const left = box.left + (box.width - width) / 2;
		return [new DOMRect(left, box.top, width, box.height)] as unknown as DOMRectList;
	};
}

vi.mock("react-pdf", () => ({
	Document: ({
		children,
		onLoadSuccess,
	}: {
		readonly children?: ReactNode;
		readonly onLoadSuccess?: (payload: { readonly numPages: number }) => void;
	}) => {
		useLayoutEffect(() => {
			onLoadSuccess?.({ numPages: 3 });
		}, [onLoadSuccess]);
		return createElement("div", { className: "react-pdf__Document" }, children);
	},
	Page: ({
		onLoadSuccess,
		scale,
		pageNumber,
	}: {
		readonly onLoadSuccess?: (page: {
			getViewport: (options: { readonly scale: number }) => { readonly width: number; readonly height: number };
		}) => void;
		readonly scale?: number;
		readonly pageNumber?: number;
	}) => {
		useLayoutEffect(() => {
			onLoadSuccess?.({
				getViewport: ({ scale: viewportScale }) => ({
					width: 595 * viewportScale,
					height: 842 * viewportScale,
				}),
			});
		}, [onLoadSuccess, pageNumber]);
		return createElement("div", {
			className: "react-pdf__Page",
			"data-scale": scale === undefined ? undefined : String(scale),
			"data-page": pageNumber === undefined ? undefined : String(pageNumber),
		});
	},
	pdfjs: { GlobalWorkerOptions: { workerSrc: "" }, version: "0.0.0" },
}));
