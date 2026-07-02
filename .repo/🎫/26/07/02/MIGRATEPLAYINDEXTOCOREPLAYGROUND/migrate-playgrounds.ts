#!/usr/bin/env bun
/** @emoji 🛝 One-shot migration: play/index.ts → core/playground.ts */

import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const REPO = path.resolve(import.meta.dir, "../../../../../..");

type DomainSpec = {
	readonly rel: string;
	readonly pkgName: string;
	readonly playEntryKind: string;
	readonly bootFn: string;
	readonly bootImport: string;
	readonly playgroundClass: string;
	readonly appIdConst: string;
	readonly controllerIdConst: string;
	readonly label: string;
	readonly exportName: string;
	readonly programId: string;
	readonly programName: string;
	readonly programAppId: string;
	readonly hasTsCore: boolean;
	readonly coreConflict?: "rust";
	readonly bootAsync?: boolean;
	readonly modeId?: string;
};

const DOMAINS: readonly DomainSpec[] = [
	{ rel: "puzzle/2d", pkgName: "@semio-tech/puzzle-2d-core", playEntryKind: "2d", bootFn: "boot2dPlay", bootImport: "@semio-tech/framework-playground-renderer-react/puzzle/2d", playgroundClass: "Playground2d", appIdConst: "PUZZLE_2D_PLAY_APP_ID", controllerIdConst: "PUZZLE_2D_PLAY_CONTROLLER_ID", label: "Puzzle 2D", exportName: "puzzle2dPlayAppDefinition", programId: "puzzle.2d", programName: "Puzzle 2D", programAppId: "puzzle2d", hasTsCore: false },
	{ rel: "puzzle/3d", pkgName: "@semio-tech/puzzle-3d-core", playEntryKind: "3d", bootFn: "bootPuzzle3dPlay", bootImport: "@semio-tech/framework-playground-renderer-react/puzzle/3d", playgroundClass: "Playground3d", appIdConst: "PUZZLE_3D_PLAY_APP_ID", controllerIdConst: "PUZZLE_3D_PLAY_CONTROLLER_ID", label: "Puzzle 3D", exportName: "puzzle3dPlayAppDefinition", programId: "puzzle.3d", programName: "Puzzle 3D", programAppId: "puzzle3d", hasTsCore: false },
	{ rel: "puzzle/5d", pkgName: "@semio-tech/puzzle-5d-core", playEntryKind: "5d", bootFn: "boot5dPlay", bootImport: "@semio-tech/framework-playground-renderer-react/puzzle/5d", playgroundClass: "Playground5d", appIdConst: "PUZZLE_5D_PLAY_APP_ID", controllerIdConst: "PUZZLE_5D_PLAY_CONTROLLER_ID", label: "Puzzle 5D", exportName: "puzzle5dPlayAppDefinition", programId: "puzzle.5d", programName: "Puzzle 5D", programAppId: "puzzle5d", hasTsCore: false },
	{ rel: "flow", pkgName: "@semio-tech/flow-core", playEntryKind: "flow", bootFn: "bootFlowPlay", bootImport: "@semio-tech/framework-playground-renderer-react/flow", playgroundClass: "PlaygroundFlow", appIdConst: "FLOW_PLAY_APP_ID", controllerIdConst: "FLOW_PLAY_CONTROLLER_ID", label: "Flow", exportName: "flowPlayAppDefinition", programId: "flow", programName: "Flow", programAppId: "flow", hasTsCore: true },
	{ rel: "mathematical/graph/port/directed/dag", pkgName: "@semio-tech/dag-host-core", playEntryKind: "dag", bootFn: "bootDagPlay", bootImport: "@semio-tech/framework-playground-renderer-react/dag", playgroundClass: "PlaygroundDag", appIdConst: "DAG_PLAY_APP_ID", controllerIdConst: "DAG_PLAY_CONTROLLER_ID", label: "DAG", exportName: "dagPlayAppDefinition", programId: "dag", programName: "DAG", programAppId: "dag", hasTsCore: false },
	{ rel: "imperative", pkgName: "@semio-tech/imperative-core", playEntryKind: "imperative", bootFn: "bootImperativePlay", bootImport: "@semio-tech/framework-playground-renderer-react/imperative", playgroundClass: "PlaygroundImperative", appIdConst: "IMPERATIVE_PLAY_APP_ID", controllerIdConst: "IMPERATIVE_PLAY_CONTROLLER_ID", label: "Imperative", exportName: "imperativePlayAppDefinition", programId: "imperative", programName: "Imperative", programAppId: "imperative", hasTsCore: true },
	{ rel: "sequence", pkgName: "@semio-tech/sequence-core", playEntryKind: "sequence", bootFn: "bootSequencePlay", bootImport: "@semio-tech/framework-playground-renderer-react/sequence", playgroundClass: "PlaygroundSequence", appIdConst: "SEQUENCE_PLAY_APP_ID", controllerIdConst: "SEQUENCE_PLAY_CONTROLLER_ID", label: "Sequence", exportName: "sequencePlayAppDefinition", programId: "sequence", programName: "Sequence", programAppId: "sequence", hasTsCore: true },
	{ rel: "layout", pkgName: "@semio-tech/layout-core", playEntryKind: "layout", bootFn: "bootLayoutPlay", bootImport: "@semio-tech/framework-playground-renderer-react/layout", playgroundClass: "PlaygroundLayout", appIdConst: "LAYOUT_PLAY_APP_ID", controllerIdConst: "LAYOUT_PLAY_CONTROLLER_ID", label: "Layout", exportName: "layoutPlayAppDefinition", programId: "layout", programName: "Layout", programAppId: "layout", hasTsCore: true },
	{ rel: "lowpoly", pkgName: "@semio-tech/lowpoly-core", playEntryKind: "lowpoly", bootFn: "bootLowpolyPlay", bootImport: "@semio-tech/framework-playground-renderer-react/lowpoly", playgroundClass: "PlaygroundLowpoly", appIdConst: "LOWPOLY_PLAY_APP_ID", controllerIdConst: "LOWPOLY_PLAY_CONTROLLER_ID", label: "Lowpoly", exportName: "lowpolyPlayAppDefinition", programId: "lowpoly", programName: "Lowpoly", programAppId: "lowpoly", hasTsCore: true, bootAsync: true },
	{ rel: "procedural/2d", pkgName: "@semio-tech/procedural-2d-core", playEntryKind: "procedural-2d", bootFn: "bootProcedural2dPlay", bootImport: "@semio-tech/framework-playground-renderer-react/procedural-2d", playgroundClass: "PlaygroundProcedural2d", appIdConst: "PROCEDURAL_2D_PLAY_APP_ID", controllerIdConst: "PROCEDURAL_2D_PLAY_CONTROLLER_ID", label: "Procedural 2D", exportName: "procedural2dPlayAppDefinition", programId: "procedural.2d", programName: "Procedural 2D", programAppId: "procedural2d", hasTsCore: false },
	{ rel: "procedural/3d", pkgName: "@semio-tech/procedural-3d-core", playEntryKind: "procedural-3d", bootFn: "bootProceduralPlay", bootImport: "@semio-tech/framework-playground-renderer-react/procedural-3d", playgroundClass: "PlaygroundProcedural", appIdConst: "PROCEDURAL_3D_PLAY_APP_ID", controllerIdConst: "PROCEDURAL_3D_PLAY_CONTROLLER_ID", label: "Procedural 3D", exportName: "procedural3dPlayAppDefinition", programId: "procedural.3d", programName: "Procedural 3D", programAppId: "procedural3d", hasTsCore: false },
	{ rel: "shooting", pkgName: "@semio-tech/shooting-core", playEntryKind: "shooting", bootFn: "bootShootingPlay", bootImport: "@semio-tech/framework-playground-renderer-react/shooting", playgroundClass: "PlaygroundShooting", appIdConst: "SHOOTING_PLAY_APP_ID", controllerIdConst: "SHOOTING_PLAY_CONTROLLER_ID", label: "Shooting", exportName: "shootingPlayAppDefinition", programId: "shooting", programName: "Shooting", programAppId: "shooting", hasTsCore: false },
	{ rel: "forms", pkgName: "@semio-tech/forms-core", playEntryKind: "forms", bootFn: "bootFormsPlay", bootImport: "@semio-tech/framework-playground-renderer-react/forms", playgroundClass: "PlaygroundForms", appIdConst: "FORMS_PLAY_APP_ID", controllerIdConst: "FORMS_PLAY_CONTROLLER_ID", label: "Forms", exportName: "formsPlayAppDefinition", programId: "forms", programName: "Forms", programAppId: "forms", hasTsCore: true },
	{ rel: "raster", pkgName: "@semio-tech/raster-core", playEntryKind: "raster", bootFn: "bootRasterPlay", bootImport: "@semio-tech/framework-playground-renderer-react/raster", playgroundClass: "PlaygroundRaster", appIdConst: "RASTER_PLAY_APP_ID", controllerIdConst: "RASTER_PLAY_CONTROLLER_ID", label: "Raster", exportName: "rasterPlayAppDefinition", programId: "raster", programName: "Raster", programAppId: "raster", hasTsCore: true },
	{ rel: "draw", pkgName: "@semio-tech/draw-core", playEntryKind: "draw", bootFn: "bootDrawPlay", bootImport: "@semio-tech/framework-playground-renderer-react/draw", playgroundClass: "PlaygroundDraw", appIdConst: "DRAW_PLAY_APP_ID", controllerIdConst: "DRAW_PLAY_CONTROLLER_ID", label: "Draw", exportName: "drawPlayAppDefinition", programId: "draw", programName: "Draw", programAppId: "draw", hasTsCore: true },
	{ rel: "writer", pkgName: "@semio-tech/writer-core", playEntryKind: "writer", bootFn: "bootWriterPlay", bootImport: "@semio-tech/framework-playground-renderer-react/writer", playgroundClass: "PlaygroundWriter", appIdConst: "WRITER_PLAY_APP_ID", controllerIdConst: "WRITER_PLAY_CONTROLLER_ID", label: "Writer", exportName: "writerPlayAppDefinition", programId: "writer", programName: "Writer", programAppId: "writer", hasTsCore: true },
	{ rel: "s", pkgName: "@semio-tech/s-core", playEntryKind: "s", bootFn: "bootSPlay", bootImport: "@semio-tech/framework-playground-renderer-react/s", playgroundClass: "PlaygroundS", appIdConst: "S_PLAY_APP_ID", controllerIdConst: "S_PLAY_CONTROLLER_ID", label: "S", exportName: "sPlayAppDefinition", programId: "s", programName: "S", programAppId: "s", hasTsCore: true },
	{ rel: "vcs", pkgName: "@semio-tech/vcs-core", playEntryKind: "vcs", bootFn: "bootVcsPlay", bootImport: "@semio-tech/framework-playground-renderer-react/vcs", playgroundClass: "PlaygroundVcs", appIdConst: "VCS_PLAY_APP_ID", controllerIdConst: "VCS_PLAY_CONTROLLER_ID", label: "VCS", exportName: "vcsPlayAppDefinition", programId: "vcs", programName: "VCS", programAppId: "vcs", hasTsCore: true },
	{ rel: "gis/2d", pkgName: "@semio-tech/gis-2d-core", playEntryKind: "map", bootFn: "bootMapPlay", bootImport: "@semio-tech/framework-playground-renderer-react/gis/2d", playgroundClass: "PlaygroundMap", appIdConst: "GIS_MAP_PLAY_APP_ID", controllerIdConst: "GIS_MAP_PLAY_CONTROLLER_ID", label: "Map", exportName: "gis2dPlayAppDefinition", programId: "gis.map", programName: "GIS Map", programAppId: "map", hasTsCore: false },
	{ rel: "reasoning/mindmap/wires", pkgName: "@semio-tech/reasoning-mindmap-wires-core", playEntryKind: "wires", bootFn: "bootWiresPlay", bootImport: "@semio-tech/framework-playground-renderer-react/reasoning/wires", playgroundClass: "Playground2d", appIdConst: "WIRES_PLAY_APP_ID", controllerIdConst: "WIRES_PLAY_CONTROLLER_ID", label: "Wires", exportName: "wiresPlayAppDefinition", programId: "reasoning.wires", programName: "Reasoning Wires", programAppId: "wires", hasTsCore: false },
	{ rel: "trinity/jack", pkgName: "@semio-tech/trinity-jack-host-core", playEntryKind: "trinity-jack", bootFn: "bootTrinityJackPlay", bootImport: "@semio-tech/framework-playground-renderer-react/trinity/jack", playgroundClass: "PlaygroundTrinityJack", appIdConst: "TRINITY_JACK_PLAY_APP_ID", controllerIdConst: "TRINITY_JACK_PLAY_CONTROLLER_ID", label: "Trinity Jack", exportName: "trinityJackPlayAppDefinition", programId: "trinity", programName: "Trinity", programAppId: "trinity-jack", hasTsCore: false, coreConflict: "rust" },
	{ rel: "trinity/rewrite", pkgName: "@semio-tech/trinity-rewrite-core", playEntryKind: "trinity-rewrite", bootFn: "bootTrinityRewritePlay", bootImport: "@semio-tech/framework-playground-renderer-react/trinity/rewrite", playgroundClass: "PlaygroundTrinityRewrite", appIdConst: "TRINITY_REWRITE_PLAY_APP_ID", controllerIdConst: "TRINITY_REWRITE_PLAY_CONTROLLER_ID", label: "Trinity Rewrite", exportName: "trinityRewritePlayAppDefinition", programId: "trinity.rewrite", programName: "Trinity Rewrite", programAppId: "trinity-rewrite", hasTsCore: false },
	{ rel: "framework/product/presentation", pkgName: "@semio-tech/framework-presentation-core", playEntryKind: "presentation", bootFn: "bootPresentationPlay", bootImport: "@semio-tech/framework-playground-renderer-react/presentation", playgroundClass: "PresentationPlay", appIdConst: "PRESENTATION_PLAY_APP_ID", controllerIdConst: "PRESENTATION_PLAY_CONTROLLER_ID", label: "Presentation", exportName: "presentationPlayAppDefinition", programId: "presentation", programName: "Presentation", programAppId: "presentation", hasTsCore: true },
];

function coreDir(spec: DomainSpec): string {
	if (spec.coreConflict === "rust") return path.join(REPO, spec.rel, "host-core");
	return path.join(REPO, spec.rel, "core");
}

function stripBootAndSExtension(source: string): string {
	let out = source;
	out = out.replace(/\n\/\/#region 🔖Boot[\s\S]*?\/\/#endregion 🔖Boot\n?/g, "\n");
	out = out.replace(/\n\/\/ #region 🔖Boot[\s\S]*?\/\/ #endregion 🔖Boot\n?/g, "\n");
	out = out.replace(/\nif \(\s*typeof document !== "undefined"[\s\S]*?PUZZLE_PLAY_ENTRY[\s\S]*?\}\)\(\);\s*\}\n?/g, "\n");
	out = out.replace(/\nbootstrapElementsSurfaceChromeDocument\(\);\s*\n/g, "\n");
	out = out.replace(/\/\/#region 🔖SExtension[\s\S]*?\/\/#endregion 🔖SExtension\n?/g, "");
	out = out.replace(/\/\/ #region 🔖SExtension[\s\S]*?\/\/ #endregion 🔖SExtension\n?/g, "");
	return out.replace(/\n{4,}/g, "\n\n");
}

function fixPlayRelativeImports(source: string): string {
	return source.replace(/from "\.\/([^"]+)"/g, 'from "../play/$1"');
}

function readViteDevHost(playDir: string): string {
	const vitePath = path.join(playDir, "vite.config.ts");
	if (!existsSync(vitePath)) return `playEntryKind: "${DOMAINS.find((d) => path.join(REPO, d.rel, "play") === playDir)?.playEntryKind ?? "unknown"}"`;
	const vite = readFileSync(vitePath, "utf8");
	const playEntryKind = vite.match(/playEntryKind:\s*"([^"]+)"/)?.[1];
	const lines: string[] = [`playEntryKind: "${playEntryKind ?? "unknown"}"`];
	const dedupe = vite.match(/resolveDedupe:\s*\[([^\]]+)\]/);
	if (dedupe) lines.push(`resolveDedupe: [${dedupe[1]}]`);
	const watch = vite.match(/watchIgnored:\s*\[([\s\S]*?)\],/);
	if (watch) lines.push(`watchIgnored: [${watch[1].trim()}]`);
	const optInclude = vite.match(/optimizeDeps:\s*\{[\s\S]*?include:\s*\[([^\]]+)\]/);
	if (optInclude) lines.push(`optimizeDeps: { include: [${optInclude[1]}] }`);
	const optExclude = vite.match(/optimizeDeps:\s*\{[\s\S]*?exclude:\s*\[([^\]]+)\]/);
	if (optExclude && !optInclude) lines.push(`optimizeDeps: { exclude: [${optExclude[1]}] }`);
	return lines.join(",\n\t\t");
}

function appendAppDefinition(spec: DomainSpec, source: string, devHostBody: string): string {
	const bootLine = spec.bootAsync
		? `\tbootRenderer: async (pg) => {\n\t\tconst { ${spec.bootFn} } = await import("${spec.bootImport}");\n\t\tawait ${spec.bootFn}(pg);\n\t},`
		: `\tbootRenderer: async (pg) => {\n\t\tconst { ${spec.bootFn} } = await import("${spec.bootImport}");\n\t\t${spec.bootFn}(pg);\n\t},`;
	const block = `
//#region 🔖PlaygroundAppDefinition
import type { PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";

/** @emoji 🛝 ${spec.label} playground app definition. */
export const ${spec.exportName}: PlaygroundAppDefinition = {
\tid: ${spec.appIdConst},
\tlabel: "${spec.label}",
\tcontrollerId: ${spec.controllerIdConst},
\tmodes: [{ id: "edit", label: "Edit" }],
\tdefaultModeId: "edit",
\tcreatePlayground: () => new ${spec.playgroundClass}(),
${bootLine}
\tdevHost: {
\t\t${devHostBody},
\t},
};
//#endregion 🔖PlaygroundAppDefinition
`;
	return `${source.trimEnd()}\n${block}`;
}

function schemaDepth(rel: string): string {
	const depth = rel.split("/").length + 1;
	return "../".repeat(depth) + "node_modules/nx/schemas/project-schema.json";
}

function repoDepth(rel: string): string {
	const depth = rel.split("/").length + 1;
	return "../".repeat(depth);
}

function writeNewCorePackage(spec: DomainSpec): void {
	const dir = coreDir(spec);
	mkdirSync(dir, { recursive: true });
	const playPkgPath = path.join(REPO, spec.rel, "play", "package.json");
	const playPkg = existsSync(playPkgPath) ? JSON.parse(readFileSync(playPkgPath, "utf8")) as { dependencies?: Record<string, string> } : {};
	const deps = { ...playPkg.dependencies, "@semio-tech/framework-playground-core": "workspace:*", "@semio-tech/framework-platform-core": "workspace:*" };
	writeFileSync(
		path.join(dir, "package.json"),
		`${JSON.stringify(
			{
				$schema: schemaDepth(spec.rel),
				name: spec.pkgName,
				version: "0.1.0",
				description: `${spec.rel} · playground harness core`,
				type: "module",
				private: true,
				exports: { ".": "./index.ts", "./playground": "./playground.ts" },
				scripts: { test: `bun nx run ${spec.pkgName}:test` },
				dependencies: deps,
				devDependencies: { typescript: "^5.9.3", vitest: "^4.0.17", "@semio-tech/framework-playground-renderer-react": "workspace:*", "@semio-tech/ui-react": "workspace:*" },
				license: "LGPL-3.0-or-later",
				repository: { type: "git", url: "https://github.com/usalu/semio.git", directory: `${spec.rel}/${spec.coreConflict === "rust" ? "host-core" : "core"}` },
				bundleKind: "library",
			},
			null,
			"\t",
		)}\n`,
	);
	writeFileSync(
		path.join(dir, "project.json"),
		`${JSON.stringify(
			{
				name: spec.pkgName,
				$schema: schemaDepth(spec.rel),
				targets: {
					test: {
						executor: "nx:run-commands",
						options: { cwd: `${spec.rel}/${spec.coreConflict === "rust" ? "host-core" : "core"}`, command: "bun ./script.ts test", forwardAllArgs: true },
					},
				},
			},
			null,
			"\t",
		)}\n`,
	);
	writeFileSync(
		path.join(dir, "script.ts"),
		`#!/usr/bin/env bun
/** 🛝 \`${spec.pkgName}\` router: \`bun ./script.ts test\`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "${repoDepth(spec.rel)}repo/lib/js/index.ts";

class TestScript extends BundleScript {
\trun(segments: string[]): void {
\t\trunVitest(this.root, segments, "vitest.config.ts");
\t}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
`,
	);
}

function patchCoreIndex(spec: DomainSpec): void {
	const dir = coreDir(spec);
	const indexPath = path.join(dir, "index.ts");
	const buildFn = `build${spec.exportName.replace(/PlayAppDefinition$/, "").replace(/^./, (c) => c.toUpperCase())}ProgramDefinition`;
	const buildFnName = spec.hasTsCore
		? (() => {
				const existing = existsSync(indexPath) ? readFileSync(indexPath, "utf8") : "";
				const m = existing.match(/export function (build\w+ProgramDefinition)/);
				return m?.[1] ?? `build${spec.programName.replace(/\s/g, "")}ProgramDefinition`;
			})()
		: `build${spec.programName.replace(/\s/g, "")}ProgramDefinition`;

	const exportLine = `export { ${spec.exportName} } from "./playground.ts";\nexport * from "./playground.ts";\n`;

	if (!spec.hasTsCore) {
		writeFileSync(
			indexPath,
			`// #region 🧲Header
/** @emoji 🛝 \`${spec.pkgName}\` — playground harness exports. */
// #endregion 🧲Header

${exportLine}
//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { ${spec.exportName} } from "./playground.ts";

/** @emoji 🧩 S program definition for ${spec.label.toLowerCase()}. */
export function ${buildFnName}(): PlatformDefinition {
\tconst app = ${spec.exportName};
\treturn {
\t\tid: "${spec.programId}",
\t\tname: "${spec.programName}",
\t\tapiVersion: "1",
\t\tapps: [{ id: "${spec.programAppId}", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
\t\tcreatePlatformApi: () => ({}),
\t};
}
//#endregion 🔖SExtension
`,
		);
		return;
	}

	let existing = existsSync(indexPath) ? readFileSync(indexPath, "utf8") : "";
	if (!existing.includes(spec.exportName)) {
		if (existing.includes("//#endregion 🔖SExtension")) {
			existing = existing.replace(
				/(export function \w+ProgramDefinition\(\): PlatformDefinition \{[\s\S]*?\n\})/,
				`export { ${spec.exportName} } from "./playground.ts";\n\n$1`,
			);
		}
		const sRegion = /\/\/#region 🔖SExtension[\s\S]*?export function (build\w+ProgramDefinition)\(\): PlatformDefinition \{[\s\S]*?\n\}/;
		if (sRegion.test(existing)) {
			existing = existing.replace(sRegion, (match, fn) => {
				return `//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { ${spec.exportName} } from "./playground.ts";

/** @emoji 🧩 S program definition for ${spec.label.toLowerCase()}. */
export function ${fn}(): PlatformDefinition {
\tconst app = ${spec.exportName};
\treturn {
\t\tid: "${spec.programId}",
\t\tname: "${spec.programName}",
\t\tapiVersion: "1",
\t\tapps: [{ id: "${spec.programAppId}", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
\t\tcreatePlatformApi: () => ({}),
\t};
}`;
			});
		} else {
			existing = `${existing.trimEnd()}\n\nexport { ${spec.exportName} } from "./playground.ts";\nexport * from "./playground.ts";\n`;
		}
		writeFileSync(indexPath, existing.endsWith("\n") ? existing : `${existing}\n`);
	}
}

for (const spec of DOMAINS) {
	const playIndex = path.join(REPO, spec.rel, "play", "index.ts");
	if (!existsSync(playIndex)) {
		console.error(`SKIP missing ${playIndex}`);
		continue;
	}
	const core = coreDir(spec);
	mkdirSync(core, { recursive: true });
	if (!spec.hasTsCore) writeNewCorePackage(spec);
	const devHost = readViteDevHost(path.join(REPO, spec.rel, "play"));
	let playground = readFileSync(playIndex, "utf8");
	playground = stripBootAndSExtension(playground);
	playground = fixPlayRelativeImports(playground);
	playground = appendAppDefinition(spec, playground, devHost);
	writeFileSync(path.join(core, "playground.ts"), playground);
	patchCoreIndex(spec);
	console.log(`OK ${spec.rel} → ${spec.coreConflict === "rust" ? "host-core" : "core"}/playground.ts`);
}

console.log("Done.");
