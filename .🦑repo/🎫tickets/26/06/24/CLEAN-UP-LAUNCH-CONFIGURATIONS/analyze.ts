import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const launchJsonPath = join(process.cwd(), ".vscode", "launch.json");
const content = readFileSync(launchJsonPath, "utf8");

interface ConfigBlock {
  name: string;
  command?: string;
  rawText: string;
  parsed: any;
  region: string;
}

const configs: ConfigBlock[] = [];

// Parse character by character
let idx = content.indexOf('"configurations"');
if (idx === -1) {
  console.error("Could not find configurations key in launch.json");
  process.exit(1);
}

idx = content.indexOf("[", idx);
if (idx === -1) {
  console.error("Could not find configurations opening [");
  process.exit(1);
}

let braceDepth = 0;
let inString = false;
let inLineComment = false;
let inBlockComment = false;
let currentBlockStart = -1;
let cleanTextBuffer: string[] = [];

let currentRegion = "default";

// Keep track of regions by reading lines leading up to configurations
const linesBefore = content.substring(0, idx).split("\n");
for (const line of linesBefore) {
  const trimmed = line.trim();
  if (trimmed.startsWith("// #region")) {
    currentRegion = trimmed;
  }
}

let i = idx + 1;
while (i < content.length) {
  const char = content[i];
  const nextChar = content[i + 1] || "";
  const prevChar = content[i - 1] || "";

  if (inLineComment) {
    if (char === "\n") {
      inLineComment = false;
      const lineText =
        content
          .substring(idx + 1, i)
          .split("\n")
          .pop() || "";
      const trimmedLine = lineText.trim();
      if (trimmedLine.startsWith("// #region")) {
        currentRegion = trimmedLine;
      }
    }
    i++;
    continue;
  }

  if (inBlockComment) {
    if (char === "*" && nextChar === "/") {
      inBlockComment = false;
      i += 2;
      continue;
    }
    i++;
    continue;
  }

  if (inString) {
    if (braceDepth > 0) cleanTextBuffer.push(char);
    if (char === '"' && prevChar !== "\\") inString = false;
    i++;
    continue;
  }

  if (char === "/" && nextChar === "/") {
    inLineComment = true;
    i += 2;
    continue;
  }
  if (char === "/" && nextChar === "*") {
    inBlockComment = true;
    i += 2;
    continue;
  }
  if (char === '"') {
    inString = true;
    if (braceDepth > 0) cleanTextBuffer.push(char);
    i++;
    continue;
  }

  if (char === "{") {
    if (braceDepth === 0) {
      currentBlockStart = i;
      cleanTextBuffer = [];
    }
    braceDepth++;
    cleanTextBuffer.push(char);
  } else if (char === "}") {
    cleanTextBuffer.push(char);
    braceDepth--;
    if (braceDepth === 0 && currentBlockStart !== -1) {
      const rawText = content.substring(currentBlockStart, i + 1);
      const cleanJson = cleanTextBuffer.join("");
      try {
        const parsed = JSON.parse(cleanJson);
        configs.push({
          name: parsed.name,
          command: parsed.command,
          rawText,
          parsed,
          region: currentRegion,
        });
      } catch (e) {
        console.error(`Failed to parse block starting at ${currentBlockStart}:`, cleanJson, e);
      }
      currentBlockStart = -1;
    }
  } else {
    if (braceDepth > 0) {
      cleanTextBuffer.push(char);
    }
  }

  if (braceDepth === 0 && char === "]") break;
  i++;
}

console.log(`Parsed ${configs.length} configurations.`);

// Normalize commands to detect semantic duplicates
const normalizeCommand = (cmd: string): string => {
  if (!cmd) return "";
  let normalized = cmd.trim();
  normalized = normalized.replace(/^bun run dev --/, "bun ./script.ts dev");
  normalized = normalized.replace(/^bun run/, "bun");
  normalized = normalized.replace(/\.\/script\.ts/, "script.ts");
  return normalized;
};

// Filter out tests, extension specific setups, and duplicates
const toKeep: ConfigBlock[] = [];
const seenNames = new Set<string>();
const seenCommands = new Set<string>();

for (const config of configs) {
  const name = config.name || "";
  const command = config.command || "";
  const normCommand = normalizeCommand(command);

  // 1. Filter out tests / validations
  if (name.includes("🧪") || name.includes("validate") || name.toLowerCase().includes("test") || (command && (command.includes("test") || command.includes("validate")))) {
    continue;
  }
  // 2. Filter out extension / module specific setups
  if (
    name.includes("🎗️vscode") ||
    name.includes("vscodeintegrated") ||
    (command && (command.includes("vscode") || command.includes("flow-module") || command.includes("flow-core:wasm") || command.includes("dag-core:wasm"))) ||
    name.includes("flow🦀module") ||
    name.includes("flow🦀rs") ||
    name.includes("dag🦀rs")
  ) {
    continue;
  }
  // 3. Filter out duplicates
  if (seenNames.has(name)) {
    continue;
  }
  if (normCommand && seenCommands.has(normCommand)) {
    continue;
  }

  toKeep.push(config);
  seenNames.add(name);
  if (normCommand) seenCommands.add(normCommand);
}

// Group configurations by region emoji prefix
const keyboard: ConfigBlock[] = [];
const mouse: ConfigBlock[] = [];
const dev: ConfigBlock[] = [];
const build: ConfigBlock[] = [];
const publish: ConfigBlock[] = [];

for (const config of toKeep) {
  const name = config.name || "";
  if (name.startsWith("⌨️")) {
    keyboard.push(config);
  } else if (name.startsWith("🖱️")) {
    mouse.push(config);
  } else if (name.startsWith("🛠️")) {
    dev.push(config);
  } else if (name.startsWith("📦")) {
    build.push(config);
  } else if (name.startsWith("⬆️")) {
    publish.push(config);
  } else {
    dev.push(config);
  }
}

// Helper to strip emojis for clean alphabetical comparison
const stripEmojis = (str: string): string => {
  return str
    .replace(/[\u2000-\u32FF]|[\ud800-\udbff][\udc00-\udfff]/g, "")
    .replace(/[^a-zA-Z0-9\s]/g, "")
    .trim();
};

// Sort each group strictly alphabetically by name (excluding emojis)
const sortAlphabetically = (a: ConfigBlock, b: ConfigBlock) => {
  const nameA = stripEmojis(a.name);
  const nameB = stripEmojis(b.name);
  return nameA.localeCompare(nameB, "en", { sensitivity: "base", numeric: true });
};

keyboard.sort(sortAlphabetically);
mouse.sort(sortAlphabetically);
dev.sort(sortAlphabetically);
build.sort(sortAlphabetically);
publish.sort(sortAlphabetically);

// Construct the new launch.json content
const prefix = content.substring(0, idx + 1); // Up to the opening '['

const formattedConfigs: string[] = [];

const addRegion = (regionName: string, items: ConfigBlock[]) => {
  if (items.length === 0) return;
  formattedConfigs.push(`    // #region ${regionName}`);
  items.forEach((item, index) => {
    // Assign sequential presentation.order to match the source JSON order
    if (!item.parsed.presentation) {
      item.parsed.presentation = {};
    }
    item.parsed.presentation.order = (index + 1) * 10;

    const stringified = JSON.stringify(item.parsed, null, 2);
    const indented = stringified
      .split("\n")
      .map((line) => "    " + line)
      .join("\n");
    formattedConfigs.push(indented + ",");
  });
  formattedConfigs.push("    // #endregion");
};

addRegion("⌨️ Keyboard", keyboard);
addRegion("🖱️ Mouse", mouse);
addRegion("🛠️ Dev", dev);
addRegion("📦 Build", build);
addRegion("⬆️ Publish", publish);

// Join configurations and handle trailing commas
let configsStr = formattedConfigs.join("\n");

const configLines = configsStr.split("\n");
for (let j = configLines.length - 1; j >= 0; j--) {
  if (configLines[j].trim() === "}" || configLines[j].trim() === "},") {
    if (configLines[j].endsWith(",")) {
      configLines[j] = configLines[j].slice(0, -1);
    }
    break;
  }
}
configsStr = configLines.join("\n");

const suffix = content.substring(i); // From the closing ']' to the end of the file

const finalContent = prefix + "\n" + configsStr + "\n  " + suffix;

writeFileSync(launchJsonPath, finalContent, "utf8");
console.log("Successfully wrote updated launch.json.");
