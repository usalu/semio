import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/semio/cad/js/renderer/play/main.tsx";
const lines = readFileSync(path, "utf8").split(/\r?\n/);

const headlessEnd = lines.findIndex((l) => l === "//#endregion 🔖Runtime");
const firstTestsStart = lines.findIndex((l) => l === "//#region 🧪Tests");
const reactStart = lines.findIndex((l) => l.startsWith('import "./globals.css"'));
const secondTestsStart = lines.findIndex((l, i) => i > reactStart && l === "//#region 🧪Tests");
const playgroundHostStart = lines.findIndex((l) => l === "//#region 🔖PlaygroundHost");

if ([headlessEnd, firstTestsStart, reactStart, secondTestsStart, playgroundHostStart].some((i) => i < 0)) {
	throw new Error("failed to locate section boundaries");
}

const headless = lines.slice(0, headlessEnd + 1);
const indexTests = lines.slice(firstTestsStart, reactStart);
const reactMiddle = lines.slice(reactStart, secondTestsStart);
const mainTests = lines.slice(secondTestsStart, playgroundHostStart);
const playgroundHost = lines.slice(playgroundHostStart);

const treeImport =
	'import type { TreeDataItem, TreeDataSection } from "@ui/react";\nimport { playgroundTreePanelRootItems } from "@framework/playground/renderer/react";\n';

const reactWithImports = [...reactMiddle];
const globalsIdx = reactWithImports.findIndex((l) => l.startsWith('import "./globals.css"'));
reactWithImports.splice(globalsIdx + 1, 0, treeImport);

const merged = [
	...headless,
	"",
	...reactWithImports,
	...playgroundHost,
	"",
	...indexTests.slice(0, -1),
	"",
	...mainTests,
].join("\n");

writeFileSync(path, merged);
console.log("fixed cad play main.tsx section order");
