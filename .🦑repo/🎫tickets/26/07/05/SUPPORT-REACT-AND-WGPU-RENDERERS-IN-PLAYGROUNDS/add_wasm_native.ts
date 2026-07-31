import { writeFileSync } from "node:fs";
import { execSync } from "node:child_process";

const launchPath = "/Users/ueli/Documents/semio/.vscode/launch.json";
// Read the clean launch.json from HEAD to avoid accumulative edits
const content = execSync("git show HEAD:.vscode/launch.json", { encoding: "utf8" });

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

const FRAMEWORK_OS_PLAYGROUND_PLUGIN_ALIASES: Record<string, string> = {
  s: "s",
  draw: "draw",
  note: "note",
  writer: "writer",
  raster: "raster",
  forms: "forms",
  vcs: "vcs",
  flow: "flow",
  dag: "dag",
  imperative: "imperative",
  sequence: "sequence",
  layout: "layout",
  lowpoly: "lowpoly",
  shooting: "shooting",
  "2d": "puzzle2d",
  "3d": "puzzle3d",
  "5d": "puzzle5d",
  "gis 2d": "gis2d",
  wires: "reasoning-wires",
  cad: "cad",
  "procedural 2d": "procedural2d",
  "procedural 3d": "procedural3d",
  "trinity jack": "trinity",
  "trinity rewrite": "trinity-rewrite",
  presentation: "presentation",
};

function getPluginId(command: string): string {
  if (command.startsWith("bun run dev:")) {
    const alias = command.replace("bun run dev:", "").split(" ")[0];
    return alias;
  }
  if (command.startsWith("bun ./script.ts dev ")) {
    const rest = command.replace("bun ./script.ts dev ", "").split(" ");
    for (let len = rest.length; len >= 1; len--) {
      const alias = rest.slice(0, len).join(" ");
      const plugin = FRAMEWORK_OS_PLAYGROUND_PLUGIN_ALIASES[alias];
      if (plugin) return plugin;
    }
  }
  if (command.startsWith("bun nx run @semio-tech/framework-os-dev:dev")) {
    return "s";
  }
  return "s";
}

function normalizePluginId(id: string): string {
  let clean = id.split(":")[0];
  if (clean === "gis") return "gis2d";
  if (clean === "procedural") {
    if (id.includes("3d")) return "procedural3d";
    return "procedural2d";
  }
  if (clean === "puzzle") {
    if (id.includes("3d")) return "puzzle3d";
    if (id.includes("5d")) return "puzzle5d";
    return "puzzle2d";
  }
  if (clean === "reasoning") return "reasoning-wires";
  if (clean === "trinity") {
    if (id.includes("rewrite")) return "trinity-rewrite";
    return "trinity";
  }
  return clean;
}

const newConfigs: typeof configs = [];

for (const config of configs) {
  if (config.name.endsWith("🧊wgpu")) {
    const baseName = config.name.replace("🧊wgpu", "");
    const rawPluginId = getPluginId(config.obj.command || "");
    const pluginId = normalizePluginId(rawPluginId);

    // 1. Create WGPU WASM config
    const wasmObj = JSON.parse(JSON.stringify(config.obj));
    wasmObj.name = baseName + "🧊wgpu🌐wasm";
    if (wasmObj.presentation && typeof wasmObj.presentation.order === "number") {
      wasmObj.presentation.order = Number(wasmObj.presentation.order.toFixed(1));
    }

    // 2. Create WGPU Native config
    const nativeObj = JSON.parse(JSON.stringify(config.obj));
    nativeObj.name = baseName + "🧊wgpu🖥️native";
    nativeObj.command = `bun ./framework/renderer/wgpu/script.ts native ${pluginId}`;
    delete nativeObj.env;
    delete nativeObj.serverReadyAction;
    if (nativeObj.presentation && typeof nativeObj.presentation.order === "number") {
      nativeObj.presentation.order = Number((nativeObj.presentation.order + 0.1).toFixed(2));
    }

    // Format both configs
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

    const wasmRaw = formatConfig(wasmObj);
    const nativeRaw = formatConfig(nativeObj);

    newConfigs.push({ raw: wasmRaw, obj: wasmObj, name: wasmObj.name });
    newConfigs.push({ raw: nativeRaw, obj: nativeObj, name: nativeObj.name });

    console.log(`Split WGPU config for ${pluginId} into wasm and native`);
  } else {
    newConfigs.push(config);
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

const replacementBlock = newConfigs.map((c) => c.raw).join(separator);

const newContent = content.substring(0, blockStart) + replacementBlock + content.substring(blockEnd);

writeFileSync(launchPath, newContent, "utf8");
console.log("Successfully added wgpu wasm and native configurations!");
