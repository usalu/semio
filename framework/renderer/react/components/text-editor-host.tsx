import { useMemo, type ReactNode } from "react";
import { Textarea } from "@semio-tech/ui-react";
import type { CommandDescriptor, UiComponentSceneNode } from "../types.ts";

//#region TextEditorTypes
type GrammarToken = {
	readonly class: string;
	readonly start: number;
	readonly end: number;
};

type EditorDiagnostic = {
	readonly start: number;
	readonly end: number;
	readonly severity?: string;
	readonly message: string;
};

const TOKEN_CLASS_COLORS: Record<string, string> = {
	keyword: "text-sky-400",
	string: "text-emerald-400",
	number: "text-amber-400",
	operator: "text-violet-400",
	ident: "text-foreground",
};
//#endregion TextEditorTypes

//#region HighlightedBuffer
function HighlightedBuffer({ buffer, tokens }: { readonly buffer: string; readonly tokens: readonly GrammarToken[] }) {
	if (tokens.length === 0) {
		return <span className="whitespace-pre-wrap font-mono text-xs text-foreground">{buffer}</span>;
	}
	const parts: ReactNode[] = [];
	let cursor = 0;
	for (const token of tokens) {
		if (token.start > cursor) {
			parts.push(<span key={`plain-${cursor}`}>{buffer.slice(cursor, token.start)}</span>);
		}
		const color = TOKEN_CLASS_COLORS[token.class] ?? "text-foreground";
		parts.push(
			<span key={`token-${token.start}-${token.end}`} className={`font-mono text-xs ${color}`}>
				{buffer.slice(token.start, token.end)}
			</span>,
		);
		cursor = Math.max(cursor, token.end);
	}
	if (cursor < buffer.length) {
		parts.push(<span key={`tail-${cursor}`}>{buffer.slice(cursor)}</span>);
	}
	return <div className="pointer-events-none absolute inset-0 overflow-hidden whitespace-pre-wrap p-3">{parts}</div>;
}
//#endregion HighlightedBuffer

//#region TextEditorHost
export function TextEditorHost({
	node,
	onCommand,
}: {
	readonly node: UiComponentSceneNode;
	readonly onCommand: (command: CommandDescriptor) => void;
}) {
	const scene = node.textEditor;
	const tokens = useMemo((): readonly GrammarToken[] => {
		if (!scene?.tokensJson) return [];
		try {
			return JSON.parse(scene.tokensJson) as GrammarToken[];
		} catch {
			return [];
		}
	}, [scene?.tokensJson]);
	const diagnostics = useMemo((): readonly EditorDiagnostic[] => {
		if (!scene?.diagnosticsJson) return [];
		try {
			return JSON.parse(scene.diagnosticsJson) as EditorDiagnostic[];
		} catch {
			return [];
		}
	}, [scene?.diagnosticsJson]);

	if (!scene) return <div className="semio-text-editor-empty">No editor scene</div>;

	return (
		<div className="semio-text-editor-host flex h-full min-h-[16rem] w-full flex-col bg-canvas" data-surface-id={node.surfaceId}>
			<div className="relative min-h-0 flex-1">
				<HighlightedBuffer buffer={scene.buffer} tokens={tokens} />
				<Textarea
					className="relative min-h-0 flex-1 resize-none bg-transparent font-mono text-xs text-transparent caret-foreground"
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
			{diagnostics.length > 0 ? (
				<div className="border-t border-border px-3 py-2 text-[11px] text-muted-foreground">
					{diagnostics.slice(0, 4).map((diag, index) => (
						<div key={`${diag.start}-${diag.end}-${index}`} className="truncate">
							{diag.severity ? `[${diag.severity}] ` : ""}
							{diag.message}
						</div>
					))}
				</div>
			) : null}
		</div>
	);
}
//#endregion TextEditorHost
