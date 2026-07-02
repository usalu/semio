#!/usr/bin/env bun
import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../");

const files = [
	"draw/core/index.ts",
	"note/core/index.ts",
	"writer/core/index.ts",
	"forms/core/index.ts",
	"s/core/index.ts",
	"layout/core/index.ts",
	"shooting/core/index.ts",
	"procedural/2d/core/index.ts",
	"procedural/3d/core/index.ts",
	"gis/2d/core/index.ts",
	"raster/core/index.ts",
	"mathematical/graph/port/directed/dag/core/index.ts",
	"puzzle/2d/core/index.ts",
	"puzzle/5d/core/index.ts",
	"trinity/rewrite/core/index.ts",
	"trinity/jack/host-core/index.ts",
	"cad/js/renderer/core/index.ts",
	"flow/core/index.ts",
	"imperative/core/index.ts",
	"sequence/core/index.ts",
	"lowpoly/core/index.ts",
	"vcs/core/index.ts",
	"framework/product/presentation/core/index.ts",
	"reasoning/mindmap/wires/core/index.ts",
	"puzzle/3d/core/index.ts",
];

for (const rel of files) {
	const path = join(repoRoot, rel);
	if (!existsSync(path)) continue;
	let content = readFileSync(path, "utf8");
	const appId = content.match(/const ([A-Z0-9_]+_PLAY_APP_ID) =/)?.[1];
	if (appId) content = content.replace(/createProductPlaygroundPlatform\(this\.id(?:, "([^"]*)")?\)/g, (_, name) =>
		name ? `createProductPlaygroundPlatform(${appId}, ${JSON.stringify(name)})` : `createProductPlaygroundPlatform(${appId})`,
	);
	content = content.replace(/fixtureHost\./g, "exampleHost.");
	content = content.replace(/resolve(\w+)PlayFixtureSlug/g, "resolve$1PlayExampleSlug");
	content = content.replace(/RASTER_PLAY_FIXTURE_DEFAULT_ID/g, "RASTER_PLAY_EXAMPLE_DEFAULT_ID");
	content = content.replace(/fixture-slugs\.js/g, "example-slugs.ts");
	content = content.replace(/\nexport \{[^}]*\} from "\.\/playground\.ts";\n?/g, "\n");
	content = content.replace(/\nexport \* from "\.\/playground\.ts";\n?/g, "\n");
	content = content.replace(/\} from "\.\/playground\.ts";\n?/g, "} from \"./index.ts\";\n");
	writeFileSync(path, content);
}

for (const rel of ["draw/core/example-slugs.ts", "note/core/example-slugs.ts", "writer/core/example-slugs.ts", "forms/core/example-slugs.ts", "s/core/example-slugs.ts", "shooting/core/example-slugs.ts", "procedural/2d/core/example-slugs.ts", "procedural/3d/core/example-slugs.ts", "raster/core/example-slugs.ts", "cad/js/renderer/core/example-slugs.ts", "trinity/jack/host-core/example-slugs.ts"]) {
	const path = join(repoRoot, rel);
	if (!existsSync(path)) continue;
	let content = readFileSync(path, "utf8");
	content = content.replace(/resolve(\w+)PlayFixtureSlug/g, "resolve$1PlayExampleSlug");
	writeFileSync(path, content);
}

console.log("[DEBUG] bulk fix complete");
