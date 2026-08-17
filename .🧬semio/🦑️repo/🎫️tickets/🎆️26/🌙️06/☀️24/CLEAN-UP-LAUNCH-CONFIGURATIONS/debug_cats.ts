import { readFileSync } from "fs";
import { join } from "path";

const launchJsonPath = join(process.cwd(), ".vscode", "launch.json");
const content = readFileSync(launchJsonPath, "utf8");

interface ConfigBlock {
  name: string;
  command?: string;
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

const stripEmojis = (str: string): string => {
  return str
    .replace(/[\u2000-\u32FF]|[\ud800-\udbff][\udc00-\udfff]/g, "")
    .replace(/[^a-zA-Z0-9\s]/g, "")
    .trim();
};

const sortAlphabetically = (a: ConfigBlock, b: ConfigBlock) => {
  const nameA = stripEmojis(a.name);
  const nameB = stripEmojis(b.name);
  return nameA.localeCompare(nameB, "en", { sensitivity: "base", numeric: true });
};

configs.sort(sortAlphabetically);

console.log("--- Stripped Alphabetical Order ---");
configs.forEach((c) => {
  console.log(`Stripped: "${stripEmojis(c.name)}" -> Original: "${c.name}"`);
});
