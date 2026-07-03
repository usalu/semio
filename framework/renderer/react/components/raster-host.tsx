import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function RasterHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.raster;
	if (!scene) return <div className="semio-raster-empty">No raster scene</div>;
	return (
		<img
			className="semio-raster-host"
			data-surface-id={node.surfaceId}
			alt="Raster viewport"
			width={scene.width}
			height={scene.height}
			src={`data:image/png;base64,${scene.pixelsBase64}`}
			onClick={() =>
				onCommand({
					controllerId: node.controllerId,
					command: "rasterClick",
					args: { surfaceId: node.surfaceId },
				})
			}
		/>
	);
}
