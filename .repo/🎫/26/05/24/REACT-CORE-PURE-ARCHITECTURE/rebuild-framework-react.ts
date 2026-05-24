#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = import.meta.dir;
const root = join(dir, "../../../../../..");
const frPath = join(root, "elements/lib/framework/renderer/react/index.tsx");
const broken = readFileSync(frPath, "utf8");
const head = readFileSync(join(dir, "framework-react-head.tsx"), "utf8");

const tailStart = broken.indexOf("//#region 📦workbench-app-context.tsx");
if (tailStart < 0) throw new Error("workbench-app-context not found in broken file");
const tail = broken.slice(tailStart);

let shell = readFileSync(join(dir, "shell-extract.tsx"), "utf8");
shell = shell
	.replace(/^\/\/ Domain-neutral[\s\S]*?^export enum Mode \{[\s\S]*?^}\n\n/m, "")
	.replace(/\bUIWindowLayoutWindowNode\b/g, "WindowLayoutWindowNode")
	.replace(/\bUIWindowLayoutStackNode\b/g, "WindowLayoutStackNode")
	.replace(/\bUIWindowLayoutAxisNode\b/g, "WindowLayoutAxisNode")
	.replace(/\bUIWindowLayout\b/g, "WindowLayout")
	.replace(/\bUIWindowLayoutNode\b/g, "WindowLayoutNode")
	.replace(/new DOMEventBindingController\(\)/g, "createDOMEventBinding()");

const domBinding = `type DOMListenerTarget = Pick<EventTarget, "addEventListener" | "removeEventListener">;

function createDOMEventBinding() {
	const cleanups: Array<() => void> = [];
	return {
		listen(target: DOMListenerTarget | null | undefined, type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions) {
			if (!target) return;
			target.addEventListener(type, listener, options);
			cleanups.push(() => target.removeEventListener(type, listener, options));
		},
		dispose() {
			while (cleanups.length > 0) cleanups.pop()?.();
		},
	};
}

`;

const header = head.slice(0, head.indexOf("//#region"));

const imports = `import {
	CommandBus,
	Controller,
	Workbench,
	WorkbenchApp,
	WorkbenchMode,
	WorkbenchWindowKind,
	createTabStackLayout,
	createWindowLayout,
	getDeclarativeSidePanelBodyFactory,
	getDeclarativeWindowBodyFactory,
	type ResolvedWorkbenchAppState,
	type ShellAppTools,
	type ShellFooterItem,
	type ShellSidePanelBodyViewContext,
	type ShellSideTabSpec,
	type ShellToolItem,
	type ShellWindowBodyViewContext,
	type ShellWindowMeasure,
	type UiBoardHostSurfaceNode,
	type UiButtonNode,
	type UiNode,
	type UiPanelHostSurfaceNode,
	type UiScene3DHostSurfaceNode,
	type UiSeparatorNode,
	type UiStackNode,
	type UiTableHostSurfaceNode,
	type UiTextNode,
	type WindowLayout,
	type WindowLayoutAxisNode,
	type WindowLayoutNode,
	type WindowLayoutStackNode,
	type WindowLayoutWindowNode,
} from "@elements/framework";
import {
	ArrowLeft,
	ArrowRight,
	ArrowUp,
	Check as CheckIcon,
	Filter as FilterIcon,
	Folder,
	FolderOpen as FolderOpenIcon,
	Hand as HandIcon,
	Info,
	Lasso as LassoIcon,
	LayoutGrid as LayoutGridIcon,
	MessageSquare,
	MoreHorizontal as MoreHorizontalIcon,
	MousePointer2 as MousePointerIcon,
	Plus as PlusIcon,
	Search as SearchIcon,
	Settings2 as Settings2Icon,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import * as React from "react";
import { createRoot, type Root } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import Fuse, { type FuseResult } from "fuse.js";
import { useTranslation } from "react-i18next";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import {
	BasicChatPanel,
	Button,
	ButtonCycle,
	ButtonGroup,
	ButtonGroupItem,
	CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
	Combobox,
	Footer,
	Input,
	Layout,
	Navbar,
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
	Slider,
	staticSidePanelTabDefinition,
	staticTreePanelDefinition,
	Stepper,
	Textarea,
	Toggle,
	ActionGroup,
	ActionGroupItem,
	ToolbarDivider,
	ToolbarItem,
	ToolbarZone,
	cn,
	resolveTranslationLabel,
	useCommandHotkey,
	useMediaQuery,
	type ContextMenuItem,
	type NavbarItem,
	type SidePanelTabConfig as UiSidePanelTabConfig,
	type TreePanelConfig,
} from "@elements/ui";

`;

const shellChrome = `//#region 📦shell-chrome-types.tsx

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

//#endregion 📦shell-chrome-types.tsx

//#region 📦shell-canvas.tsx
/** @emoji 🖼 Golden-layout shell: layouts, canvas portals, toolbar, search, and find. */
${domBinding}
${shell}
//#endregion 📦shell-canvas.tsx

`;

const out = header + imports + shellChrome + tail;
writeFileSync(frPath, out, "utf8");
console.log("rebuilt", out.split(/\r?\n/).length, "lines");
