import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function World3dHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.world3d;
	if (!scene) return <div className="semio-world-3d-empty">No world scene</div>;
	return (
		<div
			className="semio-world-3d-host"
			data-surface-id={node.surfaceId}
			style={{ width: "100%", height: "100%", minHeight: "24rem" }}
			onPointerDown={() =>
				onCommand({
					controllerId: node.controllerId,
					command: "worldPointerDown",
					args: { surfaceId: node.surfaceId },
				})
			}
		>
			<pre className="sr-only">{scene.instancesJson}</pre>
		</div>
	);
}
