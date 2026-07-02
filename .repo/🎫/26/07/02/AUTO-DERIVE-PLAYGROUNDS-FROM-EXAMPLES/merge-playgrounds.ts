#!/usr/bin/env bun
/** @emoji 🛝 Merge core/playground.ts into index.ts via createPlaygroundApp and delete playground.ts. */

import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../");

const CORE_DIRS = [
	"draw/core",
	"note/core",
	"writer/core",
	"forms/core",
	"s/core",
	"layout/core",
	"shooting/core",
	"procedural/2d/core",
	"procedural/3d/core",
	"gis/2d/core",
	"raster/core",
	"mathematical/graph/port/directed/dag/core",
	"reasoning/mindmap/wires/core",
	"puzzle/2d/core",
	"puzzle/3d/core",
	"puzzle/5d/core",
	"framework/product/presentation/core",
	"trinity/rewrite/core",
	"trinity/jack/host-core",
	"cad/js/renderer/core",
	"flow/core",
	"imperative/core",
	"sequence/core",
	"lowpoly/core",
	"vcs/core",
] as const;

function mergeOne(coreDir: string): void {
	const playgroundPath = join(repoRoot, coreDir, "playground.ts");
	const indexPath = join(repoRoot, coreDir, "index.ts");
	if (!existsSync(playgroundPath) || !existsSync(indexPath)) return;

	const playground = readFileSync(playgroundPath, "utf8");
	let index = readFileSync(indexPath, "utf8");
	index = index.replace(/\nexport \{[^}]*\} from "\.\/playground\.ts";\n?/g, "\n");

	const appDefMatch = playground.match(/export const (\w+PlayAppDefinition): PlaygroundAppDefinition = \{([\s\S]*?)\n\};\s*(?:\/\/#endregion)?/);
	const classMatch = playground.match(/export class Playground\w+ extends Playground \{([\s\S]*?)\n\}/);
	if (!appDefMatch || !classMatch) {
		console.warn(`[DEBUG] skip ${coreDir}: parse failed`);
		return;
	}

	const appDefName = appDefMatch[1]!;
	let appDefBody = appDefMatch[2]!;
	const classBody = classMatch[1]!;

	const keybindings = classBody.match(/readonly keybindings = ([\s\S]*?);/)?.[1];
	const createRuntimeBody = classBody.match(/createRuntime\(\): Platform \{([\s\S]*?)\n\t\}/)?.[1]?.trim();
	const registerBodiesBody = classBody.match(/registerBodies\(\): void \{([\s\S]*?)\n\t\}/)?.[1]?.trim();
	const registerSurfaceHostsBody = classBody.match(/registerSurfaceHosts\(\): void \{([\s\S]*?)\n\t\}/)?.[1]?.trim();
	if (!createRuntimeBody || !registerBodiesBody) {
		console.warn(`[DEBUG] skip ${coreDir}: missing runtime/bodies`);
		return;
	}

	appDefBody = appDefBody
		.replace(/\n\tcreatePlayground: \(\) => new Playground\w+\(\),?/g, "")
		.replace(/\n\tbootRenderer: async \(pg\) => \{([\s\S]*?)\n\t\},?/g, (_, body) => `\n\t__BOOT_RENDERER__: async (pg) => {${body}\n\t},`);

	const bootRendererMatch = appDefBody.match(/\n\t__BOOT_RENDERER__: async \(pg\) => \{([\s\S]*?)\n\t\},?/);
	const bootRendererBlock = bootRendererMatch
		? `\n\tbootRenderer: async (pg) => {${bootRendererMatch[1]}\n\t},`
		: "";
	appDefBody = appDefBody.replace(/\n\t__BOOT_RENDERER__: async \(pg\) => \{[\s\S]*?\n\t\},?/g, "");

	const helperSection = playground
		.replace(/\/\/ #region 🧲Header[\s\S]*?\/\/ #endregion 🧲Header\n\n?/g, "")
		.replace(/import \{[\s\S]*?\} from "\.\/index\.ts";\n\n?/g, "")
		.replace(/import \{[\s\S]*?\} from "@semio-tech\/framework-playground-core";\n/g, "")
		.replace(/export class Playground\w+ extends Playground \{[\s\S]*?\n\}\n\n?/g, "")
		.replace(/\/\/#region 🔖PlaygroundAppDefinition[\s\S]*$/g, "")
		.replace(/\/\/ #region 🧪Tests[\s\S]*/g, "")
		.trim();

	const vitestSection = playground.match(/\/\/ #region 🧪Tests[\s\S]*/)?.[0] ?? "";

	const playRegion = `//#region 🔖Play
${helperSection ? `${helperSection}\n\n` : ""}import {
\tcreatePlaygroundApp,
} from "@semio-tech/framework-playground-core";

export const ${appDefName} = createPlaygroundApp({${appDefBody}
\tcreateRuntime: () => {
${createRuntimeBody.split("\n").map((line) => `\t\t${line.replace(/^\t/, "")}`).join("\n")}
\t},
\tregisterBodies: () => {
${registerBodiesBody.split("\n").map((line) => `\t\t${line.replace(/^\t/, "")}`).join("\n")}
\t},${registerSurfaceHostsBody ? `\n\tregisterSurfaceHosts: () => {\n${registerSurfaceHostsBody.split("\n").map((line) => `\t\t${line.replace(/^\t/, "")}`).join("\n")}\n\t},` : ""}${keybindings ? `\n\tkeybindings: ${keybindings},` : ""}${bootRendererBlock}
});
//#endregion 🔖Play
`;

	const hadPlayApp = index.includes("createPlaygroundApp");
	const testInsert = index.search(/\n\/\/ #region 🧪Tests/);
	if (testInsert >= 0) index = index.slice(0, testInsert) + "\n" + playRegion + index.slice(testInsert);
	else index = index.trimEnd() + "\n\n" + playRegion + (vitestSection ? `\n\n${vitestSection}` : "");

	if (!hadPlayApp) {
		writeFileSync(indexPath, index);
		rmSync(playgroundPath);
		console.log(`[DEBUG] merged ${coreDir}`);
	}
}


for (const coreDir of CORE_DIRS) mergeOne(coreDir);
console.log("[DEBUG] merge complete");
