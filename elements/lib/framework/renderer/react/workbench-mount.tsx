// #region 🧲Header
/** @emoji 🖥 Mount helpers for {@link WorkbenchView} into a DOM root. */
// #endregion 🧲Header

import type { Workbench } from "@elements/framework";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";

import { WorkbenchView } from "./workbench-view.tsx";

type ElementsDomRoot = HTMLElement & { __elementsReactRoot?: Root };

function getElementById<T extends HTMLElement = HTMLElement>(id: string): T | null {
	return document.getElementById(id) as T | null;
}

/** @emoji ⚛️ Imperative React root helpers for workbench shells. */
export class ReactUI {
	private static mountedRoot: Root | null = null;

	/** @emoji 🖥️ Mounts a {@link Workbench} shell into `#root` (or `rootId`) with {@link WorkbenchView}. */
	static mount(workbench: Workbench, rootId = "root"): void {
		if (typeof document === "undefined") return;
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		if (!rootElement) {
			throw new Error(`React root #${rootId} missing.`);
		}
		rootElement.__elementsReactRoot ??= createRoot(rootElement);
		ReactUI.mountedRoot = rootElement.__elementsReactRoot;
		rootElement.__elementsReactRoot.render(<WorkbenchView workbench={workbench} />);
	}

	static unmount(rootId = "root"): void {
		const rootElement = getElementById<ElementsDomRoot>(rootId);
		rootElement?.__elementsReactRoot?.unmount();
		if (rootElement) {
			delete rootElement.__elementsReactRoot;
		}
		ReactUI.mountedRoot = null;
	}
}

/** @emoji 🖥️ Mounts an arbitrary React tree into `#root` (or `rootId`). */
export function mountReactApp(element: React.ReactElement, rootId = "root"): void {
	if (typeof document === "undefined") return;
	const rootElement = getElementById<ElementsDomRoot>(rootId);
	if (!rootElement) {
		throw new Error(`React root #${rootId} missing.`);
	}
	rootElement.__elementsReactRoot ??= createRoot(rootElement);
	rootElement.__elementsReactRoot.render(element);
}

/** @emoji 🖥️ Loads a {@link Workbench} asynchronously then mounts {@link WorkbenchView}. */
export async function mountAsyncReactApp(loadWorkbench: () => Promise<Workbench>, rootId = "root"): Promise<void> {
	ReactUI.mount(await loadWorkbench(), rootId);
}
