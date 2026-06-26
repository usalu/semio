// #region 🧱Header
/** 🧱 `@semio-tech/framework-platform-core` — Renderer-agnostic platform shell: {@link Platform} → {@link AppRuntime} → {@link ModeRuntime}, declarative {@link UiNode} bodies, {@link PluginHost}, {@link SurfaceRouter}, and {@link PlatformDefinition} + {@link SurfaceDefinition} for contribution routing. */
// #endregion 🧱Header

export * from "@semio-tech/framework-core";

import {
	BaseAppRuntime,
	BaseModeRuntime,
	BaseWindowKindRuntime,
	CommandBus,
	Controller,
	Store,
	Platform,
	createTabStackLayout,
	mergeAppTools,
	mergeById,
	mergeNamedLayouts,
	mergeSearchItems,
	resolveMode,
	type AppTools,
	type Disposable,
	type FindItem,
	type FooterItem,
	type SurfaceComponent,
	type PlatformSubscriber,
	type SearchItemSpec,
	type SideTabSpec,
	type NamedLayout,
	type WindowLayout,
	type WindowMeasure,
} from "@semio-tech/framework-core";

//#region 🔖UiNode
export interface UiStackNode {
	readonly type: "stack";
	readonly direction: "horizontal" | "vertical";
	readonly gap?: "none" | "tight" | "standard" | "relaxed";
	readonly padding?: "none" | "standard";
	readonly children: readonly UiNode[];
}

export type { UiButtonNode, UiSeparatorNode, UiTextNode } from "@semio-tech/framework-core";

export interface UiTextNode {
	readonly type: "text";
	readonly value: string;
	readonly emphasize?: boolean;
	readonly dataAttributes?: Readonly<Record<string, string>>;
}

export interface UiButtonNode {
	readonly type: "button";
	readonly id?: string;
	readonly iconId: string;
	readonly label: string;
	readonly command: CommandDescriptor;
	readonly style?: StyleSpec;
}

export interface UiSeparatorNode {
	readonly type: "separator";
}

/** @emoji ✏️ Text or number input bound to a command. */
export interface UiInputNode {
	readonly type: "input";
	readonly id: string;
	readonly inputKind: "text" | "number";
	readonly value: string;
	readonly placeholder?: string;
	readonly commit?: "change" | "blur";
	readonly onChange: CommandDescriptor;
}

/** @emoji 📋 Select control bound to a command (`value` in args). */
export interface UiSelectNode {
	readonly type: "select";
	readonly id: string;
	readonly value: string;
	readonly items: readonly { readonly value: string; readonly label: string }[];
	readonly placeholder?: string;
	readonly onChange: CommandDescriptor;
}

/** @emoji 🔘 Toggle control bound to a command (`pressed` in args). */
export interface UiToggleNode {
	readonly type: "toggle";
	readonly id: string;
	readonly iconId: string;
	readonly pressed: boolean;
	readonly text?: string;
	readonly onChange: CommandDescriptor;
}

/** @emoji 📐 Three-axis numeric row; `value` null renders mixed placeholder. */
export interface UiVec3Node {
	readonly type: "vec3";
	readonly id: string;
	readonly value: readonly [number, number, number] | null;
	readonly onChange: CommandDescriptor;
}

/** @emoji 📋 Read-only label/value rows. */
export interface UiKeyValueNode {
	readonly type: "keyValue";
	readonly entries: readonly { readonly label: string; readonly value: string }[];
}

/** @emoji 🎚️ Slider control bound to a command (`value` in args). */
export interface UiSliderNode {
	readonly type: "slider";
	readonly id: string;
	readonly value: number;
	readonly min: number;
	readonly max: number;
	readonly step: number;
	readonly onChange: CommandDescriptor;
}

/** @emoji 🔢 Numeric stepper with absolute input and ± delta buttons. */
export interface UiNumberStepperNode {
	readonly type: "numberStepper";
	readonly id: string;
	readonly value: number;
	readonly step: number;
	readonly uniform: boolean;
	readonly onAbsolute: CommandDescriptor;
	readonly onDelta: CommandDescriptor;
}

/** @emoji ⭕ Ring orb picker bound to a command (`t` in args). */
export interface UiRingNode {
	readonly type: "ring";
	readonly id: string;
	readonly orbId: string;
	readonly t: number;
	readonly disabled?: boolean;
	readonly onChange: CommandDescriptor;
}

/** @emoji 🖼️ Icon picker with a registered classifier kind. */
export interface UiIconSelectNode {
	readonly type: "iconSelect";
	readonly id: string;
	readonly value: string;
	readonly uniform: boolean;
	readonly classifierKind: "puzzle2d";
	readonly onChange: CommandDescriptor;
}

/** @emoji 🏷️ Labeled field wrapping one declarative control. */
export interface UiFieldNode {
	readonly type: "field";
	readonly id: string;
	readonly label: string;
	readonly child: UiControlNode;
}

/** @emoji 🎛️ Inline control hosted on a tree item row (includes panel table surfaces). */
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
	| UiIconSelectNode
	| UiFieldNode
	| UiTableHostSurfaceNode
	| UiPanelHostSurfaceNode;

/** @emoji 📂 Collapsible form section used while building {@link UiTreeNode} bodies (not a panel root). */
export interface UiSectionNode {
	readonly type: "section";
	readonly id: string;
	readonly label?: string;
	readonly defaultOpen?: boolean;
	readonly children: readonly UiNode[];
}

/** @emoji 👁️ Inline tree-row action (hide/lock toggles, …). */
export interface UiTreeItemAction {
	readonly id?: string;
	readonly icon: string;
	readonly title?: string;
	readonly onClick: () => void;
	readonly revealOnHover?: boolean;
}

/** @emoji 🖱️ Serializable tree-row context menu entry. */
export interface UiTreeContextMenuItem {
	readonly id: string;
	readonly label?: string;
	readonly icon?: string;
	readonly disabled?: boolean;
	readonly onSelect?: () => void;
	readonly children?: readonly UiTreeContextMenuItem[];
}

/** @emoji 🌿 One tree row; optional nested items, selection command, and inline control. */
export interface UiTreeItemNode {
	readonly id: string;
	readonly label: string;
	readonly description?: string;
	/** @emoji 🖼️ Lucide catalog id (or registered element icon id) for the row glyph. */
	readonly icon?: string;
	readonly selected?: boolean;
	readonly defaultOpen?: boolean;
	readonly command?: CommandDescriptor;
	readonly draggable?: boolean;
	readonly dragData?: Readonly<Record<string, string>>;
	readonly onPointerEnter?: () => void;
	readonly onPointerLeave?: () => void;
	readonly items?: readonly UiTreeItemNode[];
	readonly control?: UiControlNode;
	readonly isHidden?: boolean;
	readonly actions?: readonly UiTreeItemAction[];
	readonly contextMenu?: readonly UiTreeContextMenuItem[];
}

/** @emoji 🌲 Tree section for {@link UiTreeNode}. */
export interface UiTreeSectionNode {
	readonly id: string;
	readonly label?: string;
	readonly defaultOpen?: boolean;
	readonly items: readonly UiTreeItemNode[];
}

/** @emoji 🎯 Optional tree selection overlay (row ids). */
export interface SidePanelTreeSelection {
	readonly selectedIds?: readonly string[];
	readonly highlightedIds?: readonly string[];
}

/** @emoji 🌲 Side-panel tab body: sections of tree items only. */
export interface UiTreeNode {
	readonly type: "tree";
	readonly sections: readonly UiTreeSectionNode[];
	readonly selectedIds?: readonly string[];
	readonly highlightedIds?: readonly string[];
	readonly selectionChange?: CommandDescriptor;
}

/** @emoji 🖱️ Collects declarative tree item `dragData` by row id (depth-first across sections). */
export function collectUiTreeItemDragData(sections: readonly UiTreeSectionNode[]): Map<string, Record<string, string>> {
	const out = new Map<string, Record<string, string>>();
	const visitItems = (items: readonly UiTreeItemNode[]): void => {
		for (const item of items) {
			if (item.dragData) {
				out.set(item.id, item.dragData);
			}
			if (item.items?.length) {
				visitItems(item.items);
			}
		}
	};
	for (const section of sections) {
		visitItems(section.items);
	}
	return out;
}

/** @emoji 🌲 Single-root tree body for a side panel tab. */
export function sidePanelTreeRootItems(
	sectionId: string,
	items: readonly UiTreeItemNode[],
	selection?: SidePanelTreeSelection,
): UiTreeNode {
	if (!items.length) {
		throw new Error("sidePanelTreeRootItems requires at least one root item.");
	}
	return {
		type: "tree",
		sections: [{ id: sectionId, defaultOpen: false, items }],
		...(selection?.selectedIds ? { selectedIds: selection.selectedIds } : {}),
		...(selection?.highlightedIds ? { highlightedIds: selection.highlightedIds } : {}),
	};
}

//#region 🔖ComponentKind
/** @emoji 🧩 Fixed platform component vocabulary wired by renderers (`table`, `virtualFileSystem`, `puzzle2d`, …). */
export type ComponentKind = "table" | "virtualFileSystem" | "puzzle2d" | "puzzle3d" | "puzzle5d" | "cad" | "gismap" | "flow" | "dag" | "trinity" | "shooting" | "panel";

const CANVAS_COMPONENT_KINDS: readonly ComponentKind[] = ["table", "virtualFileSystem", "puzzle2d", "puzzle3d", "puzzle5d", "cad", "gismap", "flow", "dag", "trinity", "shooting"];
//#endregion 🔖ComponentKind

/** @emoji 📊 Host-bound tabular surface; `paneId` disambiguates multiple table slots in one app. */
export interface UiTableHostSurfaceNode {
	readonly type: "table";
	readonly componentKind: "table";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 📁 Host-bound virtual file system surface (hierarchical table). */
export interface UiVirtualFileSystemHostSurfaceNode {
	readonly type: "virtualFileSystem";
	readonly componentKind: "virtualFileSystem";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 📋 Host-bound 2D puzzle surface. */
export interface UiPuzzle2dHostSurfaceNode {
	readonly type: "puzzle2d";
	readonly componentKind: "puzzle2d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🧊 Host-bound 3D puzzle scene surface. */
export interface UiPuzzle3dHostSurfaceNode {
	readonly type: "puzzle3d";
	readonly componentKind: "puzzle3d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

/** @emoji 🌐 Host-bound unified 2D+3D topology surface (`FiveD`). */
export interface UiPuzzle5dHostSurfaceNode {
	readonly type: "puzzle5d";
	readonly componentKind: "puzzle5d";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🗺️ Host-bound GIS map surface (Web Mercator tiles + overlays). */
export interface UiGisMapHostSurfaceNode {
	readonly type: "gismap";
	readonly componentKind: "gismap";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🌊 Host-bound flow DAG surface. */
export interface UiFlowHostSurfaceNode {
	readonly type: "flow";
	readonly componentKind: "flow";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🌳 Host-bound directed acyclic graph surface. */
export interface UiDagHostSurfaceNode {
	readonly type: "dag";
	readonly componentKind: "dag";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 🔺 Host-bound trinity directed property port graph surface. */
export interface UiTrinityHostSurfaceNode {
	readonly type: "trinity";
	readonly componentKind: "trinity";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId?: string;
	readonly bindingId?: string;
}

/** @emoji 📐 Host-bound CAD spatial surface. */
export interface UiCadHostSurfaceNode {
	readonly type: "cad";
	readonly componentKind: "cad";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

/** @emoji 🧩 Host-bound side-panel surface; renderer maps `surfaceId` to panel body chrome. */
export interface UiPanelHostSurfaceNode {
	readonly type: "panel";
	readonly componentKind: "panel";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly bindingId?: string;
}

/** @emoji 📸 Host-bound shooting surface (`model` interactive viewport or `icon` shot preview). */
export interface UiShootingHostSurfaceNode {
	readonly type: "shooting";
	readonly componentKind: "shooting";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly view: "model" | "icon";
	readonly bindingId?: string;
}

export type UiComponentHostSurfaceNode =
	| UiTableHostSurfaceNode
	| UiVirtualFileSystemHostSurfaceNode
	| UiPuzzle2dHostSurfaceNode
	| UiPuzzle3dHostSurfaceNode
	| UiPuzzle5dHostSurfaceNode
	| UiGisMapHostSurfaceNode
	| UiFlowHostSurfaceNode
	| UiDagHostSurfaceNode
	| UiTrinityHostSurfaceNode
	| UiCadHostSurfaceNode
	| UiShootingHostSurfaceNode
	| UiPanelHostSurfaceNode;

/** @emoji 🌲 Converts declarative form sections into a strict side-panel tree. */
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
	if (node.type === "text") {
		return { id: `${fallbackId}.text`, label: node.value };
	}
	if (node.type === "field") {
		if (node.child.type === "text") {
			return { id: node.id, label: node.label, description: node.child.value };
		}
		return { id: node.id, label: node.label, control: node.child };
	}
	if (node.type === "button") {
		return { id: node.id ?? fallbackId, label: node.label, control: node };
	}
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
	if (node.type === "separator") {
		return { id: `${fallbackId}.sep`, label: "—" };
	}
	return { id: fallbackId, label: node.type };
}

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
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
	| UiTreeNode
	| UiComponentHostSurfaceNode;

/** @emoji 📊 Canonical table window body: only the host-bound table surface. */
export function buildTableWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiTableHostSurfaceNode {
	return {
		type: "table",
		componentKind: "table",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 📁 Canonical virtual file system window body. */
export function buildVirtualFileSystemWindowBody(
	surfaceId: string,
	controllerId: string,
	paneId?: string,
	bindingId?: string,
): UiVirtualFileSystemHostSurfaceNode {
	return {
		type: "virtualFileSystem",
		componentKind: "virtualFileSystem",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 📋 Canonical 2D puzzle window body. */
export function buildPuzzle2dWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiPuzzle2dHostSurfaceNode {
	return {
		type: "puzzle2d",
		componentKind: "puzzle2d",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🧊 Canonical 3D puzzle window body. */
export function buildPuzzle3dWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiPuzzle3dHostSurfaceNode {
	return { type: "puzzle3d", componentKind: "puzzle3d", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

/** @emoji 🌐 Canonical 5D topology window body. */
export function buildPuzzle5dWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiPuzzle5dHostSurfaceNode {
	return {
		type: "puzzle5d",
		componentKind: "puzzle5d",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🗺️ Canonical GIS map window body. */
export function buildMapWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiGisMapHostSurfaceNode {
	return {
		type: "gismap",
		componentKind: "gismap",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🌊 Canonical flow window body. */
export function buildFlowWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiFlowHostSurfaceNode {
	return {
		type: "flow",
		componentKind: "flow",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🌳 Canonical DAG window body. */
export function buildDagWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiDagHostSurfaceNode {
	return {
		type: "dag",
		componentKind: "dag",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 🔺 Canonical trinity window body. */
export function buildTrinityWindowBody(surfaceId: string, controllerId: string, paneId?: string, bindingId?: string): UiTrinityHostSurfaceNode {
	return {
		type: "trinity",
		componentKind: "trinity",
		surfaceId,
		controllerId,
		...(paneId ? { paneId } : {}),
		...(bindingId ? { bindingId } : {}),
	};
}

/** @emoji 📐 Canonical CAD window body. */
export function buildCadWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiCadHostSurfaceNode {
	return { type: "cad", componentKind: "cad", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

/** @emoji 🧩 Canonical panel window body. */
export function buildPanelWindowBody(surfaceId: string, controllerId: string, bindingId?: string): UiPanelHostSurfaceNode {
	return { type: "panel", componentKind: "panel", surfaceId, controllerId, ...(bindingId ? { bindingId } : {}) };
}

/** @emoji 📸 Canonical shooting window body for model or icon viewport. */
export function buildShootingWindowBody(
	surfaceId: string,
	controllerId: string,
	view: "model" | "icon",
	bindingId?: string,
): UiShootingHostSurfaceNode {
	return { type: "shooting", componentKind: "shooting", surfaceId, controllerId, view, ...(bindingId ? { bindingId } : {}) };
}

function isCanvasComponentNode(node: UiNode): boolean {
	if (node.type === "text") return true;
	if (node.type === "panel") return true;
	return CANVAS_COMPONENT_KINDS.includes(node.type as ComponentKind);
}

/** @emoji ✅ True when a window body is a lone canvas component surface or a short error `text` node. */
export function isCanvasOnlyWindowBody(node: UiNode): boolean {
	if (isCanvasComponentNode(node)) return true;
	if (node.type === "stack" && node.padding === "none" && node.children.length === 1) {
		return isCanvasComponentNode(node.children[0]);
	}
	return false;
}

function assertCanvasOnlyWindowBody(bodyKey: string, node: UiNode): void {
	if (isCanvasOnlyWindowBody(node)) return;
		throw new Error(
			`Declarative window body "${bodyKey}" must be a single table, virtualFileSystem, puzzle2d, puzzle3d, puzzle5d, cad, or shooting surface (optional none padding stack wrapper). Found "${node.type}". Use ModeRuntime.tools, side tabs, or window measures for chrome.`,
		);
}
//#endregion 🔖UiNode

//#region 🔖ComponentModels
/** @emoji 📊 Column descriptor for {@link TableModel}. */
export interface TableColumnModel {
	readonly id: string;
	readonly label: string;
	readonly width?: number;
	readonly sortable?: boolean;
}

/** @emoji 📊 Row descriptor for {@link TableModel}. */
export interface TableRowModel {
	readonly id: string;
	readonly cells: Readonly<Record<string, string | number | boolean | null>>;
	readonly navigateUri?: string;
	readonly depth?: number;
	readonly hasChildren?: boolean;
	readonly expanded?: boolean;
	readonly expandToggle?: { readonly command: string; readonly args?: unknown };
}

/** @emoji 📊 Render-agnostic tabular view-model for {@link Table}. */
export interface TableModel {
	readonly columns: readonly TableColumnModel[];
	readonly rows: readonly TableRowModel[];
	readonly selectedRowIds?: readonly string[];
	readonly sortColumnId?: string | null;
	readonly sortDescending?: boolean;
	readonly emptyMessage?: string;
}

/** @emoji 🏷️ Render-agnostic descriptor presentation kinds for virtual file system columns. */
export type VirtualFileSystemDescriptorKindModel =
	| { readonly id: string; readonly name: string; readonly description?: string; readonly presentation: "text" }
	| {
			readonly id: string;
			readonly name: string;
			readonly description?: string;
			readonly presentation: "time";
			readonly format?: "date" | "datetime" | "relative";
	  }
	| { readonly id: string; readonly name: string; readonly description?: string; readonly presentation: "avatar" };

/** @emoji 🏷️ Column binding on a {@link VirtualFileSystemFileNodeKindModel}. */
export interface VirtualFileSystemFileNodeDescriptorModel {
	readonly id: string;
	readonly descriptorKindId: string;
	readonly label?: string;
	readonly description?: string;
}

/** @emoji 📁 File node kind registry entry for {@link VirtualFileSystemSchemaModel}. */
export interface VirtualFileSystemFileNodeKindModel {
	readonly id: string;
	readonly name: string;
	readonly icon?: string;
	readonly description?: string;
	readonly descriptors: readonly VirtualFileSystemFileNodeDescriptorModel[];
}

/** @emoji 📁 Cell value for one descriptor column on a virtual file system node. */
export type VirtualFileSystemDescriptorValueModel =
	| { readonly presentation: "text"; readonly text: string }
	| { readonly presentation: "time"; readonly iso: string }
	| { readonly presentation: "avatar"; readonly name: string; readonly icon?: string };

/** @emoji 📁 Schema driving virtual file system columns (render-agnostic). */
export interface VirtualFileSystemSchemaModel {
	readonly fileNodeKinds: Readonly<Record<string, VirtualFileSystemFileNodeKindModel>>;
	readonly descriptorKinds: Readonly<Record<string, VirtualFileSystemDescriptorKindModel>>;
	readonly descriptorColumnIds: readonly string[];
}

/** @emoji 📁 One lazy-loaded node record for {@link VirtualFileSystemController}. */
export interface VirtualFileSystemNodeRecord {
	readonly id: string;
	readonly fileNodeKindId: string;
	readonly name: string;
	readonly path: string;
	readonly parentId: string | null;
	readonly hasChildren: boolean;
	readonly icon?: string;
	readonly navigateUri?: string;
	readonly descriptorValues?: Readonly<Record<string, VirtualFileSystemDescriptorValueModel>>;
	readonly canDrag?: boolean;
}

/** @emoji 📁 Flat row for {@link VirtualFileSystemModel}. */
export interface VirtualFileSystemRowModel {
	readonly id: string;
	readonly fileNodeKindId: string;
	readonly name: string;
	readonly path: string;
	readonly depth: number;
	readonly hasChildren: boolean;
	readonly icon?: string;
	readonly expanded?: boolean;
	readonly expandToggle?: { readonly command: string; readonly args?: unknown };
	readonly canDrag?: boolean;
	readonly navigateUri?: string;
	readonly descriptorValues?: Readonly<Record<string, VirtualFileSystemDescriptorValueModel>>;
}

/** @emoji 📁 Render-agnostic virtual file system view-model for {@link VirtualFileSystem}. */
export interface VirtualFileSystemModel {
	readonly schema: VirtualFileSystemSchemaModel;
	readonly rows: readonly VirtualFileSystemRowModel[];
	readonly selectedRowIds?: readonly string[];
	readonly hoveredRowId?: string | null;
	readonly emptyMessage?: string;
	readonly dragDropEnabled?: boolean;
}

/** @emoji 📋 Node descriptor for {@link Puzzle2dModel}. */
export interface Puzzle2dNodeModel {
	readonly id: string;
	readonly label?: string;
	readonly x?: number;
	readonly y?: number;
}

/** @emoji 📋 Edge descriptor for {@link Puzzle2dModel}. */
export interface Puzzle2dEdgeModel {
	readonly id: string;
	readonly sourceId: string;
	readonly targetId: string;
}

/** @emoji 📋 Render-agnostic 2D puzzle view-model for {@link Puzzle2d}. */
export interface Puzzle2dModel {
	readonly nodes: readonly Puzzle2dNodeModel[];
	readonly edges: readonly Puzzle2dEdgeModel[];
	readonly portColors?: Readonly<Record<string, string>>;
	readonly emptyMessage?: string;
}

/** @emoji 🧊 Render-agnostic 3D scene view-model for {@link Puzzle3d}. */
export interface Puzzle3dModel {
	readonly instanceId?: string;
	readonly emptyMessage?: string;
}

/** @emoji 🌐 Render-agnostic unified topology view-model for {@link Puzzle5d}. */
export interface Puzzle5dModel {
	readonly presentation: "flat" | "volume";
	readonly instanceId: string;
	readonly emptyMessage?: string;
	readonly puzzle2dSelection?: readonly string[];
	readonly puzzle2dHoveredId?: string | null;
}

/** @emoji 📐 Render-agnostic CAD view-model for {@link Cad}. */
export interface CadModel {
	readonly instanceId?: string;
	readonly emptyMessage?: string;
}

/** @emoji 🧩 Render-agnostic panel body for {@link Panel}. */
export interface PanelModel {
	readonly body: UiNode;
}

/** @emoji 🗺️ Serializable flat+volume fixtures composed at render time by puzzle topology. */
export interface PlatformTopologyPayload {
	readonly flat: Record<string, unknown>;
	readonly volume: Record<string, unknown>;
}

export const PLATFORM_TOPOLOGY_STORE_PREFIX = "topology:";

/** @emoji 🔑 Controller store id for {@link PlatformTopologyStore} (`topology:<instanceId>`). */
export function platformTopologyStoreId(instanceId: string): string {
	return `${PLATFORM_TOPOLOGY_STORE_PREFIX}${instanceId}`;
}

/** @emoji 🗄️ Controller-owned topology fixture pair for FiveD hosts. */
export class PlatformTopologyStore extends Store<PlatformTopologyPayload> {
	private payload: PlatformTopologyPayload;

	constructor(payload: PlatformTopologyPayload) {
		super();
		this.payload = payload;
	}

	override getSnapshot(): PlatformTopologyPayload {
		return this.payload;
	}

	replacePayload(next: PlatformTopologyPayload): void {
		this.payload = next;
		this.notify();
	}
}

/** @emoji 🎛 Finds a mounted app {@link Controller} by id. */
export function getPlatformControllerById(platform: Platform, controllerId: string): Controller | undefined {
	for (const app of platform.apps) {
		if (app.controller.id === controllerId) return app.controller;
	}
	return undefined;
}
//#endregion 🔖ComponentModels

//#region 🔖Component
/** @emoji 🧩 Render-agnostic platform surface backed by a {@link Store} snapshot. */
export abstract class Component<TSnapshot> extends Store<TSnapshot> implements SurfaceComponent {
	readonly componentKind: ComponentKind;
	readonly surfaceId: string;
	readonly controllerId: string;
	private snapshotValue: TSnapshot;

	constructor(componentKind: ComponentKind, surfaceId: string, controllerId: string, initialSnapshot: TSnapshot) {
		super();
		this.componentKind = componentKind;
		this.surfaceId = surfaceId;
		this.controllerId = controllerId;
		this.snapshotValue = initialSnapshot;
	}

	override getSnapshot(): TSnapshot {
		return this.snapshotValue;
	}

	protected setSnapshot(next: TSnapshot): void {
		if (Object.is(this.snapshotValue, next)) return;
		this.snapshotValue = next;
		this.notify();
	}

	abstract buildSnapshot(): TSnapshot;

	refresh(): void {
		this.setSnapshot(this.buildSnapshot());
	}
}

/** @emoji 📊 Table surface component base class. */
export class Table extends Component<TableModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: TableModel = { columns: [], rows: [] }) {
		super("table", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): TableModel {
		return this.getSnapshot();
	}
}

/** @emoji 📁 Virtual file system surface component base class (scoped to one app). */
export class VirtualFileSystem extends Component<VirtualFileSystemModel> {
	readonly appId: string;

	constructor(
		appId: string,
		surfaceId: string,
		controllerId: string,
		initialSnapshot: VirtualFileSystemModel = { schema: { fileNodeKinds: {}, descriptorKinds: {}, descriptorColumnIds: [] }, rows: [] },
	) {
		super("virtualFileSystem", surfaceId, controllerId, initialSnapshot);
		this.appId = appId;
	}

	get scope(): VirtualFileSystemScope {
		return { appId: this.appId, surfaceId: this.surfaceId };
	}

	buildSnapshot(): VirtualFileSystemModel {
		return this.getSnapshot();
	}
}

/** @emoji 📁 One app-bound virtual file system instance (`appId` + host `surfaceId`). */
export interface VirtualFileSystemScope {
	readonly appId: string;
	readonly surfaceId: string;
}

/** @emoji 📁 Stable scope key for controller-owned VFS stores. */
export function virtualFileSystemScopeKey(scope: VirtualFileSystemScope): string {
	return `${scope.appId}::${scope.surfaceId}`;
}

/** @emoji 📁 Canonical surface id for an app VFS (`vfs:<appId>:<slot>`). */
export function virtualFileSystemSurfaceId(appId: string, slot = "main"): string {
	return `vfs:${appId}:${slot}`;
}

/** @emoji 📁 Parses {@link virtualFileSystemSurfaceId} into a {@link VirtualFileSystemScope}. */
export function parseVirtualFileSystemSurfaceId(surfaceId: string): VirtualFileSystemScope | null {
	const match = /^vfs:([^:]+):(.+)$/.exec(surfaceId);
	if (!match) return null;
	return { appId: match[1]!, surfaceId };
}

/** @emoji 📁 Controller store id for expanded VFS node ids. */
export function virtualFileSystemExpandedStoreId(scope: VirtualFileSystemScope): string {
	return `vfs-expanded:${virtualFileSystemScopeKey(scope)}`;
}

/** @emoji 📁 Controller store id for lazily loaded VFS children. */
export function virtualFileSystemChildrenStoreId(scope: VirtualFileSystemScope): string {
	return `vfs-children:${virtualFileSystemScopeKey(scope)}`;
}

/** @emoji 📁 Expanded node ids per VFS surface. */
export class VirtualFileSystemExpandedStore extends Store<readonly string[]> {
	private expanded: string[];

	constructor(initial: readonly string[] = []) {
		super();
		this.expanded = [...initial];
	}

	override getSnapshot(): readonly string[] {
		return this.expanded;
	}

	toggle(nodeId: string): void {
		const index = this.expanded.indexOf(nodeId);
		if (index >= 0) this.expanded.splice(index, 1);
		else this.expanded.push(nodeId);
		this.notify();
	}

	setAll(next: readonly string[]): void {
		this.expanded = [...next];
		this.notify();
	}
}

/** @emoji 📁 Lazily loaded children keyed by parent node id. */
export class VirtualFileSystemChildrenStore extends Store<Readonly<Record<string, readonly VirtualFileSystemNodeRecord[]>>> {
	private childrenByParentId: Record<string, readonly VirtualFileSystemNodeRecord[]>;

	constructor(initial: Readonly<Record<string, readonly VirtualFileSystemNodeRecord[]>> = {}) {
		super();
		this.childrenByParentId = { ...initial };
	}

	override getSnapshot(): Readonly<Record<string, readonly VirtualFileSystemNodeRecord[]>> {
		return this.childrenByParentId;
	}

	setChildren(parentId: string, children: readonly VirtualFileSystemNodeRecord[]): void {
		this.childrenByParentId = { ...this.childrenByParentId, [parentId]: children };
		this.notify();
	}

	moveNode(nodeId: string, targetParentId: string | null, rootId: string): void {
		let moved: VirtualFileSystemNodeRecord | undefined;
		const next: Record<string, readonly VirtualFileSystemNodeRecord[]> = {};
		for (const [parentId, children] of Object.entries(this.childrenByParentId)) {
			const kept = children.filter((child) => {
				if (child.id === nodeId) {
					moved = { ...child, parentId: targetParentId === rootId ? rootId : targetParentId };
					return false;
				}
				return true;
			});
			if (kept.length) next[parentId] = kept;
		}
		if (!moved) return;
		const bucket = !targetParentId || targetParentId === rootId ? "__root__" : targetParentId;
		next[bucket] = [...(next[bucket] ?? []), moved];
		this.childrenByParentId = next;
		this.notify();
	}
}

/** @emoji 📁 Flattens visible VFS rows from expanded ids and lazily loaded children. */
export function buildVirtualFileSystemModelRows(
	root: VirtualFileSystemNodeRecord,
	childrenByParentId: Readonly<Record<string, readonly VirtualFileSystemNodeRecord[]>>,
	expandedIds: ReadonlySet<string>,
	options?: {
		readonly expandCommand?: string;
		readonly scope?: VirtualFileSystemScope;
	},
): VirtualFileSystemRowModel[] {
	const rows: VirtualFileSystemRowModel[] = [];
	const rootBucket = childrenByParentId["__root__"];
	const visit = (node: VirtualFileSystemNodeRecord, depth: number) => {
		const hasChildren = node.hasChildren;
		const expanded = hasChildren && expandedIds.has(node.id);
		rows.push({
			id: node.id,
			fileNodeKindId: node.fileNodeKindId,
			name: node.name,
			path: node.path,
			depth,
			hasChildren,
			expanded,
			...(hasChildren && options?.expandCommand
				? {
						expandToggle: {
							command: options.expandCommand,
							args: { nodeId: node.id, appId: options.scope?.appId, surfaceId: options.scope?.surfaceId },
						},
					}
				: {}),
			...(node.canDrag === false ? { canDrag: false } : { canDrag: true }),
			...(node.icon ? { icon: node.icon } : {}),
			...(node.navigateUri ? { navigateUri: node.navigateUri } : {}),
			...(node.descriptorValues ? { descriptorValues: node.descriptorValues } : {}),
		});
		if (!expanded) return;
		const children = childrenByParentId[node.id];
		if (!children?.length) return;
		for (const child of children) visit(child, depth + 1);
	};
	const rootChildren = childrenByParentId[root.id] ?? rootBucket;
	if (rootChildren?.length) {
		for (const child of rootChildren) visit(child, 0);
	}
	return rows;
}

/** @emoji 📁 One VFS tree node currently visible (root + expanded branches) with its parent id. */
export type VirtualFileSystemVisibleNode = VirtualFileSystemNodeRecord & { readonly parentId: string | null };

/** @emoji 📁 Collects visible VFS nodes from root, expanded ids, and lazily loaded children. */
export function visibleVirtualFileSystemNodesFromTree(
	root: VirtualFileSystemNodeRecord,
	childrenByParentId: Readonly<Record<string, readonly VirtualFileSystemNodeRecord[]>>,
	expandedIds: ReadonlySet<string>,
): readonly VirtualFileSystemVisibleNode[] {
	const nodes: VirtualFileSystemVisibleNode[] = [];
	const rootBucket = childrenByParentId["__root__"];
	const visit = (node: VirtualFileSystemNodeRecord, parentId: string | null, depth: number) => {
		nodes.push({ ...node, parentId });
		if (!node.hasChildren || !expandedIds.has(node.id)) return;
		const children = childrenByParentId[node.id];
		if (!children?.length) return;
		for (const child of children) visit(child, node.id, depth + 1);
	};
	const rootChildren = childrenByParentId[root.id] ?? rootBucket;
	if (rootChildren?.length) {
		for (const child of rootChildren) visit(child, root.id, 0);
	}
	return nodes;
}

/** @emoji 📁 Base controller: per-app VFS; loads children only for expanded nodes. */
export abstract class VirtualFileSystemController extends Controller {
	protected readonly expandedByScope = new Map<string, VirtualFileSystemExpandedStore>();
	protected readonly childrenByScope = new Map<string, VirtualFileSystemChildrenStore>();
	protected readonly pendingChildrenLoadsByScope = new Map<string, Set<string>>();
	protected readonly childrenLoadPromisesByScope = new Map<string, Map<string, Promise<void>>>();
	protected readonly selectedRowIdsByScope = new Map<string, string[]>();
	protected readonly selectionAnchorRowIdByScope = new Map<string, string>();

	protected constructor(id: string, commandBus: CommandBus, hostNotify: () => void) {
		super(id, commandBus, hostNotify);
	}

	protected abstract getSchema(scope: VirtualFileSystemScope): VirtualFileSystemSchemaModel;

	protected abstract getRoot(scope: VirtualFileSystemScope): VirtualFileSystemNodeRecord;

	protected abstract loadChildren(parentId: string, scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[];

	/** @emoji 📁 When true, {@link loadChildrenAsync} loads rows; otherwise {@link loadChildren} runs synchronously. */
	protected virtualFileSystemUsesAsyncChildren(): boolean {
		return false;
	}

	/** @emoji 📁 Async child loader used when {@link virtualFileSystemUsesAsyncChildren} is true. */
	protected loadChildrenAsync(
		_parentId: string,
		_scope: VirtualFileSystemScope,
	): Promise<readonly VirtualFileSystemNodeRecord[]> {
		return Promise.resolve([]);
	}

	protected resolveScope(args?: unknown, surfaceIdFallback?: string): VirtualFileSystemScope | null {
		const payload = (args ?? {}) as { appId?: string; surfaceId?: string };
		const surfaceId = payload.surfaceId ?? surfaceIdFallback ?? "";
		if (!surfaceId) return null;
		const parsed = parseVirtualFileSystemSurfaceId(surfaceId);
		if (parsed) return payload.appId ? { appId: payload.appId, surfaceId: parsed.surfaceId } : parsed;
		if (!payload.appId) return null;
		return { appId: payload.appId, surfaceId };
	}

	protected expandedStore(scope: VirtualFileSystemScope, initial: readonly string[] = []): VirtualFileSystemExpandedStore {
		const key = virtualFileSystemScopeKey(scope);
		const existing = this.expandedByScope.get(key);
		if (existing) return existing;
		const store = new VirtualFileSystemExpandedStore(initial);
		this.expandedByScope.set(key, store);
		this.provideStore(virtualFileSystemExpandedStoreId(scope), store);
		return store;
	}

	protected childrenStore(scope: VirtualFileSystemScope): VirtualFileSystemChildrenStore {
		const key = virtualFileSystemScopeKey(scope);
		const existing = this.childrenByScope.get(key);
		if (existing) return existing;
		const store = new VirtualFileSystemChildrenStore();
		this.childrenByScope.set(key, store);
		this.provideStore(virtualFileSystemChildrenStoreId(scope), store);
		return store;
	}

	protected selectedRows(scope: VirtualFileSystemScope): string[] {
		const key = virtualFileSystemScopeKey(scope);
		let selected = this.selectedRowIdsByScope.get(key);
		if (!selected) {
			selected = [];
			this.selectedRowIdsByScope.set(key, selected);
		}
		return selected;
	}

	protected ensureChildrenLoaded(parentId: string, scope: VirtualFileSystemScope): void {
		const childrenStore = this.childrenStore(scope);
		const snapshot = childrenStore.getSnapshot();
		const key = parentId === this.getRoot(scope).id ? "__root__" : parentId;
		if (snapshot[key]) return;
		const scopeKey = virtualFileSystemScopeKey(scope);
		let pending = this.pendingChildrenLoadsByScope.get(scopeKey);
		if (!pending) {
			pending = new Set();
			this.pendingChildrenLoadsByScope.set(scopeKey, pending);
		}
		if (pending.has(key)) return;
		if (!this.virtualFileSystemUsesAsyncChildren()) {
			childrenStore.setChildren(key, this.loadChildren(parentId, scope));
			return;
		}
		pending.add(key);
		void this.loadChildrenAsync(parentId, scope).then((loaded) => {
			pending!.delete(key);
			childrenStore.setChildren(key, loaded);
			this.emit();
		});
	}

	/** @emoji 📁 Like {@link ensureChildrenLoaded} but resolves when children are present (sync or async). */
	protected ensureChildrenLoadedAsync(parentId: string, scope: VirtualFileSystemScope): Promise<void> {
		const childrenStore = this.childrenStore(scope);
		const snapshot = childrenStore.getSnapshot();
		const key = parentId === this.getRoot(scope).id ? "__root__" : parentId;
		if (snapshot[key] !== undefined) {
			return Promise.resolve();
		}
		const scopeKey = virtualFileSystemScopeKey(scope);
		let promises = this.childrenLoadPromisesByScope.get(scopeKey);
		if (!promises) {
			promises = new Map();
			this.childrenLoadPromisesByScope.set(scopeKey, promises);
		}
		const existing = promises.get(key);
		if (existing) {
			return existing;
		}
		const load = (async () => {
			if (!this.virtualFileSystemUsesAsyncChildren()) {
				this.ensureChildrenLoaded(parentId, scope);
				return;
			}
			const loaded = await this.loadChildrenAsync(parentId, scope);
			childrenStore.setChildren(key, loaded);
			this.emit();
		})().finally(() => {
			promises!.delete(key);
		});
		promises.set(key, load);
		return load;
	}

	protected syncOpenBranches(scope: VirtualFileSystemScope): void {
		const rootId = this.getRoot(scope).id;
		this.ensureChildrenLoaded(rootId, scope);
		for (const nodeId of this.expandedStore(scope).getSnapshot()) {
			this.ensureChildrenLoaded(nodeId, scope);
		}
	}

	/** @emoji 📁 Root node id when binding an app VFS surface. */
	virtualFileSystemRootId(scope: VirtualFileSystemScope): string {
		return this.getRoot(scope).id;
	}

	buildVirtualFileSystemModel(scope: VirtualFileSystemScope): VirtualFileSystemModel {
		this.syncOpenBranches(scope);
		const expandedIds = new Set(this.expandedStore(scope).getSnapshot());
		const rows = buildVirtualFileSystemModelRows(this.getRoot(scope), this.childrenStore(scope).getSnapshot(), expandedIds, {
			expandCommand: "toggleVirtualFileSystemExpand",
			scope,
		});
		return {
			schema: this.getSchema(scope),
			rows,
			selectedRowIds: this.selectedRows(scope),
			emptyMessage: rows.length ? undefined : "No file system nodes",
			dragDropEnabled: true,
		};
	}

	/** @emoji 📁 Visible file nodes for the current expansion state (same visibility as the VFS table). */
	visibleVirtualFileSystemNodes(scope: VirtualFileSystemScope): readonly VirtualFileSystemVisibleNode[] {
		this.syncOpenBranches(scope);
		const expandedIds = new Set(this.expandedStore(scope).getSnapshot());
		return visibleVirtualFileSystemNodesFromTree(this.getRoot(scope), this.childrenStore(scope).getSnapshot(), expandedIds);
	}

	protected runVirtualFileSystemCommand(command: string, args?: unknown): boolean {
		const scope = this.resolveScope(args);
		if (!scope) return false;
		const payload = (args ?? {}) as { nodeId?: string; rowId?: string; active?: string; over?: string | null };
		switch (command) {
			case "toggleVirtualFileSystemExpand": {
				if (!payload.nodeId) return true;
				const expanded = this.expandedStore(scope);
				expanded.toggle(payload.nodeId);
				if (expanded.getSnapshot().includes(payload.nodeId)) {
					this.ensureChildrenLoaded(payload.nodeId, scope);
				}
				this.emit();
				return true;
			}
			case "setVirtualFileSystemRowSelection": {
				const selectionPayload = args as { rowIds?: readonly string[]; anchorRowId?: string };
				const key = virtualFileSystemScopeKey(scope);
				const rowIds = selectionPayload.rowIds ? [...selectionPayload.rowIds] : [];
				this.selectedRowIdsByScope.set(key, rowIds);
				if (selectionPayload.anchorRowId) {
					this.selectionAnchorRowIdByScope.set(key, selectionPayload.anchorRowId);
				} else if (!rowIds.length) {
					this.selectionAnchorRowIdByScope.delete(key);
				}
				this.emit();
				return true;
			}
			case "virtualFileSystemDragEnd": {
				if (!payload.active || !payload.over) return true;
				const childrenStore = this.childrenStore(scope);
				const rootId = this.getRoot(scope).id;
				const targetParentId = payload.over === rootId ? rootId : payload.over;
				childrenStore.moveNode(payload.active, targetParentId, rootId);
				this.emit();
				return true;
			}
			default:
				return false;
		}
	}

	override run(command: string, args?: unknown): void {
		if (this.runVirtualFileSystemCommand(command, args)) return;
	}
}

/** @emoji 📁 Builds descriptor cell values from a {@link VirtualFileSystemSchemaModel}. */
export function virtualFileSystemDescriptorValues(
	schema: VirtualFileSystemSchemaModel,
	fileNodeKindId: string,
	options: {
		readonly path?: string;
		readonly updatedIso?: string;
		readonly createdBy?: { readonly name: string; readonly icon?: string };
		readonly textByDescriptorId?: Readonly<Record<string, string>>;
		readonly extra?: Readonly<Record<string, VirtualFileSystemDescriptorValueModel>>;
	} = {},
): Readonly<Record<string, VirtualFileSystemDescriptorValueModel>> {
	const fileNodeKind = schema.fileNodeKinds[fileNodeKindId];
	const values: Record<string, VirtualFileSystemDescriptorValueModel> = { ...options.extra };
	if (options.path !== undefined) values.path = { presentation: "text", text: options.path };
	if (fileNodeKind) values.fileNodeKind = { presentation: "text", text: fileNodeKind.name };
	if (options.updatedIso) values.updated = { presentation: "time", iso: options.updatedIso };
	if (options.createdBy) {
		values.createdBy = { presentation: "avatar", name: options.createdBy.name, icon: options.createdBy.icon };
	}
	if (options.textByDescriptorId) {
		for (const [descriptorId, text] of Object.entries(options.textByDescriptorId)) {
			values[descriptorId] = { presentation: "text", text };
		}
	}
	return values;
}

/** @emoji 📁 Demo virtual file system schema (render-agnostic). */
export const PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA: VirtualFileSystemSchemaModel = {
	descriptorKinds: {
		text: { id: "text", name: "Text", presentation: "text" },
		time: { id: "time", name: "Time", presentation: "time", format: "datetime" },
		avatar: { id: "avatar", name: "Avatar", presentation: "avatar" },
	},
	fileNodeKinds: {
		workspace: {
			id: "workspace",
			name: "Workspace",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		folder: {
			id: "folder",
			name: "Folder",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		branch: {
			id: "branch",
			name: "Branch",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
		leaf: {
			id: "leaf",
			name: "Leaf",
			descriptors: [
				{ id: "path", descriptorKindId: "text", label: "Path" },
				{ id: "fileNodeKind", descriptorKindId: "text", label: "Node kind" },
			],
		},
	},
	descriptorColumnIds: ["path", "fileNodeKind"],
};

/** @emoji 📁 Builds standard path and node-kind descriptor values for demo nodes. */
export function platformVirtualFileSystemDemoDescriptorValues(
	fileNodeKindId: string,
	path: string,
): Readonly<Record<string, VirtualFileSystemDescriptorValueModel>> {
	const fileNodeKind = PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA.fileNodeKinds[fileNodeKindId];
	return {
		path: { presentation: "text", text: path },
		fileNodeKind: { presentation: "text", text: fileNodeKind?.name ?? fileNodeKindId },
	};
}

/** @emoji 📁 Demo VFS controller: each app id gets its own in-memory tree. */
export class PlatformVirtualFileSystemDemoController extends VirtualFileSystemController {
	static readonly APP_A = "demo-app-a";
	static readonly APP_B = "demo-app-b";

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super("platform-vfs-demo-ctrl", commandBus, hostNotify);
	}

	protected override getSchema(_scope: VirtualFileSystemScope): VirtualFileSystemSchemaModel {
		return PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA;
	}

	protected override getRoot(scope: VirtualFileSystemScope): VirtualFileSystemNodeRecord {
		if (scope.appId === PlatformVirtualFileSystemDemoController.APP_B) {
			return {
				id: "workspace-b",
				fileNodeKindId: "workspace",
				name: "Beta Workspace",
				path: "/",
				parentId: null,
				hasChildren: true,
				canDrag: false,
				descriptorValues: platformVirtualFileSystemDemoDescriptorValues("workspace", "/"),
			};
		}
		return {
			id: "workspace-a",
			fileNodeKindId: "workspace",
			name: "Alpha Workspace",
			path: "/",
			parentId: null,
			hasChildren: true,
			canDrag: false,
			descriptorValues: platformVirtualFileSystemDemoDescriptorValues("workspace", "/"),
		};
	}

	protected override loadChildren(parentId: string, scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[] {
		if (scope.appId === PlatformVirtualFileSystemDemoController.APP_B) {
			if (parentId === "workspace-b") {
				const path = "/Beta Branch";
				return [
					{
						id: "branch-b1",
						fileNodeKindId: "branch",
						name: "Beta Branch",
						path,
						parentId,
						hasChildren: false,
						descriptorValues: platformVirtualFileSystemDemoDescriptorValues("branch", path),
					},
				];
			}
			return [];
		}
		if (parentId === "workspace-a") {
			return [
				{
					id: "folder-models",
					fileNodeKindId: "folder",
					name: "Models",
					path: "/Models",
					parentId,
					hasChildren: true,
					descriptorValues: platformVirtualFileSystemDemoDescriptorValues("folder", "/Models"),
				},
				{
					id: "branch-alpha",
					fileNodeKindId: "branch",
					name: "Alpha",
					path: "/Alpha",
					parentId,
					hasChildren: false,
					descriptorValues: platformVirtualFileSystemDemoDescriptorValues("branch", "/Alpha"),
				},
			];
		}
		if (parentId === "folder-models") {
			const path = "/Models/Capsule";
			return [
				{
					id: "leaf-capsule",
					fileNodeKindId: "leaf",
					name: "Capsule",
					path,
					parentId,
					hasChildren: false,
					descriptorValues: platformVirtualFileSystemDemoDescriptorValues("leaf", path),
				},
			];
		}
		return [];
	}
}

/** @emoji 📁 App-bound {@link VirtualFileSystem} surface driven by a {@link VirtualFileSystemController}. */
export class AppBoundVirtualFileSystemSurface extends VirtualFileSystem {
	constructor(
		readonly owner: VirtualFileSystemController,
		readonly vfsScope: VirtualFileSystemScope,
	) {
		super(vfsScope.appId, vfsScope.surfaceId, owner.id, {
			schema: { fileNodeKinds: {}, descriptorKinds: {}, descriptorColumnIds: [] },
			rows: [],
		});
	}

	override buildSnapshot(): VirtualFileSystemModel {
		return this.owner.buildVirtualFileSystemModel(this.vfsScope);
	}
}

/** @emoji 📁 Registers one app-owned VFS surface on a {@link Platform}. */
export function registerAppVirtualFileSystem(
	platform: Platform,
	app: AppRuntime,
	controller: VirtualFileSystemController,
	options: {
		readonly bodyKey: string;
		readonly slot?: string;
		readonly surfaceId?: string;
		readonly paneId?: string;
		readonly initialExpanded?: readonly string[];
	},
): AppBoundVirtualFileSystemSurface {
	const surfaceId = options.surfaceId ?? virtualFileSystemSurfaceId(app.id, options.slot ?? "main");
	const scope: VirtualFileSystemScope = { appId: app.id, surfaceId };
	const surface = new AppBoundVirtualFileSystemSurface(controller, scope);
	registerPlatformComponent(platform, surface);
	const refresh = () => surface.refresh();
	platform.subscribe(refresh);
	controller.expandedStore(scope, options.initialExpanded ?? []);
	surface.refresh();
	registerWindowBody(options.bodyKey, () => buildVirtualFileSystemWindowBody(surfaceId, controller.id, options.paneId));
	return surface;
}

/** @emoji 📁 Registers two demo apps (A/B) each with its own VFS on a {@link Platform}. */
export function registerPlatformVirtualFileSystemDemo(platform: Platform): PlatformVirtualFileSystemDemoController {
	const ctrl = new PlatformVirtualFileSystemDemoController(platform.commandBus, () => platform.notify());
	const appA = new AppRuntime(
		PlatformVirtualFileSystemDemoController.APP_A,
		"Demo A",
		undefined,
		ctrl,
		createTabStackLayout(["main"], ["Main"]),
		[new WindowKindRuntime("main", "Main", "demo.vfs.a.main")],
	);
	const appB = new AppRuntime(
		PlatformVirtualFileSystemDemoController.APP_B,
		"Demo B",
		undefined,
		ctrl,
		createTabStackLayout(["main"], ["Main"]),
		[new WindowKindRuntime("main", "Main", "demo.vfs.b.main")],
	);
	platform.addApp(appA);
	platform.addApp(appB);
	registerAppVirtualFileSystem(platform, appA, ctrl, { bodyKey: "demo.vfs.a.main" });
	registerAppVirtualFileSystem(platform, appB, ctrl, { bodyKey: "demo.vfs.b.main" });
	return ctrl;
}

/** @emoji 📋 2D puzzle surface component base class. */
export class Puzzle2d extends Component<Puzzle2dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle2dModel = { nodes: [], edges: [] }) {
		super("puzzle2d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle2dModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧊 3D puzzle scene surface component base class. */
export class Puzzle3d extends Component<Puzzle3dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle3dModel = {}) {
		super("puzzle3d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle3dModel {
		return this.getSnapshot();
	}
}

/** @emoji 🌐 5D topology surface component base class. */
export class Puzzle5d extends Component<Puzzle5dModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: Puzzle5dModel) {
		super("puzzle5d", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): Puzzle5dModel {
		return this.getSnapshot();
	}
}

/** @emoji 📐 CAD surface component base class. */
export class Cad extends Component<CadModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: CadModel = {}) {
		super("cad", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): CadModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧩 Panel surface component base class. */
export class Panel extends Component<PanelModel> {
	constructor(surfaceId: string, controllerId: string, initialSnapshot: PanelModel) {
		super("panel", surfaceId, controllerId, initialSnapshot);
	}

	buildSnapshot(): PanelModel {
		return this.getSnapshot();
	}
}

/** @emoji 🧩 Registers a {@link Component} on a {@link Platform} instance. */
export function registerPlatformComponent(platform: Platform, component: Component<unknown>): void {
	platform.registerComponent(component);
}

/** @emoji 🔍 Typed lookup of a registered {@link Component} by surface id. */
export function getPlatformComponent<T extends Component<unknown>>(platform: Platform, surfaceId: string): T | undefined {
	return platform.getComponent(surfaceId) as T | undefined;
}
//#endregion 🔖Component

//#region 🔖ContextKeys
/** @emoji 🔑 Opaque context bag for `SurfaceSelector.when` resolution (products inject evaluators). */
export type ContextKey = string;

export type ContextKeyResolver = (when: string | undefined) => boolean;

export const matchAllContext: ContextKeyResolver = (when) => when === undefined || when === "" || when === "*";
//#endregion 🔖ContextKeys

//#region 🔖Capability
/** @emoji 🏷 Semantic affordance string attached to surfaces and matched by plugin selectors. */
export type Capability = string;

/** @emoji ✅ True when `required` ⊆ `available` as a set. */
export function capabilitiesSatisfy(available: readonly Capability[], required: readonly Capability[]): boolean {
	for (const c of required) {
		if (!available.includes(c)) return false;
	}
	return true;
}
//#endregion 🔖Capability

//#region 🔖SurfaceDefinition
/** @emoji 🪟 Extension-capable area: typed API factory + contribution application. */
export interface SurfaceDefinition<TApi = unknown, TContribution = unknown> {
	readonly id: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly kind: "window" | "toolbar" | "panel" | "overlay" | "tool" | "menu" | "inspector" | "analysis" | string;
	readonly capabilities: readonly Capability[];
	createApi(ctx: SurfaceContext): TApi;
	applyContribution(contribution: TContribution, ctx: SurfaceContext, api: TApi): Disposable;
}

/** @emoji 🔗 Typed pair used in {@link PlatformDefinition} surface maps. */
export interface SurfaceBinding<TApi, TContribution> {
	readonly api: TApi;
	readonly contributions: TContribution;
}
//#endregion 🔖SurfaceDefinition

//#region 🔖WindowKindRuntime
/** @emoji 🪟 Declarative window kind; React renderer maps `bodyKey` to a component. */
export class WindowKindRuntime extends BaseWindowKindRuntime {
	readonly capabilities: Capability[] = [];
	readonly surfaces: SurfaceDefinition[] = [];
	commands: SearchItemSpec[] = [];

	constructor(
		id: string,
		label: string,
		bodyKey: string,
		iconId?: string,
		measures: readonly WindowMeasure[] = [],
		capabilities?: readonly Capability[],
		templates: readonly WindowTemplate[] = [],
	) {
		super(id, label, bodyKey, iconId, measures, templates);
		if (capabilities?.length) this.capabilities.push(...capabilities);
	}
}
//#endregion 🔖WindowKindRuntime

//#region 🔖ModeRuntime
/** @emoji 🎚 Single app mode: toolbars, window kinds, and side tab specs. */
export class ModeRuntime extends BaseModeRuntime {
	commands: SearchItemSpec[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string | null) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};

	declare windowKinds: WindowKindRuntime[];

	constructor(id: string, label: string, iconId: string | undefined) {
		super(id, label, iconId);
	}
}
//#endregion 🔖ModeRuntime

//#region 🔖ResolvedState
/** @emoji 📸 Merged view of app + active mode used by the React product bridge. */
export interface ResolvedAppState {
	readonly id: string;
	readonly activeModeId: string | null;
	readonly label: string;
	readonly iconId: string | undefined;
	readonly tools: AppTools | undefined;
	readonly commands: SearchItemSpec[];
	readonly windowKinds: readonly WindowKindRuntime[];
	readonly namedLayouts: readonly NamedLayout[];
	readonly defaultLayout: WindowLayout;
	readonly panelTabs: SideTabSpec[];
	readonly footerItems: FooterItem[];
	readonly findItems: FindItem[];
	readonly onFindSelect?: (itemId: string) => void;
	readonly onActiveWindowChange?: (windowKindId: string | null) => void;
	readonly selection: Record<string, unknown>;
	readonly hover: Record<string, unknown>;
	readonly options: Record<string, unknown>;
}

/** @emoji 🧮 Resolves active mode overlays for the platform product shell. */
export function resolveAppState(app: AppRuntime, requestedModeId?: string | null): ResolvedAppState {
	const mode = resolveMode(app, requestedModeId) as ModeRuntime | null;
	const mergedWindowKinds = mergeById(app.windowKinds, mode?.windowKinds) ?? app.windowKinds;
	const mergedPanelTabs = mergeById(app.panelTabs, mode?.panelTabs) ?? app.panelTabs;
	return {
		id: app.id,
		activeModeId: mode?.id ?? null,
		label: mode?.label ?? app.label,
		iconId: mode?.iconId ?? app.iconId,
		tools: mergeAppTools(app.tools, mode?.tools),
		commands: mergeSearchItems(app.commands, mode?.commands) ?? app.commands,
		windowKinds: mergedWindowKinds,
		namedLayouts: mergeNamedLayouts(app.namedLayouts, mode?.namedLayouts),
		defaultLayout: mode?.defaultLayout ?? app.defaultLayout,
		panelTabs: mergedPanelTabs,
		footerItems: mergeById(app.footerItems, mode?.footerItems) ?? app.footerItems,
		findItems: mergeById(app.findItems, mode?.findItems) ?? app.findItems,
		onFindSelect: mode?.onFindSelect ?? app.onFindSelect,
		onActiveWindowChange: mode?.onActiveWindowChange ?? app.onActiveWindowChange,
		selection: { ...app.selection, ...(mode?.selection ?? {}) },
		hover: { ...app.hover, ...(mode?.hover ?? {}) },
		options: { ...app.options, ...(mode?.options ?? {}) },
	};
}
//#endregion 🔖ResolvedState

//#region 🔖AppRuntime
/** @emoji 🧩 One registered app with modes, layout, and a primary {@link Controller}. */
export class AppRuntime extends BaseAppRuntime {
	commands: SearchItemSpec[] = [];
	findItems: FindItem[] = [];
	onFindSelect?: (itemId: string) => void;
	onActiveWindowChange?: (windowKindId: string | null) => void;
	selection: Record<string, unknown> = {};
	hover: Record<string, unknown> = {};
	options: Record<string, unknown> = {};

	declare modes: ModeRuntime[];
	declare windowKinds: WindowKindRuntime[];

	constructor(
		id: string,
		label: string,
		iconId: string | undefined,
		controller: import("@semio-tech/framework-core").Controller,
		layout: WindowLayout,
		windowKinds: readonly WindowKindRuntime[],
	) {
		super(id, label, iconId, controller, layout, windowKinds);
	}

	override addMode(mode: ModeRuntime): void {
		super.addMode(mode);
	}

	override resolve(requestedModeId?: string | null): ResolvedAppState {
		const modeId = requestedModeId ?? this.getActiveModeId();
		return resolveAppState(this, modeId);
	}
}
//#endregion 🔖AppRuntime

/** @emoji 🧭 Resolves the command palette rows visible for the active UI/app/mode/window scope. */
export function resolveCommandPaletteItems(platform: Platform, app: ResolvedAppState, activeWindowKindId?: string | null): SearchItemSpec[] {
	const uiCommands = mergeSearchItems(platform.searchItems, platform.commands) ?? platform.commands;
	const windowKind = activeWindowKindId ? app.windowKinds.find((entry) => entry.id === activeWindowKindId) : undefined;
	return mergeSearchItems(mergeSearchItems(uiCommands, app.commands), windowKind?.commands) ?? [];
}

//#region 🔖WindowBodyViewContext
/** @emoji 🪟 View context for declarative window bodies: platform snapshot without DOM or React roots. */
export interface WindowBodyViewContext {
	readonly platform: Platform;
	readonly windowKindId: string;
	readonly bodyKey: string;
	readonly activeModeId: string | null;
	readonly generation: number;
}

const windowBodyByKey = new Map<string, (ctx: WindowBodyViewContext) => UiNode>();

/** @emoji 📝 Registers a framework-free window body tree for `bodyKey` (host renders DOM). */
export function registerWindowBody(bodyKey: string, build: (ctx: WindowBodyViewContext) => UiNode): void {
	windowBodyByKey.set(bodyKey, (ctx) => {
		const node = build(ctx);
		assertCanvasOnlyWindowBody(bodyKey, node);
		return node;
	});
}

/** @emoji 🔍 Returns the declarative builder registered for `bodyKey`, if any. */
export function getWindowBodyFactory(bodyKey: string): ((ctx: WindowBodyViewContext) => UiNode) | undefined {
	return windowBodyByKey.get(bodyKey);
}

/** @emoji 🧹 Removes a declarative window registration (tests / hot reload). */
export function unregisterWindowBody(bodyKey: string): void {
	windowBodyByKey.delete(bodyKey);
}
//#endregion 🔖WindowBodyViewContext

//#region 🔖SidePanelBodyViewContext
/** @emoji 📑 View context for declarative side-panel tab bodies (same snapshot fields as window bodies). */
export type SidePanelBodyViewContext = WindowBodyViewContext;

const sidePanelBodyByKey = new Map<string, (ctx: SidePanelBodyViewContext) => UiTreeNode>();

function assertSidePanelTreeBody(bodyKey: string, node: UiNode): asserts node is UiTreeNode {
	if (node.type !== "tree") {
		throw new Error(`Declarative side-panel body "${bodyKey}" must be type "tree". Found "${node.type}".`);
	}
	if (!node.sections.length) {
		throw new Error(`Declarative side-panel body "${bodyKey}" must have at least one section.`);
	}
}

/** @emoji 📝 Registers a framework-free side-panel tree for `bodyKey`. */
export function registerSidePanelBody(bodyKey: string, build: (ctx: SidePanelBodyViewContext) => UiTreeNode): void {
	sidePanelBodyByKey.set(bodyKey, (ctx) => {
		const node = build(ctx);
		assertSidePanelTreeBody(bodyKey, node);
		return node;
	});
}

/** @emoji 🔍 Returns the declarative side-panel builder for `bodyKey`, if any. */
export function getSidePanelBodyFactory(bodyKey: string): ((ctx: SidePanelBodyViewContext) => UiTreeNode) | undefined {
	return sidePanelBodyByKey.get(bodyKey);
}

/** @emoji 🧹 Removes a declarative side-panel registration (tests). */
export function unregisterSidePanelBody(bodyKey: string): void {
	sidePanelBodyByKey.delete(bodyKey);
}
//#endregion 🔖SidePanelBodyViewContext

//#region 🔖PlatformDefinition
/** @emoji 🧭 Static product graph: apps, modes, window kinds, and surfaces (serializable + typed). */
export interface WindowKindDefinition {
	readonly id: string;
	readonly appId: string;
	readonly modeId: string;
	readonly kind: "table" | "diagram" | "scene" | string;
	readonly label: string;
	readonly capabilities: readonly Capability[];
	readonly bodyKey?: string;
	readonly iconId?: string;
	readonly measures?: readonly WindowMeasure[];
	readonly surfaces: readonly SurfaceDefinition[];
}

export interface ModeDefinition {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly windowKinds: readonly WindowKindDefinition[];
	readonly defaultLayout?: WindowLayout;
	readonly tools?: AppTools;
	readonly panelTabs?: readonly SideTabSpec[];
}

export interface AppDefinition {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly modes: readonly ModeDefinition[];
	readonly defaultModeId?: string;
}

export interface PlatformDefinition<TProductApi = unknown> {
	readonly id: string;
	readonly name: string;
	readonly apiVersion: string;
	readonly apps: readonly AppDefinition[];
	createPlatformApi(ctx: PluginContext): TProductApi;
}
//#endregion 🔖PlatformDefinition

//#region 🔖SurfaceContext
/** @emoji 🧩 Activation context for a single {@link SurfaceDefinition} instance. */
export interface SurfaceContext<TSurfaceId extends string = string> {
	readonly surfaceId: TSurfaceId;
	readonly productId: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly platform: Platform;
	readonly activeModeId: string | null;
	readonly generation: number;
}
//#endregion 🔖SurfaceContext

//#region 🔖SurfaceSelector
/** @emoji 🧭 Declarative filter for routing contributions to surfaces. */
export interface SurfaceSelector {
	readonly product?: string;
	readonly app?: string;
	readonly mode?: string;
	readonly windowKind?: string;
	readonly surface?: string;
	readonly kind?: string;
	readonly capabilities?: readonly Capability[];
	readonly when?: string;
}

/** @emoji ✅ True when `selector` matches the routing row derived from a surface definition. */
export function matchesSurface(selector: SurfaceSelector, row: SurfaceRoutingRow, resolveWhen: ContextKeyResolver = matchAllContext): boolean {
	if (selector.product && selector.product !== row.productId) return false;
	if (selector.app && selector.app !== row.appId) return false;
	if (selector.mode && selector.mode !== row.modeId) return false;
	if (selector.windowKind && selector.windowKind !== row.windowKindId) return false;
	if (selector.surface && selector.surface !== row.surfaceId) return false;
	if (selector.kind && selector.kind !== row.surfaceKind) return false;
	if (selector.capabilities?.length && !capabilitiesSatisfy(row.capabilities, selector.capabilities)) return false;
	if (!resolveWhen(selector.when)) return false;
	return true;
}

/** @emoji 📇 Flattened surface identity used by {@link SurfaceRouter}. */
export interface SurfaceRoutingRow {
	readonly productId: string;
	readonly appId: string;
	readonly modeId: string;
	readonly windowKindId: string;
	readonly surfaceId: string;
	readonly surfaceKind: string;
	readonly capabilities: readonly Capability[];
	readonly surface: SurfaceDefinition;
}
//#endregion 🔖SurfaceSelector

//#region 🔖ContributionRoute
/** @emoji 🛤 One plugin-authored routing rule: selector + opaque contribution payload. */
export interface ContributionRoute {
	readonly pluginId: string;
	readonly where: SurfaceSelector;
	readonly contribution: unknown;
}
//#endregion 🔖ContributionRoute

//#region 🔖ContributionRegistry
/** @emoji 📚 Collects {@link ContributionRoute} rows before {@link SurfaceRouter} applies them. */
export class ContributionRegistry {
	private readonly routes: ContributionRoute[] = [];

	add(route: ContributionRoute): void {
		this.routes.push(route);
	}

	list(): readonly ContributionRoute[] {
		return this.routes;
	}

	clear(): void {
		this.routes.length = 0;
	}
}
//#endregion 🔖ContributionRegistry

//#region 🔖SurfaceRouter
/** @emoji 🧭 Walks product graph + runtime apps and applies contributions to matching surfaces. */
export class SurfaceRouter {
	static flattenFromPlatformDefinition(product: PlatformDefinition, resolveWhen: ContextKeyResolver = matchAllContext): SurfaceRoutingRow[] {
		const rows: SurfaceRoutingRow[] = [];
		for (const app of product.apps) {
			for (const mode of app.modes) {
				for (const wk of mode.windowKinds) {
					for (const surface of wk.surfaces) {
						const caps = [...new Set([...wk.capabilities, ...surface.capabilities])];
						rows.push({
							productId: product.id,
							appId: app.id,
							modeId: mode.id,
							windowKindId: wk.id,
							surfaceId: surface.id,
							surfaceKind: surface.kind,
							capabilities: caps,
							surface,
						});
					}
				}
			}
		}
		void resolveWhen;
		return rows;
	}

	static flattenFromRuntimeApps(productId: string, apps: readonly AppRuntime[], resolveWhen: ContextKeyResolver = matchAllContext): SurfaceRoutingRow[] {
		const rows: SurfaceRoutingRow[] = [];
		for (const app of apps) {
			const modeId = app.getActiveModeId();
			const resolved = app.resolve(modeId);
			for (const wk of resolved.windowKinds) {
				const implicitWindowSurfaceId = `framework.window:${app.id}:${resolved.activeModeId ?? "default"}:${wk.id}`;
				const implicit: SurfaceDefinition = {
					id: implicitWindowSurfaceId,
					appId: app.id,
					modeId: resolved.activeModeId ?? "default",
					windowKindId: wk.id,
					kind: "window",
					capabilities: [...wk.capabilities],
					createApi: () => ({}),
					applyContribution: () => ({ dispose: () => undefined }),
				};
				rows.push({
					productId,
					appId: app.id,
					modeId: resolved.activeModeId ?? "default",
					windowKindId: wk.id,
					surfaceId: implicit.id,
					surfaceKind: implicit.kind,
					capabilities: [...wk.capabilities],
					surface: implicit,
				});
				for (const surface of wk.surfaces) {
					const caps = [...new Set([...wk.capabilities, ...surface.capabilities])];
					rows.push({
						productId,
						appId: app.id,
						modeId: resolved.activeModeId ?? "default",
						windowKindId: wk.id,
						surfaceId: surface.id,
						surfaceKind: surface.kind,
						capabilities: caps,
						surface,
					});
				}
			}
		}
		void resolveWhen;
		return rows;
	}

	static applyRoutes(
		routes: readonly ContributionRoute[],
		rows: readonly SurfaceRoutingRow[],
		buildContext: (row: SurfaceRoutingRow) => SurfaceContext,
		resolveWhen: ContextKeyResolver = matchAllContext,
	): Disposable {
		const disposables: Disposable[] = [];
		for (const route of routes) {
			for (const row of rows) {
				const selector: SurfaceSelector = { ...route.where, product: route.where.product ?? row.productId };
				if (!matchesSurface(selector, row, resolveWhen)) continue;
				const ctx = buildContext(row);
				const api = row.surface.createApi(ctx);
				disposables.push(row.surface.applyContribution(route.contribution, ctx, api));
			}
		}
		return {
			dispose: () => {
				for (const d of disposables.splice(0)) d.dispose();
			},
		};
	}
}
//#endregion 🔖SurfaceRouter

//#region 🔖PluginContext
/** @emoji 🔌 Disposable returned from {@link PluginContext.subscribe}. */
export interface PluginSubscription {
	dispose(): void;
}

/** @emoji 🧰 Activation context: product runtime, manifest, and registration helpers (VS Code `ExtensionContext` analogue). */
export class PluginContext {
	private readonly disposables: PluginSubscription[] = [];

	constructor(
		readonly platform: Platform,
		readonly manifest: PluginManifest,
	) {}

	registerWindowBody(bodyKey: string, build: (ctx: WindowBodyViewContext) => UiNode): void {
		registerWindowBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterWindowBody(bodyKey),
		});
	}

	registerSidePanelBody(bodyKey: string, build: (ctx: SidePanelBodyViewContext) => UiNode): void {
		registerSidePanelBody(bodyKey, build);
		this.disposables.push({
			dispose: () => unregisterSidePanelBody(bodyKey),
		});
	}

	addContributedApps(getController: (controllerId: string) => Controller | undefined): void {
		for (const spec of this.manifest.contributes.apps ?? []) {
			const controller = getController(spec.controllerId);
			if (!controller) continue;
			const windowKinds = spec.windowKinds.map((wk) => {
				const windowKind = new WindowKindRuntime(wk.id, wk.label, wk.bodyKey, wk.iconId, wk.measures);
				if (wk.commands?.length) windowKind.commands = [...wk.commands];
				return windowKind;
			});
			const app = new AppRuntime(spec.id, spec.label, spec.iconId, controller, spec.defaultLayout, windowKinds);
			if (spec.defaultModeId) app.defaultModeId = spec.defaultModeId;
			if (spec.tools) app.tools = spec.tools;
			if (spec.commands?.length) app.commands = [...spec.commands];
			if (spec.panelTabs?.length) app.panelTabs = [...spec.panelTabs];
			if (spec.footerItems?.length) app.footerItems = [...spec.footerItems];
			if (spec.findItems?.length) app.findItems = [...spec.findItems];
			for (const modeSpec of spec.modes ?? []) {
				const mode = new ModeRuntime(modeSpec.id, modeSpec.label, modeSpec.iconId);
				if (modeSpec.tools) mode.tools = modeSpec.tools;
				if (modeSpec.commands?.length) mode.commands = [...modeSpec.commands];
				if (modeSpec.windowKinds?.length) {
					mode.windowKinds = modeSpec.windowKinds.map((wk) => {
						const windowKind = new WindowKindRuntime(wk.id, wk.label, wk.bodyKey, wk.iconId, wk.measures);
						if (wk.commands?.length) windowKind.commands = [...wk.commands];
						return windowKind;
					});
				}
				if (modeSpec.defaultLayout) mode.defaultLayout = modeSpec.defaultLayout;
				app.addMode(mode);
			}
			this.platform.addApp(app);
			this.disposables.push({
				dispose: () => {
					const index = this.platform.apps.findIndex((entry) => entry.id === spec.id);
					if (index >= 0) this.platform.apps.splice(index, 1);
				},
			});
		}
	}

	subscribe(listener: PlatformSubscriber): PluginSubscription {
		const unsubscribe = this.platform.subscribe(listener);
		const sub: PluginSubscription = {
			dispose: () => unsubscribe(),
		};
		this.disposables.push(sub);
		return sub;
	}

	disposeAll(): void {
		for (const disposable of this.disposables.splice(0)) disposable.dispose();
	}
}
//#endregion 🔖PluginContext

//#region 🔖PluginManifest
/** @emoji 🧩 Static app contribution merged by {@link PluginHost} before {@link AppRuntime} construction. */
export interface PluginManifestAppContribute {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly windowKinds: readonly {
		readonly id: string;
		readonly label: string;
		readonly bodyKey: string;
		readonly iconId?: string;
		readonly measures?: readonly WindowMeasure[];
		readonly commands?: readonly SearchItemSpec[];
	}[];
	readonly defaultLayout: WindowLayout;
	readonly defaultModeId?: string;
	readonly modes?: readonly {
		readonly id: string;
		readonly label: string;
		readonly iconId?: string;
		readonly tools?: AppTools;
		readonly commands?: readonly SearchItemSpec[];
		readonly windowKinds?: readonly { readonly id: string; readonly label: string; readonly bodyKey: string; readonly iconId?: string; readonly measures?: readonly WindowMeasure[]; readonly commands?: readonly SearchItemSpec[] }[];
		readonly defaultLayout?: WindowLayout;
	}[];
	readonly tools?: AppTools;
	readonly commands?: readonly SearchItemSpec[];
	readonly panelTabs?: readonly SideTabSpec[];
	readonly footerItems?: readonly FooterItem[];
	readonly findItems?: readonly FindItem[];
}

/** @emoji 📦 VS Code–style `contributes` block (serializable); runtime bodies register in {@link PluginModule.activate}. */
export interface PluginManifestContributes {
	readonly apps?: readonly PluginManifestAppContribute[];
	readonly commands?: readonly {
		readonly id: string;
		readonly controllerId: string;
		readonly command: string;
		readonly title?: string;
	}[];
}

/** @emoji 🧾 Extension package descriptor (id + contributes); optional {@link PluginModule}. */
export interface PluginManifest {
	readonly id: string;
	readonly label?: string;
	readonly version?: string;
	readonly target?: { readonly product: string; readonly api: string };
	readonly contributes: PluginManifestContributes;
}

/** @emoji 🧩 Runtime plugin module (`activate` / `deactivate`). */
export interface PluginModule {
	readonly id: string;
	activate(context: PluginContext): void | Promise<void>;
	deactivate?(): void | Promise<void>;
}

/** @emoji 🏗 Loads plugin manifests, activates modules, and owns contributed {@link AppRuntime} rows. */
export class PluginHost {
	private readonly plugins = new Map<string, { manifest: PluginManifest; module?: PluginModule }>();
	private readonly contexts = new Map<string, PluginContext>();
	private activated = false;

	constructor(readonly platform: Platform) {}

	register(manifest: PluginManifest, module?: PluginModule): void {
		if (module && module.id !== manifest.id) {
			throw new Error(`Plugin module id "${module.id}" does not match manifest id "${manifest.id}".`);
		}
		this.plugins.set(manifest.id, { manifest, module });
	}

	getControllerById(controllerId: string): Controller | undefined {
		for (const app of this.platform.apps) {
			if (app.controller.id === controllerId) return app.controller;
		}
		return undefined;
	}

	async activateAll(getController: (controllerId: string) => Controller | undefined): Promise<void> {
		if (this.activated) return;
		this.activated = true;
		for (const { manifest, module } of this.plugins.values()) {
			const context = new PluginContext(this.platform, manifest);
			this.contexts.set(manifest.id, context);
			context.addContributedApps(getController);
			if (module) await module.activate(context);
		}
	}

	async deactivateAll(): Promise<void> {
		for (const [id, { module }] of [...this.plugins.entries()].reverse()) {
			await module?.deactivate?.();
			this.contexts.get(id)?.disposeAll();
			this.contexts.delete(id);
		}
		this.activated = false;
	}
}
//#endregion 🔖PluginManifest

//#region 🔖PlatformPlugin
/** @emoji 🧩 Typed product plugin: manifest target, optional per-surface activation, and selector-based contributions. */
export interface PlatformPlugin<TProductApi = unknown, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>> = Record<string, SurfaceBinding<unknown, unknown>>> {
	readonly id: string;
	readonly target: { readonly product: string; readonly api: string };
	readonly manifest?: PluginManifest;
	activate?(ctx: PluginContext, product: TProductApi): void | Promise<void>;
	deactivate?(): void | Promise<void>;
	surfaces?: { [K in keyof TSurfaceMap]?: (ctx: SurfaceContext<K & string>, surface: TSurfaceMap[K]["api"]) => Disposable | Promise<Disposable> };
	contributes?: { readonly selectors?: readonly ContributionRoute[] };
}

/** @emoji ✅ Identity helper for authoring {@link PlatformPlugin} definitions. */
export function definePlatformPlugin<TProductApi, TSurfaceMap extends Record<string, SurfaceBinding<unknown, unknown>>>(
	plugin: PlatformPlugin<TProductApi, TSurfaceMap>,
): PlatformPlugin<TProductApi, TSurfaceMap> {
	return plugin;
}
//#endregion 🔖PlatformPlugin

//#region 🔖PlatformPluginActivationHost
/** @emoji 🎛 Activates {@link PlatformPlugin} instances: product lifecycle + surface handlers + routed contributions. */
export class PlatformPluginActivationHost<TProductApi = unknown> {
	private readonly disposables: Disposable[] = [];
	private productApi: TProductApi | undefined;

	constructor(
		readonly platform: Platform,
		readonly productId: string,
		readonly createApi: (ctx: PluginContext) => TProductApi,
	) {}

	async activateAll(plugins: readonly PlatformPlugin<TProductApi>[], getController: (controllerId: string) => Controller | undefined): Promise<void> {
		void getController;
		const bootstrapCtx = new PluginContext(this.platform, { id: "__product", contributes: {} });
		this.productApi ??= this.createApi(bootstrapCtx);
		const rows = () => SurfaceRouter.flattenFromRuntimeApps(this.productId, this.platform.apps);
		for (const plugin of plugins) {
			const manifest: PluginManifest = plugin.manifest ?? { id: plugin.id, contributes: {} };
			const ctx = new PluginContext(this.platform, manifest);
			await plugin.activate?.(ctx, this.productApi!);
			const flat = rows();
			for (const row of flat) {
				const handler = plugin.surfaces?.[row.surfaceId as keyof typeof plugin.surfaces];
				if (!handler) continue;
				const sctx: SurfaceContext = {
					surfaceId: row.surfaceId,
					productId: this.productId,
					appId: row.appId,
					modeId: row.modeId,
					windowKindId: row.windowKindId,
					platform: this.platform,
					activeModeId: this.platform.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.platform.generation,
				};
				const result = await handler(sctx as SurfaceContext<string>, {} as never);
				if (result && typeof (result as Disposable).dispose === "function") {
					this.disposables.push(result as Disposable);
				}
			}
			const registry = new ContributionRegistry();
			for (const route of plugin.contributes?.selectors ?? []) {
				registry.add({ ...route, pluginId: plugin.id });
			}
			this.disposables.push(
				SurfaceRouter.applyRoutes(registry.list(), flat, (row) => ({
					surfaceId: row.surfaceId,
					productId: this.productId,
					appId: row.appId,
					modeId: row.modeId,
					windowKindId: row.windowKindId,
					platform: this.platform,
					activeModeId: this.platform.getActiveApp()?.getActiveModeId() ?? null,
					generation: this.platform.generation,
				})),
			);
		}
	}

	disposeAll(): void {
		for (const d of this.disposables.splice(0)) d.dispose();
	}
}
//#endregion 🔖PlatformPluginActivationHost

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("canvas-only declarative window bodies", () => {
		it("accepts lone puzzle and table nodes", () => {
			expect(isCanvasOnlyWindowBody(buildPuzzle3dWindowBody("s", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildPuzzle2dWindowBody("b", "c", "pane"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildTableWindowBody("t", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildVirtualFileSystemWindowBody("vfs", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody(buildPanelWindowBody("p", "c"))).toBe(true);
			expect(isCanvasOnlyWindowBody({ type: "text", value: "loading" })).toBe(true);
		});

		it("rejects window bodies with toolbar buttons", () => {
			expect(() =>
				assertCanvasOnlyWindowBody("bad", {
					type: "stack",
					direction: "vertical",
					padding: "none",
					children: [
						{
							type: "button",
							label: "Nope",
							command: { controllerId: "c", command: "x" },
						},
						buildPuzzle5dWindowBody("s", "c"),
					],
				}),
			).toThrow(/table, virtualFileSystem, puzzle2d, puzzle3d, puzzle5d, or cad/);
		});
	});

	describe("Platform", () => {
		it("constructs from PlatformSpec metadata", () => {
			const platform = new Platform({ id: "demo", name: "Demo", defaultActiveAppId: "home" });
			expect(platform.id).toBe("demo");
			expect(platform.name).toBe("Demo");
			expect(platform.activeAppId).toBe("home");
		});
	});

	describe("VirtualFileSystemController", () => {
		it("loads children only for expanded nodes per app", () => {
			const platform = new Platform({ id: "vfs", name: "VFS" });
			const ctrl = registerPlatformVirtualFileSystemDemo(platform);
			const scopeA: VirtualFileSystemScope = {
				appId: PlatformVirtualFileSystemDemoController.APP_A,
				surfaceId: virtualFileSystemSurfaceId(PlatformVirtualFileSystemDemoController.APP_A),
			};
			let model = ctrl.buildVirtualFileSystemModel(scopeA);
			expect(model.rows.map((row) => row.id)).toEqual(["folder-models", "branch-alpha"]);
			expect(model.rows.every((row) => !row.expanded)).toBe(true);
			expect(ctrl.visibleVirtualFileSystemNodes(scopeA).map((node) => node.id)).toEqual(["folder-models", "branch-alpha"]);
			ctrl.run("toggleVirtualFileSystemExpand", { ...scopeA, nodeId: "folder-models" });
			model = ctrl.buildVirtualFileSystemModel(scopeA);
			expect(model.rows.map((row) => row.id)).toEqual(["folder-models", "leaf-capsule", "branch-alpha"]);
			expect(ctrl.visibleVirtualFileSystemNodes(scopeA).map((node) => node.id)).toEqual([
				"folder-models",
				"leaf-capsule",
				"branch-alpha",
			]);
			const scopeB: VirtualFileSystemScope = {
				appId: PlatformVirtualFileSystemDemoController.APP_B,
				surfaceId: virtualFileSystemSurfaceId(PlatformVirtualFileSystemDemoController.APP_B),
			};
			expect(ctrl.buildVirtualFileSystemModel(scopeB).rows.map((row) => row.id)).toEqual(["branch-b1"]);
		});

		it("replaces row selection from setVirtualFileSystemRowSelection", () => {
			const platform = new Platform({ id: "vfs", name: "VFS" });
			const ctrl = registerPlatformVirtualFileSystemDemo(platform);
			const scope: VirtualFileSystemScope = {
				appId: PlatformVirtualFileSystemDemoController.APP_A,
				surfaceId: virtualFileSystemSurfaceId(PlatformVirtualFileSystemDemoController.APP_A),
			};
			ctrl.run("setVirtualFileSystemRowSelection", {
				...scope,
				rowIds: ["folder-models", "branch-alpha"],
				anchorRowId: "folder-models",
			});
			expect(ctrl.buildVirtualFileSystemModel(scope).selectedRowIds).toEqual(["folder-models", "branch-alpha"]);
		});

		it("virtualFileSystemUsesAsyncChildren enables async loader path", async () => {
			class AsyncVfsDemoController extends VirtualFileSystemController {
				constructor(commandBus: CommandBus, hostNotify: () => void) {
					super("async-vfs-demo", commandBus, hostNotify);
				}
				protected override virtualFileSystemUsesAsyncChildren(): boolean {
					return true;
				}
				protected override getSchema(_scope: VirtualFileSystemScope): VirtualFileSystemSchemaModel {
					return PLATFORM_VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA;
				}
				protected override getRoot(_scope: VirtualFileSystemScope): VirtualFileSystemNodeRecord {
					return {
						id: "root",
						fileNodeKindId: "root",
						name: "Root",
						hasChildren: true,
						descriptorValues: platformVirtualFileSystemDemoDescriptorValues("workspace", "/"),
					};
				}
				protected override loadChildren(_parentId: string, _scope: VirtualFileSystemScope): readonly VirtualFileSystemNodeRecord[] {
					return [];
				}
				protected override loadChildrenAsync(
					parentId: string,
					_scope: VirtualFileSystemScope,
				): Promise<readonly VirtualFileSystemNodeRecord[]> {
					return Promise.resolve([
						{
							id: `${parentId}-async`,
							fileNodeKindId: "leaf",
							name: "Async child",
							parentId,
							hasChildren: false,
							descriptorValues: platformVirtualFileSystemDemoDescriptorValues("leaf", "/Async"),
						},
					]);
				}
			}
			const bus = new CommandBus();
			const ctrl = new AsyncVfsDemoController(bus, () => {});
			const scope: VirtualFileSystemScope = { appId: "async", surfaceId: "vfs:async" };
			expect(ctrl.virtualFileSystemUsesAsyncChildren()).toBe(true);
			const rows = await ctrl.loadChildrenAsync("root", scope);
			expect(rows.map((row) => row.id)).toEqual(["root-async"]);
		});
	});

	describe("Component registry", () => {
		it("registers components by surface id and refreshes models", () => {
			class DemoTable extends Table {
				override buildSnapshot(): TableModel {
					return {
						columns: [{ id: "name", label: "Name" }],
						rows: [{ id: "1", cells: { name: "alpha" } }],
					};
				}
			}
			const platform = new Platform({ id: "demo", name: "Demo" });
			const table = new DemoTable("surface/table/v1", "ctrl");
			registerPlatformComponent(platform, table);
			table.refresh();
			const resolved = getPlatformComponent<DemoTable>(platform, "surface/table/v1");
			expect(resolved?.getSnapshot().rows[0]?.cells.name).toBe("alpha");
		});
	});

	describe("PluginHost", () => {
		it("merges contributed apps and declarative window bodies", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new Platform();
			const ctrl = new TCtrl("ext-ctrl", bus, () => rt.notify());
			const host = new PluginHost(rt);
			host.register(
				{
					id: "demo.ext",
					contributes: {
						apps: [
							{
								id: "demo-app",
								label: "Demo",
								controllerId: "ext-ctrl",
								windowKinds: [{ id: "main", label: "Main", bodyKey: "demo.ext.main" }],
								defaultLayout: createTabStackLayout(["main"], ["Main"]),
							},
						],
					},
				},
				{
					id: "demo.ext",
					activate(ctx) {
						ctx.registerWindowBody("demo.ext.main", () => ({
							type: "text",
							value: "hello",
						}));
					},
				},
			);
			await host.activateAll((id) => (id === "ext-ctrl" ? ctrl : undefined));
			expect(rt.apps.some((app) => app.id === "demo-app")).toBe(true);
			const factory = getWindowBodyFactory("demo.ext.main");
			expect(factory?.({ platform: rt, windowKindId: "main", bodyKey: "demo.ext.main", activeModeId: null, generation: 0 }).type).toBe("text");
		});
	});

	describe("matchesSurface", () => {
		const row: SurfaceRoutingRow = {
			productId: "p",
			appId: "a",
			modeId: "m",
			windowKindId: "wk",
			surfaceId: "s1",
			surfaceKind: "diagram",
			capabilities: ["diagram.read", "energy.overlay"],
			surface: {
				id: "s1",
				appId: "a",
				modeId: "m",
				windowKindId: "wk",
				kind: "diagram",
				capabilities: ["energy.overlay"],
				createApi: () => ({}),
				applyContribution: () => ({ dispose: () => undefined }),
			},
		};

		it("matches by app/mode/windowKind/surface/kind", () => {
			expect(matchesSurface({ app: "a", mode: "m", windowKind: "wk", surface: "s1", kind: "diagram" }, row)).toBe(true);
			expect(matchesSurface({ app: "other" }, row)).toBe(false);
			expect(matchesSurface({ kind: "scene" }, row)).toBe(false);
		});

		it("matches capabilities as subset", () => {
			expect(matchesSurface({ capabilities: ["energy.overlay"] }, row)).toBe(true);
			expect(matchesSurface({ capabilities: ["energy.overlay", "missing"] }, row)).toBe(false);
		});
	});

	describe("capability-only routing across implicit window surfaces", () => {
		it("routes contributions to every compatible implicit window surface", () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new Platform();
			const ctrl = new TCtrl("c", bus, () => rt.notify());
			const wk = new WindowKindRuntime("main", "Main", "demo.body", undefined, [], ["foo.overlay"]);
			rt.addApp(new AppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["main"]), [wk]));
			const flat = SurfaceRouter.flattenFromRuntimeApps("prod", rt.apps);
			let applied = 0;
			const disposable = SurfaceRouter.applyRoutes(
				[{ pluginId: "p1", where: { capabilities: ["foo.overlay"] }, contribution: {} }],
				flat,
				(row) =>
					({
						surfaceId: row.surfaceId,
						productId: "prod",
						appId: row.appId,
						modeId: row.modeId,
						windowKindId: row.windowKindId,
						platform: rt,
						activeModeId: null,
						generation: 0,
					}) as SurfaceContext,
			);
			for (const r of flat) {
				if (matchesSurface({ capabilities: ["foo.overlay"] }, r)) applied++;
			}
			expect(applied).toBe(1);
			disposable.dispose();
		});
	});

	describe("definePlatformPlugin lifecycle", () => {
		it("runs activate once and disposes surface contributions", async () => {
			class TCtrl extends Controller {
				override run(): void {}
			}
			const bus = new CommandBus();
			const rt = new Platform();
			const ctrl = new TCtrl("c", bus, () => rt.notify());
			rt.addApp(new AppRuntime("app", "App", undefined, ctrl, createTabStackLayout(["w"]), [new WindowKindRuntime("w", "W", "k", undefined, [], ["x"])]));
			let surfaceActivations = 0;
			const plugin = definePlatformPlugin({
				id: "pl",
				target: { product: "p", api: "^1" },
				surfaces: {
					[`framework.window:app:default:w`]: async () => {
						surfaceActivations++;
						return { dispose: () => undefined };
					},
				},
			});
			const host = new PlatformPluginActivationHost(rt, "p", () => ({}) as Record<string, never>);
			await host.activateAll([plugin], () => ctrl);
			expect(surfaceActivations).toBe(1);
			host.disposeAll();
		});
	});

		describe("resolveCommandPaletteItems", () => {
			it("merges ui, app, mode, and active window kind commands by active scope", () => {
				const runtime = new Platform();
				runtime.commands = [{ id: "ui", label: "UI", controllerId: "ctrl", command: "ui" }];
				runtime.searchItems = [{ id: "legacy", label: "Legacy", controllerId: "ctrl", command: "legacy" }];
				class TCtrl extends Controller {
					constructor() {
						super("ctrl", runtime.commandBus, () => runtime.notify());
					}
					run(): void {}
				}
				const baseWindow = new WindowKindRuntime("base", "Base", "base.body");
				baseWindow.commands = [{ id: "base-window", label: "Base Window", controllerId: "ctrl", command: "base-window" }];
				const app = new AppRuntime("app", "App", undefined, new TCtrl(), createTabStackLayout(["base"]), [baseWindow]);
				app.commands = [{ id: "app", label: "App", controllerId: "ctrl", command: "app" }];
				const inspect = new ModeRuntime("inspect", "Inspect", undefined);
				inspect.commands = [{ id: "mode", label: "Mode", controllerId: "ctrl", command: "mode" }];
				const inspectWindow = new WindowKindRuntime("inspect", "Inspect", "inspect.body");
				inspectWindow.commands = [{ id: "inspect-window", label: "Inspect Window", controllerId: "ctrl", command: "inspect-window" }];
				inspect.windowKinds = [inspectWindow];
				app.addMode(inspect);
				const resolved = resolveAppState(app, "inspect");

				expect(resolveCommandPaletteItems(runtime, resolved, "inspect").map((item) => item.id)).toEqual(["legacy", "ui", "app", "mode", "inspect-window"]);
				expect(resolveCommandPaletteItems(runtime, resolved, "base").map((item) => item.id)).toEqual(["legacy", "ui", "app", "mode", "base-window"]);
			});
		});
}
//#endregion 🧪Tests
