import { useMemo, useState, type ReactElement } from "react";
import {
	ButtonGroup,
	ButtonGroupItem,
	Icon,
	ToolbarDivider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	Toggle,
	type IconName,
} from "@semio-tech/ui-react";
import { ICONS } from "@semio-tech/ui-asset";
import type { CommandDescriptor, ToolLeaf, ToolNode } from "./types.ts";

type ToolTreeProps = {
	readonly tools: readonly ToolNode[];
	readonly onCommand: (command: CommandDescriptor) => void;
};

function resolveLeafCommand(
	node: ToolLeaf | Extract<ToolNode, { readonly kind: "button" | "toggle" }>,
): CommandDescriptor | null {
	if ("onPress" in node && node.onPress) return node.onPress;
	if ("onChange" in node && node.onChange) return node.onChange;
	if (node.kind === "button" || node.kind === "toggle") {
		if (!node.command || !node.controllerId) return null;
		return { controllerId: node.controllerId, command: node.command, args: node.args as Record<string, unknown> | undefined };
	}
	return null;
}

function toolIcon(iconId: string): IconName {
	return iconId in ICONS ? (iconId as IconName) : "circle";
}

function renderToolLeaf(node: ToolNode, onCommand: (command: CommandDescriptor) => void): ReactElement | null {
	if (node.kind === "separator") return <ToolbarDivider key={node.id} />;
	if (node.kind === "button") {
		const command = resolveLeafCommand(node);
		if (!command) return null;
		return (
			<ToolbarItem key={node.id}>
				<ButtonGroupItem
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					disabled={node.disabled}
					onClick={() => onCommand(command)}
				>
					<Icon icon={toolIcon(node.iconId)} size="small" />
				</ButtonGroupItem>
			</ToolbarItem>
		);
	}
	if (node.kind === "toggle") {
		const command = resolveLeafCommand(node);
		if (!command) return null;
		return (
			<ToolbarItem key={node.id}>
				<Toggle
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					icon={<Icon icon={toolIcon(node.iconId)} size="small" />}
					pressed={node.pressed ?? false}
					disabled={node.disabled}
					onPressedChange={() => onCommand(command)}
				/>
			</ToolbarItem>
		);
	}
	return null;
}

function ToolCollection({
	node,
	onCommand,
}: {
	readonly node: Extract<ToolNode, { readonly kind: "collection" }>;
	readonly onCommand: (command: CommandDescriptor) => void;
}): ReactElement {
	const [open, setOpen] = useState(false);
	const leaves = node.children.filter((child) => child.kind !== "collection");
	return (
		<ToolbarGroup key={node.id}>
			<ToolbarItem>
				<Toggle
					aria-label={node.title ?? node.label ?? node.id}
					title={node.title ?? node.label}
					icon={<Icon icon={toolIcon(node.iconId)} size="small" />}
					pressed={open}
					disabled={node.disabled}
					onPressedChange={setOpen}
				/>
			</ToolbarItem>
			{open
				? leaves.map((child) => {
						if (child.kind === "separator") return <ToolbarDivider key={child.id} />;
						if (child.kind === "button") return renderToolLeaf(child, onCommand);
						if (child.kind === "toggle") return renderToolLeaf(child, onCommand);
						return null;
					})
				: null}
		</ToolbarGroup>
	);
}

export function ToolTree({ tools, onCommand }: ToolTreeProps): ReactElement | null {
	const content = useMemo(() => {
		if (!tools.length) return null;
		return (
			<ToolbarZone>
				<ButtonGroup>
					{tools.map((node) => {
						if (node.kind === "collection") {
							return <ToolCollection key={node.id} node={node} onCommand={onCommand} />;
						}
						return renderToolLeaf(node, onCommand);
					})}
				</ButtonGroup>
			</ToolbarZone>
		);
	}, [onCommand, tools]);
	return content;
}
