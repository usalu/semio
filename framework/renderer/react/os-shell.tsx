import { useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import { DEFAULT_PLUGIN_REGISTRY, loadPluginModule, type PluginWasmHandle } from "./plugin-runtime.ts";
import type { AppDefinition, CommandDescriptor, PluginManifest, UiNode, ViewState } from "./types.ts";
import { interpretUiNode } from "./ui-interpreter.tsx";

type LoadedPluginState = {
	readonly handle: PluginWasmHandle;
	readonly manifest: PluginManifest;
};

type ActiveSession = {
	readonly pluginId: string;
	readonly instanceId: number;
	readonly app: AppDefinition;
	readonly viewState: ViewState;
};

export type FrameworkOsBootOptions = {
	readonly rootId?: string;
	readonly plugin?: string;
	readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
};

export async function bootFrameworkOs(options: FrameworkOsBootOptions = {}): Promise<void> {
	const root = document.getElementById(options.rootId ?? "root");
	if (!root) throw new Error("missing #root");
	createRoot(root).render(
		<FrameworkOsShell
			pluginFilter={options.plugin}
			plugins={options.plugins ?? DEFAULT_PLUGIN_REGISTRY}
		/>,
	);
}

export function FrameworkOsShell({
	pluginFilter,
	plugins,
}: {
	readonly pluginFilter?: string;
	readonly plugins: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
}) {
	const [loadedPlugins, setLoadedPlugins] = useState<readonly LoadedPluginState[]>([]);
	const [session, setSession] = useState<ActiveSession | null>(null);
	const [uiTree, setUiTree] = useState<UiNode | null>(null);
	const [error, setError] = useState<string | null>(null);

	const registry = useMemo(
		() => (pluginFilter ? plugins.filter((entry) => entry.pluginId === pluginFilter) : plugins),
		[pluginFilter, plugins],
	);

	useEffect(() => {
		let cancelled = false;
		void (async () => {
			try {
				const loaded = await Promise.all(registry.map((entry) => loadPluginModule(entry.pluginId, entry.moduleUrl)));
				if (cancelled) return;
				setLoadedPlugins(loaded.map((handle) => ({ handle, manifest: handle.manifest })));
				const first = loaded[0];
				const firstApp = first?.manifest.apps[0];
				if (first && firstApp) {
					const instanceId = await first.createApp(firstApp.id);
					const viewState: ViewState = {
						activeModeId: firstApp.defaultModeId ?? firstApp.modes[0]?.id,
						activeWindowKindId: firstApp.windowKinds[0]?.id,
					};
					setSession({
						pluginId: first.pluginId,
						instanceId,
						app: firstApp,
						viewState,
					});
				}
			} catch (bootError) {
				if (!cancelled) {
					console.error("[DEBUG] framework os boot failed", bootError);
					setError(bootError instanceof Error ? bootError.message : String(bootError));
				}
			}
		})();
		return () => {
			cancelled = true;
		};
	}, [registry]);

	useEffect(() => {
		if (!session) return;
		const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
		if (!plugin) return;
		const bodyKey = session.app.windowKinds[0]?.bodyKey ?? "main";
		void plugin.render(session.instanceId, bodyKey, session.viewState).then(setUiTree).catch((renderError) => {
			console.error("[DEBUG] render failed", renderError);
			setError(renderError instanceof Error ? renderError.message : String(renderError));
		});
	}, [loadedPlugins, session]);

	const onCommand = useCallback(
		(command: CommandDescriptor) => {
			if (!session) return;
			const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
			if (!plugin) return;
			void plugin
				.handleCommand(session.instanceId, JSON.stringify(command), session.viewState)
				.then(() => plugin.render(session.instanceId, session.app.windowKinds[0]?.bodyKey ?? "main", session.viewState))
				.then(setUiTree)
				.catch((commandError) => {
					console.error("[DEBUG] command failed", commandError);
				});
		},
		[loadedPlugins, session],
	);

	const apps = loadedPlugins.flatMap((entry) => entry.manifest.apps);

	return (
		<div className="semio-framework-os" style={{ display: "grid", gridTemplateRows: "auto 1fr", height: "100vh" }}>
			<header className="semio-framework-os-navbar" style={{ display: "flex", gap: "0.5rem", padding: "0.5rem", borderBottom: "1px solid var(--semio-border)" }}>
				<strong>semio os</strong>
				{apps.map((app) => (
					<button
						key={app.id}
						type="button"
						aria-pressed={session?.app.id === app.id}
						onClick={() => {
							const pluginEntry = loadedPlugins.find((entry) => entry.manifest.apps.some((candidate) => candidate.id === app.id));
							if (!pluginEntry) return;
							void pluginEntry.handle.createApp(app.id).then((instanceId) => {
								setSession({
									pluginId: pluginEntry.handle.pluginId,
									instanceId,
									app,
									viewState: {
										activeModeId: app.defaultModeId ?? app.modes[0]?.id,
										activeWindowKindId: app.windowKinds[0]?.id,
									},
								});
							});
						}}
					>
						{app.label}
					</button>
				))}
			</header>
			<main className="semio-framework-os-main" style={{ padding: "0.5rem", overflow: "auto" }}>
				{error ? <p role="alert">{error}</p> : null}
				{uiTree ? interpretUiNode(uiTree, { onCommand }) : <p>Loading plugins…</p>}
			</main>
		</div>
	);
}
