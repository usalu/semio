import { readFileSync, writeFileSync } from "node:fs";

const launchPath = "/Users/ueli/Documents/semio/.vscode/launch.json";
const content = readFileSync(launchPath, "utf8");

const startIdx = content.indexOf('"configurations": [');
if (startIdx === -1) {
	console.error("Could not find configurations array");
	process.exit(1);
}

const arrayStart = content.indexOf("[", startIdx);
let depth = 1;
let i = arrayStart + 1;
let insideString = false;
let escape = false;

const configsRaw: string[] = [];
let currentConfigStart = -1;

while (i < content.length && depth > 0) {
	const char = content[i];
	
	// Handle string escape
	if (escape) {
		escape = false;
		i++;
		continue;
	}
	if (char === "\\") {
		escape = true;
		i++;
		continue;
	}

	// Handle comments when not inside a string
	if (!insideString) {
		if (content[i] === "/" && content[i + 1] === "/") {
			// Skip single line comment
			while (i < content.length && content[i] !== "\n") {
				i++;
			}
			continue;
		}
		if (content[i] === "/" && content[i + 1] === "*") {
			// Skip block comment
			i += 2;
			while (i < content.length && !(content[i] === "*" && content[i + 1] === "/")) {
				i++;
			}
			i += 2;
			continue;
		}
	}

	if (char === '"') {
		insideString = !insideString;
		i++;
		continue;
	}

	if (!insideString) {
		if (char === "{") {
			if (depth === 1) {
				currentConfigStart = i;
			}
			depth++;
		} else if (char === "}") {
			depth--;
			if (depth === 1 && currentConfigStart !== -1) {
				configsRaw.push(content.substring(currentConfigStart, i + 1));
				currentConfigStart = -1;
			}
		} else if (char === "[") {
			depth++;
		} else if (char === "]") {
			depth--;
		}
	}
	i++;
}

// Map each configuration to its parsed name
const configs = configsRaw.map((raw) => {
	const obj = eval("(" + raw + ")");
	return { raw, obj, name: obj.name };
});

// Remove any configuration whose name ends with "🧊wgpu-alt" or similar
const filteredConfigs = configs.filter((config) => {
	if (config.name.endsWith("🧊wgpu-alt")) {
		console.log(`Removing alt config: ${config.name}`);
		return false;
	}
	return true;
});

// Reconstruct launch.json
const firstConfigRaw = configsRaw[0];
const lastConfigRaw = configsRaw[configsRaw.length - 1];

const blockStart = content.indexOf(firstConfigRaw);
const blockEnd = content.lastIndexOf(lastConfigRaw) + lastConfigRaw.length;

const firstConfigEnd = content.indexOf(firstConfigRaw) + firstConfigRaw.length;
const secondConfigStart = content.indexOf(configsRaw[1]);
const separator = content.substring(firstConfigEnd, secondConfigStart);

const replacementBlock = filteredConfigs.map((c) => c.raw).join(separator);

const newContent = content.substring(0, blockStart) + replacementBlock + content.substring(blockEnd);

writeFileSync(launchPath, newContent, "utf8");
console.log("Successfully removed all wgpu-alt configurations!");
