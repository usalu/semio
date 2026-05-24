// #region 🧲Header
/** @emoji 🧭 URI history hook for workbench shells (framework-react, no `@elements/ui`). */
// #endregion 🧲Header

import * as React from "react";

/** @emoji 🔖 Single URI stack entry. */
export interface UIHistoryEntry {
	readonly uri: string;
}

/** @emoji 🔖 URI navigation stack state. */
export interface UIHistory {
	readonly entries: readonly UIHistoryEntry[];
	readonly index: number;
}

/** @emoji 🧭 Manages URI history with back, forward, up, and navigate. */
export function useUIHistory(initialUri = "/"): {
	readonly history: UIHistory;
	readonly uri: string;
	readonly canGoBack: boolean;
	readonly canGoForward: boolean;
	readonly canGoUp: boolean;
	readonly parentUri: string | null;
	readonly goBack: () => void;
	readonly goForward: () => void;
	readonly goUp: () => void;
	readonly navigate: (uri: string) => void;
} {
	const [history, setHistory] = React.useState<UIHistory>({
		entries: [{ uri: initialUri }],
		index: 0,
	});
	const uri = history.entries[history.index]?.uri ?? initialUri;
	const canGoBack = history.index > 0;
	const canGoForward = history.index < history.entries.length - 1;
	const segments = uri.split("/").filter(Boolean);
	const canGoUp = segments.length > 0;
	const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;

	const goBack = React.useCallback(() => {
		setHistory((prev) => (prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev));
	}, []);
	const goForward = React.useCallback(() => {
		setHistory((prev) => (prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev));
	}, []);
	const goUp = React.useCallback(() => {
		if (!canGoUp || parentUri === null) return;
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
		});
	}, [canGoUp, parentUri]);
	const navigate = React.useCallback((targetUri: string) => {
		setHistory((prev) => {
			const newEntries = prev.entries.slice(0, prev.index + 1);
			return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
		});
	}, []);

	return { history, uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate };
}

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("useUIHistory types", () => {
		it("exports history entry shape", () => {
			const entry: UIHistoryEntry = { uri: "/test" };
			expect(entry.uri).toBe("/test");
		});
	});
}
//#endregion 🧪Tests
