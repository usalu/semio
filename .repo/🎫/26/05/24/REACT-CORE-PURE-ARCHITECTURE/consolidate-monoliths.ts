#!/usr/bin/env bun
import { readFileSync, writeFileSync, unlinkSync, rmSync, renameSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const root = join(import.meta.dir, "../../../../../..");

function read(path: string): string {
	return readFileSync(join(root, path), "utf8");
}

function write(path: string, content: string): void {
	writeFileSync(join(root, path), content, "utf8");
}

function stripBoardTsReexports(tsx: string): string {
	const lines = tsx.split(/\r?\n/);
	const out: string[] = [];
	let skip = false;
	for (let i = 0; i < lines.length; i++) {
		const line = lines[i]!;
		if (line.startsWith("import {") && lines[i + 1]?.includes("BOARD_DEFAULT_KIND")) {
			skip = true;
		}
		if (skip) {
			if (line === "import { ContextMenuController, type ContextMenuItem } from \"@elements/ui\";") {
				skip = false;
				out.push(line);
			}
			continue;
		}
		if (line.startsWith("export {") && line.includes("from \"./index\"")) continue;
		if (line.startsWith("export type {") && line.includes("from \"./index\"")) continue;
		out.push(line);
	}
	return out.join("\n");
}

function stripPlayHostBoardImports(host: string): string {
	return host
		.replace(/import \{[\s\S]*?\} from "\.\/index";\r?\n/, "")
		.replace(/import \{ BoardCanvas, Edge, Handle, Node, useBoardEvent \} from "\.\/index\.tsx";\r?\n/, "");
}

function stripSceneHostImports(host: string): string {
	return host.replace(/import \{[\s\S]*?\} from "\.\/index\.tsx";\r?\n/, "");
}

function stripTopologyHostImports(host: string): string {
	return host.replace(/import \{[\s\S]*?\} from "\.\/react\/index\.tsx";\r?\n/, "");
}

// Board: already merged when index.ts is gone
if (existsSync(join(root, "elements/lib/react/board/index.ts"))) {
	const ts = read("elements/lib/react/board/index.ts");
	const tsx = stripBoardTsReexports(read("elements/lib/react/board/index.tsx"));
	const host = stripPlayHostBoardImports(read("elements/lib/react/board/board-play-host.tsx"));
	const merged = `// #region 🧲Header\n/** @emoji 📋 \`@elements/board\` — WASM board renderer + React canvas + play harness (monolith). */\n// #endregion 🧲Header\n\n${ts}\n\n// #region 🎨ReactCanvas\n${tsx.replace(/^[\s\S]*?\/\/ #region 🧲Header[\s\S]*?#endregion 🧲Header\n\n?/, "")}\n\n// #region 🛝PlayHost\n${host.replace(/^[\s\S]*?\/\/ #endregion 🧲Header\n\n?/, "").replace(/^\/\/ #region 📥Imports[\s\S]*?\/\/ #endregion 📥Imports\n\n?/, "")}\n// #endregion 🛝PlayHost\n`;
	write("elements/lib/react/board/index.tsx", merged);
	unlinkSync(join(root, "elements/lib/react/board/index.ts"));
	unlinkSync(join(root, "elements/lib/react/board/board-play-host.tsx"));
	console.log("board merged");
} else {
	console.log("board skip (already merged)");
}

// Scene: append play host
{
	const base = read("elements/lib/react/scene/index.tsx");
	const host = stripSceneHostImports(read("elements/lib/react/scene/scene-play-host.tsx"));
	const merged = `${base}\n\n// #region 🛝PlayHost\n${host.replace(/^[\s\S]*?#endregion 🧲Header\n\n?/, "")}\n// #endregion 🛝PlayHost\n`;
	write("elements/lib/react/scene/index.tsx", merged);
	unlinkSync(join(root, "elements/lib/react/scene/scene-play-host.tsx"));
	console.log("scene merged");
}

// Topology: react/index.tsx + topology-play-host → topology/index.tsx
{
	const react = read("elements/lib/react/topology/react/index.tsx");
	const host = stripTopologyHostImports(read("elements/lib/react/topology/topology-play-host.tsx"));
	const fixedReact = react
		.replace(/from "\.\.\/\.\.\/board\/index\.ts"/g, 'from "../board/index.tsx"')
		.replace(/from "\.\.\/\.\.\/board\/index\.tsx"/g, 'from "../board/index.tsx"')
		.replace(/from "\.\.\/\.\.\/scene\/index\.tsx"/g, 'from "../scene/index.tsx"');
	const merged = `// #region 🧲Header\n/** @emoji 🔗 \`@elements/topology\` — paired board + scene surfaces + play harness (monolith). */\n// #endregion 🧲Header\n\n${fixedReact}\n\n// #region 🛝PlayHost\n${host.replace(/^[\s\S]*?#endregion 🧲Header\n\n?/, "")}\n// #endregion 🛝PlayHost\n`;
	write("elements/lib/react/topology/index.tsx", merged);
	unlinkSync(join(root, "elements/lib/react/topology/topology-play-host.tsx"));
	rmSync(join(root, "elements/lib/react/topology/react"), { recursive: true, force: true });
	console.log("topology merged");
}

// Spatial
{
	const reactPath = "elements/lib/react/spatial/react/index.tsx";
	const hostPath = "elements/lib/react/spatial/spatial-play-host.tsx";
	if (existsSync(join(root, reactPath)) && existsSync(join(root, hostPath))) {
		const react = read(reactPath);
		const host = read(hostPath).replace(/import \{[\s\S]*?\} from "\.\/react\/index\.tsx";\r?\n/, "");
		const merged = `// #region 🧲Header\n/** @emoji 📐 \`@elements/spatial\` — spatial play + react surfaces (monolith). */\n// #endregion 🧲Header\n\n${react}\n\n// #region 🛝PlayHost\n${host.replace(/^[\s\S]*?#endregion 🧲Header\n\n?/, "")}\n// #endregion 🛝PlayHost\n`;
		write("elements/lib/react/spatial/index.tsx", merged);
		unlinkSync(join(root, hostPath));
		rmSync(join(root, "elements/lib/react/spatial/react"), { recursive: true, force: true });
		console.log("spatial merged");
	}
}

// Framework renderer flatten: renderer/react → renderer/
{
	const frReact = join(root, "elements/lib/framework/renderer/react");
	const fr = join(root, "elements/lib/framework/renderer");
	for (const name of ["index.tsx", "package.json", "project.json", "script.ts", "vitest.config.ts"]) {
		const src = join(frReact, name);
		if (existsSync(src)) {
			renameSync(src, join(fr, name));
		}
	}
	rmSync(frReact, { recursive: true, force: true });
	// patch root package.json workspace
	const pkg = read("package.json");
	write(
		"package.json",
		pkg.replace('"elements/lib/framework/renderer/react"', '"elements/lib/framework/renderer"'),
	);
	console.log("framework renderer flattened");
}
