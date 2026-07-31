import { readFileSync } from "fs";
import { join } from "path";

const launchJsonPath = join(process.cwd(), ".vscode", "launch.json");
const content = readFileSync(launchJsonPath, "utf8");

interface ConfigBlock {
  name: string;
  command?: string;
  rawText: string;
  parsed: any;
}

const configs: ConfigBlock[] = [];

let idx = content.indexOf('"configurations"');
idx = content.indexOf("[", idx);

let braceDepth = 0;
let inString = false;
let inLineComment = false;
let inBlockComment = false;
let currentBlockStart = -1;
let cleanTextBuffer: string[] = [];

let i = idx + 1;
while (i < content.length) {
  const char = content[i];
  const nextChar = content[i + 1] || "";
  const prevChar = content[i - 1] || "";

  if (inLineComment) {
    if (char === "\n") inLineComment = false;
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
        });
      } catch (e) {}
      currentBlockStart = -1;
    }
  } else {
    if (braceDepth > 0) cleanTextBuffer.push(char);
  }
  if (braceDepth === 0 && char === "]") break;
  i++;
}

// Apply filtering
const filteredConfigs: ConfigBlock[] = [];
for (const config of configs) {
  const name = config.name || "";
  const command = config.command || "";

  if (name.includes("🧪️") || name.includes("validate") || name.toLowerCase().includes("test") || (command && (command.includes("test") || command.includes("validate")))) {
    continue;
  }
  if (
    name.includes("🎗️vscode") ||
    name.includes("vscodeintegrated") ||
    (command && (command.includes("vscode") || command.includes("flow-module") || command.includes("flow-core:wasm") || command.includes("dag-core:wasm"))) ||
    name.includes("flow🦀️module") ||
    name.includes("flow🦀️rs") ||
    name.includes("dag🦀️rs")
  ) {
    continue;
  }
  filteredConfigs.push(config);
}

// Let's normalize commands to detect semantic duplicates
const normalizeCommand = (cmd: string): string => {
  if (!cmd) return "";
  let normalized = cmd.trim();
  // replace "bun run dev --" with "bun ./📜️script.ts dev" or equivalent
  normalized = normalized.replace(/^bun run dev --/, "bun ./📜️script.ts dev");
  // replace "bun run" with "bun"
  normalized = normalized.replace(/^bun run/, "bun");
  // replace "./📜️script.ts" with "script.ts"
  normalized = normalized.replace(/\.\/script\.ts/, "script.ts");
  return normalized;
};

const seenNormalized = new Map<string, ConfigBlock[]>();
const seenNames = new Map<string, ConfigBlock[]>();

for (const config of filteredConfigs) {
  const name = config.name;
  const cmd = config.command || "";
  const norm = normalizeCommand(cmd);

  if (!seenNames.has(name)) seenNames.set(name, []);
  seenNames.get(name)!.push(config);

  if (norm) {
    if (!seenNormalized.has(norm)) seenNormalized.set(norm, []);
    seenNormalized.get(norm)!.push(config);
  }
}

console.log("--- Duplicate Names (in Kept Configs) ---");
for (const [name, list] of seenNames.entries()) {
  if (list.length > 1) {
    console.log(`Name: ${name}`);
    for (const c of list) {
      console.log(`  - Command: ${c.command}`);
    }
  }
}

console.log("\n--- Duplicate / Equivalent Commands (in Kept Configs) ---");
for (const [norm, list] of seenNormalized.entries()) {
  if (list.length > 1) {
    console.log(`Normalized command: "${norm}"`);
    for (const c of list) {
      console.log(`  - Name: "${c.name}" (Original command: "${c.command}")`);
    }
  }
}
