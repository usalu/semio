// #region 🧲Header
/** @emoji 📜 Host-rendered UI protocol: JSON-safe trees and semantic commands (no DOM, no React in contributors). */
// #endregion 🧲Header

//#region 🔖JsonValue
export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | readonly JsonValue[] | { readonly [key: string]: JsonValue };
//#endregion 🔖JsonValue

//#region 🔖Commands
/** @emoji 🎯 Semantic command routed through the host to {@link CommandBus.dispatch}. */
export interface ShellCommandDescriptor {
	readonly controllerId: string;
	readonly command: string;
	readonly args?: JsonValue;
}
//#endregion 🔖Commands

//#region 🔖Style
/** @emoji 🎨 Tokenized chrome hints (host maps to CSS); raw CSS strings are intentionally unsupported. */
export interface ShellStyleSpec {
	readonly variant?: "default" | "subtle" | "danger" | "success";
	readonly size?: "small" | "medium" | "large";
	readonly density?: "compact" | "normal" | "comfortable";
}
//#endregion 🔖Style

//#region 🔖UiNode
export interface UiStackNode {
	readonly type: "stack";
	readonly direction: "horizontal" | "vertical";
	readonly gap?: "none" | "tight" | "standard" | "relaxed";
	readonly padding?: "none" | "standard";
	readonly children: readonly UiNode[];
}

export interface UiTextNode {
	readonly type: "text";
	readonly value: string;
	readonly emphasize?: boolean;
	readonly dataAttributes?: Readonly<Record<string, string>>;
}

export interface UiButtonNode {
	readonly type: "button";
	readonly id?: string;
	readonly label: string;
	readonly command: ShellCommandDescriptor;
	readonly style?: ShellStyleSpec;
}

export interface UiSeparatorNode {
	readonly type: "separator";
}

/** @emoji 🧊 Host-bound 3D surface: plugin supplies ids; host maps `surfaceId` to canvas implementation. */
export interface UiScene3DHostSurfaceNode {
	readonly type: "scene3d";
	readonly surfaceId: string;
	readonly controllerId: string;
}

/** @emoji 📋 Host-bound 2D board canvas; `paneId` selects the play window slot. */
export interface UiBoardHostSurfaceNode {
	readonly type: "board";
	readonly surfaceId: string;
	readonly controllerId: string;
	readonly paneId: string;
}

/** @emoji 📑 Host-bound side panel body; `surfaceId` maps to a registered panel host renderer. */
export interface UiPanelHostSurfaceNode {
	readonly type: "panel";
	readonly surfaceId: string;
	readonly controllerId: string;
}

export type UiNode =
	| UiStackNode
	| UiTextNode
	| UiButtonNode
	| UiSeparatorNode
	| UiScene3DHostSurfaceNode
	| UiBoardHostSurfaceNode
	| UiPanelHostSurfaceNode;
//#endregion 🔖UiNode

//#region 🔖ShellWindowMeasure
/** @emoji 📐 Framework-free window measure; host maps `onChange` to {@link CommandBus.dispatch}. */
export interface ShellWindowMeasureSelect {
	readonly kind: "select";
	readonly id: string;
	readonly label?: string;
	readonly value: string;
	readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
	readonly onChange: ShellCommandDescriptor;
}

export type ShellWindowMeasure = ShellWindowMeasureSelect;
//#endregion 🔖ShellWindowMeasure
