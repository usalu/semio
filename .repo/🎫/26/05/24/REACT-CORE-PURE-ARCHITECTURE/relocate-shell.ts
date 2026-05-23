#!/usr/bin/env bun
/** One-shot: move @elements/ui shell block into @elements/framework-react. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");
const uiPath = join(root, "elements/lib/react/core/index.tsx");
const frPath = join(root, "elements/lib/framework/renderer/react/index.tsx");

const ui = readFileSync(uiPath, "utf8");
const uiLines = ui.split(/\r?\n/);

const startMarker = "// #region 🎊UI";
const endMarker = "// #endregion 🎊UI";
const startIdx = uiLines.findIndex((l) => l === startMarker);
const endIdx = uiLines.findIndex((l) => l === endMarker);
if (startIdx < 0 || endIdx < 0) throw new Error(`markers not found: ${startIdx} ${endIdx}`);

let shell = uiLines.slice(startIdx + 1, endIdx).join("\n");

const skipPrefixes = [
	"// Domain-neutral composite",
	"// An app has window kinds",
	"// Every UI has a toolbar",
	"// Every app has a find",
	"// Every panel has a tree.",
	"/**",
	" * Window kind classification",
	"export enum WindowKind",
	" * UI theme classification",
	"export enum Theme",
	" * UI interaction mode",
	"export enum Mode",
];
shell = shell
	.split(/\r?\n/)
	.filter((line) => {
		if (skipPrefixes.some((p) => line.startsWith(p) || line.includes("WindowKind {"))) return false;
		if (line.match(/^export enum (WindowKind|Theme|Mode)/)) return false;
		if (line === "}" && shell.includes("WindowKind")) {
			/* keep closing braces — handled by enum block removal below */
		}
		return true;
	})
	.join("\n");

shell = shell.replace(/\bUIWindowLayoutWindowNode\b/g, "WindowLayoutWindowNode");
shell = shell.replace(/\bUIWindowLayoutStackNode\b/g, "WindowLayoutStackNode");
shell = shell.replace(/\bUIWindowLayoutAxisNode\b/g, "WindowLayoutAxisNode");
shell = shell.replace(/\bUIWindowLayout\b/g, "WindowLayout");
shell = shell.replace(/\bUIWindowLayoutNode\b/g, "WindowLayoutNode");
shell = shell.replace(/\bLayoutNode\b/g, "WindowLayout");
shell = shell.replace(/export type LayoutStack = WindowLayoutStackNode;/g, "export type LayoutStack = WindowLayoutStackNode;");
shell = shell.replace(/export type LayoutRow = WindowLayoutAxisNode & \{ kind: "row" \};/g, "export type LayoutRow = WindowLayoutAxisNode & { kind: \"row\" };");
shell = shell.replace(/export type LayoutColumn = WindowLayoutAxisNode & \{ kind: "column" \};/g, "export type LayoutColumn = WindowLayoutAxisNode & { kind: \"column\" };");

const fr = readFileSync(frPath, "utf8");

const importBlockEnd = fr.indexOf("//#region 📦shell-chrome-types.tsx");
if (importBlockEnd < 0) throw new Error("shell-chrome-types region not found");

const expandedUiImport = `import {
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
	StaticSidePanelTabDefinition,
	StaticTreePanelDefinition,
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
	type NavbarItem,
} from "@elements/ui";
`;

const frWithoutShellImports = fr
	.replace(
		/import \{[\s\S]*?\} from "@elements\/ui";\n\n\/\/#region 📦shell-chrome-types/,
		`${expandedUiImport}
//#region 📦shell-canvas.tsx
${shell}
//#endregion 📦shell-canvas.tsx

//#region 📦shell-chrome-types`,
	)
	.replace(
		/\tUICanvas,\n\tUIFind,\n\tUIFindProvider,\n\tUISearch,\n\tUIToolbar,\n\tcn,\n\tcountAppTools,\n\tlistPopulatedAppToolCategories,\n\tmergeAppTools,\n\tuseCommandHotkey,\n\tuseMediaQuery,\n\tuseUIFind,\n\ttype NavbarItem,\n\ttype UIFindItem,\n\ttype UIWindowLayout,\n/g,
		"",
	);

const deduped = frWithoutShellImports.replace(
	/\/\/#region 📦shell-chrome-types\.tsx[\s\S]*?\/\/#endregion 📦shell-chrome-types\.tsx\n\n/,
	"//#region 📦shell-chrome-types.tsx\n\n/** @emoji 👣 Footer row rendered by the workbench shell. */\nexport interface FooterItem {\n\treadonly id: string;\n\treadonly icon?: React.ReactNode;\n\treadonly text?: string;\n\treadonly content?: React.ReactNode;\n\treadonly order?: number;\n\treadonly onClick?: () => void;\n\treadonly className?: string;\n\treadonly disabled?: boolean;\n}\n\n/** @emoji 🌲 Minimal tree panel payload for declarative side tabs. */\nexport interface ShellChromeTreePanelConfig {\n\treadonly sections: readonly { readonly id: string; readonly content: React.ReactNode }[];\n}\n\n/** @emoji 📑 Side panel tab registration consumed by {@link WorkbenchView}. */\nexport interface SidePanelTabConfig {\n\treadonly id: string;\n\treadonly icon: React.ComponentType<{ readonly size?: number }>;\n\treadonly order?: number;\n\treadonly tree: ShellChromeTreePanelConfig;\n}\n\n//#endregion 📦shell-chrome-types.tsx\n\n",
);

writeFileSync(frPath, deduped, "utf8");

const newUi = [...uiLines.slice(0, startIdx), ...uiLines.slice(endIdx + 1)].join("\n");
writeFileSync(uiPath, newUi, "utf8");
console.log("relocated shell", endIdx - startIdx, "lines");
