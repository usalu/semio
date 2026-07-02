// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/s-react` — studio provider, media graph canvas, app host surfaces. */
// #endregion 🧲Header

import React, { createContext, useCallback, useContext, useEffect, useMemo, useRef, useSyncExternalStore } from "react";
import { DagCanvas, type DagSession } from "@semio-tech/dag-react";
import {
	type SAppInstance,
	type SMediaGraph,
	type StudioCommand,
	type StudioStore,
	applyDagFixtureJsonToSMediaGraph,
	listSPrograms,
	sAppRegistration,
	sMediaGraphToDagFixtureJson,
	sResourceDescriptor,
} from "@semio-tech/s-core";
import { CATALOGUE_DRAG_MIME } from "@semio-tech/ui-react";

//#region 🔖StudioContext
const StudioStoreContext = createContext<StudioStore | null>(null);

export function SStudioProvider({ store, children }: { readonly store: StudioStore; readonly children: React.ReactNode }): React.ReactElement {
	return <StudioStoreContext.Provider value={store}>{children}</StudioStoreContext.Provider>;
}

export function useStudioStore(): StudioStore {
	const store = useContext(StudioStoreContext);
	if (!store) throw new Error("SStudioProvider is required");
	return store;
}

export function useStudioProjection(): ReturnType<StudioStore["projection"]> {
	const store = useStudioStore();
	return useSyncExternalStore(
		store.subscribe.bind(store),
		() => {
			void store.getGeneration();
			return store.projection();
		},
		() => store.projection(),
	);
}

export function useStudioGeneration(): number {
	const store = useStudioStore();
	return useSyncExternalStore(store.subscribe.bind(store), () => store.getGeneration(), () => store.getGeneration());
}

export function useDispatchStudioCommand(): (command: StudioCommand) => void {
	const store = useStudioStore();
	return useCallback((command: StudioCommand) => store.dispatch(command), [store]);
}
//#endregion 🔖StudioContext

//#region 🔖MediaGraphCanvas
export interface SMediaGraphCanvasProps {
	readonly graph: SMediaGraph;
	readonly instances: readonly SAppInstance[];
	readonly activeInstanceId?: string | null;
	readonly onSelectInstance?: (instanceId: string) => void;
	readonly onOpenInstance?: (instanceId: string) => void;
	readonly onMoveNode?: (nodeId: string, x: number, y: number) => void;
	readonly onConnectPorts?: (sourceNodeId: string, sourcePortId: string, targetNodeId: string, targetPortId: string) => void;
	readonly onRemoveInstance?: (instanceId: string) => void;
	readonly onDisconnectEdge?: (edgeId: string) => void;
	readonly onSpawnApp?: (programId: string, appId: string, position: { readonly x: number; readonly y: number }) => void;
	readonly editable?: boolean;
}

export function SMediaGraphCanvas({
	graph,
	instances,
	activeInstanceId,
	onSelectInstance,
	onOpenInstance,
	onMoveNode,
	onConnectPorts,
	onRemoveInstance,
	onSpawnApp,
	onDisconnectEdge,
	editable = false,
}: SMediaGraphCanvasProps): React.ReactElement {
	const sessionRef = useRef<DagSession | null>(null);
	const lastFixtureRef = useRef<string>("");
	const cameraRef = useRef({ x: 0, y: 0, zoom: 1 });
	const dispatchProxy = useCallback(
		(command: StudioCommand) => {
			if (command.kind === "moveMediaNode") onMoveNode?.(command.nodeId, command.x, command.y);
			if (command.kind === "connectMediaPorts") {
				onConnectPorts?.(command.sourceNodeId, command.sourcePortId, command.targetNodeId, command.targetPortId);
			}
			if (command.kind === "disconnectMediaEdge") onDisconnectEdge?.(command.edgeId);
		},
		[onConnectPorts, onDisconnectEdge, onMoveNode],
	);

	const fixtureJson = useMemo(() => sMediaGraphToDagFixtureJson(graph, instances, cameraRef.current), [graph, instances]);

	useEffect(() => {
		lastFixtureRef.current = fixtureJson;
	}, [fixtureJson]);

	const handleFixtureChange = useCallback(
		(nextJson: string) => {
			if (!editable) return;
			const before = JSON.parse(lastFixtureRef.current) as { readonly camera?: { readonly x: number; readonly y: number; readonly zoom: number } };
			if (before.camera) cameraRef.current = before.camera;
			applyDagFixtureJsonToSMediaGraph(graph, nextJson, dispatchProxy);
			lastFixtureRef.current = nextJson;
		},
		[dispatchProxy, editable, graph],
	);

	const handlePointerUp = useCallback(() => {
		const session = sessionRef.current;
		if (!session || !onOpenInstance) return;
		try {
			const instanceId = session.takePendingOpenInstanceId?.();
			if (instanceId) onOpenInstance(instanceId);
		} catch {
			/* wasm export unavailable */
		}
	}, [onOpenInstance]);

	const handleDrop = useCallback(
		(event: React.DragEvent<HTMLDivElement>) => {
			if (!editable || !onSpawnApp) return;
			event.preventDefault();
			const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
			if (!raw) return;
			let payload: { readonly programId?: string; readonly appId?: string };
			try {
				payload = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string };
			} catch {
				return;
			}
			if (!payload.programId || !payload.appId) return;
			const rect = event.currentTarget.getBoundingClientRect();
			const sx = event.clientX - rect.left;
			const sy = event.clientY - rect.top;
			const session = sessionRef.current;
			let position = { x: sx, y: sy };
			try {
				const world = session?.screenToWorld?.(sx, sy);
				if (world && world.length >= 2) position = { x: world[0]!, y: world[1]! };
			} catch {
				/* fallback screen coords */
			}
			onSpawnApp(payload.programId, payload.appId, position);
		},
		[editable, onSpawnApp],
	);

	return (
		<div
			className="relative h-full w-full"
			onDragOver={(event) => {
				if (editable && onSpawnApp) event.preventDefault();
			}}
			onDrop={handleDrop}
		>
			<DagCanvas
				className="h-full w-full"
				fixtureJson={fixtureJson}
				onFixtureChange={editable ? handleFixtureChange : undefined}
				onSessionReady={(session) => {
					sessionRef.current = session;
				}}
				onAfterPointerUp={handlePointerUp}
				automaticLod
			/>
			<div className="pointer-events-none absolute inset-x-0 top-0 flex flex-wrap gap-2 p-2">
				{graph.nodes.map((node) => {
					const instance = instances.find((entry) => entry.id === node.instanceId);
					const active = activeInstanceId === node.instanceId;
					return (
						<button
							key={node.id}
							type="button"
							className={`pointer-events-auto rounded border px-2 py-1 text-xs ${active ? "border-[var(--semio-accent)] bg-[var(--semio-accent-subtle)]" : "border-[var(--semio-border-default)] bg-[var(--semio-surface-raised)]"}`}
							onClick={() => onSelectInstance?.(node.instanceId)}
						>
							{instance?.label ?? node.instanceId}
						</button>
					);
				})}
			</div>
		</div>
	);
}
//#endregion 🔖MediaGraphCanvas

//#region 🔖ProgramLauncher
export function SProgramLauncherPanel(): React.ReactElement {
	const dispatch = useDispatchStudioCommand();
	const programs = useMemo(() => listSPrograms().filter((program) => program.id !== "s.system"), []);
	return (
		<div className="flex h-full flex-col gap-2 overflow-auto p-3">
			<div className="text-xs font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">Programs</div>
			{programs.map((program) => (
				<div key={program.id} className="rounded border border-[var(--semio-border-default)] p-2">
					<div className="text-sm font-semibold text-[var(--semio-text-primary)]">{program.name}</div>
					<div className="mt-2 flex flex-col gap-1">
						{program.apps.map((app) => (
							<button
								key={app.id}
								type="button"
								className="rounded px-2 py-1 text-left text-xs hover:bg-[var(--semio-surface-muted)]"
								onClick={() => dispatch({ kind: "spawnAppInstance", programId: program.id, appId: app.id })}
							>
								{app.label}
							</button>
						))}
					</div>
				</div>
			))}
		</div>
	);
}
//#endregion 🔖ProgramLauncher

//#region 🔖Catalogue
export function buildSPlayCatalogueTree(): import("@semio-tech/framework-playground-core").UiTreeNode {
	const programs = listSPrograms().filter((program) => program.id !== "s.system");
	return {
		type: "section",
		label: "Apps",
		children: programs.map((program) => ({
			type: "section",
			label: program.name,
			children: program.apps.map((app) => ({
				type: "item",
				label: app.label,
				dragData: { [CATALOGUE_DRAG_MIME]: JSON.stringify({ programId: program.id, appId: app.id }) },
				meta: sAppRegistration(program.id, app.id)?.outputs.map((port) => port.resourceKind).join(", ") ?? "",
			})),
		})),
	};
}
//#endregion 🔖Catalogue

//#region 🔖History
export function SStudioHistoryPanel(): React.ReactElement {
	const store = useStudioStore();
	const generation = useStudioGeneration();
	void generation;
	const document = store.getDocument();
	return (
		<div className="flex h-full flex-col gap-2 overflow-auto p-3 text-xs">
			<div className="font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">History</div>
			{document.vcs.operations.map((change) => (
				<div key={change.id} className="rounded border border-[var(--semio-border-default)] px-2 py-1">
					{change.description ?? change.id}
				</div>
			))}
		</div>
	);
}
//#endregion 🔖History

//#region 🔖AppHost
export function SAppHostSurface({ instanceId }: { readonly instanceId: string | null }): React.ReactElement {
	const projection = useStudioProjection();
	const instance = projection.appInstances.find((entry) => entry.id === instanceId) ?? null;
	const resource = instance ? sResourceDescriptor(instance.yields) : null;
	return (
		<div className="flex h-full flex-col">
			<div className="border-b border-[var(--semio-border-default)] px-3 py-2 text-sm font-semibold">
				{instance?.label ?? "No app selected"}
			</div>
			<div className="px-3 py-1 text-xs text-[var(--semio-text-secondary)]">
				{instance ? `${instance.programId}/${instance.appId} · ${resource?.name ?? instance.yields}` : "Spawn or select an app instance."}
			</div>
		</div>
	);
}
//#endregion 🔖AppHost
