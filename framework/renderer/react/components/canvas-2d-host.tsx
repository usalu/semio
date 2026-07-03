import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function Canvas2dHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.canvas2d;
	if (!scene) return <div className="semio-canvas-2d-empty">No canvas scene</div>;
	return (
		<div
			className="semio-canvas-2d-host"
			data-surface-id={node.surfaceId}
			data-controller-id={node.controllerId}
			style={{ width: "100%", height: "100%", minHeight: "24rem", background: "var(--semio-canvas-bg, #111)" }}
			onPointerDown={() =>
				onCommand({
					controllerId: node.controllerId,
					command: "canvasPointerDown",
					args: { surfaceId: node.surfaceId },
				})
			}
		>
			<canvas className="semio-canvas-2d" style={{ width: "100%", height: "100%" }} />
			<pre className="sr-only">{scene.layersJson}</pre>
		</div>
	);
}
