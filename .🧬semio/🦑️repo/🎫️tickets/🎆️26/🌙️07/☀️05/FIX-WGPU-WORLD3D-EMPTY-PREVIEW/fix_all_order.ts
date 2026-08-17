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

// Rename the second "🛠️dev🖥️s🧊️wgpu" to "🛠️dev🖥️s🧊️wgpu-alt"
let wgpuCount = 0;
for (const config of configs) {
  if (config.name === "🛠️dev🖥️s🧊️wgpu") {
    wgpuCount++;
    if (wgpuCount === 2) {
      console.log("Renaming second 🛠️dev🖥️s🧊️wgpu to 🛠️dev🖥️s🧊️wgpu-alt");
      config.name = "🛠️dev🖥️s🧊️wgpu-alt";
      config.obj.name = "🛠️dev🖥️s🧊️wgpu-alt";

      // Format the updated object
      const lines = config.raw.split("\n");
      const firstLineIndent = lines[0]?.match(/^\s*/)?.[0] || "    ";
      const formatConfig = (obj: any) => {
        const str = JSON.stringify(obj, null, 2);
        return str
          .split("\n")
          .map((line, idx) => {
            if (idx === 0) return line;
            return firstLineIndent + line;
          })
          .join("\n");
      };
      config.raw = formatConfig(config.obj);
    }
  }
}

// Reorder configs array.
const orderedConfigs: typeof configs = [];
const processed = new Set<number>();

for (let j = 0; j < configs.length; j++) {
  if (processed.has(j)) continue;

  const config = configs[j];

  // Skip WGPU configs for now, we will add them right after their React counterparts.
  const isWgpu = config.name.endsWith("🧊️wgpu");
  const isWgpuAlt = config.name.endsWith("🧊️wgpu-alt");
  if (isWgpu || isWgpuAlt) {
    const base = isWgpu ? config.name.replace("🧊️wgpu", "") : config.name.replace("🧊️wgpu-alt", "");
    const reactName = base + "⚛️react";
    if (configs.some((c) => c.name === reactName)) {
      // Skip and let the React pass handle it
      continue;
    }
  }

  orderedConfigs.push(config);
  processed.add(j);

  if (config.name.endsWith("⚛️react")) {
    const base = config.name.replace("⚛️react", "");
    const wgpuName = base + "🧊️wgpu";
    const wgpuAltName = base + "🧊️wgpu-alt";

    // Find and append corresponding WGPU configs
    for (let k = 0; k < configs.length; k++) {
      if (configs[k].name === wgpuName || configs[k].name === wgpuAltName) {
        orderedConfigs.push(configs[k]);
        processed.add(k);
      }
    }
  }
}

// Append any remaining unprocessed configurations
for (let j = 0; j < configs.length; j++) {
  if (!processed.has(j)) {
    orderedConfigs.push(configs[j]);
  }
}

// Reconstruct launch.json
const firstConfigRaw = configsRaw[0];
const lastConfigRaw = configsRaw[configsRaw.length - 1];

const blockStart = content.indexOf(firstConfigRaw);
const blockEnd = content.lastIndexOf(lastConfigRaw) + lastConfigRaw.length;

const firstConfigEnd = content.indexOf(firstConfigRaw) + firstConfigRaw.length;
const secondConfigStart = content.indexOf(configsRaw[1]);
const separator = content.substring(firstConfigEnd, secondConfigStart);

const replacementBlock = orderedConfigs.map((c) => c.raw).join(separator);

const newContent = content.substring(0, blockStart) + replacementBlock + content.substring(blockEnd);

writeFileSync(launchPath, newContent, "utf8");
console.log("Successfully fixed launch configuration ordering and names!");
