// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/semios-react` — studio provider, media graph canvas, app host surfaces. */
// #endregion 🧲Header

import React, { createContext, useCallback, useContext, useMemo, useRef, useState, useSyncExternalStore } from "react";
import {
	type SemiosAppInstance,
	type SemiosMediaGraphEdge,
	type SemiosMediaGraphNode,
	type SemiosMediaGraphV1,
	type StudioCommand,
	type StudioStore,
	listSemiosPrograms,
	semiosResourceDescriptor,
} from "@semio-tech/semios-core";

//#region 🔖StudioContext
const StudioStoreContext = createContext<StudioStore | null>(null);

export function SemiosStudioProvider({ store, children }: { readonly store: StudioStore; readonly children: React.ReactNode }): React.ReactElement {
	return <StudioStoreContext.Provider value={store}>{children}</StudioStoreContext.Provider>;
}

export function useStudioStore(): StudioStore {
	const store = useContext(StudioStoreContext);
	if (!store) throw new Error("SemiosStudioProvider is required");
	return store;
}

export function useStudioProjection(): ReturnType<StudioStore["projection"]> {
	const store = useStudioStore();
	return useSyncExternalStore(store.subscribe.bind(store), () => store.projection(), () => store.projection());
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
export interface SemiosMediaGraphCanvasProps {
	readonly graph: SemiosMediaGraphV1;
	readonly instances: readonly SemiosAppInstance[];
	readonly activeInstanceId?: string | null;
	readonly onSelectInstance?: (instanceId: string) => void;
	readonly onMoveNode?: (nodeId: string, x: number, y: number) => void;
	readonly onConnectPorts?: (sourceNodeId: string, sourcePortId: string, targetNodeId: string, targetPortId: string) => void;
	readonly onRemoveInstance?: (instanceId: string) => void;
	readonly editable?: boolean;
}

export function SemiosMediaGraphCanvas({
	graph,
	instances,
	activeInstanceId,
	onSelectInstance,
	onMoveNode,
	onConnectPorts,
	onRemoveInstance,
	editable = false,
}: SemiosMediaGraphCanvasProps): React.ReactElement {
	const instanceById = useMemo(() => new Map(instances.map((instance) => [instance.id, instance])), [instances]);
	const [pendingSource, setPendingSource] = useState<{ nodeId: string; portId: string } | null>(null);
	const dragRef = useRef<{ nodeId: string; offsetX: number; offsetY: number } | null>(null);

	const handleNodePointerDown = (node: SemiosMediaGraphNode, event: React.PointerEvent<SVGGElement>) => {
		if (!editable || !onMoveNode) return;
		const target = event.currentTarget;
		target.setPointerCapture(event.pointerId);
		dragRef.current = { nodeId: node.id, offsetX: event.clientX - node.x, offsetY: event.clientY - node.y };
	};

	const handleNodePointerMove = (event: React.PointerEvent<SVGGElement>) => {
		const drag = dragRef.current;
		if (!drag || !onMoveNode) return;
		onMoveNode(drag.nodeId, event.clientX - drag.offsetX, event.clientY - drag.offsetY);
	};

	const handleNodePointerUp = (event: React.PointerEvent<SVGGElement>) => {
		if (dragRef.current) {
			event.currentTarget.releasePointerCapture(event.pointerId);
			dragRef.current = null;
		}
	};

	const handleOutputClick = (node: SemiosMediaGraphNode, portId: string, event: React.MouseEvent) => {
		event.stopPropagation();
		if (!editable) return;
		setPendingSource({ nodeId: node.id, portId });
	};

	const handleInputClick = (node: SemiosMediaGraphNode, portId: string, event: React.MouseEvent) => {
		event.stopPropagation();
		if (!editable || !pendingSource || !onConnectPorts) return;
		onConnectPorts(pendingSource.nodeId, pendingSource.portId, node.id, portId);
		setPendingSource(null);
	};

	return (
		<svg
			className="h-full w-full bg-[var(--semio-surface-canvas)]"
			role="img"
			aria-label="Studio media graph"
			onPointerMove={handleNodePointerMove}
			onPointerUp={handleNodePointerUp}
		>
			{graph.edges.map((edge: SemiosMediaGraphEdge) => {
				const source = graph.nodes.find((node) => node.id === edge.sourceNodeId);
				const target = graph.nodes.find((node) => node.id === edge.targetNodeId);
				if (!source || !target) return null;
				const x1 = source.x + source.width;
				const y1 = source.y + source.height / 2;
				const x2 = target.x;
				const y2 = target.y + target.height / 2;
				return <line key={edge.id} x1={x1} y1={y1} x2={x2} y2={y2} stroke="var(--semio-edge-default)" strokeWidth={2} />;
			})}
			{graph.nodes.map((node) => {
				const instance = instanceById.get(node.instanceId);
				const resource = instance ? semiosResourceDescriptor(instance.yields) : null;
				const active = activeInstanceId === node.instanceId;
				return (
					<g
						key={node.id}
						transform={`translate(${node.x} ${node.y})`}
						onClick={() => onSelectInstance?.(node.instanceId)}
						onPointerDown={(event) => handleNodePointerDown(node, event)}
						style={{ cursor: editable ? "grab" : "pointer" }}
					>
						<rect
							width={node.width}
							height={node.height}
							rx={8}
							fill={active ? "var(--semio-accent-subtle)" : "var(--semio-surface-raised)"}
							stroke={active ? "var(--semio-accent)" : "var(--semio-border-default)"}
							strokeWidth={active ? 2 : 1}
						/>
						<text x={12} y={24} fill="var(--semio-text-primary)" fontSize={13} fontWeight={600}>
							{instance?.label ?? node.instanceId}
						</text>
						<text x={12} y={44} fill="var(--semio-text-secondary)" fontSize={11}>
							{resource?.kind ?? "resource"}
						</text>
						<text x={12} y={60} fill="var(--semio-text-tertiary)" fontSize={10}>
							{instance?.programId}/{instance?.appId}
						</text>
						{editable ? (
							<>
								<circle cx={0} cy={node.height / 2} r={5} fill="var(--semio-accent)" onClick={(event) => handleInputClick(node, node.inputs[0]?.id ?? "", event)} />
								<circle
									cx={node.width}
									cy={node.height / 2}
									r={5}
									fill={pendingSource?.nodeId === node.id ? "var(--semio-accent)" : "var(--semio-border-strong)"}
									onClick={(event) => handleOutputClick(node, node.outputs[0]?.id ?? "", event)}
								/>
								{onRemoveInstance ? (
									<text
										x={node.width - 12}
										y={14}
										textAnchor="end"
										fill="var(--semio-text-tertiary)"
										fontSize={10}
										onClick={(event) => {
											event.stopPropagation();
											onRemoveInstance(node.instanceId);
										}}
									>
										×
									</text>
								) : null}
							</>
						) : null}
					</g>
				);
			})}
		</svg>
	);
}
//#endregion 🔖MediaGraphCanvas

//#region 🔖ProgramLauncher
export function SemiosProgramLauncherPanel(): React.ReactElement {
	const dispatch = useDispatchStudioCommand();
	const programs = useMemo(() => listSemiosPrograms().filter((program) => program.id !== "semios.system"), []);
	return (
		<div className="flex h-full flex-col gap-2 overflow-auto p-3">
			<div className="text-xs font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">Programs</div>
			{programs.map((program) => (
				<div key={program.id} className="rounded border border-[var(--semio-border-subtle)] p-2">
					<div className="text-sm font-medium text-[var(--semio-text-primary)]">{program.name}</div>
					<div className="mt-2 flex flex-col gap-1">
						{program.apps.map((app) => (
							<button
								key={`${program.id}/${app.id}`}
								type="button"
								className="rounded bg-[var(--semio-surface-raised)] px-2 py-1 text-left text-xs text-[var(--semio-text-primary)] hover:bg-[var(--semio-accent-subtle)]"
								onClick={() =>
									dispatch({
										kind: "spawnAppInstance",
										programId: program.id,
										appId: app.id,
										position: { x: 40 + Math.random() * 120, y: 40 + Math.random() * 120 },
									})
								}
							>
								Spawn {app.label}
							</button>
						))}
					</div>
				</div>
			))}
		</div>
	);
}
//#endregion 🔖ProgramLauncher

//#region 🔖StudioHistory
export function SemiosStudioHistoryPanel(): React.ReactElement {
	const store = useStudioStore();
	const dispatch = useDispatchStudioCommand();
	const document = useSyncExternalStore(store.subscribe.bind(store), () => store.getDocument(), () => store.getDocument());
	const projection = useStudioProjection();
	return (
		<div className="flex h-full flex-col gap-3 overflow-auto p-3 text-xs text-[var(--semio-text-primary)]">
			<div className="font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">Studio</div>
			<div>{document.name}</div>
			<div className="text-[var(--semio-text-secondary)]">Instances: {projection.appInstances.length}</div>
			<div className="text-[var(--semio-text-secondary)]">Edges: {projection.mediaGraph.edges.length}</div>
			<div className="text-[var(--semio-text-secondary)]">Backbone: {document.backbone?.uri ?? "none"}</div>
			<div className="flex gap-2">
				<button type="button" className="rounded border px-2 py-1" onClick={() => dispatch({ kind: "undo" })}>
					Undo
				</button>
				<button type="button" className="rounded border px-2 py-1" onClick={() => dispatch({ kind: "redo" })}>
					Redo
				</button>
				<button type="button" className="rounded border px-2 py-1" onClick={() => dispatch({ kind: "commitCheckpoint", message: "snapshot" })}>
					Checkpoint
				</button>
			</div>
			<div className="font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">Checkpoints</div>
			{document.vcs.checkpoints.length === 0 ? <div className="text-[var(--semio-text-tertiary)]">No checkpoints</div> : null}
			{document.vcs.checkpoints.map((checkpoint) => (
				<div key={checkpoint.id} className="rounded border border-[var(--semio-border-subtle)] p-2">
					<div>{checkpoint.message ?? checkpoint.id}</div>
					<div className="text-[var(--semio-text-tertiary)]">{checkpoint.savedAt}</div>
				</div>
			))}
			<div className="font-semibold uppercase tracking-wide text-[var(--semio-text-secondary)]">Alternatives</div>
			{document.vcs.alternatives.length === 0 ? <div className="text-[var(--semio-text-tertiary)]">No alternatives</div> : null}
			{document.vcs.alternatives.map((alternative) => (
				<button
					key={alternative.id}
					type="button"
					className="rounded border border-[var(--semio-border-subtle)] p-2 text-left hover:bg-[var(--semio-accent-subtle)]"
					onClick={() => dispatch({ kind: "switchAlternative", alternativeId: alternative.id })}
				>
					<div>{alternative.name}</div>
					<div className="text-[var(--semio-text-tertiary)]">
						{projection.activeAlternativeId === alternative.id ? "active" : `${alternative.checkpointIds.length} checkpoint(s)`}
					</div>
				</button>
			))}
			<button
				type="button"
				className="rounded border px-2 py-1"
				onClick={() => dispatch({ kind: "createAlternative", name: `Branch ${document.vcs.alternatives.length + 1}` })}
			>
				New alternative
			</button>
		</div>
	);
}
//#endregion 🔖StudioHistory

//#region 🔖AppHost
export interface SemiosAppHostSurfaceProps {
	readonly instance: SemiosAppInstance | null;
	readonly children: React.ReactNode;
}

export function SemiosAppHostSurface({ instance, children }: SemiosAppHostSurfaceProps): React.ReactElement {
	if (!instance) {
		return (
			<div className="flex h-full items-center justify-center text-sm text-[var(--semio-text-secondary)]">
				Select an app instance in the media graph
			</div>
		);
	}
	return (
		<div className="flex h-full min-h-0 flex-col" data-semios-instance-id={instance.id} data-semios-program-id={instance.programId}>
			<div className="border-b border-[var(--semio-border-subtle)] px-3 py-2 text-sm font-medium text-[var(--semio-text-primary)]">
				{instance.label} · {instance.programId}
			</div>
			<div className="min-h-0 flex-1">{children}</div>
		</div>
	);
}
//#endregion 🔖AppHost
