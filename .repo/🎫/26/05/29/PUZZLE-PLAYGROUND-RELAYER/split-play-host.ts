#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio";

function splitReact(dim: "2d" | "3d" | "5d", marker: string): void {
	const reactIndex = join(root, `puzzle/${dim}/react/index.tsx`);
	const playHost = join(root, `puzzle/${dim}/play/host.tsx`);
	const lines = readFileSync(reactIndex, "utf8").split(/\r?\n/);
	const start = lines.findIndex((l) => l.includes(marker));
	if (start < 0) {
		console.error(`[split] marker not found in ${dim}: ${marker}`);
		process.exit(1);
	}
	const head = lines.slice(0, start).join("\n");
	const hostBody = lines.slice(start).join("\n");
	const hostHeader =
		dim === "2d"
			? `/** @emoji 🛝 Board play React host — entry-only via play/main.ts; wires @puzzle/2d-react into @framework/playground/core-renderer-react. */\n`
			: dim === "3d"
				? `/** @emoji 🛝 Scene play React host — entry-only via play/main.ts. */\n`
				: `/** @emoji 🛝 Topology play React host — entry-only via play/main.ts. */\n`;
	writeFileSync(reactIndex, `${head}\n`);
	writeFileSync(
		playHost,
		`${hostHeader}${hostBody.replaceAll("../index.tsx", "../react/index.tsx").replaceAll("./play/", "./")}\n`,
	);
	console.log(`[split] ${dim}: react ${head.split("\n").length} lines, host ${hostBody.split("\n").length} lines`);
}

splitReact("2d", "// #region 🛝PlayHost");
splitReact("3d", "// #region 🛝PlayHost");
splitReact("5d", "// #region 🛝PlayHost");
