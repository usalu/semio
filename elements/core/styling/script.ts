#!/usr/bin/env bun
/** @emoji 🧭 `elements/core/styling` task router — framework-neutral tokens + adapter codegen (`bun ./script.ts generate`). */
import { generateStylingArtifacts } from "./js/tailwind/generate.ts";

const cmd = process.argv[2];
if (cmd === "generate") {
	generateStylingArtifacts();
	process.exit(0);
}
console.error("usage: bun ./script.ts generate");
process.exit(1);
