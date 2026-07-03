// #region 🧲Header
/** @emoji 🧩 Lazy platform puzzle renderers — keeps puzzle packages off the OS home boot path. */
// #endregion 🧲Header

import * as React from "react";
import type { TreeDragAndDropController } from "@semio-tech/ui-react";
import {
	CommandBus,
	Puzzle2d,
	Puzzle3d,
	Puzzle5d,
	getPlatformControllerById,
	platformTopologyStoreId,
	type Component,
	type ComponentKind,
	type Platform,
	type PlatformTopologyPayload,
	type Puzzle5dModel,
	type UiComponentHostSurfaceNode,
	type UiPuzzle2dHostSurfaceNode,
	type UiPuzzle5dHostSurfaceNode,
} from "@semio-tech/framework-platform-core";
import {
	PUZZLE_2D_FIXTURE_DRAG_MIME,
	Puzzle2dCanvas,
	parsePuzzle2dFixture,
	puzzle2dFixturePaletteTreeDragController,
	type Puzzle2dPreselectSnapshot,
	type Puzzle2dSelectionSnapshot,
} from "@semio-tech/puzzle-2d-react";
import { parseFixture, puzzle3dFixturePaletteTreeDragController, type SelectionSnapshot as Puzzle3dSelectionSnapshot } from "@semio-tech/puzzle-3d-react";
import { FiveD, StoreProvider, compose5d, createStore, mergeLiveForceGraphTopologyModel, prepareTopologyModel } from "@semio-tech/puzzle-5d-react";
import { useControllerStore, useStore } from "./index.tsx";

type ComponentKindRendererProps = {
	readonly component: Component<unknown>;
	readonly node: UiComponentHostSurfaceNode;
	readonly commandBus: CommandBus;
	readonly layout: "canvas" | "panel";
	readonly platform?: Platform;
};

type ComponentKindRenderer = React.ComponentType<ComponentKindRendererProps>;

type SurfaceBindingHost = React.ComponentType<{ readonly node: UiComponentHostSurfaceNode; readonly platform?: Platform }>;

const BuiltinPuzzle2dKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Puzzle2d);
	if (model.nodes.length === 0 && model.edges.length === 0) {
		return (
			<div className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground" data-surface-id={node.surfaceId}>
				{model.emptyMessage ?? "Empty puzzle 2d"}
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
			<Puzzle2dCanvas className="h-full w-full" />
		</div>
	);
};

const BuiltinPuzzle3dKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Puzzle3d);
	return (
		<div className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground" data-surface-id={node.surfaceId}>
			{model.emptyMessage ?? `3D scene${model.instanceId ? ` · ${model.instanceId}` : ""}`}
		</div>
	);
};

/** @emoji 🔑 Stable topology identity ignoring flat node positions (live force updates positions locally). */
export function platformTopologyStructureKey(flat: Record<string, unknown>, volume: Record<string, unknown>): string {
	const parsed = parsePuzzle2dFixture(flat);
	if (!parsed) return "";
	const nodes = [...parsed.nodes]
		.map((node) => node.id)
		.sort()
		.join(",");
	const edges = [...parsed.edges]
		.map((edge) => `${edge.id}:${edge.source}:${edge.target}`)
		.sort()
		.join(";");
	return `${nodes}|${edges}|${JSON.stringify(parsed.camera)}|${JSON.stringify(volume)}`;
}

function usePlatformTopologyStore(controller: import("@semio-tech/framework-platform-core").Controller | undefined, instanceId: string): ReturnType<typeof createStore> | null {
	const payload = useControllerStore<PlatformTopologyPayload>(controller, platformTopologyStoreId(instanceId));
	const topologyStoreRef = React.useRef<ReturnType<typeof createStore> | null>(null);
	const lastStructureKeyRef = React.useRef<string | null>(null);
	const flatPayloadRef = React.useRef(payload?.flat);
	const volumePayloadRef = React.useRef(payload?.volume);
	flatPayloadRef.current = payload?.flat;
	volumePayloadRef.current = payload?.volume;
	const structureKey =
		flatPayloadRef.current && volumePayloadRef.current
			? platformTopologyStructureKey(flatPayloadRef.current, volumePayloadRef.current)
			: null;
	const [, setTopologyEpoch] = React.useState(0);
	React.useEffect(() => {
		if (!structureKey) {
			if (topologyStoreRef.current !== null) {
				topologyStoreRef.current = null;
				lastStructureKeyRef.current = null;
				setTopologyEpoch((epoch) => epoch + 1);
			}
			return;
		}
		const flatPayload = flatPayloadRef.current;
		const volumePayload = volumePayloadRef.current;
		if (!flatPayload || !volumePayload) {
			return;
		}
		const model = prepareTopologyModel(compose5d(parsePuzzle2dFixture(flatPayload)!, parseFixture(volumePayload)!));
		const existing = topologyStoreRef.current;
		if (existing) {
			if (lastStructureKeyRef.current !== structureKey) {
				const nextModel =
					instanceId.endsWith(":kit:wires")
						? mergeLiveForceGraphTopologyModel(model, existing.read().model)
						: model;
				existing.replaceModel(nextModel);
				lastStructureKeyRef.current = structureKey;
			}
			return;
		}
		topologyStoreRef.current = createStore(model);
		lastStructureKeyRef.current = structureKey;
		setTopologyEpoch((epoch) => epoch + 1);
	}, [structureKey, instanceId]);
	return topologyStoreRef.current;
}

/** @emoji 🎯 Maps FiveD flat/volume selection to `puzzle5dSelection` command payload. */
export function puzzle5dSelectionPayload(
	instanceId: string,
	presentation: Puzzle5dModel["presentation"],
	snapshot: Puzzle2dSelectionSnapshot | Puzzle3dSelectionSnapshot,
): { readonly instanceId: string; readonly puzzle2dIds: readonly string[] } {
	if (presentation === "flat") {
		return { instanceId, puzzle2dIds: (snapshot as Puzzle2dSelectionSnapshot).ids };
	}
	const volume = snapshot as Puzzle3dSelectionSnapshot;
	return { instanceId, puzzle2dIds: [...volume.objectIds, ...volume.vortexIds, ...volume.attractionIds] };
}

const BuiltinPuzzle5dKindRenderer: ComponentKindRenderer = ({ component, node, commandBus, platform }) => {
	const model = useStore(component as Puzzle5d);
	const instanceId = model.instanceId || node.surfaceId;
	const controller = platform ? getPlatformControllerById(platform, component.controllerId) : undefined;
	const topologyStore = usePlatformTopologyStore(controller, instanceId);
	const puzzle2dSelect = React.useMemo(
		() =>
			model.presentation === "flat"
				? {
						...(model.puzzle2dSelection !== undefined ? { selection: { ids: [...model.puzzle2dSelection] } } : {}),
						...(model.puzzle2dHoveredId !== undefined ? { hoveredId: model.puzzle2dHoveredId } : {}),
						onSelect: (snapshot: Puzzle2dSelectionSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dSelection", puzzle5dSelectionPayload(instanceId, "flat", snapshot));
						},
						...(instanceId.endsWith(":kit:wires")
							? {
									onActivate: (snapshot: Puzzle2dSelectionSnapshot) => {
										commandBus.dispatch(component.controllerId, "puzzle5dActivate", {
											instanceId,
											puzzle2dIds: snapshot.ids,
										});
									},
								}
							: {}),
						onHover: (payload: { readonly id: string | null }) => {
							commandBus.dispatch(component.controllerId, "puzzle5dHover", { instanceId, nodeId: payload.id });
						},
						onPreselect: (snapshot: Puzzle2dPreselectSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dPreselect", {
								instanceId,
								preselect: { ids: [...snapshot.ids], removedIds: [...snapshot.removedIds] },
							});
						},
					}
				: undefined,
		[commandBus, component.controllerId, instanceId, model.presentation, model.puzzle2dHoveredId, model.puzzle2dSelection],
	);
	const puzzle3dSelect = React.useMemo(
		() =>
			model.presentation === "volume"
				? {
						onSelect: (snapshot: Puzzle3dSelectionSnapshot) => {
							commandBus.dispatch(component.controllerId, "puzzle5dSelection", puzzle5dSelectionPayload(instanceId, "volume", snapshot));
						},
					}
				: undefined,
		[commandBus, component.controllerId, instanceId, model.presentation],
	);
	const fiveDMode = model.presentation === "volume" ? "3d" : "2d";
	if (model.emptyMessage) {
		return (
			<div
				className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
				data-surface-id={node.surfaceId}
			>
				{model.emptyMessage}
			</div>
		);
	}
	if (!topologyStore) {
		return (
			<div
				className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
				data-surface-id={node.surfaceId}
			>
				Topology loading…
			</div>
		);
	}
	return (
		<div
			className="absolute inset-0 min-h-0 min-w-0"
			data-surface-id={node.surfaceId}
			data-testid={`platform-five-d-${instanceId}`}
		>
			<StoreProvider store={topologyStore}>
				<FiveD
					instanceId={instanceId}
					graphPortMode={instanceId.endsWith(":kit:wires") || instanceId.endsWith(":diagram") ? "normal" : undefined}
					liveForceGraph={instanceId.endsWith(":kit:wires")}
					mode={fiveDMode}
					puzzle2d={puzzle2dSelect}
					puzzle3d={puzzle3dSelect}
				/>
			</StoreProvider>
		</div>
	);
};

const BuiltinPuzzle2dCanvas: React.FC<{ readonly node: UiPuzzle2dHostSurfaceNode }> = ({ node }) => (
	<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
		<Puzzle2dCanvas className="h-full w-full" />
	</div>
);

const BuiltinPuzzle5dCanvas: React.FC<{ readonly node: UiPuzzle5dHostSurfaceNode; readonly platform?: Platform }> = ({
	node,
	platform,
}) => {
	if (platform) {
		const registered = platform.getComponent(node.surfaceId);
		if (registered?.componentKind === "puzzle5d") {
			const KindRenderer = getComponentKindRendererRef?.("puzzle5d");
			if (KindRenderer) {
				return (
					<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId}>
						<KindRenderer
							component={registered as Component<unknown>}
							node={node}
							commandBus={platform.commandBus}
							layout="canvas"
							platform={platform}
						/>
					</div>
				);
			}
		}
	}
	return (
		<div
			className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
			data-surface-id={node.surfaceId}
		>
			Loading…
		</div>
	);
};

let getComponentKindRendererRef: ((kind: ComponentKind) => ComponentKindRenderer | undefined) | undefined;

/** @emoji 🌲 Builds puzzle fixture palette drag controller for declarative tree panels. */
export function buildPuzzle2dTreeDragController(dragByItemId: ReadonlyMap<string, Record<string, string>>): TreeDragAndDropController {
	return puzzle2dFixturePaletteTreeDragController(dragByItemId);
}

/** @emoji 🌲 Builds puzzle 3D fixture palette drag controller for declarative tree panels. */
export function buildPuzzle3dTreeDragController(dragByItemId: ReadonlyMap<string, Record<string, string>>): TreeDragAndDropController {
	return puzzle3dFixturePaletteTreeDragController(dragByItemId);
}

export { PUZZLE_2D_FIXTURE_DRAG_MIME };

/** @emoji 🧩 Registers puzzle component renderers and default surface hosts after lazy load. */
export function registerPlatformPuzzleIntegration(options: {
	readonly registerComponentKindRenderer: (kind: ComponentKind, renderer: ComponentKindRenderer) => void;
	readonly defaultComponentHosts: Partial<Record<ComponentKind, SurfaceBindingHost>>;
	readonly getComponentKindRenderer: (kind: ComponentKind) => ComponentKindRenderer | undefined;
}): void {
	getComponentKindRendererRef = options.getComponentKindRenderer;
	options.registerComponentKindRenderer("puzzle2d", BuiltinPuzzle2dKindRenderer);
	options.registerComponentKindRenderer("puzzle3d", BuiltinPuzzle3dKindRenderer);
	options.registerComponentKindRenderer("puzzle5d", BuiltinPuzzle5dKindRenderer);
	options.defaultComponentHosts.puzzle2d = BuiltinPuzzle2dCanvas as SurfaceBindingHost;
	options.defaultComponentHosts.puzzle5d = BuiltinPuzzle5dCanvas as SurfaceBindingHost;
}
