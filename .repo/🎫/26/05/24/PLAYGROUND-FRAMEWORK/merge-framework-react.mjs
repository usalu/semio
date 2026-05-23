import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const dir = "c:/git/semio/elements/lib/framework/renderer/react";

const FILES = [
	"shell-chrome-types.tsx",
	"workbench-app-context.tsx",
	"workbench-history.tsx",
	"ui-declarative-renderer.tsx",
	"shell-bridge.tsx",
	"workbench-view.tsx",
	"workbench-mount.tsx",
];

function stripHeader(content) {
	return content.replace(/^\/\/ #region 🧲Header[\s\S]*?\/\/ #endregion 🧲Header\s*\n+/, "");
}

function stripInternalImports(content) {
	return content.replace(/^import\s+(?:type\s+)?[\s\S]*?\s+from\s+["']\.\/[^"']+["'];?\s*\n/gm, "");
}

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

const parts = [];
for (const file of FILES) {
	let content = readFileSync(join(dir, file), "utf8");
	content = stripHeader(content);
	content = stripInternalImports(content);
	content = content.replaceAll("@elements/ui/chrome", "@elements/ui");
	parts.push(`//#region 📦${file}\n${content}\n//#endregion 📦${file}\n`);
}

writeFileSync(join(dir, "index.tsx"), header + parts.join("\n"));

for (const file of [...FILES, "workbench-bridge.tsx", "level-context.tsx"]) {
	unlinkSync(join(dir, file));
}

console.log("merged framework-react into index.tsx");
