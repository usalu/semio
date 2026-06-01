/** @emoji 🧪 jsdom polyfills for presentation renderer tests. */
import { createElement, type ReactNode } from "react";
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

vi.mock("react-pdf", () => ({
	Document: ({ children }: { readonly children?: ReactNode }) =>
		createElement("div", { className: "react-pdf__Document" }, children),
	Page: () => createElement("div", { className: "react-pdf__Page" }),
	pdfjs: { GlobalWorkerOptions: { workerSrc: "" }, version: "0.0.0" },
}));
