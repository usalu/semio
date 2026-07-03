export type CommandDescriptor = {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: unknown;
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
};

export type UiButtonNode = {
	readonly type: "button";
	readonly id?: string;
	readonly iconId: string;
	readonly label: string;
	readonly command: CommandDescriptor;
};

export type UiSeparatorNode = { readonly type: "separator" };

export type UiInputNode = {
	readonly type: "input";
	readonly id: string;
	readonly inputKind: string;
	readonly value: string;
	readonly placeholder?: string;
	readonly onChange: CommandDescriptor;
};

export type UiSelectNode = {
	readonly type: "select";
	readonly id: string;
	readonly value: string;
	readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
	readonly onChange: CommandDescriptor;
};

export type UiToggleNode = {
	readonly type: "toggle";
	readonly id: string;
	readonly iconId: string;
	readonly label?: string;
	readonly pressed: boolean;
	readonly onChange: CommandDescriptor;
};

export type UiSliderNode = {
	readonly type: "slider";
	readonly id: string;
	readonly label?: string;
	readonly value: number;
	readonly min: number;
	readonly max: number;
	readonly step?: number;
	readonly onChange: CommandDescriptor;
};

export type UiSectionNode = {
	readonly type: "section";
	readonly id: string;
	readonly title: string;
	readonly children: readonly UiNode[];
};

export type UiTreeItemNode = {
	readonly id: string;
	readonly label: string;
	readonly iconId?: string;
	readonly expanded?: boolean;
	readonly selected?: boolean;
	readonly children: readonly UiNode[];
};

export type UiTreeNode = {
	readonly type: "tree";
	readonly id: string;
	readonly items: readonly UiTreeItemNode[];
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
	readonly textEditor?: TextEditorScene;
	readonly table?: TableScene;
	readonly raster?: RasterScene;
};

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
	| UiInputNode
	| UiSelectNode
	| UiToggleNode
	| UiSliderNode
	| UiSectionNode
	| UiTreeNode
	| UiComponentSceneNode;

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
	readonly windowKinds: readonly { readonly id: string; readonly label: string; readonly bodyKey: string }[];
	readonly panelTabs: readonly { readonly id: string; readonly label: string; readonly group: string; readonly bodyKey: string }[];
	readonly keybindings: readonly { readonly keys: string; readonly command: CommandDescriptor }[];
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
