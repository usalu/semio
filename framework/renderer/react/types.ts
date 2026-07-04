export type CommandDescriptor = {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: unknown;
};

export type StyleSpec = {
	readonly variant?: string;
	readonly size?: string;
	readonly density?: string;
};

export type UiStackNode = {
	readonly type: "stack";
	readonly direction: string;
	readonly gap?: string;
	readonly padding?: string;
	readonly children: readonly UiNode[];
};

export type UiTextNode = {
	readonly type: "text";
	readonly value: string;
	readonly emphasize?: boolean;
	readonly dataAttributes?: Readonly<Record<string, string>>;
};

export type UiButtonNode = {
	readonly type: "button";
	readonly id?: string;
	readonly iconId: string;
	readonly label: string;
	readonly command: CommandDescriptor;
	readonly style?: StyleSpec;
};

export type UiSeparatorNode = { readonly type: "separator" };

export type UiInputNode = {
	readonly type: "input";
	readonly id: string;
	readonly inputKind: string;
	readonly value: string;
	readonly placeholder?: string;
	readonly commit?: string;
	readonly onChange: CommandDescriptor;
};

export type UiSelectItem = {
	readonly value: string;
	readonly label: string;
};

export type UiSelectNode = {
	readonly type: "select";
	readonly id: string;
	readonly value: string;
	readonly items: readonly UiSelectItem[];
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

export type UiKeyValueEntry = {
	readonly label: string;
	readonly value: string;
};

export type UiKeyValueNode = {
	readonly type: "keyValue";
	readonly entries: readonly UiKeyValueEntry[];
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
	readonly classifierKind: string;
	readonly onChange: CommandDescriptor;
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

export type UiFieldNode = {
	readonly type: "field";
	readonly id: string;
	readonly label: string;
	readonly child: UiControlNode;
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
	readonly iconId?: string;
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

export type UiInspectorFieldGroup = {
	readonly id: string;
	readonly label: string;
	readonly defaultOpen?: boolean;
	readonly fields: readonly UiNode[];
};

export type Canvas2dScene = {
	readonly cameraX: number;
	readonly cameraY: number;
	readonly zoom: number;
	readonly layersJson: string;
};

export type World3dScene = {
	readonly cameraJson: string;
	readonly instancesJson: string;
};

export type NodeGraphScene = {
	readonly nodesJson: string;
	readonly edgesJson: string;
	readonly viewportJson: string;
};

export type FlowCanvasScene = {
	readonly fixtureJson: string;
	readonly operatorsJson?: string;
	readonly editable?: boolean;
};

export type TextEditorScene = {
	readonly buffer: string;
	readonly language?: string;
	readonly selectionJson?: string;
};

export type TableScene = {
	readonly columnsJson: string;
	readonly rowsJson: string;
};

export type RasterScene = {
	readonly width: number;
	readonly height: number;
	readonly pixelsBase64: string;
};

export type VirtualFileSystemScene = {
	readonly schemaJson: string;
	readonly rowsJson: string;
	readonly selectedRowIdsJson?: string;
	readonly hoveredRowId?: string;
	readonly emptyMessage?: string;
	readonly dragDropEnabled?: boolean;
};

export type UiComponentSceneNode = {
	readonly type: "componentScene";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly componentKind: string;
	readonly paneId?: string;
	readonly bindingId?: string;
	readonly canvas2d?: Canvas2dScene;
	readonly world3d?: World3dScene;
	readonly nodeGraph?: NodeGraphScene;
	readonly flowCanvas?: FlowCanvasScene;
	readonly textEditor?: TextEditorScene;
	readonly table?: TableScene;
	readonly raster?: RasterScene;
	readonly virtualFileSystem?: VirtualFileSystemScene;
};

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
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
	| UiSectionNode
	| UiTreeNode
	| UiComponentSceneNode;

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

export type WindowEngagementOption = {
	readonly id: string;
	readonly label?: string;
	readonly iconId?: string;
	readonly pressed?: boolean;
	readonly disabled?: boolean;
	readonly command?: CommandDescriptor;
};

export type WindowEngagementInput = {
	readonly id?: string;
	readonly value?: string;
	readonly placeholder?: string;
	readonly disabled?: boolean;
	readonly onChange?: CommandDescriptor;
	readonly onSubmit?: CommandDescriptor;
	readonly onRepeatLast?: CommandDescriptor;
	readonly onAbort?: CommandDescriptor;
};

export type WindowEngagementStatus = {
	readonly id: string;
	readonly text: string;
};

export type WindowEngagementPossible = {
	readonly id: string;
	readonly label: string;
	readonly detail?: string;
	readonly command?: CommandDescriptor;
};

export type WindowEngagementRingOption = {
	readonly id: string;
	readonly label: string;
	readonly disabled?: boolean;
};

export type WindowEngagementToggleGroupOption = {
	readonly id: string;
	readonly label: string;
	readonly disabled?: boolean;
};

export type WindowEngagementSelectItem = {
	readonly id: string;
	readonly value: string;
	readonly label: string;
};

export type WindowEngagementControl =
	| {
			readonly kind: "slider";
			readonly id?: string;
			readonly label?: string;
			readonly value: number;
			readonly min: number;
			readonly max: number;
			readonly step?: number;
			readonly unit?: string;
			readonly disabled?: boolean;
			readonly onChange?: CommandDescriptor;
			readonly onCommit?: CommandDescriptor;
	  }
	| {
			readonly kind: "stepper";
			readonly id?: string;
			readonly label?: string;
			readonly value: number;
			readonly min?: number;
			readonly max?: number;
			readonly step?: number;
			readonly unit?: string;
			readonly disabled?: boolean;
			readonly onChange?: CommandDescriptor;
			readonly onCommit?: CommandDescriptor;
	  }
	| {
			readonly kind: "ring";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly options: readonly WindowEngagementRingOption[];
			readonly disabled?: boolean;
			readonly onSelect?: CommandDescriptor;
	  }
	| {
			readonly kind: "toggleGroup";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly options: readonly WindowEngagementToggleGroupOption[];
			readonly disabled?: boolean;
			readonly onSelect?: CommandDescriptor;
	  }
	| {
			readonly kind: "select";
			readonly id?: string;
			readonly label?: string;
			readonly value?: string;
			readonly placeholder?: string;
			readonly items: readonly WindowEngagementSelectItem[];
			readonly disabled?: boolean;
			readonly onChange?: CommandDescriptor;
	  };

export type WindowEngagement = {
	readonly sessionActive?: boolean;
	readonly options?: readonly WindowEngagementOption[];
	readonly input?: WindowEngagementInput;
	readonly control?: WindowEngagementControl;
	readonly controls?: readonly WindowEngagementControl[];
	readonly status?: readonly WindowEngagementStatus[];
	readonly possibleEngagements?: readonly WindowEngagementPossible[];
};

export type WindowMeasure =
	| {
			readonly kind: "select";
			readonly id: string;
			readonly label?: string;
			readonly value: string;
			readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
			readonly onChange: CommandDescriptor;
	  }
	| {
			readonly kind: "slider";
			readonly id: string;
			readonly label?: string;
			readonly value: number;
			readonly min: number;
			readonly max: number;
			readonly step?: number;
			readonly onChange: CommandDescriptor;
	  }
	| {
			readonly kind: "toggle";
			readonly id: string;
			readonly iconId: string;
			readonly label?: string;
			readonly pressed: boolean;
			readonly text?: string;
			readonly onChange: CommandDescriptor;
	  }
	| {
			readonly kind: "group";
			readonly id: string;
			readonly label: string;
			readonly defaultOpen?: boolean;
			readonly children: readonly WindowMeasure[];
	  };

export type ViewState = {
	readonly activeModeId?: string;
	readonly activeWindowKindId?: string;
	readonly selectionJson?: string;
	readonly panelJson?: string;
};

export type AppDefinition = {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly controllerId: string;
	readonly modes: readonly { readonly id: string; readonly label: string }[];
	readonly defaultModeId?: string;
	readonly windowKinds: readonly {
		readonly id: string;
		readonly label: string;
		readonly bodyKey: string;
		readonly iconId?: string;
		readonly measures?: readonly WindowMeasure[];
		readonly engagement?: WindowEngagement;
	}[];
	readonly panelTabs: readonly { readonly id: string; readonly label: string; readonly group: string; readonly bodyKey: string }[];
	readonly keybindings: readonly { readonly keys: string; readonly command: CommandDescriptor }[];
	readonly namedLayouts?: readonly NamedLayout[];
	readonly defaultLayout?: WindowLayout;
};

export type PluginManifest = {
	readonly pluginId: string;
	readonly label: string;
	readonly version: string;
	readonly apps: readonly AppDefinition[];
	readonly programs: readonly { readonly programId: string; readonly appId: string; readonly label: string; readonly yields: string }[];
	readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string }[];
};

export type PluginHotSwapEvent = {
	readonly pluginId: string;
	readonly version: string;
	readonly addedApps: readonly string[];
	readonly removedApps: readonly string[];
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

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

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
