import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function NodeGraphHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.nodeGraph;
	if (!scene) return <div className="semio-node-graph-empty">No graph scene</div>;
	return (
		<div
			className="semio-node-graph-host"
			data-surface-id={node.surfaceId}
			style={{ width: "100%", height: "100%", minHeight: "24rem", border: "1px solid var(--semio-border)" }}
			onPointerDown={() =>
				onCommand({
					controllerId: node.controllerId,
					command: "graphPointerDown",
					args: { surfaceId: node.surfaceId },
				})
			}
		>
			<pre style={{ fontSize: "0.75rem", opacity: 0.6 }}>{scene.nodesJson.slice(0, 200)}</pre>
		</div>
	);
}
