// #region 🧲Header
/** @emoji 🧭 Browser History API sync for OS shell {@link Platform} routing. */
// #endregion 🧲Header

import type { Platform } from "@semio-tech/framework-core";
import { reactHostPort, useUIHistory } from "@semio-tech/framework-platform-renderer-react";

function readBrowserUri(): string {
	if (typeof window === "undefined") return "/";
	return `${window.location.pathname}${window.location.search}`;
}

/** @emoji 🧭 Syncs {@link Platform.applyUri} with browser history for OS home/studio routing. */
export function useOsShellHistory(platform: Platform): {
	readonly uri: string;
	readonly navigate: (targetUri: string) => void;
} {
	const { uri, canGoBack, canGoForward, canGoUp, navigate, syncUri } = useUIHistory(readBrowserUri());

	reactHostPort.useEffect(() => {
		platform.applyUri?.(uri);
		platform.uri = uri;
		platform.canGoBack = canGoBack;
		platform.canGoForward = canGoForward;
		platform.canGoUp = canGoUp;
		if (typeof window !== "undefined") {
			const current = `${window.location.pathname}${window.location.search}`;
			if (current !== uri) {
				window.history.pushState(null, "", uri);
			}
		}
		platform.notify();
	}, [uri, canGoBack, canGoForward, canGoUp, platform]);

	reactHostPort.useEffect(() => {
		if (typeof window === "undefined") return;
		const onPopState = () => {
			syncUri(readBrowserUri());
		};
		window.addEventListener("popstate", onPopState);
		return () => window.removeEventListener("popstate", onPopState);
	}, [syncUri]);

	const handleNavigate = reactHostPort.useCallback(
		(targetUri: string) => {
			platform.applyUri?.(targetUri);
			navigate(targetUri);
		},
		[navigate, platform],
	);

	reactHostPort.useEffect(() => {
		platform.onNavigate = handleNavigate;
		return () => {
			if (platform.onNavigate === handleNavigate) platform.onNavigate = undefined;
		};
	}, [handleNavigate, platform]);

	return { uri, navigate: handleNavigate };
}
