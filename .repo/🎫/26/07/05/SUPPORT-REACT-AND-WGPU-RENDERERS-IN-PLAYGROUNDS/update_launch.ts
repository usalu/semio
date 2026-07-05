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

console.log(`Found ${configsRaw.length} configurations raw.`);

const playgrounds = [
	"🛠️dev📐cad",
	"🛠️dev📐cad🧩concrete🌲forest",
	"🛠️dev🌳dag",
	"🛠️dev🌊flow",
	"🛠️dev⚙️imperative",
	"🛠️dev📜sequence",
	"🛠️dev🔷lowpoly",
	"🛠️dev📄layout",
	"🛠️dev🌐gis📍2d",
	"🛠️dev📽️presentationplay",
	"🛠️dev🔧procedural🩻2d",
	"🛠️dev🔧procedural🏙️3d",
	"🛠️dev🔧procedural🏙️3d🧩hexagonal🧱column",
	"🛠️dev🧩puzzle🩻2d",
	"🛠️dev🧩puzzle🏙️3d",
	"🛠️dev🧩puzzle🏙️3d🎛️concrete🌲forest",
	"🛠️dev🧩puzzle👯5d",
	"🛠️dev🧩puzzle👯5d🎛️concrete🌲forest",
	"🛠️dev🧠reasoning🔗wires",
	"🛠️dev📸shooting",
	"🛠️dev📸shooting🎛️base",
	"🛠️dev🔺trinity🃏jack",
	"🛠️dev🔺trinity♻️rewrite",
	"🛠️dev📋forms",
	"🛠️dev🖼️raster",
	"🛠️dev🗄️vcs🎛️play",
	"🛠️dev✏️draw",
	"🛠️dev📝note",
	"🛠️dev✍️writer"
];

let newContent = content;

// Iterate backwards
for (let j = configsRaw.length - 1; j >= 0; j--) {
	const configStr = configsRaw[j]!;
	let configObj: any;
	try {
		// Use eval to safely parse JSON with comments and trailing commas
		configObj = eval("(" + configStr + ")");
	} catch (err) {
		console.warn(`Failed to parse config ${j} name ${configStr.substring(0, 100)}...:`, err);
		continue;
	}

	const name = configObj.name;
	if (playgrounds.includes(name)) {
		console.log(`Duplicating playground config: ${name}`);

		// Create React version:
		const reactObj = JSON.parse(JSON.stringify(configObj));
		if (!reactObj.env) reactObj.env = {};
		reactObj.env.SEMIO_RENDERER = "react";

		// Create WGPU version:
		const wgpuObj = JSON.parse(JSON.stringify(configObj));
		wgpuObj.name = name + "🧊wgpu";
		if (!wgpuObj.env) wgpuObj.env = {};
		wgpuObj.env.SEMIO_RENDERER = "wgpu";
		
		if (reactObj.presentation && typeof reactObj.presentation.order === "number") {
			wgpuObj.presentation.order = reactObj.presentation.order + 0.1;
		}

		// Detect indentation
		const lines = configStr.split("\n");
		const firstLineIndent = lines[0]?.match(/^\s*/)?.[0] || "    ";
		
		const formatConfig = (obj: any) => {
			const str = JSON.stringify(obj, null, 2);
			return str.split("\n").map((line, idx) => {
				if (idx === 0) return line;
				return firstLineIndent + line;
			}).join("\n");
		};

		const formattedReact = formatConfig(reactObj);
		const formattedWgpu = formatConfig(wgpuObj);

		const replacement = `${formattedReact},\n${firstLineIndent}${formattedWgpu}`;

		const idx = newContent.lastIndexOf(configStr);
		if (idx !== -1) {
			newContent = newContent.substring(0, idx) + replacement + newContent.substring(idx + configStr.length);
		}
	}
}

// Restore original launchPath
writeFileSync(launchPath, newContent, "utf8");
console.log("Successfully updated launch.json!");
