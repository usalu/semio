import { readFileSync, writeFileSync } from "fs";
import JSON5 from "json5";

const launchPath = ".vscode/launch.json";
const content = readFileSync(launchPath, "utf8");
const lines = content.split("\n");

function cleanJsoncLine(line: string): string {
  let inString = false;
  let escape = false;
  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    if (escape) { escape = false; continue; }
    if (ch === "\\") { escape = true; continue; }
    if (ch === '"') { inString = !inString; continue; }
    if (!inString && ch === "/" && line[i + 1] === "/") {
      return line.slice(0, i);
    }
  }
  return line;
}

// Find ranges [startLineIndex, endLineIndex] for configs starting with 🧪️
const configRangesToRemove: [number, number][] = [];

let currentStartLine = -1;
let insideConfigs = false;
let depth = 0;
let isTestConfig = false;

for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  if (line.includes('"configurations": [')) {
    insideConfigs = true;
    continue;
  }
  if (!insideConfigs) continue;

  const cleaned = cleanJsoncLine(line);
  let inString = false;
  let escape = false;
  let lineOpenBraces = 0;
  let lineCloseBraces = 0;

  for (let j = 0; j < cleaned.length; j++) {
    const ch = cleaned[j];
    if (escape) { escape = false; continue; }
    if (ch === "\\") { escape = true; continue; }
    if (ch === '"') { inString = !inString; continue; }
    if (!inString) {
      if (ch === "{") lineOpenBraces++;
      if (ch === "}") lineCloseBraces++;
    }
  }

  if (lineOpenBraces > 0 && depth === 0) {
    currentStartLine = i;
    isTestConfig = false;
  }

  if (depth > 0 || lineOpenBraces > 0) {
    if (line.includes('"name"') && line.includes('🧪')) {
      isTestConfig = true;
    }
  }

  depth += lineOpenBraces - lineCloseBraces;

  if (depth === 0 && currentStartLine !== -1) {
    if (isTestConfig) {
      configRangesToRemove.push([currentStartLine, i]);
    }
    currentStartLine = -1;
  }
}

console.log(`Found ${configRangesToRemove.length} config ranges to remove.`);

// Filter out lines in ranges to remove
const keepLineFlags = new Array(lines.length).fill(true);
for (const [start, end] of configRangesToRemove) {
  for (let i = start; i <= end; i++) {
    keepLineFlags[i] = false;
  }
}

let newLines = lines.filter((_, idx) => keepLineFlags[idx]);

// Fix trailing commas before closing bracket of configurations array
// Find the last configuration closing brace before configurations end
let newContent = newLines.join("\n");

// Verify JSON5 validity
try {
  const parsed = JSON5.parse(newContent);
  console.log(`Success! Total remaining configs: ${parsed.configurations.length}`);
  const remainingTestConfigs = parsed.configurations.filter((c: any) =>
    (c.name || "").includes("🧪") || (c.name || "").toLowerCase().includes("test")
  );
  console.log(`Remaining test configs: ${remainingTestConfigs.length}`);
  
  if (remainingTestConfigs.length === 0) {
    writeFileSync(launchPath, newContent, "utf8");
    console.log("Updated .vscode/launch.json successfully!");
  } else {
    console.error("Warning: Some test configs remain!", remainingTestConfigs);
  }
} catch (e: any) {
  console.error("JSON5 parse error after removal:", e.message);
}
