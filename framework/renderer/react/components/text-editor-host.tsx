import { useMemo } from "react";
import { Textarea } from "@semio-tech/ui-react";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

//#region TextEditorHost
export function TextEditorHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.textEditor;
	const writerDocument = useMemo((): WriterDocument | null => {
		if (!scene || scene.language !== "wire") return null;
		return createWriterDocument({
			id: node.surfaceId,
			languageId: "wire",
			text: scene.buffer,
		});
	}, [node.surfaceId, scene]);

	if (!scene) return <div className="semio-text-editor-empty">No editor scene</div>;

	if (writerDocument) {
		return (
			<div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col bg-canvas" data-surface-id={node.surfaceId}>
				<WriterCanvas
					className="h-full min-h-0"
					document={writerDocument}
					onChange={(next) =>
						onCommand({
							controllerId: node.controllerId,
							command: "setDocument",
							args: { surfaceId: node.surfaceId, document: next.text },
						})
					}
				/>
			</div>
		);
	}

	return (
		<div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col bg-canvas" data-surface-id={node.surfaceId}>
			<Textarea
				className="min-h-0 flex-1 resize-none font-mono text-xs"
				id={`${node.surfaceId}.editor`}
				lazy
				rows={24}
				value={scene.buffer}
				placeholder={scene.language ? `${scene.language} document` : "Document"}
				onLazyChange={(value) =>
					onCommand({
						controllerId: node.controllerId,
						command: "setDocument",
						args: { surfaceId: node.surfaceId, document: value },
					})
				}
			/>
		</div>
	);
}
//#endregion TextEditorHost
