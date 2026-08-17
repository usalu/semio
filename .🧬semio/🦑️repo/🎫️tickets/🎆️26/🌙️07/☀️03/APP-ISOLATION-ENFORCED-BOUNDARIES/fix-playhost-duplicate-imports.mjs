import { existsSync, readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

function findReactIndexFiles(root) {
  const out = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir)) {
      if (entry === "node_modules" || entry === ".git") continue;
      const path = join(dir, entry);
      const stat = statSync(path);
      if (stat.isDirectory()) {
        const indexPath = join(path, "index.tsx");
        if (path.endsWith("/react") && existsSync(indexPath)) {
          out.push(indexPath);
        } else {
          walk(path);
        }
      }
    }
  };
  walk(root);
  return out;
}

const files = findReactIndexFiles("/Users/ueli/Documents/semio").filter((path) => !path.includes("/framework/product/platform/") && !path.includes("/framework/product/playground/") && !path.includes("/ui/react/"));

function parseImportBlock(source, startIndex) {
  const match = source.slice(startIndex).match(/^import\s+(?:type\s+)?[\s\S]*?from\s+["']([^"']+)["'];?/);
  if (!match) return null;
  const full = match[0];
  const module = match[1];
  const names = new Set();
  for (const part of full.matchAll(/(?:import|export)\s+(?:type\s+)?\{([^}]*)\}/g)) {
    for (const entry of part[1].split(",")) {
      const trimmed = entry.trim();
      if (!trimmed) continue;
      const name = trimmed
        .replace(/^type\s+/, "")
        .split(/\s+as\s+/)[0]
        ?.trim();
      if (name) names.add(name);
    }
  }
  const star = full.match(/import\s+\*\s+as\s+(\w+)/);
  if (star) names.add(`*as:${star[1]}`);
  const defaultImport = full.match(/^import\s+(\w+)\s+from/m);
  if (defaultImport) names.add(`default:${defaultImport[1]}`);
  return { full, module, names, end: startIndex + full.length };
}

function collectTopImports(source, playHostIndex) {
  const top = source.slice(0, playHostIndex);
  const byModule = new Map();
  let index = 0;
  while (index < top.length) {
    const next = top.indexOf("import", index);
    if (next < 0) break;
    const block = parseImportBlock(top, next);
    if (!block) {
      index = next + 6;
      continue;
    }
    const existing = byModule.get(block.module) ?? new Set();
    for (const name of block.names) existing.add(name);
    byModule.set(block.module, existing);
    index = block.end;
  }
  return byModule;
}

function stripDuplicateSpecifiers(importText, topByModule) {
  const moduleMatch = importText.match(/from\s+["']([^"']+)["'];?/);
  if (!moduleMatch) return importText;
  const module = moduleMatch[1];
  const topNames = topByModule.get(module);
  if (!topNames) return importText;
  if (importText.includes("* as ")) {
    const star = importText.match(/\*\s+as\s+(\w+)/);
    if (star && topNames.has(`*as:${star[1]}`)) return "";
  }
  const braceMatch = importText.match(/\{([\s\S]*?)\}/);
  if (!braceMatch) return importText;
  const kept = [];
  for (const entry of braceMatch[1].split(",")) {
    const trimmed = entry.trim();
    if (!trimmed) continue;
    const isType = trimmed.startsWith("type ");
    const name = trimmed
      .replace(/^type\s+/, "")
      .split(/\s+as\s+/)[0]
      ?.trim();
    if (name && topNames.has(name)) continue;
    kept.push(isType ? `type ${name}` : trimmed);
  }
  if (kept.length === 0) return "";
  const prefix = importText.slice(0, braceMatch.index);
  const suffix = importText.slice(braceMatch.index + braceMatch[0].length);
  return `${prefix}{ ${kept.join(", ")} }${suffix}`;
}

let changed = 0;
for (const path of files) {
  const source = readFileSync(path, "utf8");
  const marker = "//#region 🔖️PlayHost";
  const playHostIndex = source.indexOf(marker);
  if (playHostIndex < 0) continue;
  const topByModule = collectTopImports(source, playHostIndex);
  const playHost = source.slice(playHostIndex);
  let next = playHost;
  const importRegex = /^import[\s\S]*?from\s+["'][^"']+["'];?\n?/gm;
  next = next.replace(importRegex, (block) => {
    const cleaned = stripDuplicateSpecifiers(block, topByModule);
    return cleaned ? cleaned : "";
  });
  if (next === playHost) continue;
  const output = source.slice(0, playHostIndex) + next;
  writeFileSync(path, output);
  changed += 1;
  console.log("[DEBUG] fixed", path.replace("/Users/ueli/Documents/semio/", ""));
}
console.log("[DEBUG] files changed", changed);
