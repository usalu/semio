// #region 🧲Header
/** @emoji 🧭 `@semio-tech/framework-core` — shared canvas pick helpers, layout factories, and inspector utilities for UI renderers. */
// #endregion 🧲Header

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_HIERARCHY = "hierarchy";

export const FRAMEWORK_PANEL_TAB_HIERARCHY_ID = "framework.panel.hierarchy";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL = "Hierarchy";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID = "framework.panel.hierarchy";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

export type CanvasPickTarget = {
	readonly domain: string;
	readonly id: string;
	readonly generality: number;
	readonly label: string;
	readonly kind?: string;
};

export type CanvasPickRequest = {
	readonly targets: readonly CanvasPickTarget[];
	readonly client: { readonly x: number; readonly y: number };
	readonly modifiers?: Readonly<Record<string, boolean>>;
};

export type CanvasHoverFocus = {
	readonly sourceId: string;
	readonly target: CanvasPickTarget | null;
};

export type CommandDescriptor = {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: unknown;
};

export type WindowLayoutWindowNode = {
	readonly kind: "window";
	readonly windowKindId: string;
	readonly title?: string;
	readonly instanceId?: string;
	readonly templateId?: string;
};

export type WindowLayoutStackNode = {
	readonly kind: "stack";
	readonly size?: number;
	readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
	readonly kind: "row" | "column";
	readonly size?: number;
	readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
	readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly layout: WindowLayout;
	readonly origin: "builtin" | "user";
	readonly groupPath?: readonly string[];
};

export enum Expertise {
	BEGINNER = "beginner",
	NORMAL = "normal",
	EXPERT = "expert",
}

export type ToolLeaf =
	| { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
	| {
			readonly id: string;
			readonly kind: "button";
			readonly iconId: string;
			readonly label?: string;
			readonly text?: string;
			readonly title?: string;
			readonly order?: number;
			readonly disabled?: boolean;
			readonly controllerId?: string;
			readonly command?: string;
			readonly args?: unknown;
	  }
	| {
			readonly id: string;
			readonly kind: "toggle";
			readonly iconId: string;
			readonly label?: string;
			readonly text?: string;
			readonly title?: string;
			readonly order?: number;
			readonly pressed?: boolean;
			readonly disabled?: boolean;
			readonly controllerId?: string;
			readonly command?: string;
			readonly args?: unknown;
	  };

export type ToolNode =
	| ToolLeaf
	| {
			readonly id: string;
			readonly kind: "collection";
			readonly iconId: string;
			readonly label?: string;
			readonly text?: string;
			readonly title?: string;
			readonly order?: number;
			readonly disabled?: boolean;
			readonly children: readonly ToolNode[];
	  };

export type UiSectionNode = {
	readonly type: "section";
	readonly id: string;
	readonly label?: string;
	readonly defaultOpen?: boolean;
	readonly children: readonly UiNode[];
};

export type UiTreeItemNode = {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	readonly icon?: string;
	readonly selected?: boolean;
	readonly defaultOpen?: boolean;
	readonly command?: CommandDescriptor;
	readonly draggable?: boolean;
	readonly dragData?: Readonly<Record<string, string>>;
	readonly items?: readonly UiTreeItemNode[];
	readonly control?: UiControlNode;
	readonly isHidden?: boolean;
};

export type UiTreeSectionNode = {
	readonly id: string;
	readonly label?: string;
	readonly defaultOpen?: boolean;
	readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
	readonly type: "tree";
	readonly sections: readonly UiTreeSectionNode[];
	readonly selectedIds?: readonly string[];
	readonly highlightedIds?: readonly string[];
	readonly selectionChange?: CommandDescriptor;
};

export type UiControlNode =
	| UiInputNode
	| UiSelectNode
	| UiToggleNode
	| UiVec3Node
	| UiButtonNode
	| UiKeyValueNode
	| UiSliderNode
	| UiNumberStepperNode
	| UiRingNode
	| UiIconSelectNode;

export type UiInputNode = {
	readonly type: "input";
	readonly id: string;
	readonly inputKind: "text" | "number";
	readonly value: string;
	readonly placeholder?: string;
	readonly commit?: "change" | "blur";
	readonly onChange: CommandDescriptor;
};

export type UiSelectNode = {
	readonly type: "select";
	readonly id: string;
	readonly value: string;
	readonly items: readonly { readonly value: string; readonly label: string }[];
	readonly placeholder?: string;
	readonly onChange: CommandDescriptor;
};

export type UiToggleNode = {
	readonly type: "toggle";
	readonly id: string;
	readonly iconId: string;
	readonly pressed: boolean;
	readonly text?: string;
	readonly onChange: CommandDescriptor;
};

export type UiVec3Node = {
	readonly type: "vec3";
	readonly id: string;
	readonly value: readonly [number, number, number] | null;
	readonly onChange: CommandDescriptor;
};

export type UiKeyValueNode = {
	readonly type: "keyValue";
	readonly entries: readonly { readonly label: string; readonly value: string }[];
};

export type UiSliderNode = {
	readonly type: "slider";
	readonly id: string;
	readonly value: number;
	readonly min: number;
	readonly max: number;
	readonly step: number;
	readonly onChange: CommandDescriptor;
};

export type UiNumberStepperNode = {
	readonly type: "numberStepper";
	readonly id: string;
	readonly value: number;
	readonly step: number;
	readonly uniform: boolean;
	readonly onAbsolute: CommandDescriptor;
	readonly onDelta: CommandDescriptor;
};

export type UiRingNode = {
	readonly type: "ring";
	readonly id: string;
	readonly orbId: string;
	readonly t: number;
	readonly disabled?: boolean;
	readonly onChange: CommandDescriptor;
};

export type UiIconSelectNode = {
	readonly type: "iconSelect";
	readonly id: string;
	readonly value: string;
	readonly uniform: boolean;
	readonly classifierKind: "puzzle2d";
	readonly onChange: CommandDescriptor;
};

export type UiFieldNode = {
	readonly type: "field";
	readonly id: string;
	readonly label: string;
	readonly child: UiControlNode;
};

export type UiButtonNode = {
	readonly type: "button";
	readonly id?: string;
	readonly iconId: string;
	readonly label: string;
	readonly command: CommandDescriptor;
};

export type UiTextNode = {
	readonly type: "text";
	readonly value: string;
	readonly emphasize?: boolean;
};

export type UiNode =
	| { readonly type: "stack"; readonly direction: string; readonly gap?: string; readonly padding?: string; readonly children: readonly UiNode[] }
	| UiTextNode
	| UiButtonNode
	| { readonly type: "separator" }
	| UiSectionNode
	| UiInputNode
	| UiSelectNode
	| UiToggleNode
	| UiVec3Node
	| UiKeyValueNode
	| UiSliderNode
	| UiNumberStepperNode
	| UiRingNode
	| UiIconSelectNode
	| UiFieldNode
	| UiTreeNode;

export type UiInspectorFieldGroup = {
	readonly id: string;
	readonly label: string;
	readonly defaultOpen?: boolean;
	readonly fields: readonly UiNode[];
};

export function canvasPickTargetKey(target: CanvasPickTarget): string {
	return `${target.domain}:${target.id}`;
}

/** @emoji 🪪 Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
	const colon = key.indexOf(":");
	if (colon < 0) return null;
	return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
	return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
	if (targets.length === 0) return null;
	return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
	return { sourceId, target };
}

export function createWindowLayout(
	windowKindId: string,
	title?: string,
	options?: { readonly instanceId?: string; readonly templateId?: string },
): WindowLayoutWindowNode {
	return {
		kind: "window",
		windowKindId,
		...(title ? { title } : {}),
		...(options?.instanceId ? { instanceId: options.instanceId } : {}),
		...(options?.templateId ? { templateId: options.templateId } : {}),
	};
}

export function createStackLayout(windowKindIds: readonly string[], titles?: readonly string[]): WindowLayout {
	return {
		root: {
			kind: "stack",
			children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
		},
	};
}

export function createDefaultLayout(
	windowIds: readonly string[],
	direction: "row" | "column" = "row",
	sizes?: readonly number[],
	titles?: readonly string[],
): WindowLayout {
	return {
		root: {
			kind: direction,
			children: windowIds.map((id, index) => ({
				kind: "stack" as const,
				...(sizes?.[index] !== undefined ? { size: sizes[index] } : {}),
				children: [createWindowLayout(id, titles?.[index] ?? id)],
			})),
		},
	};
}

export function createTabStackLayout(windowIds: readonly string[], titles?: readonly string[]): WindowLayout {
	return createStackLayout(windowIds, titles);
}

export function createNamedLayout(
	id: string,
	label: string,
	layout: WindowLayout,
	origin: NamedLayout["origin"] = "builtin",
	iconId?: string,
	groupPath?: readonly string[],
): NamedLayout {
	return {
		id,
		label,
		layout,
		origin,
		...(iconId ? { iconId } : {}),
		...(groupPath?.length ? { groupPath } : {}),
	};
}

export function uiInspectorAllEqual<T>(values: readonly T[]): boolean {
	if (values.length <= 1) return true;
	const first = values[0];
	for (let index = 1; index < values.length; index += 1) {
		if (values[index] !== first) return false;
	}
	return true;
}

export function uiInspectorMixedText(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
	const uniform = uiInspectorAllEqual(values);
	return { value: uniform ? (values[0] ?? "") : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}

export function uiInspectorMixedNumber(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
	const uniform = uiInspectorAllEqual(values);
	return { value: uniform ? (values[0] ?? 0) : Number.NaN, uniform };
}

export function uiInspectorMixedSelect(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
	return uiInspectorMixedText(values);
}

export function uiInspectorMixedToggle(values: readonly boolean[]): { readonly pressed: boolean; readonly uniform: boolean } {
	const uniform = uiInspectorAllEqual(values);
	return { pressed: uniform ? (values[0] ?? false) : false, uniform };
}

export function uiInspectorMixedSlider(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
	return uiInspectorMixedNumber(values);
}

export function uiInspectorMixedVec3(
	values: readonly (readonly [number, number, number])[],
): { readonly value: readonly [number, number, number] | null; readonly uniform: boolean } {
	const uniform = uiInspectorAllEqual(values.map((row) => JSON.stringify(row)));
	return { value: uniform && values[0] ? values[0] : null, uniform };
}

export function uiInspectorGroupsToTree(groups: readonly UiInspectorFieldGroup[]): UiTreeNode {
	return uiDeclarativeSectionsToTree(
		groups
			.filter((group) => group.fields.length > 0)
			.map((group) => ({
				type: "section" as const,
				id: group.id,
				label: group.label,
				defaultOpen: group.defaultOpen ?? true,
				children: group.fields,
			})),
	);
}

export function uiDeclarativeSectionsToTree(sections: readonly UiSectionNode[]): UiTreeNode {
	const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
		id: section.id,
		label: section.label,
		defaultOpen: section.defaultOpen ?? true,
		items: section.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${section.id}.${index}`)),
	}));
	return {
		type: "tree",
		sections: treeSections.length ? treeSections : [{ id: "empty", items: [{ id: "empty", label: "—" }] }],
	};
}

function uiDeclarativeChildToTreeItem(node: UiNode, fallbackId: string): UiTreeItemNode {
	if (node.type === "text") return { id: `${fallbackId}.text`, label: node.value };
	if (node.type === "field") {
		if (node.child.type === "text") return { id: node.id, label: node.label, description: node.child.value };
		return { id: node.id, label: node.label, control: node.child };
	}
	if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
	if (
		node.type === "input" ||
		node.type === "select" ||
		node.type === "toggle" ||
		node.type === "vec3" ||
		node.type === "keyValue" ||
		node.type === "slider" ||
		node.type === "numberStepper" ||
		node.type === "ring" ||
		node.type === "iconSelect"
	) {
		return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
	}
	if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
	return { id: fallbackId, label: node.type };
}
