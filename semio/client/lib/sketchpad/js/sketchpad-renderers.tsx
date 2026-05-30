// #region 🧲Header
/** @emoji 🖼️ Sketchpad {@link registerComponentKindRenderer} overrides (FiveD + topology stores). */
// #endregion 🧲Header

//#region 🔌Adapters
import {
	Puzzle5d,
	registerComponentKindRenderer,
	useStore,
	type ComponentKindRenderer,
} from "@framework/platform/renderer/react";
import { FiveD, TopologyStoreProvider } from "@puzzle/5d/react";
import {
	getSketchpadShellController,
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
				<FiveD mode={model.presentation === "volume" ? "volume" : "flat"} instanceId={instanceId} />
			</TopologyStoreProvider>
		</div>
	);
};

let sketchpadKindRenderersRegistered = false;

/** @emoji 🧩 Registers sketchpad puzzle5d renderer (call before {@link mountPlatform}). */
export function registerSketchpadComponentKindRenderers(): void {
	if (sketchpadKindRenderersRegistered) return;
	sketchpadKindRenderersRegistered = true;
	registerComponentKindRenderer("puzzle5d", SketchpadPuzzle5dKindRenderer);
}
