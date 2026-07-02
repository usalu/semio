#!/usr/bin/env bun
/** Split core/index.ts → internal.ts + thin barrel to break playground circular imports. */

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const REPO = path.resolve(import.meta.dir, "../../../../../..");

const CORES = [
	{ rel: "draw", pkg: "@semio-tech/draw-core" },
	{ rel: "writer", pkg: "@semio-tech/writer-core" },
	{ rel: "raster", pkg: "@semio-tech/raster-core" },
	{ rel: "forms", pkg: "@semio-tech/forms-core" },
	{ rel: "layout", pkg: "@semio-tech/layout-core" },
	{ rel: "lowpoly", pkg: "@semio-tech/lowpoly-core" },
	{ rel: "sequence", pkg: "@semio-tech/sequence-core" },
	{ rel: "imperative", pkg: "@semio-tech/imperative-core" },
	{ rel: "s", pkg: "@semio-tech/s-core" },
	{ rel: "flow", pkg: "@semio-tech/flow-core" },
	{ rel: "vcs", pkg: "@semio-tech/vcs-core" },
	{ rel: "framework/product/presentation", pkg: "@semio-tech/framework-presentation-core" },
];

for (const { rel, pkg } of CORES) {
	const coreDir = path.join(REPO, rel, "core");
	const indexPath = path.join(coreDir, "index.ts");
	const internalPath = path.join(coreDir, "internal.ts");
	const playgroundPath = path.join(coreDir, "playground.ts");
	if (!existsSync(indexPath) || !existsSync(playgroundPath)) continue;
	if (existsSync(internalPath)) {
		const playground = readFileSync(playgroundPath, "utf8");
		if (playground.includes(pkg)) writeFileSync(playgroundPath, playground.replaceAll(pkg, "./internal.ts"));
		continue;
	}

	let index = readFileSync(indexPath, "utf8");
	const playground = readFileSync(playgroundPath, "utf8");
	if (!playground.includes(pkg)) continue;

	const sExtMatch = index.match(/(\/\/#region 🔖SExtension[\s\S]*)$/);
	const body = sExtMatch ? index.slice(0, sExtMatch.index).trimEnd() : index.trimEnd();
	const sExt = sExtMatch ? sExtMatch[1] : "";

	writeFileSync(internalPath, `${body}\n`);
	writeFileSync(
		indexPath,
		`export * from "./internal.ts";\nexport * from "./playground.ts";\n\n${sExt.replaceAll('from "./playground.ts"', 'from "./playground.ts"').replace(/import \{ (\w+PlayAppDefinition) \} from "\.\/playground\.ts";/, 'import { $1 } from "./playground.ts";')}`,
	);
	writeFileSync(playgroundPath, playground.replaceAll(pkg, "./internal.ts"));
	console.log(`split ${rel}/core`);
}
