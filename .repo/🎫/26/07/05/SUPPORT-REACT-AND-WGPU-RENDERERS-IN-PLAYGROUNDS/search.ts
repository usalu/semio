import { readFileSync } from "node:fs";
import { join } from "node:path";

const libFile = "/Users/ueli/Documents/semio/repo/lib/js/index.ts";
const content = readFileSync(libFile, "utf8");

function findLine(query: string) {
	const lines = content.split("\n");
	lines.forEach((line, index) => {
		if (line.includes(query)) {
			console.log(`${index + 1}: ${line}`);
		}
	});
}

console.log("=== resolveFrameworkOsPlaygroundPlugin ===");
findLine("resolveFrameworkOsPlaygroundPlugin");

console.log("=== frameworkOsPlaygroundDevEnv ===");
findLine("frameworkOsPlaygroundDevEnv");

console.log("=== SEMIO_RENDERER ===");
findLine("SEMIO_RENDERER");
