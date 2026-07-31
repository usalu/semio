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

// Find index of s React, s wgpu, s wgpu-alt raw configs
let sReactIdx = -1;
let sWgpuIdx = -1;
let sWgpuAltIdx = -1;

for (let j = 0; j < configsRaw.length; j++) {
  try {
    const obj = eval("(" + configsRaw[j] + ")");
    if (obj.name === "🛠️dev🖥️s⚛️react") {
      sReactIdx = j;
    } else if (obj.name === "🛠️dev🖥️s🧊️wgpu") {
      sWgpuIdx = j;
    } else if (obj.name === "🛠️dev🖥️s🧊️wgpu-alt") {
      sWgpuAltIdx = j;
    }
  } catch {}
}

if (sReactIdx !== -1 && sWgpuIdx !== -1 && sWgpuAltIdx !== -1) {
  console.log(`Found s React at index ${sReactIdx}, WGPU at ${sWgpuIdx}, WGPU-alt at ${sWgpuAltIdx}`);

  const sReactRaw = configsRaw[sReactIdx];
  const sWgpuRaw = configsRaw[sWgpuIdx];
  const sWgpuAltRaw = configsRaw[sWgpuAltIdx];

  // We want to replace this group of configs in content.
  // Let's detect the indices of these configs in content.
  const reactPos = content.indexOf(sReactRaw);
  const wgpuPos = content.indexOf(sWgpuRaw);
  const wgpuAltPos = content.indexOf(sWgpuAltRaw);

  // Ensure all were found
  if (reactPos !== -1 && wgpuPos !== -1 && wgpuAltPos !== -1) {
    // Detect the exact lines and spaces around them to move them cleanly.
    // Since React is currently last, let's remove React block (and its trailing/leading commas/whitespace)
    // and insert it before the WGPU block.

    // To do this simply without breaking launch.json formatting, we can rewrite the file content by replacing the blocks.
    // Let's build the new content:
    // React should come first, then WGPU, then WGPU-alt.
    // Let's get the indentation from first line of WGPU config
    const firstLineIndent = sWgpuRaw.split("\n")[0]?.match(/^\s*/)?.[0] || "    ";

    let newContent = content;

    // Since we want to reorder them in-place:
    // Let's replace:
    // sWgpuRaw -> sReactRaw
    // sReactRaw -> sWgpuAltRaw
    // sWgpuAltRaw -> sWgpuRaw
    // Wait, that's just a permutation!
    // Let's replace the three blocks directly:
    // We'll replace the block from sWgpuRaw to sReactRaw inclusive.
    // Let's find the start index of sWgpuRaw and the end index of sReactRaw.
    const blockStart = wgpuPos;
    const blockEnd = reactPos + sReactRaw.length;

    const originalBlock = content.substring(blockStart, blockEnd);
    // Let's construct the replacement block.
    // We want: React, then WGPU, then WGPU-alt
    // Let's extract formatting details (commas and newlines) between them.
    // In the original block, we have:
    // [WGPU] <sep1> [WGPU-alt] <sep2> [React]
    // Let's parse the separators:
    const sep1Start = wgpuPos + sWgpuRaw.length;
    const sep1End = wgpuAltPos;
    const sep1 = content.substring(sep1Start, sep1End);

    const sep2Start = wgpuAltPos + sWgpuAltRaw.length;
    const sep2End = reactPos;
    const sep2 = content.substring(sep2Start, sep2End);

    // Replacement:
    // [React] <sep1> [WGPU] <sep2> [WGPU-alt]
    const replacementBlock = sReactRaw + sep1 + sWgpuRaw + sep2 + sWgpuAltRaw;

    newContent = content.substring(0, blockStart) + replacementBlock + content.substring(blockEnd);

    writeFileSync(launchPath, newContent, "utf8");
    console.log("Successfully reordered s React and WGPU configurations!");
  }
} else {
  console.error("Could not find all s configurations");
}
