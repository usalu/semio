// #region 🧲Header
/** @emoji 🖥 Translates {@link UiNode} trees into React/DOM: semantic commands only; domain canvases via {@link registerUiScene3DSurfaceHost}. */
// #endregion 🧲Header

import type { CommandBus } from "@elements/ui-shell";
import type {
	UiBoardHostSurfaceNode,
	UiButtonNode,
	UiNode,
	UiTableHostSurfaceNode,
	UiScene3DHostSurfaceNode,
	UiSeparatorNode,
	UiStackNode,
	UiTextNode,
} from "@elements/ui-shell";
import { clsx, type ClassValue } from "clsx";
import * as React from "react";
import { twMerge } from "tailwind-merge";

function cn(...inputs: ClassValue[]): string {
	return twMerge(clsx(inputs));
}

//#region 🔖Scene3DRegistry
type Scene3DSurfaceHost = React.ComponentType<{ readonly node: UiScene3DHostSurfaceNode }>;

const scene3dSurfaceHosts = new Map<string, Scene3DSurfaceHost>();

/** @emoji 🧭 Binds a `surfaceId` from {@link UiScene3DHostSurfaceNode} to a host React canvas implementation. */
export function registerUiScene3DSurfaceHost(surfaceId: string, Component: Scene3DSurfaceHost): void {
	scene3dSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a surface binding (tests). */
export function unregisterUiScene3DSurfaceHost(surfaceId: string): void {
	scene3dSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖Scene3DRegistry

//#region 🔖BoardRegistry
type BoardSurfaceHost = React.ComponentType<{ readonly node: UiBoardHostSurfaceNode }>;

const boardSurfaceHosts = new Map<string, BoardSurfaceHost>();

/** @emoji 📋 Binds `surfaceId` from {@link UiBoardHostSurfaceNode} to a host board canvas. */
export function registerUiBoardSurfaceHost(surfaceId: string, Component: BoardSurfaceHost): void {
	boardSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a board surface binding (tests). */
export function unregisterUiBoardSurfaceHost(surfaceId: string): void {
	boardSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖BoardRegistry

//#region 🔖TableRegistry
type TableSurfaceHost = React.ComponentType<{ readonly node: UiTableHostSurfaceNode }>;

const tableSurfaceHosts = new Map<string, TableSurfaceHost>();

/** @emoji 📑 Binds `surfaceId` from {@link UiTableHostSurfaceNode} to a host table body. */
export function registerUiTableSurfaceHost(surfaceId: string, Component: TableSurfaceHost): void {
	tableSurfaceHosts.set(surfaceId, Component);
}

/** @emoji 🧹 Drops a table surface binding (tests). */
export function unregisterUiTableSurfaceHost(surfaceId: string): void {
	tableSurfaceHosts.delete(surfaceId);
}
//#endregion 🔖TableRegistry

//#region 🔖StackLayout
function stackClass(spec: UiStackNode): string {
	const dir = spec.direction === "horizontal" ? "flex-row" : "flex-col";
	const gap =
		spec.gap === "none"
			? "gap-0"
			: spec.gap === "tight"
				? "gap-1"
				: spec.gap === "relaxed"
					? "gap-4"
					: "gap-2";
	const pad = spec.padding === "none" ? "p-0" : "p-2";
	return cn("flex", dir, gap, pad, spec.direction === "vertical" ? "min-h-0 min-w-0" : "min-w-0");
}
//#endregion 🔖StackLayout

//#region 🔖Renderer
export interface UiRendererProps {
	readonly node: UiNode;
	readonly commandBus: CommandBus;
}

function renderText(node: UiTextNode): React.ReactElement {
	const dataProps = node.dataAttributes
		? Object.fromEntries(Object.entries(node.dataAttributes).map(([k, v]) => [`data-${k}`, v]))
		: {};
	return (
		<span
			className={cn(
				"text-muted-foreground px-1 text-xs",
				node.emphasize && "font-semibold uppercase tracking-wide",
			)}
			{...dataProps}
		>
			{node.value}
		</span>
	);
}

function renderButton(node: UiButtonNode, commandBus: CommandBus): React.ReactElement {
	const variant = node.style?.variant ?? "default";
	return (
		<button
			type="button"
			id={node.id}
			className={cn(
				"rounded-md border px-2 py-1 text-sm",
				variant === "danger" && "border-destructive text-destructive",
				variant === "success" && "border-green-600 text-green-700",
				variant === "subtle" && "border-transparent bg-muted/60",
				variant === "default" && "border-border bg-background",
			)}
			onClick={() => commandBus.dispatch(node.command.controllerId, node.command.command, node.command.args)}
		>
			{node.label}
		</button>
	);
}

function renderSeparator(_node: UiSeparatorNode, horizontalParent: boolean): React.ReactElement {
	return (
		<span
			role="separator"
			className={cn(
				"shrink-0 bg-border",
				horizontalParent ? "mx-1 h-4 w-px self-center" : "my-1 h-px w-full",
			)}
			aria-hidden
		/>
	);
}

function renderScene3d(node: UiScene3DHostSurfaceNode): React.ReactElement {
	const Host = scene3dSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported scene3d surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderBoard(node: UiBoardHostSurfaceNode): React.ReactElement {
	const Host = boardSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported board surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<Host node={node} />
		</div>
	);
}

function renderTable(node: UiTableHostSurfaceNode): React.ReactElement {
	const Host = tableSurfaceHosts.get(node.surfaceId);
	if (!Host) {
		return (
			<div className="flex flex-1 items-center justify-center p-4 text-xs text-muted-foreground">
				Unsupported table surface &quot;{node.surfaceId}&quot;
			</div>
		);
	}
	return (
		<div className="relative min-h-0 min-w-0 flex-1 overflow-auto">
			<Host node={node} />
		</div>
	);
}

function renderNode(node: UiNode, commandBus: CommandBus, horizontalParent: boolean): React.ReactElement {
	switch (node.type) {
		case "stack":
			return (
				<div className={cn(stackClass(node), node.direction === "vertical" && node.children.some((c) => c.type === "scene3d" || c.type === "board") && "relative min-h-0 flex-1")}>
					{node.children.map((child, index) => (
						<React.Fragment key={index}>{renderNode(child, commandBus, node.direction === "horizontal")}</React.Fragment>
					))}
				</div>
			);
		case "text":
			return renderText(node);
		case "button":
			return renderButton(node, commandBus);
		case "separator":
			return renderSeparator(node, horizontalParent);
		case "scene3d":
			return renderScene3d(node);
		case "board":
			return renderBoard(node);
		case "table":
			return renderTable(node);
		default:
			return (
				<div className="p-2 text-xs text-destructive">
					Unsupported UiNode {(node as { type?: string }).type ?? "unknown"}
				</div>
			);
	}
}

/** @emoji 🧩 Host entry: turns declarative {@link UiNode} trees into mounted React structure. */
export function UiRenderer({ node, commandBus }: UiRendererProps): React.ReactElement {
	return renderNode(node, commandBus, false);
}
//#endregion 🔖Renderer
