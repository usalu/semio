// #region 🧲Header
/** @emoji 🖼️ Sketchpad {@link registerComponentKindRenderer} overrides (FiveD + topology stores). */
// #endregion 🧲Header

//#region 🔌Adapters
import {
	Cad,
	Puzzle5d,
	registerComponentKindRenderer,
	useStore,
	type ComponentKindRenderer,
} from "@framework/platform/renderer/react";
import type { BoardSelectionSnapshot } from "@puzzle/2d/react";
import { FiveD, TopologyStoreProvider } from "@puzzle/5d/react";
import { useMemo } from "react";
import {
	findTypeInKit,
	getSketchpadShellController,
	parseSketchpadCadInstanceId,
	sketchpadApplyBoardSelection,
	sketchpadKitFileUrlById,
	sketchpadResolvePieceMeshUrl,
	sketchpadTopologyStoreId,
	type SketchpadTopologyStoreBridge,
} from "./index.ts";
//#endregion 🔌Adapters

const SketchpadPuzzle5dKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Puzzle5d);
	const instanceId = model.instanceId || node.surfaceId;
	const bridge = getSketchpadShellController()?.getStore<unknown>(
		sketchpadTopologyStoreId(instanceId),
	) as SketchpadTopologyStoreBridge | undefined;
	const flat = useMemo(
		() =>
			model.presentation === "flat"
				? {
						onSelect: (snapshot: BoardSelectionSnapshot) => {
							sketchpadApplyBoardSelection(instanceId, snapshot.ids);
						},
					}
				: undefined,
		[instanceId, model.presentation],
	);
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
	if (!bridge) {
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
		<div className="absolute inset-0 min-h-0 min-w-0" data-surface-id={node.surfaceId} data-testid={`sketchpad-five-d-${instanceId}`}>
			<TopologyStoreProvider store={bridge.inner}>
				<FiveD mode={model.presentation === "volume" ? "volume" : "flat"} instanceId={instanceId} flat={flat} />
			</TopologyStoreProvider>
		</div>
	);
};

const SketchpadCadKindRenderer: ComponentKindRenderer = ({ component, node }) => {
	const model = useStore(component as Cad);
	const { kitId, typeId } = parseSketchpadCadInstanceId(model.instanceId ?? "");
	const kit = kitId ? getSketchpadShellController()?.getKitStore(kitId)?.getSnapshot().kit : undefined;
	const type = kit && typeId ? findTypeInKit(kit, typeId) : undefined;
	if (model.emptyMessage && !type) {
		return (
			<div
				className="absolute inset-0 flex items-center justify-center p-2 text-xs text-muted-foreground"
				data-surface-id={node.surfaceId}
			>
				{model.emptyMessage}
			</div>
		);
	}
	const reps = type?.representations?.length ?? 0;
	const connectors = type?.connectors?.length ?? 0;
	return (
		<div
			className="absolute inset-0 flex flex-col gap-2 p-4 text-sm"
			data-surface-id={node.surfaceId}
			data-testid={`sketchpad-cad-${kitId ?? "none"}-${typeId ?? "none"}`}
		>
			<div className="font-medium">{type?.name ?? typeId ?? "Type"}</div>
			<div className="text-muted-foreground text-xs">
				{reps} representation(s) · {connectors} connector(s)
			</div>
			<div className="text-muted-foreground text-xs">CAD authoring via @cad/js connects when representations are bound to mesh files.</div>
		</div>
	);
};

let sketchpadKindRenderersRegistered = false;

/** @emoji 🧩 Registers sketchpad puzzle5d renderer (call before {@link mountPlatform}). */
export function registerSketchpadComponentKindRenderers(): void {
	if (sketchpadKindRenderersRegistered) return;
	sketchpadKindRenderersRegistered = true;
	registerComponentKindRenderer("puzzle5d", SketchpadPuzzle5dKindRenderer);
	registerComponentKindRenderer("cad", SketchpadCadKindRenderer);
}
