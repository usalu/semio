// #region 🧲Header
/** @emoji 🧱 Framework-react shell chrome types — shared by {@link shell-bridge} and {@link WorkbenchView} (no `@elements/ui` import). */
// #endregion 🧲Header

import type * as React from "react";

/** @emoji 👣 Footer row rendered by the workbench shell. */
export interface FooterItem {
	readonly id: string;
	readonly icon?: React.ReactNode;
	readonly text?: string;
	readonly content?: React.ReactNode;
	readonly order?: number;
	readonly onClick?: () => void;
	readonly className?: string;
	readonly disabled?: boolean;
}

/** @emoji 🌲 Minimal tree panel payload for declarative side tabs. */
export interface ShellChromeTreePanelConfig {
	readonly sections: readonly { readonly id: string; readonly content: React.ReactNode }[];
}

/** @emoji 📑 Side panel tab registration consumed by {@link WorkbenchView}. */
export interface SidePanelTabConfig {
	readonly id: string;
	readonly icon: React.ComponentType<{ readonly size?: number }>;
	readonly order?: number;
	readonly tree: ShellChromeTreePanelConfig;
}

/** @emoji 📐 Floating window measure/control descriptors (golden-layout chrome). */
export type UIWindowMeasure =
	| { readonly kind: "display"; readonly id: string; readonly label?: string; readonly content: React.ReactNode }
	| { readonly kind: "reading"; readonly id: string; readonly label?: string; readonly text: string; readonly monospace?: boolean }
	| { readonly kind: "section"; readonly id: string; readonly title: string }
	| { readonly kind: "separator"; readonly id: string }
	| {
			readonly kind: "toggle";
			readonly id: string;
			readonly label?: string;
			readonly pressed?: boolean;
			readonly defaultPressed?: boolean;
			readonly icon?: React.ReactNode;
			readonly text?: string;
			readonly onPressedChange?: (pressed: boolean) => void;
	  }
	| {
			readonly kind: "select";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly defaultValue?: string;
			readonly items: readonly { readonly id: string; readonly value: string; readonly label: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| {
			readonly kind: "combobox";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly placeholder?: string;
			readonly choices: readonly { readonly value: string; readonly label: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| { readonly kind: "button"; readonly id: string; readonly label?: string; readonly text: string; readonly icon?: React.ReactNode; readonly onClick?: () => void }
	| {
			readonly kind: "buttonCycle";
			readonly id: string;
			readonly label?: string;
			readonly value?: string;
			readonly items: readonly { readonly value: string; readonly label: string; readonly icon?: React.ReactNode; readonly text?: string; readonly id?: string }[];
			readonly onValueChange?: (value: string) => void;
	  }
	| { readonly kind: "input"; readonly id: string; readonly label?: string; readonly value?: string; readonly placeholder?: string; readonly onLazyChange?: (value: string) => void }
	| { readonly kind: "textarea"; readonly id: string; readonly label?: string; readonly value?: string; readonly placeholder?: string; readonly rows?: number; readonly onLazyChange?: (value: string) => void }
	| { readonly kind: "checkbox"; readonly id: string; readonly label?: string; readonly checked?: boolean; readonly defaultChecked?: boolean; readonly onCheckedChange?: (checked: boolean) => void }
	| { readonly kind: "radio"; readonly id: string; readonly label?: string; readonly value: string; readonly items: readonly { readonly value: string; readonly label: string }[]; readonly onChange?: (value: string) => void }
	| { readonly kind: "slider"; readonly id: string; readonly label?: string; readonly value?: number; readonly min?: number; readonly max?: number; readonly step?: number; readonly onValueChange?: (value: number) => void }
	| { readonly kind: "number"; readonly id: string; readonly label?: string; readonly value?: number; readonly min?: number; readonly max?: number; readonly step?: number; readonly onChange?: (value: number) => void }
	| { readonly kind: "color"; readonly id: string; readonly label?: string; readonly value?: string; readonly onChange?: (value: string) => void };

/** @emoji 🪟 Golden-layout window kind registration. */
export interface UIWindowKindDefinition {
	readonly id: string;
	readonly label?: string;
	readonly icon?: React.ReactNode;
	readonly component: React.ComponentType;
	readonly measures?: readonly UIWindowMeasure[];
}

/** @emoji 🧰 Toolbar item for a single category slot. */
export interface UIToolbarItem {
	readonly id: string;
	readonly icon?: React.ReactNode;
	readonly label?: string;
	readonly text?: string;
	readonly onClick?: () => void;
	readonly kind?: "button" | "toggle" | "separator";
	readonly pressed?: boolean;
	readonly onPressedChange?: (pressed: boolean) => void;
	readonly order?: number;
}

/** @emoji 🧰 Toolbar category ids shared by framework shell tool maps. */
export type AppToolCategory = "history" | "hand" | "selection" | "lasso" | "filter" | "open" | "create" | "view" | "actions" | "settings";

/** @emoji 📋 Default toolbar category order. */
export const APP_TOOL_CATEGORY_ORDER: readonly AppToolCategory[] = [
	"history",
	"hand",
	"selection",
	"lasso",
	"filter",
	"open",
	"create",
	"view",
	"actions",
	"settings",
];

/** @emoji 🗂️ Per-category toolbar tools for the workbench shell. */
export type AppTools = Partial<Record<AppToolCategory, readonly UIToolbarItem[]>>;
