#!/usr/bin/env bun
/** @emoji 🧭 `@ui/styling` task router — `bun ./script.ts generate`. */
import { fetchElementsFonts, generateStylingArtifacts } from "../script.ts";

const cmd = process.argv[2];
if (cmd === "generate") {
	generateStylingArtifacts();
	process.exit(0);
}
if (cmd === "fonts") {
	await fetchElementsFonts();
	process.exit(0);
}
console.error("usage: bun ./script.ts <generate|fonts>");
process.exit(1);
