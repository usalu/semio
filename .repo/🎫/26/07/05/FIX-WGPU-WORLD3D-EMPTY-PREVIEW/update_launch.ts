import { writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

// Get original content of launch.json from HEAD
const gitShow = spawnSync("git", ["show", "HEAD:.vscode/launch.json"], { encoding: "utf8" });
if (gitShow.status !== 0) {
	console.error("Failed to read original launch.json from HEAD");
	process.exit(1);
}
const content = gitShow.stdout;

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
		configObj = eval("(" + configStr + ")");
	} catch (err) {
		console.warn(`Failed to parse config ${j} name ${configStr.substring(0, 100)}...:`, err);
		continue;
	}

	const name = configObj.name;
	
	// Check if this config is one of our playgrounds or a WGPU variant of them
	const isBase = playgrounds.includes(name);
	// "🧊wgpu" length is 6 in UTF-16
	const isWgpu = name.endsWith("🧊wgpu") && playgrounds.includes(name.slice(0, -6));

	if (isBase) {
		console.log(`Setting React env for base playground: ${name}`);
		const reactObj = JSON.parse(JSON.stringify(configObj));
		if (!reactObj.env) reactObj.env = {};
		reactObj.env.SEMIO_RENDERER = "react";

		// Detect indentation and format
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
		const idx = newContent.lastIndexOf(configStr);
		if (idx !== -1) {
			newContent = newContent.substring(0, idx) + formattedReact + newContent.substring(idx + configStr.length);
		}
	} else if (isWgpu) {
		console.log(`Setting WGPU env and port offset for: ${name}`);
		const wgpuObj = JSON.parse(JSON.stringify(configObj));
		if (!wgpuObj.env) wgpuObj.env = {};
		wgpuObj.env.SEMIO_RENDERER = "wgpu";

		// Get base name
		const baseName = name.slice(0, -6);
		// Find base config in raw configs to find the original port
		let origPortStr = "";
		for (const raw of configsRaw) {
			try {
				const obj = eval("(" + raw + ")");
				if (obj.name === baseName) {
					if (obj.env) {
						for (const key of Object.keys(obj.env)) {
							if (key.endsWith("_PORT") || key.endsWith("_PLAY_PORT")) {
								origPortStr = obj.env[key];
								break;
							}
						}
					}
				}
			} catch {}
		}

		// Offset port numbers by 100
		let newPortStr = "";
		if (wgpuObj.env) {
			for (const key of Object.keys(wgpuObj.env)) {
				if (key.endsWith("_PORT") || key.endsWith("_PLAY_PORT")) {
					const val = wgpuObj.env[key];
					if (typeof val === "string" && /^\d+$/.test(val)) {
						const portNum = Number(val);
						newPortStr = String(portNum + 100);
						wgpuObj.env[key] = newPortStr;
					}
				}
			}
		}

		// Update serverReadyAction pattern
		if (wgpuObj.serverReadyAction && wgpuObj.serverReadyAction.pattern && origPortStr && newPortStr) {
			wgpuObj.serverReadyAction.pattern = wgpuObj.serverReadyAction.pattern.replaceAll(origPortStr, newPortStr);
		}

		// Detect indentation and format
		const lines = configStr.split("\n");
		const firstLineIndent = lines[0]?.match(/^\s*/)?.[0] || "    ";
		const formatConfig = (obj: any) => {
			const str = JSON.stringify(obj, null, 2);
			return str.split("\n").map((line, idx) => {
				if (idx === 0) return line;
				return firstLineIndent + line;
			}).join("\n");
		};

		const formattedWgpu = formatConfig(wgpuObj);
		const idx = newContent.lastIndexOf(configStr);
		if (idx !== -1) {
			newContent = newContent.substring(0, idx) + formattedWgpu + newContent.substring(idx + configStr.length);
		}
	}
}

// Write to launchPath
const launchPath = "/Users/ueli/Documents/semio/.vscode/launch.json";
writeFileSync(launchPath, newContent, "utf8");
console.log("Successfully updated launch.json with WGPU port offsets!");
