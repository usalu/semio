import { readFileSync } from "node:fs";

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

const parsedConfigs = configsRaw.map((raw, idx) => {
	try {
		return { name: eval("(" + raw + ")").name, index: idx };
	} catch (e) {
		return { name: `ERROR_${idx}`, index: idx };
	}
});

console.log("=== CONFIG ORDER ===");
parsedConfigs.forEach((c) => {
	console.log(`${c.index}: ${c.name}`);
});

// For every react config, verify it is immediately followed by wgpu wasm and wgpu native
for (let j = 0; j < parsedConfigs.length; j++) {
	const name = parsedConfigs[j].name;
	if (name.endsWith("⚛️react")) {
		const base = name.replace("⚛️react", "");
		const expectedWasm = base + "🧊wgpu🌐wasm";
		const expectedNative = base + "🧊wgpu🖥️native";

		const next1 = j + 1 < parsedConfigs.length ? parsedConfigs[j + 1].name : "";
		const next2 = j + 2 < parsedConfigs.length ? parsedConfigs[j + 2].name : "";

		if (next1 !== expectedWasm) {
			console.log(`ORDER BUG at index ${j + 1}: expected ${expectedWasm}, found ${next1}`);
		}
		if (next2 !== expectedNative) {
			console.log(`ORDER BUG at index ${j + 2}: expected ${expectedNative}, found ${next2}`);
		}
	}
}
