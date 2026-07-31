#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = import.meta.dir;
const root = join(dir, "../../../../../..");
const frPath = join(root, "elements/lib/framework/renderer/react/index.tsx");
let shell = readFileSync(join(dir, "shell-extract.tsx"), "utf8");

shell = shell
  .replace(/^\/\/ Domain-neutral[\s\S]*?^export enum Mode \{[\s\S]*?^}\n\n/m, "")
  .replace(/\bUIWindowLayoutWindowNode\b/g, "WindowLayoutWindowNode")
  .replace(/\bUIWindowLayoutStackNode\b/g, "WindowLayoutStackNode")
  .replace(/\bUIWindowLayoutAxisNode\b/g, "WindowLayoutAxisNode")
  .replace(/\bUIWindowLayout\b/g, "WindowLayout")
  .replace(/\bUIWindowLayoutNode\b/g, "WindowLayoutNode")
  .replace(/export type LayoutNode = WindowLayout;/g, "export type LayoutNode = WindowLayout;")
  .replace(/export type LayoutStack = WindowLayoutStackNode;/g, "export type LayoutStack = WindowLayoutStackNode;")
  .replace(/export type LayoutRow = WindowLayoutAxisNode & \{ kind: "row" \};/g, 'export type LayoutRow = WindowLayoutAxisNode & { kind: "row" };')
  .replace(/export type LayoutColumn = WindowLayoutAxisNode & \{ kind: "column" \};/g, 'export type LayoutColumn = WindowLayoutAxisNode & { kind: "column" };');

const shellHeader = `/** @emoji 🖼️ Golden-layout shell: window layout, canvas portals, toolbar, search, and find palettes. */
`;

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

shell = shell.replace(/new DOMEventBindingController\(\)/g, "createDOMEventBinding()");

const fr = readFileSync(frPath, "utf8");
const start = fr.indexOf("//#region 📦️shell-canvas.tsx");
const end = fr.indexOf("//#endregion 📦️shell-canvas.tsx") + "//#endregion 📦️shell-canvas.tsx".length;
if (start < 0 || end < 0) throw new Error("shell-canvas region missing");

const frameworkImport = `import {
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
`;

const before = fr.slice(0, fr.indexOf("import {\n\tCommandBus,"));
const afterShell = fr.slice(end);
const uiImport = fr.slice(fr.indexOf("import {\n\tBasicChatPanel"), fr.indexOf("//#region 📦️shell-canvas.tsx"));

const rebuilt = before + frameworkImport + uiImport + `\n//#region 📦️shell-canvas.tsx\n\n` + domBinding + shellHeader + shell + `\n//#endregion 📦️shell-canvas.tsx\n` + afterShell;

writeFileSync(frPath, rebuilt, "utf8");
console.log("fixed framework-react");
