import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

export function TextEditorHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.textEditor;
	if (!scene) return <div className="semio-text-editor-empty">No editor scene</div>;
	return (
		<textarea
			className="semio-text-editor-host"
			data-surface-id={node.surfaceId}
			value={scene.buffer}
			style={{ width: "100%", height: "100%", minHeight: "16rem", fontFamily: "monospace" }}
			onChange={(event) =>
				onCommand({
					controllerId: node.controllerId,
					command: "setDocument",
					args: { document: event.target.value },
				})
			}
		/>
	);
}
