#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = import.meta.dir;
const root = join(dir, "../../../../../..");
const frPath = join(root, "elements/lib/framework/renderer/react/index.tsx");
const headLines = readFileSync(join(dir, "framework-react-head.tsx"), "utf8").split(/\r?\n/);

const midStart = headLines.findIndex((l) => l.includes("workbench-app-context"));
if (midStart < 0) throw new Error("workbench-app-context not in head");
const mid = headLines.slice(midStart).join("\n");

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

const header = `// #region 🧲Header
/** @emoji ⚛️ \`@elements/framework-react\` — React renderer for {@link @elements/framework}: declarative {@link UiNode} host (monolith). */
// #endregion 🧲Header

export type { Workbench } from "@elements/framework";

export type { Level } from "@elements/ui";
export {
	LevelProvider,
	useLevel,
	getLevelBgClass,
	getLevelHoverClass,
	getLevelActiveHoverClass,
	getLevelZClass,
	getLevelBorderElementClass,
	getLevelDivideElementClass,
} from "@elements/ui";

`;

const imports = readFileSync(join(dir, "imports-block.txt"), "utf8");

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

const out = header + imports + shellChrome + mid;
writeFileSync(frPath, out, "utf8");
console.log("rebuilt", out.split(/\r?\n/).length, "lines");
