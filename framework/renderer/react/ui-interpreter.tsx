import type { ReactNode } from "react";
import type { CommandDescriptor, UiNode } from "./types.ts";
import { Canvas2dHost } from "./components/canvas-2d-host.tsx";
import { NodeGraphHost } from "./components/node-graph-host.tsx";
import { RasterHost } from "./components/raster-host.tsx";
import { TableHost } from "./components/table-host.tsx";
import { TextEditorHost } from "./components/text-editor-host.tsx";
import { World3dHost } from "./components/world-3d-host.tsx";

export type UiInterpreterContext = {
	readonly onCommand: (command: CommandDescriptor) => void;
};

export function interpretUiNode(node: UiNode, context: UiInterpreterContext): ReactNode {
	switch (node.type) {
		case "stack":
			return (
				<div
					className={`semio-ui-stack semio-ui-stack--${node.direction}`}
					style={{ display: "flex", flexDirection: node.direction === "horizontal" ? "row" : "column", gap: "0.5rem" }}
				>
					{node.children.map((child, index) => (
						<div key={index}>{interpretUiNode(child, context)}</div>
					))}
				</div>
			);
		case "text":
			return <p className={node.emphasize ? "font-semibold" : undefined}>{node.value}</p>;
		case "button":
			return (
				<button type="button" onClick={() => context.onCommand(node.command)}>
					{node.label}
				</button>
			);
		case "separator":
			return <hr />;
		case "input":
			return (
				<input
					id={node.id}
					value={node.value}
					placeholder={node.placeholder}
					onChange={(event) =>
						context.onCommand({
							...node.onChange,
							args: { value: event.target.value },
						})
					}
				/>
			);
		case "select":
			return (
				<select
					id={node.id}
					value={node.value}
					onChange={(event) =>
						context.onCommand({
							...node.onChange,
							args: { value: event.target.value },
						})
					}
				>
					{node.items.map((item) => (
						<option key={item.id} value={item.value}>
							{item.label}
						</option>
					))}
				</select>
			);
		case "toggle":
			return (
				<button
					type="button"
					aria-pressed={node.pressed}
					onClick={() =>
						context.onCommand({
							...node.onChange,
							args: { pressed: !node.pressed },
						})
					}
				>
					{node.label ?? node.text ?? node.iconId}
				</button>
			);
		case "slider":
			return (
				<label>
					{node.label}
					<input
						type="range"
						min={node.min}
						max={node.max}
						step={node.step}
						value={node.value}
						onChange={(event) =>
							context.onCommand({
								...node.onChange,
								args: { value: Number(event.target.value) },
							})
						}
					/>
				</label>
			);
		case "section":
			return (
				<section>
					<h3>{node.title}</h3>
					{node.children.map((child, index) => (
						<div key={index}>{interpretUiNode(child, context)}</div>
					))}
				</section>
			);
		case "tree":
			return (
				<ul>
					{node.items.map((item) => (
						<li key={item.id}>
							{item.label}
							{item.children.map((child, index) => (
								<div key={index}>{interpretUiNode(child, context)}</div>
							))}
						</li>
					))}
				</ul>
			);
		case "componentScene":
			switch (node.componentKind) {
				case "canvas-2d":
					return <Canvas2dHost node={node} onCommand={context.onCommand} />;
				case "world-3d":
					return <World3dHost node={node} onCommand={context.onCommand} />;
				case "node-graph":
					return <NodeGraphHost node={node} onCommand={context.onCommand} />;
				case "text-editor":
					return <TextEditorHost node={node} onCommand={context.onCommand} />;
				case "table":
					return <TableHost node={node} onCommand={context.onCommand} />;
				case "raster":
					return <RasterHost node={node} onCommand={context.onCommand} />;
				default:
					return <p>Unknown component: {node.componentKind}</p>;
			}
	}
}
